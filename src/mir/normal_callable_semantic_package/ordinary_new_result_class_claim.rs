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
//! edges to unclaimed callees, parameters, rebound locals, and mixed
//! classes all leave the callable unclaimed — additive evidence, never a
//! fallback.

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

/// The proven result class of a selected callable. The class name is the
/// agreed `new` class; the arm records whether a `null` literal exit
/// also exists. `NullableObject` is never a Handle authorization — it
/// only states that every sealed exit is `new C(...)` or `null`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum OrdinaryNewResultClassV1 {
    /// Every `return` constructs `new` of the agreed class — definite object.
    Object(Box<str>),
    /// Exits are `new` of the agreed class, the exact `null` literal, or
    /// forwards to a nullable callee.
    NullableObject(Box<str>),
}

impl OrdinaryNewResultClassV1 {
    pub(crate) fn class(&self) -> &str {
        match self {
            Self::Object(class) | Self::NullableObject(class) => class.as_ref(),
        }
    }

    const fn is_nullable(&self) -> bool {
        matches!(self, Self::NullableObject(_))
    }
}

/// Key: the selected canonical callable key. Value: the agreed result
/// class claim every `return` satisfies.
pub(crate) type OrdinaryNewResultClassClaimsV1 =
    BTreeMap<CanonicalSameModuleCallableKeyV1, OrdinaryNewResultClassV1>;

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
}

/// An exit after callee resolution: `Fwd` waits on the callee's claim.
enum PendingExitV1 {
    New(Box<str>),
    Null,
    Fwd(CanonicalSameModuleCallableKeyV1),
}

