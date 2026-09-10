use crate::config::env;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};

static EMIT_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Minimal debug hub: JSONL event emitter (dev-only; default OFF).
///
/// Env knobs:
/// - NYASH_DEBUG_ENABLE=1           master gate
/// - NYASH_DEBUG_KINDS=resolve,ssa  allowed cats (comma-separated)
/// - NYASH_DEBUG_SINK=path          file to append JSONL events
pub fn emit(
    cat: &str,
    kind: &str,
    fn_name: Option<&str>,
    region_id: Option<&str>,
    meta: serde_json::Value,
) {
    let Some(sink) = gated_sink(cat) else { return };
    write_event(cat, kind, fn_name, region_id, meta, &sink);
}

/// Emit an event whose caller-owned fields are constructed only after the
/// existing debug gate accepts the event. The callback is synchronous and is
/// invoked at most once.
pub(crate) fn emit_lazy<F>(cat: &str, kind: &str, build: F)
where
    F: FnOnce() -> (Option<String>, Option<String>, serde_json::Value),
{
    let Some(sink) = gated_sink(cat) else { return };
    let (fn_name, region_id, meta) = build();
    write_event(
        cat,
        kind,
        fn_name.as_deref(),
        region_id.as_deref(),
        meta,
        &sink,
    );
}

fn gated_sink(cat: &str) -> Option<String> {
    if env::env_string("NYASH_DEBUG_ENABLE").as_deref() != Some("1") {
        return None;
    }
    if let Some(kinds) = env::env_string("NYASH_DEBUG_KINDS") {
        if !kinds.split(',').any(|k| k.trim().eq_ignore_ascii_case(cat)) {
            return None;
        }
    }
    // Optional sampling: emit every N events (default 1 = no sampling)
    let sample_every = env::env_string("NYASH_DEBUG_SAMPLE_EVERY")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1);
    if sample_every > 1 {
        let n = EMIT_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
        if n % sample_every != 0 {
            return None;
        }
    }
    match env::env_string("NYASH_DEBUG_SINK") {
        Some(s) if !s.is_empty() => Some(s),
        _ => None,
    }
}

fn write_event(
    cat: &str,
    kind: &str,
    fn_name: Option<&str>,
    region_id: Option<&str>,
    meta: serde_json::Value,
    sink: &str,
) {
    let ts = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let obj = serde_json::json!({
        "ts": ts,
        "phase": "builder",
        "fn": fn_name.unwrap_or("<unknown>"),
        "region_id": region_id.unwrap_or(""),
        "cat": cat,
        "kind": kind,
        "meta": meta,
    });
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(sink)
    {
        let _ = writeln!(f, "{}", obj.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::emit_lazy;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn lazy_event_skips_builder_when_debug_is_off() {
        crate::test_support::with_env_vars(
            &[
                ("NYASH_DEBUG_ENABLE", Some("0")),
                ("NYASH_DEBUG_KINDS", None),
                ("NYASH_DEBUG_SAMPLE_EVERY", None),
                ("NYASH_DEBUG_SINK", None),
            ],
            || {
                let called = AtomicBool::new(false);
                emit_lazy("resolve", "try", || {
                    called.store(true, Ordering::SeqCst);
                    (None, None, serde_json::json!({"should_not_build": true}))
                });
                assert!(!called.load(Ordering::SeqCst));
            },
        );
    }

    #[test]
    fn lazy_event_preserves_enabled_json_shape() {
        let sink = std::path::PathBuf::from("/tmp/hakorune-debug-hub-lazy.jsonl");
        let _ = std::fs::remove_file(&sink);
        crate::test_support::with_env_vars(
            &[
                ("NYASH_DEBUG_ENABLE", Some("1")),
                ("NYASH_DEBUG_KINDS", Some("resolve")),
                ("NYASH_DEBUG_SAMPLE_EVERY", Some("1")),
                (
                    "NYASH_DEBUG_SINK",
                    Some("/tmp/hakorune-debug-hub-lazy.jsonl"),
                ),
            ],
            || {
                let called = AtomicBool::new(false);
                emit_lazy("resolve", "try", || {
                    called.store(true, Ordering::SeqCst);
                    (
                        Some("Main".to_owned()),
                        Some("region-1".to_owned()),
                        serde_json::json!({"method": "get", "arity": 0}),
                    )
                });
                assert!(called.load(Ordering::SeqCst));
            },
        );
        let line = std::fs::read_to_string(&sink).expect("lazy event sink");
        assert!(line.contains("\"kind\":\"try\""));
        assert!(line.contains("\"fn\":\"Main\""));
        assert!(line.contains("\"region_id\":\"region-1\""));
        assert!(line.contains("\"method\":\"get\""));
        let _ = std::fs::remove_file(sink);
    }
}
