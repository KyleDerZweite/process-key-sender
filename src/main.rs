use clap::Parser;
use process_key_sender::cli::Cli;
use process_key_sender::runner::{init_tracing, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    init_tracing(cli.verbose);
    run(cli).await
}
