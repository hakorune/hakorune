pub(super) fn emit_program_json_payload(source_text: &str) -> Result<String, String> {
    crate::stage1::program_json_v0::emit_program_json_v0_for_stage1_bridge_emit_program_json(
        source_text,
    )
}

#[cfg(test)]
mod tests {
    use super::emit_program_json_payload;

    #[test]
    fn emit_program_json_payload_preserves_bridge_error_prefix() {
        let relaxed_source = include_str!("../../../../lang/src/runner/launcher.hako");
        let error =
            emit_program_json_payload(relaxed_source).expect_err("launcher source should fail");
        assert!(
            error.starts_with("emit-program-json-v0: "),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn emit_program_json_payload_returns_program_json_for_strict_source() {
        let strict_source = include_str!("../../../../lang/src/runner/stage1_cli_env.hako");
        let out = emit_program_json_payload(strict_source).expect("strict source payload");
        assert!(out.contains("\"kind\":\"Program\""));
    }
}
