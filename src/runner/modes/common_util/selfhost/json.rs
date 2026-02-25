use crate::mir::MirModule;
use serde_json::Value;

#[derive(Debug)]
pub enum StageAPayload {
    MirModule(MirModule),
    ProgramJson(String),
    Empty,
}

#[derive(Debug)]
pub struct StageAPayloadResolution {
    pub payload: StageAPayload,
    pub mir_parse_error: Option<String>,
}

/// Extract the first JSON v0 line from stdout text.
/// Heuristic: a line starting with '{' and containing keys "version" and "kind".
pub fn first_json_v0_line<S: AsRef<str>>(s: S) -> Option<String> {
    for line in s.as_ref().lines() {
        let t = line.trim();
        if t.starts_with('{') && t.contains("\"version\"") && t.contains("\"kind\"") {
            return Some(t.to_string());
        }
    }
    None
}

/// Extract the first MIR JSON v0 line from stdout text.
/// Heuristic: a line starting with '{' and containing key "functions".
pub fn first_mir_json_v0_line<S: AsRef<str>>(s: S) -> Option<String> {
    for line in s.as_ref().lines() {
        let t = line.trim();
        if t.starts_with('{') && t.contains("\"functions\"") {
            return Some(t.to_string());
        }
    }
    None
}

/// Parse a JSON v0 line into MirModule using the existing bridge.
pub fn parse_json_v0_line(line: &str) -> Result<MirModule, String> {
    crate::runner::json_v0_bridge::parse_json_v0_to_module(line)
        .map_err(|e| format!("JSON v0 parse error: {}", e))
}

/// Parse a MIR JSON v0 line into MirModule.
pub fn parse_mir_json_v0_line(line: &str) -> Result<MirModule, String> {
    if strict_or_dev_enabled() {
        if let Some(msg) = legacy_callsite_reject_message(line) {
            return Err(msg);
        }
    }
    crate::runner::mir_json_v0::parse_mir_v0_to_module(line)
        .map_err(|e| format!("MIR JSON v0 parse error: {}", e))
}

/// Resolve Stage-A child payload ownership boundary.
/// Priority:
/// 1) valid MIR(JSON v0) -> `MirModule`
/// 2) Program(JSON v0) (including MIR parse error fallback)
/// 3) Empty
pub fn resolve_stage_a_payload(
    mir_line: Option<&str>,
    program_line: Option<&str>,
) -> StageAPayloadResolution {
    if let Some(line) = mir_line {
        match parse_mir_json_v0_line(line) {
            Ok(module) => {
                return StageAPayloadResolution {
                    payload: StageAPayload::MirModule(module),
                    mir_parse_error: None,
                };
            }
            Err(err) => {
                if let Some(program) = program_line {
                    return StageAPayloadResolution {
                        payload: StageAPayload::ProgramJson(program.to_string()),
                        mir_parse_error: Some(err),
                    };
                }
                return StageAPayloadResolution {
                    payload: StageAPayload::Empty,
                    mir_parse_error: Some(err),
                };
            }
        }
    }

    if let Some(program) = program_line {
        return StageAPayloadResolution {
            payload: StageAPayload::ProgramJson(program.to_string()),
            mir_parse_error: None,
        };
    }

    StageAPayloadResolution {
        payload: StageAPayload::Empty,
        mir_parse_error: None,
    }
}

fn strict_or_dev_enabled() -> bool {
    crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled()
}

fn legacy_callsite_reject_message(line: &str) -> Option<String> {
    let (reason, fn_name, bb, inst_idx, op) = detect_legacy_callsite_emit(line)?;
    Some(format!(
        "[freeze:contract][callsite-retire:{}] fn={} bb={} inst_idx={} op={}",
        reason, fn_name, bb, inst_idx, op
    ))
}

