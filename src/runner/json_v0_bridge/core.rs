use super::ast::{ProgramV0, StmtV0};
use super::lowering::lower_program;
use std::collections::BTreeMap;

pub fn parse_json_v0_to_module(json: &str) -> Result<crate::mir::MirModule, String> {
    parse_json_v0_to_module_with_imports(json, BTreeMap::new())
}

pub fn parse_json_v0_to_module_with_imports(
    json: &str,
    mut imports: BTreeMap<String, String>,
) -> Result<crate::mir::MirModule, String> {
    let prog: ProgramV0 =
        serde_json::from_str(json).map_err(|e| format!("invalid JSON v0: {}", e))?;
    if crate::config::env::cli_verbose() {
        let first = prog
            .body
            .get(1)
            .map(|s| match s {
                StmtV0::Try { .. } => "Try",
                StmtV0::FiniReg { .. } => "FiniReg",
                _ => "Other",
            })
            .unwrap_or("<none>");
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[Bridge] JSON v0: body_len={} first_type={}",
            prog.body.len(),
            first
        ));
    }
    if prog.version != 0 || prog.kind != "Program" {
        return Err("unsupported IR: expected {version:0, kind:\"Program\"}".into());
    }
    // Phase 29bq: Merge imports from JSON with passed-in imports
    // JSON imports take precedence over passed-in imports
    for (alias, module_path) in &prog.imports {
        imports.insert(alias.clone(), module_path.clone());
    }

    let mut module = lower_program(prog, imports)?;
    // Keep Program(JSON v0) bridge aligned with runtime preflight callsite contract.
    let _ = crate::mir::passes::callsite_canonicalize::canonicalize_callsites(&mut module);
    crate::mir::refresh_module_thin_entry_candidates(&mut module);
    crate::mir::refresh_module_thin_entry_selections(&mut module);
    crate::mir::refresh_module_sum_placement_facts(&mut module);
    crate::mir::refresh_module_sum_placement_selections(&mut module);
    crate::mir::refresh_module_sum_placement_layouts(&mut module);
    crate::mir::refresh_module_string_kernel_plans(&mut module);
    Ok(module)
}

pub fn parse_source_v0_to_module(input: &str) -> Result<crate::mir::MirModule, String> {
    let json = super::lexer::parse_source_v0_to_json(input)?;
    if std::env::var("NYASH_DUMP_JSON_IR").ok().as_deref() == Some("1") {
        println!("{}", json);
    }
    parse_json_v0_to_module(&json)
}

pub fn maybe_dump_mir(module: &crate::mir::MirModule) {
    super::lowering::maybe_dump_mir(module)
}
