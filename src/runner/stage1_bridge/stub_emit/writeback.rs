/*!
 * Stage-1 bridge stub emit - writeback helper.
 */

use super::parse::ParsedStage1StubEmitPayload;
use crate::cli::CliGroups;
use crate::mir::MirPrinter;
use std::path::Path;

pub(super) fn write(
    groups: &CliGroups,
    payload: ParsedStage1StubEmitPayload,
) -> Result<(), String> {
    match payload {
        ParsedStage1StubEmitPayload::MirJson { line, module } => {
            write_mir_json(groups, &line, &module)
        }
        ParsedStage1StubEmitPayload::ProgramJsonV0 { line } => write_program_json(groups, &line),
    }
}

fn write_mir_json(
    groups: &CliGroups,
    line: &str,
    module: &crate::mir::MirModule,
) -> Result<(), String> {
    crate::runner::json_v0_bridge::maybe_dump_mir(module);
    print_dumped_mir_if_requested(groups, module);
    emit_mir_json_output(groups, line, module)
}

fn write_program_json(groups: &CliGroups, line: &str) -> Result<(), String> {
    emit_program_json_output(groups, line)
}

fn print_dumped_mir_if_requested(groups: &CliGroups, module: &crate::mir::MirModule) {
    if !groups.debug.dump_mir {
        return;
    }
    let mut printer = if groups.debug.mir_verbose {
        MirPrinter::verbose()
    } else {
        MirPrinter::new()
    };
    if groups.debug.mir_verbose_effects {
        printer.set_show_effects_inline(true);
    }
    println!("{}", printer.print_module(module));
}

fn emit_mir_json_output(
    groups: &CliGroups,
    line: &str,
    module: &crate::mir::MirModule,
) -> Result<(), String> {
    match mir_json_out_path(groups) {
        Some(path) => write_mir_json_file(&path, module),
        None => {
            println!("{}", line);
            Ok(())
        }
    }
}

fn mir_json_out_path(groups: &CliGroups) -> Option<String> {
    super::super::emit_paths::resolve_mir_out_path(groups.emit.emit_mir_json.clone())
}

fn write_mir_json_file(path: &str, module: &crate::mir::MirModule) -> Result<(), String> {
    let out_path = Path::new(path);
    crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(module, out_path)
        .map_err(|error| format!("❌ MIR JSON emit error: {}", error))?;
    println!("MIR JSON written: {}", out_path.display());
    Ok(())
}

fn emit_program_json_output(groups: &CliGroups, line: &str) -> Result<(), String> {
    match program_json_out_path(groups) {
        Some(path) => write_program_json_file(&path, line),
        None => {
            println!("{}", line);
            Ok(())
        }
    }
}

fn program_json_out_path(groups: &CliGroups) -> Option<String> {
    super::super::emit_paths::resolve_program_json_out_path(
        groups.emit.emit_program_json_v0.clone(),
    )
}

fn write_program_json_file(path: &str, line: &str) -> Result<(), String> {
    let out_path = Path::new(path);
    std::fs::write(out_path, line.as_bytes())
        .map_err(|error| format!("❌ Program JSON emit error: {}", error))?;
    println!("Program JSON written: {}", out_path.display());
    Ok(())
}
