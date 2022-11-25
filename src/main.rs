mod pool;
use cors::CORS;
use pool::Db;

mod user_guard;
use user_guard::UserGuard;

mod jwt_util;
use jwt_util::JWTUtil;

mod cors;

use migration::{MigratorTrait, JoinType};
use rocket::{fairing::{AdHoc, self}, Rocket, Build, serde::json::Json, http::{CookieJar, Cookie, Method}};
use serde::{Deserialize};
use sea_orm_rocket::{Database, Connection};
use sea_orm::{ActiveModelTrait, FromQueryResult};
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::RelationTrait;
use sea_orm::QuerySelect;
use sea_orm::QueryTrait;

use entity::user::{self, Entity as User};
use entity::question::{self, Entity as Question};
use entity::answer::{self, Entity as Answer};

use bcrypt::{bcrypt, hash, verify};

#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;

#[get("/refresh")]
async fn refresh(cookies: &CookieJar<'_>) -> Result<String, String> {

    let refresh_token_cookie = cookies.get("refresh_token").ok_or(String::from("No refresh cookie"))?;
    let refresh_token = refresh_token_cookie.value();
 
    let username = JWTUtil::verify_refresh_jwt(refresh_token);

    if let None = username { return Err(String::from("Invalid refresh token")) }

    let access_jwt = JWTUtil::sign_access_jwt(&username.unwrap());

    Ok(access_jwt)
}

#[get("/users/<id>/questions")]
async fn user_questions(conn: Connection<'_, Db>, id: i32) -> Result<Json<Vec<serde_json::Value>>, String> {
    let db = conn.into_inner();

    let questions_and_answers: Vec<(question::Model, Vec<answer::Model>)> = Question::find()
    .filter(question::Column::AskedId.eq(id))
    .find_with_related(Answer)
    .all(db)
    .await
    .map_err(|_| String::from("Database error"))?;

    let questions = questions_and_answers.into_iter().map(
        |(question, answers)| serde_json::json!(
            {
                "question": question,
                "answer": answers.first(),
            }
        )
    ).collect::<Vec<_>>();

    Ok(Json(questions))
}

#[get("/users?<search>")]
async fn users(conn: Connection<'_, Db>, search: Option<&'_ str>) -> Result<Json<Vec<user::Model>>, String> {
    let db = conn.into_inner();

    match search{
        Some(search) => { 
            let users: Vec<user::Model> = User::find()
                .filter(user::Column::Username.like(&format!("{}%", search)))
                .all(db)
                .await
                .map_err(|_| String::from("Database error"))?;

            Ok(Json(users))
        }
        None => {
            let users: Vec<user::Model> = User::find()
                .all(db)
                .await
                .map_err(|_| String::from("Database error"))?;
            
            Ok(Json(users))
        }
    }

}

#[get("/users/<id>")]
async fn user_page(conn: Connection<'_, Db>, id: i32) -> Result<Json<user::Model>, String> {
    let db = conn.into_inner();

    let user: user::Model = User::find_by_id(id)
        .one(db)
        .await
        .map_err(|_| String::from("Database error"))?
        .ok_or(String::from("User does not exist"))?;

    Ok(Json(user))
}

#[get("/user")]
async fn current_user(conn: Connection<'_, Db>, user: UserGuard) -> Result<Json<user::Model>, String> {
    let db = conn.into_inner();
    let username = user.into_inner();

    let user: user::Model = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|_| String::from("Database error"))?
        .ok_or(String::from("User is not logged int"))?;

    Ok(Json(user))
}

#[derive(Deserialize)]
struct AnswerQuestionData<'a> {
    question_id: i32,
    content: &'a str
}

#[post("/answer", data = "<answer_data>")]
async fn answer_question(
    conn: Connection<'_, Db>, 
    user: UserGuard, 
    answer_data: Json<AnswerQuestionData<'_>>
) -> Result<(), String> {
    let AnswerQuestionData { question_id, content } = answer_data.into_inner();
    let db = conn.into_inner();
    let username = user.into_inner();

    let user: user::Model = User::find()
        .filter(user::Column::Username.eq(username))    
        .one(db)
        .await
        .map_err(|_| String::from("database error"))?
        .ok_or(String::from("User does not exist"))?;

    //This is here to check whether question (id: question_id) actually exists
    let question: question::Model = Question::find_by_id(question_id)
        .one(db)
        .await
        .map_err(|_| String::from("database error"))?
        .ok_or(String::from("User does not exist"))?;

    if question.asked_id != user.id { return Err(String::from("You are not allowed to answer this question")) }
    
    let now = chrono::offset::Utc::now().naive_utc();

    let answer = answer::ActiveModel {
        question_id: Set(question_id),
        content: Set(content.to_string()),
        answered_at: Set(now),
        last_edit_at: Set(now),
        ..Default::default()
    };

    answer.save(db)
        .await
        .map_err(|_| String::from("Database error"))?;


    Ok(())
}

#[derive(Deserialize)]
struct AskQuestionData<'a> {
    asked_id: i32,
    content: &'a str,
}

#[post("/ask", data = "<question_data>")]
async fn ask_question(conn: Connection<'_, Db>, user: UserGuard, question_data: Json<AskQuestionData<'_>>) -> Result<(), String> {
    let AskQuestionData { asked_id, content } = question_data.into_inner();
    let username = user.into_inner();
    let db = conn.into_inner();

    let user_asking_question: user::Model = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|_| String::from("database error"))?
        .ok_or(String::from("User (Asking) does not exist"))?;


    //this is here to check whether the user (id: asked_id) actually exists
    let user_being_asked: user::Model = User::find_by_id(asked_id)
        .one(db)
        .await
        .map_err(|_| String::from("database error"))?
        .ok_or(String::from("User (Being asked) does not exist"))?;

    if user_being_asked.id == user_asking_question.id { return Err(String::from("You cannot ask yourself a question")) }

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
async fn register(
    cookies: &CookieJar<'_>,
    conn: Connection<'_, Db>, 
    register_data: Json<LoginData<'_>>
) -> Result<String, String> {
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

    let access_jwt = JWTUtil::sign_access_jwt(&username);
    let refresh_jwt = JWTUtil::sign_refresh_jwt(&username);

    cookies.add(Cookie::build("refresh_token", refresh_jwt).http_only(true).finish());

    Ok(access_jwt)
}

#[post("/login", data = "<login_data>")]
async fn login(
    cookies: &CookieJar<'_>,
    conn: Connection<'_, Db>, 
    login_data: Json<LoginData<'_>>
) -> Result<String, String> {
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

    let access_jwt = JWTUtil::sign_access_jwt(&username);
    let refresh_jwt = JWTUtil::sign_refresh_jwt(&username);

    cookies.add(Cookie::build("refresh_token", refresh_jwt).http_only(true).finish());

    Ok(access_jwt)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![
            register, 
            login, 
            user_page, 
            user_questions,
            ask_question, 
            answer_question, 
            refresh,
            users,
            current_user
        ])
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

