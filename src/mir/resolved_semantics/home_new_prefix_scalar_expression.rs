//! Pure scalar expression preflight. Source rows define operators and paths;
//! field declarations remain obligations until the whole batch is proved.
use super::*;
use crate::mir::resolved_semantics::{ResolvedBinaryOperatorV1, SourcePathSegmentV1, SourcePathV1};

pub(super) fn observe_scalar_expression<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    required: Option<SourceScalarKind>,
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        bool,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
) -> Result<Option<SourceScalarKind>, E> {
    let mut requests = Vec::new();
    let Some(kind) = preflight(input, site, locals, &mut requests) else {
        return Ok(None);
    };
    if required.is_some_and(|expected| expected != kind) {
        return Ok(None);
    }
    if !requests.is_empty() {
        let Some(results) = local_field_read(&requests, true)? else {
            return Ok(None);
        };
        if results.len() != requests.len()
            || results
                .iter()
                .any(|result| *result != LocalFieldReadResultV1::Scalar)
        {
            return Ok(None);
        }
    }
    Ok(Some(kind))
}

fn preflight(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    requests: &mut Vec<LocalFieldReadRequestV1>,
) -> Option<SourceScalarKind> {
    match locals.observe(site) {
        Some(OrdinaryObservation::Integer(_)) => return Some(SourceScalarKind::Integer),
        Some(OrdinaryObservation::Bool(_)) => return Some(SourceScalarKind::Bool),
        Some(OrdinaryObservation::TrivialLocal(_, Some(kind))) => return Some(kind),
        _ => {}
    }
    if let Some(request) = field_read::field_read_request(input, site, locals) {
        requests.push(request);
        return Some(SourceScalarKind::Integer);
    }
    let row = input.function().expression_source().binary(site)?;
    if !binary_sites_match(site, row) {
        return None;
    }
    use ResolvedBinaryOperatorV1 as Op;
    use SourceScalarKind as Kind;
    if matches!(row.operator(), Op::Equal | Op::NotEqual) {
        // A proven field-read alias is a borrowed handle: its only
        // admitted scalar equality is against the `null` literal. The
        // alias binding carries the sealed declared class; the read
        // result itself stays borrowed — no request, no release.
        let alias_null = |value: &SourceExprSiteV1, null: &SourceExprSiteV1| {
            let Some(ResolvedLexicalRefV1::Local(binding)) =
                input.function().variable_ref(value)
            else {
                return false;
            };
            matches!(
                locals.field_read_receiver(binding),
                Some(local_flow::FieldReadReceiverV1::Alias { .. })
            ) && matches!(locals.observe(null), Some(OrdinaryObservation::Null))
        };
        if alias_null(row.lhs(), row.rhs()) || alias_null(row.rhs(), row.lhs()) {
            return Some(Kind::Bool);
        }
    }
    let (operand, result) = match row.operator() {
        Op::Add | Op::Subtract => (Kind::Integer, Kind::Integer),
        Op::Equal | Op::NotEqual | Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual => {
            (Kind::Integer, Kind::Bool)
        }
        Op::And | Op::Or => (Kind::Bool, Kind::Bool),
        _ => return None,
    };
    let mark = requests.len();
    let lhs = preflight(input, row.lhs(), locals, requests)?;
    let rhs = preflight(input, row.rhs(), locals, requests)?;
    if matches!(
        row.operator(),
        Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual
    ) {
        // Order-compare operands admit the receivers the compare lane has:
        // a guarded formal via its sealed object view, or the `me` entry
        // receiver. Any other provenance declines the whole root.
        for request in &requests[mark..] {
            let me_receiver = input
                .body_shape()
                .and_then(|shape| shape.expression_shape(&request.receiver_site))
                .is_some_and(|shape| {
                    matches!(shape, crate::mir::resolved_semantics::BodyExpressionShapeV1::Me { .. })
                });
            if !request.formal && !me_receiver {
                return None;
            }
        }
    }
    (lhs == operand && rhs == operand).then_some(result)
}

