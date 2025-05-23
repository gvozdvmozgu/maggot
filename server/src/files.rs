use app::App;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, Response, StatusCode, Uri};
use axum::response::{IntoResponse, Response as AxumResponse};
use leptos::*;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler =
            leptos_axum::render_app_to_stream(options.to_owned(), move || view! { <App/> });
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let request = Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();
    match ServeDir::new(root).oneshot(request).await {
        Ok(res) => Ok(res.map(Body::new)),
        Err(err) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {err}")))
        }
    }
}
