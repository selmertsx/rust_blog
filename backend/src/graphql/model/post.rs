use diesel::Queryable;

#[derive(Queryable)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub body: Option<String>,
}

#[juniper::graphql_object]
impl Post {
    #[graphql(name="id")]
    fn id(&self) -> String {
        String::from(&self.id)
    }

    #[graphql(name="user_id")]
    fn user_id(&self) -> String {
        String::from(&self.user_id)
    }

    #[graphql(name="title")]
    fn title(&self) -> String {
        String::from(&self.title)
    }

    #[graphql(name="body")]
    fn body(&self) -> String {
        match &self.body {
            Some(body) => String::from(body),
            None => "".to_string(),
        }
    }
}