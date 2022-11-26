use rocket::{serde::json::Json, http::{CookieJar, Cookie}};
use serde::Deserialize;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

use super::ops::*;

use super::jwt_util::JWTUtil;
use super::user_guard::UserGuard;

use bcrypt::{hash, verify};

pub mod user_routes;
pub mod login_routes;
pub mod questions_routes;
pub use user_routes::*;
pub use login_routes::*;
pub use questions_routes::*;