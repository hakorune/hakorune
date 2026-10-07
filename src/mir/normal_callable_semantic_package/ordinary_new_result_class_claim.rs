//! Callable result-class claims for the bounded ordinary-`New` cohort.
//!
//! Source authority: the resolver's sealed `BodyStatementShapeV1::Return`
//! rows and `constructions()`/`literal()`/`direct_call_target()`/
//! `method_call()`/`variable_ref()` membership — the same passive facts
//! the birth-site index and field-write claims already walk. Canonical
//! issuer: this module's draft, driven from
//! `issue_ordinary_source_cohort_v1` where the selected-key map is
//! already in scope.
//!
//! Pass A (`observe_function`) classifies every sealed `return` exit of
//! every keyed callable into `New` / `Null` / `ForwardCall` /
//! `ForwardLocal` — order-free, because `batch.declarations()` is parser
//! order, not call-graph order. Pass B (`finish`) resolves forwarded
//! exits to callee keys and composes by monotone fixpoint: claims only
//! grow, so iteration is bounded by the pending-key count and
//! self/mutual recursion simply stays unclaimed. A callee whose exits
//! are exactly `return new C(..)` claims `Object(C)`; adding exact
//! `null` exits — or forwarding to a `NullableObject` callee — degrades
//! the claim to `NullableObject(C)`. `NullableObject` is never a Handle
//! authorization. Missing inventories, value-less returns, forwarded
//! edges to unavailable callees, rebound locals, and mixed classes leave
//! the callable unavailable. One product also retains exact passive
//! Null/Fresh/ForwardFormal exit origins; mixed and null-only origins do
//! not gain legacy class membership or ownership authority.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, BodyStatementShapeV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedBodyShapeInventoryV1, VerifiedResolvedFunctionV1,
};
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

#[path = "ordinary_new_result_class_claim/call_witness.rs"]
mod call_witness;
#[path = "ordinary_new_result_class_claim/child_relation.rs"]
mod child_relation;
pub(super) use child_relation::attach_source_child_relations_v1;
#[path = "ordinary_new_result_class_claim/evaluate.rs"]
mod evaluate;
use evaluate::{evaluate_row, ExitVerdictV1};
#[path = "ordinary_new_result_class_claim/product.rs"]
mod product;
pub(crate) use product::{ResultExitOriginV1, ResultValueOriginV1};
#[path = "ordinary_new_result_class_claim/object_return_loan.rs"]
mod object_return_loan;
#[path = "ordinary_new_result_class_claim/witness.rs"]
mod witness;
pub(crate) use object_return_loan::ObjectReturnCallQualificationV1;
pub(crate) use witness::{ResultFormalSubstitutionV1, ResultOriginWitnessV1, ResultWitnessStepV1};

/// The proven result class of a selected callable. The class name is the
/// agreed `new` class; the arm records whether a `null` literal exit
/// also exists. `NullableObject` is never a Handle authorization — it
/// only states that every sealed exit is `new C(...)` or `null`.
/// `NullableForwarded` is the class-free identity arm: every value exit
/// returns the callable's own `ordinal`-th formal, so the result is the
/// caller's arg-`ordinal` object or `null`. Its class resolves only at
/// the caller, by substituting the call site's actual — an unprovable
/// actual leaves the caller's row dead.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OrdinaryNewResultClassV1 {
    /// Every `return` constructs `new` of the agreed class — definite object.
    Object(Box<str>),
    /// Exits are `new` of the agreed class, the exact `null` literal, or
    /// forwards to a nullable callee.
    NullableObject(Box<str>),
    /// Every value `return` passes the `ordinal`-th formal through — the
    /// result is the caller's own arg-`ordinal` value or `null`. The
    /// callee asserts identity only; the caller supplies the class.
    NullableForwarded { ordinal: u32 },
}

impl OrdinaryNewResultClassV1 {
    pub(crate) fn class(&self) -> Option<&str> {
        match self {
            Self::Object(class) | Self::NullableObject(class) => Some(class.as_ref()),
            Self::NullableForwarded { .. } => None,
        }
    }
}

