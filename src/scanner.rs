use std::{
    collections::HashSet,
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::{net::TcpStream, sync::mpsc, time::timeout};
use tracing::error;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub ip: String,
    pub ports: HashSet<u16>,
    pub timestamp: Instant,
}

pub async fn run_scanner(tx: mpsc::Sender<ScanResult>, concurrency: usize) {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));

    let targets = ["127.0.0.1"];
    for ip in targets {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let tx = tx.clone();

        tokio::spawn(async move {
            let result = scan_host(ip).await;

            if tx.send(result).await.is_err() {
                error!("scanner: receiver dropped!");
            }

            drop(permit);
        });
    }
}

async fn scan_port_with<F, Fut>(timeout_duration: Duration, connect: F) -> bool
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = std::io::Result<()>>,
{
    timeout(timeout_duration, connect())
        .await
        .map(|res| res.is_ok())
        .unwrap_or(false)
}

async fn scan_port(ip: &str, port: u16) -> bool {
    scan_port_with(Duration::from_millis(200), || async move {
        TcpStream::connect((ip, port)).await.map(|r| ())
    })
    .await
}

async fn scan_host(ip: &str) -> ScanResult {
    let ports_to_scan = [22, 80, 443];
    let mut open_ports = HashSet::new();

    for port in ports_to_scan {
        if scan_port(ip, port).await {
            open_ports.insert(port);
        }
    }

    ScanResult {
        ip: ip.to_string(),
        ports: open_ports,
        timestamp: Instant::now(),
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::sleep;

    use super::*;

    #[tokio::test]
    async fn scan_port_returns_true_on_success() {
        let open = scan_port_with(Duration::from_millis(1), || async { Ok(()) }).await;
        assert!(open)
    }

    #[tokio::test]
    async fn scan_port_returns_false_on_connect_error() {
        let open = scan_port_with(Duration::from_millis(1), || async {
            Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "refused",
            ))
        })
        .await;
        assert!(!open)
    }

    #[tokio::test]
    async fn scan_port_returns_false_on_timeout() {
        let open = scan_port_with(Duration::from_millis(10), || async {
            sleep(Duration::from_millis(50)).await;
            Ok(())
        })
        .await;

        assert!(!open);
    }
}
