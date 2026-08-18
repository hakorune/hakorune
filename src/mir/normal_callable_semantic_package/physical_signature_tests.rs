use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::compiler::pinned_text_backend_frame::issue_pinned_text_backend_frame_contract_v1;
use crate::mir::compiler::pinned_text_residence_backend_carrier::PinnedTextResidenceBackendCarrierIssueV1;
use crate::mir::compiler::target_capability::{
    PinnedTextCompileTargetCapabilityIssuerV1, PinnedTextCompileTargetProfileV1,
};
use crate::mir::normal_callable_semantic_package::physical_signature::PhysicalCallableLaneRoleV1;
use crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1;
use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
use crate::mir::pinned_text_residence_lifecycle::PreparedPinnedTextResidenceLifecycleV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};
use crate::runtime::text_formal_residence::residence_abi_layout_v1;

use super::issue_normal_callable_semantic_package_v1;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("physical signature source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn issue(source: &str) -> super::VerifiedNormalCallableSemanticPackageV1 {
    let mut resolver = FunctionSemanticResolverSessionV1::new(940).expect("resolver");
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
        .expect("physical signature package")
}

#[test]
fn static_and_instance_rows_keep_receiver_and_formal_axes_distinct() {
    let package = issue(
        r#"
static box StaticApi {
  find(source: StringBox, needle: StringBox, limit: i64) { return limit }
}
box InstanceApi {
  find(source: StringBox) { return 0 }
}
"#,
    );
    let rows = package.physical_signature().rows().collect::<Vec<_>>();
    assert_eq!(rows.len(), 2);

    let static_row = rows
        .iter()
        .copied()
        .find(|row| row.mode() == CallableParameterDeclarationModeV1::StaticBoxMethod)
        .expect("static row");
    assert_eq!(static_row.source_logical_arity(), 3);
    assert_eq!(static_row.receiver_lane_count(), 0);
    assert_eq!(static_row.physical_formal_lane_count(), 5);
    assert_eq!(static_row.physical_callable_lane_count(), 5);
    assert_eq!(
        static_row.lanes()[0].role(),
        PhysicalCallableLaneRoleV1::ExactTextSlot
    );
    assert_eq!(
        static_row.lanes()[1].role(),
        PhysicalCallableLaneRoleV1::ExactTextGeneration
    );
    assert_eq!(
        static_row.lanes()[2].role(),
        PhysicalCallableLaneRoleV1::ExactTextSlot
    );
    assert_eq!(
        static_row.lanes()[3].role(),
        PhysicalCallableLaneRoleV1::ExactTextGeneration
    );
    assert_eq!(
        static_row.lanes()[4].role(),
        PhysicalCallableLaneRoleV1::OrdinaryScalar
    );
    assert!(static_row.lanes().iter().all(|lane| lane.index()
        == static_row
            .lanes()
            .iter()
            .position(|candidate| candidate == lane)
            .unwrap() as u32));

    let instance_row = rows
        .iter()
        .copied()
        .find(|row| row.mode() == CallableParameterDeclarationModeV1::InstanceBoxMethod)
        .expect("instance row");
    assert_eq!(instance_row.source_logical_arity(), 1);
    assert_eq!(instance_row.receiver_lane_count(), 1);
    assert_eq!(instance_row.physical_formal_lane_count(), 2);
    assert_eq!(instance_row.physical_callable_lane_count(), 3);
    assert_eq!(
        instance_row.lanes()[0].role(),
        PhysicalCallableLaneRoleV1::InstanceReceiver
    );
    assert_eq!(
        instance_row.lanes()[1].role(),
        PhysicalCallableLaneRoleV1::ExactTextSlot
    );
    assert_eq!(
        instance_row.lanes()[2].role(),
        PhysicalCallableLaneRoleV1::ExactTextGeneration
    );
    assert_eq!(instance_row.lanes()[0].logical_ordinal(), None);
    assert_eq!(instance_row.lanes()[1].logical_ordinal(), Some(0));
}

