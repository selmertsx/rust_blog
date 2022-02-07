
use crate::graphql::object::{User, UserKind};

pub fn query() -> Vec<User> {
    vec![User {
        id: 1,
        kind: UserKind::Admin,
        name: "user1".into(),
    }]
}
