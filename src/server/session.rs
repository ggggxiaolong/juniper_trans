use actix_web::{HttpResponse, ResponseError};
use failure::Fail;

#[derive(Debug, Fail)]
enum SessionError {
    #[fail(display = "you are not logged in")]
    Missing,
    #[fail(display = "session expired")]
    Invalid,
    #[fail(display = "internal server error")]
    Internal,
}

// impl ResponseError for SessionError {
//     fn error_response(&self) -> HttpResponse {
//         let response_builder = match *self {
//             SessionError::Missing => HttpResponse::Unauthorized(),
//         }
//     };
// }