struct ResultClassDraftRowV1 {
    key: CanonicalSameModuleCallableKeyV1,
    batch_slot: u32,
    exits: Vec<ResultClassExitDraftV1>,
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

/// The binding's sole initializer site — `None` when the binding is
/// rebound, never initialized, or initialized more than once. A
/// rebound or multi-initializer local cannot prove a result edge.
fn sole_initializer_site(
    function: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
) -> Option<SourceExprSiteV1> {
    if function.assignment_targets().any(|(_, target)| {
        matches!(
            target,
            ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == binding
        )
    }) {
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

/// Resolve a `return <call>` / `<binding> = <call>` site to the callee's
/// canonical key: direct calls through the sealed direct-call target,
/// `me.m()` through the own-box instance method, `me.f.m()` through the
/// field-write claim, and `x.m()` through the receiver binding's sole
/// initializer. Anything else (qualified receivers, dynamic receivers,
/// parameter receivers, unresolvable edges) proves nothing.
fn resolve_call_key(
    site: &SourceExprSiteV1,
    function: &VerifiedResolvedFunctionV1,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    caller_key: &CanonicalSameModuleCallableKeyV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    field_write_claims: &super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Option<CanonicalSameModuleCallableKeyV1> {
    if let Some(target) = function.direct_call_target(site) {
        let callee_owner = target.callable().owner();
        let mut declarations = batch
            .declarations()
            .filter(|declaration| declaration.owner() == callee_owner);
        let declaration = declarations.next()?;
        if declarations.next().is_some() {
            return None;
        }
        return match selected.key_for_batch_slot(declaration.batch_slot()) {
            Some(SelectedNormalCallableKeyV1::Cataloged(key)) => Some(key.clone()),
            Some(SelectedNormalCallableKeyV1::TopLevel(top_level)) => {
                let arity = u32::try_from(top_level.declared_arity()).ok()?;
                Some(CanonicalSameModuleCallableKeyV1::free_function(
                    top_level.declared_name(),
                    arity,
                ))
            }
            None => None,
        };
    }
    let call = function.method_call(site)?;
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
            .map(|(key, _)| key)
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
            .map(|(key, _)| key)
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
    .map(PendingExitV1::Fwd)
}

/// Whether every exit of this row now composes: `New`/`Null` are self-
/// evident, `Fwd` waits on the callee's claim. Waiting on a key that is
/// neither claimed nor still pending means the callee can never prove a
/// class — the row is dead, not pending.
enum ExitVerdictV1 {
    Resolvable(OrdinaryNewResultClassV1),
    Waiting,
    Dead,
}

fn evaluate_row(
    exits: &[PendingExitV1],
    claims: &OrdinaryNewResultClassClaimsV1,
    pending: &BTreeSet<CanonicalSameModuleCallableKeyV1>,
) -> ExitVerdictV1 {
    let mut class: Option<Box<str>> = None;
    let mut nullable = false;
    let mut waiting = false;
    for exit in exits {
        let found_class: &str = match exit {
            PendingExitV1::Null => {
                nullable = true;
                continue;
            }
            PendingExitV1::New(found) => found.as_ref(),
            PendingExitV1::Fwd(found_key) => match claims.get(found_key) {
                Some(claim) => {
                    nullable |= claim.is_nullable();
                    claim.class()
                }
                None if pending.contains(found_key) => {
                    waiting = true;
                    continue;
                }
                None => return ExitVerdictV1::Dead,
            },
        };
        match &class {
            None => class = Some(found_class.into()),
            Some(existing) if existing.as_ref() == found_class => {}
            Some(_) => return ExitVerdictV1::Dead,
        }
    }
    if waiting {
        return ExitVerdictV1::Waiting;
    }
    let Some(class) = class else {
        return ExitVerdictV1::Dead;
    };
    ExitVerdictV1::Resolvable(if nullable {
        OrdinaryNewResultClassV1::NullableObject(class)
    } else {
        OrdinaryNewResultClassV1::Object(class)
    })
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
    /// parameter, a field read, an upvar, a bare literal — drops the row
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
                ResultClassExitDraftV1::ForwardCall(site)
            } else {
                match function.variable_ref(&site) {
                    Some(ResolvedLexicalRefV1::Local(binding))
                        if function.binding(binding).is_some_and(|record| {
                            matches!(record.kind(), BindingKindV1::Local { .. })
                        }) =>
                    {
                        ResultClassExitDraftV1::ForwardLocal(binding)
                    }
                    _ => return,
                }
            };
            exits.push(exit);
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
    /// Claims whose class is not an ordinary box of this package are
    /// dropped at the end, as before.
    pub(crate) fn finish(
        self,
        ordinary_box_coverage: &ParserOrdinaryBoxSourceCoverageV1,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        field_write_claims: &super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    ) -> OrdinaryNewResultClassClaimsV1 {
        let mut pending: BTreeMap<CanonicalSameModuleCallableKeyV1, Vec<PendingExitV1>> =
            BTreeMap::new();
        for row in self.rows {
            let resolved = batch.with_lowering_input(row.batch_slot, |input| {
                let function = input.function();
                let Some(body_shape) = input.body_shape() else {
                    return None;
                };
                row.exits
                    .iter()
                    .map(|exit| match exit {
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
                        .map(PendingExitV1::Fwd),
                        ResultClassExitDraftV1::ForwardLocal(binding) => resolve_forward_local(
                            *binding,
                            function,
                            body_shape,
                            &row.key,
                            batch,
                            selected,
                            field_write_claims,
                        ),
                    })
                    .collect::<Option<Vec<_>>>()
            });
            if let Ok(Some(exits)) = resolved {
                pending.insert(row.key, exits);
            }
        }
        let mut claims = OrdinaryNewResultClassClaimsV1::new();
        loop {
            let pending_keys: BTreeSet<CanonicalSameModuleCallableKeyV1> =
                pending.keys().cloned().collect();
            let mut inserts = Vec::new();
            let mut deads = Vec::new();
            for (key, exits) in &pending {
                match evaluate_row(exits, &claims, &pending_keys) {
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
            .into_iter()
            .filter(|(_, claim)| {
                ordinary_box_coverage
                    .row_for(claim.class())
                    .ok()
                    .flatten()
                    .is_some()
            })
            .collect()
    }
}
