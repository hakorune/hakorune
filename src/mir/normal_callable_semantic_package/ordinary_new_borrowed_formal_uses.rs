//! Structural use closure inside the ordinary lexical-call owner.
//!
//! This draft is not ABI authority. Argument rows remain unresolved until
//! the same package issuer joins their exact selected target/formal and
//! verifies all incoming edges and live actuals. No carrier is installed here.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedAssignmentTargetV1,
    ResolvedLexicalRefV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
};

#[path = "ordinary_new_borrowed_formal_use_array_element.rs"]
mod array_element;

#[path = "ordinary_new_borrowed_formal_use_field_read.rs"]
mod field_read;

#[path = "ordinary_new_borrowed_formal_use_new_argument.rs"]
mod new_argument;

#[path = "ordinary_new_borrowed_formal_use_call_operand.rs"]
mod call_operand;
#[path = "ordinary_new_borrowed_formal_use_operands.rs"]
mod operands;
use call_operand::IntegerCallOperandSourceV1;
pub(super) use call_operand::StaticOperandContextV1;
pub(super) use operands::compare_operand_kind;

#[path = "ordinary_new_borrowed_guarded_actual.rs"]
mod guarded_actual;
pub(super) use guarded_actual::BorrowedGuardedActualV1;

#[path = "ordinary_new_borrowed_mul_source.rs"]
mod mul_source;
pub(in crate::mir) use mul_source::{BorrowedMulSideV1, BorrowedMulSourceV1};

use operands::{
    add_operand_kind, is_call_argument, normal_integer_operand, null_compare_operand_kind,
    use_dominated_by_if,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BorrowedFormalUseDraftErrorV1 {
    SourceIdentity,
    Rebound(BindingRefV1),
    AliasDeclaration(BindingRefV1),
    AnnotatedAlias(BindingRefV1),
    Captured(BindingRefV1),
    UnsupportedUse(OwnedExprSiteV1),
    AmbiguousUse(OwnedExprSiteV1),
}

/// Immutable original operands lent by the existing checked-compare classifier.
/// No ValueId, physical coordinate, or transport authority is issued here.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct BorrowedCompareSourceV1 {
    operator: crate::mir::resolved_semantics::ResolvedBinaryOperatorV1,
    left: OwnedExprSiteV1,
    right: OwnedExprSiteV1,
    integer_literal: Option<(OwnedExprSiteV1, i64)>,
    integer_call: Option<IntegerCallOperandSourceV1>,
    envelope:
        &'static crate::mir::dynamic_operator_contract::VerifiedDynamicOperatorExecutionEnvelopeV1,
}

