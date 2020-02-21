use juniper::{IntoFieldError, FieldError, Value, graphql_value};

const CODE_TOKEN: &str = "CODE_TOKEN_EXPIRE";
const CODE_INTERNAL: &str = "CODE_SERVER_INTERNAL_ERROR";
const CODE_MAIL_OR_PASSWORD_FAIL: &str = "CODE_MAIL_OR_PASSWORD_FAIL";

pub enum CustomError {
    TokenError,
    Internal(String),
    MailOrPasswordFail,
}

impl IntoFieldError for CustomError {
    fn into_field_error(self) -> FieldError {
        match self {
            CustomError::TokenError => FieldError::new(
                CODE_TOKEN, Value::null(),
            ),
            CustomError::Internal(message) => FieldError::new(CODE_INTERNAL, graphql_value!({"info": message}),),
            CustomError::MailOrPasswordFail => FieldError::new(CODE_MAIL_OR_PASSWORD_FAIL, Value::null(),),
        }
    }
}