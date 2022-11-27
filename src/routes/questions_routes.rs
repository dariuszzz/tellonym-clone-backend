use rocket::http::Status;

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
) -> Result<Status, TellonymError> {
    let AnswerQuestionData { content } = answer_data.into_inner();
    let db = conn.into_inner();
    let username = user.into_inner();

    let user: user::Model = query::user_by_username(db, &username).await?;

    //This is here to check whether question (id: question_id) actually exists
    let QuestionDTO { question, answer } = query::question_w_answer_by_id(db, question_id).await?;

    //TODO: Add editing of questions

    if question.asked_id != user.id { return Err(TellonymError::ConstraintError) }
    
    let now = chrono::offset::Utc::now().naive_utc();

    let answer: answer::ActiveModel = match answer {
        Some(answer) => {
            let mut ans: answer::ActiveModel = answer.into();

            ans.content = Set(content.to_string());
            ans.last_edit_at = Set(now);
            
            ans
        },
        None => {
            answer::ActiveModel {
                question_id: Set(question_id),
                content: Set(content.to_string()),
                answered_at: Set(now),
                last_edit_at: Set(now),
                ..Default::default()
            }
        }
    };

    mutation::add_or_edit_answer(db, answer).await?;

    Ok(Status::Created)
}

#[get("/questions/<question_id>")]
pub async fn get_question(conn: Connection<'_, Db>, question_id: i32) -> Result<Json<QuestionDTO>, TellonymError> {
    let db = conn.into_inner();

    let question_and_answer: QuestionDTO = query::question_w_answer_by_id(db, question_id).await?;

    Ok(Json(question_and_answer))
}