/// Single source product; legacy lookups expose only safe class projections.
pub(crate) type OrdinaryNewResultClassClaimsV1 = product::VerifiedSourceCallableResultFactsV1;

/// Pass-A exit classification. Forwarded shapes carry the exact source
/// site so pass B can resolve the callee key from the same sealed facts.
enum ResultClassExitDraftV1 {
    /// `return new <class>(..)`
    New(Box<str>),
    /// `return null`
    Null,
    /// `return <call>` — the call site the forwarded edge leaves through.
    ForwardCall(SourceExprSiteV1),
    /// `return <local>` — the returned binding; pass B proves its sole
    /// initializer and no rebind before composing.
    ForwardLocal(BindingRefV1),
    /// `return <formal>` — a `Parameter` binding pass-through; pass B
    /// reads the formal's contract kind for class or forwarded identity.
    ForwardFormal { binding: BindingRefV1, ordinal: u32 },
}

/// An exit after callee resolution: `Fwd` waits on the callee's claim and
/// carries the call site's `Local` actuals so a `NullableForwarded`
/// callee can substitute the forwarded formal's class at the caller.
/// Ordinary declared and opaque formal returns preserve borrowed identity.
/// `Formal` records the original input ordinal without an owning class.
enum PendingExitV1 {
    New(Box<str>),
    Null,
    Fwd {
        call_site: crate::mir::resolved_semantics::OwnedExprSiteV1,
        key: CanonicalSameModuleCallableKeyV1,
        actuals: Box<[ResultActualSourceV1]>,
    },
    Formal {
        binding: BindingRefV1,
        ordinal: u32,
    },
}

#[derive(Clone)]
struct ResultActualSourceV1 {
    site: crate::mir::resolved_semantics::OwnedExprSiteV1,
    binding: Option<BindingRefV1>,
}

struct PendingResultExitV1 {
    site: crate::mir::resolved_semantics::OwnedExprSiteV1,
    exit: PendingExitV1,
}

struct ResultClassDraftRowV1 {
    key: CanonicalSameModuleCallableKeyV1,
    batch_slot: u32,
    exits: Vec<(
        crate::mir::resolved_semantics::OwnedExprSiteV1,
        ResultClassExitDraftV1,
    )>,
}

#[derive(Default)]
pub(crate) struct OrdinaryNewResultClassClaimDraftV1 {
    rows: Vec<ResultClassDraftRowV1>,
}

/// The value sites of a callable's verified explicit value returns —
/// `Some` only when the Completion product proves every function exit
/// is an explicit value `return` (no implicit end, no bare `return`).
/// This is the sole exit-evidence authority for this package: callers
/// that re-derive "the last statement is a return" from the flat
/// `statements()` list misread a nested trailing `if`/`loop` return as
/// the body tail.
pub(in crate::mir::normal_callable_semantic_package) fn verified_value_return_sites(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
) -> Option<Vec<SourceExprSiteV1>> {
    let completion =
        crate::mir::resolved_control_flow::verify_function_completion_v1(input).ok()?;
    if !completion.returns_value() {
        return None;
    }
    let return_values: BTreeMap<SourceStmtSiteV1, SourceExprSiteV1> = shape
        .statements()
        .iter()
        .filter_map(|statement| {
            let BodyStatementShapeV1::Return { site, value } = statement else {
                return None;
            };
            value.as_ref().map(|value| (site.clone(), value.clone()))
        })
        .collect();
    completion
        .explicit_sites()
        .iter()
        .map(|site| return_values.get(site).cloned())
        .collect()
}

/// The owning box's own method-call target — `me` receivers only exist
/// inside `InstanceBoxMethod` callables.
fn own_box(key: &CanonicalSameModuleCallableKeyV1) -> Option<&str> {
    (key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod).then(|| key.owner())
}

/// The binding is a live `me` receiver of the containing function.
fn is_receiver_binding(function: &VerifiedResolvedFunctionV1, binding: BindingRefV1) -> bool {
    function
        .binding(binding)
        .is_some_and(|record| record.kind() == BindingKindV1::Receiver)
}

