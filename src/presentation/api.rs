use axum::{routing::get, Json, Router, extract::State, serve};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::{sync::watch, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer, compression::CompressionLayer};

use crate::domain::entities::HostStats;
use crate::infrastructure::metrics::get_info;

#[derive(Clone)]
struct AppState {
    rx: watch::Receiver<HostStats>,
}

async fn info(State(state): State<AppState>) -> Json<HostStats> {
    Json(state.rx.borrow().clone())
}

pub async fn start_server(port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let cancel = CancellationToken::new();

    let initial = get_info();
    let (tx, rx) = watch::channel(initial);
    let state = AppState { rx };

    let bg_cancel = cancel.clone();
    let mut ticker = tokio::time::interval(Duration::from_secs(2));
    let bg: JoinHandle<()> = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    let fresh = get_info();
                    let _ = tx.send(fresh);
                }
                _ = bg_cancel.cancelled() => {
                    tracing::info!("background updater: cancelled");
                    break;
                }
            }
        }
    });

    let app = Router::new()
        .route("/get_info", get(info))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(5)))
        .with_state(state);

    tracing::info!("🚀 Server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve(listener, app)
        .with_graceful_shutdown(shutdown(cancel.clone()))
        .await?;

    let waited = tokio::time::timeout(Duration::from_secs(3), bg).await;
    match waited {
        Ok(Ok(())) => {}
        Ok(Err(e)) => tracing::warn!("background task join error: {e}"),
        Err(_) => tracing::warn!("background task didn't stop in time"),
    }

    Ok(())
}

async fn shutdown(cancel: CancellationToken) {
    #[cfg(unix)]
    let term = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("sigterm handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let term = std::future::pending::<()>();

    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = term => {},
    }

    tracing::info!("Shutting down...");
    cancel.cancel();
}
