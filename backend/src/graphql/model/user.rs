use juniper::{ graphql_object, GraphQLEnum};
use crate::context::{Context};

#[derive(Clone, Copy, Debug, GraphQLEnum)]
pub enum UserKind {
    Admin,
    User,
    Guest,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub kind: UserKind,
    pub name: String,
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
