use crate::mir::MirCompiler;
use crate::runner::NyashRunner;
use std::collections::BTreeSet;

fn json_v0_import_bundle_trace_enabled() -> bool {
    crate::config::env::json_v0_import_bundle_trace_enabled()
}

fn trace_json_v0_import_bundle_summary<S: AsRef<str>>(msg: S) {
    if json_v0_import_bundle_trace_enabled() {
        crate::runtime::get_global_ring0().log.info(msg.as_ref());
    }
}

fn trace_json_v0_import_bundle_detail<S: AsRef<str>>(msg: S) {
    if json_v0_import_bundle_trace_enabled() {
        crate::runner::trace::log(msg.as_ref());
    }
}

fn trace_json_v0_import_bundle_sanitize(msg: &str) -> String {
    msg.replace('\n', "\\n").replace('\r', "\\r")
}

pub(super) fn load_program_json_v0_to_module(
    runner: &NyashRunner,
    json: &str,
) -> Result<crate::mir::MirModule, String> {
    let mut module = crate::runner::json_v0_bridge::parse_json_v0_to_module(json)
        .map_err(|error| format!("JSON v0 bridge error: {}", error))?;
    maybe_merge_program_json_v0_imports(runner, json, &mut module)?;
    Ok(module)
}

fn maybe_merge_program_json_v0_imports(
    runner: &NyashRunner,
    json: &str,
    module: &mut crate::mir::MirModule,
) -> Result<(), String> {
    let import_targets = extract_program_json_v0_used_import_targets(json)?;
    trace_json_v0_import_bundle_summary(format!(
        "[json_v0/import_bundle] phase=enter source=Program(JSON v0) used_targets={}",
        import_targets.len()
    ));
    if import_targets.is_empty() {
        trace_json_v0_import_bundle_summary(
            "[json_v0/import_bundle] phase=skip reason=no_used_import_targets",
        );
        return Ok(());
    }

    trace_json_v0_import_bundle_detail(format!(
        "[json_v0/import_bundle] phase=merge.begin targets={}",
        import_targets.len()
    ));
    let import_module = compile_program_json_v0_imports_bundle(runner, &import_targets)?;
    for (name, function) in import_module.functions {
        if name == "main" {
            continue;
        }
        if module.functions.contains_key(&name) {
            trace_json_v0_import_bundle_summary(format!(
                "[json_v0/import_bundle] phase=fail result=duplicate_function name={}",
                name
            ));
            return Err(format!(
                "[freeze:contract][json_v0/imports] duplicate function: {}",
                name
            ));
        }
        module.functions.insert(name, function);
    }
    trace_json_v0_import_bundle_summary(format!(
        "[json_v0/import_bundle] phase=merge.done targets={} merged_functions={}",
        import_targets.len(),
        module.functions.len()
    ));
    Ok(())
}

fn extract_program_json_v0_used_import_targets(json: &str) -> Result<Vec<String>, String> {
    let value: serde_json::Value = serde_json::from_str(json)
        .map_err(|error| format!("[freeze:contract][json_v0/imports] invalid JSON: {error}"))?;
    let object = value
        .as_object()
        .ok_or_else(|| "[freeze:contract][json_v0/imports] expected object".to_string())?;
    let version = object
        .get("version")
        .and_then(|value| value.as_i64())
        .ok_or_else(|| "[freeze:contract][json_v0/imports] missing version".to_string())?;
    let kind = object
        .get("kind")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "[freeze:contract][json_v0/imports] missing kind".to_string())?;
    if version != 0 || kind != "Program" {
        return Ok(Vec::new());
    }

    let Some(imports) = object.get("imports") else {
        return Ok(Vec::new());
    };
    let Some(map) = imports.as_object() else {
        return Err("[freeze:contract][json_v0/imports] imports must be an object".to_string());
    };

    let mut used_aliases = BTreeSet::new();
    if let Some(body) = object.get("body") {
        collect_program_json_v0_used_aliases(body, &mut used_aliases);
    }

    let mut out = BTreeSet::new();
    for (alias, target) in map {
        let Some(target) = target.as_str() else {
            return Err(
                "[freeze:contract][json_v0/imports] imports value must be string".to_string(),
            );
        };
        let target = target.trim();
        if target.is_empty() {
            continue;
        }
        if used_aliases.contains(alias) {
            out.insert(target.to_string());
        }
    }
    Ok(out.into_iter().collect())
}

fn collect_program_json_v0_used_aliases(value: &serde_json::Value, out: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::Object(object) => {
            if object.get("type").and_then(|value| value.as_str()) == Some("Var") {
                if let Some(name) = object.get("name").and_then(|value| value.as_str()) {
                    out.insert(name.to_string());
                }
            }
            if object.get("type").and_then(|value| value.as_str()) == Some("Call") {
                if let Some(name) = object.get("name").and_then(|value| value.as_str()) {
                    let alias = name.split('.').next().unwrap_or(name).trim();
                    if !alias.is_empty() {
                        out.insert(alias.to_string());
                    }
                }
            }
            for child in object.values() {
                collect_program_json_v0_used_aliases(child, out);
            }
        }
        serde_json::Value::Array(items) => {
            for child in items {
                collect_program_json_v0_used_aliases(child, out);
            }
        }
        _ => {}
    }
}

