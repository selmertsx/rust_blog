
use crate::graphql::model::User;
use crate::graphql::repository::UserRepository;

pub fn query() -> Vec<User> {
    let repository = UserRepository::new();
    repository.fetch_all()
}
