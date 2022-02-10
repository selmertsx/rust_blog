//! This example demonstrates async/await usage with warp.

use warp::Filter;
mod context;
pub use context::Context;

mod graphql;
use graphql::{ schema };

#[tokio::main]
async fn main() {
    let log = warp::log("warp_server");
    let state = warp::any().map(|| Context);
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([0, 0, 0, 0], 8080))
    .await
}
