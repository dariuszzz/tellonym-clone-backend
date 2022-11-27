use rocket::{http::SameSite, response::status};

use super::*;

#[post("/refresh")]
pub async fn refresh(cookies: &CookieJar<'_>) -> Result<String, TellonymError> {

    let refresh_token_cookie = cookies.get("refresh_token")
        .ok_or(TellonymError::NoRefreshCookie)?;

    let refresh_token = refresh_token_cookie.value();
 
    let username = JWTUtil::verify_refresh_jwt(refresh_token)
        .ok_or(TellonymError::InvalidJWT);

    let access_jwt = JWTUtil::sign_access_jwt(&username.unwrap());

    Ok(access_jwt)
}


#[derive(Deserialize)]
pub struct LoginData<'a> {
    username: &'a str,
    password: &'a str,
}

#[post("/register", data = "<register_data>")]
pub async fn register(
    cookies: &CookieJar<'_>,
    conn: Connection<'_, Db>, 
    register_data: Json<LoginData<'_>>
) -> Result<status::Created<String>, TellonymError> {
    let LoginData { username, password } = register_data.into_inner();
    let db = conn.into_inner();
    
    let hashed_pass = hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| TellonymError::ServerError)?;

    let user = user::ActiveModel {
        username: Set(username.to_string()),
        password: Set(hashed_pass), 
        ..Default::default()
    };
    
    mutation::register_user(db, user).await?;

    let access_jwt = JWTUtil::sign_access_jwt(&username);
    let refresh_jwt = JWTUtil::sign_refresh_jwt(&username);

    cookies.add(
        Cookie::build("refresh_token", refresh_jwt)
        .same_site(SameSite::None)
        .http_only(true)
        .secure(true)
        .finish()
    );

    Ok(status::Created::new("?").body(access_jwt))
}

#[post("/login", data = "<login_data>")]
pub async fn login(
    cookies: &CookieJar<'_>,
    conn: Connection<'_, Db>, 
    login_data: Json<LoginData<'_>>
) -> Result<String, TellonymError> {
    let LoginData { username, password } = login_data.into_inner();
    let db = conn.into_inner();

    let user: user::Model = query::user_by_username(db, username).await?;

    let valid = verify(password, &user.password)
        .map_err(|_| TellonymError::ServerError)?;

    if !valid { return Err(TellonymError::InvalidLogin); }


    let access_jwt = JWTUtil::sign_access_jwt(&username);
    let refresh_jwt = JWTUtil::sign_refresh_jwt(&username);

    cookies.add(
        Cookie::build("refresh_token", refresh_jwt)
        .http_only(true)
        .same_site(SameSite::None)
        .secure(true)
        .finish()
    );

    Ok(access_jwt)
}