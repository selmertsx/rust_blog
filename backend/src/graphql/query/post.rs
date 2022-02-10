use diesel::prelude::*;
use backend::establish_connection;
use backend::schema::posts::dsl::*;

use crate::graphql::model::Post;

// TODO: 
// juniper::IDで受け取って変換するか
// そもそもULIDを利用するか
pub fn query() -> Post {
    let connection = establish_connection();
    posts.find(1).first::<Post>(&connection).expect("Post not found")
}
