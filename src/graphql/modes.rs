use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
pub enum LanaguageSearchType {
    All,
    New,
    Update,
    Change
}

#[derive(GraphQLObject)]
pub struct Token {
    access_token: String,
    refresh_token: String,
}

impl Token {
    pub fn new(access_token: String, refresh_token: String) -> Token {
        Token {
            access_token: access_token,
            refresh_token: refresh_token,
        }
    }
}

pub struct Trans {
    pub en: String,
}

#[derive(GraphQLInputObject, Clone)]
pub struct UpdateLang {
    pub id: i32,
    pub en: Option<String>,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub sk: Option<String>,
    pub cs: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
    #[graphql(name="not_trans")]
    pub not_trans: Option<i32>,
    pub descripe: Option<String>,
    pub label: Option<String>,
    #[graphql(name="file_name")]
    pub file_name: Option<String>,
    #[graphql(name="project_id")]
    pub project_id: Option<i32>,
    #[graphql(name="mode_name")]
    pub mode_name: Option<String>,
}