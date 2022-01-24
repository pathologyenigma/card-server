use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Token {
    id: i32,
    username: String,
    email: Option<String>,
}

impl From<crate::entity::users::Model> for Token {
    fn from(model: crate::entity::users::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
        }
    }
}

impl Token {
    pub fn new(id: i32, username: String, email: Option<String>) -> Self {
        Self {
            id,
            username,
            email,
        }
    }
    pub fn encode(self, secret: String) -> jsonwebtoken::errors::Result<String> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
    }
}
