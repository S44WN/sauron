use std::sync::Arc;
use std::time::Duration;

use log::info;
use opencv::core::MatTraitConst;
use tokio::time;

use crate::mailbox::Mailbox;

/// Fixed-rate processing metronome — fires exactly 10 times per second.
///
/// On each tick:
///   • Reads the latest frame from the `Mailbox`.
///   • If a frame exists, logs its dimensions.
///   • If no frame is available, skips silently.
///
/// This guarantees a predictable, constant processing rate regardless
/// of camera FPS.
pub async fn process_loop(mailbox: Arc<Mailbox>) {
    let mut ticker = time::interval(Duration::from_millis(100));

    loop {
        ticker.tick().await;

        if let Some(frame) = mailbox.get_latest_frame() {
            let cols = frame.cols();
            let rows = frame.rows();
            info!("process: frame {}×{}", cols, rows);
        }
        // No frame yet → skip silently.
    }
}
