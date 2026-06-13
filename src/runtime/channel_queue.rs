//! Reference runtime scaffold for the future `Channel<T>` queue.
//!
//! This module is intentionally separate from the legacy P2P `ChannelBox`.
//! It owns only the reference close/drain/send-after-close contract for
//! `CONC-CHANNEL-002`; it does not activate source-level waits, MIR lowering,
//! or worker-pool scheduling.

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Condvar, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelQueueError {
    SendAfterClose,
    AlreadyClosed,
    RecvClosed,
    Poisoned,
}

impl ChannelQueueError {
    pub const fn tag(&self) -> &'static str {
        match self {
            Self::SendAfterClose => "[channel/send-after-close]",
            Self::AlreadyClosed => "[channel/double-close]",
            Self::RecvClosed => "[channel/recv-closed]",
            Self::Poisoned => "[channel/poisoned]",
        }
    }
}

impl fmt::Display for ChannelQueueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag())
    }
}

#[derive(Debug)]
pub enum ChannelQueueSendError<T> {
    Closed { value: T },
    Poisoned { value: T },
}

impl<T> ChannelQueueSendError<T> {
    pub const fn tag(&self) -> &'static str {
        match self {
            Self::Closed { .. } => ChannelQueueError::SendAfterClose.tag(),
            Self::Poisoned { .. } => ChannelQueueError::Poisoned.tag(),
        }
    }

    pub fn into_value(self) -> T {
        match self {
            Self::Closed { value } | Self::Poisoned { value } => value,
        }
    }
}

impl<T> fmt::Display for ChannelQueueSendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag())
    }
}

#[derive(Debug)]
struct ChannelQueueInner<T> {
    closed: bool,
    buffer: VecDeque<T>,
}

/// Reference queue for future canonical `Channel<T>` semantics.
#[derive(Debug)]
pub struct ChannelQueue<T> {
    inner: Mutex<ChannelQueueInner<T>>,
    ready: Condvar,
}

impl<T> ChannelQueue<T> {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(ChannelQueueInner {
                closed: false,
                buffer: VecDeque::new(),
            }),
            ready: Condvar::new(),
        }
    }

    pub fn send(&self, value: T) -> Result<(), ChannelQueueSendError<T>> {
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(_) => return Err(ChannelQueueSendError::Poisoned { value }),
        };

        if inner.closed {
            return Err(ChannelQueueSendError::Closed { value });
        }

        inner.buffer.push_back(value);
        self.ready.notify_one();
        Ok(())
    }

    pub fn close(&self) -> Result<(), ChannelQueueError> {
        let mut inner = self.inner.lock().map_err(|_| ChannelQueueError::Poisoned)?;
        if inner.closed {
            return Err(ChannelQueueError::AlreadyClosed);
        }

        inner.closed = true;
        self.ready.notify_all();
        Ok(())
    }

    /// Non-blocking reference receive used to prove post-close drain semantics.
    pub fn try_recv(&self) -> Result<Option<T>, ChannelQueueError> {
        let mut inner = self.inner.lock().map_err(|_| ChannelQueueError::Poisoned)?;
        if let Some(value) = inner.buffer.pop_front() {
            return Ok(Some(value));
        }

        if inner.closed {
            return Err(ChannelQueueError::RecvClosed);
        }

        Ok(None)
    }

    /// Blocking reference receive for the close-wakes-waiters proof.
    ///
    /// This is a runtime scaffold only. It must not be exposed as an ordinary
    /// source-level blocking call; source `recv` remains await-visible.
    pub fn recv_blocking_reference(&self) -> Result<T, ChannelQueueError> {
        let mut inner = self.inner.lock().map_err(|_| ChannelQueueError::Poisoned)?;
        loop {
            if let Some(value) = inner.buffer.pop_front() {
                return Ok(value);
            }
            if inner.closed {
                return Err(ChannelQueueError::RecvClosed);
            }
            inner = self
                .ready
                .wait(inner)
                .map_err(|_| ChannelQueueError::Poisoned)?;
        }
    }

    pub fn is_closed(&self) -> Result<bool, ChannelQueueError> {
        let inner = self.inner.lock().map_err(|_| ChannelQueueError::Poisoned)?;
        Ok(inner.closed)
    }

    pub fn buffered_len(&self) -> Result<usize, ChannelQueueError> {
        let inner = self.inner.lock().map_err(|_| ChannelQueueError::Poisoned)?;
        Ok(inner.buffer.len())
    }
}

pub fn channel_queue_reference_report_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        ("channel_queue_reference_runtime_enabled", "1"),
        ("channel_queue_legacy_p2p_channelbox_reused", "0"),
        ("channel_queue_close_wakes_waiters_reference", "1"),
        ("channel_queue_send_after_close_rejected", "1"),
        ("channel_queue_drain_after_close_enabled", "1"),
        ("channel_queue_double_close_rejected", "1"),
        ("channel_queue_true_parallel_scheduler_required", "0"),
        ("channel_queue_source_blocking_call_enabled", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn channel_queue_closes_and_rejects_double_close() {
        let queue = ChannelQueue::<i64>::new();
        assert_eq!(queue.is_closed(), Ok(false));

        queue.close().expect("first close should succeed");
        assert_eq!(queue.is_closed(), Ok(true));
        assert_eq!(queue.close(), Err(ChannelQueueError::AlreadyClosed));
    }

    #[test]
    fn channel_queue_rejects_send_after_close_without_silent_drop() {
        let queue = ChannelQueue::new();
        queue.close().expect("close should succeed");

        let error = queue
            .send(42_i64)
            .expect_err("send after close must fail-fast");

        assert_eq!(error.tag(), "[channel/send-after-close]");
        assert_eq!(error.into_value(), 42);
        assert_eq!(queue.buffered_len(), Ok(0));
    }

    #[test]
    fn channel_queue_drains_buffer_after_close_then_reports_closed() {
        let queue = ChannelQueue::new();
        queue.send(1_i64).expect("send should succeed");
        queue.send(2_i64).expect("send should succeed");
        queue.close().expect("close should succeed");

        assert_eq!(queue.try_recv(), Ok(Some(1)));
        assert_eq!(queue.try_recv(), Ok(Some(2)));
        assert_eq!(queue.try_recv(), Err(ChannelQueueError::RecvClosed));
    }

    #[test]
    fn channel_queue_close_wakes_blocked_reference_receiver() {
        let queue = Arc::new(ChannelQueue::<i64>::new());
        let waiting_queue = Arc::clone(&queue);
        let waiter = std::thread::spawn(move || waiting_queue.recv_blocking_reference());

        std::thread::sleep(Duration::from_millis(20));
        queue.close().expect("close should wake waiter");

        assert_eq!(
            waiter.join().expect("waiter thread should join"),
            Err(ChannelQueueError::RecvClosed)
        );
    }

    #[test]
    fn channel_queue_reference_report_fields_are_stable() {
        assert_eq!(
            channel_queue_reference_report_fields(),
            vec![
                ("channel_queue_reference_runtime_enabled", "1"),
                ("channel_queue_legacy_p2p_channelbox_reused", "0"),
                ("channel_queue_close_wakes_waiters_reference", "1"),
                ("channel_queue_send_after_close_rejected", "1"),
                ("channel_queue_drain_after_close_enabled", "1"),
                ("channel_queue_double_close_rejected", "1"),
                ("channel_queue_true_parallel_scheduler_required", "0"),
                ("channel_queue_source_blocking_call_enabled", "0"),
            ]
        );
    }
}
