use super::*;
use crate::mir::function::CanonicalCallableDefinitionPublicationErrorV1;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

fn static_key() -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::test_static_box_method("MathBox", "sum", 2)
}

fn static_function(key: &CanonicalSameModuleCallableKeyV1, func: ValueId) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: key.mir_symbol_projection(),
            params: vec![MirType::Integer; key.arity() as usize],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let target = key
        .canonical_global_target_v1()
        .expect("static key must project to global target");
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(10)),
            func,
            callee: Some(Callee::Global(target)),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        });
    function
}

fn instance_key() -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::test_instance_box_method("Counter", "step", 1)
}

fn instance_function(
    key: &CanonicalSameModuleCallableKeyV1,
    func: ValueId,
    receiver: ValueId,
) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: key.mir_symbol_projection(),
            params: vec![MirType::Integer; key.arity() as usize + 1],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(10)),
            func,
            callee: Some(Callee::SameModuleInstance {
                key: key.clone(),
                receiver,
            }),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });
    function
}

fn canonical_value_function() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "value-call/0".to_owned(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::call(
            Some(ValueId::new(2)),
            Callee::Value(ValueId::new(1)),
            vec![ValueId::new(1)],
            EffectMask::PURE,
        ));
    function
}

fn free_key() -> CanonicalSameModuleCallableKeyV1 {
    CanonicalSameModuleCallableKeyV1::free_function("helper", 1)
}

fn free_function(key: &CanonicalSameModuleCallableKeyV1, func: ValueId) -> MirFunction {
    free_function_with_args(key, func, vec![ValueId::new(1)])
}

fn free_function_with_args(
    key: &CanonicalSameModuleCallableKeyV1,
    func: ValueId,
    args: Vec<ValueId>,
) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: key.mir_symbol_projection(),
            params: vec![MirType::Integer; key.arity() as usize],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let target = key
        .canonical_global_target_v1()
        .expect("free key must project to global target");
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(10)),
            func,
            callee: Some(Callee::Global(target)),
            args,
            effects: EffectMask::PURE,
        });
    function
}

