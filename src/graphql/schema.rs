use super::exception::CustomError;
use super::modes::*;
use super::translate::translate;
use crate::entity::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use juniper::RootNode;

pub struct Context {
    pub conn: r2d2::Pool<ConnectionManager<SqliteConnection>>,
    pub user: Option<User>,
}

impl juniper::Context for Context {}

impl Context {
    fn get_conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, CustomError> {
        self.conn
            .get()
            .map_err(|e| CustomError::Internal(format!("{:?}", e)))
    }

    fn user_id(&self) -> i32 {
        self.user.clone().unwrap().id
    }
}

pub struct QueryRoot;

type CustomeResult<T> = Result<T, CustomError>;

#[juniper::graphql_object(Context = Context,)]
impl QueryRoot {
    #[graphql(description = "用户登录")]
    fn login(context: &Context, mail: String, password: String) -> CustomeResult<Token> {
        use crate::database::schema::user;
        let conn = context.get_conn()?;
        match user::table.filter(user::mail.eq(mail)).load::<User>(&conn) {
            Ok(mut users) => {
                if let Some(mut user) = users.pop() {
                    match bcrypt::verify(&password, &user.password) {
                        Ok(true) => {
                            user.remove_password();
                            Ok(crate::app::token::gen_user_token(user))
                        }
                        Ok(false) => Err(CustomError::MailOrPasswordFail),
                        Err(e) => Err(CustomError::Internal(format!("{:?}", e))),
                    }
                } else {
                    Err(CustomError::MailOrPasswordFail)
                }
            }
            Err(e) => Err(CustomError::Internal(format!("{:?}", e))),
        }
    }

    #[graphql(description = "刷新token")]
    fn refreshToken(token: String) -> CustomeResult<Token> {
        crate::app::token::validate_token(&token)
            .map(|user| Ok(crate::app::token::gen_user_token(user)))
            .unwrap_or(Err(CustomError::TokenError))
    }

    #[graphql(
        description = "查询语言",
        arguments(
            page(default = 0, description = "default 0",),
            page_size(default = 20, description = "default 20",),
            project_id(default = 1,),
            status_type(default= LanaguageSearchType::All)
        )
    )]
    fn language(
        context: &Context,
        page: i32,
        page_size: i32,
        search: Option<String>,
        project_id: i32,
        status_type: LanaguageSearchType,
    ) -> CustomeResult<Vec<Lang>> {
        use crate::database::schema::lang as langs;
        if context.user.is_some() {
            match context.conn.get() {
                Ok(conn) => {
                    let page_size: i64 = page_size as i64;
                    let offset = page_size * (page as i64);
                    let query = match status_type {
                        LanaguageSearchType::All => langs::table
                            .filter(langs::project_id.eq(project_id))
                            .filter(
                                langs::en.like(format!("%{}%", search.unwrap_or("".to_owned()))),
                            )
                            .limit(page_size)
                            .offset(offset)
                            .order(langs::update_time.desc())
                            .load::<Lang>(&conn),
                        LanaguageSearchType::Change => langs::table
                            .filter(langs::project_id.eq(project_id))
                            .filter(
                                langs::en.like(format!("%{}%", search.unwrap_or("".to_owned()))),
                            )
                            .filter(langs::status.eq(1))
                            .limit(page_size)
                            .offset(offset)
                            .order(langs::update_time.desc())
                            .load::<Lang>(&conn),
                        LanaguageSearchType::New => langs::table
                            .filter(langs::project_id.eq(project_id))
                            .filter(
                                langs::en.like(format!("%{}%", search.unwrap_or("".to_owned()))),
                            )
                            .filter(langs::status.eq(2))
                            .limit(page_size)
                            .offset(offset)
                            .order(langs::update_time.desc())
                            .load::<Lang>(&conn),
                        LanaguageSearchType::Update => langs::table
                            .filter(langs::project_id.eq(project_id))
                            .filter(
                                langs::en.like(format!("%{}%", search.unwrap_or("".to_owned()))),
                            )
                            .filter(langs::status.ne(0))
                            .limit(page_size)
                            .offset(offset)
                            .order(langs::update_time.desc())
                            .load::<Lang>(&conn),
                    };
                    query.or_else(|e| Err(CustomError::Internal(format!("{:?}", e))))
                }
                Err(e) => Err(CustomError::Internal(format!("{:?}", e))),
            }
        } else {
            Err(CustomError::TokenError)
        }
    }

    #[graphql(description = "查询所有的项目信息")]
    fn projects(context: &Context) -> CustomeResult<Vec<Project>> {
        use crate::database::schema::project;
        if let Some(_) = context.user {
            match context
                .conn
                .get()
                .map(|conn| project::table.load::<Project>(&conn))
            {
                Ok(Ok(projects)) => Ok(projects),
                Ok(Err(e)) => Err(CustomError::Internal(format!("{:?}", e))),
                Err(e) => Err(CustomError::Internal(format!("{:?}", e))),
            }
        } else {
            Err(CustomError::TokenError)
        }
    }

    #[graphql(description = "将英语翻译成其他语言")]
    fn trans(context: &Context, en: String) -> CustomeResult<Trans> {
        if context.user.is_some() {
            Ok(Trans { en: en })
        } else {
            Err(CustomError::TokenError)
        }
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context,)]
impl MutationRoot {
    #[graphql(description = "添加语言")]
    fn addLang(context: &Context, lang: AddLang) -> CustomeResult<Lang> {
        let add_lang = lang;
        if context.user.is_some() {
            use crate::database::schema::lang::dsl::*;
            use diesel::result::Error;
            let conn = context.get_conn()?;
            conn.transaction::<Lang, Error, _>(|| {
                diesel::insert_into(lang)
                    .values(&crate::database::entity::AddLang::from_add_lang(
                        add_lang,
                        context.user_id(),
                    ))
                    .execute(&conn)?;
                lang.order(id.desc()).first(&conn)
            })
            .map_err(|e| CustomError::Internal(format!("{:?}", e)))
        } else {
            Err(CustomError::TokenError)
        }
    }

