use super::*;

#[get("/users/<user_id>")]
pub async fn get_user(conn: Connection<'_, Db>, user_id: i32) -> Result<Json<user::Model>, String> {
    let db = conn.into_inner();

    let user: user::Model = query::user_by_id(db, user_id).await?;

    Ok(Json(user))
}

#[get("/me")]
pub async fn current_user(conn: Connection<'_, Db>, user: UserGuard) -> Result<Json<user::Model>, String> {
    let db = conn.into_inner();
    let username = user.into_inner();

    let user: user::Model = query::user_by_username(db, &username).await?;

    Ok(Json(user))
}

#[get("/users?<search>")]
pub async fn users(conn: Connection<'_, Db>, search: Option<&'_ str>) -> Result<Json<Vec<user::Model>>, String> {
    let db = conn.into_inner();

    match search{
        Some(search) => { 

            //Username starts with `search`
            let users: Vec<user::Model> = User::find()
                .filter(user::Column::Username.like(&format!("{}%", search)))
                .all(db)
                .await
                .map_err(|_| String::from("Database error"))?;

            Ok(Json(users))
        }
        None => {
            //All users
            let users: Vec<user::Model> = User::find()
                .all(db)
                .await
                .map_err(|_| String::from("Database error"))?;
            
            Ok(Json(users))
        }
    }

}

#[derive(Deserialize)]
pub struct AskQuestionData<'a> {
    anonymous: bool,
    content: &'a str,
}

#[post("/users/<asked_id>/ask", data = "<question_data>")]
pub async fn ask_question(conn: Connection<'_, Db>, user: UserGuard, asked_id: i32,  question_data: Json<AskQuestionData<'_>>) -> Result<(), String> {
    let AskQuestionData { anonymous,  content } = question_data.into_inner();
    let username = user.into_inner();
    let db = conn.into_inner();

    let user_asking_question: user::Model = query::user_by_username(db, &username).await?;

    //this is here to check whether the user (id: asked_id) actually exists
    let user_being_asked: user::Model = query::user_by_id(db, asked_id).await?;

    if user_being_asked.id == user_asking_question.id { return Err(String::from("You cannot ask yourself a question")) }

    let question = question::ActiveModel {
        content: Set(content.to_string()),
        asked_id: Set(asked_id),
        asker_id: Set(if anonymous { None } else { Some(user_asking_question.id) }),
        asked_at: Set(chrono::offset::Utc::now().naive_utc()),
        ..Default::default()
    };

    question.insert(db)
        .await
        .map_err(|_| String::from("Database error"))?;

    Ok(())
}

#[get("/users/<user_id>/questions")]
pub async fn user_questions(conn: Connection<'_, Db>, user_id: i32) -> Result<Json<Vec<(question::Model, Option<answer::Model>)>>, String> {
    let db = conn.into_inner();

    let questions_and_answers = query::questions_w_answers_by_asked_id(db, user_id).await?;

    Ok(Json(questions_and_answers))
}

//TODO: 
#[get("/users/<id>/follows")]
pub async fn user_follows(conn: Connection<'_, Db>, id: i32) -> Result<Json<Vec<serde_json::Value>>, String> {
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


//TODO: 
#[get("/users/<id>/followers")]
pub async fn user_followers(conn: Connection<'_, Db>, id: i32) -> Result<Json<Vec<serde_json::Value>>, String> {
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

//TODO: 
#[post("/users/<id>/follow")]
pub async fn follow_user(conn: Connection<'_, Db>, user: UserGuard, id: i32) -> Result<Json<Vec<serde_json::Value>>, String> {
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

//TODO: 
#[post("/users/<id>/unfollow")]
pub async fn unfollow_user(conn: Connection<'_, Db>, user: UserGuard, id: i32) -> Result<Json<Vec<serde_json::Value>>, String> {
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




