mod pool;
use pool::Db;

use migration::{MigratorTrait, DbErr};
use rocket::{fairing::{AdHoc, self}, Rocket, Build, response::content::RawJson};
use sea_orm_rocket::{Database, Connection};
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::ActiveValue::Set;

use entity::test_num::{Entity as TestNum, self};

#[macro_use] extern crate rocket;

#[get("/")]
async fn index(conn: Connection<'_, Db>) -> Result<String, String> {
    let db = conn.into_inner();
    let nums: Vec<test_num::Model> = TestNum::find()
        .all(db)
        .await
        .map_err(|_| String::from("Error"))?;
    
    let concated = nums
        .into_iter()
        .map(|model| model.num.to_string())
        .reduce(|acc, num| acc + " " + num.as_ref())
        .unwrap();

    Ok(concated)
}

#[get("/add/<num>")]
async fn add(conn: Connection<'_, Db>, num: i32) -> Result<(), String> {
    let db = conn.into_inner();

    test_num::ActiveModel {
        num: Set(num),
        ..Default::default()
    }
    .save(db)
    .await
    .map_err(|_| String::from("Error"))?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![add, index])
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}