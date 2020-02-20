use crate::entity::User;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    user: User,
    is_refresh: bool,
    exp: usize,
}

pub fn gen_token(user: User, is_refresh: bool)-> String {
    let claims = Claims {
        user: user,
        is_refresh: is_refresh,
        exp: if is_refresh { 60 * 60 * 24 * 7 } else { 60 * 60 }
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

pub fn validate_token(token: &str) -> Option<User> {
    match decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
        Ok(c) => Some(c.claims.user),
        _ => None,
    }
}