/// Original binding identity is usable only when the sealed source records no
/// rebind. This conservative proof does not infer assignment ordering.
fn binding_is_unrebound(function: &VerifiedResolvedFunctionV1, binding: BindingRefV1) -> bool {
    !function.assignment_targets().any(|(_, target)| {
        matches!(target, ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == binding)
    })
}

/// The binding's sole initializer site — `None` when the binding is
/// rebound, never initialized, or initialized more than once. A
/// rebound or multi-initializer local cannot prove a result edge.
fn sole_initializer_site(
    function: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
) -> Option<SourceExprSiteV1> {
    if !binding_is_unrebound(function, binding) {
        return None;
    }
    let mut initializers = function
        .expression_source()
        .initializers()
        .filter(|initializer| initializer.binding() == binding);
    let initializer = initializers.next()?;
    if initializers.next().is_some() {
        return None;
    }
    initializer.initializer_site().cloned()
}

/// Class of a `local x = <initializer>` binding when the initializer is
/// exactly `new C(..)` or a `me.f` field read with a field-write claim.
/// Call-result or other initializers prove nothing here — forwarded
/// composition handles the call case at the callee level.
fn receiver_local_class(
    function: &VerifiedResolvedFunctionV1,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    binding: BindingRefV1,
    own_box: Option<&str>,
    field_write_claims: &super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Option<Box<str>> {
    let initializer_site = sole_initializer_site(function, binding)?;
    if let Some(construction) = function.expression_source().construction(&initializer_site) {
        return Some(construction.class().into());
    }
    if let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        body_shape.expression_shape(&initializer_site)
    {
        let Some(BodyExpressionShapeV1::Me {
            receiver: BodyMeReceiverV1::Lexical(receiver),
            ..
        }) = body_shape.expression_shape(object)
        else {
            return None;
        };
        if !is_receiver_binding(function, *receiver) {
            return None;
        }
        return field_write_claims
            .get(&(own_box?.into(), field.clone()))
            .cloned();
    }
    None
}

/// Both direct and method calls must preserve the original actual binding.
fn call_actual_binding(
    site: &SourceExprSiteV1,
    function: &VerifiedResolvedFunctionV1,
) -> Option<BindingRefV1> {
    match function.variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) if binding_is_unrebound(function, binding) => {
            Some(binding)
        }
        _ => None,
    }
}

/// The `Local` bindings carried at one call's argument positions —
/// `None` for any position the source does not resolve to a `Local`
/// reference. A callee's `NullableForwarded` claim substitutes these.
fn call_actual_bindings(
    sites: &[SourceExprSiteV1],
    function: &VerifiedResolvedFunctionV1,
) -> Box<[ResultActualSourceV1]> {
    sites
        .iter()
        .map(|site| ResultActualSourceV1 {
            site: crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                function.owner(),
                site.clone(),
            ),
            binding: call_actual_binding(site, function),
        })
        .collect()
}

