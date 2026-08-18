use super::s6c_scan_with_init_tests::issue_facts;
use super::{
    issue_s6c_prephysical_ingress_v2, issue_s6c_scan_with_init_logical_output_v1,
    produce_s6c_scan_with_init_recipe_v2, S6CPrephysicalOperationRoleV2,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::CoreMethodSemanticLawV2;

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

#[test]
fn prephysical_ingress_seals_exact_source_and_transfer_census() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 951))
            .expect("exact S6C Recipe product"),
    )
    .expect("logical output rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("prephysical ingress");

    ingress
        .with_ingress(|view| {
            assert_eq!(view.operation_count(), 13);
            assert_eq!(view.operation_roles().count(), 13);
            assert_eq!(
                view.operation(S6CPrephysicalOperationRoleV2::BodyIndexRead)
                    .item(),
                super::LoopItemKeyV1::new(3)
            );
            assert_eq!(
                view.operation(S6CPrephysicalOperationRoleV2::StepWrite)
                    .item(),
                super::LoopItemKeyV1::new(14)
            );
            assert_eq!(view.input_bindings().len(), 3);
            assert!(matches!(
                view.operation_execution(S6CPrephysicalOperationRoleV2::LengthCall),
                super::LoopOperationExecutionClassV2::ExternallyBoundOutcome { .. }
            ));
            assert_eq!(
                view.after(),
                (
                    super::LoopNodeKeyV1::new(0),
                    super::LoopBindingKeyV1::new(0),
                    super::LoopValueClassV2::I64
                )
            );
            let completion = view.completion();
            assert_eq!(completion.explicit_exit_count(), 2);
            assert!(completion.cleanup_empty());
            Ok(())
        })
        .expect("ingress façade");

    ingress.with_text_eq_leaf(|text_eq| {
        assert_eq!(
            text_eq.operation().role(),
            S6CPrephysicalOperationRoleV2::TextEqual
        );
        assert_eq!(text_eq.operation().item(), super::LoopItemKeyV1::new(7));
        assert!(matches!(
            text_eq.row(),
            super::s6c_scan_with_init_joinir_output_rows::S6CLogicalItemV1::TextEq { .. }
        ));
        assert!(matches!(
            text_eq.if_row(),
            super::s6c_scan_with_init_joinir_output_rows::S6CLogicalItemV1::If { .. }
        ));
        assert_eq!(
            text_eq.binary().role(),
            crate::mir::callable_semantic_batch::S6CBinaryRoleV1::TextEqual
        );
    });

    ingress.with_completion(|completion| {
        assert_ne!(completion.loop_return_site(), completion.tail_site());
        assert_ne!(completion.loop_return_value(), completion.tail_value());
        let _ = completion.tail_operand();
    });
}

#[test]
fn scalar_scan_source_view_co_seals_law_and_derived_slice_relation() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 953))
            .expect("exact S6C Recipe product"),
    )
    .expect("logical output rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("prephysical ingress");

    ingress
        .with_scalar_scan_source(|source| {
            assert_ne!(source.subject_binding(), source.needle_binding());
            assert_ne!(source.subject_binding(), source.index_binding());
            assert_eq!(source.initial_index(), 0);
            assert_eq!(source.length_law(), CoreMethodSemanticLawV2::CodePointCount);
            assert_eq!(
                source.substring_law(),
                CoreMethodSemanticLawV2::CodePointHalfOpenClamped
            );
            assert_eq!(source.length().operation(), CoreMethodOp::StringLen);
            assert_eq!(source.length().arity(), 0);
            assert_eq!(
                source.substring().operation(),
                CoreMethodOp::StringSubstring
            );
            assert_eq!(source.substring().arity(), 2);
            assert_eq!(source.substring().arguments().len(), 2);
            assert_eq!(source.text_equal_item().raw(), 7);
            assert_eq!(source.text_equal_result().raw(), 10);
            assert_eq!(source.text_equal_if().raw(), 8);
            assert_eq!(source.step_add().raw(), 14);
            assert_eq!(source.logical_transfer().branches().len(), 1);
            assert_eq!(source.logical_transfer().summary_transfers().len(), 1);
            assert_eq!(source.completion().explicit_exit_count(), 2);
            assert!(source.completion().cleanup_empty());
            Ok(())
        })
        .expect("scalar source corridor");
}
