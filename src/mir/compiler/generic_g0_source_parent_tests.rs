use super::generic_g0_result_abi::{
    issue_generic_g0_result_abi_transport_v1, GenericG0ResultAbiRejectV1,
};
use super::generic_g0_source_parent::{
    with_generic_g0_source_parent_v1, GenericG0SourceParentRejectV1,
};
use super::generic_g0_storage_lane_source::{
    issue_generic_g0_storage_lane_source_projection_v1, GenericG0StorageLaneCarrierV1,
    GenericG0StorageLaneSourceRejectV1,
};
use super::generic_g0_top_level_declaration_header::issue_generic_g0_top_level_declaration_header_v1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;

#[test]
fn source_parent_lends_one_cohort_with_exact_entry_rows() {
    let (unit, selection) = generic_source_unit_and_selection_for_test();
    let input = unit.root_function_input().expect("root input");
    let owner = input.owner();
    let result = with_generic_g0_source_parent_v1(input, selection, |cohort| {
        assert_eq!(cohort.owner(), owner);
        assert_eq!(cohort.entries().len(), 2);
        assert_eq!(cohort.entries()[0].parameter_index(), 0);
        assert_eq!(cohort.entries()[1].parameter_index(), 1);
        assert_eq!(cohort.declaration_header().name(), "generic_g0");
        assert_eq!(cohort.declaration_header().parameters().len(), 2);
        assert_eq!(cohort.declaration_header().parameters()[0].ordinal(), 0);
        assert_eq!(cohort.declaration_header().parameters()[0].name(), "i");
        assert_eq!(
            cohort.declaration_header().parameters()[0].declared_type_name(),
            Some("i64")
        );
        assert_eq!(cohort.declaration_header().return_type_name(), Some("i64"));
        assert!(!cohort.declaration_header().is_static());
        assert!(cohort.declaration_header().metadata_is_empty());
        let storage_lane = cohort.storage_lane();
        assert_eq!(
            storage_lane.receiver_policy(),
            crate::mir::resolved_semantics::ReceiverPolicyV1::DeclaredInstance
        );
        assert_eq!(storage_lane.receiver_lane_count(), 1);
        assert_eq!(storage_lane.source_logical_arity(), 2);
        assert_eq!(storage_lane.physical_formal_lane_count(), 2);
        assert_eq!(storage_lane.physical_callable_lane_count(), 3);
        assert!(storage_lane.uses().is_empty());
        assert!(storage_lane.attrs().is_empty());
        let receiver = storage_lane.receiver().expect("instance receiver");
        assert_eq!(
            receiver.carrier(),
            GenericG0StorageLaneCarrierV1::ExistingCallableI64
        );
        assert_eq!(storage_lane.formals().len(), 2);
        assert!(storage_lane
            .formals()
            .iter()
            .all(|row| row.declared_type_name() == "i64"
                && row.abi()
                    == crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1::I64
                && row.carrier() == GenericG0StorageLaneCarrierV1::ExistingCallableI64));
        assert_eq!(cohort.body_shape().owner(), owner);
        assert_eq!(
            *cohort.body_shape().body_root(),
            input.function().root_profile().body_root()
        );
        assert!(!cohort.body_shape().effects().is_empty());
        assert_eq!(cohort.function_effect().owner(), owner);
        assert_eq!(cohort.function_effect().local_write_count(), 2);
        assert_eq!(cohort.function_effect().tail_return_count(), 1);
        assert_eq!(cohort.result_abi().owner(), owner);
        assert_eq!(cohort.result_abi().abi().source_type_name(), "i64");
        assert_eq!(cohort.completion().owner(), owner);
        assert_eq!(
            cohort.completion().target_function(),
            input.function().function_region()
        );
        assert!(cohort.completion().returns_value());
        assert_eq!(cohort.completion().explicit_sites().len(), 1);
        assert!(cohort.completion().cleanup().crossed_scopes().is_empty());
        cohort.product().core().owner()
    })
    .expect("source cohort");
    assert_eq!(result, owner);
}

#[test]
fn storage_lane_rejects_foreign_input_before_row_publication() {
    let (unit_a, selection_a) = generic_source_unit_and_selection_for_test();
    let (unit_b, _) = generic_source_unit_and_selection_for_test();
    let input_a = unit_a.root_function_input().expect("root A");
    let input_b = unit_b.root_function_input().expect("root B");
    let result = with_generic_g0_source_parent_v1(input_a, selection_a, |cohort| {
        issue_generic_g0_storage_lane_source_projection_v1(
            &input_b,
            cohort.product(),
            cohort.declaration_header(),
            cohort.body_shape(),
            cohort.entries(),
        )
    })
    .expect("parent cohort");
    assert!(matches!(
        result,
        Err(GenericG0StorageLaneSourceRejectV1::OwnerMismatch)
    ));
}

#[test]
fn static_source_keeps_absent_receiver_policy_as_source_fact() {
    let program = crate::parser::NyashParser::parse_from_string(
        r#"
static function generic_g0(i: i64, j: i64): i64 {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#,
    )
    .expect("fixture parses");
    let function = match program {
        crate::ast::ASTNode::Program { statements, .. } => statements
            .into_iter()
            .find(|node| matches!(node, crate::ast::ASTNode::FunctionDeclaration { .. }))
            .expect("function fixture"),
        _ => panic!("fixture is a program"),
    };
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function).expect("resolves");
    let input = unit.root_function_input().expect("root input");
    assert_eq!(
        input.function().root_profile().receiver_policy(),
        crate::mir::resolved_semantics::ReceiverPolicyV1::Absent
    );
    assert!(input
        .function()
        .declaration_binding(&crate::mir::resolved_semantics::SourceBindingSiteV1::Receiver)
        .is_none());
}

#[test]
fn source_parent_rejects_foreign_resolver_input_before_product() {
    let (unit_a, selection) = generic_source_unit_and_selection_for_test();
    let (unit_b, _) = generic_source_unit_and_selection_for_test();
    let _input_a = unit_a.root_function_input().expect("root A");
    let input_b = unit_b.root_function_input().expect("root B");
    let result = with_generic_g0_source_parent_v1(input_b, selection, |_| ());
    assert!(matches!(
        result,
        Err(GenericG0SourceParentRejectV1::SelectionOwnerMismatch)
            | Err(GenericG0SourceParentRejectV1::SelectionOriginMismatch)
            | Err(GenericG0SourceParentRejectV1::SelectionSiteMismatch)
    ));
}

#[test]
fn result_abi_transport_rejects_foreign_candidate_before_retention() {
    let (_, selection_a) = generic_source_unit_and_selection_for_test();
    let (unit_b, _) = generic_source_unit_and_selection_for_test();
    let input_b = unit_b.root_function_input().expect("root B");
    let header_b = issue_generic_g0_top_level_declaration_header_v1(&input_b).expect("header B");

    let result = issue_generic_g0_result_abi_transport_v1(&input_b, &selection_a, &header_b);

    assert!(matches!(
        result,
        Err(GenericG0ResultAbiRejectV1::CandidateOwnerMismatch)
    ));
}

#[test]
fn source_parent_rejects_bare_input_before_product() {
    let (unit, selection) = generic_source_unit_and_selection_for_test();
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        unit.syntax_root(),
        unit.forest(),
        unit.projection(),
    )
    .expect("bare input");

    let result = with_generic_g0_source_parent_v1(input, selection, |_| ());

    assert!(matches!(
        result,
        Err(GenericG0SourceParentRejectV1::BodyShapeMissing)
    ));
}
