use crate::entity::card::ActiveModel;
use crate::traits::prelude::ToModel;
use crate::BadInputErrorHandler;
use async_graphql::{CustomValidator, InputObject, SimpleObject};
use sea_orm::Set;
use serde::{Deserialize, Serialize};
pub struct CardNameValidator;
impl CustomValidator<String> for CardNameValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if value.trim().is_empty() {
            return Err("empty name not allowed".to_string());
        }
        if value.trim().len() > 20 {
            return Err("card name too long".to_string());
        }
        Ok(())
    }
}

#[derive(InputObject, Clone)]
pub struct NewCard {
    #[graphql(validator(custom = "CardNameValidator{}"))]
    /// name of the card, should not be longer than 20 characters
    pub name: String,
    /// the description about this card
    /// just anything you like to describe this card
    /// but don't using too much dirty words
    /// that may send me to prison cause my country really cares about it
    pub description: String,
    /// any picture to display in the body of the card
    /// make sure you have the copyright of the picture
    /// this stuff could also send me to the prison
    pub logo: Option<String>,
    /// so effects that this card should have
    /// more like extra datas
    /// will have an official format in the future version
    /// now it will be ignored
    pub effects: Option<String>,
}
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct Card {
    pub id: uuid::Uuid,
    /// name of the card, would not be longer than 20 characters
    pub name: String,
    /// the description about this card
    pub description: String,
    /// the picture to display in the body of the card
    pub logo: Option<String>,
    /// so effects that this card should have
    /// more like extra datas
    /// will have an official format in the future version
    /// now it will be ignored
    pub effects: Option<String>,
}

impl ToModel for NewCard {
    type Args = i32;
    type Output = ActiveModel;
    fn to_model(self, args: Self::Args) -> Self::Output {
        ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            name: Set(self.name),
            description: Set(self.description),
            logo: Set(self.logo),
            effects: Set(self.effects),
            created_by: Set(args),
        }
    }
}

impl NewCard {
    /// this function is for check if the effects is valid
    /// for now we don't have a rule for it
    /// so will always return Ok(())
    pub fn check_valid(&self, mut _err_handler: BadInputErrorHandler) -> async_graphql::Result<()> {
        Ok(())
    }
}
