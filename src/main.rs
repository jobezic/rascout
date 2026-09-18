mod scanner;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "http://localhost:8080/ingest")]
    endpoint: String,

    #[arg(long, default_value_t = 100)]
    concurrency: usize,

    #[arg(long, default_value_t = 30)]
    interval: u64,

    #[arg(long, default_value_t = 50)]
    batch_size: usize,

    #[arg(long)]
    simulate: Option<usize>, // number of fake devices
}

#[tokio::main]
async fn main() {}
