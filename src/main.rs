use anyhow::Result;
use app_state::AppState;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_session::SessionStore;
use async_sqlx_session::PostgresSessionStore;
use auth::sign_in;
use database::Database;
use graphql_schema::{ContextData, MutationRoot, QueryRoot};
use models::User;
use std::env;
use tide::{
    http::{headers, mime, Cookie},
    Request, Response, Server, StatusCode,
};
mod app_state;
mod auth;
mod database;
mod graphql_schema;
mod models;
async fn graphql(req: Request<AppState>) -> tide::Result<Response> {
    let schema = req.state().schema.clone();
    let mut current_user = None;
    dbg!(&req.cookie("session_id"));
    if let Some(session_cookie) = req.cookie("session_id") {
        let user_session = req
            .state()
            .session_store
            .load_session(session_cookie.value().to_string())
            .await?;
        if let Some(user_session) = &user_session {
            dbg!(&user_session);

            let user_id = user_session.get("user_id");
            if let Some(uid) = user_id {
                current_user = User::get_by_id(uid).await?;
            }
        }
    }
    dbg!(&current_user);
    async_graphql_tide::graphql(req, schema, |query_builder| {
        if let Some(current_user) = current_user {
            query_builder.data(current_user)
        } else {
            query_builder
        }
    })
    .await
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

async fn handle_login(mut req: Request<AppState>) -> tide::Result {
    let body: auth::LoginInfo = req.body_json().await?;
    let session_store = req.state().clone().session_store;
    let cookie = sign_in(body.email, body.password, &session_store).await?;
    let mut response = Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .build();

    response.insert_cookie(Cookie::new("session_id", cookie));
    Ok(response)
}

// async fn handle_logout(req: Request<AppState>) -> tide::Result {
//     let AppState { ref auth, .. } = req.state();

//     let logout_url = auth.get_logout_redirect();

//     let mut resp: Response = Redirect::new(logout_url.to_string()).into();

//     // Remove any bearer cookies we have hanging around
//     if let Some(cookie) = req.cookie("bearer") {
//         resp.remove_cookie(cookie);
//     }

//     Ok(resp)
// }

#[async_std::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let database_url = env::var("DATABASE_URL")?;
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or(String::from("0.0.0.0:3000"));

    let db = Database::new(&database_url).await?;

    let session_store =
        PostgresSessionStore::new_with_table_name(&database_url, "users_sessions").await?;

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(ContextData {
            db,
            current_user: None,
        })
        .finish();

    let app_state = AppState {
        schema,
        session_store,
    };

    let mut app = Server::with_state(app_state);
    app.at("/login").post(handle_login);
    app.at("/").post(graphql);
    app.at("/").get(handle_graphiql);

    app.listen(listen_addr).await?;

    Ok(())
}
