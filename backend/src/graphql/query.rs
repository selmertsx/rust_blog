use juniper::{ graphql_object, FieldError };
use crate::context::{Context};
use crate::graphql::model::{User, Post};

mod users;
mod user;
mod post;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users() -> Vec<User> {
        users::query()
    }

    async fn user(id: juniper::ID) -> User {
        user::query(id)
    }

    async fn post(id: juniper::ID) -> Post {
        post::query(id)
    }

    async fn request(url: String) -> Result<String, FieldError> {
        Ok(reqwest::get(&url).await?.text().await?)
    }
}
