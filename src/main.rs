mod pool;
mod user_guard;
mod jwt_util;
mod cors;
mod routes;
mod ops;

use cors::CORS;
use pool::Db;

use rocket::{fairing::{AdHoc, self}, Rocket, Build};

use migration::MigratorTrait;
use sea_orm_rocket::Database;

#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv_codegen;

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

            routes::get_user, 
            routes::user_questions,
            routes::ask_question, 
            routes::users,
            routes::current_user,
            routes::follow_user,
            routes::user_followers,
            routes::user_follows,

            routes::answer_question, 
            routes::get_question,
        ])
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

