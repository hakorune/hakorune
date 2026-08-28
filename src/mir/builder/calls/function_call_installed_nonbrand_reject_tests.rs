use super::*;

use crate::mir::builder::calls::RawBrandCallAuthorityV1;

#[test]
fn installed_nonbrand_caller_none_rejects_before_arguments() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("source_backed_top_level/0".to_owned());
    let mut port = RecordingPortV1::default();
    let prepared = PreparedRawFunctionPreflightV1::prepare_with_brand_authority(
        &builder,
        "helper".to_owned(),
        vec![integer(1)],
        RawBrandCallAuthorityV1::InstalledNonBrand { caller: None },
    );

    let error =
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, prepared)
            .expect_err("source-backed caller without an exact relation must reject");
    assert!(error.contains("installed-source-relation-missing"));
    assert_eq!(port.expression_count, 0);
    assert!(port.events.is_empty());
    let calls = builder
        .function_state
        .current_function
        .as_ref()
        .expect("test function")
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .count();
    assert_eq!(calls, 0);
}
