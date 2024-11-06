use app::*;
use axum::Router;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{LeptosRoutes, generate_route_list};

pub mod fileserv;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let conf = get_configuration(None).await.unwrap();
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(options);

    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
