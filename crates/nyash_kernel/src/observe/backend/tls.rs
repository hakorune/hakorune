use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};

use super::super::config;

include!("tls/state.rs");
include!("tls/methods.rs");
include!("tls/api.rs");

#[cfg(test)]
mod tests {
    include!("tls/tests.rs");
}