impl BorrowedCompareSourceV1 {
    /// Original source-only call child; this is not a physical result value.
    pub(in crate::mir::normal_callable_semantic_package) fn integer_call_source(
        &self,
    ) -> Option<&Rc<crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1>>{
        self.integer_call
            .as_ref()
            .map(IntegerCallOperandSourceV1::original)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn comparison_parts(
        &self,
    ) -> (
        crate::mir::resolved_semantics::ResolvedBinaryOperatorV1,
        &OwnedExprSiteV1,
        &OwnedExprSiteV1,
        &'static crate::mir::dynamic_operator_contract::VerifiedDynamicOperatorExecutionEnvelopeV1,
    ) {
        (self.operator, &self.left, &self.right, self.envelope)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn integer_literal(
        &self,
    ) -> Option<(&OwnedExprSiteV1, i64)> {
        self.integer_literal
            .as_ref()
            .map(|(site, value)| (site, *value))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BorrowedFormalUseDraftKindV1 {
    Copy {
        destination: BindingRefV1,
    },
    /// Exact argument position only. This is not a selected forwarding edge.
    UnresolvedArgument {
        call: OwnedExprSiteV1,
        ordinal: u32,
    },
    /// A checked-compare operand use admitted under the existing operation
    /// owner's `Greater(NormalInteger, NormalInteger)` view envelope. The
    /// binary site pins the sole admitted operand use; the view stays lent
    /// to this binding/ValueId only.
    CompareOperand {
        binary: OwnedExprSiteV1,
        source: Rc<BorrowedCompareSourceV1>,
    },
    /// An ordered `+` operand use dominated by an admitted checked compare
    /// of the same formal, under the operation owner's
    /// `Add(NormalInteger, NormalInteger)` envelope. The lent view carries
    /// no borrowed identity into the fresh-integer result.
    AddOperand {
        binary: OwnedExprSiteV1,
    },
    /// Exact ordered Mul product, including each operand's original guard.
    /// This does not use the legacy Add count-only admission.
    MulOperand {
        binary: OwnedExprSiteV1,
        source: Rc<BorrowedMulSourceV1>,
        side: BorrowedMulSideV1,
    },
    /// Exact value Return after the SAME checked Compare in one sequence.
    /// This source use neither changes the tagged carrier nor activates entry.
    IntegerReturn {
        exit: crate::mir::resolved_semantics::SourceStmtSiteV1,
        guard: Rc<guarded_actual::CheckedIntegerGuardV1>,
    },
    /// The value argument of a `.set(index, value)` element write on a
    /// proven `me.<ArrayBox>` receiver, dominated by an admitted checked
    /// compare of the same formal. The call site pins the sole admitted
    /// operand use; the receiver and index never carry the lent view.
    ArrayElementValue {
        call: OwnedExprSiteV1,
    },
    /// One argument position of a `new <Child>(...)` construction dominated
    /// by an admitted checked compare of the same formal. The (new site,
    /// ordinal) pair pins the sole admitted argument use; the transport
    /// spells the tagged actual only for this exact ordinal.
    NewArgument {
        site: OwnedExprSiteV1,
        ordinal: u32,
    },
    /// A `==` operand of an admitted null equality: the binary is a direct
    /// `if` condition and the sibling is the exact `null` literal, under
    /// the operation owner's `Equal(Dynamic, Null)` envelope. The lent
    /// view is read-only and non-suspending; the false successor supplies
    /// non-null only — never Integer, class or liveness.
    NullCompareOperand {
        binary: OwnedExprSiteV1,
    },
    /// The receiver of a `formal.field` read dominated by an admitted null
    /// compare of the same formal — the surviving successor is the only
    /// path where the borrowed object is live. The FieldAccess site pins
    /// the sole admitted read; class and field declaration belong to the
    /// issuer's sealed object view, never to this draft.
    FieldReadOperand {
        site: OwnedExprSiteV1,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct BorrowedFormalUseDraftRowV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) binding: BindingRefV1,
    pub(super) formal: BindingRefV1,
    pub(super) kind: BorrowedFormalUseDraftKindV1,
}

#[derive(Debug)]
pub(super) struct BorrowedFormalUsesDraftV1 {
    /// Includes ignored formals and each direct Copy's actual binding.
    pub(super) origins: BTreeMap<BindingRefV1, BindingRefV1>,
    pub(super) uses: Box<[BorrowedFormalUseDraftRowV1]>,
}

impl BorrowedFormalUsesDraftV1 {
    /// Read the original source row and replay its SAME guard/exit/value
    /// correspondence. This is source proof, never entry or ABI permission.
    pub(super) fn integer_return_at(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        value: &OwnedExprSiteV1,
    ) -> Result<Option<&BorrowedFormalUseDraftRowV1>, BorrowedFormalUseDraftErrorV1> {
        if value.owner() != input.owner() {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        let mut rows = self.uses.iter().filter(|row| &row.site == value);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        if rows.next().is_some() {
            return Err(BorrowedFormalUseDraftErrorV1::AmbiguousUse(value.clone()));
        }
        let BorrowedFormalUseDraftKindV1::IntegerReturn { guard, .. } = &row.kind else {
            return Ok(None);
        };
        if self.origins.get(&row.binding) != Some(&row.formal)
            || guard
                .return_kind(input, row.formal, row.binding, value.site())
                .as_ref()
                != Some(&row.kind)
        {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        Ok(Some(row))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BorrowedForwardJoinDraftErrorV1 {
    MissingCall(OwnedExprSiteV1),
    CallIdentity(OwnedExprSiteV1),
    MissingCallee(OwnedExprSiteV1),
    FormalIdentity(OwnedExprSiteV1),
}

#[derive(Debug)]
pub(super) struct BorrowedForwardUseDraftRowV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) binding: BindingRefV1,
    pub(super) source_formal: BindingRefV1,
    pub(super) call: OwnedExprSiteV1,
    pub(super) target: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    pub(super) ordinal: u32,
    pub(super) callee_formal: BindingRefV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BorrowedIncomingDraftErrorV1 {
    SourceIdentity,
    BatchLoan,
    UnresolvedCaller(OwnedExprSiteV1),
    OutsideOrdinaryScope(OwnedExprSiteV1),
    CallIdentity(OwnedExprSiteV1),
    NoIncoming(FunctionOwnerIdV1),
}

#[derive(Debug, Clone)]
pub(super) struct BorrowedIncomingCallDraftV1 {
    pub(super) source: BorrowedIncomingSourceV1,
    pub(super) call: OwnedExprSiteV1,
    pub(super) callee: FunctionOwnerIdV1,
    pub(super) arguments: Box<[(u32, SourceExprSiteV1, BindingRefV1)]>,
}

#[path = "ordinary_new_borrowed_call_source.rs"]
mod call_source;
pub(super) use call_source::{borrow_call_sources_v1, BorrowedCallSourceLoanV1};
#[path = "ordinary_new_borrowed_incoming_source.rs"]
mod incoming_source;
pub(super) use incoming_source::BorrowedIncomingSourceV1;

#[path = "ordinary_new_borrowed_formal_incoming.rs"]
mod incoming;
#[cfg(test)]
pub(super) use incoming::inventory_borrowed_incoming_calls_v1;
pub(super) use incoming::{
    inventory_borrowed_incoming_with_stored_dispatch_v1, BorrowedIncomingInventoryV1,
    StaticIncomingContextV1,
};

/// Existing direct-test adapter uses the same whole-batch scan.
#[cfg(test)]
pub(super) fn draft_borrowed_incoming_calls_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::VerifiedSelectedCallableBatchMapV1,
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &super::LexicalInstanceCallSourceTargetV1>,
    ordinary_callers: &std::collections::BTreeSet<FunctionOwnerIdV1>,
) -> Result<Box<[BorrowedIncomingCallDraftV1]>, BorrowedIncomingDraftErrorV1> {
    inventory_borrowed_incoming_calls_v1(
        batch,
        selected,
        drafts,
        contracts,
        calls,
        ordinary_callers,
        None,
    )?
    .project(&drafts.keys().copied().collect())
}

/// Join every unresolved argument to the existing exact lexical disposition
/// and the callee's source-owned opaque binding. The complete finite draft
/// set permits cycles without assuming a recursive edge succeeds. This
/// result still does not prove incoming coverage, availability or carrier ABI.
pub(super) fn join_borrowed_forward_uses_v1(
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, BorrowedCallSourceLoanV1<'_>>,
) -> Result<Box<[BorrowedForwardUseDraftRowV1]>, BorrowedForwardJoinDraftErrorV1> {
    let mut rows = Vec::new();
    for (owner, draft) in drafts {
        for use_row in &draft.uses {
            let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } = &use_row.kind
            else {
                continue;
            };
            let error_site = use_row.site.clone();
            let exact_call = calls
                .get(call)
                .ok_or_else(|| BorrowedForwardJoinDraftErrorV1::MissingCall(error_site.clone()))?;
            if use_row.site.owner() != *owner
                || call.owner() != *owner
                || use_row.binding.owner() != *owner
                || use_row.formal.owner() != *owner
                || draft.origins.get(&use_row.binding) != Some(&use_row.formal)
                || exact_call.call_site() != call
                || exact_call.target().namespace() != exact_call.namespace()
                || exact_call.argument_sites().get(*ordinal as usize) != Some(use_row.site.site())
            {
                return Err(BorrowedForwardJoinDraftErrorV1::CallIdentity(error_site));
            }
            let mut matching = contracts
                .iter()
                .filter(|contract| contract.owner == exact_call.callee_owner());
            let contract = matching.next().ok_or_else(|| {
                BorrowedForwardJoinDraftErrorV1::MissingCallee(error_site.clone())
            })?;
            let callee_draft = drafts.get(&contract.owner).ok_or_else(|| {
                BorrowedForwardJoinDraftErrorV1::MissingCallee(error_site.clone())
            })?;
            if matching.next().is_some()
                || contract.batch_slot != exact_call.target_batch_slot()
                || contract.mode != exact_call.declaration_mode()
                || contract.parameters.len() != exact_call.argument_sites().len()
                || contract.parameters.len() != exact_call.target().arity() as usize
            {
                return Err(BorrowedForwardJoinDraftErrorV1::FormalIdentity(error_site));
            }
            let formal = contract.parameters.get(*ordinal as usize).ok_or_else(|| {
                BorrowedForwardJoinDraftErrorV1::FormalIdentity(error_site.clone())
            })?;
            if formal.ordinal != *ordinal
                || formal.binding.owner() != contract.owner
                || !formal.kind.is_ordinary_borrowed_handle()
                || callee_draft.origins.get(&formal.binding) != Some(&formal.binding)
            {
                return Err(BorrowedForwardJoinDraftErrorV1::FormalIdentity(error_site));
            }
            rows.push(BorrowedForwardUseDraftRowV1 {
                site: use_row.site.clone(),
                binding: use_row.binding,
                source_formal: use_row.formal,
                call: call.clone(),
                target: exact_call.target().clone(),
                ordinal: *ordinal,
                callee_formal: formal.binding,
            });
        }
    }
    Ok(rows.into_boxed_slice())
}

#[derive(Debug)]
pub(super) struct BorrowedFormalSourceProductV1 {
    pub(super) draft: Result<BorrowedFormalUsesDraftV1, BorrowedFormalUseDraftErrorV1>,
    pub(super) guarded_actuals: BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
}

/// Direct adapters borrow the same source product, never a second use scan.
#[cfg(test)]
pub(super) fn draft_borrowed_formal_uses_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    contract: &OwnedCallableParameterContractDeclarationV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<BorrowedFormalUsesDraftV1, BorrowedFormalUseDraftErrorV1> {
    draft_borrowed_formal_source_product_v1(input, contract, constructors, receiver, None)?.draft
}

pub(super) fn draft_borrowed_formal_source_product_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    contract: &OwnedCallableParameterContractDeclarationV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    static_operands: Option<&StaticOperandContextV1<'_>>,
) -> Result<BorrowedFormalSourceProductV1, BorrowedFormalUseDraftErrorV1> {
    let function = input.function();
    if input.owner() != contract.owner
        || function.owner() != contract.owner
        || function
            .declaration_sites()
            .filter(|site| matches!(site, SourceBindingSiteV1::Parameter { .. }))
            .count()
            != contract.parameters.len()
    {
        return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
    }
    let mut origins = BTreeMap::new();
    for (index, formal) in contract.parameters.iter().enumerate() {
        if formal.ordinal as usize != index
            || formal.binding.owner() != contract.owner
            || function.declaration_binding(&SourceBindingSiteV1::Parameter {
                index: formal.ordinal,
            }) != Some(formal.binding)
            || !matches!(function.binding(formal.binding).map(|row| row.kind()),
                Some(BindingKindV1::Parameter { index }) if index == formal.ordinal)
        {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        if formal.kind.is_ordinary_borrowed_handle() {
            if origins.insert(formal.binding, formal.binding).is_some() {
                return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
            }
        }
    }

    // Discover direct aliases to a fixed point, independent of declaration
    // iteration order. Each pass adds a finite sealed binding or terminates.
    let mut copies = BTreeMap::<SourceExprSiteV1, BindingRefV1>::new();
    loop {
        let before = origins.len();
        for initializer in function.expression_source().initializers() {
            let Some(site) = initializer.initializer_site() else {
                continue;
            };
            let Some(ResolvedLexicalRefV1::Local(source)) = function.variable_ref(site) else {
                continue;
            };
            let Some(formal) = origins.get(&source).copied() else {
                continue;
            };
            let destination = initializer.binding();
            if destination.owner() != input.owner()
                || function.declaration_binding(initializer.declaration_site()) != Some(destination)
                || !matches!(
                    function.binding(destination).map(|row| row.kind()),
                    Some(BindingKindV1::Local { .. })
                )
                || function
                    .expression_source()
                    .initializers()
                    .filter(|row| row.binding() == destination)
                    .count()
                    != 1
            {
                return Err(BorrowedFormalUseDraftErrorV1::AliasDeclaration(destination));
            }
            if initializer.declared_type_name().is_some() {
                return Err(BorrowedFormalUseDraftErrorV1::AnnotatedAlias(destination));
            }
            if let Some(previous) = origins.insert(destination, formal) {
                if previous != formal {
                    return Err(BorrowedFormalUseDraftErrorV1::AliasDeclaration(destination));
                }
            }
            if let Some(previous) = copies.insert(site.clone(), destination) {
                if previous != destination {
                    return Err(BorrowedFormalUseDraftErrorV1::AmbiguousUse(
                        OwnedExprSiteV1::new(input.owner(), site.clone()),
                    ));
                }
            }
        }
        if origins.len() == before {
            break;
        }
    }

    for (_, target) in function.assignment_targets() {
        if let ResolvedAssignmentTargetV1::BindingRebind(binding) = target {
            if origins.contains_key(binding) {
                return Err(BorrowedFormalUseDraftErrorV1::Rebound(*binding));
            }
        }
    }
    // Captures live in descendant semantic owners, not this function's read
    // inventory. Check the complete forest, including non-function owners.
    for (_, owner) in input.forest().semantic_owners() {
        for (_, target) in owner.assignment_targets() {
            if let ResolvedAssignmentTargetV1::UpvarRebind(upvar) = target {
                if origins.contains_key(&upvar.source()) {
                    return Err(BorrowedFormalUseDraftErrorV1::Captured(upvar.source()));
                }
            }
        }
        for (_, reference) in owner.variable_refs() {
            if let ResolvedLexicalRefV1::Upvar(upvar) = reference {
                if origins.contains_key(&upvar.source()) {
                    return Err(BorrowedFormalUseDraftErrorV1::Captured(upvar.source()));
                }
            }
        }
    }

    let numeric_origins: BTreeMap<_, _> = origins
        .iter()
        .filter(|(_, root)| {
            contract.parameters.iter().any(|formal| {
                formal.binding == **root
                    && formal.kind == CallableParameterContractKindV1::OpaqueHandle
            })
        })
        .map(|(binding, root)| (*binding, *root))
        .collect();

    // Admitted checked compares dominate the lent view's later uses; record
    // each compare's owning `if` statement per formal before the use loop so
    // a dominated `+` can prove its guard regardless of visit order. The
    // same pass records each admitted null compare's `if` — the surviving
    // successor is the only path where a `formal.field` read may carry a
    // live borrowed object.
    let mut compare_guards: BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>> = BTreeMap::new();
    let mut null_guards: BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>> = BTreeMap::new();
    let mut checked_compares = BTreeMap::new();
    let mut integer_guards: BTreeMap<BindingRefV1, Vec<Rc<guarded_actual::CheckedIntegerGuardV1>>> =
        BTreeMap::new();
    for (site, reference) in function.variable_refs() {
        let ResolvedLexicalRefV1::Local(binding) = reference else {
            continue;
        };
        let Some(formal) = origins.get(binding).copied() else {
            continue;
        };
        if copies.contains_key(site) || is_call_argument(input, site)? {
            continue;
        }
        if let Some(kind @ BorrowedFormalUseDraftKindV1::CompareOperand { .. }) =
            compare_operand_kind(
                input,
                &numeric_origins,
                constructors,
                receiver,
                site,
                static_operands,
            )?
        {
            let BorrowedFormalUseDraftKindV1::CompareOperand { binary, source } = &kind else {
                unreachable!()
            };
            integer_guards.entry(formal).or_default().push(Rc::new(
                guarded_actual::CheckedIntegerGuardV1::from_compare(
                    input,
                    formal,
                    binary,
                    source.clone(),
                )?,
            ));
            let guard = function
                .with_if_region_for_condition(binary.site(), |row| row.site().node().clone())
                .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
            compare_guards.entry(formal).or_default().push(guard);
            checked_compares.insert(site.clone(), kind);
        }
        if let Some(BorrowedFormalUseDraftKindV1::NullCompareOperand { binary }) =
            null_compare_operand_kind(input, site)?
        {
            let guard = function
                .with_if_region_for_condition(binary.site(), |row| row.site().node().clone())
                .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
            null_guards.entry(formal).or_default().push(guard);
        }
    }

    let mul_sources =
        mul_source::collect_mul_sources(input, &numeric_origins, &integer_guards, static_operands)?;
    let mut uses = Vec::new();
    let mut unsupported = None;
    let mut guarded_actuals = BTreeMap::new();
    for (site, reference) in function.variable_refs() {
        let ResolvedLexicalRefV1::Local(binding) = reference else {
            continue;
        };
        let Some(formal) = origins.get(binding).copied() else {
            continue;
        };
        let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
        let mut kind = copies
            .get(site)
            .map(|destination| BorrowedFormalUseDraftKindV1::Copy {
                destination: *destination,
            });
        for (call_site, call) in function.method_calls() {
            if call.owner() != input.owner() || call.arguments().len() != call.arity() as usize {
                return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
            }
            for (ordinal, argument) in call.arguments().iter().enumerate() {
                if argument.ordinal() as usize != ordinal {
                    return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
                }
                if argument.site() != site {
                    continue;
                }
                if kind.is_some() {
                    return Err(BorrowedFormalUseDraftErrorV1::AmbiguousUse(owned));
                }
                if let Some(fact) = integer_guards.get(&formal).and_then(|guards| {
                    guards.iter().find_map(|guard| {
                        BorrowedGuardedActualV1::from_call(
                            input,
                            guard.clone(),
                            call_site,
                            argument.ordinal(),
                            *binding,
                            site,
                        )
                    })
                }) {
                    if guarded_actuals
                        .insert((fact.call().clone(), argument.ordinal()), fact)
                        .is_some()
                    {
                        return Err(BorrowedFormalUseDraftErrorV1::AmbiguousUse(owned));
                    }
                }
                kind = if argument.ordinal() == 1 && numeric_origins.contains_key(binding) {
                    array_element::array_element_value_kind(
                        input,
                        &numeric_origins,
                        constructors,
                        receiver,
                        formal,
                        &compare_guards,
                        call,
                        site,
                    )?
                } else {
                    None
                };
                if kind.is_none() {
                    kind = Some(BorrowedFormalUseDraftKindV1::UnresolvedArgument {
                        call: OwnedExprSiteV1::new(input.owner(), call_site.clone()),
                        ordinal: argument.ordinal(),
                    });
                }
            }
        }
        if kind.is_none() && numeric_origins.contains_key(binding) {
            kind = new_argument::new_argument_kind(input, formal, &compare_guards, site)?;
        }
        if kind.is_none() && numeric_origins.contains_key(binding) {
            kind = checked_compares.remove(site);
        }
        if kind.is_none() && numeric_origins.contains_key(binding) {
            kind = add_operand_kind(
                input,
                &numeric_origins,
                constructors,
                receiver,
                formal,
                &compare_guards,
                site,
            )?;
        }
        if kind.is_none() && numeric_origins.contains_key(binding) {
            kind = mul_source::mul_operand_kind(&mul_sources, *binding, formal, site)?;
        }
        if kind.is_none() && numeric_origins.contains_key(binding) {
            kind = integer_guards.get(&formal).and_then(|guards| {
                guards
                    .iter()
                    .find_map(|guard| guard.return_kind(input, formal, *binding, site))
            });
        }
        if kind.is_none() {
            kind = null_compare_operand_kind(input, site)?;
        }
        if kind.is_none() {
            kind = field_read::field_read_operand_kind(input, formal, &null_guards, site)?;
        }
        let Some(kind) = kind else {
            unsupported.get_or_insert(BorrowedFormalUseDraftErrorV1::UnsupportedUse(owned));
            continue;
        };
        uses.push(BorrowedFormalUseDraftRowV1 {
            site: owned,
            binding: *binding,
            formal,
            kind,
        });
    }
    let draft = match unsupported {
        Some(error) => Err(error),
        None => Ok(BorrowedFormalUsesDraftV1 {
            origins,
            uses: uses.into_boxed_slice(),
        }),
    };
    Ok(BorrowedFormalSourceProductV1 {
        draft,
        guarded_actuals,
    })
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_use_tests.rs"]
mod tests;
