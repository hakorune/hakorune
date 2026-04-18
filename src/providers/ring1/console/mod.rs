//! ring1 ConsoleService provider.
//!
//! Promotion status is managed in phase-29y docs.
//! This module provides the runtime implementation used by provider_lock.

use crate::runtime::core_services::ConsoleService;
use crate::runtime::ring0::get_global_ring0;
use crate::{box_trait::NyashBox, boxes::ConsoleBox};

#[derive(Debug, Default)]
pub struct Ring1ConsoleService;

impl Ring1ConsoleService {
    pub fn new() -> Self {
        Self
    }
}

/// Create the canonical runtime ConsoleBox through the ring1 console seam.
pub fn new_console_box() -> Box<dyn NyashBox> {
    Box::new(ConsoleBox::new())
}

impl ConsoleService for Ring1ConsoleService {
    fn println(&self, msg: &str) {
        let ring0 = get_global_ring0();
        ring0.log.info(msg);
    }

    fn print(&self, msg: &str) {
        let ring0 = get_global_ring0();
        let _ = ring0.io.stdout_write(msg.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::ring0::{default_ring0, GLOBAL_RING0};
    use std::sync::Arc;

    fn ensure_ring0_initialized() {
        let _ = GLOBAL_RING0.get_or_init(|| Arc::new(default_ring0()));
    }

    #[test]
    fn ring1_console_service_print_methods_do_not_panic() {
        ensure_ring0_initialized();
        let provider = Ring1ConsoleService::new();
        provider.println("ring1 console println test");
        provider.print("ring1 console print test");
    }

    #[test]
    fn ring1_console_new_box_returns_consolebox() {
        let boxed = new_console_box();
        assert!(boxed.as_any().downcast_ref::<ConsoleBox>().is_some());
    }
}
