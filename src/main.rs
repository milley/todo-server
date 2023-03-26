use std::{net::SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use todo_server::http;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_server=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    http::serve(addr).await?;
    
    Ok(())
}
