mod pool;
mod user_guard;
mod jwt_util;
mod cors;
mod routes;
mod ops;
mod error;

use std::path::Path;

use cors::CORS;
use pool::Db;

use rocket::{fairing::{AdHoc, self}, Rocket, Build, fs::{FileServer, NamedFile}};

use migration::MigratorTrait;
use sea_orm_rocket::Database;
use rocket::fs::relative;


#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;

#[get("/pfps/<name>")]
async fn get_pfp(name: &str) -> NamedFile {
    let path = Path::new(relative!("pfps")).join(name);
    let default_path = Path::new(relative!("pfps")).join("0.jpg");

    match NamedFile::open(path).await {
        Ok(file) => file,
        Err(_) => NamedFile::open(default_path).await.unwrap()
    }
}

#[get("/bgs/<name>")]
async fn get_bg(name: &str) -> NamedFile {
    let path = Path::new(relative!("bgs")).join(name);
    let default_path = Path::new(relative!("bgs")).join("0.jpg");

    match NamedFile::open(path).await {
        Ok(file) => file,
        Err(_) => NamedFile::open(default_path).await.unwrap()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![
            routes::register, 
            routes::login, 
            routes::refresh,
            routes::logout,

            routes::get_user, 
            routes::user_questions,
            routes::ask_question, 
            routes::users,
            routes::current_user,
            routes::follow_user,
            routes::user_followers,
            routes::user_follows,
            routes::edit_profile,
            routes::get_homepage,

            routes::answer_question, 
            routes::get_question,
            routes::vote_answer,
            routes::vote_question,

            get_pfp,
            get_bg,
        ])
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

