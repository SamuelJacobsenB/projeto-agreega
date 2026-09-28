use super::{dtos::ClientResponseDto, models::Client};

impl From<Client> for ClientResponseDto {
    fn from(client: Client) -> Self {
        ClientResponseDto {
            id: client.id,
            company_name: client.company_name,
            document: client.document,
            phone: client.phone,
            email: client.email,
            address: client.address,
            city: client.city,
            state: client.state,
            created_at: client.created_at,
            updated_at: client.updated_at,
        }
    }
}
