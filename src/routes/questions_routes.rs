

use entity::like::LikeType;

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
    let user_id = user.into_inner();

    let user: user::Model = query::user_by_id(db, user_id).await?;

    //This is here to check whether question (id: question_id) actually exists
    let QuestionDTO { question, answer } = query::question_w_answer_by_id(db, question_id).await?;

    //TODO: Add editing of questions

    if question.asked_id != user.id { return Err(TellonymError::ConstraintError(String::from("You can not answer questions not intended for you"))) }
    
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
pub async fn get_question(
    conn: Connection<'_, Db>, 
    question_id: i32
) -> Result<Json<QuestionDTO>, TellonymError> {
    let db = conn.into_inner();

    let question_and_answer: QuestionDTO = query::question_w_answer_by_id(db, question_id).await?;

    Ok(Json(question_and_answer))
}

#[derive(Deserialize)]
pub struct VoteData {
    is_like: bool,
}

#[post("/questions/<question_id>/vote_question", data = "<vote_data>")]
pub async fn vote_question(
    conn: Connection<'_, Db>, 
    user: UserGuard, 
    question_id: i32, 
    vote_data: Json<VoteData>
) -> Result<Status, TellonymError> {
    let db = conn.into_inner();
    let VoteData { is_like  } = vote_data.into_inner();
    let user_id = user.into_inner();

    let user = query::user_by_id(db, user_id).await?;
    let QuestionDTO { question, .. } = query::question_w_answer_by_id(db, question_id).await?;

    let request_like_type = if is_like { LikeType::QuestionLike } else { LikeType::QuestionDislike };

    let like = query::exact_like(db, user.id, request_like_type, question.id).await?;
    
    match like {
        Some(like) => { 
            mutation::delete_like(db, like).await?;

            if is_like {
                change_question_vote(db, question, -1).await?;
            } else {
                change_question_vote(db, question, 1).await?;
            }
            // -1 do question if is_like
            // +1 do question if not is_like
        }
        None => {
            let opposite_like = query::exact_like(db, user.id, request_like_type.opposite_type(), question_id).await?;
            
            if let Some(like) = opposite_like {
                let mut active_like: like::ActiveModel = like.into();
        
                active_like.like_type = Set(request_like_type);
    
                mutation::update_like(db, active_like).await?;
                
                // +2 do question if is_like
                // -2 do question if not is_like
                
                if is_like {
                    change_question_vote(db, question, 2).await?;
                } else {
                    change_question_vote(db, question, -2).await?;
                }

            } else {
                let new_like = like::ActiveModel {
                    liker_id: Set(user.id),
                    like_type: Set(request_like_type),
                    resource_id: Set(question_id),
                    ..Default::default()
                };

                mutation::update_like(db, new_like).await?;

                // +1 do question if is_like
                // -1 do question if not is_like
                if is_like {
                    change_question_vote(db, question, 1).await?;
                } else {
                    change_question_vote(db, question, -1).await?;
                }
            }
        }
    }

    Ok(Status::Created)
}


#[post("/questions/<question_id>/vote_answer", data = "<vote_data>")]
pub async fn vote_answer(
    conn: Connection<'_, Db>, 
    user: UserGuard, 
    question_id: i32, 
    vote_data: Json<VoteData>
) -> Result<Status, TellonymError> {
    let db = conn.into_inner();
    let VoteData { is_like  } = vote_data.into_inner();
    let user_id = user.into_inner();

    let user = query::user_by_id(db, user_id).await?;
    let QuestionDTO { answer, .. } = query::question_w_answer_by_id(db, question_id).await?;

    let answer = answer.ok_or(TellonymError::ResourceNotFound)?;

    let request_like_type = if is_like { LikeType::AnswerLike } else { LikeType::AnswerDislike };

    let like = query::exact_like(db, user.id, request_like_type, answer.id).await?;
    
    match like {
        Some(like) => { 
            mutation::delete_like(db, like).await?;

            if is_like {
                mutation::change_answer_vote(db, answer, -1).await?;
            } else {
                mutation::change_answer_vote(db, answer, 1).await?;
            }
            // -1 do question if is_like
            // +1 do question if not is_like
        }
        None => {
            let opposite_like = query::exact_like(db, user.id, request_like_type.opposite_type(), answer.id).await?;
            
            if let Some(like) = opposite_like {
                let mut active_like: like::ActiveModel = like.into();
        
                active_like.like_type = Set(request_like_type);
    
                mutation::update_like(db, active_like).await?;
                
                // +2 do question if is_like
                // -2 do question if not is_like
                
                if is_like {
                    mutation::change_answer_vote(db, answer, 2).await?;
                } else {
                    mutation::change_answer_vote(db, answer, -2).await?;
                }

            } else {
                let new_like = like::ActiveModel {
                    liker_id: Set(user.id),
                    like_type: Set(request_like_type),
                    resource_id: Set(answer.id),
                    ..Default::default()
                };

                mutation::update_like(db, new_like).await?;

                // +1 do question if is_like
                // -1 do question if not is_like
                if is_like {
                    mutation::change_answer_vote(db, answer, 1).await?;
                } else {
                    mutation::change_answer_vote(db, answer, -1).await?;
                }
            }
        }
    }

    Ok(Status::Created)
}
