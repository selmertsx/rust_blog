use crate::graphql::object::{ User, UserKind };
use crate::graphql::repository::UserRepository;

pub fn mutate(_id: juniper::ID) -> User {
    let repository = UserRepository::new();
    let user = User {
        id: 2,
        kind: UserKind::Admin,
        name: "user2".into(),
    };

    repository.upsert(user)
}