    #[graphql(description = "更新语言")]
    fn updateLang(context: &Context, lang: UpdateLang) -> CustomeResult<Lang> {
        let update_lang = lang;
        let update_id = update_lang.id;
        if context.user.is_some() {
            use crate::database::schema::lang::dsl::*;
            use diesel::result::Error;
            let conn = context.get_conn()?;
            conn.transaction::<Lang, Error, _>(|| {
                diesel::update(lang)
                    .filter(id.eq(update_id))
                    .set(crate::database::entity::UpdateLang::from_lang(
                        update_lang,
                        context.user_id(),
                    ))
                    .execute(&conn)?;
                lang.filter(id.eq(update_id)).first(&conn)
            })
            .map_err(|e| CustomError::Internal(format!("{:?}", e)))
        } else {
            Err(CustomError::TokenError)
        }
    }

    #[graphql(description = "批量更新语言")]
    fn updateMultiLang(context: &Context, langs: Vec<UpdateLang>) -> CustomeResult<i32> {
        if context.user.is_some() {
            use crate::database::schema::lang::dsl::*;
            use diesel::result::Error;
            let conn = context.get_conn()?;
            conn.transaction::<i32, Error, _>(|| {
                for lan in langs.iter() {
                    diesel::update(lang)
                        .filter(id.eq(lan.id))
                        .set(crate::database::entity::UpdateLang::from_lang(
                            lan.clone(),
                            context.user_id(),
                        ))
                        .execute(&conn)?;
                }
                Ok(langs.len() as i32)
            })
            .map_err(|e| CustomError::Internal(format!("{:?}", e)))
        } else {
            Err(CustomError::TokenError)
        }
    }

    #[graphql(description = "将新增和修改的数据合并到数据库")]
    fn mergeUpdate(context: &Context, project_id: i32) -> CustomeResult<i32> {
        if context.user.is_some() {
            let conn = context.get_conn()?;
            use diesel::result::Error;
            conn.transaction::<usize, Error, _>(|| {
                diesel::dsl::sql::<()>("delete from tem_lang;").execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("insert into tem_lang select id, new_en, new_ja, new_ko, new_sk, new_cs, new_fr, new_es, new_not_trans, new_descripe, new_label, new_file_name, new_mode_name, new_project_id from lang where project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set en = (select new_en from tem_lang where tem_lang.id = lang.id) where new_en is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set ja = (select new_ja from tem_lang where tem_lang.id = lang.id) where new_ja is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set ko = (select new_ko from tem_lang where tem_lang.id = lang.id) where new_ko is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set sk = (select new_sk from tem_lang where tem_lang.id = lang.id) where new_sk is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set cs = (select new_cs from tem_lang where tem_lang.id = lang.id) where new_cs is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set fr = (select new_fr from tem_lang where tem_lang.id = lang.id) where new_fr is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set es = (select new_es from tem_lang where tem_lang.id = lang.id) where new_es is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set user_id = {} where status != 0 and project_id = {};", context.user_id(), project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set not_trans = (select new_not_trans from tem_lang where tem_lang.id = lang.id) where new_not_trans is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set descripe = (select new_descripe from tem_lang where tem_lang.id = lang.id) where new_descripe is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set label = (select new_label from tem_lang where tem_lang.id = lang.id) where new_label is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set file_name = (select new_file_name from tem_lang where tem_lang.id = lang.id) where new_file_name is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set mode_name = (select new_mode_name from tem_lang where tem_lang.id = lang.id) where new_mode_name is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set project_id = (select new_project_id from tem_lang where tem_lang.id = lang.id) where new_project_id is not null and status != 0 and project_id = {};", project_id)).execute(&conn)?;
                diesel::dsl::sql::<()>(&format!("update lang set new_user_id = null, new_en=null, new_ja=null, new_ko=null, new_sk=null, new_cs=null, new_fr=null, new_es=null, new_not_trans=null, new_descripe=null, new_label=null, new_file_name=null, new_mode_name=null, new_project_id=null, status = 0, update_time = CURRENT_TIMESTAMP where status != 0 and project_id = {};", project_id)).execute(&conn)
            })
            .map(|n|n as i32).map_err(|e| CustomError::Internal(format!("{:?}", e)))
        } else {
            Err(CustomError::TokenError)
        }
    }
}

#[juniper::graphql_object]
impl Trans {
    fn en(&self) -> String {
        self.en.clone()
    }
    async fn ja(&self) -> String {
        translate(&self.en, "en", "ja").await
    }
    async fn ko(&self) -> String {
        translate(&self.en, "en", "ko").await
    }
    async fn sk(&self) -> String {
        translate(&self.en, "en", "sk").await
    }
    async fn cs(&self) -> String {
        translate(&self.en, "en", "cs").await
    }
    async fn fr(&self) -> String {
        translate(&self.en, "en", "fr").await
    }
    async fn es(&self) -> String {
        translate(&self.en, "en", "es").await
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
