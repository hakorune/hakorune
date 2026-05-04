// Stage1 compiler bridge exports.
//
// These ABI leaves are thin handle adapters. Program(JSON v0) authority and
// route policy remain in `nyash_rust::stage1::program_json_v0`.

use crate::plugin::{materialize_owned_string, owned_string_from_handle};

#[export_name = "nyash.stage1.emit_program_json_v0_h"]
pub extern "C" fn nyash_stage1_emit_program_json_v0_h(source_h: i64) -> i64 {
    let Some(source_text) = owned_string_from_handle(source_h) else {
        return 0;
    };
    let program_json =
        nyash_rust::stage1::program_json_v0::emit_program_json_v0_for_current_stage1_build_box_mode(
            &source_text,
        );
    match program_json {
        Ok(text) | Err(text) => materialize_owned_string(text),
    }
}

#[export_name = "nyash.stage1.emit_mir_from_source_v0_h"]
pub extern "C" fn nyash_stage1_emit_mir_from_source_v0_h(source_h: i64) -> i64 {
    let Some(source_text) = owned_string_from_handle(source_h) else {
        return 0;
    };
    match nyash_rust::host_providers::mir_builder::source_to_mir_json(&source_text) {
        Ok(text) | Err(text) => materialize_owned_string(text),
    }
}

#[export_name = "nyash.stage1.emit_mir_from_program_json_v0_h"]
pub extern "C" fn nyash_stage1_emit_mir_from_program_json_v0_h(program_json_h: i64) -> i64 {
    let Some(program_json_text) = owned_string_from_handle(program_json_h) else {
        return 0;
    };
    match nyash_rust::host_providers::mir_builder::program_json_to_mir_json_with_user_box_decls(
        &program_json_text,
    ) {
        Ok(text) | Err(text) => materialize_owned_string(text),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        nyash_stage1_emit_mir_from_program_json_v0_h, nyash_stage1_emit_mir_from_source_v0_h,
        nyash_stage1_emit_program_json_v0_h,
    };
    use crate::plugin::{materialize_owned_string, owned_string_from_handle};

    #[test]
    fn stage1_emit_program_json_v0_h_round_trips_source_handle_to_json_handle() {
        let source = "static box Main { main() { print(42) return 0 } }";
        let source_h = materialize_owned_string(source.to_string());

        let out_h = nyash_stage1_emit_program_json_v0_h(source_h);

        let out = owned_string_from_handle(out_h).expect("program json handle");
        assert!(out.contains("\"kind\":\"Program\""));
        assert!(out.contains("\"version\":0"));
    }

    #[test]
    fn stage1_emit_program_json_v0_h_returns_zero_for_invalid_source_handle() {
        assert_eq!(nyash_stage1_emit_program_json_v0_h(0), 0);
    }

    #[test]
    fn stage1_emit_mir_from_source_v0_h_round_trips_source_handle_to_json_handle() {
        let source = "static box Main { main() { print(42) return 0 } }";
        let source_h = materialize_owned_string(source.to_string());

        let out_h = nyash_stage1_emit_mir_from_source_v0_h(source_h);

        let out = owned_string_from_handle(out_h).expect("mir json handle");
        assert!(out.contains("\"functions\""));
        assert!(out.contains("\"user_box_decls\""));
    }

    #[test]
    fn stage1_emit_mir_from_source_v0_h_returns_zero_for_invalid_source_handle() {
        assert_eq!(nyash_stage1_emit_mir_from_source_v0_h(0), 0);
    }

    #[test]
    fn stage1_emit_mir_from_program_json_v0_h_round_trips_program_handle_to_json_handle() {
        let program_json = r#"{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":1}}]}"#;
        let program_json_h = materialize_owned_string(program_json.to_string());

        let out_h = nyash_stage1_emit_mir_from_program_json_v0_h(program_json_h);

        let out = owned_string_from_handle(out_h).expect("mir json handle");
        assert!(out.contains("\"functions\""));
        assert!(out.contains("\"user_box_decls\""));
    }

    #[test]
    fn stage1_emit_mir_from_program_json_v0_h_returns_zero_for_invalid_program_handle() {
        assert_eq!(nyash_stage1_emit_mir_from_program_json_v0_h(0), 0);
    }
}
