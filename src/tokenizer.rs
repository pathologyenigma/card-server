use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fn decode(token: String, secret: String) -> async_graphql::Result<Self> {
        let res = jsonwebtoken::decode::<Self>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        res.map(|token| token.claims).map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                crate::new_not_authenticated_error("InvalidToken".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                crate::new_not_authenticated_error("InvalidSignature".to_string())
            }
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                crate::new_not_authenticated_error("ExpiredSignature".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidAudience => {
                crate::new_not_authenticated_error("InvalidAudience".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidSubject => {
                crate::new_not_authenticated_error("InvalidSubject".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                crate::new_not_authenticated_error("InvalidIssuer".to_string())
            }
            jsonwebtoken::errors::ErrorKind::ImmatureSignature => {
                crate::new_not_authenticated_error("ImmatureSignature".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidEcdsaKey => {
                crate::new_internal_server_error("InvalidEcdsaKey".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidRsaKey => {
                crate::new_internal_server_error("InvalidRsaKey".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName => {
                crate::new_internal_server_error("InvalidAlgorithmName".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidKeyFormat => {
                crate::new_internal_server_error("InvalidKeyFormat".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => {
                crate::new_internal_server_error("InvalidAlgorithm".to_string())
            }
            jsonwebtoken::errors::ErrorKind::Base64(_)
            | jsonwebtoken::errors::ErrorKind::Json(_)
            | jsonwebtoken::errors::ErrorKind::Utf8(_)
            | jsonwebtoken::errors::ErrorKind::Crypto(_) => crate::new_not_authenticated_error(
                "failed to parse token to json, check your token".to_string(),
            ),
            _ => crate::new_internal_server_error("unknown error".to_string()),
        })
    }
}
pub async fn on_connection_init(value: serde_json::Value) -> async_graphql::Result<async_graphql::Data> {
    #[derive(Deserialize)]
    struct Payload {
        token: String,
    }

    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = async_graphql::Data::default();
        data.insert(crate::TokenFromHeader(payload.token));
        Ok(data)
    } else {
        Err("Token is required".into())
    }
}