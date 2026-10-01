use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
};

use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use crate::{
    app::AppState,
    domains::auth::{
        dtos::{
            AuthTokenResponseDto, LoginRequestDto, RequestPasswordResetRequestDto,
            ResetPasswordRequestDto,
        },
        service::Service,
    },
    response::{ApiResponse, ApiResult, AppError, ValidatedJson},
};

pub type AuthApiResult<T> = Result<(StatusCode, CookieJar, ApiResponse<T>), AppError>;

pub struct Handler;

impl Handler {
    pub async fn login(
        State(state): State<AppState>,
        headers: HeaderMap,
        jar: CookieJar,
        ValidatedJson(dto): ValidatedJson<LoginRequestDto>,
    ) -> AuthApiResult<AuthTokenResponseDto> {
        let user_agent = Self::user_agent(&headers);

        let tokens = Service::login(
            &state.pool,
            &dto.email,
            &dto.password,
            &state.config.jwt_secret,
            user_agent,
        )
        .await?;

        let jar = jar.add(Self::refresh_cookie(
            &tokens.refresh_token,
            state.config.cookie_secure,
        ));

        let response = AuthTokenResponseDto {
            access_token: tokens.access_token,
            expires_at: tokens.expires_at,
        };

        Ok((
            StatusCode::OK,
            jar,
            ApiResponse::success("Autenticação realizada com sucesso.", Some(response)),
        ))
    }

    pub async fn refresh(
        State(state): State<AppState>,
        headers: HeaderMap,
        jar: CookieJar,
    ) -> AuthApiResult<AuthTokenResponseDto> {
        let user_agent = Self::user_agent(&headers);

        let refresh_token = jar
            .get("refresh_token")
            .ok_or_else(|| AppError::Unauthorized("Refresh token ausente.".to_string()))?;

        let tokens = Service::refresh(
            &state.pool,
            refresh_token.value(),
            &state.config.jwt_secret,
            user_agent,
        )
        .await?;

        let jar = jar.add(Self::refresh_cookie(
            &tokens.refresh_token,
            state.config.cookie_secure,
        ));

        let response = AuthTokenResponseDto {
            access_token: tokens.access_token,
            expires_at: tokens.expires_at,
        };

        Ok((
            StatusCode::OK,
            jar,
            ApiResponse::success("Sessão renovada com sucesso.", Some(response)),
        ))
    }

    pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> AuthApiResult<()> {
        if let Some(refresh_token) = jar.get("refresh_token") {
            Service::logout(&state.pool, refresh_token.value()).await?;
        }

        let jar = jar.remove(Self::expired_refresh_cookie(state.config.cookie_secure));

        Ok((
            StatusCode::OK,
            jar,
            ApiResponse::success("Sessão encerrada com sucesso.", None),
        ))
    }
}

impl Handler {
    pub async fn request_password_reset(
        State(state): State<AppState>,
        ValidatedJson(dto): ValidatedJson<RequestPasswordResetRequestDto>,
    ) -> ApiResult<()> {
        Service::request_password_reset(
            &state.pool,
            &state.email_service,
            &state.config.app_url,
            &dto.email,
        )
        .await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Se o e-mail estiver cadastrado, você receberá instruções para redefinir a senha.",
                None,
            ),
        ))
    }

    pub async fn reset_password(
        State(state): State<AppState>,
        ValidatedJson(dto): ValidatedJson<ResetPasswordRequestDto>,
    ) -> ApiResult<()> {
        Service::reset_password(&state.pool, &dto.token, &dto.password).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Senha redefinida com sucesso.", None),
        ))
    }
}

impl Handler {
    fn refresh_cookie(token: &str, cookie_secure: bool) -> Cookie<'static> {
        Cookie::build(("refresh_token", token.to_owned()))
            .http_only(true)
            .secure(cookie_secure)
            .same_site(SameSite::Lax)
            .path("/api/auth")
            .build()
    }

    fn expired_refresh_cookie(cookie_secure: bool) -> Cookie<'static> {
        Cookie::build(("refresh_token", ""))
            .http_only(true)
            .secure(cookie_secure)
            .same_site(SameSite::Lax)
            .path("/api/auth")
            .build()
    }

    fn user_agent(headers: &HeaderMap) -> Option<&str> {
        headers
            .get("user-agent")
            .and_then(|value| value.to_str().ok())
    }
}
