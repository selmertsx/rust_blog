use diesel::prelude::*;
use backend::establish_connection;
use backend::schema::posts::dsl::*;

use crate::graphql::model::Post;

pub fn query(id: juniper::ID) -> Post {
    //let post_id = backend::schema::posts::id::from(id);
    let connection = establish_connection();
    posts.find("1").first::<Post>(&connection).expect("Post not found")
}
