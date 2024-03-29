
use rocket::{fs::TempFile, form::Form};
use serde::Serialize;
use rocket::fs::relative;
use serde_json::json;

use super::*;

#[get("/users/<user_id>")]
pub async fn get_user(
    conn: Connection<'_, Db>, 
    user_id: i32
) -> Result<Json<user::Model>, TellonymError> {
    let db = conn.into_inner();

    let user: user::Model = query::user_by_id(db, user_id).await?;

    Ok(Json(user))
}

#[derive(Serialize, Deserialize)]
pub struct UserWithLikesDTO {
    user: user::Model,
    likes: Vec<like::Model>
}

#[get("/me")]
pub async fn current_user(
    conn: Connection<'_, Db>, 
    user: UserGuard
) -> Result<Json<UserWithLikesDTO>, TellonymError> {
    let db = conn.into_inner();
    let user_id = user.into_inner();

    let user: Vec<(user::Model, Vec<like::Model>)> = User::find_by_id(user_id)
        .find_with_related(Like)
        .all(db)
        .await
        .map_err(|e| TellonymError::DatabaseError(e.to_string()))?;

    let (user, likes) = user.into_iter().next().ok_or(TellonymError::ResourceNotFound)?;

    let user_dto = UserWithLikesDTO {
        user,
        likes
    };

    Ok(Json(user_dto))
}

