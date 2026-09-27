use super::*;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

use super::tests::*;

#[test]
fn published_view_rejects_missing_symbol_and_arity_definition_rows() {
    let key = static_key();
    let symbol = key.mir_symbol_projection();

    let mut missing = MirModule::new("view-definition-missing".to_owned());
    missing
        .add_cataloged_box_method(key.clone(), static_function(&key))
        .expect("publish relation");
    missing.functions.remove(&symbol);
    assert!(matches!(
        PublishedMirBackendView::try_new(&missing).unwrap_err(),
        PublishedMirBackendViewErrorV1::DefinitionMissing { .. }
    ));

    let mut wrong_symbol = MirModule::new("view-definition-symbol-drift".to_owned());
    wrong_symbol
        .add_cataloged_box_method(key.clone(), static_function(&key))
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
        .add_cataloged_box_method(key.clone(), static_function(&key))
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
    missing.add_function(static_function(&key));
    assert!(matches!(
        PublishedMirBackendView::try_new(&missing).unwrap_err(),
        PublishedMirBackendViewErrorV1::StaticCallDefinitionMissing { .. }
    ));

    let mut wrong_arity = MirModule::new("static-call-arity-drift".to_owned());
    wrong_arity
        .add_cataloged_box_method(key.clone(), static_function(&key))
        .expect("publish relation");
    if let MirInstruction::Call(call) = &mut wrong_arity
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry block")
        .instructions[0]
    {
        call.args.pop();
    } else {
        panic!("static helper must begin with a call");
    }
    assert!(matches!(
        PublishedMirBackendView::try_new(&wrong_arity).unwrap_err(),
        PublishedMirBackendViewErrorV1::StaticCallArityMismatch { .. }
    ));

    let mut wrong_result = MirModule::new("static-call-result-drift".to_owned());
    wrong_result
        .add_cataloged_box_method(key.clone(), static_function(&key))
        .expect("publish relation");
    wrong_result
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .signature
        .return_type = MirType::Void;
    // A cataloged call whose result leaves the checked Integer domain
    // has no selected-C consumer: the module classifies
    // `UnsupportedBeforeObject`, never `CanonicalTyped`, and the call
    // stays off the corridor row vocabulary.
    let view = PublishedMirBackendView::try_new(&wrong_result).expect("classified view");
    assert_eq!(
        view.route(),
        PublishedStaticMethodRouteV1::UnsupportedBeforeObject
    );
    assert!(view.static_method_calls().is_empty());
}

#[test]
fn non_integer_static_result_marks_module_unsupported_before_object() {
    // A String-returning cataloged static call is honest document
    // data, never a corridor row: the row vocabulary keeps its
    // Integer invariant while the module route records that this
    // canonical call family has no selected-C consumer.
    let key = static_key();
    let symbol = key.mir_symbol_projection();
    let mut module = MirModule::new("string-result-unsupported".to_owned());
    module
        .add_cataloged_box_method(key.clone(), static_function(&key))
        .expect("publish relation");
    module
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .signature
        .return_type = MirType::String;
    module.add_function(canonical_value_function());

    let view = PublishedMirBackendView::try_new(&module).expect("document view");
    assert_eq!(
        view.route(),
        PublishedStaticMethodRouteV1::UnsupportedBeforeObject
    );
    assert!(view.static_method_calls().is_empty());
}

#[test]
fn non_integer_static_result_keeps_sibling_integer_rows() {
    // The classification is per call, not per module: an Integer
    // sibling stays a checked corridor row even when a String call
    // takes the module route to `UnsupportedBeforeObject`.
    let key = static_key();
    let symbol = key.mir_symbol_projection();
    let other = CanonicalSameModuleCallableKeyV1::test_static_box_method("MathBox", "id", 1);
    let mut module = MirModule::new("mixed-result-domains".to_owned());
    module
        .add_cataloged_box_method(key.clone(), static_function(&key))
        .expect("publish string relation");
    module
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .signature
        .return_type = MirType::String;
    module
        .add_cataloged_box_method(other.clone(), {
            let mut function = MirFunction::new(
                FunctionSignature {
                    name: other.mir_symbol_projection(),
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
                    Some(ValueId::new(20)),
                    Callee::Global(
                        other
                            .canonical_global_target_v1()
                            .expect("static key projects to global target"),
                    ),
                    vec![ValueId::new(11)],
                    EffectMask::PURE,
                ));
            function
        })
        .expect("publish integer relation");

    let view = PublishedMirBackendView::try_new(&module).expect("document view");
    assert_eq!(
        view.route(),
        PublishedStaticMethodRouteV1::UnsupportedBeforeObject
    );
    assert_eq!(view.static_method_calls().len(), 1);
    assert_eq!(view.static_method_calls()[0].key(), &other);
}

#[test]
fn published_view_rejects_free_function_result_drift() {
    let key = free_key();
    let symbol = key.mir_symbol_projection();
    let mut module = MirModule::new("free-function-result-drift".to_owned());
    module
        .add_cataloged_box_method(key.clone(), free_function(&key))
        .expect("publish relation");
    module
        .functions
        .get_mut(&symbol)
        .expect("published definition")
        .signature
        .return_type = MirType::Void;
    let view = PublishedMirBackendView::try_new(&module).expect("classified view");
    assert_eq!(
        view.route(),
        PublishedStaticMethodRouteV1::UnsupportedBeforeObject
    );
    assert!(view.free_function_calls().is_empty());
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
    assert!(
        !std::path::PathBuf::from(format!("{}.published-static-method.o", exe_path.display()))
            .exists()
    );
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
            static_function(&static_key),
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

#[test]
fn intrinsic_map_substrate_stops_before_v1_object_without_compatibility() {
    for with_write in [false, true] {
        let mut module = intrinsic_array_module();
        let block = module
            .functions
            .get_mut("main")
            .unwrap()
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap();
        block.instructions[0] = if with_write {
            MirInstruction::MapLiteralEntryWrite {
                receiver: ValueId::new(3),
                key: ValueId::new(4),
                value: ValueId::new(5),
            }
        } else {
            MirInstruction::NewBox {
                dst: ValueId::new(0),
                target: crate::mir::ConstructionTarget::IntrinsicMap,
                args: vec![],
            }
        };
        let view = PublishedMirBackendView::try_new(&module).unwrap();
        assert_eq!(
            view.route(),
            PublishedStaticMethodRouteV1::UnsupportedBeforeObject
        );
        let output = tempfile::tempdir().unwrap();
        let object = output.path().join("map.o");
        let result =
            crate::host_providers::llvm_codegen::try_compile_published_static_method_object(
                &module,
                object.to_str().unwrap(),
            );
        assert!(result.is_err());
        assert_eq!(std::fs::read_dir(output.path()).unwrap().count(), 0);
    }
}
