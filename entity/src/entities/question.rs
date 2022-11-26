use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "questions")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: String,
    pub likes: i32,
    pub asked_id: i32,
    pub asker_id: Option<i32>,
    pub asked_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
         belongs_to = "super::user::Entity",
         from = "Column::AskedId",
         to = "super::user::Column::Id"
    )]
    AskedUser,
    #[sea_orm(has_one = "super::answer::Entity")]
    Answer
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AskedUser.def()
    }
}

impl Related<super::answer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Answer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
