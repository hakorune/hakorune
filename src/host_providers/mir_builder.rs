mod lowering;

#[cfg(test)]
use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::atomic::{AtomicU64, Ordering};
// use std::io::Write; // kept for future pretty-print extensions

pub(crate) const FAILFAST_TAG: &str = "[freeze:contract][hako_mirbuilder]";
#[cfg(test)]
pub(crate) const TRACE_ENV: &str = "HAKO_STAGE1_MODULE_DISPATCH_TRACE";
static MIR_JSON_TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[cfg(test)]
pub(crate) fn trace_enabled() -> bool {
    std::env::var(TRACE_ENV).ok().as_deref() == Some("1")
}

#[cfg(test)]
pub(crate) fn trace_log(message: impl AsRef<str>) {
    if trace_enabled() {
        eprintln!("{}", message.as_ref());
    }
}

pub(crate) fn failfast_error(message: impl Display) -> String {
    let tag = format!("{FAILFAST_TAG} {}", message);
    crate::runtime::get_global_ring0().log.error(&tag);
    tag
}

pub(crate) fn unique_mir_json_tmp_path() -> std::path::PathBuf {
    let seq = MIR_JSON_TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "hako_mirbuilder_out-{}-{}.json",
        std::process::id(),
        seq
    ))
}

pub(crate) struct ScopedEnvVar {
    key: &'static str,
    prev: Option<String>,
}

impl ScopedEnvVar {
    pub(crate) fn set(key: &'static str, value: &'static str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, prev }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        match self.prev.take() {
            Some(v) => std::env::set_var(self.key, v),
            None => std::env::remove_var(self.key),
        }
    }
}

pub(crate) struct Phase0MirJsonEnvGuard {
    _schema_v1: ScopedEnvVar,
    _unified_call: ScopedEnvVar,
}

impl Phase0MirJsonEnvGuard {
    pub(crate) fn new() -> Self {
        Self {
            _schema_v1: ScopedEnvVar::set("NYASH_JSON_SCHEMA_V1", "0"),
            _unified_call: ScopedEnvVar::set("NYASH_MIR_UNIFIED_CALL", "0"),
        }
    }
}

/// Convert Program(JSON v0) to MIR(JSON v0) and return it as a String.
/// Fail-Fast: prints stable tags on stderr and returns Err with the same tag text.
#[cfg(test)]
pub(crate) fn program_json_to_mir_json(program_json: &str) -> Result<String, String> {
    lowering::program_json_to_mir_json(program_json)
}

pub fn program_json_to_mir_json_with_user_box_decls(program_json: &str) -> Result<String, String> {
    emit_guarded_mir_json_from_program_json(program_json)
}

fn emit_guarded_mir_json_from_program_json(program_json: &str) -> Result<String, String> {
    let _env_guard = Phase0MirJsonEnvGuard::new();
    emit_mir_json_from_program_json_module(program_json)
}

fn emit_mir_json_from_program_json_module(program_json: &str) -> Result<String, String> {
    let module = parse_program_json_module(program_json)?;
    emit_module_mir_json_with_stage1_user_box_decls(program_json, &module)
}

/// Test-only helper that still exposes the transient Program(JSON v0) plus MIR(JSON)
/// while the current authority remains Rust-owned.
#[cfg(test)]
pub fn source_to_program_and_mir_json(source_text: &str) -> Result<(String, String), String> {
    emit_program_and_plain_mir_json_for_source(source_text)
}

pub fn source_to_mir_json(source_text: &str) -> Result<String, String> {
    let (_, mir_json) = emit_program_and_guarded_mir_json_for_source(source_text)?;
    Ok(mir_json)
}

/// Convert Program(JSON v0) to MIR(JSON v0) with using imports support.
#[cfg(test)]
pub(crate) fn program_json_to_mir_json_with_imports(
    program_json: &str,
    imports: BTreeMap<String, String>,
) -> Result<String, String> {
    lowering::program_json_to_mir_json_with_imports(program_json, imports)
}

fn emit_mir_json_from_program_json_text(program_json: &str) -> Result<String, String> {
    emit_guarded_mir_json_from_program_json(program_json)
}

#[cfg(test)]
fn emit_program_and_plain_mir_json_for_source(
    source_text: &str,
) -> Result<(String, String), String> {
    emit_program_and_mir_json_for_source(source_text, emit_plain_mir_json_from_program_json_text)
}

fn emit_program_and_guarded_mir_json_for_source(
    source_text: &str,
) -> Result<(String, String), String> {
    emit_program_and_mir_json_for_source(source_text, emit_mir_json_from_program_json_text)
}

