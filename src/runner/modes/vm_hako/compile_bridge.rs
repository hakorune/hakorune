use super::{temp_seed, VmHakoErr, VM_HAKO_PHASE};
use crate::mir::{MirCompiler, MirModule};
use crate::runner::NyashRunner;
use nyash_rust::parser::NyashParser;
use std::collections::HashMap;

pub(super) fn compile_source_to_mir_json_v0(
    runner: &NyashRunner,
    filename: &str,
    code: &str,
) -> Result<String, VmHakoErr> {
    let (prepared_source, using_imports) =
        prepare_vm_hako_source_and_imports(runner, filename, code)?;

    let ast = match NyashParser::parse_from_string(&prepared_source) {
        Ok(ast) => ast,
        Err(e) => {
            crate::runner::modes::common_util::diag::print_parse_error_with_context(
                filename,
                &prepared_source,
                &e,
            );
            return Err(("parse-error", e.to_string()));
        }
    };
    let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);

    let mut compiler = MirCompiler::with_options(!runner.config.no_optimize);
    let compile_result =
        match crate::runner::modes::common_util::source_hint::compile_with_source_hint_and_imports(
            &mut compiler,
            ast,
            Some(filename),
            using_imports,
        ) {
            Ok(result) => result,
            Err(e) => return Err(("compile-error", e.to_string())),
        };
    crate::runner::modes::common_util::verifier_gate::enforce_vm_verify_gate_or_exit(
        &compile_result.module,
        "vm-hako",
    );
    crate::runner::modes::common_util::safety_gate::enforce_vm_lifecycle_safety_or_exit(
        &compile_result.module,
        "vm-hako",
    );
    emit_mir_json_v0_string(&compile_result.module).map_err(|e| ("emit-error", e))
}

fn prepare_vm_hako_source_and_imports(
    runner: &NyashRunner,
    filename: &str,
    code: &str,
) -> Result<(String, HashMap<String, String>), VmHakoErr> {
    let prepared = match crate::runner::modes::common_util::source_hint::prepare_source_with_imports(
        runner, filename, code,
    ) {
        Ok(prepared) => prepared,
        Err(e) => return Err(("resolve-error", e)),
    };

    crate::runner::modes::common_util::safety_gate::enforce_vm_source_safety_or_exit(
        &prepared.code,
        "vm-hako",
    );

    Ok((prepared.code, prepared.imports))
}

fn emit_mir_json_v0_string(module: &MirModule) -> Result<String, String> {
    let path = std::env::temp_dir().join(format!(
        "vm_hako_{}_mir_{}.json",
        VM_HAKO_PHASE,
        temp_seed()
    ));
    let _unified_guard = ScopedEnvVar::set("NYASH_MIR_UNIFIED_CALL", "1");
    let _schema_guard = ScopedEnvVar::set("NYASH_JSON_SCHEMA_V1", "1");
    let emit_result = crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(module, &path);

    if let Err(e) = emit_result {
        let _ = std::fs::remove_file(&path);
        return Err(e);
    }
    let out = std::fs::read_to_string(&path).map_err(|e| e.to_string());
    let _ = std::fs::remove_file(&path);
    out
}

struct ScopedEnvVar {
    key: &'static str,
    prev: Option<String>,
}

impl ScopedEnvVar {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, prev }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        if let Some(v) = &self.prev {
            std::env::set_var(self.key, v);
        } else {
            std::env::remove_var(self.key);
        }
    }
}
