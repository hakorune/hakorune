use super::*;

use crate::mir::builder::calls::RawBrandCallAuthorityV1;
use crate::mir::{Callee, Effect, EffectMask};

fn installed_gc_preflight(builder: &MirBuilder, name: &str) -> PreparedRawFunctionPreflightV1 {
    PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        builder,
        name.to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::InstalledNonBrand { caller: None },
    )
}

#[test]
fn installed_gc_names_are_targeted_before_arguments() {
    for name in ["gc_collect", "gc_stats"] {
        let builder = MirBuilder::new();
        let prepared = installed_gc_preflight(&builder, name);
        assert!(matches!(
            prepared.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary {
                completion: PreparedRawOrdinaryFunctionCompletionV1::Targeted {
                    callee: Callee::Global(ref symbol),
                    arguments,
                }
            } if symbol == name && arguments.len() == 1
        ));
    }
}

#[test]
fn gc_targeting_does_not_capture_compatibility_or_math_routes() {
    let builder = MirBuilder::new();
    for name in ["gc_collect", "gc_stats"] {
        let prepared =
            PreparedRawFunctionPreflightV1::prepare(&builder, name.to_owned(), vec![integer(1)]);
        assert!(matches!(
            prepared.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary {
                completion: PreparedRawOrdinaryFunctionCompletionV1::Resolved { .. }
            }
        ));
    }
    for name in ["sin", "cos"] {
        let prepared = installed_gc_preflight(&builder, name);
        assert!(matches!(
            prepared.route,
            PreparedRawFunctionPreflightRouteV1::Math { .. }
        ));
    }
}

#[test]
fn installed_gc_target_is_consumed_once_with_existing_effect_parity() {
    for name in ["gc_collect", "gc_stats"] {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test(format!("gc_target/{name}"));
        let mut port = RecordingPortV1::default();
        let prepared = installed_gc_preflight(&builder, name);
        let result =
            lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
                .unwrap();
        assert_eq!(port.expression_count, 1);
        assert_eq!(port.events, vec!["child", "header"]);
        let calls = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
            .collect::<Vec<_>>();
        assert_eq!(calls.len(), 1);
        assert!(matches!(
            calls[0],
            MirInstruction::Call {
                dst: Some(dst),
                func,
                callee: Some(Callee::Global(symbol)),
                args,
                effects,
            } if *dst == result
                && *func == ValueId::INVALID
                && symbol == name
                && args.len() == 1
                && *effects == EffectMask::READ.add(Effect::ReadHeap)
        ));
    }
}
