use super::*;
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
        .add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func,
            callee: Some(Callee::Global(target)),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        });
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
        .add_instruction(MirInstruction::Call {
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
    let row = frame.row(0);
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
    let row = frame.row(0);
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
        .add_instruction(MirInstruction::Call {
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
    let row = frame.row(0);
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
        .add_instruction(MirInstruction::Call {
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
fn module_without_selected_static_method_is_explicit_compatibility() {
    let mut module = MirModule::new("compat".to_owned());
    module.add_function(MirFunction::new(
        FunctionSignature {
            name: "legacy/0".to_owned(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    ));

    let view = PublishedMirBackendView::try_new(&module).expect("compatibility view");
    assert_eq!(
        view.route(),
        PublishedStaticMethodRouteV1::ExplicitCompatibility
    );
    assert!(view.static_method_calls().is_empty());
}
