use async_graphql::{InputObject, CustomValidator};
pub struct CardNameValidator;
impl CustomValidator<String> for CardNameValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if value.trim().is_empty() {
            return Err("empty name not allowed".to_string())
        }
        if value.trim().len() > 20 {
            return Err("card name too long".to_string())
        }
        Ok(())
    }
}

#[derive(InputObject)]
pub struct NewCard {
    #[graphql(validator(custom = "CardNameValidator{}"))]
    name: String,
    description: String,
    logo: String,
}
