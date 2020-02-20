use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
enum LanaguageSearchType {
    All,
    New,
    Update,
    Change
}

#[derive(GraphQLInputObject)]
#[graphql(description = "add lang")]
pub struct AddLang {
    en: String,
    ja: Option<String>,
    ko: Option<String>,
    sk: Option<String>,
    cs: Option<String>,
    fr: Option<String>,
    es: Option<String>,
    not_trans: i32,
    description: Option<String>,
    label: Option<String>,
    file_name: Option<String>,
    project_id: i32,
    mode_name: Option<String>,
}

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


#[derive(GraphQLObject)]
pub struct Trans {
    en: String,
    ja: String,
    ko: String,
    sk: String,
    cs: String,
    fr: String,
    es: String,
}

#[derive(GraphQLInputObject)]
pub struct UpdateLang {
    id: i32,
    en: Option<String>,
    ja: Option<String>,
    ko: Option<String>,
    sk: Option<String>,
    cs: Option<String>,
    fr: Option<String>,
    es: Option<String>,
    not_trans: Option<i32>,
    descripe: Option<String>,
    label: Option<String>,
    file_name: Option<String>,
    project_id: Option<i32>,
    mode_name: Option<String>,
}
