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
