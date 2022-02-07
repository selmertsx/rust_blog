use juniper::{ graphql_object, FieldError };
use crate::context::{Context};

mod user;
use user::{User, UserKind};

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
