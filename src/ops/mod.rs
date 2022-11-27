pub mod mutation;
pub use mutation::*;

pub mod query;
pub use query::*;

pub use sea_orm_rocket::Connection;
use sea_orm_rocket::{Pool, Database};
use serde::{Deserialize, Serialize};

pub use super::pool::Db;

pub use entity::user::{self, Entity as User};
pub use entity::question::{self, Entity as Question};
pub use entity::answer::{self, Entity as Answer};
pub use entity::follow::{self, Entity as Follow};

use sea_orm::EntityTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue;
use sea_orm::DeleteResult;
use sea_orm::ModelTrait;

use migration::Expr;

use crate::error::TellonymError;

type DbType<'a> = &'a <<Db as Database>::Pool as Pool>::Connection;

#[derive(Serialize, Deserialize)]
pub struct QuestionDTO {
    pub question: question::Model,
    pub answer: Option<answer::Model>
}