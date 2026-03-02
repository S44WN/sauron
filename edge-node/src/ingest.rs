use std::sync::Arc;
use std::time::Duration;

use log::{info, warn};
use opencv::core::Mat;
use opencv::prelude::MatTraitConst;
use opencv::videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst};

use crate::mailbox::Mailbox;

/// Camera source — either a local device index or an RTSP URL.
#[derive(Clone)]
pub enum CameraSource {
    Device(i32),
    Url(String),
}

/// Opens a VideoCapture for the given source. Returns `None` on failure.
fn open_capture(source: &CameraSource) -> Option<videoio::VideoCapture> {
    let cap = match source {
        CameraSource::Device(idx) => {
            videoio::VideoCapture::new(*idx, videoio::CAP_ANY).ok()
        }
        CameraSource::Url(url) => {
            videoio::VideoCapture::from_file(url, videoio::CAP_ANY).ok()
        }
    };

    match cap {
        Some(c) if c.is_opened().unwrap_or(false) => Some(c),
        _ => None,
    }
}

/// Long-running ingestion task. Never exits, never panics.
///
/// * Connects (or reconnects) to the camera source.
/// * Reads frames as fast as the camera delivers them.
/// * Overwrites the shared `Mailbox` with every successful read.
/// * On any failure, logs a warning, sleeps 5 s, and retries.
pub async fn ingest_loop(source: CameraSource, mailbox: Arc<Mailbox>) {
    loop {
        info!("ingest: attempting to connect…");

        let mut cap = match open_capture(&source) {
            Some(c) => {
                info!("ingest: camera connected");
                c
            }
            None => {
                warn!("ingest: failed to open camera, retrying in 5 s");
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        // Read loop — runs until the stream drops.
        loop {
            let mut frame = Mat::default();
            match cap.read(&mut frame) {
                Ok(true) if !frame.empty() => {
                    mailbox.overwrite_frame(frame);
                }
                _ => {
                    warn!("ingest: frame read failed, reconnecting in 5 s");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    break; // break inner loop → reconnect
                }
            }

            // Yield to the Tokio runtime so other tasks can run.
            tokio::task::yield_now().await;
        }
    }
}
