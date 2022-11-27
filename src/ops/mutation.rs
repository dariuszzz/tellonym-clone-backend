

use super::*;

pub async fn register_user(db: DbType<'_>, user: user::ActiveModel) -> Result<user::Model, TellonymError> {
    let user = user.insert(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))?;

    Ok(user)
}

pub async fn add_question(db: DbType<'_>, question: question::ActiveModel) -> Result<question::Model, TellonymError> {
    let question = question.insert(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))?;

    Ok(question)
}

pub async fn add_or_edit_answer(db: DbType<'_>, answer: answer::ActiveModel) -> Result<answer::ActiveModel, TellonymError> {
    let answer = answer.save(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))?;

    Ok(answer)
}

pub async fn insert_follow(db: DbType<'_>, follow: follow::ActiveModel) -> Result<follow::Model, TellonymError> {
    let follow = follow.insert(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))?;

    Ok(follow)
}

pub async fn delete_follow(db: DbType<'_>, follow: follow::Model) -> Result<DeleteResult, TellonymError> {
    let res = follow.delete(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))?;

    Ok(res)
}

pub async fn update_user(db: DbType<'_>, user: user::ActiveModel) -> Result<user::ActiveModel, TellonymError>{
    let user = user.save(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))?;

    Ok(user)
}

//TODO: refactor names
pub async fn change_follow_counts(
    db: DbType<'_>,
    follower: user::Model,
    following: user::Model,
    change: i32 
) -> Result<(user::ActiveModel, user::ActiveModel), TellonymError> {
    
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