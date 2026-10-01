use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::invitations::{
        dtos::{
            AcceptInvitationRequestDto, CreateInvitationRequestDto, CreatedInvitationResponseDto,
            InvitationResponseDto,
        },
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn list_invitations(
        State(state): State<AppState>,
    ) -> ApiResult<Vec<InvitationResponseDto>> {
        let invitations = Service::list_invitations(&state.pool).await?;

        let invitation_dtos = invitations
            .into_iter()
            .map(InvitationResponseDto::from)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Convites encontrados.", Some(invitation_dtos)),
        ))
    }

    pub async fn create_invitation(
        State(state): State<AppState>,
        AuthUser { id: invited_by, .. }: AuthUser,
        ValidatedJson(dto): ValidatedJson<CreateInvitationRequestDto>,
    ) -> ApiResult<CreatedInvitationResponseDto> {
        let (invitation, token) = Service::create_invitation(&state.pool, &dto, invited_by).await?;

        let response = CreatedInvitationResponseDto {
            invitation: InvitationResponseDto::from(invitation),
            token,
        };

        Ok((
            StatusCode::CREATED,
            ApiResponse::success("Convite criado e enviado com sucesso.", Some(response)),
        ))
    }

    pub async fn accept_invitation(
        State(state): State<AppState>,
        ValidatedJson(dto): ValidatedJson<AcceptInvitationRequestDto>,
    ) -> ApiResult<()> {
        Service::accept_invitation(&state.pool, &dto).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success("Conta criada com sucesso.", None),
        ))
    }

    pub async fn delete_invitation(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete_invitation(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Convite excluído com sucesso.", None),
        ))
    }
}
