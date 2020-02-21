use super::exception::CustomError;
use super::modes::*;
use super::translate::translate;
use crate::entity::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use juniper::RootNode;

pub struct Context {
    pub conn: r2d2::Pool<ConnectionManager<SqliteConnection>>,
    pub user: Option<User>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

type CustomeResult<T> = Result<T, CustomError>;

#[juniper::graphql_object(Context = Context,)]
impl QueryRoot {
    #[graphql(description = "用户登录")]
    fn login(context: &Context, mail: String, password: String) -> CustomeResult<Token> {
        use crate::database::schema::user;
        match context.conn.get() {
            Ok(conn) => match user::table.filter(user::mail.eq(mail)).load::<User>(&conn) {
                Ok(mut users) => {
                    if let Some(mut user) = users.pop() {
                        match bcrypt::verify(&password, &user.password) {
                            Ok(true) => {
                                user.remove_password();
                                Ok(crate::app::token::gen_user_token(user))
                            }
                            Ok(false) => Err(CustomError::MailOrPasswordFail),
                            Err(e) => Err(CustomError::Internal(format!("{:?}",e)))
                        }
                    } else {
                        Err(CustomError::MailOrPasswordFail)
                    }
                }
                Err(e) => Err(CustomError::Internal(format!("{:?}",e))),
            },
            Err(e) => Err(CustomError::Internal(format!("{:?}",e))),
        }
    }

    #[graphql(description = "刷新token")]
    fn refreshToken(token: String) -> CustomeResult<Token> {
        crate::app::token::validate_token(&token).map(|user|{
            Ok(crate::app::token::gen_user_token(user))
        }).unwrap_or(Err(CustomError::TokenError))
    }

    #[graphql(
        description = "查询语言",
        arguments(
            page(default = 0, description = "default 0",),
            pageSize(default = 20, description = "default 20",),
            projectId(default = 1,)
        )
    )]
    fn language(
        context: &Context,
        page: i32,
        pageSize: i32,
        search: Option<String>,
        projectId: i32,
        statusType: Option<String>,
    ) -> CustomeResult<Lang> {
        Err(CustomError::TokenError)
    }

    #[graphql(description = "查询所有的项目信息")]
    fn projects(context: &Context) -> CustomeResult<Project> {
        Err(CustomError::TokenError)
    }

    #[graphql(description = "将英语翻译成其他语言")]
    fn trans(context: &Context, en: String) -> CustomeResult<Trans> {
        Ok(Trans { en: en })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context,)]
impl MutationRoot {
    #[graphql(description = "添加语言")]
    fn addLang(context: &Context, lang: AddLang) -> CustomeResult<Lang> {
        Err(CustomError::TokenError)
    }

    #[graphql(description = "更新语言")]
    fn updateLang(context: &Context, lang: UpdateLang) -> CustomeResult<Lang> {
        Err(CustomError::TokenError)
    }

    #[graphql(description = "将新增和修改的数据合并到数据库")]
    fn mergeUpdate(context: &Context, projectId: i32) -> CustomeResult<String> {
        Err(CustomError::TokenError)
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
