use anyhow::Result;
use app_state::AppState;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use database::Database;
use graphql_schema::{ContextData, MutationRoot, QueryRoot};
use std::env;
use tide::{
    http::{headers, mime},
    Request, Response, Server, StatusCode,
};
mod app_state;
mod database;
mod graphql_schema;
mod models;

async fn graphql(req: Request<AppState>) -> tide::Result<Response> {
    let schema = req.state().schema.clone();
    async_graphql_tide::graphql(req, schema, |query_builder| query_builder).await
}
async fn handle_graphiql(_req: Request<AppState>) -> tide::Result {
    let playground_config = GraphQLPlaygroundConfig::new("/");
    let body = playground_source(playground_config);

    let mut resp = Response::new(StatusCode::Ok);
    resp.insert_header(headers::CONTENT_TYPE, mime::HTML);

    // TODO: Should the bearer cookie be removed once we set up this Graphiql session?

    resp.set_body(body);

    Ok(resp)
}
#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL")?;
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or(String::from("0.0.0.0:3000"));

    let db = Database::new(&database_url).await?;

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(ContextData { db })
        .finish();
    let app_state = AppState { schema };

    let mut app = Server::with_state(app_state);

    app.at("/").post(graphql);
    app.at("/").get(handle_graphiql);

    app.listen(listen_addr).await?;

    Ok(())
}
