use super::*;

use crate::mir::builder::calls::RawBrandCallAuthorityV1;

fn installed_gc_preflight(builder: &MirBuilder, name: &str) -> PreparedRawFunctionPreflightV1 {
    PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        builder,
        name.to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::InstalledNonBrand { caller: None },
    )
}

#[test]
fn installed_gc_names_reject_before_arguments() {
    for name in ["gc_collect", "gc_stats"] {
        let builder = MirBuilder::new();
        let prepared = installed_gc_preflight(&builder, name);
        assert!(matches!(
            prepared.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary {
                completion: PreparedRawOrdinaryFunctionCompletionV1::Rejected { ref error }
            } if error.contains("gc-global-retired")
        ));
    }
}

#[test]
fn gc_targeting_does_not_capture_compatibility_or_math_routes() {
    let builder = MirBuilder::new();
    for name in ["gc_collect", "gc_stats"] {
        let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
            &builder,
            name.to_owned(),
            vec![integer(1)],
            RawBrandCallAuthorityV1::RawRootMainParkedCompatibility,
        );
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
fn installed_gc_rejection_does_not_descend_or_publish() {
    for name in ["gc_collect", "gc_stats"] {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test(format!("gc_target/{name}"));
        let mut port = RecordingPortV1::default();
        let prepared = installed_gc_preflight(&builder, name);
        let result =
            lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
                .expect_err("retired GC globals must fail before children");
        assert!(result.contains("gc-global-retired"));
        assert_eq!(port.expression_count, 0);
        assert!(port.events.is_empty());
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
        assert!(calls.is_empty());
    }
}
