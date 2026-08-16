use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::physical_signature::PhysicalCallableLaneRoleV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

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
