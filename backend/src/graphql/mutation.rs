use crate::Context;
use crate::graphql::model::User;
use crate::graphql::error::Error;

mod user_name_change;

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
        _ctx: &Context,
        input: UserChangeNameInput,
    ) -> Result<UserChangeNameOutput, Error> {
        let res = user_name_change::mutate(input.id);
        Ok(UserChangeNameOutput { user: res })
    }
}
