//! Env-gated process RSS checkpoints for runtime diagnostics.
//!
//! This module is diagnostic-only. It does not change runtime behavior unless
//! `HAKO_NYRT_RSS_CHECKPOINTS=1` is set.

const ENV_KEY: &str = "HAKO_NYRT_RSS_CHECKPOINTS";

pub fn checkpoint(label: &str) {
    tagged_checkpoint("runtime/rss", label);
}

pub fn tagged_checkpoint(tag: &str, label: &str) {
    if std::env::var(ENV_KEY).ok().as_deref() != Some("1") {
        return;
    }
    eprintln!(
        "[{}] checkpoint={} rss_bytes={}",
        tag,
        label,
        current_rss_bytes()
    );
}

#[cfg(target_os = "linux")]
pub fn current_rss_bytes() -> u64 {
    let Ok(status) = std::fs::read_to_string("/proc/self/status") else {
        return 0;
    };
    for line in status.lines() {
        let Some(rest) = line.strip_prefix("VmRSS:") else {
            continue;
        };
        let mut parts = rest.split_whitespace();
        let Some(kb_text) = parts.next() else {
            return 0;
        };
        return kb_text.parse::<u64>().unwrap_or(0).saturating_mul(1024);
    }
    0
}

#[cfg(not(target_os = "linux"))]
pub fn current_rss_bytes() -> u64 {
    0
}
