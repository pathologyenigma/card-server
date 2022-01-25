use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub exp: usize,
}
fn default_exp() -> usize {
    std::time::SystemTime::now()
        .checked_add(std::time::Duration::from_secs(3600))
        .unwrap()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("failed to generate exp")
        .as_secs() as usize
}
impl From<crate::entity::users::Model> for Token {
    fn from(model: crate::entity::users::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            exp: default_exp(),
        }
    }
}

impl Token {
    pub fn new(id: i32, username: String, email: Option<String>) -> Self {
        Self {
            id,
            username,
            email,
            exp: default_exp(),
        }
    }
    pub fn encode(self, secret: String) -> jsonwebtoken::errors::Result<String> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
    }
    pub fn decode(token: String, secret: String) -> jsonwebtoken::errors::Result<Self> {
        let res = jsonwebtoken::decode::<Self>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        res.map(|token| token.claims)
    }
}
