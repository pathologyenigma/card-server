use crate::entity::level_settings::ActiveModel;
use crate::ErrorHandlerWithErrorExtensions;
use crate::BadInputErrorHandler;
use async_graphql::CustomValidator;
use async_graphql::InputObject;
use sea_orm::Set;
use async_graphql::SimpleObject;
pub struct LevelValidator;
impl CustomValidator<Vec<String>> for LevelValidator {
    fn check(&self, value: &Vec<String>) -> Result<(), String> {
        if value.is_empty() {
            return Err("you need at least one level".to_string());
        }
        Ok(())
    }
}
#[derive(InputObject, Clone)]
pub struct NewLevelSetting {
    pub title: String,
    pub is_numberic_level: bool,
    #[graphql(validator(custom = "LevelValidator{}"))]
    pub levels: Vec<String>,
    pub counts: Option<i32>,
    pub tip_for_setting_user: Option<String>,
}

impl NewLevelSetting {
    pub fn to_model(self, user_id: i32) -> ActiveModel {
        ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            user_id: Set(user_id.to_owned()),
            title: Set(self.title.to_owned()),
            is_numberic_level: Set(self.is_numberic_level.to_owned()),
            levels: Set(serde_json::json!(self.levels)),
            counts: Set(self.counts.to_owned()),
            tip_for_setting_user: Set(self.tip_for_setting_user.unwrap().to_owned()),
        }
    }
    pub fn check_valid(
        &mut self,
        mut err_handler: BadInputErrorHandler,
    ) -> async_graphql::Result<()> {
        match self.is_numberic_level {
            true => match self.counts.clone() {
                Some(counts) => {
                    if self.levels.len() > 1 {
                        err_handler.append("levels".to_string(), "multiple levels is not allowed for numberic level, when it is a numberic level, the levels first value will be the name of your level like ['star'] will make your level be like one star, two star ...".to_string());
                        return Err(err_handler.to_err());
                    } else {
                        if self.tip_for_setting_user.is_none() {
                            let default_tip = format!("this is a numberic level setting with level name: {} and level counts {}, this means that levels in card pool will be 1 to {} {}s", self.levels[0], counts, counts, self.levels[0]);
                            self.tip_for_setting_user = Some(default_tip);
                        }
                    }
                }
                None => {
                    err_handler.append("counts is empty".to_string(), "numberic level should have counts field to represent how many levels it will be".to_string());
                    return Err(err_handler.to_err());
                }
            },
            false => {
                if self.tip_for_setting_user.is_none() {
                    let mut default_tip = "this is text level setting, your level will be ".to_string();
                    for level in self.levels.iter() {
                        default_tip = format!("{}, {}", default_tip, level);
                    }
                    self.tip_for_setting_user = Some(default_tip);
                }
            },
        }
        Ok(())
    }
}
#[derive(SimpleObject)]
pub struct LevelSetting {
    pub id: String,
    pub title: String,
    pub is_numberic_level: bool,
    pub levels: Vec<String>,
    pub counts: Option<i32>,
    pub tip_for_setting_user: Option<String>,
}