#[test]
fn ordinary_formal_stays_one_lane_and_exact_text_pair_is_adjacent() {
    let package = issue("static box Api { run(value, text: StringBox) { return value } }");
    let row = package.physical_signature().rows().next().expect("one row");
    assert_eq!(row.source_logical_arity(), 2);
    assert_eq!(row.receiver_lane_count(), 0);
    assert_eq!(row.physical_formal_lane_count(), 3);
    assert_eq!(
        row.lanes()
            .iter()
            .map(|lane| lane.role())
            .collect::<Vec<_>>(),
        vec![
            PhysicalCallableLaneRoleV1::OrdinaryScalar,
            PhysicalCallableLaneRoleV1::ExactTextSlot,
            PhysicalCallableLaneRoleV1::ExactTextGeneration,
        ]
    );
    assert_eq!(row.lanes()[1].binding(), row.lanes()[2].binding());
}

#[test]
fn residence_backend_carrier_exports_source_issued_root_mapping() {
    let package = issue("static box Api { run(text: StringBox) { return 0 } }");
    let row = package.physical_signature().rows().next().expect("one row");
    let loan = ResolvedCallablePhysicalSignatureLoanV1::from_s6c_row(row);
    let target = PinnedTextCompileTargetCapabilityIssuerV1::issue(
        PinnedTextCompileTargetProfileV1::NyRtTextResidencePtr64As0V1,
    )
    .expect("target capability");
    let plans = PinnedTextAccessPlanTableV1::new(41);
    let frame = issue_pinned_text_backend_frame_contract_v1(
        &loan,
        &plans,
        residence_abi_layout_v1(),
        &target,
    )
    .expect("frame contract");
    let lifecycle = PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
        row.owner(),
        &plans,
        frame.borrow(),
        crate::mir::BasicBlockId::new(1),
        crate::mir::BasicBlockId::new(2),
    )
    .expect("lifecycle carrier");
    let carrier = crate::mir::compiler::pinned_text_residence_backend_carrier::
        PinnedTextResidenceBackendCarrierV1::issue(
            row,
            frame.borrow(),
            lifecycle.plan(),
            crate::mir::BasicBlockId::new(0),
            crate::mir::BasicBlockId::new(1),
            crate::mir::BasicBlockId::new(2),
            vec![crate::mir::BasicBlockId::new(3)].into_boxed_slice(),
            1,
        )
        .expect("carrier");
    let json = carrier.to_transport_json();
    assert_eq!(json["contract_id"], "hako.pinned_text_residence_carrier@1");
    assert_eq!(json["roots"][0]["frame_row"], 0);
    assert_eq!(json["roots"][0]["logical_ordinal"], 0);
    assert_eq!(json["roots"][0]["slot_lane"], 0);
    assert_eq!(json["roots"][0]["generation_lane"], 1);
    assert_eq!(
        json["finish_obligation"],
        "finish_every_explicit_normal_return"
    );
}

#[test]
fn residence_backend_carrier_rejects_finish_on_trap_before_transport() {
    let package = issue("static box Api { run(text: StringBox) { return 0 } }");
    let row = package.physical_signature().rows().next().expect("one row");
    let loan = ResolvedCallablePhysicalSignatureLoanV1::from_s6c_row(row);
    let target = PinnedTextCompileTargetCapabilityIssuerV1::issue(
        PinnedTextCompileTargetProfileV1::NyRtTextResidencePtr64As0V1,
    )
    .expect("target capability");
    let plans = PinnedTextAccessPlanTableV1::new(42);
    let frame = issue_pinned_text_backend_frame_contract_v1(
        &loan,
        &plans,
        residence_abi_layout_v1(),
        &target,
    )
    .expect("frame contract");
    let lifecycle = PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
        row.owner(),
        &plans,
        frame.borrow(),
        crate::mir::BasicBlockId::new(1),
        crate::mir::BasicBlockId::new(2),
    )
    .expect("lifecycle carrier");
    let result = crate::mir::compiler::pinned_text_residence_backend_carrier::
        PinnedTextResidenceBackendCarrierV1::issue(
            row,
            frame.borrow(),
            lifecycle.plan(),
            crate::mir::BasicBlockId::new(0),
            crate::mir::BasicBlockId::new(1),
            crate::mir::BasicBlockId::new(2),
            vec![crate::mir::BasicBlockId::new(2)].into_boxed_slice(),
            1,
        );
    assert_eq!(
        result,
        Err(PinnedTextResidenceBackendCarrierIssueV1::FinishOnTrap)
    );
}
