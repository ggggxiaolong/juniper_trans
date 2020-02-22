use chrono::{Utc,NaiveDateTime};
use crate::database::schema::lang;
#[derive(AsChangeset)]
#[table_name = "lang"]
pub struct UpdateLang {
    id: i32,
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
    status: i32, //0 为最新， 1为更新， 2为新增
    update_time: NaiveDateTime,
}
impl UpdateLang {
    pub fn from_lang(lang: crate::graphql::modes::UpdateLang) -> UpdateLang {
        UpdateLang {
            id: lang.id,
            new_en: lang.en,
            new_ja: lang.ja,
            new_ko: lang.ko,
            new_sk: lang.sk,
            new_cs: lang.sk,
            new_fr: lang.fr,
            new_es: lang.es,
            new_not_trans: lang.not_trans,
            new_descripe: lang.descripe,
            new_label: lang.label,
            new_file_name: lang.file_name,
            new_mode_name: lang.mode_name,
            new_project_id: lang.project_id,
            status: 1,
            update_time: Utc::now().naive_utc()
        }
    }
}