/// Select only the new field-expression condition responsibility. Existing
/// conditions with no eligible field receiver keep their original admission.
pub(super) fn contains_field_request(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> bool {
    profile_scope(input, root, locals, false).unwrap_or(false)
}

/// Morphology-only scope: declaration, scalar class and liveness stay with
/// the subsequent proof. An excluded subtree never selects the new profile.
/// Order compares select the scalar lane only for a guarded formal field
/// read — owned receiver operands keep the existing compare lane, so a
/// `me.`/`new`-local order condition never silently changes lanes.
fn profile_scope(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    order_compare: bool,
) -> Option<bool> {
    if let Some(row) = input.function().expression_source().binary(site) {
        use ResolvedBinaryOperatorV1 as Op;
        if !matches!(
            row.operator(),
            Op::Add
                | Op::Subtract
                | Op::Equal
                | Op::NotEqual
                | Op::Less
                | Op::Greater
                | Op::LessEqual
                | Op::GreaterEqual
                | Op::And
                | Op::Or
        ) {
            return None;
        }
        let operand_order = matches!(
            row.operator(),
            Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual
        );
        let left = profile_scope(input, row.lhs(), locals, operand_order)?;
        let right = profile_scope(input, row.rhs(), locals, operand_order)?;
        return Some(left || right);
    }
    if matches!(
        input.function().expression_source().literal(site),
        Some(
            crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(_)
                | crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Bool(_)
        )
    ) || matches!(
        input.function().variable_ref(site),
        Some(ResolvedLexicalRefV1::Local(_))
    ) {
        return Some(false);
    }
    let shape = input.body_shape()?;
    let crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. } =
        shape.expression_shape(site)?
    else {
        return None;
    };
    let via_me = matches!(
        shape.expression_shape(object),
        Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me { .. })
    );
    let binding = match input.function().variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => match shape.expression_shape(object) {
            Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return None,
        },
    };
    if order_compare {
        // Only a guarded formal selects this lane for an order compare;
        // `me.` operands stay admissible inside a selected root, and any
        // other receiver provenance falls the whole root back to the
        // existing compare lane unchanged.
        if matches!(
            locals.field_read_receiver(binding),
            Some(local_flow::FieldReadReceiverV1::GuardedFormal)
        ) {
            return Some(true);
        }
        return via_me.then_some(false);
    }
    Some(locals.is_field_read_candidate(binding))
}

fn binary_sites_match(
    root: &SourceExprSiteV1,
    row: &crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
) -> bool {
    let path = SourcePathV1::from_node(root.node());
    row.site() == root
        && row.lhs() == &path.child(SourcePathSegmentV1::Lhs).expr()
        && row.rhs() == &path.child(SourcePathSegmentV1::Rhs).expr()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1;

    #[test]
    fn scalar_expression_binary_rejects_foreign_root_and_swapped_child_sites() {
        let path = SourcePathV1::root_body(0).child(SourcePathSegmentV1::Initializer(0));
        let root = path.expr();
        let lhs = path.child(SourcePathSegmentV1::Lhs).expr();
        let rhs = path.child(SourcePathSegmentV1::Rhs).expr();
        let good = ResolvedBinaryExpressionSourceV1::from_parts_for_test(
            root.clone(),
            ResolvedBinaryOperatorV1::Add,
            lhs.clone(),
            rhs.clone(),
        );
        assert!(binary_sites_match(&root, &good));
        assert!(!binary_sites_match(
            &SourcePathV1::root_body(1).expr(),
            &good
        ));
        let swapped = ResolvedBinaryExpressionSourceV1::from_parts_for_test(
            root.clone(),
            ResolvedBinaryOperatorV1::Add,
            rhs,
            lhs,
        );
        assert!(!binary_sites_match(&root, &swapped));
    }
}
