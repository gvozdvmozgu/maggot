mod files;

use anyhow::Context;
use app::App;
use leptos_axum::{LeptosRoutes as _, generate_route_list};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).context("couldn't initialize logging")?;

    let options = leptos::get_configuration(None).await?.leptos_options;
    let tcp_listener = tokio::net::TcpListener::bind(options.site_addr).await?;
    let make_service = axum::Router::new()
        .leptos_routes(&options, generate_route_list(App), App)
        .fallback(files::handler)
        .with_state(options)
        .into_make_service();

    axum::serve(tcp_listener, make_service).await.map_err(Into::into)
}
