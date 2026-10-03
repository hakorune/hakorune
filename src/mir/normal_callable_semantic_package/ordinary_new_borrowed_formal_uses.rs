//! Structural use closure inside the ordinary lexical-call owner.
//!
//! This draft is not ABI authority. Argument rows remain unresolved until
//! the same package issuer joins their exact selected target/formal and
//! verifies all incoming edges and live actuals. No carrier is installed here.

use std::collections::BTreeMap;

use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceNodeSiteV1,
};

#[path = "ordinary_new_borrowed_formal_use_array_element.rs"]
mod array_element;

#[path = "ordinary_new_borrowed_formal_use_new_argument.rs"]
mod new_argument;

#[path = "ordinary_new_borrowed_formal_use_operands.rs"]
mod operands;

use operands::{
    add_operand_kind, compare_operand_kind, is_call_argument, normal_integer_operand,
    null_compare_operand_kind, use_dominated_by_if,
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
    },
    /// An ordered `+` operand use dominated by an admitted checked compare
    /// of the same formal, under the operation owner's
    /// `Add(NormalInteger, NormalInteger)` envelope. The lent view carries
    /// no borrowed identity into the fresh-integer result.
    AddOperand {
        binary: OwnedExprSiteV1,
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

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BorrowedIncomingDraftErrorV1 {
    SourceIdentity,
    BatchLoan,
    UnresolvedCaller(OwnedExprSiteV1),
    OutsideOrdinaryScope(OwnedExprSiteV1),
    CallIdentity(OwnedExprSiteV1),
    NoIncoming(FunctionOwnerIdV1),
}

#[derive(Debug)]
pub(super) struct BorrowedIncomingCallDraftV1 {
    pub(super) source: super::LexicalInstanceCallSourceTargetV1,
    pub(super) call: OwnedExprSiteV1,
    pub(super) callee: FunctionOwnerIdV1,
    pub(super) arguments: Box<[(u32, SourceExprSiteV1, BindingRefV1)]>,
}

/// Enumerate the complete source batch, including unselected callers. An
/// unresolved selector/arity match can veto selection but never proves a
/// target. `ordinary_callers` must later be corroborated against the issued
/// Ordinary source scopes; these draft rows install no ABI or live actual.
pub(super) fn draft_borrowed_incoming_calls_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::VerifiedSelectedCallableBatchMapV1,
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &super::LexicalInstanceCallSourceTargetV1>,
    ordinary_callers: &std::collections::BTreeSet<FunctionOwnerIdV1>,
) -> Result<Box<[BorrowedIncomingCallDraftV1]>, BorrowedIncomingDraftErrorV1> {
    let mut definitions = BTreeMap::new();
    for owner in drafts.keys() {
        let mut matching = contracts.iter().filter(|row| row.owner == *owner);
        let contract = matching
            .next()
            .ok_or(BorrowedIncomingDraftErrorV1::SourceIdentity)?;
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        };
        if matching.next().is_some()
            || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || key.arity() as usize != contract.parameters.len()
            || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
            || !contract.parameters.iter().enumerate().all(|(ordinal, formal)| {
                formal.ordinal as usize == ordinal && formal.binding.owner() == *owner
                    && (formal.kind != CallableParameterContractKindV1::OpaqueHandle
                        || drafts[owner].origins.get(&formal.binding) == Some(&formal.binding))
            })
            || !contract
                .parameters
                .iter()
                .any(|formal| formal.kind == CallableParameterContractKindV1::OpaqueHandle)
        {
            return Err(BorrowedIncomingDraftErrorV1::SourceIdentity);
        }
        definitions.insert(*owner, (contract, key));
    }
    let mut rows = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for declaration in batch.declarations() {
        batch
            .with_lowering_input(declaration.batch_slot(), |input| {
                for (site, call) in input.function().method_calls() {
                    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
                    let exact = calls.get(&owned);
                    for (callee, (contract, key)) in &definitions {
                        if call.selector() != key.name() || call.arity() != key.arity() {
                            continue;
                        }
                        let Some(exact) = exact else {
                            return Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(owned));
                        };
                        // A proven different receiver class is not an incoming
                        // edge of this definition, despite the same method name.
                        if exact.target() != *key {
                            continue;
                        }
                        if !ordinary_callers.contains(&input.owner()) {
                            return Err(BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(owned));
                        }
                        if exact.call_site() != &owned
                            || exact.callee_owner() != *callee
                            || exact.target_batch_slot() != contract.batch_slot
                            || call.owner() != input.owner()
                            || call.site() != site
                            || call.arguments().len() != contract.parameters.len()
                            || exact.argument_sites().len() != call.arguments().len()
                            || !call
                                .arguments()
                                .iter()
                                .enumerate()
                                .all(|(ordinal, argument)| {
                                    argument.ordinal() as usize == ordinal
                                        && exact.argument_sites()[ordinal] == *argument.site()
                                        && contract.parameters[ordinal].ordinal as usize == ordinal
                                })
                        {
                            return Err(BorrowedIncomingDraftErrorV1::CallIdentity(owned));
                        }
                        let arguments = call
                            .arguments()
                            .iter()
                            .zip(&contract.parameters)
                            .filter(|(_, formal)| {
                                formal.kind == CallableParameterContractKindV1::OpaqueHandle
                            })
                            .map(|(argument, formal)| {
                                (argument.ordinal(), argument.site().clone(), formal.binding)
                            })
                            .collect();
                        rows.push(BorrowedIncomingCallDraftV1 {
                            source: (*exact).clone(),
                            call: owned.clone(),
                            callee: *callee,
                            arguments,
                        });
                        seen.insert(*callee);
                    }
                }
                Ok(())
            })
            .map_err(|_| BorrowedIncomingDraftErrorV1::BatchLoan)??;
    }
    for owner in definitions.keys() {
        if !seen.contains(owner) {
            return Err(BorrowedIncomingDraftErrorV1::NoIncoming(*owner));
        }
    }
    Ok(rows.into_boxed_slice())
}