fn emit_program_and_mir_json_for_source(
    source_text: &str,
    emit_mir_json: fn(&str) -> Result<String, String>,
) -> Result<(String, String), String> {
    let program_json = emit_program_json_for_source(source_text)?;
    let mir_json = emit_mir_json(&program_json)?;
    Ok((program_json, mir_json))
}

#[cfg(test)]
fn emit_plain_mir_json_from_program_json_text(program_json: &str) -> Result<String, String> {
    lowering::program_json_to_mir_json(program_json)
}

pub(crate) fn module_to_mir_json(module: &crate::mir::MirModule) -> Result<String, String> {
    let tmp_path = emit_module_to_temp_mir_json(module)?;
    finalize_temp_mir_json_output(&tmp_path)
}

fn emit_strict_program_json_for_source(source_text: &str) -> Result<String, String> {
    crate::stage1::program_json_v0::emit_program_json_v0_for_strict_authority_source(source_text)
        .map_err(|error| format!("{FAILFAST_TAG} {}", error))
}

fn emit_program_json_for_source(source_text: &str) -> Result<String, String> {
    emit_strict_program_json_for_source(source_text)
}

fn parse_program_json_module(program_json: &str) -> Result<crate::mir::MirModule, String> {
    crate::runner::json_v0_bridge::parse_json_v0_to_module(program_json).map_err(failfast_error)
}

fn emit_module_mir_json_with_stage1_user_box_decls(
    program_json: &str,
    module: &crate::mir::MirModule,
) -> Result<String, String> {
    let user_box_decls = resolve_stage1_user_box_decls_from_program_json(program_json)?;
    let module = with_stage1_user_box_decls(module, &user_box_decls);
    emit_module_mir_json(&module)
}

fn emit_module_to_temp_mir_json(
    module: &crate::mir::MirModule,
) -> Result<std::path::PathBuf, String> {
    let tmp_path = unique_mir_json_tmp_path();
    match crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(module, &tmp_path) {
        Ok(()) => Ok(tmp_path),
        Err(error) => Err(failfast_error(error)),
    }
}

fn read_temp_mir_json_output(tmp_path: &std::path::Path) -> Result<String, String> {
    std::fs::read_to_string(tmp_path).map_err(failfast_error)
}

fn cleanup_temp_mir_json_output(tmp_path: &std::path::Path) {
    let _ = std::fs::remove_file(tmp_path);
}

fn finalize_temp_mir_json_output(tmp_path: &std::path::Path) -> Result<String, String> {
    let raw = read_temp_mir_json_output(tmp_path)?;
    cleanup_temp_mir_json_output(tmp_path);
    Ok(canonicalize_mir_json_output(raw))
}

fn canonicalize_mir_json_output(raw: String) -> String {
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(value) => serde_json::to_string(&value).unwrap_or(raw),
        Err(_) => raw,
    }
}

fn emit_module_mir_json(module: &crate::mir::MirModule) -> Result<String, String> {
    module_to_mir_json(module)
}

fn with_stage1_user_box_decls(
    module: &crate::mir::MirModule,
    user_box_decls: &[serde_json::Value],
) -> crate::mir::MirModule {
    let mut module = module.clone();
    module.metadata.user_box_decls = stage1_user_box_decl_map(user_box_decls);
    module
}

fn build_stage1_user_box_decls(program_value: &serde_json::Value) -> Vec<serde_json::Value> {
    let names = collect_stage1_user_box_decl_names(program_value);
    build_stage1_user_box_decls_from_names(names)
}

fn resolve_stage1_user_box_decls_from_program_json(
    program_json: &str,
) -> Result<Vec<serde_json::Value>, String> {
    let program_value = parse_program_json_value(program_json)?;
    Ok(resolve_stage1_user_box_decls(&program_value))
}

fn resolve_stage1_user_box_decls(program_value: &serde_json::Value) -> Vec<serde_json::Value> {
    match explicit_stage1_user_box_decls(program_value) {
        Some(user_box_decls) => user_box_decls,
        None => build_stage1_user_box_decls(program_value),
    }
}

fn explicit_stage1_user_box_decls(
    program_value: &serde_json::Value,
) -> Option<Vec<serde_json::Value>> {
    let decls = program_value.get("user_box_decls")?.as_array()?;
    Some(
        decls
            .iter()
            .filter_map(normalize_stage1_user_box_decl)
            .collect(),
    )
}

fn normalize_stage1_user_box_decl(decl: &serde_json::Value) -> Option<serde_json::Value> {
    let name = decl.get("name")?.as_str()?.trim();
    if name.is_empty() {
        return None;
    }
    let fields = decl
        .get("fields")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    Some(serde_json::json!({ "name": name, "fields": fields }))
}