#[test]
fn published_static_method_is_typed_and_definition_backed() {
    let key = static_key();
    let mut module = MirModule::new("typed".to_owned());
    module
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");

    let view = PublishedMirBackendView::try_new(&module).expect("typed view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
    assert_eq!(view.static_method_calls().len(), 1);
    assert_eq!(view.static_method_calls()[0].key(), &key);
    assert!(view.definition(&key).is_some());
}

#[test]
fn c_frame_keeps_exact_site_and_one_way_symbol_projection() {
    let key = static_key();
    let mut module = MirModule::new("typed-c".to_owned());
    module
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");

    let view = PublishedMirBackendView::try_new(&module).expect("typed view");
    let frame = PublishedStaticMethodCFrameV1::from_view(&view).expect("C frame");
    assert_eq!(frame.len(), 1);
    let row = frame.as_slice()[0];
    assert_eq!(row.block_id, 0);
    assert_eq!(row.instruction_index, 0);
    assert_eq!(row.arity, 2);
    assert_eq!(row.kind, PublishedCallKindV1::StaticMethod as u32);
    assert!(!row.function_name.is_null());
    assert!(!row.target_symbol.is_null());
    assert!(!frame.as_ptr().is_null());
}

#[test]
fn published_free_function_is_typed_and_definition_backed() {
    let key = free_key();
    let mut module = MirModule::new("free-function".to_owned());
    module
        .add_cataloged_box_method(key.clone(), free_function(&key, ValueId::INVALID))
        .expect("publish relation");

    let view = PublishedMirBackendView::try_new(&module).expect("typed free-function view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
    assert_eq!(view.free_function_calls().len(), 1);
    assert_eq!(view.free_function_calls()[0].key(), &key);
    assert_eq!(view.free_function_calls()[0].args(), &[ValueId::new(1)]);
    assert!(view.definition(&key).is_some());

    let frame = PublishedStaticMethodCFrameV1::from_view(&view).expect("free-function C frame");
    let row = frame.as_slice()[0];
    assert_eq!(row.kind, PublishedCallKindV1::FreeFunction as u32);
    assert_eq!(row.arity, 1);
    assert!(!row.target_symbol.is_null());
}

#[test]
fn published_free_function_rejects_legacy_carrier_and_wrong_arity() {
    let key = free_key();
    let mut legacy = MirModule::new("free-function-legacy".to_owned());
    legacy
        .add_cataloged_box_method(key.clone(), free_function(&key, ValueId::new(9)))
        .expect("publish relation");
    assert!(matches!(
        PublishedMirBackendView::try_new(&legacy).unwrap_err(),
        PublishedMirBackendViewErrorV1::FreeFunctionCallUsesLegacyFunctionCarrier { .. }
    ));

    let mut wrong_arity = MirModule::new("free-function-arity".to_owned());
    let function = free_function_with_args(&key, ValueId::INVALID, Vec::new());
    wrong_arity
        .add_cataloged_box_method(key, function)
        .expect("publish relation");
    assert!(matches!(
        PublishedMirBackendView::try_new(&wrong_arity).unwrap_err(),
        PublishedMirBackendViewErrorV1::FreeFunctionCallArityMismatch { .. }
    ));
}

#[test]
fn published_free_function_rejects_missing_definition() {
    let key = free_key();
    let mut module = MirModule::new("free-function-missing-definition".to_owned());
    module.add_function(free_function(&key, ValueId::INVALID));

    assert!(matches!(
        PublishedMirBackendView::try_new(&module).unwrap_err(),
        PublishedMirBackendViewErrorV1::FreeFunctionCallDefinitionMissing { .. }
    ));
}

fn builtin_print_function(func: ValueId, dst: Option<ValueId>, args: Vec<ValueId>) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_owned(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::LegacyCallV0 {
            dst,
            func,
            callee: Some(Callee::Global(CanonicalGlobalTargetV1::builtin_print())),
            args,
            effects: EffectMask::IO,
        });
    function
}

#[test]
fn published_builtin_print_is_typed_and_has_no_definition_lookup() {
    let mut module = MirModule::new("builtin-print".to_owned());
    module.add_function(builtin_print_function(
        ValueId::INVALID,
        None,
        vec![ValueId::new(1)],
    ));

    let view = PublishedMirBackendView::try_new(&module).expect("typed builtin view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
    assert!(view.static_method_calls().is_empty());
    assert_eq!(view.builtin_print_calls().len(), 1);
    assert_eq!(view.builtin_print_calls()[0].args(), &[ValueId::new(1)]);

    let frame = PublishedStaticMethodCFrameV1::from_view(&view).expect("builtin C frame");
    let row = frame.as_slice()[0];
    assert_eq!(row.kind, PublishedCallKindV1::BuiltinPrint as u32);
    assert_eq!(row.arity, 1);
    assert!(!row.function_name.is_null());
    assert!(row.target_symbol.is_null());
}

#[test]
fn published_builtin_print_rejects_legacy_function_carrier() {
    let mut module = MirModule::new("builtin-print-legacy".to_owned());
    module.add_function(builtin_print_function(
        ValueId::new(9),
        None,
        vec![ValueId::new(1)],
    ));

    let error = PublishedMirBackendView::try_new(&module).unwrap_err();
    assert!(matches!(
        error,
        PublishedMirBackendViewErrorV1::BuiltinPrintUsesLegacyFunctionCarrier { .. }
    ));
}

#[test]
fn published_builtin_print_rejects_destination_and_wrong_arity() {
    let mut with_destination = MirModule::new("builtin-print-dst".to_owned());
    with_destination.add_function(builtin_print_function(
        ValueId::INVALID,
        Some(ValueId::new(2)),
        vec![ValueId::new(1)],
    ));
    assert!(matches!(
        PublishedMirBackendView::try_new(&with_destination).unwrap_err(),
        PublishedMirBackendViewErrorV1::BuiltinPrintHasDestination { .. }
    ));

    let mut wrong_arity = MirModule::new("builtin-print-arity".to_owned());
    wrong_arity.add_function(builtin_print_function(ValueId::INVALID, None, Vec::new()));
    assert!(matches!(
        PublishedMirBackendView::try_new(&wrong_arity).unwrap_err(),
        PublishedMirBackendViewErrorV1::BuiltinPrintArityMismatch { .. }
    ));
}

#[test]
fn published_array_element_writes_are_typed_rows_for_all_four_kinds() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "array-writes/0".to_owned(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block");
    for (site_id, kind, index) in [
        (1, crate::mir::ArrayElementWriteKind::LiteralAppend, None),
        (2, crate::mir::ArrayElementWriteKind::Push, None),
        (3, crate::mir::ArrayElementWriteKind::Set, Some(ValueId::new(2))),
        (4, crate::mir::ArrayElementWriteKind::Insert, Some(ValueId::new(3))),
    ] {
        block.add_instruction(
            crate::mir::array_element_write::instruction(
                crate::mir::ArrayWriteSiteId::new(site_id),
                None,
                kind,
                crate::mir::ArrayWriteProducerKind::MethodCall,
                ValueId::new(1),
                index,
                ValueId::new(4),
            )
            .expect("valid array write"),
        );
    }
    let mut module = MirModule::new("array-write-view".to_owned());
    module.add_function(function);

    let view = PublishedMirBackendView::try_new(&module).expect("typed array-write view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
    let writes = view.array_element_writes();
    assert_eq!(writes.len(), 4);
    assert_eq!(writes[0].kind(), crate::mir::ArrayElementWriteKind::LiteralAppend);
    assert_eq!(writes[1].kind(), crate::mir::ArrayElementWriteKind::Push);
    assert_eq!(writes[2].kind(), crate::mir::ArrayElementWriteKind::Set);
    assert_eq!(writes[2].index(), Some(ValueId::new(2)));
    assert_eq!(writes[3].kind(), crate::mir::ArrayElementWriteKind::Insert);
    assert_eq!(writes[3].receiver(), ValueId::new(1));
    assert_eq!(writes[3].value(), ValueId::new(4));

    let frame = PublishedStaticMethodCFrameV1::from_view(&view).expect("array-write C frame");
    assert_eq!(frame.len(), 4);
    assert_eq!(
        frame.as_slice()[0].kind,
        PublishedCallKindV1::ArrayLiteralAppend as u32
    );
    assert_eq!(frame.as_slice()[1].kind, PublishedCallKindV1::ArrayPush as u32);
    assert_eq!(frame.as_slice()[2].kind, PublishedCallKindV1::ArraySet as u32);
    assert_eq!(frame.as_slice()[3].kind, PublishedCallKindV1::ArrayInsert as u32);
    assert_eq!(frame.as_slice()[2].receiver, 1);
    assert_eq!(frame.as_slice()[2].index, 2);
    assert_eq!(frame.as_slice()[2].value, 4);
    assert_eq!(frame.as_slice()[2].flags, 2);
    assert!(frame.as_slice().iter().all(|row| row.dst == 0));
    assert!(frame.as_slice().iter().all(|row| row.flags & 1 == 0));
}

#[test]
fn published_array_element_writes_reject_invalid_kind_index_shape() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "array-write-invalid/0".to_owned(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::ArrayElementWrite {
            site_id: crate::mir::ArrayWriteSiteId::new(1),
            dst: None,
            kind: crate::mir::ArrayElementWriteKind::Set,
            producer: crate::mir::ArrayWriteProducerKind::IndexAssignment,
            receiver: ValueId::new(1),
            index: None,
            value: ValueId::new(2),
        });
    let mut module = MirModule::new("array-write-invalid".to_owned());
    module.add_function(function);
    assert!(matches!(
        PublishedMirBackendView::try_new(&module).unwrap_err(),
        PublishedMirBackendViewErrorV1::ArrayElementWriteShapeMismatch { .. }
    ));
}

#[test]
fn published_array_element_writes_compile_object_without_void_result_leak() {
    std::thread::Builder::new()
        .name("published-array-write-object".to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            compile_array_element_writes_object_on_large_stack();
        })
        .expect("spawn large-stack object compile test")
        .join()
        .expect("large-stack object compile test");
}

fn compile_array_element_writes_object_on_large_stack() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "array-write-object/0".to_owned(),
            params: Vec::new(),
            // The selected-C generic emitter currently materializes every
            // function as i64.  Keep the write operations themselves
            // destination-free (their logical result is Void), while using
            // local constants for the physical operands.
            return_type: MirType::Integer,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_owned(),
        args: Vec::new(),
    });
    for (dst, value) in [(2, 1), (3, 9), (4, 0)] {
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(dst),
            value: crate::mir::ConstValue::Integer(value),
        });
    }
    for (site_id, kind, index) in [
        (11, crate::mir::ArrayElementWriteKind::LiteralAppend, None),
        (12, crate::mir::ArrayElementWriteKind::Push, None),
        (13, crate::mir::ArrayElementWriteKind::Set, Some(ValueId::new(2))),
        (14, crate::mir::ArrayElementWriteKind::Insert, Some(ValueId::new(2))),
    ] {
        block.add_instruction(
            crate::mir::array_element_write::instruction(
                crate::mir::ArrayWriteSiteId::new(site_id),
                None,
                kind,
                crate::mir::ArrayWriteProducerKind::IndexAssignment,
                ValueId::new(1),
                index,
                ValueId::new(3),
            )
            .expect("valid array write"),
        );
    }
    block.add_instruction(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });

    let mut module = MirModule::new("array-write-object".to_owned());
    module.add_function(function);
    let out = std::env::temp_dir().join(format!(
        "hakorune-array-write-object-{}.o",
        std::process::id()
    ));
    let result = crate::host_providers::llvm_codegen::try_compile_published_static_method_object(
        &module,
        out.to_str().expect("temporary path is valid UTF-8"),
    );
    assert_eq!(
        result,
        Ok(true),
        "typed array object compile failed: {result:?}"
    );
    assert!(out.exists(), "typed array object was not emitted");
    assert!(std::fs::metadata(&out).expect("object metadata").len() > 0);
    let _ = std::fs::remove_file(out);

    // The same published rows must also own the executable path.  The
    // release runtime/FFI archive is a build prerequisite for this optional
    // local smoke; object generation above remains the always-on focused gate.
    let runtime_archive = std::path::Path::new("target/release/libnyash_kernel.a");
    let ffi_library = std::path::Path::new("target/release/libhako_llvmc_ffi.so");
    if runtime_archive.exists() && ffi_library.exists() {
        let exe = std::env::temp_dir().join(format!(
            "hakorune-array-write-object-{}",
            std::process::id()
        ));
        let exe_result = crate::host_providers::llvm_codegen::emit_published_static_method_exe(
            &module,
            exe.to_str().expect("temporary path is valid UTF-8"),
            Some("target/release"),
            None,
        );
        assert_eq!(exe_result, Ok(true), "typed array exe compile failed: {exe_result:?}");
        let status = std::process::Command::new(&exe)
            .status()
            .expect("run typed array executable");
        assert!(status.success(), "typed array executable failed: {status}");
        let _ = std::fs::remove_file(exe);
    }
}

