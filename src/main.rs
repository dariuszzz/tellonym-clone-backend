mod pool;
use pool::Db;

mod query;

use migration::{MigratorTrait, DbErr};
use rocket::{fairing::{AdHoc, self}, Rocket, Build, serde::json::Json};
use serde::{Deserialize};
use sea_orm_rocket::{Database, Connection};
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::Set;

use entity::user::{self, Entity as User};

#[macro_use] extern crate rocket;

#[get("/user/<id>")]
async fn user_page(conn: Connection<'_, Db>, id: i32) -> Result<Json<Option<serde_json::Value>>, String> {
    let db = conn.into_inner();

    let user: Option<serde_json::Value> = User::find_by_id(id)
        .into_json()
        .one(db)
        .await
        .map_err(|_| String::from("database error"))?;

    Ok(Json(user))
}

#[derive(Deserialize)]
struct RegisterData<'a> {
    username: &'a str,
    password: &'a str,
}

#[post("/register", data = "<register_data>")]
async fn register(conn: Connection<'_, Db>, register_data: Json<RegisterData<'_>>) -> Result<(), String> {
    let RegisterData { username, password } = register_data.into_inner();
    let db = conn.into_inner();
    
    let user = user::ActiveModel {
        username: Set(username.to_string()),
        password: Set(password.to_string()),
        ..Default::default()
    };

    user.save(db).await.map_err(|_| String::from("db error"))?;
    
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![register, user_page])
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}