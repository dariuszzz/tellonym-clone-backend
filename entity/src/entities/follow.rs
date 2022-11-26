use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[serde(skip_deserializing)]
    #[sea_orm(primary_key)]
    pub id: i32,
    pub follower_id: i32,
    pub following_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
