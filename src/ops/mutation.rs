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

    let follower_following_count: u32 = follower.following_count.take()
        .unwrap_or(0);
    let following_follower_count: u32 = following.follower_count.take()
        .unwrap_or(0);

    follower.following_count = ActiveValue::Set((follower_following_count as i32 + change) as u32);
    following.follower_count = ActiveValue::Set((following_follower_count as i32 + change) as u32);

    follower = mutation::update_user(db, follower).await?;
    following = mutation::update_user(db, following).await?;
    
    Ok((follower, following))
}

pub async fn delete_like(db: DbType<'_>, like: like::Model) -> Result<DeleteResult, TellonymError> {
    let res = like.delete(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))?;

    Ok(res)
}

pub async fn update_like(db: DbType<'_>, like: like::ActiveModel) -> Result<like::ActiveModel, TellonymError> {
    like.save(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))
}

pub async fn change_question_vote(db: DbType<'_>, question: question::Model, change: i32) -> Result<question::ActiveModel, TellonymError> {        
    let likes = question.likes.clone();

    let mut active_question: question::ActiveModel = question.into();
    active_question.likes = ActiveValue::Set(likes + change);

    active_question.save(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))
    
}

pub async fn change_answer_vote(db: DbType<'_>, answer: answer::Model, change: i32) -> Result<answer::ActiveModel, TellonymError> {
    let likes = answer.likes.clone();

    let mut active_answer: answer::ActiveModel = answer.into();
    active_answer.likes = ActiveValue::Set(likes + change);

    active_answer.save(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))
}