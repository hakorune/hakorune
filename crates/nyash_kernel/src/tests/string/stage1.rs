use super::super::*;
use crate::c_string::cstring;
use nyash_rust::{box_trait::StringBox, runtime::host_handles as handles};
use std::sync::Arc;

#[test]
fn invoke_by_name_accepts_stage1_using_resolver_module_receiver() {
    let result_handle = dispatch_stage1_module(
        "lang.compiler.entry.using_resolver_box",
        "resolve_for_source",
        "static box Main { main() { return 0 } }",
    );
    assert!(result_handle > 0, "expected StringBox handle");

    let result_object = handles::get(result_handle as u64).expect("result handle");
    let result_string = result_object
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("StringBox result");
    assert_eq!(result_string.value, "");
}

#[test]
fn invoke_by_name_accepts_stage1_mir_builder_source_route_for_stage1_cli_env() {
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!("../../../../../lang/src/runner/stage1_cli_env.hako"),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {}",
        mir_json
    );
    assert!(mir_json.contains("\"functions\""));
    let mir_value: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");
    let user_box_decls = mir_value["user_box_decls"]
        .as_array()
        .expect("user_box_decls array");
    let box_names = user_box_decls
        .iter()
        .filter_map(|decl| decl["name"].as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        box_names.contains("Stage1InputContractBox"),
        "source authority route should expose Stage1InputContractBox user_box_decl"
    );
    assert!(
        box_names.contains("Stage1SourceMirAuthorityBox"),
        "source authority route should expose Stage1SourceMirAuthorityBox user_box_decl"
    );
    assert!(
        box_names.contains("Stage1ProgramJsonCompatBox"),
        "source authority route should preserve explicit same-file closure box decls"
    );
}

#[test]
fn invoke_by_name_export_accepts_stage1_mir_builder_source_route_for_stage1_cli_env() {
    with_env_vars(
        &[
            ("HAKO_MIR_BUILDER_INTERNAL", "1"),
            ("NYASH_VM_USE_FALLBACK", "1"),
        ],
        || {
            let recv_handle = handles::to_handle_arc(Arc::new(StringBox::new(
                "lang.mir.builder.MirBuilderBox".to_string(),
            ))) as i64;
            let method = cstring("emit_from_source_v0");
            let source_handle = handles::to_handle_arc(Arc::new(StringBox::new(
                include_str!("../../../../../lang/src/runner/stage1_cli_env.hako").to_string(),
            ))) as i64;

            let result_handle =
                nyash_plugin_invoke_by_name_i64(recv_handle, method.as_ptr(), 1, source_handle, 0);
            assert!(result_handle > 0, "expected MIR JSON StringBox handle");

            let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
            assert!(
                mir_json.starts_with('{'),
                "expected MIR JSON payload, got: {}",
                mir_json
            );
            assert!(mir_json.contains("\"functions\""));
            let mir_value: serde_json::Value =
                serde_json::from_str(&mir_json).expect("valid mir json");
            let user_box_decls = mir_value["user_box_decls"]
                .as_array()
                .expect("user_box_decls array");
            let box_names = user_box_decls
                .iter()
                .filter_map(|decl| decl["name"].as_str())
                .collect::<std::collections::BTreeSet<_>>();
            assert!(
                box_names.contains("Stage1InputContractBox"),
                "source authority route should expose Stage1InputContractBox user_box_decl"
            );
            assert!(
                box_names.contains("Stage1SourceMirAuthorityBox"),
                "source authority route should expose Stage1SourceMirAuthorityBox user_box_decl"
            );
            assert!(
                box_names.contains("Stage1ProgramJsonCompatBox"),
                "source authority route should preserve explicit same-file closure box decls"
            );
        },
    );
}

#[test]
fn invoke_by_name_accepts_stage1_mir_builder_source_route_for_hello_simple_llvm() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!("../../../../../apps/tests/hello_simple_llvm.hako"),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {}",
        mir_json
    );
    assert!(mir_json.contains("\"functions\""));
}

#[test]
fn invoke_by_name_stage1_mir_builder_source_route_accepts_decode_escapes_nested_loop_fixture() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.mir.builder.MirBuilderBox",
        "emit_from_source_v0",
        include_str!(
            "../../../../../apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako"
        ),
    );
    assert!(result_handle > 0, "expected MIR JSON StringBox handle");

    let mir_json = decode_string_like_handle(result_handle).expect("mir json string");
    assert!(
        mir_json.starts_with('{'),
        "expected MIR JSON payload, got: {mir_json}"
    );
    assert!(mir_json.contains("\"functions\""));
}

#[test]
fn invoke_by_name_stage1_using_resolver_route_is_stubbed_empty_in_kernel_dispatch() {
    ensure_test_ring0();
    let result_handle = dispatch_stage1_module(
        "lang.compiler.entry.using_resolver_box",
        "resolve_for_source",
        include_str!("../../../../../lang/src/runner/stage1_cli_env.hako"),
    );
    assert!(result_handle > 0, "expected stub StringBox handle");

    let prefix = decode_string_like_handle(result_handle).expect("prefix text");
    assert_eq!(
        prefix, "",
        "kernel direct module dispatch intentionally stubs resolve_for_source"
    );
}
