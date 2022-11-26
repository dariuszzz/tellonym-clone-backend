
use migration::Condition;

use super::*;

#[must_use]
pub async fn user_by_id(db: DbType<'_>, user_id: i32) -> Result<user::Model, String> {

    let user: user::Model = User::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| format!("Database error when querying user by id: {}", e.to_string()))?
        .ok_or(String::from("User does not exist"))?;

    Ok(user)
}

#[must_use]
pub async fn user_by_username(db: DbType<'_>, username: &str) -> Result<user::Model, String> {

    let user: user::Model = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|e| format!("Database error when querying user by username: {}", e.to_string()))?
        .ok_or(String::from("User does not exist"))?;

    Ok(user)
}

#[must_use]
pub async fn questions_w_answers_by_asked_id(db: DbType<'_>, asked_id: i32) -> Result<Vec<QuestionDTO>, String> {
    let questions: Vec<(question::Model, Vec<answer::Model>)> = Question::find()
        .filter(question::Column::AskedId.eq(asked_id))
        .find_with_related(Answer)
        .all(db)
        .await
        .map_err(|e| format!("Database error when queying questions by asked_id: {}", e.to_string()))?;

    let questions = questions.into_iter()
        .map(|(question, answer)| QuestionDTO { question, answer: answer.into_iter().next() } )
        .collect::<Vec<_>>();

    Ok(questions)
}

#[must_use]
pub async fn question_w_answer_by_id(db: DbType<'_>, question_id: i32) -> Result<QuestionDTO, String> {
    let questions: Vec<(question::Model, Vec<answer::Model>)> = Question::find_by_id(question_id)
        .find_with_related(Answer)
        .all(db)
        .await
        .map_err(|e| format!("Database error when querying answer by id: {}", e.to_string()))?;

    let (question, answers) = questions.into_iter()
        .next()
        .ok_or(String::from("Question does not exist"))?;

    let question = QuestionDTO { question, answer: answers.into_iter().next() };

    Ok(question)
}

#[must_use]
pub async fn users_starting_with(db: DbType<'_>, search: &str) -> Result<Vec<user::Model>, String> {
    let users: Vec<user::Model> = User::find()
        .filter(user::Column::Username.like(&format!("{}%", search)))
        .all(db)
        .await
        .map_err(|e| format!("Database error when querying filtered users: {}", e.to_string()))?;

    Ok(users)
}

#[must_use]
pub async fn all_users(db: DbType<'_>) -> Result<Vec<user::Model>, String> {
    let users: Vec<user::Model> = User::find()
        .all(db)
        .await
        .map_err(|e| format!("Database error when querying all users: {}", e.to_string()))?;

    Ok(users)
}

#[must_use]
pub async fn follows_with_following_id(db: DbType<'_>, following_id: i32) -> Result<Vec<follow::Model>, String> {
    let follows: Vec<follow::Model> = Follow::find()
        .filter(follow::Column::FollowingId.eq(following_id))
        .all(db)
        .await
        .map_err(|e| format!("Database error when querying followers of user: {}", e))?;

    Ok(follows)
} 

#[must_use]
pub async fn follows_with_follower_id(db: DbType<'_>, follower_id: i32) -> Result<Vec<follow::Model>, String> {
    let follows: Vec<follow::Model> = Follow::find()
        .filter(follow::Column::FollowingId.eq(follower_id))
        .all(db)
        .await
        .map_err(|e| format!("Database error when querying users followed by another user: {}", e))?;

    Ok(follows)
} 

pub async fn follows_with_both_ids(db: DbType<'_>, follower_id: i32, following_id: i32) -> Result<Option<follow::Model>, String> {
    let follow = Follow::find()
        .filter(Condition::all()
            .add(follow::Column::FollowerId.eq(follower_id))
            .add(follow::Column::FollowingId.eq(following_id))
        )
        .one(db)
        .await
        .map_err(|e| format!("Database error when querying follows by both ids: {}", e.to_string()))?;

    Ok(follow)
}

#[must_use]
pub async fn users_with_ids(db: DbType<'_>, ids: &[i32]) -> Result<Vec<user::Model>, String> {
    let users = User::find()
        .filter(
            Expr::tbl(user::Column::Id.entity_name(), user::Column::Id).is_in(ids.to_vec())
        )
        .all(db)
        .await
        .map_err(|e| format!("Database error when querying users with ids: {}", e.to_string()))?;

    Ok(users)
}