fn stage1_user_box_decl_map(
    user_box_decls: &[serde_json::Value],
) -> std::collections::HashMap<String, Vec<String>> {
    user_box_decls
        .iter()
        .filter_map(stage1_user_box_decl_entry)
        .collect()
}

fn stage1_user_box_decl_entry(
    decl: &serde_json::Value,
) -> Option<(String, Vec<String>)> {
    let name = decl.get("name")?.as_str()?.trim();
    if name.is_empty() {
        return None;
    }
    let fields = decl
        .get("fields")
        .and_then(serde_json::Value::as_array)
        .map(|fields| {
            fields
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    Some((name.to_string(), fields))
}

fn parse_program_json_value(program_json: &str) -> Result<serde_json::Value, String> {
    serde_json::from_str(program_json)
        .map_err(|error| format!("program json parse error: {}", error))
}

fn collect_stage1_user_box_decl_names(
    program_value: &serde_json::Value,
) -> std::collections::BTreeSet<String> {
    let mut seen = std::collections::BTreeSet::new();
    seen.insert("Main".to_string());
    insert_stage1_def_box_names(program_value, &mut seen);
    seen
}

fn insert_stage1_def_box_names(
    program_value: &serde_json::Value,
    seen: &mut std::collections::BTreeSet<String>,
) {
    if let Some(defs) = program_value
        .get("defs")
        .and_then(serde_json::Value::as_array)
    {
        for def in defs {
            if let Some(box_name) = def.get("box").and_then(serde_json::Value::as_str) {
                if !box_name.is_empty() {
                    seen.insert(box_name.to_string());
                }
            }
        }
    }
}

fn build_stage1_user_box_decls_from_names(
    names: std::collections::BTreeSet<String>,
) -> Vec<serde_json::Value> {
    names
        .into_iter()
        .map(stage1_user_box_decl_from_name)
        .collect()
}

fn stage1_user_box_decl_from_name(name: String) -> serde_json::Value {
    serde_json::json!({ "name": name, "fields": [] })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_test_ring0() {
        let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    }

    #[test]
    fn test_imports_resolution() {
        ensure_test_ring0();
        // Program JSON with MatI64.new(4, 4)
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "body": [
                {
                    "type": "Local",
                    "name": "n",
                    "expr": {"type": "Int", "value": 4}
                },
                {
                    "type": "Local",
                    "name": "A",
                    "expr": {
                        "type": "Method",
                        "recv": {"type": "Var", "name": "MatI64"},
                        "method": "new",
                        "args": [
                            {"type": "Var", "name": "n"},
                            {"type": "Var", "name": "n"}
                        ]
                    }
                },
                {
                    "type": "Return",
                    "expr": {
                        "type": "Method",
                        "recv": {"type": "Var", "name": "A"},
                        "method": "at",
                        "args": [
                            {"type": "Int", "value": 0},
                            {"type": "Int", "value": 0}
                        ]
                    }
                }
            ]
        }"#;

        // Create imports map
        let mut imports = BTreeMap::new();
        imports.insert("MatI64".to_string(), "MatI64".to_string());

        // Call with imports
        let result = program_json_to_mir_json_with_imports(program_json, imports);

        // Should succeed
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        // MIR JSON should contain functions
        assert!(
            mir_json.contains("functions"),
            "MIR JSON should contain functions"
        );
    }

    #[test]
    fn test_stageb_program_json_with_stagebdriver_main_call() {
        ensure_test_ring0();
        let program_json = r#"{
            "body": [
                {
                    "expr": {
                        "args": [{"name": "args", "type": "Var"}],
                        "name": "StageBDriverBox.main",
                        "type": "Call"
                    },
                    "type": "Return"
                }
            ],
            "kind": "Program",
            "version": 0
        }"#;

        let result = program_json_to_mir_json(program_json);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());
        let mir_json = result.unwrap();
        assert!(mir_json.contains("functions"));
    }

    #[test]
    fn test_program_json_to_mir_json_keeps_main_params_canonical_for_core_exec() {
        ensure_test_ring0();
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "body": [
                {
                    "type": "Return",
                    "expr": {"type": "Int", "value": 42}
                }
            ]
        }"#;

        let result = program_json_to_mir_json(program_json);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&mir_json).expect("mir json must parse as JSON");
        assert_eq!(
            parsed["functions"][0]["params"],
            serde_json::json!([0]),
            "main params must stay canonical for core exec"
        );
    }

    #[test]
    fn test_imported_alias_qualified_call_uses_json_imports() {
        ensure_test_ring0();
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "imports": {
                "BuildBox": "lang.compiler.build.build_box"
            },
            "body": [
                {
                    "type": "Return",
                    "expr": {
                        "type": "Call",
                        "name": "BuildBox.emit_program_json_v0",
                        "args": [
                            {"type": "Str", "value": "box MainBox { method main() { return 1 } }"},
                            {"type": "Null"}
                        ]
                    }
                }
            ]
        }"#;

        let result = program_json_to_mir_json(program_json);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        assert!(mir_json.contains("lang.compiler.build.build_box"));
        assert!(!mir_json.contains("\"BuildBox.emit_program_json_v0\""));
    }

    #[test]
    fn test_source_to_mir_json_handles_stage1_cli_env_source() {
        ensure_test_ring0();
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let result = source_to_mir_json(source);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        assert!(mir_json.contains("functions"));
        assert!(mir_json.contains("user_box_decls"));
    }

    #[test]
    fn test_program_json_to_mir_json_with_user_box_decls_keeps_explicit_route_contract() {
        ensure_test_ring0();
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "defs": [
                {
                    "box": "HelperBox",
                    "name": "helper",
                    "params": [],
                    "body": {"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":1}}]}
                }
            ],
            "body": [
                {
                    "type": "Return",
                    "expr": {"type": "Int", "value": 42}
                }
            ]
        }"#;

        let result = program_json_to_mir_json_with_user_box_decls(program_json);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        assert!(mir_json.contains("user_box_decls"));
        assert!(mir_json.contains("\"name\":\"Main\""));
        assert!(mir_json.contains("\"name\":\"HelperBox\""));
    }

    #[test]
    fn test_program_json_to_mir_json_with_user_box_decls_prefers_explicit_payload() {
        ensure_test_ring0();
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "user_box_decls": [
                {"name": "Main", "fields": []},
                {"name": "ExplicitBox", "fields": ["value"]}
            ],
            "defs": [
                {
                    "box": "CompatOnlyBox",
                    "name": "helper",
                    "params": [],
                    "body": {"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":1}}]}
                }
            ],
            "body": [
                {
                    "type": "Return",
                    "expr": {"type": "Int", "value": 42}
                }
            ]
        }"#;

        let result = program_json_to_mir_json_with_user_box_decls(program_json);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let mir_json = result.unwrap();
        assert!(mir_json.contains("\"user_box_decls\""));
        assert!(mir_json.contains("\"name\":\"ExplicitBox\""));
        assert!(mir_json.contains("\"fields\":[\"value\"]"));
        assert!(!mir_json.contains("\"name\":\"CompatOnlyBox\""));
    }

    #[test]
    fn test_source_to_program_and_mir_json_returns_program_and_mir() {
        ensure_test_ring0();
        let source = include_str!("../../lang/src/runner/stage1_cli_env.hako");
        let result = source_to_program_and_mir_json(source);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let (program_json, mir_json) = result.unwrap();
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(mir_json.contains("functions"));
    }

    #[test]
    fn test_source_to_program_and_mir_json_handles_hello_simple_llvm_source() {
        ensure_test_ring0();
        let source = include_str!("../../apps/tests/hello_simple_llvm.hako");
        let result = source_to_program_and_mir_json(source);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let (program_json, mir_json) = result.unwrap();
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(program_json.contains("env.console.log"));
        assert!(mir_json.contains("\"functions\""));
    }

    #[test]
    fn test_source_to_program_and_mir_json_accepts_decode_escapes_nested_loop_fixture() {
        ensure_test_ring0();
        let source = include_str!(
            "../../apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"
        );
        let result = source_to_program_and_mir_json(source);
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());

        let (program_json, mir_json) = result.unwrap();
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(mir_json.contains("\"functions\""));
    }

    #[test]
    fn test_source_to_program_and_mir_json_rejects_dev_local_alias_sugar_on_authority_path() {
        ensure_test_ring0();
        let source = r#"
static box Main {
  main() {
    @x = 41
    return x + 1
  }
}
"#;
        let result = source_to_program_and_mir_json(source);
        let error = result.expect_err("authority path should stay strict");
        assert!(error.contains(FAILFAST_TAG), "unexpected error: {error}");
        assert!(
            error.contains(
                "source route rejects compat-only relaxed-compat source shape (dev-local-alias-sugar)"
            ),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn test_source_to_program_and_mir_json_rejects_launcher_on_authority_path() {
        ensure_test_ring0();
        let source = include_str!("../../lang/src/runner/launcher.hako");
        let result = source_to_program_and_mir_json(source);
        let error = result.expect_err("launcher should remain compat-only on authority path");
        assert!(error.contains(FAILFAST_TAG), "unexpected error: {error}");
        assert!(
            error.contains(
                "source route rejects compat-only relaxed-compat source shape (dev-local-alias-sugar)"
            ),
            "unexpected error: {error}"
        );
    }
}
