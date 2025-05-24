use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use log::{info, warn};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;

struct HttpServeState {
    path: PathBuf,
}

pub async fn http_serve_process(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on port: {}", path, addr);
    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/{*}", get(index_handler))
        .with_state(Arc::new(state));
    let listenner = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listenner, router).await?;
    Ok(())
}

async fn index_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Serving file: {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File not found: {}", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Failed to read file: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read file: {}", e),
                )
            }
        }
    }
}
