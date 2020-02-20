extern crate chrono;
use juniper::{GraphQLObject};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};



#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub mail: String,
    pub password: String,
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

#[derive(Queryable, GraphQLObject)]
pub struct Project {
    id: i32,
    name: String,
}