/*!
 * Stage-1 bridge stub emit - stdout parse / validation helper.
 */

use super::Stage1StubEmitMode;
use crate::mir::MirModule;

#[derive(Debug)]
pub(super) enum ParsedStage1StubEmitPayload {
    MirJson { line: String, module: MirModule },
    ProgramJsonV0 { line: String },
}

pub(super) fn parse_payload(
    stdout: &str,
    mode: Stage1StubEmitMode,
) -> Result<ParsedStage1StubEmitPayload, String> {
    match mode {
        Stage1StubEmitMode::MirJson => parse_mir_payload(stdout),
        Stage1StubEmitMode::ProgramJsonV0 => parse_program_payload(stdout),
    }
}

fn parse_mir_payload(stdout: &str) -> Result<ParsedStage1StubEmitPayload, String> {
    let line = first_mir_json_line(stdout)?;
    let module = parse_mir_json_line(&line)?;
    Ok(ParsedStage1StubEmitPayload::MirJson { line, module })
}

fn parse_program_payload(stdout: &str) -> Result<ParsedStage1StubEmitPayload, String> {
    let line = first_program_json_line(stdout)?;
    parse_program_json_line(&line)?;
    Ok(ParsedStage1StubEmitPayload::ProgramJsonV0 { line })
}

fn first_mir_json_line(stdout: &str) -> Result<String, String> {
    crate::runner::modes::common_util::selfhost::json::first_mir_json_v0_line(stdout)
        .ok_or_else(|| "[stage1-cli] emit-mir: no MIR(JSON v0) found in stub output".to_string())
}

fn parse_mir_json_line(line: &str) -> Result<MirModule, String> {
    crate::runner::modes::common_util::selfhost::json::parse_mir_json_v0_line(line)
        .map_err(|error| format!("[stage1-cli] emit-mir: MIR(JSON v0) parse error: {}", error))
}

fn first_program_json_line(stdout: &str) -> Result<String, String> {
    crate::runner::modes::common_util::selfhost::json::first_json_v0_line(stdout).ok_or_else(
        || "[stage1-cli] emit-program: no Program(JSON v0) found in stub output".to_string(),
    )
}

fn parse_program_json_line(line: &str) -> Result<(), String> {
    crate::runner::modes::common_util::selfhost::json::parse_json_v0_line(line).map_err(|error| {
        format!(
            "[stage1-cli] emit-program: Program(JSON v0) parse error: {}",
            error
        )
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        parse_payload, ParsedStage1StubEmitPayload,
    };
    use crate::runner::stage1_bridge::args::Stage1StubEmitMode;

    #[test]
    fn parse_payload_reports_exact_missing_program_json_prefix() {
        let error = parse_payload("not-json", Stage1StubEmitMode::ProgramJsonV0)
            .expect_err("missing program json must fail");
        assert_eq!(
            error,
            "[stage1-cli] emit-program: no Program(JSON v0) found in stub output"
        );
    }

    #[test]
    fn parse_payload_reports_exact_mir_parse_error_prefix() {
        let error = parse_payload("{\"functions\":", Stage1StubEmitMode::MirJson)
            .expect_err("invalid mir json must fail");
        assert!(
            error.starts_with("[stage1-cli] emit-mir: MIR(JSON v0) parse error:"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn parse_payload_accepts_program_json_line() {
        let payload = parse_payload(
            "{\"version\":0,\"kind\":\"Program\",\"body\":[{\"type\":\"Return\",\"expr\":{\"type\":\"Int\",\"value\":1}}]}",
            Stage1StubEmitMode::ProgramJsonV0,
        )
        .expect("program json payload");
        match payload {
            ParsedStage1StubEmitPayload::ProgramJsonV0 { line } => {
                assert!(line.contains("\"kind\":\"Program\""));
            }
            ParsedStage1StubEmitPayload::MirJson { .. } => {
                panic!("expected Program(JSON) payload")
            }
        }
    }
}