fn compile_program_json_v0_imports_bundle(
    runner: &NyashRunner,
    targets: &[String],
) -> Result<crate::mir::MirModule, String> {
    use crate::parser::NyashParser;
    use crate::runner::modes::common_util::resolve::prelude_manager::PreludeManagerBox;
    use crate::runner::modes::common_util::resolve::strip::resolve_prelude_paths_profiled;
    use crate::using::resolver::resolve_using_target_common;

    struct EnvVarRestore {
        key: &'static str,
        prev: Option<std::ffi::OsString>,
        trace_enabled: bool,
    }

    impl EnvVarRestore {
        fn set(key: &'static str, value: &str, trace_enabled: bool) -> Self {
            let prev = std::env::var_os(key);
            std::env::set_var(key, value);
            if trace_enabled {
                trace_json_v0_import_bundle_detail(format!(
                    "[json_v0/import_bundle] phase=guard.set key={} prev={}",
                    key,
                    if prev.is_some() { "set" } else { "unset" }
                ));
            }
            Self {
                key,
                prev,
                trace_enabled,
            }
        }
    }

    impl Drop for EnvVarRestore {
        fn drop(&mut self) {
            if self.trace_enabled {
                trace_json_v0_import_bundle_detail(format!(
                    "[json_v0/import_bundle] phase=restore key={} prev={}",
                    self.key,
                    if self.prev.is_some() { "set" } else { "unset" }
                ));
            }
            match self.prev.take() {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }

    let trace_enabled = json_v0_import_bundle_trace_enabled();
    if trace_enabled {
        trace_json_v0_import_bundle_summary(format!(
            "[json_v0/import_bundle] phase=enter targets={}",
            targets.len()
        ));
    }

    let result = (|| -> Result<crate::mir::MirModule, String> {
        let _guard_strict = EnvVarRestore::set("HAKO_JOINIR_STRICT", "1", trace_enabled);
        let _guard_planner_required =
            EnvVarRestore::set("HAKO_JOINIR_PLANNER_REQUIRED", "1", trace_enabled);

        let using_ctx = runner.init_using_context();
        let verbose = crate::config::env::cli_verbose();

        let mut prelude_paths: Vec<String> = Vec::new();
        let mut seen_prelude = std::collections::HashSet::new();
        let mut cleaned_roots: Vec<String> = Vec::new();

        for target in targets {
            let resolved = resolve_using_target_common(
                target,
                &using_ctx.pending_modules,
                &using_ctx.module_roots,
                &using_ctx.using_paths,
                &using_ctx.packages,
                None,
                true,
                verbose,
            )
            .map_err(|error| format!("[freeze:contract][json_v0/imports] {error}"))?;

            let abs = std::fs::canonicalize(&resolved).map_err(|error| {
                format!(
                    "[freeze:contract][json_v0/imports] canonicalize {}: {error}",
                    resolved
                )
            })?;
            let path = abs.to_string_lossy().to_string();
            let source = std::fs::read_to_string(&path).map_err(|error| {
                format!("[freeze:contract][json_v0/imports] read {}: {error}", path)
            })?;

            let (cleaned, nested_preludes) = resolve_prelude_paths_profiled(runner, &source, &path)
                .map_err(|error| format!("[freeze:contract][json_v0/imports] using: {error}"))?;
            cleaned_roots.push(cleaned);
            for prelude in nested_preludes {
                if seen_prelude.insert(prelude.clone()) {
                    prelude_paths.push(prelude);
                }
            }
        }

        let mut main_source = String::new();
        for (index, source) in cleaned_roots.iter().enumerate() {
            if index > 0 {
                main_source.push_str("\n\n");
            }
            main_source.push_str(source);
        }

        let prelude_manager = PreludeManagerBox::new(runner);
        let merged = if crate::config::env::using_ast_enabled() {
            prelude_manager
                .merge_ast(&main_source, "<json_v0/imports>", &prelude_paths)
                .map_err(|error| format!("[freeze:contract][json_v0/imports] merge_ast: {error}"))?
        } else {
            prelude_manager
                .merge_text(&main_source, "<json_v0/imports>", &prelude_paths)
                .map_err(|error| {
                    format!("[freeze:contract][json_v0/imports] merge_text: {error}")
                })?
        };

        let ast = NyashParser::parse_from_string(&merged.merged_content)
            .map_err(|error| format!("[freeze:contract][json_v0/imports] parse: {error}"))?;
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);

        let mut compiler = MirCompiler::with_options(true);
        let compile = crate::runner::modes::common_util::source_hint::compile_with_source_hint(
            &mut compiler,
            ast,
            Some("<json_v0/imports>"),
        )
        .map_err(|error| format!("[freeze:contract][json_v0/imports] compile: {error}"))?;
        Ok(compile.module)
    })();

    if trace_enabled {
        match &result {
            Ok(module) => trace_json_v0_import_bundle_summary(format!(
                "[json_v0/import_bundle] phase=exit result=ok functions={}",
                module.functions.len()
            )),
            Err(error) => trace_json_v0_import_bundle_summary(format!(
                "[json_v0/import_bundle] phase=fail result=err err={}",
                trace_json_v0_import_bundle_sanitize(error)
            )),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::extract_program_json_v0_used_import_targets;

    #[test]
    fn extract_program_json_v0_used_import_targets_keeps_only_call_alias_references() {
        let json = r#"{
            "kind":"Program",
            "version":0,
            "imports":{"foo":"alpha.hako","bar":"beta.hako","baz":"gamma.hako"},
            "body":{"type":"Block","stmts":[{"type":"Call","name":"foo.emit_program_json_v0","args":[]}]}
        }"#;

        let targets =
            extract_program_json_v0_used_import_targets(json).expect("import targets should parse");
        assert_eq!(targets, vec!["alpha.hako".to_string()]);
    }

    #[test]
    fn extract_program_json_v0_used_import_targets_ignores_non_program_payloads() {
        let json = r#"{
            "kind":"Module",
            "version":0,
            "imports":{"foo":"alpha.hako"},
            "body":{"type":"Var","name":"foo"}
        }"#;

        let targets =
            extract_program_json_v0_used_import_targets(json).expect("non-program should parse");
        assert!(targets.is_empty());
    }
}