/// Resolve a `return <call>` / `<binding> = <call>` site to the callee's
/// canonical key: direct calls through the sealed direct-call target,
/// `me.m()` through the own-box instance method, `me.f.m()` through the
/// field-write claim, and `x.m()` through the receiver binding's sole
/// initializer. Anything else (qualified receivers, dynamic receivers,
/// parameter receivers, unresolvable edges) proves nothing. The returned
/// actuals are the call site's `Local` bindings per argument position.
fn resolve_call_key(
    site: &SourceExprSiteV1,
    function: &VerifiedResolvedFunctionV1,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    caller_key: &CanonicalSameModuleCallableKeyV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    field_write_claims: &super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Option<(
    CanonicalSameModuleCallableKeyV1,
    Box<[ResultActualSourceV1]>,
)> {
    if let Some(target) = function.direct_call_target(site) {
        let callee_owner = target.callable().owner();
        let mut declarations = batch
            .declarations()
            .filter(|declaration| declaration.owner() == callee_owner);
        let declaration = declarations.next()?;
        if declarations.next().is_some() {
            return None;
        }
        let actuals = function
            .direct_call_observation(site)
            .map(|observation| call_actual_bindings(observation.argument_sites(), function))
            .unwrap_or_default();
        return match selected.key_for_batch_slot(declaration.batch_slot()) {
            Some(SelectedNormalCallableKeyV1::Cataloged(key)) => Some((key.clone(), actuals)),
            Some(SelectedNormalCallableKeyV1::TopLevel(top_level)) => {
                let arity = u32::try_from(top_level.declared_arity()).ok()?;
                Some((
                    CanonicalSameModuleCallableKeyV1::free_function(
                        top_level.declared_name(),
                        arity,
                    ),
                    actuals,
                ))
            }
            None => None,
        };
    }
    let call = function.method_call(site)?;
    let actuals: Box<[ResultActualSourceV1]> = call
        .arguments()
        .iter()
        .map(|argument| ResultActualSourceV1 {
            site: crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                function.owner(),
                argument.site().clone(),
            ),
            binding: call_actual_binding(argument.site(), function),
        })
        .collect();
    match call.receiver() {
        ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) => {
            let class: Box<str> = if is_receiver_binding(function, binding) {
                own_box(caller_key)?.into()
            } else {
                let record = function.binding(binding)?;
                if !matches!(record.kind(), BindingKindV1::Local { .. }) {
                    return None;
                }
                receiver_local_class(
                    function,
                    body_shape,
                    binding,
                    own_box(caller_key),
                    field_write_claims,
                )?
            };
            super::lexical_instance_call::unique_instance_target(
                selected,
                class.as_ref(),
                call.selector(),
                call.arity(),
            )
            .map(|(key, _)| (key, actuals.clone()))
        }
        ResolvedMethodCallReceiverSourceV1::Other => {
            let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
                body_shape.expression_shape(call.receiver_site())
            else {
                return None;
            };
            let Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(receiver),
                ..
            }) = body_shape.expression_shape(object)
            else {
                return None;
            };
            if !is_receiver_binding(function, *receiver) {
                return None;
            }
            let class = field_write_claims.get(&(own_box(caller_key)?.into(), field.clone()))?;
            super::lexical_instance_call::unique_instance_target(
                selected,
                class.as_ref(),
                call.selector(),
                call.arity(),
            )
            .map(|(key, _)| (key, actuals.clone()))
        }
        _ => None,
    }
}

/// Resolve a `return <local>` exit: the binding must be an unrebound
/// local with a sole initializer, and the initializer must be a
/// supported forwarded call — a `new`/`null`/other initializer kills
/// the row, keeping the grammar to F5 exactly.
fn resolve_forward_local(
    binding: BindingRefV1,
    function: &VerifiedResolvedFunctionV1,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    caller_key: &CanonicalSameModuleCallableKeyV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    field_write_claims: &super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Option<PendingExitV1> {
    let record = function.binding(binding)?;
    if !matches!(record.kind(), BindingKindV1::Local { .. }) {
        return None;
    }
    let initializer_site = sole_initializer_site(function, binding)?;
    resolve_call_key(
        &initializer_site,
        function,
        body_shape,
        caller_key,
        batch,
        selected,
        field_write_claims,
    )
    .map(|(key, actuals)| PendingExitV1::Fwd {
        call_site: crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            function.owner(),
            initializer_site,
        ),
        key,
        actuals,
    })
}

/// Exact ordinal/kind of one original formal binding in its declaration.
/// Class membership never supplies an owning destination contract.
fn parameter_contract<'a>(
    parameter_contracts: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    batch_slot: u32,
    binding: BindingRefV1,
) -> Option<&'a crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1>
{
    parameter_contracts
        .iter()
        .find(|declaration| declaration.batch_slot == batch_slot)?
        .parameters
        .iter()
        .find(|parameter| parameter.binding == binding)
}

