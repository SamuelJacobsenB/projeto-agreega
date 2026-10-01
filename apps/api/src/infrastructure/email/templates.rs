pub fn invitation(token: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
            <html lang="pt-BR">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Convite para o Agreega</title>
            </head>

            <body style="
                margin: 0;
                padding: 40px 20px;
                background: #f7f7f7;
                font-family: Arial, Helvetica, sans-serif;
                color: #171717;
            ">
                <div style="
                    max-width: 480px;
                    margin: 0 auto;
                    padding: 40px;
                    background: #ffffff;
                    border-radius: 12px;
                ">
                    <div style="
                        margin-bottom: 36px;
                        font-size: 24px;
                        font-weight: 700;
                        letter-spacing: -0.5px;
                    ">
                        Agreega
                    </div>

                    <h1 style="
                        margin: 0 0 16px;
                        font-size: 28px;
                        line-height: 1.2;
                        letter-spacing: -0.5px;
                    ">
                        Você foi convidado.
                    </h1>

                    <p style="
                        margin: 0 0 28px;
                        font-size: 16px;
                        line-height: 1.6;
                        color: #666666;
                    ">
                        Você recebeu um convite para acessar o Agreega.
                        Clique abaixo para criar sua conta e começar.
                    </p>

                    <a
                        href="https://app.agreega.com.br/invitations/accept?token={token}"
                        style="
                            display: inline-block;
                            padding: 13px 22px;
                            background: #171717;
                            color: #ffffff;
                            text-decoration: none;
                            border-radius: 7px;
                            font-size: 14px;
                            font-weight: 600;
                        "
                    >
                        Aceitar convite
                    </a>

                    <p style="
                        margin: 32px 0 0;
                        font-size: 12px;
                        line-height: 1.5;
                        color: #999999;
                    ">
                        Se você não esperava este convite, pode ignorar este e-mail.
                    </p>

                    <div style="
                        margin-top: 32px;
                        padding-top: 20px;
                        border-top: 1px solid #eeeeee;
                        font-size: 12px;
                        color: #aaaaaa;
                    ">
                        © Agreega
                    </div>
                </div>
            </body>
            </html>"#
    )
}

pub fn password_reset(app_url: &str, token: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Redefinição de senha - Agreega</title>
</head>

<body style="
    margin: 0;
    padding: 40px 20px;
    background: #f7f7f7;
    font-family: Arial, Helvetica, sans-serif;
    color: #171717;
">
    <div style="
        max-width: 480px;
        margin: 0 auto;
        padding: 40px;
        background: #ffffff;
        border-radius: 12px;
    ">
        <div style="
            margin-bottom: 36px;
            font-size: 24px;
            font-weight: 700;
            letter-spacing: -0.5px;
        ">
            Agreega
        </div>

        <h1 style="
            margin: 0 0 16px;
            font-size: 28px;
            line-height: 1.2;
            letter-spacing: -0.5px;
        ">
            Redefina sua senha.
        </h1>

        <p style="
            margin: 0 0 28px;
            font-size: 16px;
            line-height: 1.6;
            color: #666666;
        ">
            Recebemos uma solicitação para redefinir a senha
            da sua conta no Agreega. Clique abaixo para criar
            uma nova senha.
        </p>

        <a
            href="{app_url}/password-reset?token={token}"
            style="
                display: inline-block;
                padding: 13px 22px;
                background: #171717;
                color: #ffffff;
                text-decoration: none;
                border-radius: 7px;
                font-size: 14px;
                font-weight: 600;
            "
        >
            Redefinir senha
        </a>

        <p style="
            margin: 32px 0 0;
            font-size: 12px;
            line-height: 1.5;
            color: #999999;
        ">
            Se você não solicitou a redefinição da sua senha,
            pode ignorar este e-mail. Sua senha atual continuará
            a mesma.
        </p>

        <div style="
            margin-top: 32px;
            padding-top: 20px;
            border-top: 1px solid #eeeeee;
            font-size: 12px;
            color: #aaaaaa;
        ">
            © Agreega
        </div>
    </div>
</body>
</html>"#,
        app_url = app_url,
        token = token,
    )
}
