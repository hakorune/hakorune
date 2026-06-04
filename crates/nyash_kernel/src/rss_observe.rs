//! Env-gated process RSS checkpoints for NyRT diagnostics.

#[cfg(not(test))]
pub(crate) fn checkpoint(label: &str) {
    nyash_rust::runtime::rss_observe::tagged_checkpoint("nyrt/rss", label);
}
