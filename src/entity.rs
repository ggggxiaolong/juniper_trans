extern crate chrono;
use juniper::{GraphQLInputObject};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

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

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub mail: String,
    pub password: String,
}

impl User {
    pub fn remove_password(&mut self) {
        self.password = "".to_owned()
    }
}

pub struct JWTData {
    user: User,
    is_refresh: bool,
}

#[derive(Queryable)]
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
    not_trans: i32,
    descripe: Option<String>,
    label: Option<String>,
    file_name: Option<String>,
    mode_name: Option<String>,
    project_id: i32,
    new_user_id: Option<i32>,
    new_en: Option<String>,
    new_ja: Option<String>,
    new_ko: Option<String>,
    new_sk: Option<String>,
    new_cs: Option<String>,
    new_fr: Option<String>,
    new_es: Option<String>,
    new_not_trans: Option<i32>,
    new_descripe: Option<String>,
    new_label: Option<String>,
    new_file_name: Option<String>,
    new_mode_name: Option<String>,
    new_project_id: Option<i32>,
    status: i32,//0 为最新， 1为更新， 2为新增
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Project {
    id: i32,
    name: String,
}

pub struct Token {
    accessToken: String,
    refreshToken: String,
}

pub struct Trans {
    en: String,
    ja: String,
    ko: String,
    sk: String,
    cs: String,
    fr: String,
    es: String,
}

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