/// Join every unresolved argument to the existing exact lexical disposition
/// and the callee's source-owned opaque binding. The complete finite draft
/// set permits cycles without assuming a recursive edge succeeds. This
/// result still does not prove incoming coverage, availability or carrier ABI.
pub(super) fn join_borrowed_forward_uses_v1(
    drafts: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    calls: &BTreeMap<OwnedExprSiteV1, &super::LexicalInstanceCallSourceTargetV1>,
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
                || exact_call.target().namespace()
                    != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
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
                || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
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
                || formal.kind != CallableParameterContractKindV1::OpaqueHandle
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

pub(super) fn draft_borrowed_formal_uses_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    contract: &OwnedCallableParameterContractDeclarationV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<BorrowedFormalUsesDraftV1, BorrowedFormalUseDraftErrorV1> {
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
        if formal.kind == CallableParameterContractKindV1::OpaqueHandle {
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

    // Admitted checked compares dominate the lent view's later uses; record
    // each compare's owning `if` statement per formal before the use loop so
    // a dominated `+` can prove its guard regardless of visit order.
    let mut compare_guards: BTreeMap<BindingRefV1, Vec<SourceNodeSiteV1>> = BTreeMap::new();
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
        let Some(BorrowedFormalUseDraftKindV1::CompareOperand { binary }) =
            compare_operand_kind(input, &origins, constructors, receiver, site)?
        else {
            continue;
        };
        let guard = function
            .with_if_region_for_condition(binary.site(), |row| row.site().node().clone())
            .map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
        compare_guards.entry(formal).or_default().push(guard);
    }

    let mut uses = Vec::new();
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
                kind = if argument.ordinal() == 1 {
                    array_element::array_element_value_kind(
                        input,
                        &origins,
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
        if kind.is_none() {
            kind = new_argument::new_argument_kind(input, formal, &compare_guards, site)?;
        }
        if kind.is_none() {
            kind = compare_operand_kind(
                input,
                &origins,
                constructors,
                receiver,
                site,
            )?;
        }
        if kind.is_none() {
            kind = add_operand_kind(
                input,
                &origins,
                constructors,
                receiver,
                formal,
                &compare_guards,
                site,
            )?;
        }
        if kind.is_none() {
            kind = null_compare_operand_kind(input, site)?;
        }
        uses.push(BorrowedFormalUseDraftRowV1 {
            site: owned.clone(),
            binding: *binding,
            formal,
            kind: kind.ok_or(BorrowedFormalUseDraftErrorV1::UnsupportedUse(owned))?,
        });
    }
    Ok(BorrowedFormalUsesDraftV1 {
        origins,
        uses: uses.into_boxed_slice(),
    })
}

#[cfg(test)]
#[path = "ordinary_new_borrowed_formal_use_tests.rs"]
mod tests;
