//! Ordered passive actual candidates from the same prefix-local state.
//! Source membership and typed-object domain are verified by the package.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedCallActualValueV1 {
    Integer(i64),
    Bool(bool),
    Scalar(BindingRefV1, SourceScalarKind),
    Home {
        binding: BindingRefV1,
        root: BindingRefV1,
        acquisition: OwnedExprSiteV1,
    },
    SelfRooted {
        binding: BindingRefV1,
        root: BindingRefV1,
    },
    Binding(BindingRefV1),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedCallActualCandidateV1 {
    pub(crate) ordinal: u32,
    pub(crate) site: SourceExprSiteV1,
    pub(crate) value: BorrowedCallActualValueV1,
}

fn observe_one_borrowed_call_actuals(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
) -> Option<Vec<BorrowedCallActualCandidateV1>> {
    if site.owner() != input.owner() {
        return None;
    }
    let (_, call) = input
        .function()
        .method_calls()
        .find(|(observed, _)| *observed == site.site())?;
    Some(
        call.arguments()
            .iter()
            .map(|argument| {
                let site = argument.site();
                let source = input.function().expression_source();
                let value = match source.literal(site) {
                    Some(ResolvedLiteralSourceV1::Integer(value)) => {
                        BorrowedCallActualValueV1::Integer(*value)
                    }
                    Some(ResolvedLiteralSourceV1::Bool(value)) => {
                        BorrowedCallActualValueV1::Bool(*value)
                    }
                    _ => {
                        // A signed immediate spelling retains its exact sealed unary
                        // site and operand. Overflow is not wrapped into a payload.
                        let negative = source.unary(site).filter(|row| {
                    row.operator() == crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Minus
                }).and_then(|row| match source.literal(row.operand()) {
                    Some(ResolvedLiteralSourceV1::Integer(value)) => value.checked_neg(),
                    _ => None,
                });
                        if let Some(value) = negative {
                            BorrowedCallActualValueV1::Integer(value)
                        } else if !prefix_known {
                            BorrowedCallActualValueV1::Unknown
                        } else {
                            match input.function().variable_ref(site) {
                                Some(ResolvedLexicalRefV1::Local(binding)) => match locals
                                    .observe(site)
                                {
                                    Some(OrdinaryObservation::TrivialLocal(actual, Some(kind)))
                                        if actual == binding =>
                                    {
                                        BorrowedCallActualValueV1::Scalar(binding, kind)
                                    }
                                    Some(OrdinaryObservation::Handle(root)) => {
                                        if let Some(acquisition) = locals.home_acquisition(root) {
                                            BorrowedCallActualValueV1::Home {
                                                binding,
                                                root,
                                                acquisition: acquisition.clone(),
                                            }
                                        } else if locals.is_self_rooted_handle(root) {
                                            BorrowedCallActualValueV1::SelfRooted { binding, root }
                                        } else {
                                            BorrowedCallActualValueV1::Binding(binding)
                                        }
                                    }
                                    _ => BorrowedCallActualValueV1::Binding(binding),
                                },
                                _ => BorrowedCallActualValueV1::Unknown,
                            }
                        }
                    }
                };
                BorrowedCallActualCandidateV1 {
                    ordinal: argument.ordinal(),
                    site: site.clone(),
                    value,
                }
            })
            .collect(),
    )
}

/// Observe only calls at exact direct argument sites, in source evaluation
/// order. The callback stages each original inner call before its enclosing
/// call; this does not turn a CallResult into an opaque payload candidate.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_borrowed_call_actuals(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
) -> Vec<(OwnedExprSiteV1, Vec<BorrowedCallActualCandidateV1>)> {
    if site.owner() != input.owner() {
        return Vec::new();
    }
    let mut pending = vec![(site.clone(), false)];
    let mut observed = Vec::new();
    while let Some((site, children_observed)) = pending.pop() {
        if children_observed {
            if let Some(actuals) =
                observe_one_borrowed_call_actuals(input, &site, locals, prefix_known)
            {
                observed.push((site, actuals));
            }
            continue;
        }
        let Some((_, call)) = input
            .function()
            .method_calls()
            .find(|(candidate, _)| *candidate == site.site())
        else {
            continue;
        };
        pending.push((site, true));
        // Reverse stack insertion preserves the declared argument order.
        // A binary/condition subtree is not a direct method-call site and
        // cannot acquire coverage merely by containing a call somewhere.
        for argument in call.arguments().iter().rev() {
            pending.push((
                OwnedExprSiteV1::new(input.owner(), argument.site().clone()),
                false,
            ));
        }
    }
    observed
}
