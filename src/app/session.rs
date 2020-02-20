use actix_web::{FromRequest, Error, HttpRequest };
use actix_web::dev::Payload;
use futures::future::{ok, Ready};
use super::token::validate_token;
use crate::entity::User;

pub struct Session {
    pub user: Option<User>,
}

impl FromRequest for Session {
    type Error = Error;
    type Future = Ready<Result<Session, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let header = req.headers().get("token").map(|v|v.to_str());
        let user = if let Some(value) = header {
            match value {
                Ok(token) => validate_token(token),
                _ => None
            }
        } else {None};
        ok(Session{user: user,})
    }
}