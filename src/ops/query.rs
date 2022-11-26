use super::*;

pub async fn user_by_id(db: DbType<'_>, user_id: i32) -> Result<user::Model, String> {

    let user: user::Model = User::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|_| String::from("Database error"))?
        .ok_or(String::from("User does not exist"))?;

    Ok(user)
}

pub async fn user_by_username(db: DbType<'_>, username: &str) -> Result<user::Model, String> {

    let user: user::Model = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|_| String::from("Database error"))?
        .ok_or(String::from("User does not exist"))?;

    Ok(user)
}

pub async fn questions_w_answers_by_asked_id(db: DbType<'_>, asked_id: i32) -> Result<Vec<QuestionDTO>, String> {
    let questions: Vec<(question::Model, Vec<answer::Model>)> = Question::find()
        .filter(question::Column::AskedId.eq(asked_id))
        .find_with_related(Answer)
        .all(db)
        .await
        .map_err(|e| e.to_string())?;

    let questions = questions.into_iter()
        .map(|(question, answer)| QuestionDTO { question, answer: answer.into_iter().next() } )
        .collect::<Vec<_>>();

    Ok(questions)
}

pub async fn question_w_answer_by_id(db: DbType<'_>, question_id: i32) -> Result<QuestionDTO, String> {
    let questions: Vec<(question::Model, Vec<answer::Model>)> = Question::find_by_id(question_id)
        .find_with_related(Answer)
        .all(db)
        .await
        .map_err(|_| String::from("database error"))?;

    let (question, answers) = questions.into_iter()
        .next()
        .ok_or(String::from("Question does not exist"))?;

    let question = QuestionDTO { question, answer: answers.into_iter().next() };

    Ok(question)
}