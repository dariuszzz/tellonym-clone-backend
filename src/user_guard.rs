use rocket::{request::{FromRequest, self}, Request, http::Status};

use crate::jwt_util::JWTUtil;

pub struct UserGuard(String);

impl UserGuard {
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_header = req.headers()
            .get("Authorization")
            .next()
            .ok_or(String::from("No authorization header"));

        if let Err(e) = auth_header { return request::Outcome::Failure((Status::BadRequest, e)) }

        let jwt_token = auth_header.expect("unwrapping token string").to_string();

        let jwt_token = jwt_token.split(" ").last()
            .ok_or(String::from("Invalid authorization header"));

        if let Err(e) = jwt_token { return request::Outcome::Failure((Status::BadRequest, e)) }

        let username = JWTUtil::verify_access_jwt(jwt_token.expect("invalid token"));

        if let None = username { return request::Outcome::Failure((Status::BadRequest, String::from("Invalid token"))) }

        request::Outcome::Success(Self(username.unwrap()))
    }
}