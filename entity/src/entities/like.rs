use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "likes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub resource_id: i32,
    #[sea_orm(primary_key)]
    pub liker_id: i32,
    pub like_type: LikeType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::LikerId",
        to = "super::user::Column::Id"
    )]
    LikerUser
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LikerUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Copy, Clone, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "liketype")]
pub enum LikeType {
    #[sea_orm(string_value = "QDislike")]
    QuestionDislike,
    #[sea_orm(string_value = "ADislike")]
    AnswerDislike,
    #[sea_orm(string_value = "QLike")]
    QuestionLike,
    #[sea_orm(string_value = "ALike")]
    AnswerLike,
}

impl LikeType {
    pub fn opposite_type(self) -> Self {
        match self {
            LikeType::AnswerDislike => LikeType::AnswerLike,
            LikeType::AnswerLike => LikeType::AnswerDislike,
            LikeType::QuestionDislike => LikeType::QuestionLike,
            LikeType::QuestionLike => LikeType::QuestionDislike,
        }
    }
}