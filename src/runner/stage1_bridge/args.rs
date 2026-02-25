/*!
 * Stage-1 CLI bridge - args builder
 *
 * Constructs stage1_args based on execution mode (emit_program / emit_mir / run).
 */

use crate::cli::CliGroups;
use crate::config::env::stage1;
use serde_json;
use std::process;

/// Stage-1 args construction result
#[derive(Debug)]
pub(super) struct Stage1Args {
    pub args: Vec<String>,
    pub env_script_args: Option<String>,
    pub source_env: Option<String>,
    pub progjson_env: Option<String>,
    pub emit_mir: bool,
}

/// Build stage1_args based on execution mode
///
/// # Modes
/// - emit_program: emit program-json <source.hako>
/// - emit_mir: emit mir-json (<source.hako> or STAGE1_PROGRAM_JSON)
/// - run: run --backend <backend> <source.hako>
pub(super) fn build_stage1_args(groups: &CliGroups) -> Stage1Args {
    // Prefer new env (NYASH_STAGE1_*) and fall back to legacy names to keep compatibility.
    let source = stage1::input_path().or_else(|| groups.input.file.as_ref().cloned());

    let emit_program = stage1::emit_program_json();
    let emit_mir = stage1::emit_mir_json();

    let mut args: Vec<String> = Vec::new();
    let mut source_env: Option<String> = None;
    let mut progjson_env: Option<String> = None;

    if emit_program {
        let src = source.as_ref().cloned().unwrap_or_else(|| {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.error("[stage1-cli] STAGE1_EMIT_PROGRAM_JSON=1 but no input file provided");
            process::exit(97);
        });
        args.push("emit".into());
        args.push("program-json".into());
        args.push(src);
        source_env = args.last().cloned();
    } else if emit_mir {
        if let Some(pjson) = stage1::program_json_path() {
            args.push("emit".into());
            args.push("mir-json".into());
            args.push("--from-program-json".into());
            args.push(pjson);
            progjson_env = args.last().cloned();
        } else {
            let src = source.as_ref().cloned().unwrap_or_else(|| {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.error("[stage1-cli] STAGE1_EMIT_MIR_JSON=1 but no input file provided");
                process::exit(97);
            });
            args.push("emit".into());
            args.push("mir-json".into());
            args.push(src);
            source_env = args.last().cloned();
        }
    } else {
        let src = source.as_ref().cloned().unwrap_or_else(|| {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.error("[stage1-cli] NYASH_USE_STAGE1_CLI=1 requires an input file to run");
            process::exit(97);
        });
        args.push("run".into());
        let backend = stage1::backend_hint().unwrap_or_else(|| groups.backend.backend.clone());
        args.push("--backend".into());
        args.push(backend);
        args.push(src);
        source_env = args.last().cloned();
    }

    // Forward script args provided to the parent process (via -- arg1 arg2 ...)
    if let Ok(json) = std::env::var("NYASH_SCRIPT_ARGS_JSON") {
        if let Ok(mut extras) = serde_json::from_str::<Vec<String>>(&json) {
            args.append(&mut extras);
        }
    }

    // Also pass args via env to guarantee argv is well-defined in the stub.
    let env_script_args = if std::env::var("NYASH_SCRIPT_ARGS_JSON").is_err() {
        serde_json::to_string(&args).ok()
    } else {
        None
    };

    Stage1Args {
        args,
        env_script_args,
        source_env,
        progjson_env,
        emit_mir,
    }
}
