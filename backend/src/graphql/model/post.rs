use juniper::GraphQLObject;
use diesel::{Queryable, Insertable};
use backend::schema::posts;

#[derive(Insertable, Queryable, GraphQLObject)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
}
