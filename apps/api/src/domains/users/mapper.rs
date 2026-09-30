use super::{dtos::UserResponseDto, models::User};

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            phone: user.phone,
            cpf: user.cpf,
            role: user.role,
            client_id: user.client_id,
            avatar_file_id: user.avatar_file_id,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
