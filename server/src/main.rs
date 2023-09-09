mod api;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use axum::body::StreamBody;
use axum::error_handling::HandleError;
use axum::extract::{Query, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use frontend::{ServerApp, AppProps};
use futures::Future;
use futures::stream::{self, StreamExt};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use yew::platform::Runtime;

use crate::api::get_api_router;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

// An executor to process requests on the Yew runtime.
//
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
//
// This increases performance in some environments (e.g.: in VM).
#[derive(Clone, Default)]
struct Executor {
    inner: Runtime,
}

impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.spawn_pinned(move || async move {
            fut.await;
        });
    }
}


async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>
) -> impl IntoResponse {
    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || AppProps{
        url: url.path().to_owned().into(),
        queries,
    });

    StreamBody::new(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

#[tokio::main]
async fn main() {
    let exec = Executor::default();
    let opt = Opt::parse();
    
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }

    tracing_subscriber::fmt::init();

    let static_path = PathBuf::from(&opt.static_dir);
    let index_html_s = tokio::fs::read_to_string(static_path.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();
    
    let handle_error = |e| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {e}"),
        )
    };

    let app = Router::new()
        .nest("/api", get_api_router())
        .fallback_service(HandleError::new(
            ServeDir::new(static_path)
                .append_index_html_on_directories(false)
                .precompressed_gzip()
                .fallback(
                    get(render)
                        .with_state((index_html_before.clone(), index_html_after.clone()))
                        .into_service()
                ),
            handle_error,
        ))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .executor(exec)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}