use super::{dtos::InvitationResponseDto, models::Invitation};

impl From<Invitation> for InvitationResponseDto {
    fn from(invitation: Invitation) -> Self {
        Self {
            id: invitation.id,
            email: invitation.email,
            invited_by: invitation.invited_by,
            client_id: invitation.client_id,
            role: invitation.role,
            expires_at: invitation.expires_at,
            accepted_at: invitation.accepted_at,
            created_at: invitation.created_at,
        }
    }
}
