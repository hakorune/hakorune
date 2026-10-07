//! Operation-use operand classifiers for the borrowed-formal use draft.
//!
//! Exact bodies moved verbatim from the `use` parent (USESIZE-T0): the
//! checked-compare `>` envelope, the ordered `+` envelope, their shared
//! Normal-Integer sibling proof, the call-argument pre-pass loan and the
//! source-order dominance helpers. No predicate, evaluation order, error
//! arm or signature changed; this child issues no ABI and installs no
//! carrier.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, OwnedExprSiteV1,
    ResolvedBinaryOperatorV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1,
};

use super::{BorrowedCompareSourceV1, BorrowedFormalUseDraftErrorV1, BorrowedFormalUseDraftKindV1};

/// `site` is a checked-compare operand of a `>` binary only when the binary
/// is a direct `if` condition and the sibling operand proves the
/// Normal-Integer class: an Integer literal, another borrowed view operand,
/// or `me.<field>` whose declaration the entry-receiver proof resolves to a
/// numeric-integer name. Anything else stays outside the draft profile.
pub(super) fn compare_operand_kind(
    input: ResolvedFunctionLoweringInputV1<'_>,
    origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    site: &SourceExprSiteV1,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    if !matches!(function.variable_ref(site), Some(ResolvedLexicalRefV1::Local(binding))
        if origins.contains_key(&binding))
    {
        return Ok(None);
    }
    let mut matching = function
        .expression_source()
        .binaries()
        .filter(|row| row.lhs() == site || row.rhs() == site);
    let Some(binary) = matching.next() else {
        return Ok(None);
    };
    if matching.next().is_some() || binary.operator() != ResolvedBinaryOperatorV1::Greater {
        return Ok(None);
    }
    if function
        .with_if_region_for_condition(binary.site(), |_| ())
        .is_err()
    {
        return Ok(None);
    }
    let other = if binary.lhs() == site {
        binary.rhs()
    } else {
        binary.lhs()
    };
    if !normal_integer_operand(input, origins, constructors, receiver, other)? {
        return Ok(None);
    }
    // The existing operation owner issues the checked-compare view envelope;
    // this draft is its first production consumer, not a new authority.
    use crate::mir::dynamic_operator_contract::{
        DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorValueClassV1,
    };
    crate::mir::dynamic_operator_contract::issue_dynamic_operator_execution_envelope_v1(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Greater,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
    )
    .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    Ok(Some(BorrowedFormalUseDraftKindV1::CompareOperand {
        binary: OwnedExprSiteV1::new(input.owner(), binary.site().clone()),
        source: BorrowedCompareSourceV1 {
            operator: binary.operator(),
            left: OwnedExprSiteV1::new(input.owner(), binary.lhs().clone()),
            right: OwnedExprSiteV1::new(input.owner(), binary.rhs().clone()),
            integer_literal: match function.expression_source().literal(other) {
                Some(ResolvedLiteralSourceV1::Integer(value)) => {
                    Some((OwnedExprSiteV1::new(input.owner(), other.clone()), *value))
                }
                _ => None,
            },
        },
    }))
}

/// Whether the sibling operand of a checked compare proves the logical
/// Normal-Integer class without inspecting storage spelling.
pub(super) fn normal_integer_operand(
    input: ResolvedFunctionLoweringInputV1<'_>,
    origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    site: &SourceExprSiteV1,
) -> Result<bool, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    if matches!(
        function.expression_source().literal(site),
        Some(ResolvedLiteralSourceV1::Integer(_))
    ) {
        return Ok(true);
    }
    if let Some(ResolvedLexicalRefV1::Local(binding)) = function.variable_ref(site) {
        return Ok(origins.contains_key(&binding));
    }
    let Some(shape) = input.body_shape() else {
        return Ok(false);
    };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(site)
    else {
        return Ok(false);
    };
    let me = match function.variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => match shape.expression_shape(object) {
            Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return Ok(false),
        },
    };
    crate::mir::normal_callable_semantic_package::ordinary_new_coseal::receiver_scalar_field(
        constructors,
        receiver,
        &OwnedExprSiteV1::new(input.owner(), site.clone()),
        me,
        field,
    )
    .map(|result| result.is_some())
    .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)
}

/// `site` is an ordered `+` operand of an `Add` binary only when the sibling
/// operand proves the Normal-Integer class and an admitted checked compare
/// of the same formal owns a region dominating this use. The physical
/// closure still proves block dominance; this draft admits only the exact
/// guarded shape.
pub(super) fn add_operand_kind(
    input: ResolvedFunctionLoweringInputV1<'_>,
    origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    formal: BindingRefV1,
    guards: &BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>>,
    site: &SourceExprSiteV1,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    let mut matching = function
        .expression_source()
        .binaries()
        .filter(|row| row.lhs() == site || row.rhs() == site);
    let Some(binary) = matching.next() else {
        return Ok(None);
    };
    if matching.next().is_some() || binary.operator() != ResolvedBinaryOperatorV1::Add {
        return Ok(None);
    }
    let other = if binary.lhs() == site {
        binary.rhs()
    } else {
        binary.lhs()
    };
    if !normal_integer_operand(input, origins, constructors, receiver, other)? {
        return Ok(None);
    }
    let dominated = guards
        .get(&formal)
        .into_iter()
        .flatten()
        .any(|guard| use_dominated_by_if(site.node(), guard));
    if !dominated {
        return Ok(None);
    }
    // The same operation owner issues the arithmetic view envelope; the
    // draft consumes it, it does not mint a new authority.
    use crate::mir::dynamic_operator_contract::{
        DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorValueClassV1,
    };
    crate::mir::dynamic_operator_contract::issue_dynamic_operator_execution_envelope_v1(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
    )
    .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    Ok(Some(BorrowedFormalUseDraftKindV1::AddOperand {
        binary: OwnedExprSiteV1::new(input.owner(), binary.site().clone()),
    }))
}

