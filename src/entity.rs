extern crate chrono;
use juniper::{GraphQLObject, GraphQLInputObject};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize,Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub mail: String,
    pub password: String,
}

impl User {
    pub fn remove_password(&mut self){
        self.password = "".to_owned();
    }
}

#[derive(Queryable, GraphQLObject)]
pub struct Lang {
    id: i32,
    user_id: i32,
    en: String,
    ja: Option<String>,
    ko: Option<String>,
    sk: Option<String>,
    cs: Option<String>,
    fr: Option<String>,
    es: Option<String>,
    pt: Option<String>,
    #[graphql(name="not_trans")]
    not_trans: i32,
    descripe: Option<String>,
    label: Option<String>,
    #[graphql(name="file_name")]
    file_name: Option<String>,
    #[graphql(name="mode_name")]
    mode_name: Option<String>,
    #[graphql(name="project_id")]
    project_id: i32,
    #[graphql(name="new_user_id")]
    new_user_id: Option<i32>,
    #[graphql(name="new_en")]
    new_en: Option<String>,
    #[graphql(name="new_ja")]
    new_ja: Option<String>,
    #[graphql(name="new_ko")]
    new_ko: Option<String>,
    #[graphql(name="new_sk")]
    new_sk: Option<String>,
    #[graphql(name="new_cs")]
    new_cs: Option<String>,
    #[graphql(name="new_fr")]
    new_fr: Option<String>,
    #[graphql(name="new_es")]
    new_es: Option<String>,
    #[graphql(name="new_pt")]
    new_pt: Option<String>,
    #[graphql(name="new_not_trans")]
    new_not_trans: Option<i32>,
    #[graphql(name="new_descripe")]
    new_descripe: Option<String>,
    #[graphql(name="new_label")]
    new_label: Option<String>,
    #[graphql(name="new_file_name")]
    new_file_name: Option<String>,
    #[graphql(name="new_mode_name")]
    new_mode_name: Option<String>,
    #[graphql(name="new_project_id")]
    new_project_id: Option<i32>,
    status: i32,//0 为最新， 1为更新， 2为新增
    #[graphql(name="create_time")]
    create_time: NaiveDateTime,
    #[graphql(name="update_time")]
    update_time: NaiveDateTime,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "add lang")]
pub struct AddLang {
    pub en: String,
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub sk: Option<String>,
    pub cs: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
    pub pt: Option<String>,
    #[graphql(name="not_trans")]
    pub not_trans: i32,
    pub descripe: Option<String>,
    pub label: Option<String>,
    #[graphql(name="file_name")]
    pub file_name: Option<String>,
    #[graphql(name="project_id")]
    pub project_id: i32,
    #[graphql(name="mode_name")]
    pub mode_name: Option<String>,
}

#[derive(Queryable, GraphQLObject)]
pub struct Project {
    id: i32,
    name: String,
}