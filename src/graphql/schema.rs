use juniper::RootNode;
use diesel::r2d2::{ConnectionManager};
use diesel::prelude::*;
use crate::entity::*;
use super::exception::CustomError;
use super::modes::*;
use super::translate::translate;

pub struct Context {
    pub conn: r2d2::Pool<ConnectionManager<SqliteConnection>>,
    pub user: Option<User>
}

impl juniper::Context for Context{}

pub struct QueryRoot;

type CustomeResult<T> = Result<T, CustomError>;

#[juniper::graphql_object(Context = Context,)]
impl QueryRoot {
    #[graphql(
        description="用户登录"
    )]
    fn login(mail: String, password: String) -> CustomeResult<Token> {
        Err(CustomError::TokenError)
        // Ok(Token::new("accessToken: String".to_owned(), "refreshToken: String".to_owned()))
    }

    #[graphql(
        description="刷新token"
    )]
    fn refreshToken(token: String) -> CustomeResult<Token> {
        Err(CustomError::TokenError)
    }

    #[graphql(
        description="查询语言",
        arguments(
            page(
                default = 0,
                description = "default 0",
            ),
            pageSize(
                default = 20,
                description = "default 20",
            ),
            projectId(
                default = 1,
            )
        )
    )]
    fn language(context: &Context, page: i32, pageSize: i32, search: Option<String>, projectId: i32, statusType: Option<String>) -> CustomeResult<Lang> {
        Err(CustomError::TokenError)
    }

    #[graphql(
        description="查询所有的项目信息"
    )]
    fn projects(context: &Context,) -> CustomeResult<Project>{
        Err(CustomError::TokenError)
    }

    #[graphql(
        description="将英语翻译成其他语言"
    )]
    fn trans(context: &Context, en: String) -> CustomeResult<Trans> {
        Err(CustomError::TokenError)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context,)]
impl MutationRoot {
    #[graphql(
        description="添加语言"
    )]
    fn addLang(context: &Context, lang: AddLang) -> CustomeResult<Lang> {
        Err(CustomError::TokenError)
    }

    #[graphql(
        description="更新语言"
    )]
    fn updateLang(context: &Context, lang: UpdateLang) -> CustomeResult<Lang> {
        Err(CustomError::TokenError)
    }

    #[graphql(
        description="将新增和修改的数据合并到数据库"
    )]
    fn mergeUpdate(context: &Context, projectId: i32) -> CustomeResult<String> {
        Err(CustomError::TokenError)
    }

}

/// ja: String,
// ko: String,
// sk: String,
// cs: String,
// fr: String,
// es: String,
#[juniper::graphql_object]
impl Trans {
    async fn ja(&self) -> String {
        translate(&self.en, "en", "ja").await
    }
}


pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}