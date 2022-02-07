use juniper::{
    graphql_object, GraphQLEnum, EmptyMutation, EmptySubscription, FieldError, RootNode,
};

#[derive(Clone, Copy, Debug)]
pub struct Context;
impl juniper::Context for Context {}

#[derive(Clone, Copy, Debug, GraphQLEnum)]
pub enum UserKind {
    Admin,
    User,
    Guest,
}

#[derive(Clone, Debug)]
pub struct User {
    id: i32,
    kind: UserKind,
    name: String,
}

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn kind(&self) -> UserKind {
        self.kind
    }

    fn name(&self) -> &str {
        &self.name
    }

    async fn friends(&self) -> Vec<User> {
        vec![]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users() -> Vec<User> {
        vec![User {
            id: 1,
            kind: UserKind::Admin,
            name: "user1".into(),
        }]
    }

    async fn request(url: String) -> Result<String, FieldError> {
        Ok(reqwest::get(&url).await?.text().await?)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}