use clap::Parser;

use hostwatch::{Cli, start_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .with_target(false)
    .compact()
    .init();

    let args:Cli = Cli::parse();
    
    start_server(args.port).await
}
