mod ingest;
mod mailbox;
mod processor;

use std::sync::Arc;

use log::info;

use ingest::CameraSource;
use mailbox::Mailbox;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("edge-node starting");

    // Shared single-slot mailbox.
    let mailbox = Arc::new(Mailbox::new());

    // Camera source: local webcam (index 0).
    // Swap with CameraSource::Url("rtsp://...".into()) for an RTSP feed.
    // let source = CameraSource::Device(0);
    let source = CameraSource::Url("rtsp://716f898c7b71.entrypoint.cloud.wowza.com:1935/app-8F9K44lJ/304679fe_stream2".into());

    // Spawn ingestion (camera watchdog).
    let ingest_mailbox = Arc::clone(&mailbox);
    let ingest_handle = tokio::spawn(async move {
        ingest::ingest_loop(source, ingest_mailbox).await;
    });

    // Spawn processing metronome (10 FPS).
    let process_mailbox = Arc::clone(&mailbox);
    let process_handle = tokio::spawn(async move {
        processor::process_loop(process_mailbox).await;
    });

    // Wait for Ctrl+C, then shut down cleanly.
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for Ctrl+C");

    info!("edge-node: shutting down");

    ingest_handle.abort();
    process_handle.abort();

    info!("edge-node: goodbye");
}
