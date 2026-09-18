mod scanner;
// mod state;
// mod diff;
// mod sender;
// mod scheduler;

use clap::Parser;
use tokio::sync::mpsc;
use tracing::*;

use scanner::ScanResult;
// use diff::DiffEvent;

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
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let (scan_tx, scan_rx) = mpsc::channel::<ScanResult>(1000);
    // let (diff_tx, diff_rx) = mpsc::channel::<DiffEvent>(1000);

    // spawn pipeline stages
    // tokio::spawn(scanner::run_scanner(scan_tx.clone(), args.concurrency, args.simulate));
    // tokio::spawn(diff::run_diff_engine(scan_rx, diff_tx));
    // tokio::spawn(sender::run_sender(diff_rx, args.endpoint, args.batch_size));

    // scheduler loop
    // scheduler::run_scheduler(scan_tx, args.interval).await;
}