#[test]
fn selected_static_method_keeps_other_families_on_compatibility_routes() {
    let key = static_key();
    let mut module = MirModule::new("mixed".to_owned());
    module
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");

    let mut legacy = MirFunction::new(
        FunctionSignature {
            name: "legacy/0".to_owned(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    legacy
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("legacy entry block")
        .add_instruction(MirInstruction::LegacyCallV0 {
            dst: None,
            func: ValueId::new(1),
            callee: None,
            args: Vec::new(),
            effects: EffectMask::PURE,
        });
    module.add_function(legacy);

    let view = PublishedMirBackendView::try_new(&module).expect("mixed typed view");
    assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
    assert_eq!(view.static_method_calls().len(), 1);
}

#[test]
fn selected_static_method_rejects_legacy_function_carrier() {
    let key = static_key();
    let mut module = MirModule::new("legacy-carrier".to_owned());
    module
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::new(9)))
        .expect("publish relation");

    let error = PublishedMirBackendView::try_new(&module).unwrap_err();
    assert!(matches!(
        error,
        PublishedMirBackendViewErrorV1::StaticCallUsesLegacyFunctionCarrier { .. }
    ));
}

#[test]
fn cataloged_publication_rejects_duplicate_and_preserves_first_definition() {
    let key = static_key();
    let mut module = MirModule::new("duplicate-cataloged".to_owned());
    module
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("first publication");
    let original_entry = module
        .get_function(&key.mir_symbol_projection())
        .expect("published definition")
        .entry_block;

    let duplicate = module
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::new(9)))
        .unwrap_err();
    assert!(matches!(
        duplicate,
        CanonicalCallableDefinitionPublicationErrorV1::DuplicateKey { .. }
    ));
    assert_eq!(module.canonical_callable_definition_count(), 1);
    assert_eq!(
        module
            .get_function(&key.mir_symbol_projection())
            .expect("first definition remains")
            .entry_block,
        original_entry
    );
    assert!(matches!(
        module.preflight_cataloged_box_method(&key, &key.mir_symbol_projection(), 2),
        Err(CanonicalCallableDefinitionPublicationErrorV1::DuplicateKey { .. })
    ));
}