#[get("/users?<search>")]
pub async fn users(
    conn: Connection<'_, Db>, 
    search: Option<&'_ str>
) -> Result<Json<Vec<user::Model>>, TellonymError> {
    let db = conn.into_inner();
 
    match search {
        Some(search) => { 

            //Username starts with `search`
            let users = query::username_contains(db, search).await?;

            Ok(Json(users))
        }
        None => {
            //All users
            let users = query::all_users(db).await?;

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
pub async fn ask_question(
    conn: Connection<'_, Db>, 
    user: UserGuard, asked_id: i32, 
    question_data: Json<AskQuestionData<'_>>
) -> Result<Status, TellonymError> {
    let AskQuestionData { anonymous,  content } = question_data.into_inner();
    let user_id = user.into_inner();
    let db = conn.into_inner();

    let user_asking_question: user::Model = query::user_by_id(db, user_id).await?;

    //this is here to check whether the user (id: asked_id) actually exists
    let user_being_asked: user::Model = query::user_by_id(db, asked_id).await?;

    if user_being_asked.id == user_asking_question.id { return Err(TellonymError::ConstraintError(String::from("You can not ask yourself a question"))) }

    let question = question::ActiveModel {
        content: Set(content.to_string()),
        asked_id: Set(asked_id),
        asker_id: Set(if anonymous { None } else { Some(user_asking_question.id) }),
        asked_at: Set(chrono::offset::Utc::now().naive_utc()),
        ..Default::default()
    };

    mutation::add_question(db, question).await?;

    Ok(Status::Created)
}

#[get("/users/<user_id>/questions")]
pub async fn user_questions(
    conn: Connection<'_, Db>, 
    user_id: i32
) -> Result<Json<Vec<QuestionDTO>>, TellonymError> {
    let db = conn.into_inner();

    let questions_and_answers = query::questions_w_answers_by_asked_id(db, &[user_id]).await?;

    Ok(Json(questions_and_answers))
}

#[get("/users/<user_id>/follows")]
pub async fn user_follows(
    conn: Connection<'_, Db>, 
    user_id: i32
) -> Result<Json<Vec<user::Model>>, TellonymError> {
    let db = conn.into_inner();

    let follows = query::follows_with_follower_id(db, user_id).await?;

    let following_ids = follows.into_iter()
        .map(|follow_model| follow_model.following_id)
        .collect::<Vec<_>>();

    let following = query::users_with_ids(db, &following_ids).await?;

    Ok(Json(following))
}

#[get("/users/<user_id>/followers")]
pub async fn user_followers(
    conn: Connection<'_, Db>, 
    user_id: i32
) -> Result<Json<Vec<user::Model>>, TellonymError> {
    let db = conn.into_inner();

    let follows = query::follows_with_following_id(db, user_id).await?;

    let follower_ids = follows.into_iter()
        .map(|follow_model| follow_model.follower_id)
        .collect::<Vec<_>>(); 

    let followers = query::users_with_ids(db, &follower_ids).await?;

    Ok(Json(followers))
}

//TODO: 
#[post("/users/<to_follow_id>/follow")]
pub async fn follow_user(
    conn: Connection<'_, Db>, 
    user: UserGuard, 
    to_follow_id: i32
) -> Result<Status, TellonymError> {
    let db = conn.into_inner();
    let user_id = user.into_inner();

    let wants_to_follow = query::user_by_id(db, user_id).await?;
    let to_be_followed = query::user_by_id(db, to_follow_id).await?;

    if wants_to_follow.id == to_be_followed.id { return Err(TellonymError::ConstraintError(String::from("You can not follow yourself"))) };

    let follow: Option<follow::Model> = query::exact_follow(db, wants_to_follow.id, to_follow_id).await?;
    
    match follow {
        Some(follow) => {

            mutation::delete_follow(db, follow).await?;

            mutation::change_follow_counts(db, wants_to_follow, to_be_followed, -1).await?;
        },
        None => {
            let new_follow = follow::ActiveModel {
                follower_id: Set(wants_to_follow.id),
                following_id: Set(to_be_followed.id),
                ..Default::default()
            };

            mutation::insert_follow(db, new_follow).await?;

            mutation::change_follow_counts(db, wants_to_follow, to_be_followed, 1).await?;
        }
    }

    Ok(Status::Created)
}

#[derive(FromForm)]
pub struct EditProfileData<'a> {
    username: &'a str,
    current_password: &'a str,
    password: &'a str,
    bio: &'a str,
    twitch: &'a str,
    twitter: &'a str,
    youtube: &'a str,
    instagram: &'a str,
    profile_pic: TempFile<'a>,
    bg: TempFile<'a>
}

#[post("/editprofile", data = "<edit_profile_data>")]
pub async fn edit_profile(
    conn: Connection<'_, Db>, 
    user: UserGuard, 
    edit_profile_data: Form<EditProfileData<'_>>
) -> Result<Status, TellonymError> {
    let db = conn.into_inner();
    let user_id = user.into_inner();
    let EditProfileData { 
        username: new_username, 
        current_password,
        password: new_password, 
        bio: new_bio,
        twitch: new_twitch,
        twitter: new_twitter,
        youtube: new_youtube,
        instagram: new_instagram,
        profile_pic: mut new_profile_pic,
        bg: mut new_bg,
    } = edit_profile_data.into_inner();
 
    let user = query::user_by_id(db, user_id).await?;
    
    // figure out new values

    let new_username = if new_username.trim().len() == 0 { user.username.clone() } else { new_username.to_string() };
    let new_password = if new_password.trim().len() == 0 { 
        user.password.clone()
    } else {
        let current_pass_matches_old = verify(current_password, &user.password)
            .map_err(|_| TellonymError::ServerError)?;

        match current_pass_matches_old {
            true => hash(new_password, bcrypt::DEFAULT_COST)
                .map_err(|_| TellonymError::ServerError)?,
            false => return Err(TellonymError::ConstraintError(String::from("Current password must match the current password")))
        } 
    };

    let new_bio = new_bio.to_string();
    
    let new_twitch = new_twitch.to_string();
    let new_twitter = new_twitter.to_string();
    let new_youtube = new_youtube.to_string();
    let new_instagram = new_instagram.to_string();

    //set new data & save
    let mut active_user: user::ActiveModel = user.into();
    
    active_user.username = Set(new_username.trim().to_string());
    active_user.password = Set(new_password.trim().to_string());
    active_user.bio = Set(new_bio.trim().to_string());
    active_user.twitch = Set(new_twitch.trim().to_string());
    active_user.twitter = Set(new_twitter.trim().to_string());
    active_user.youtube = Set(new_youtube.trim().to_string());
    active_user.instagram = Set(new_instagram.trim().to_string());
    
    mutation::update_user(db, active_user).await?;

    //Save pfp if it was passed
    if new_profile_pic.len() > 0 {
        let path = relative!("pfps").to_string() + &format!("/{}.png", user_id);
    
        new_profile_pic.persist_to(path)
            .await
            .map_err(|_| {
                return TellonymError::ServerError;
            })?;
    }
    
    if new_bg.len() > 0 {
        let path = relative!("bgs").to_string() + &format!("/{}.png", user_id);
    
        new_bg.persist_to(path)
            .await
            .map_err(|_| {
                return TellonymError::ServerError;
            })?;
    }

    Ok(Status::Created)
}

#[get("/homepage")]
pub async fn get_homepage(
    conn: Connection<'_, Db>, 
    user: UserGuard
) -> Result<Json<Vec<serde_json::Value>>, TellonymError> {
    let db = conn.into_inner();
    let user_id = user.into_inner();
    
    let follows = query::follows_with_follower_id(db, user_id).await?;

    let following_ids = follows.into_iter()
        .map(|follow_model| follow_model.following_id)
        .collect::<Vec<_>>();

    let questions = query::questions_w_answers_by_asked_id(db, &following_ids).await?;

    let answered = questions.into_iter()
        .filter(|dto| dto.answer.is_some())
        .map(|dto| (dto.question, dto.answer.unwrap()))
        .collect::<Vec<_>>();
    
    let json = answered.iter().map(|(question, answer)| json!(
        {
            "question": question,
            "answer": answer
        }
    )).collect::<Vec<_>>();

    Ok(Json(json))
}