impl OrdinaryNewResultClassClaimDraftV1 {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Pass A: observe one selected callable, classifying every sealed
    /// `return` exit. The exit set is the verified Completion's own —
    /// `returns_value` proves every function exit is an explicit value
    /// return, so a nested `return` inside a trailing `if`/`loop` can
    /// never masquerade as the body tail. Any unclassifiable value — a
    /// field read, an upvar, a non-null literal — drops the row
    /// entirely; the row can never compose a class.
    pub(crate) fn observe_function(
        &mut self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        key: &CanonicalSameModuleCallableKeyV1,
        batch_slot: u32,
    ) {
        let function = input.function();
        let Some(shape) = input.body_shape() else {
            return;
        };
        let Some(value_sites) = verified_value_return_sites(input, shape) else {
            return;
        };
        let mut exits = Vec::new();
        for site in value_sites {
            let exit = if matches!(
                function.expression_source().literal(&site),
                Some(ResolvedLiteralSourceV1::Null)
            ) {
                ResultClassExitDraftV1::Null
            } else if let Some(construction) = function.expression_source().construction(&site) {
                ResultClassExitDraftV1::New(construction.class().into())
            } else if function.direct_call_target(&site).is_some()
                || function.method_call(&site).is_some()
            {
                ResultClassExitDraftV1::ForwardCall(site.clone())
            } else {
                match function.variable_ref(&site) {
                    Some(ResolvedLexicalRefV1::Local(binding)) => {
                        match function.binding(binding).map(|record| record.kind()) {
                            Some(BindingKindV1::Local { .. }) => {
                                ResultClassExitDraftV1::ForwardLocal(binding)
                            }
                            Some(BindingKindV1::Parameter { index }) => {
                                if !binding_is_unrebound(function, binding) {
                                    return;
                                }
                                ResultClassExitDraftV1::ForwardFormal {
                                    binding,
                                    ordinal: index,
                                }
                            }
                            _ => return,
                        }
                    }
                    _ => return,
                }
            };
            exits.push((
                crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), site),
                exit,
            ));
        }
        self.rows.push(ResultClassDraftRowV1 {
            key: key.clone(),
            batch_slot,
            exits,
        });
    }

    /// Pass B: resolve forwarded exits against the batch's own sealed
    /// facts, then compose by monotone fixpoint — every iteration either
    /// mints newly-provable claims or removes rows whose forwarded callee
    /// can never claim, so iteration is bounded by the pending-key count.
    /// Fresh classes require ordinary source coverage before insertion;
    /// forwarded identities carry no acquired Home. Passive mixed/null-only
    /// rows remain absent from the legacy class projection.
    pub(crate) fn finish(
        self,
        ordinary_box_coverage: &ParserOrdinaryBoxSourceCoverageV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        field_write_claims: &super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    ) -> OrdinaryNewResultClassClaimsV1 {
        use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
        let mut pending: BTreeMap<
            CanonicalSameModuleCallableKeyV1,
            (u32, Vec<PendingResultExitV1>),
        > = BTreeMap::new();
        for row in self.rows {
            if !matches!(selected.key_for_batch_slot(row.batch_slot),
                Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == &row.key)
            {
                continue;
            }
            let resolved = batch.with_lowering_input(row.batch_slot, |input| {
                let function = input.function();
                let Some(body_shape) = input.body_shape() else {
                    return None;
                };
                row.exits
                    .iter()
                    .map(|(site, exit)| {
                        if site.owner() != input.owner() {
                            return None;
                        }
                        let exit = match exit {
                            ResultClassExitDraftV1::New(class) => {
                                Some(PendingExitV1::New(class.clone()))
                            }
                            ResultClassExitDraftV1::Null => Some(PendingExitV1::Null),
                            ResultClassExitDraftV1::ForwardCall(site) => resolve_call_key(
                                site,
                                function,
                                body_shape,
                                &row.key,
                                batch,
                                selected,
                                field_write_claims,
                            )
                            .map(|(key, actuals)| PendingExitV1::Fwd {
                                call_site: crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    input.owner(),
                                    site.clone(),
                                ),
                                key,
                                actuals,
                            }),
                            ResultClassExitDraftV1::ForwardLocal(binding) => resolve_forward_local(
                                *binding,
                                function,
                                body_shape,
                                &row.key,
                                batch,
                                selected,
                                field_write_claims,
                            ),
                            ResultClassExitDraftV1::ForwardFormal { binding, ordinal } => {
                                // Both ordinary kinds borrow their input. A type
                                // annotation proves class, not a moved-in Home.
                                parameter_contract(parameter_contracts, row.batch_slot, *binding)
                                    .filter(|parameter| parameter.ordinal == *ordinal)
                                    .filter(|parameter| {
                                        matches!(
                                            parameter.kind,
                                            CallableParameterContractKindV1::DeclaredObject(_)
                                                | CallableParameterContractKindV1::OpaqueHandle
                                        )
                                    })
                                    .map(|_| PendingExitV1::Formal {
                                        binding: *binding,
                                        ordinal: *ordinal,
                                    })
                            }
                        }?;
                        Some(PendingResultExitV1 {
                            site: site.clone(),
                            exit,
                        })
                    })
                    .collect::<Option<Vec<_>>>()
            });
            if let Ok(Some(exits)) = resolved {
                pending.insert(row.key, (row.batch_slot, exits));
            }
        }
        let mut claims = OrdinaryNewResultClassClaimsV1::new();
        loop {
            let pending_keys: BTreeSet<CanonicalSameModuleCallableKeyV1> =
                pending.keys().cloned().collect();
            let mut inserts = Vec::new();
            let mut deads = Vec::new();
            for (key, (batch_slot, exits)) in &pending {
                match evaluate_row(
                    exits,
                    &claims,
                    &pending_keys,
                    parameter_contracts,
                    *batch_slot,
                    ordinary_box_coverage,
                ) {
                    ExitVerdictV1::Resolvable(claim) => inserts.push((key.clone(), claim)),
                    ExitVerdictV1::Dead => deads.push(key.clone()),
                    ExitVerdictV1::Waiting => {}
                }
            }
            if inserts.is_empty() && deads.is_empty() {
                break;
            }
            for key in deads {
                pending.remove(&key);
            }
            for (key, claim) in inserts {
                pending.remove(&key);
                claims.insert(key, claim);
            }
        }
        claims
    }
}

