use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "answers")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i32,
    pub question_id: i32,
    pub content: String,
    pub likes: i32,
    pub answered_at: chrono::NaiveDateTime,
    pub last_edit_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Question,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Question => Entity::belongs_to(super::question::Entity)
                .from(Column::QuestionId)
                .to(super::question::Column::Id)
                .into(),
        }
    }
}

impl Related<super::question::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Question.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
