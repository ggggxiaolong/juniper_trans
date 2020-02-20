use juniper::{IntoFieldError, FieldError, Value};

const CODE_TOKEN: &str = "CODE_TOKEN_EXPIRE";

pub enum CustomError {
    TokenError,
}

impl IntoFieldError for CustomError {
    fn into_field_error(self) -> FieldError {
        match self {
            CustomError::TokenError => FieldError::new(
                CODE_TOKEN, Value::null(),
            )
        }
    }
}