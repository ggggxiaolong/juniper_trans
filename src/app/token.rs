use crate::entity::User;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use crate::graphql::modes::Token;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
struct Claims {
    user: User,
    is_refresh: bool,
    exp: i64,
}

fn gen_token(user: &User, is_refresh: bool)-> String {
    let now = Utc::now().timestamp();
    let claims = Claims {
        user: user.clone(),
        is_refresh: is_refresh,
        exp: if is_refresh { now + 1000 * 60 * 60 * 24 * 7 } else { now + 1000 * 60 * 60 }
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
        Ok(c) if !c.claims.is_refresh => Some(c.claims.user),
        _ => None,
    }
}