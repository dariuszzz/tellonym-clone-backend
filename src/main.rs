mod pool;
use bcrypt::{bcrypt, hash, verify};
use jwt_util::JWTUtil;
use pool::Db;

mod user_guard;
mod jwt_util;

use migration::{MigratorTrait};
use rocket::{fairing::{AdHoc, self}, Rocket, Build, serde::json::Json};
use serde::{Deserialize};
use sea_orm_rocket::{Database, Connection};
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

use entity::user::{self, Entity as User};
use entity::question::{self, Entity as Question};
use user_guard::UserGuard;

#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;



#[get("/user/<id>")]
async fn user_page(conn: Connection<'_, Db>, id: i32) -> Result<Json<Vec<(user::Model, Vec<question::Model>)>>, String> {
    let db = conn.into_inner();

    let user: Vec<(user::Model, Vec<question::Model>)> = User::find_by_id(id)
        .find_with_related(Question)
        .all(db)
        .await
        .map_err(|_| String::from("database error"))?;

    Ok(Json(user))
}

#[derive(Deserialize)]
struct AskQuestionData<'a> {
    asked_id: i32,
    content: &'a str
}

#[post("/ask", data = "<question_data>")]
async fn ask_question(conn: Connection<'_, Db>, _user: UserGuard, question_data: Json<AskQuestionData<'_>>) -> Result<(), String> {
    let AskQuestionData { asked_id, content } = question_data.into_inner();
    let db = conn.into_inner();

    let user: Option<user::Model> = User::find_by_id(asked_id)
        .one(db)
        .await
        .map_err(|_| String::from("database error"))?;

    let user = user.ok_or(String::from("User does not exist"))?;

    let question = question::ActiveModel {
        content: Set(content.to_string()),
        asked_id: Set(asked_id),
        asked_at: Set(chrono::offset::Utc::now().naive_utc()),
        ..Default::default()
    };

    question.insert(db)
        .await
        .map_err(|_| String::from("Database error"))?;



    Ok(())
}


#[derive(Deserialize)]
struct LoginData<'a> {
    username: &'a str,
    password: &'a str,
}

#[post("/register", data = "<register_data>")]
async fn register(conn: Connection<'_, Db>, register_data: Json<LoginData<'_>>) -> Result<String, String> {
    let LoginData { username, password } = register_data.into_inner();
    let db = conn.into_inner();
    
    let hashed_pass = hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| String::from("Hashing error"))?;

    let user = user::ActiveModel {
        username: Set(username.to_string()),
        password: Set(hashed_pass), 
        ..Default::default()
    };
    
    user.insert(db)
        .await
        .map_err(|_| String::from("Database error"))?;
    
    let jwt = JWTUtil::sign_jwt(username.to_string());

    Ok(jwt)
}

#[post("/login", data = "<login_data>")]
async fn login(conn: Connection<'_, Db>, login_data: Json<LoginData<'_>>) -> Result<String, String> {
    let LoginData { username, password } = login_data.into_inner();
    let db = conn.into_inner();

    let user: Option<user::Model> = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|_| String::from("Database error"))?;

    if let None = user {
        return Err(String::from("Incorrect credentials"))
    }

    let valid = verify(password, &user.unwrap().password)
        .map_err(|_| String::from("Veryfing error"));

    if valid.unwrap() == false {
        return Err(String::from("Invalid token"))
    }

    let jwt = JWTUtil::sign_jwt(username.to_string());

    Ok(jwt)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![register, login, user_page, ask_question])
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

