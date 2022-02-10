use crate::graphql::model::{User, UserKind};

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self
    }

    pub fn find(&self, _id: juniper::ID) -> User {
        User {
            id: 1,
            kind: UserKind::Admin,
            name: "user1".into(),
        }
    }

    pub fn fetch_all(&self) -> Vec<User>{
        vec![
            User {
                id: 1,
                kind: UserKind::Admin,
                name: "user1".into(),
            },
            User {
                id: 2,
                kind: UserKind::Admin,
                name: "user2".into(),
            }
        ]
    }

    pub fn upsert(&self, user: User) -> User {
        user
    }
}