#[test]
fn cataloged_publication_rejects_symbol_and_arity_drift() {
    let key = static_key();

    let mut wrong_symbol = static_function(&key, ValueId::INVALID);
    wrong_symbol.signature.name = "other/2".to_owned();
    let mut symbol_module = MirModule::new("cataloged-symbol-drift".to_owned());
    assert!(matches!(
        symbol_module.add_cataloged_box_method(key.clone(), wrong_symbol),
        Err(CanonicalCallableDefinitionPublicationErrorV1::KeySymbolMismatch { .. })
    ));

    let mut wrong_arity = static_function(&key, ValueId::INVALID);
    wrong_arity.signature.params.pop();
    let mut arity_module = MirModule::new("cataloged-arity-drift".to_owned());
    assert!(matches!(
        arity_module.add_cataloged_box_method(key, wrong_arity),
        Err(CanonicalCallableDefinitionPublicationErrorV1::KeyArityMismatch { .. })
    ));
}

#[test]
fn published_view_rejects_missing_symbol_and_arity_definition_rows() {
    let key = static_key();
    let symbol = key.mir_symbol_projection();

    let mut missing = MirModule::new("view-definition-missing".to_owned());
    missing
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");
    missing.functions.remove(&symbol);
    assert!(matches!(
        PublishedMirBackendView::try_new(&missing).unwrap_err(),
        PublishedMirBackendViewErrorV1::DefinitionMissing { .. }
    ));

    let mut wrong_symbol = MirModule::new("view-definition-symbol-drift".to_owned());
    wrong_symbol
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");
    let wrong_signature = FunctionSignature {
        name: "wrong/2".to_owned(),
        params: vec![MirType::Integer; 2],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    wrong_symbol.functions.insert(
        "wrong/2".to_owned(),
        MirFunction::new(wrong_signature, BasicBlockId::new(1)),
    );
    wrong_symbol
        .canonical_callable_definitions
        .insert(key.clone(), "wrong/2".to_owned());
    assert!(matches!(
        PublishedMirBackendView::try_new(&wrong_symbol).unwrap_err(),
        PublishedMirBackendViewErrorV1::DefinitionSymbolMismatch { .. }
    ));

    let mut wrong_arity = MirModule::new("view-definition-arity-drift".to_owned());
    wrong_arity
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");
    wrong_arity
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .signature
        .params
        .pop();
    assert!(matches!(
        PublishedMirBackendView::try_new(&wrong_arity).unwrap_err(),
        PublishedMirBackendViewErrorV1::DefinitionArityMismatch { .. }
    ));
}

#[test]
fn published_view_rejects_static_call_definition_arity_and_result_drift() {
    let key = static_key();
    let symbol = key.mir_symbol_projection();

    let mut missing = MirModule::new("static-call-definition-missing".to_owned());
    missing.add_function(static_function(&key, ValueId::INVALID));
    assert!(matches!(
        PublishedMirBackendView::try_new(&missing).unwrap_err(),
        PublishedMirBackendViewErrorV1::StaticCallDefinitionMissing { .. }
    ));

    let mut wrong_arity = MirModule::new("static-call-arity-drift".to_owned());
    wrong_arity
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");
    if let MirInstruction::LegacyCallV0 { args, .. } = &mut wrong_arity
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .instructions[0]
    {
        args.pop();
    } else {
        panic!("static helper must begin with a call");
    }
    assert!(matches!(
        PublishedMirBackendView::try_new(&wrong_arity).unwrap_err(),
        PublishedMirBackendViewErrorV1::StaticCallArityMismatch { .. }
    ));

    let mut wrong_result = MirModule::new("static-call-result-drift".to_owned());
    wrong_result
        .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
        .expect("publish relation");
    wrong_result
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .signature
        .return_type = MirType::Void;
    assert!(matches!(
        PublishedMirBackendView::try_new(&wrong_result).unwrap_err(),
        PublishedMirBackendViewErrorV1::StaticMethodRequiresIntegerReturn { .. }
    ));
}

#[test]
fn published_view_rejects_free_function_result_drift() {
    let key = free_key();
    let symbol = key.mir_symbol_projection();
    let mut module = MirModule::new("free-function-result-drift".to_owned());
    module
        .add_cataloged_box_method(key.clone(), free_function(&key, ValueId::INVALID))
        .expect("publish relation");
    module
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .signature
        .return_type = MirType::Void;
    assert!(matches!(
        PublishedMirBackendView::try_new(&module).unwrap_err(),
        PublishedMirBackendViewErrorV1::FreeFunctionRequiresIntegerReturn { .. }
    ));
}

#[test]
fn published_same_module_instance_is_unsupported_before_object() {
    let key = instance_key();
    let mut module = MirModule::new("instance-unsupported".to_owned());
    module
        .add_cataloged_box_method(
            key.clone(),
            instance_function(&key, ValueId::INVALID, ValueId::new(7)),
        )
        .expect("publish instance relation");
    module.add_function(canonical_value_function());

    let view = PublishedMirBackendView::try_new(&module).expect("physical admission result");
    assert_eq!(
        view.route(),
        PublishedStaticMethodRouteV1::UnsupportedBeforeObject
    );
    assert!(view.static_method_calls().is_empty());
    assert!(view.free_function_calls().is_empty());
    assert!(view.builtin_print_calls().is_empty());
    let exe_path = std::env::temp_dir().join(format!(
        "hakorune-canonical-value-stop-{}",
        std::process::id()
    ));
    let error = crate::host_providers::llvm_codegen::emit_published_static_method_exe(
        &module,
        exe_path.to_str().expect("temporary path is valid UTF-8"),
        None,
        None,
    )
    .expect_err("unsupported canonical call must stop before object emission");
    assert!(error.contains("UnsupportedBeforeObject"));
    assert!(!exe_path.exists());
    assert!(!std::path::PathBuf::from(format!(
        "{}.published-static-method.o",
        exe_path.display()
    ))
    .exists());
    let _ = std::fs::remove_file(exe_path);
}

#[test]
fn module_without_selected_static_method_is_explicit_compatibility() {
    let mut module = MirModule::new("compat".to_owned());
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "legacy/0".to_owned(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block");
    block.add_instruction(MirInstruction::LegacyCallV0 {
        dst: None,
        func: ValueId::new(1),
        callee: Some(Callee::Value(ValueId::new(1))),
        args: Vec::new(),
        effects: EffectMask::PURE,
    });
    module.add_function(function);

    let view = PublishedMirBackendView::try_new(&module).expect("compatibility view");
    assert_eq!(
        view.route(),
        PublishedStaticMethodRouteV1::ExplicitCompatibility
    );
}

#[test]
fn mixed_same_module_instance_takes_unsupported_precedence() {
    let static_key = static_key();
    let instance_key = instance_key();
    let mut module = MirModule::new("mixed-instance-unsupported".to_owned());
    module
        .add_cataloged_box_method(
            static_key.clone(),
            static_function(&static_key, ValueId::INVALID),
        )
        .expect("publish static relation");
    module
        .add_cataloged_box_method(
            instance_key.clone(),
            instance_function(&instance_key, ValueId::INVALID, ValueId::new(7)),
        )
        .expect("publish instance relation");

    let view = PublishedMirBackendView::try_new(&module).expect("physical admission result");
    assert_eq!(
        view.route(),
        PublishedStaticMethodRouteV1::UnsupportedBeforeObject
    );
    assert_eq!(view.static_method_calls().len(), 1);
}
