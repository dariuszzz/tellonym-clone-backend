use rocket::{response::{Responder, self}, Response, Request, futures::io::Cursor, http::{ContentType, Status}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TellonymError {
    ResourceNotFound,
    InvalidJWT,
    NoRefreshCookie,
    InvalidLogin,
    ConstraintError(String),
    ServerError,
    BadHeaders,
    DatabaseError(String),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for TellonymError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let formatted = format!("{self:?}");

        let status = match self {
            Self::ResourceNotFound => Status::NotFound,
            Self::InvalidJWT => Status::Unauthorized,
            Self::NoRefreshCookie => Status::Unauthorized,
            Self::InvalidLogin => Status::Unauthorized,
            Self::ConstraintError(_) => Status::BadRequest,
            Self::ServerError => Status::InternalServerError,
            Self::BadHeaders => Status::BadRequest,
            Self::DatabaseError(_) => Status::InternalServerError,
        };

        Response::build_from(formatted.respond_to(req)?)
            .header(ContentType::Plain)
            .status(status)
            .ok()
    }
}