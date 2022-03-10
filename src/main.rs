use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{
    GraphQLBadRequest, GraphQLResponse, GraphQLWebSocket,
};
use card_server::TokenFromHeader;
use dotenv;
use sea_orm::{Database, DbErr};
use std::convert::Infallible;
use tracing::log::{error, info};
use warp::{http::Response, hyper::StatusCode, ws::Ws, Filter, Rejection};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_test_writer().init();
    let url =
        std::env::var("DATABASE_URL").expect("you don't have a DATABASE_URL environment variable");
    let db;
    match Database::connect(url).await {
        Ok(ok) => {
            info!("database connected!");
            db = Some(ok);
        }
        Err(err) => {
            error!("failed to connect to database with {}, check your DATABASE_URL environment variable",err);
            db = None;
        }
    };
    if let Some(conn) = db {
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["X-PINGOTHER", "Content-Type"])
            .allow_methods(vec!["GET", "POST", "DELETE"]);
        let schema = card_server::build(conn);
        let graphql_post = warp::header::optional::<String>("token")
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |token, (schema, mut request): (card_server::Schema, async_graphql::Request)| async move {
                if let Some(token) = token {
                    request = request.data(TokenFromHeader(token));
                }
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        );
        let graphql_playground = warp::path("graphql").and(warp::get()).map(|| {
            Response::builder()
                .header("content-type", "text/html")
                .body(playground_source(
                    GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
                ))
        });
        let subscription = warp::path!("ws")
            .and(warp::ws())
            .and(warp::header::optional::<String>("token"))
            .and(warp::any().map(move || schema.clone()))
            .and(async_graphql_warp::graphql_protocol())
            .map(
                move |ws: Ws, token, schema: card_server::Schema, protocol| {
                    let reply = ws.on_upgrade(move |socket| {
                        let mut data = async_graphql::Data::default();
                        if let Some(token) = token {
                            data.insert(token);
                        }
                        GraphQLWebSocket::new(socket, schema, protocol)
                            .with_data(data)
                            .on_connection_init(card_server::on_connection_init)
                            .serve()
                    });
                    warp::reply::with_header(
                        reply,
                        "Sec-WebSocket-Protocol",
                        protocol.sec_websocket_protocol(),
                    )
                },
            );
        let routes = subscription
            .or(graphql_playground)
            .or(graphql_post)
            .recover(|err: Rejection| async move {
                if let Some(GraphQLBadRequest(err)) = err.find() {
                    return Ok::<_, Infallible>(warp::reply::with_status(
                        err.to_string(),
                        StatusCode::BAD_REQUEST,
                    ));
                }
                println!("{:?}", err);
                Ok(warp::reply::with_status(
                    "INTERNAL_SERVER_ERROR".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            })
            .with(cors);
        warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
    }
    Ok(())
}
