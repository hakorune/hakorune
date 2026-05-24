//! Env-gated process RSS checkpoints for NyRT diagnostics.

const ENV_KEY: &str = "HAKO_NYRT_RSS_CHECKPOINTS";

pub(crate) fn checkpoint(label: &str) {
    if std::env::var(ENV_KEY).ok().as_deref() != Some("1") {
        return;
    }
    eprintln!("[nyrt/rss] checkpoint={} rss_bytes={}", label, current_rss_bytes());
}

#[cfg(target_os = "linux")]
fn current_rss_bytes() -> u64 {
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
fn current_rss_bytes() -> u64 {
    0
}
