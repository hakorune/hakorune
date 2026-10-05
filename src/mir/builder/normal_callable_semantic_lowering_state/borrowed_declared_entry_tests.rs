//! Original declared class is lent by the same pre-entry source ledger.
use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::function::MirParamDecl;
use crate::mir::MirType;

fn declared_state(typed: bool, actual: &str) -> CallableSemanticLoweringState {
    let parameter = if typed { "p: Wrap" } else { "p" };
    let text = format!(
        "box Wrap {{ value: i64 birth(value) {{ me.value = value }} }}
         box Transport {{ birth() {{ }} probe({parameter}): i64 {{ return 0 }} }}
         static box Main {{ main() {{ local recv = new Transport()
             local obj = new Wrap(1) local out = recv.probe({actual}) return 0 }} }}"
    );
    let package = package_from_text(&text);
    state_from_package(package, 1)
}

fn declared_builder() -> crate::mir::MirBuilder {
    let mut function = carrier_function(&[ValueId(51), ValueId(72)]);
    function.signature.params[1] = MirType::Box("Wrap".into());
    function.metadata.declared_param_decls = vec![
        MirParamDecl {
            name: "me".into(),
            declared_type_name: Some("Transport".into()),
            implicit_receiver: true,
        },
        MirParamDecl {
            name: "p".into(),
            declared_type_name: Some("Wrap".into()),
            implicit_receiver: false,
        },
    ];
    carrier_builder(&function)
}

#[test]
fn original_declared_class_preflights_existing_value_and_tagged_column() {
    for actual in ["null", "obj"] {
        let state = declared_state(true, actual);
        let builder = declared_builder();
        let entry = PreparedCallableEntryValuesV1::instance_method(&builder, 1).unwrap();
        let (carriers, projected) = state
            .prepare_borrowed_entry_carriers(&entry, &builder)
            .expect("original class/header correspondence")
            .unwrap();
        assert_eq!(
            &*carriers,
            &[Carrier::ExistingCallableI64, Carrier::BorrowedTaggedValue]
        );
        assert_eq!(projected, vec![(1, ValueId(72))]);
        let function = builder.function_state.current_function.as_ref().unwrap();
        assert_eq!(function.signature.params[1], MirType::Box("Wrap".into()));
        assert_eq!(
            builder.function_state.type_ctx.value_types[&ValueId(72)],
            MirType::Box("Wrap".into())
        );
        assert_eq!(
            function.metadata.physical_param_carriers.as_deref(),
            Some(&[Carrier::ExistingCallableI64; 2][..])
        );
        assert!(!state.entry_installed);
        assert!(state.values.is_empty());
    }
}

#[test]
fn declared_header_and_value_drift_reject_before_any_entry_effect() {
    for change in 0..9 {
        let state = declared_state(true, "null");
        let mut builder = declared_builder();
        let entry = PreparedCallableEntryValuesV1::instance_method(&builder, 1).unwrap();
        let function = builder.function_state.current_function.as_mut().unwrap();
        match change {
            0 => function.metadata.declared_param_decls.clear(),
            1 => function.metadata.declared_param_decls[1].name = "foreign".into(),
            2 => {
                function.metadata.declared_param_decls[1].declared_type_name = Some("Other".into())
            }
            3 => function.metadata.declared_param_decls[1].implicit_receiver = true,
            4 => function.signature.params[1] = MirType::Integer,
            5 => {
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(ValueId(72), MirType::Integer);
            }
            6 => function.params[1] = ValueId(99),
            7 => {
                function.metadata.physical_param_carriers.as_mut().unwrap()[1] =
                    Carrier::CheckedMapStorage
            }
            8 => {
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .remove(&ValueId(72));
            }
            _ => unreachable!(),
        }
        let before = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .metadata
            .physical_param_carriers
            .clone();
        let error = state
            .prepare_borrowed_entry_carriers(&entry, &builder)
            .unwrap_err();
        assert!(error.contains("borrowed-entry/"), "{change}: {error}");
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .metadata
                .physical_param_carriers,
            before
        );
        assert!(!state.entry_installed);
        assert!(state.values.is_empty());
    }
}

#[test]
fn inferred_opaque_class_never_authorizes_a_declared_box_header() {
    let state = declared_state(false, "obj");
    let builder = declared_builder();
    let entry = PreparedCallableEntryValuesV1::instance_method(&builder, 1).unwrap();
    assert!(state
        .prepare_borrowed_entry_carriers(&entry, &builder)
        .unwrap_err()
        .contains("carrier-formal-drift"));
    assert!(!state.entry_installed);
}
