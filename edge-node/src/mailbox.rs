use opencv::core::Mat;
use parking_lot::RwLock;

/// Single-slot mailbox for zero-latency frame sharing.
///
/// Stores exactly ONE frame at a time. Every new frame overwrites the previous.
/// No queues, no channels, no buffering — latest frame always wins.
pub struct Mailbox {
    frame: RwLock<Option<Mat>>,
}

impl Mailbox {
    pub fn new() -> Self {
        Self {
            frame: RwLock::new(None),
        }
    }

    /// Overwrites the stored frame with the latest capture.
    /// Any previous frame is dropped immediately.
    pub fn overwrite_frame(&self, new_frame: Mat) {
        let mut slot = self.frame.write();
        *slot = Some(new_frame);
    }

    /// Returns a clone of the latest frame, or `None` if no frame has arrived yet.
    pub fn get_latest_frame(&self) -> Option<Mat> {
        let slot = self.frame.read();
        slot.as_ref().map(|m| m.clone())
    }
}
