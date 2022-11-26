

use super::*;

pub async fn register_user(db: DbType<'_>, user: user::ActiveModel) -> Result<user::Model, String> {
    let user = user.insert(db)
        .await
        .map_err(|e| format!("Database error when inserting user: {}", e.to_string()))?;

    Ok(user)
}

pub async fn add_question(db: DbType<'_>, question: question::ActiveModel) -> Result<question::Model, String> {
    let question = question.insert(db)
        .await
        .map_err(|e| format!("Database error when inserting question: {}", e.to_string()))?;

    Ok(question)
}

pub async fn add_or_edit_answer(db: DbType<'_>, answer: answer::ActiveModel) -> Result<answer::ActiveModel, String> {
    let answer = answer.save(db)
        .await
        .map_err(|e| format!("Database error when updating answer: {}", e.to_string()))?;

    Ok(answer)
}

pub async fn insert_follow(db: DbType<'_>, follow: follow::ActiveModel) -> Result<follow::Model, String> {
    let follow = follow.insert(db)
        .await
        .map_err(|e| format!("Database error when inserting follow: {}", e.to_string()))?;

    Ok(follow)
}

pub async fn delete_follow(db: DbType<'_>, follow: follow::Model) -> Result<DeleteResult, String> {
    let res = follow.delete(db)
        .await
        .map_err(|e| format!("Database error when deleting follow: {}", e.to_string()))?;

    Ok(res)
}

pub async fn update_user(db: DbType<'_>, user: user::ActiveModel) -> Result<user::ActiveModel, String>{
    let user = user.save(db)
        .await
        .map_err(|e| format!("Database error when updating user: {}", e.to_string()))?;

    Ok(user)
}

//TODO: refactor names
pub async fn change_follow_counts(
    db: DbType<'_>,
    follower: user::Model,
    following: user::Model,
    change: i32 
) -> Result<(user::ActiveModel, user::ActiveModel), String> {
    
    let mut follower: user::ActiveModel = follower.into();
    let mut following: user::ActiveModel = following.into();

    let follower_following_count: i32 = follower.following_count.take()
        .unwrap_or(0);
    let following_follower_count: i32 = following.follower_count.take()
        .unwrap_or(0);

    follower.following_count = ActiveValue::Set(follower_following_count + change);
    following.follower_count = ActiveValue::Set(following_follower_count + change);

    follower = mutation::update_user(db, follower).await?;
    following = mutation::update_user(db, following).await?;
    
    Ok((follower, following))
}