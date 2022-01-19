mod info;
mod monitor;

use crate::monitor::Monitor;

#[tokio::main]
async fn main() {
    Monitor::default().start_monitoring().await;
}
