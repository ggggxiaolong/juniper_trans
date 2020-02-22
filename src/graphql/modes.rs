use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
pub enum LanaguageSearchType {
    All,
    New,
    Update,
    Change
}

// impl AddLang {
//     pub fn to_lang(&self) -> Lang {

//     }
// }

#[derive(GraphQLObject)]
pub struct Token {
    accessToken: String,
    refreshToken: String,
}

impl Token {
    pub fn new(accessToken: String, refreshToken: String) -> Token {
        Token {
            accessToken: accessToken,
            refreshToken: refreshToken,
        }
    }
}

pub struct Trans {
    pub en: String,
}

#[derive(GraphQLInputObject)]
pub struct UpdateLang {
    pub id: i32,
    pub en: Option<String>,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub sk: Option<String>,
    pub cs: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
    pub not_trans: Option<i32>,
    pub descripe: Option<String>,
    pub label: Option<String>,
    pub file_name: Option<String>,
    pub project_id: Option<i32>,
    pub mode_name: Option<String>,
}

pub struct TestObject{}