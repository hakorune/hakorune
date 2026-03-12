/*!
 * Stage-1 bridge stub emit - stdout parse / validation helper.
 */

use super::Stage1StubEmitMode;
use crate::mir::MirModule;

pub(super) enum ParsedStage1StubEmitPayload {
    MirJson { line: String, module: MirModule },
    ProgramJsonV0 { line: String },
}

pub(super) fn parse_payload(
    stdout: &str,
    mode: Stage1StubEmitMode,
) -> Result<ParsedStage1StubEmitPayload, String> {
    match mode {
        Stage1StubEmitMode::MirJson => parse_mir_json(stdout),
        Stage1StubEmitMode::ProgramJsonV0 => parse_program_json(stdout),
    }
}

fn parse_mir_json(stdout: &str) -> Result<ParsedStage1StubEmitPayload, String> {
    let line = crate::runner::modes::common_util::selfhost::json::first_mir_json_v0_line(stdout)
        .ok_or_else(|| "[stage1-cli] emit-mir: no MIR(JSON v0) found in stub output".to_string())?;
    let module = crate::runner::modes::common_util::selfhost::json::parse_mir_json_v0_line(&line)
        .map_err(|error| {
        format!("[stage1-cli] emit-mir: MIR(JSON v0) parse error: {}", error)
    })?;
    Ok(ParsedStage1StubEmitPayload::MirJson { line, module })
}

fn parse_program_json(stdout: &str) -> Result<ParsedStage1StubEmitPayload, String> {
    let line = crate::runner::modes::common_util::selfhost::json::first_json_v0_line(stdout)
        .ok_or_else(|| {
            "[stage1-cli] emit-program: no Program(JSON v0) found in stub output".to_string()
        })?;
    crate::runner::modes::common_util::selfhost::json::parse_json_v0_line(&line).map_err(
        |error| {
            format!(
                "[stage1-cli] emit-program: Program(JSON v0) parse error: {}",
                error
            )
        },
    )?;
    Ok(ParsedStage1StubEmitPayload::ProgramJsonV0 { line })
}
