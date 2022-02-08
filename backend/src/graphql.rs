use juniper::{ EmptySubscription, RootNode };

use crate::context::{Context};

mod query;
use query::Query;

mod mutation;
use mutation::Mutation;

pub mod object;
pub mod repository;

mod error;

type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        Mutation,
        EmptySubscription::<Context>::new(),
    )
}