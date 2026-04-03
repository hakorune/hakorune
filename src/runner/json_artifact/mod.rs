/*!
 * JSON artifact loaders — classify JSON artifacts and lower them to `MirModule`.
 *
 * Responsibility
 * - Mainline `MIR(JSON)` intake.
 * - Compat `Program(JSON v0)` intake.
 * - Shared convergence: `load artifact -> MirModule`.
 */

mod mir_loader;
mod program_json_v0_loader;

use super::NyashRunner;

pub(crate) fn load_json_artifact_to_module(
    runner: &NyashRunner,
    json: &str,
) -> Result<crate::mir::MirModule, String> {
    // Artifact-family classifier: MIR(JSON) mainline first, Program(JSON v0) compat second.
    // Callers that already hold direct MIR(JSON) should stay on the core-executor seam instead.
    let mut payload = json.to_string();

    if crate::config::env::nyvm_v1_downconvert() {
        if let Ok(canonicalized) =
            crate::runner::modes::common_util::core_bridge::canonicalize_module_json(&payload)
        {
            payload = canonicalized;
        }
    }

    if let Some(module) = mir_loader::load_mir_json_to_module(&payload)? {
        return Ok(module);
    }

    if let Ok(canonicalized) =
        crate::runner::modes::common_util::core_bridge::canonicalize_module_json(&payload)
    {
        payload = canonicalized;
        if let Some(module) = mir_loader::load_mir_json_to_module(&payload)? {
            return Ok(module);
        }
    }

    program_json_v0_loader::load_program_json_v0_to_module(runner, &payload)
}

pub(crate) fn load_mir_json_to_module(text: &str) -> Result<Option<crate::mir::MirModule>, String> {
    mir_loader::load_mir_json_to_module(text)
}

pub(crate) fn parse_direct_mir_json_text_with_v0_fallback(
    text: &str,
    path: &str,
) -> Result<crate::mir::MirModule, String> {
    mir_loader::parse_direct_mir_json_text_with_v0_fallback(text, path)
}

#[cfg(test)]
mod tests {
    use super::load_mir_json_to_module;

    #[test]
    fn load_mir_json_to_module_returns_none_for_program_json_v0() {
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "body": [{"type":"Return","expr":{"type":"Int","value":1}}]
        }"#;

        let result = load_mir_json_to_module(program_json).expect("program json should not error");
        assert!(result.is_none());
    }
}
