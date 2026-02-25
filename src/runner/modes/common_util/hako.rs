/*!
 * Hako-like source detection and minimal normalization helpers.
 *
 * - looks_like_hako_code: heuristics to detect Hako surface in Nyash path
 * - strip_local_decl: drop leading `local ` at line head for Nyash parser compatibility
 * - fail_fast_on_hako: env-gated policy (default ON) to fail fast on Hako-like source in Nyash VM path
 */

/// Heuristic detection of Hako-like source (development-only convenience)
pub fn looks_like_hako_code(s: &str) -> bool {
    s.contains("using selfhost.")
        || s.contains("using hakorune.")
        || s.lines().any(|l| l.trim_start().starts_with("local "))
}

/// Remove leading `local ` declarations at line head to keep Nyash parser stable
/// Conservative: only when line-head token is exactly `local` followed by a space.
/// Phase 21.2 fix: ONLY strip truly top-level `local` (zero indentation).
/// Keep `local` inside blocks (indented lines) to preserve Nyash variable declaration semantics.
pub fn strip_local_decl(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for line in s.lines() {
        let bytes = line.as_bytes();
        let mut i = 0;
        while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
        let mut stripped = false;
        // Only strip `local ` if it's at the very beginning (i == 0)
        // Keep `local ` inside blocks (i > 0) to preserve variable declarations
        if i == 0 && i + 6 <= bytes.len() && &bytes[i..i + 6] == b"local " {
            out.push_str(&line[..i]);
            out.push_str(&line[i + 6..]);
            out.push('\n');
            stripped = true;
        }
        if !stripped {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// Policy toggle: fail fast when Hako-like code enters Nyash VM path
/// Default: ON (true)
pub fn fail_fast_on_hako() -> bool {
    // Default: OFF（仕様不変＝拡張子だけで拒否しない）。
    // 明示時のみ ON（bring-up やデバッグ用途）。
    match std::env::var("HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM")
        .ok()
        .as_deref()
    {
        Some("1") | Some("true") | Some("on") => true,
        _ => false,
    }
}