fn detect_legacy_callsite_emit(
    line: &str,
) -> Option<(&'static str, String, String, usize, String)> {
    let root: Value = serde_json::from_str(line).ok()?;
    let functions = root.get("functions")?.as_array()?;
    for func in functions {
        let fn_name = func
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("main")
            .to_string();
        let Some(blocks) = func.get("blocks").and_then(Value::as_array) else {
            continue;
        };
        for block in blocks {
            let bb = block
                .get("id")
                .and_then(Value::as_u64)
                .map(|id| id.to_string())
                .unwrap_or_else(|| "?".to_string());
            let Some(instructions) = block.get("instructions").and_then(Value::as_array) else {
                continue;
            };
            for (inst_idx, inst) in instructions.iter().enumerate() {
                let Some(op) = inst.get("op").and_then(Value::as_str) else {
                    continue;
                };
                let reason = match op {
                    "boxcall" => "legacy-boxcall",
                    "externcall" => "legacy-externcall",
                    _ => continue,
                };
                return Some((reason, fn_name, bb, inst_idx, op.to_string()));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{parse_mir_json_v0_line, resolve_stage_a_payload, StageAPayload};
    use crate::mir::{BasicBlockId, Callee, MirInstruction};
    use std::sync::{Mutex, OnceLock};

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn set(vars: &[(&'static str, &'static str)]) -> Self {
            let mut saved = Vec::with_capacity(vars.len());
            for (k, v) in vars {
                saved.push((*k, std::env::var(k).ok()));
                std::env::set_var(k, v);
            }
            Self { saved }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, old) in self.saved.drain(..) {
                if let Some(v) = old {
                    std::env::set_var(k, v);
                } else {
                    std::env::remove_var(k);
                }
            }
        }
    }

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn legacy_boxcall_fixture() -> &'static str {
        r#"{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"boxcall","dst":2,"box":1,"method":"id","args":[]},{"op":"ret","value":2}]}]}]}"#
    }

    fn legacy_externcall_fixture() -> &'static str {
        r#"{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"externcall","dst":2,"func":"env.console.log","args":[1]},{"op":"ret","value":2}]}]}]}"#
    }

    fn release_mode_env_guard() -> EnvGuard {
        EnvGuard::set(&[
            ("HAKO_JOINIR_STRICT", "0"),
            ("NYASH_JOINIR_STRICT", "0"),
            ("NYASH_JOINIR_DEV", "0"),
            ("HAKO_JOINIR_DEBUG", "0"),
            ("NYASH_JOINIR_DEBUG", "0"),
        ])
    }

    #[test]
    fn strict_or_dev_rejects_legacy_boxcall_emit() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set(&[
            ("HAKO_JOINIR_STRICT", "1"),
            ("NYASH_JOINIR_STRICT", "0"),
            ("NYASH_JOINIR_DEV", "0"),
            ("HAKO_JOINIR_DEBUG", "0"),
            ("NYASH_JOINIR_DEBUG", "0"),
        ]);

        let err = parse_mir_json_v0_line(legacy_boxcall_fixture())
            .expect_err("strict/dev must reject legacy boxcall emit");
        assert!(
            err.contains("[freeze:contract][callsite-retire:legacy-boxcall]"),
            "missing callsite-retire tag: {}",
            err
        );
        assert!(err.contains("op=boxcall"), "missing op in error: {}", err);
    }

    #[test]
    fn strict_or_dev_rejects_legacy_externcall_emit() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set(&[
            ("HAKO_JOINIR_STRICT", "1"),
            ("NYASH_JOINIR_STRICT", "0"),
            ("NYASH_JOINIR_DEV", "0"),
            ("HAKO_JOINIR_DEBUG", "0"),
            ("NYASH_JOINIR_DEBUG", "0"),
        ]);

        let err = parse_mir_json_v0_line(legacy_externcall_fixture())
            .expect_err("strict/dev must reject legacy externcall emit");
        assert!(
            err.contains("[freeze:contract][callsite-retire:legacy-externcall]"),
            "missing callsite-retire tag: {}",
            err
        );
        assert!(
            err.contains("op=externcall"),
            "missing op in error: {}",
            err
        );
    }

    #[test]
    fn release_mode_keeps_legacy_callsite_compat() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = release_mode_env_guard();

        let module = parse_mir_json_v0_line(legacy_boxcall_fixture())
            .expect("release/default path keeps legacy parse compatibility");
        let func = module.get_function("main").expect("main exists");
        let insts = &func
            .blocks
            .get(&BasicBlockId::new(0))
            .expect("bb0 exists")
            .instructions;
        assert!(matches!(
            &insts[0],
            MirInstruction::Call {
                callee: Some(Callee::Method { method, .. }),
                ..
            } if method == "id"
        ));
    }

    #[test]
    fn resolve_stage_a_payload_prefers_mir_when_parse_succeeds() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = release_mode_env_guard();
        let resolved = resolve_stage_a_payload(
            Some(legacy_boxcall_fixture()),
            Some(r#"{"version":"0","kind":"Program"}"#),
        );
        assert!(
            matches!(resolved.payload, StageAPayload::MirModule(_)),
            "expected MirModule payload"
        );
        assert!(resolved.mir_parse_error.is_none());
    }

    #[test]
    fn resolve_stage_a_payload_falls_back_to_program_when_mir_invalid() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = release_mode_env_guard();
        let program = r#"{"version":"0","kind":"Program","source":"foo"}"#;
        let resolved = resolve_stage_a_payload(Some("{invalid"), Some(program));
        match resolved.payload {
            StageAPayload::ProgramJson(line) => assert_eq!(line, program),
            _ => panic!("expected ProgramJson fallback"),
        }
        assert!(
            resolved
                .mir_parse_error
                .as_deref()
                .unwrap_or("")
                .contains("MIR JSON v0 parse error"),
            "expected MIR parse error detail"
        );
    }

    #[test]
    fn resolve_stage_a_payload_empty_when_no_program_fallback() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = release_mode_env_guard();
        let resolved = resolve_stage_a_payload(Some("{invalid"), None);
        assert!(matches!(resolved.payload, StageAPayload::Empty));
        assert!(
            resolved.mir_parse_error.is_some(),
            "invalid MIR without program should preserve parse error"
        );
    }
}
