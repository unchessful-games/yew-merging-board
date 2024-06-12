mod managers;
mod matchmaking;

use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use clap::Parser;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Parser, Debug)]
#[clap(name = "server")]
struct Opt {
    #[clap(short, long, default_value = "static")]
    static_dir: String,

    #[clap(short, long, default_value = "127.0.0.1")]
    addr: IpAddr,

    #[clap(short, long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let opt = Opt::parse();

    let state = managers::launch();

    let app = Router::new()
        .route("/api/matchmaking/counts", get(matchmaking::counts))
        .route(
            "/api/matchmaking/ws",
            get(matchmaking::handle_matchmaking_request),
        )
        .fallback_service(get(|req: Request<Body>| async move {
            let res = ServeDir::new(&opt.static_dir).oneshot(req).await.unwrap(); // serve dir is infallible
            let status = res.status();
            match status {
                // If we don't find a file corresponding to the path we serve index.html.
                // If you want to serve a 404 status code instead you can add a route check as shown in
                // https://github.com/rksm/axum-yew-setup/commit/a48abfc8a2947b226cc47cbb3001c8a68a0bb25e
                StatusCode::NOT_FOUND => {
                    let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                    tokio::fs::read_to_string(index_path)
                        .await
                        .map(|index_content| (StatusCode::OK, Html(index_content)).into_response())
                        .unwrap_or_else(|_| {
                            (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found")
                                .into_response()
                        })
                }

                // path was found as a file in the static dir
                _ => res.into_response(),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    let sock_addr = SocketAddr::from((opt.addr, opt.port));
    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