/// `site` is a `==` operand of an admitted null equality only when the
/// binary is a direct `if` condition and the sibling operand is an exact
/// `null` literal; the borrowed carrier may sit in either operand order.
/// `!=` and general tagged equality stay outside the profile, and the
/// admitted compare proves nothing about class, lifetime or the Integer
/// lane — the false successor supplies non-null only.
pub(super) fn null_compare_operand_kind(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    let mut matching = function
        .expression_source()
        .binaries()
        .filter(|row| row.lhs() == site || row.rhs() == site);
    let Some(binary) = matching.next() else {
        return Ok(None);
    };
    if matching.next().is_some() || binary.operator() != ResolvedBinaryOperatorV1::Equal {
        return Ok(None);
    }
    if function
        .with_if_region_for_condition(binary.site(), |_| ())
        .is_err()
    {
        return Ok(None);
    }
    let other = if binary.lhs() == site {
        binary.rhs()
    } else {
        binary.lhs()
    };
    // The sibling must be the exact `null` literal; an Integer literal,
    // Bool or another origin binding is not a null producer.
    if !matches!(
        function.expression_source().literal(other),
        Some(ResolvedLiteralSourceV1::Null)
    ) {
        return Ok(None);
    }
    // The same operation owner issues the bounded borrowed-value/null
    // equality envelope; the draft consumes it, it does not mint a new
    // authority. Either source operand order shares the one envelope.
    use crate::mir::dynamic_operator_contract::{
        DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorValueClassV1,
    };
    crate::mir::dynamic_operator_contract::issue_dynamic_operator_execution_envelope_v1(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Equal,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Null,
        ),
    )
    .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
    Ok(Some(BorrowedFormalUseDraftKindV1::NullCompareOperand {
        binary: OwnedExprSiteV1::new(input.owner(), binary.site().clone()),
    }))
}

/// Whether `site` serves any method-call argument position. The main use
/// loop re-validates ordinal identity for admitted arguments; this loan
/// only keeps argument sites out of the compare-guard pre-pass.
pub(super) fn is_call_argument(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Result<bool, BorrowedFormalUseDraftErrorV1> {
    for (_, call) in input.function().method_calls() {
        if call.owner() != input.owner() || call.arguments().len() != call.arity() as usize {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        if call
            .arguments()
            .iter()
            .any(|argument| argument.site() == site)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Source-order dominance evidence: `use_node` sits strictly after the
/// guarding `if` statement in the same enclosing sequence — the bounded
/// dominated region of this slice. A use inside the guard's own arms (the
/// early-return interior) or inside the condition itself stays outside
/// admission per the task-4 D0 boundary; block-level dominance remains the
/// physical gate's own proof.
pub(super) fn use_dominated_by_if(use_node: &SourceNodeSiteV1, guard: &SourceNodeSiteV1) -> bool {
    let guard = guard.segments();
    let path = use_node.segments();
    let Some(index) = guard
        .iter()
        .zip(path.iter())
        .position(|(left, right)| left != right)
    else {
        return false;
    };
    let (Some(guard), Some(use_)) = (guard.get(index), path.get(index)) else {
        return false;
    };
    std::mem::discriminant(guard) == std::mem::discriminant(use_)
        && sequence_ordinal(guard)
            .zip(sequence_ordinal(use_))
            .is_some_and(|(guard, use_)| guard < use_)
}

/// Sibling statement positions share one sequence vocabulary per container
/// kind; only equal-kind indexed steps are ordered.
fn sequence_ordinal(segment: &SourcePathSegmentV1) -> Option<u32> {
    match segment {
        SourcePathSegmentV1::Body(index)
        | SourcePathSegmentV1::ProgramBody(index)
        | SourcePathSegmentV1::ScopeBody(index)
        | SourcePathSegmentV1::TaskScopeBody(index)
        | SourcePathSegmentV1::FastMemBody(index)
        | SourcePathSegmentV1::IfThen(index)
        | SourcePathSegmentV1::IfElse(index)
        | SourcePathSegmentV1::LoopBody(index)
        | SourcePathSegmentV1::LambdaBody(index)
        | SourcePathSegmentV1::BlockExprPrelude(index)
        | SourcePathSegmentV1::TryBody(index)
        | SourcePathSegmentV1::CatchBody(index)
        | SourcePathSegmentV1::CleanupBody(index) => Some(*index),
        _ => None,
    }
}
