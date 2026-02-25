//! SSOT bridge — thin callable shim from Rust to Hako resolver (Phase 22.1)
//!
//! MVP: does not invoke Hako VM yet. It mirrors the Hako box logic for modules-only
//! resolution, returning the mapped path when present. Callers must keep behavior
//! identical to existing resolver and use this only under an explicit env toggle.

use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

#[derive(Default, Debug, Clone)]
pub struct SsotCtx {
    pub modules: HashMap<String, String>,
    pub using_paths: Vec<String>,
    pub cwd: Option<String>,
}

/// Attempt to resolve via SSOT bridge. Returns Some(path) if found; otherwise None.
///
/// Behavior (MVP):
/// - Only consults `modules` map (exact match).
/// - Does not access filesystem nor invoke Hako VM.
pub fn call_using_resolve_ssot(name: &str, ctx: &SsotCtx) -> Option<String> {
    if name.is_empty() {
        return None;
    }
    // Optional: delegate to Hako resolver when explicitly requested.
    if std::env::var("HAKO_USING_SSOT_HAKO").ok().as_deref() == Some("1") {
        if let Some(hit) = call_hako_box(name, ctx) {
            return Some(hit);
        }
    }
    // MVP: modules-only
    ctx.modules.get(name).cloned()
}

/// Try resolving via Hako `UsingResolveSSOTBox.resolve(name, ctx)` by spawning the nyash VM.
/// Guarded by `HAKO_USING_SSOT_HAKO=1`. Returns Some(path) on success; otherwise None.
fn call_hako_box(name: &str, ctx: &SsotCtx) -> Option<String> {
    // Build inline Hako code that constructs a minimal ctx with modules map.
    let mut code = String::new();
    code.push_str("using hako.using.resolve.ssot as UsingResolveSSOTBox\n");
    code.push_str("static box Main {\n  main() {\n    local modules = new MapBox()\n");
    for (k, v) in ctx.modules.iter() {
        // Escape quotes conservatively
        let kk = k.replace('\"', "\\\"");
        let vv = v.replace('\"', "\\\"");
        code.push_str(&format!("    modules.set(\"{}\", \"{}\")\n", kk, vv));
    }
    code.push_str("    local ctx = new MapBox()\n    ctx.set(\"modules\", modules)\n");
    // relative_hint: opt-in via parent env HAKO_USING_SSOT_RELATIVE=1
    if std::env::var("HAKO_USING_SSOT_RELATIVE").ok().as_deref() == Some("1") {
        code.push_str("    ctx.set(\\\"relative_hint\\\", \\\"1\\\")\\n");
    }
    // using_paths
    if !ctx.using_paths.is_empty() {
        code.push_str("    local ups = new ArrayBox()\n");
        for up in ctx.using_paths.iter() {
            let upq = up.replace('\"', "\\\"");
            code.push_str(&format!("    ups.push(\"{}\")\n", upq));
        }
        code.push_str("    ctx.set(\\\"using_paths\\\", ups)\n");
    }
    // cwd
    if let Some(cwd) = &ctx.cwd {
        let cwq = cwd.replace('\"', "\\\"");
        code.push_str(&format!("    ctx.set(\\\"cwd\\\", \"{}\")\n", cwq));
    }
    let nn = name.replace('\"', "\\\"");
    code.push_str(&format!(
        "    local r = UsingResolveSSOTBox.resolve(\"{}\", ctx)\n    if r == null {{ return 0 }}\n    print(r)\n    return 0\n  }}\n",
        nn
    ));

    // Write to a temp file (Fail-Fast aware)
    let mut tf = match tempfile::Builder::new()
        .prefix("ny_ssot_")
        .suffix(".hako")
        .tempfile()
    {
        Ok(f) => f,
        Err(e) => {
            if crate::config::env::fail_fast() {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.error(&format!("[failfast/ssot/tempfile] {}", e));
                panic!("Fail-Fast: SSOT tempfile creation failed");
            }
            return None;
        }
    };
    let _ = write!(tf, "{}", code);
    let path = tf.path().to_path_buf();
    // Resolve nyash binary; Fail-Fast aware fallback
    let bin = if let Ok(b) = std::env::var("NYASH_BIN") {
        b
    } else {
        if let Ok(p) = std::env::current_exe() {
            p.to_string_lossy().to_string()
        } else {
            if crate::config::env::fail_fast() {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.error("[failfast/ssot/nyash-bin] unable to resolve NYASH_BIN/current_exe");
                panic!("Fail-Fast: cannot resolve nyash binary for SSOT child");
            }
            "target/release/nyash".to_string()
        }
    };

    // Stage‑3 + tolerance (matches smokes wrappers)
    let mut cmd = Command::new(bin);
    cmd.arg("--backend")
        .arg("vm")
        .arg(&path)
        // Parser/entry tolerances (same as smokes "safe" mode)
        .env("NYASH_FEATURES", "stage3")
        .env("NYASH_PARSER_ALLOW_SEMICOLON", "1")
        .env("NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN", "1")
        // Disable inline compiler for stability
        .env("NYASH_DISABLE_NY_COMPILER", "1")
        .env("HAKO_DISABLE_NY_COMPILER", "1")
        // Hard-disable SSOT in the child to avoid recursion; mark invoking guard
        .env("HAKO_USING_SSOT", "0")
        .env("HAKO_USING_SSOT_HAKO", "0")
        .env("HAKO_USING_SSOT_RELATIVE", "0")
        .env("HAKO_USING_SSOT_INVOKING", "1");
    // Any spawn/IO error → Fail-Fast or None
    let out = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            if crate::config::env::fail_fast() {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.error(&format!("[failfast/ssot/hako-spawn] {}", e));
                panic!("Fail-Fast: SSOT child spawn failed");
            }
            return None;
        }
    };
    if !out.status.success() {
        if crate::config::env::fail_fast() {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.error(&format!("[failfast/ssot/hako-exit] status={}", out.status));
            panic!("Fail-Fast: SSOT child exited with error");
        }
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        if crate::config::env::fail_fast() {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.error("[failfast/ssot/hako-empty]");
            panic!("Fail-Fast: SSOT child produced empty output");
        }
        None
    } else {
        Some(s)
    }
}
