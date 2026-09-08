//! Owned four-table candidates. Actual C ingress/emission remains separately gated.
use super::super::super::c_transport_v2::{
    FrameHeader, PublishedStaticMethodCFrameV2, ORIGINAL_REQUIRED,
};
use super::*;
use std::ffi::CStr;

fn input(original: bool, float: bool) -> (MirModule, String) {
    let key = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "stash", 2);
    let name = key.mir_symbol_projection();
    let mut callee = function(&name, 2);
    callee.signature.params = vec![MirType::Unknown; 2];
    let params = callee.params.clone();
    write(&mut callee, params[0].as_u32());
    add(
        &mut callee,
        MirInstruction::call(
            None,
            Callee::Global(key.canonical_global_target_v1().unwrap()),
            params.clone(),
            EffectMask::PURE,
        ),
    );
    add(
        &mut callee,
        MirInstruction::Return {
            value: Some(params[usize::from(!original)]),
        },
    );
    let mut root = function("main", 0);
    constant(
        &mut root,
        7,
        if float {
            ConstValue::Float(-0.0)
        } else {
            ConstValue::Integer(7)
        },
    );
    constant(&mut root, 8, ConstValue::Bool(true));
    constant(&mut root, 9, ConstValue::Integer(30));
    for value in [7, 8] {
        add(
            &mut root,
            MirInstruction::call(
                None,
                Callee::Global(key.canonical_global_target_v1().unwrap()),
                vec![ValueId::new(value), ValueId::new(9)],
                EffectMask::PURE,
            ),
        );
    }
    let mut module = module(root);
    module.add_cataloged_box_method(key, callee).unwrap();
    // An unrelated original definition must not collide with the private target.
    module.add_function(function("__hako_map_expanded_v2_0", 0));
    (module, name)
}

#[test]
fn map_literal_owned_frame_binds_recursive_mixed_formals_and_survives_move() {
    for (original, float) in [(false, true), (true, false)] {
        let (frame, name, formal) = {
            let (module, name) = input(original, float);
            let formal = module.get_function(&name).unwrap().params[0];
            let view = PublishedMirBackendView::try_new(&module).unwrap();
            (
                PublishedStaticMethodCFrameV2::from_view(&view, []).unwrap(),
                name,
                formal,
            )
        }; // The wire backing does not borrow view/module strings.
        let before = frame.header();
        let frame = Box::new(frame);
        let header = frame.header();
        assert_eq!(header.values, before.values);
        assert_eq!(header.revision, 2);
        assert_eq!(
            header.byte_size as usize,
            std::mem::size_of::<FrameHeader>()
        );
        assert_eq!(
            (
                header.call_count,
                header.map_operation_count,
                header.value_count,
                header.expanded_function_count
            ),
            (3, 2, 3, 1)
        );
        unsafe {
            let expanded = &*header.expanded_functions;
            assert_eq!(
                CStr::from_ptr(expanded.function_name).to_str().unwrap(),
                name
            );
            assert_eq!(
                CStr::from_ptr(expanded.internal_target).to_str().unwrap(),
                "__hako_map_expanded_v2_1"
            );
            let values = std::slice::from_raw_parts(header.values, header.value_count as usize);
            let row = values
                .iter()
                .find(|r| CStr::from_ptr(r.function_name).to_str().unwrap() == name)
                .unwrap();
            assert_eq!(
                (row.value_id, row.action, row.source_ordinal),
                (formal.as_u32(), 6, 0)
            );
            assert_eq!(row.flags, if original { ORIGINAL_REQUIRED } else { 0 });
            let actual = values.iter().find(|r| r.value_id == 7).unwrap();
            assert_eq!(actual.payload, if float { 1 << 63 } else { 7 });
            assert_eq!(actual.value_kind, if float { 3 } else { 1 });
            assert_eq!(actual.flags, row.flags);
            for call in std::slice::from_raw_parts(header.calls, header.call_count as usize) {
                assert_eq!((call.kind, call.arity), (1, 2));
                // Calls retain logical canonical identity; C joins the same expanded table.
                assert_eq!(CStr::from_ptr(call.target_symbol).to_str().unwrap(), name);
            }
            let ops = std::slice::from_raw_parts(header.map_operations, 2);
            assert_eq!((ops[0].kind, ops[1].kind), (1, 2));
            assert!(ops.iter().all(|op| op.reserved == 0));
        }
    }
}

#[test]
fn map_literal_owned_frame_has_no_partial_float_or_foreign_observation_result() {
    let (module, _) = input(true, true);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    assert!(PublishedStaticMethodCFrameV2::from_view(&view, [])
        .unwrap_err()
        .contains("original-float-unavailable"));
    let (module, _) = input(false, true);
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    assert!(PublishedStaticMethodCFrameV2::from_view(
        &view,
        [(("main", 0, 99), NamedAllocationConsumer::Array)]
    )
    .is_err());
}

#[test]
fn map_literal_owned_frame_empty_tables_are_null_and_unrelated_defs_stay_unexpanded() {
    let module = module(function("main", 0));
    let view = PublishedMirBackendView::try_new(&module).unwrap();
    let frame = PublishedStaticMethodCFrameV2::from_view(&view, []).unwrap();
    let header = frame.header();
    assert_eq!(
        (
            header.call_count,
            header.map_operation_count,
            header.value_count,
            header.expanded_function_count
        ),
        (0, 0, 0, 0)
    );
    assert!(
        header.calls.is_null()
            && header.map_operations.is_null()
            && header.values.is_null()
            && header.expanded_functions.is_null()
    );
}
