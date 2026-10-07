//! Source predicates selecting the existing homes-aware completion walk.
//! These borrow source claims; they issue no new type authority or ledger row.
use super::{
    terminal_home, BindingRefV1, OrdinaryNewCandidate, OrdinaryNewCoSealIssueV1, OwnedExprSiteV1,
    VerifiedInstanceConstructorSemanticBatchV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

pub(super) fn has_map_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    has_map_formal: impl FnOnce() -> bool,
) -> bool {
    input.body_shape().is_some_and(|shape| {
        shape.expressions().iter().any(|row| {
            matches!(
                row,
                crate::mir::resolved_semantics::BodyExpressionShapeV1::MapLiteral { .. }
            )
        })
    }) || has_map_formal()
}

/// A me object-field initializer needs the verified walk's staged Alias
/// proof, which the plain seed / route-decision FieldGet cannot issue.
pub(super) fn has_me_object_field_read_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let owner = input.owner();
    let function = input.function();
    let mut found = false;
    if let Some((me, _)) = receiver_proof {
        if let Some(shape) = input.body_shape() {
            for initializer in function.expression_source().initializers() {
                let Some(site) = initializer.initializer_site() else {
                    continue;
                };
                let Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
                    object,
                    field,
                    ..
                }) = shape.expression_shape(site)
                else {
                    continue;
                };
                let receiver = match function.variable_ref(object) {
                    Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(receiver)) => {
                        receiver
                    }
                    _ => match shape.expression_shape(object) {
                        Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                            receiver:
                                crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding),
                            ..
                        }) => *binding,
                        _ => continue,
                    },
                };
                if receiver != me {
                    continue;
                }
                let owned = OwnedExprSiteV1::new(owner, site.clone());
                match terminal_home::local_read_field(
                    instance_constructors,
                    candidates,
                    coverage,
                    receiver_proof,
                    &owned,
                    me,
                    None,
                    field,
                )? {
                    Some((
                        _,
                        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Alias(
                            _,
                        ),
                    )) => {
                        found = true;
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }
    Ok(found)
}

/// An original opaque Static local continuation must retain its observation in
/// the canonical verified root flow. Only original initializer/fact membership
/// selects the walk; no new local classifier or completed ledger is consulted.
pub(super) fn has_static_source_local_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
) -> bool {
    let Ok(source) = source else {
        return false;
    };
    input
        .function()
        .expression_source()
        .initializers()
        .any(|initializer| {
            initializer.initializer_site().is_some_and(|site| {
                source.contains(&OwnedExprSiteV1::new(input.owner(), site.clone()))
            })
        })
}

/// Keep the existing plain root literal path separate from a selected Static
/// continuation's canonical root flow. This predicates source membership only.
pub(super) fn plain_main_integer_result_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    is_main: bool,
    no_sites: bool,
    no_map_or_owner_loan: bool,
    source: &Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
) -> bool {
    is_main
        && no_sites
        && no_map_or_owner_loan
        && !has_static_source_local_v1(input, source)
        && !input.function().declaration_sites().any(|site| {
            matches!(
                site,
                crate::mir::resolved_semantics::SourceBindingSiteV1::Receiver
            )
        })
        && input
            .forest()
            .ordered_capture_demands(input.owner())
            .is_empty()
}
