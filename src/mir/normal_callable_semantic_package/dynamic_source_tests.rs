use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use super::tests::final_source;

use super::{
    issue_normal_callable_semantic_package_v1,
    NormalCallableDynamicProjectionRefV1, NormalCallableSemanticPackageIssueV1,
};

fn issue(
    source: &str,
) -> Result<super::VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(93).unwrap();
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
}

#[test]
fn parser_scan_source_seals_one_dynamic_candidate_and_all_parameter_contracts() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact parser scan semantic package");

    assert_eq!(package.batch().declarations().len(), 4);
    assert_eq!(package.parameter_declaration_count(), 4);
    assert_eq!(package.parameter_count(), 15);
    let NormalCallableDynamicProjectionRefV1::Selected { completion } = package.dynamic_projection()
    else {
        panic!("exact Dynamic row must remain selected")
    };
    assert_eq!(completion.explicit_sites().len(), 2);
}


#[test]
fn dynamic_admission_retains_parameter_and_target_refusals() {
    use super::dynamic_admission::{
        DynamicCallAdmissionRejectV1, DynamicCallEnvelopeRejectV1, DynamicParameterContractRejectV1,
    };

    let scan = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let exact = "skip_while(src, pos: i64, end: i64, pred_chars)";
    let text_position = scan.replace(exact, "skip_while(src, pos: StringBox, end: i64, pred_chars)");
    assert!(matches!(
        issue(&text_position),
        Err(NormalCallableSemanticPackageIssueV1::DynamicRecipe {
            _error: DynamicParameterContractRejectV1::ParameterContractMismatch,
        })
    ));
    let non_dynamic_receiver = scan.replace(exact, "skip_while(src, pos: i64, end: i64, pred_chars: MapBox)");
    assert!(matches!(
        issue(&non_dynamic_receiver),
        Err(NormalCallableSemanticPackageIssueV1::DynamicRecipeEnvelope {
            _error: DynamicCallEnvelopeRejectV1::Calls(DynamicCallAdmissionRejectV1::MissingTarget),
        })
    ));
}
