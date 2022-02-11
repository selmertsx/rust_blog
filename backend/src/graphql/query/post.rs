use diesel::prelude::*;
use backend::establish_connection;
use backend::schema::posts::dsl::*;

use crate::graphql::model::Post;

// TODO: 
// juniper::IDで受け取って変換するか
// そもそもULIDを利用するか
// expect も辞めたい。Result 型で受け取りたい。
pub fn query(_id: juniper::ID) -> Post {
    let connection = establish_connection();
    posts.find(1).first::<Post>(&connection).expect("Post not found")
}
