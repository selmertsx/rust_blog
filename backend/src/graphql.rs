use juniper::{ EmptyMutation, EmptySubscription, RootNode };

use crate::context::{Context};

mod query;
use query::Query;

pub mod object;
pub mod repository;

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}