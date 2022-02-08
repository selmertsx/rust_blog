use crate::Context;
use crate::graphql::object::{ User, UserKind };
use crate::graphql::error::Error;

#[derive(Debug, juniper::GraphQLInputObject)]
pub struct UserChangeNameInput {
    pub id: juniper::ID,
    pub name: String,
}

#[derive(Debug, juniper::GraphQLObject)]
#[graphql(context = Context, scalar = juniper::DefaultScalarValue)]
pub struct UserChangeNameOutput {
    user: User,
}

pub struct Mutation;

#[juniper::graphql_object(context = Context, scalar = juniper::DefaultScalarValue)]
impl Mutation {
    #[graphql(description = "Userを更新する")]
    async fn user_name_change(
        ctx: &Context,
        input: UserChangeNameInput,
    ) -> Result<UserChangeNameOutput, Error> {
        Ok(
            UserChangeNameOutput{ 
                user: User {
                    id: 1,
                    kind: UserKind::Admin,
                    name: "user1".into(),
                } 
            })
    }
}
