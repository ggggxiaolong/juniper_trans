use crate::entity::User;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use crate::graphql::modes::Token;

#[derive(Serialize, Deserialize)]
struct Claims {
    user: User,
    is_refresh: bool,
    exp: usize,
}

fn gen_token(user: &User, is_refresh: bool)-> String {
    let claims = Claims {
        user: user.clone(),
        is_refresh: is_refresh,
        exp: if is_refresh { 60 * 60 * 24 * 7 } else { 60 * 60 }
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

pub fn gen_user_token(user: User) -> Token{
    let access_token = gen_token(&user, false);
    let refresh_token = gen_token(&user, true);
    Token::new(access_token, refresh_token)
}

pub fn validate_token(token: &str) -> Option<User> {
    match decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
        Ok(c) => Some(c.claims.user),
        _ => None,
    }
}