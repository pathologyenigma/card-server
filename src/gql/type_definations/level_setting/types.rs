use crate::entity::level_settings::ActiveModel;
use crate::BadInputErrorHandler;
use crate::ErrorHandlerWithErrorExtensions;
use async_graphql::CustomValidator;
use async_graphql::InputObject;
use async_graphql::SimpleObject;
use redis::FromRedisValue;
use redis::ToRedisArgs;
use sea_orm::Set;
use serde_json::json;
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
/// input type for create new level setting
pub struct NewLevelSetting {
    /// the name of your level setting
    /// one user could have two levels of the same name
    pub title: String,
    /// told system that you want a numberic one or a text one
    pub is_numberic_level: bool,
    #[graphql(validator(custom = "LevelValidator{}"))]
    /// the level names
    /// if you are adding a numberic level
    /// gives one value for the name of your level
    /// like star for 1 star 2 star 3 star kind of level
    /// else if you are adding a text level
    /// those should just be your level names
    /// like ["n", "r", "sr", "ssr", "ur"]
    /// for n, r, sr, ssr, ur
    /// ps: the level names just texts
    /// you need to binding probabilities and algorithms
    /// to make the rare one to be rare
    /// or you can make the rare one come out more often
    pub levels: Vec<String>,
    /// how many levels you want for a numberic level
    /// this will be ignored if you choose text one
    pub counts: Option<i32>,
    /// the description of this level setting
    /// you can add things you want to descript this
    /// or leave this null to get default description
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
                self.counts = None;
                if self.tip_for_setting_user.is_none() {
                    let mut default_tip =
                        "this is text level setting, your level will be ".to_string();
                    for level in self.levels.iter() {
                        default_tip = format!("{}, {}", default_tip, level);
                    }
                    self.tip_for_setting_user = Some(default_tip);
                }
            }
        }
        Ok(())
    }
}
#[derive(SimpleObject, Serialize, Deserialize)]
/// insert result for adding a new setting
pub struct LevelSetting {
    /// the uuid that you will got when done insert
    pub id: String,
    /// will be the same as the title you gives
    pub title: String,
    pub is_numberic_level: bool,
    pub levels: Vec<String>,
    pub counts: Option<i32>,
    pub tip_for_setting_user: String,
}

impl From<crate::entity::level_settings::Model> for LevelSetting {
    fn from(value: crate::entity::level_settings::Model) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            is_numberic_level: value.is_numberic_level,
            levels: value
                .levels
                .as_array()
                .unwrap()
                .into_iter()
                .map(|item| item.as_str().unwrap().to_string())
                .collect(),
            counts: value.counts,
            tip_for_setting_user: value.tip_for_setting_user,
        }
    }
}
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct LevelSettingPage {
    pub data: Vec<LevelSetting>,
    pub page: u32,
    pub page_size: u64,
    pub user_id: i32,
}
impl ToRedisArgs for LevelSettingPage {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg_fmt(json!(self))
    }
}

impl FromRedisValue for LevelSettingPage {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        if let redis::Value::Data(value) = v {
            let data = String::from_utf8(value.to_vec()).unwrap();
            let res: LevelSettingPage = serde_json::from_str(data.as_str()).unwrap();
            return Ok(res);
        }
        Err(redis::RedisError::from((redis::ErrorKind::TypeError, "")))
    }
}
