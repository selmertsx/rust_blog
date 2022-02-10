
use crate::graphql::model::User;
use crate::graphql::repository::UserRepository;

pub fn query(id: juniper::ID) -> User {
    let repository = UserRepository::new();
    repository.find(id)
}
