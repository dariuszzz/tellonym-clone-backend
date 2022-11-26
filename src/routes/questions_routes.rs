use super::*;

#[derive(Deserialize)]
pub struct AnswerQuestionData<'a> {
    content: &'a str
}

#[post("/questions/<question_id>/answer", data = "<answer_data>")]
pub async fn answer_question(
    conn: Connection<'_, Db>, 
    user: UserGuard, 
    question_id: i32,
    answer_data: Json<AnswerQuestionData<'_>>
) -> Result<(), String> {
    let AnswerQuestionData { content } = answer_data.into_inner();
    let db = conn.into_inner();
    let username = user.into_inner();

    let user: user::Model = query::user_by_username(db, &username).await?;

    //This is here to check whether question (id: question_id) actually exists
    let QuestionDTO { question, .. } = query::question_w_answer_by_id(db, question_id).await?;

    //TODO: Add editing of questions

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

#[get("/questions/<question_id>")]
pub async fn get_question(conn: Connection<'_, Db>, question_id: i32) -> Result<Json<QuestionDTO>, String> {
    let db = conn.into_inner();

    let question_and_answer: QuestionDTO = query::question_w_answer_by_id(db, question_id).await?;

    Ok(Json(question_and_answer))
}