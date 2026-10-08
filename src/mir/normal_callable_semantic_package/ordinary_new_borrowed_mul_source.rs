//! Exact ordered Mul source, issued once per original binary by the use owner.
//! No physical arithmetic, entry, result publication or finishing grant.
use super::guarded_actual::{CheckedIntegerGuardV1, CheckedIntegerOperandReachV1};
use super::*;
use crate::mir::dynamic_operator_contract::{
    issue_dynamic_operator_execution_envelope_v1, DynamicOperatorDomainV1, DynamicOperatorFamilyV1,
    DynamicOperatorValueClassV1, VerifiedDynamicOperatorExecutionEnvelopeV1,
};
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use crate::mir::resolved_semantics::{ResolvedBinaryOperatorV1, ResolvedLiteralSourceV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum BorrowedMulSideV1 {
    Left,
    Right,
}

impl BorrowedMulSideV1 {
    fn index(self) -> usize {
        match self {
            Self::Left => 0,
            Self::Right => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BorrowedMulOperandSourceV1 {
    CheckedView(CheckedIntegerOperandReachV1),
    IntegerLiteral { site: OwnedExprSiteV1, value: i64 },
    IntegerCall(IntegerCallOperandSourceV1),
}

impl BorrowedMulOperandSourceV1 {
    fn site(&self) -> &OwnedExprSiteV1 {
        match self {
            Self::CheckedView(reach) => reach.operand().2,
            Self::IntegerLiteral { site, .. } => site,
            Self::IntegerCall(call) => call.original().call_site(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct BorrowedMulSourceV1 {
    binary: OwnedExprSiteV1,
    operands: [BorrowedMulOperandSourceV1; 2],
    envelope: &'static VerifiedDynamicOperatorExecutionEnvelopeV1,
}

impl BorrowedMulSourceV1 {
    pub(in crate::mir) fn binary(&self) -> &OwnedExprSiteV1 {
        &self.binary
    }

    pub(super) fn view_at(&self, side: BorrowedMulSideV1) -> Option<&CheckedIntegerOperandReachV1> {
        match &self.operands[side.index()] {
            BorrowedMulOperandSourceV1::CheckedView(reach) => Some(reach),
            _ => None,
        }
    }

    pub(super) fn integer_call_sources(&self) -> impl Iterator<Item = &Rc<StaticIncomingSourceV1>> {
        self.operands.iter().filter_map(|operand| match operand {
            BorrowedMulOperandSourceV1::IntegerCall(call) => Some(call.original()),
            _ => None,
        })
    }

    /// Revalidate source and SAME per-formal Compare receipts. The canonical
    /// Static inventory separately corroborates each retained call's full loan.
    pub(super) fn corroborates(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        draft: &BorrowedFormalUsesDraftV1,
    ) -> bool {
        if self.binary.owner() != input.owner() || !std::ptr::eq(self.envelope, mul_envelope()) {
            return false;
        }
        let function = input.function();
        let Some(original) = function.expression_source().binary(self.binary.site()) else {
            return false;
        };
        if original.operator() != ResolvedBinaryOperatorV1::Multiply
            || self.operands[0].site().site() != original.lhs()
            || self.operands[1].site().site() != original.rhs()
        {
            return false;
        }
        let mut seen = 0u8;
        for row in &draft.uses {
            let BorrowedFormalUseDraftKindV1::MulOperand {
                binary,
                source,
                side,
            } = &row.kind
            else {
                continue;
            };
            if binary != &self.binary {
                continue;
            }
            let Some(reach) = self.view_at(*side) else {
                return false;
            };
            let (binding, formal, site) = reach.operand();
            let bit = 1 << side.index();
            if !std::ptr::eq(source.as_ref(), self)
                || seen & bit != 0
                || row.binding != binding
                || row.formal != formal
                || &row.site != site
            {
                return false;
            }
            seen |= bit;
        }
        let required = [BorrowedMulSideV1::Left, BorrowedMulSideV1::Right]
            .into_iter()
            .filter(|side| self.view_at(*side).is_some())
            .fold(0u8, |mask, side| mask | (1 << side.index()));
        if seen != required {
            return false;
        }
        self.operands.iter().all(|operand| {
            if operand.site().owner() != input.owner() {
                return false;
            }
            match operand {
                BorrowedMulOperandSourceV1::CheckedView(reach) => {
                    let (binding, formal, _) = reach.operand();
                    draft.origins.get(&binding) == Some(&formal)
                        && reach.corroborates(input)
                        && draft.uses.iter().any(|row| match &row.kind {
                            BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } =>
                                row.formal == formal && reach.guard().matches_compare(formal, binary, source),
                            _ => false,
                        })
                }
                BorrowedMulOperandSourceV1::IntegerLiteral { site, value } =>
                    integer_literal(input, site.site()) == Some(*value),
                BorrowedMulOperandSourceV1::IntegerCall(call) => {
                    let original = call.original();
                    original.is_zeroarg_i64_v1()
                        && !original.is_qualified()
                        && function.method_call(original.call_site().site()).is_some_and(|row|
                            row.owner() == input.owner()
                                && row.receiver() == crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::CurrentOwner
                                && row.arguments().is_empty()
                                && row.selector() == original.target().name())
                }
            }
        })
    }
}

fn mul_envelope() -> &'static VerifiedDynamicOperatorExecutionEnvelopeV1 {
    issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
        DynamicOperatorFamilyV1::Mul,
        DynamicOperatorValueClassV1::NormalInteger,
        DynamicOperatorValueClassV1::NormalInteger,
    ))
    .expect("sole NormalInteger Mul domain")
}

fn integer_literal(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Option<i64> {
    let source = input.function().expression_source();
    match source.literal(site) {
        Some(ResolvedLiteralSourceV1::Integer(value)) => Some(*value),
        _ => source.negative_integer_immediate(site),
    }
}

fn operand_source(
    input: ResolvedFunctionLoweringInputV1<'_>,
    origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    guards: &BTreeMap<BindingRefV1, Vec<Rc<CheckedIntegerGuardV1>>>,
    site: &SourceExprSiteV1,
    static_operands: Option<&StaticOperandContextV1<'_>>,
) -> Result<Option<BorrowedMulOperandSourceV1>, BorrowedFormalUseDraftErrorV1> {
    if let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(site) {
        let Some(formal) = origins.get(&binding) else {
            return Ok(None);
        };
        return Ok(guards
            .get(formal)
            .and_then(|rows| {
                rows.iter()
                    .find_map(|guard| guard.normal_path_for_operand(input, *formal, binding, site))
            })
            .map(BorrowedMulOperandSourceV1::CheckedView));
    }
    if let Some(value) = integer_literal(input, site) {
        return Ok(Some(BorrowedMulOperandSourceV1::IntegerLiteral {
            site: OwnedExprSiteV1::new(input.owner(), site.clone()),
            value,
        }));
    }
    Ok(
        call_operand::integer_call_operand_source_v1(input, site, static_operands)?
            .map(BorrowedMulOperandSourceV1::IntegerCall),
    )
}

/// Only borrowed-view Mul needs this product. Stable origin/rebind/capture
/// closure has already completed in the SAME use-owner prepass.
pub(super) fn collect_mul_sources(
    input: ResolvedFunctionLoweringInputV1<'_>,
    origins: &BTreeMap<BindingRefV1, BindingRefV1>,
    guards: &BTreeMap<BindingRefV1, Vec<Rc<CheckedIntegerGuardV1>>>,
    static_operands: Option<&StaticOperandContextV1<'_>>,
) -> Result<BTreeMap<SourceExprSiteV1, Rc<BorrowedMulSourceV1>>, BorrowedFormalUseDraftErrorV1> {
    let mut sources = BTreeMap::new();
    for binary in input
        .function()
        .expression_source()
        .binaries()
        .filter(|row| row.operator() == ResolvedBinaryOperatorV1::Multiply)
    {
        if ![binary.lhs(), binary.rhs()].into_iter().any(|site| {
            matches!(input.function().variable_ref(site), Some(ResolvedLexicalRefV1::Local(binding))
                if origins.contains_key(&binding))
        }) {
            continue;
        }
        let Some(left) = operand_source(input, origins, guards, binary.lhs(), static_operands)?
        else {
            continue;
        };
        let Some(right) = operand_source(input, origins, guards, binary.rhs(), static_operands)?
        else {
            continue;
        };
        let source = Rc::new(BorrowedMulSourceV1 {
            binary: OwnedExprSiteV1::new(input.owner(), binary.site().clone()),
            operands: [left, right],
            envelope: mul_envelope(),
        });
        if sources.insert(binary.site().clone(), source).is_some() {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
    }
    Ok(sources)
}

pub(super) fn mul_operand_kind(
    sources: &BTreeMap<SourceExprSiteV1, Rc<BorrowedMulSourceV1>>,
    binding: BindingRefV1,
    formal: BindingRefV1,
    site: &SourceExprSiteV1,
) -> Result<Option<BorrowedFormalUseDraftKindV1>, BorrowedFormalUseDraftErrorV1> {
    let mut matching = sources.values().flat_map(|source| {
        [BorrowedMulSideV1::Left, BorrowedMulSideV1::Right]
            .into_iter()
            .filter_map(move |side| {
                let reach = source.view_at(side)?;
                let (original_binding, original_formal, original_site) = reach.operand();
                (original_binding == binding
                    && original_formal == formal
                    && original_site.site() == site)
                    .then(|| BorrowedFormalUseDraftKindV1::MulOperand {
                        binary: source.binary.clone(),
                        source: Rc::clone(source),
                        side,
                    })
            })
    });
    let kind = matching.next();
    if matching.next().is_some() {
        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
    }
    Ok(kind)
}

impl BorrowedFormalUsesDraftV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn mul_operand_at(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<&BorrowedFormalUseDraftRowV1>, BorrowedFormalUseDraftErrorV1> {
        if site.owner() != input.owner() {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        let mut matching = self.uses.iter().filter(|row| &row.site == site);
        let Some(row) = matching.next() else {
            return Ok(None);
        };
        if matching.next().is_some() {
            return Err(BorrowedFormalUseDraftErrorV1::AmbiguousUse(site.clone()));
        }
        let BorrowedFormalUseDraftKindV1::MulOperand {
            binary,
            source,
            side,
        } = &row.kind
        else {
            return Ok(None);
        };
        let Some(reach) = source.view_at(*side) else {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        };
        let (binding, formal, original) = reach.operand();
        if binary != source.binary()
            || original != site
            || row.binding != binding
            || row.formal != formal
            || !source.corroborates(input, self)
        {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        Ok(Some(row))
    }

    /// ALL same-site retained child candidates must name the SAME original Rc.
    pub(in crate::mir::normal_callable_semantic_package) fn static_operand_call_source_at(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<&Rc<StaticIncomingSourceV1>>, BorrowedFormalUseDraftErrorV1> {
        if site.owner() != input.owner() {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        let mut retained = None;
        for row in &self.uses {
            let compare = match &row.kind {
                BorrowedFormalUseDraftKindV1::CompareOperand { source, .. } => {
                    source.integer_call_source()
                }
                _ => None,
            };
            let mul = match &row.kind {
                BorrowedFormalUseDraftKindV1::MulOperand { source, .. } => Some(source),
                _ => None,
            };
            for child in compare
                .into_iter()
                .chain(
                    mul.into_iter()
                        .flat_map(|source| source.integer_call_sources()),
                )
                .filter(|child| child.call_site() == site)
            {
                if let BorrowedFormalUseDraftKindV1::MulOperand { source, .. } = &row.kind {
                    if !source.corroborates(input, self) {
                        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
                    }
                }
                if retained.is_some_and(|old| !Rc::ptr_eq(old, child)) {
                    return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
                }
                retained = Some(child);
            }
        }
        Ok(retained)
    }
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_mul_source_tests.rs"]
mod tests;