#[cfg(test)]
mod source_brand_tests {
    use super::*;
    use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog as issue;

    #[test]
    fn foreign_owner_or_target_cannot_issue_result_origins() {
        let source = "box Token { value: i64 birth(value) { me.value = value } } box Door { give(h: Token) { return h } } static box Main { main() { return 0 } }";
        let package = issue(source).unwrap();
        let foreign = issue(source).unwrap();
        let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Door", "give", 1);
        let slot = package
            .batch
            .declarations()
            .find(|row| {
                matches!(
                    package.selected.key_for_batch_slot(row.batch_slot()),
                    Some(SelectedNormalCallableKeyV1::Cataloged(found)) if found == &key
                )
            })
            .unwrap()
            .batch_slot();
        let foreign_owner = foreign
            .batch
            .declarations()
            .find(|row| {
                matches!(
                    foreign.selected.key_for_batch_slot(row.batch_slot()),
                    Some(SelectedNormalCallableKeyV1::Cataloged(found)) if found == &key
                )
            })
            .unwrap()
            .owner();
        for corrupt_target in [false, true] {
            let mut draft = OrdinaryNewResultClassClaimDraftV1::new();
            package
                .batch
                .with_lowering_input(slot, |input| draft.observe_function(input, &key, slot))
                .unwrap();
            assert_eq!(draft.rows.len(), 1);
            if corrupt_target {
                draft.rows[0].key =
                    CanonicalSameModuleCallableKeyV1::instance_box_method("Foreign", "give", 1);
            } else {
                let site = &mut draft.rows[0].exits[0].0;
                *site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                    foreign_owner,
                    site.site().clone(),
                );
            }
            let facts = draft.finish(
                package.batch.ordinary_box_coverage(),
                &package.batch,
                &package.selected,
                &package.ordinary_new_claim_ledger.field_write_claims,
                &package.parameter_contracts,
            );
            assert!(facts.outcomes(&key).is_none());
            assert!(facts
                .outcomes(&CanonicalSameModuleCallableKeyV1::instance_box_method(
                    "Foreign", "give", 1
                ))
                .is_none());
            assert!(!facts.contains_key(&key));
        }
    }
}

#[cfg(test)]
#[path = "ordinary_new_result_class_claim/witness_tests.rs"]
mod witness_tests;

#[cfg(test)]
pub(in crate::mir::normal_callable_semantic_package) use witness_tests::source_result_facts_for_test;
