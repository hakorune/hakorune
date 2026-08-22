use super::lane::{classify_extern_provider_lane, ExternProviderLane};
use crate::backend::mir_interpreter::{MirInterpreter, VMValue};
use crate::mir::{
    BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirModule, MirType, ValueId,
};

#[test]
fn classify_runtime_direct_lane_for_console_and_env() {
    for extern_name in [
        "print",
        "env.console.log",
        "env.console.warn",
        "env.console.error",
        "env.get",
        "env.now_ms",
        "env.set",
        "hako.analysis.decoded_utf8_byte_len_v0",
        "hako.analysis.strict_json_tree_v0.kind",
        "hako.analysis.strict_json_tree_v0.object_len",
        "hako.analysis.strict_json_tree_v0.object_key_at",
        "hako.analysis.strict_json_tree_v0.object_value_at",
        "hako.analysis.strict_json_tree_v0.array_len",
        "hako.analysis.strict_json_tree_v0.array_at",
        "hako.analysis.strict_json_tree_v0.string_value",
        "hako.analysis.strict_json_tree_v0.bool_value",
        "hako.analysis.strict_json_tree_v0.i64_value",
        "hako.analysis.strict_json_tree_v0.u64_fits_i64",
        "hako.analysis.strict_json_tree_v0.u64_as_i64",
    ] {
        assert_eq!(
            classify_extern_provider_lane(extern_name),
            Some(ExternProviderLane::RuntimeDirect),
            "expected runtime-direct lane for {}",
            extern_name
        );
    }
}

#[test]
fn classify_loader_cold_lane_for_provider_and_hostbridge() {
    for extern_name in [
        "env.mirbuilder.emit",
        "env.mirbuilder_emit",
        "env.codegen.emit_object",
        "env.codegen.emit_object_compat_harness",
        "env.codegen.compile_ll_text",
        "env.codegen.link_object",
        "env.box_introspect.kind",
        "hostbridge.extern_invoke",
    ] {
        assert_eq!(
            classify_extern_provider_lane(extern_name),
            Some(ExternProviderLane::LoaderCold),
            "expected loader-cold lane for {}",
            extern_name
        );
    }
}

#[test]
fn classify_non_provider_names_as_none() {
    for extern_name in ["nyash.string.concat_hh", "exit", "panic"] {
        assert_eq!(classify_extern_provider_lane(extern_name), None);
    }
}

fn module_with_decoded_utf8_byte_len(value: &str) -> MirModule {
    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.decoded_utf8_byte_len/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        entry,
    );
    let value_id = function.next_value_id();
    let result_id = function.next_value_id();
    let mut block = BasicBlock::new(entry);
    block.add_instruction(MirInstruction::Const {
        dst: value_id,
        value: ConstValue::String(value.to_string()),
    });
    block.add_instruction(MirInstruction::Call {
        dst: Some(result_id),
        func: ValueId::INVALID,
        callee: Some(Callee::Extern(
            "hako.analysis.decoded_utf8_byte_len_v0".to_string(),
        )),
        args: vec![value_id],
        effects: EffectMask::IO,
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result_id),
    });
    function.add_block(block);
    let mut module = MirModule::new("decoded-utf8-byte-len-runtime-direct".to_string());
    module.add_function(function);
    module
}

#[test]
fn runtime_direct_decoded_utf8_byte_len_counts_unicode_and_embedded_nul() {
    for (value, expected) in [
        ("abc", 3),
        ("猫", 3),
        ("😸", 4),
        ("猫😸", 7),
        ("e\u{0301}", 3),
        ("é", 2),
        ("\0", 1),
        ("a\0b", 3),
    ] {
        let actual = MirInterpreter::new()
            .execute_function_with_args(
                &module_with_decoded_utf8_byte_len(value),
                "Main.decoded_utf8_byte_len/0",
                &[],
            )
            .expect("decoded UTF-8 byte-length runtime direct call should succeed");
        assert_eq!(actual, VMValue::Integer(expected), "value={value:?}");
    }
}
