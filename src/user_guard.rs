use rocket::{request::{FromRequest, self}, Request, http::Status};

use crate::{jwt_util::JWTUtil, error::TellonymError};

pub struct UserGuard(String);

impl UserGuard {
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = TellonymError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_header = req.headers()
            .get("Authorization")
            .next()
            .ok_or(TellonymError::BadHeaders);

        if let Err(e) = auth_header { return request::Outcome::Failure((Status::BadRequest, e)) }

        let jwt_token = auth_header.expect("unwrapping token string").to_string();

        let jwt_token = jwt_token.split(" ").last()
            .ok_or(TellonymError::InvalidJWT);

        if let Err(e) = jwt_token { return request::Outcome::Failure((Status::Unauthorized, e)) }

        let username = JWTUtil::verify_access_jwt(jwt_token.expect("invalid token"))
            .ok_or(TellonymError::InvalidJWT);

        if let Err(e) = username { return request::Outcome::Failure((Status::Unauthorized, e)) }

        request::Outcome::Success(Self(username.unwrap()))
    }
}