//! Shared original field-use walk. No container kind, element type or Home is
//! inferred here; each semantic owner consumes the same exact use observations.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingKindV1, BindingRefV1, BodyExpressionShapeV1,
    BodyMeReceiverV1, CallableSemanticSourceLedgerView, ResolvedAssignmentTargetV1,
    ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1, VerifiedResolvedBodyShapeInventoryV1};
use std::collections::{BTreeMap, BTreeSet};

/// Per-field census accumulated across the owning Box's selected methods.
#[derive(Debug, Default)]
pub(super) struct FieldUseCensusV1 {
    /// `me.<field>` store `(method, value_site)` rows.
    pub(super) stores: Vec<(FieldUseMethodV1, SourceExprSiteV1)>,
    /// All original receiver calls. The container/result owner chooses semantics.
    pub(super) calls: Vec<(FieldUseMethodV1, std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>)>,
    pub(super) aliases: BTreeSet<BindingRefV1>,
    /// Same-spelled field through an unqualified receiver: not canonical evidence.
    pub(super) foreign: bool,
    pub(super) captured_receiver: bool,
    /// A foreign occurrence excluded the field.
    pub(super) vetoed: bool,
}

/// One method under the census: a selected batch slot, or the owning
/// Box's `birth` row reached through the constructor batch — `birth`
/// carries the stored-field-initializer prologue that hosts the sole
/// admitted provider store, and it is not always a selected key.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(super) enum FieldUseMethodV1 {
    Selected { slot: u32, is_birth: bool },
    BirthRow,
}

impl FieldUseMethodV1 {
    pub(super) const fn is_birth(&self) -> bool {
        match self {
            Self::Selected { is_birth, .. } => *is_birth,
            Self::BirthRow => true,
        }
    }
}

/// `true` when `object` resolves to this frame's own `me` receiver binding.
pub(crate) fn is_self_receiver(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    object: &SourceExprSiteV1,
) -> bool {
    matches!(
        shape.expression_shape(object),
        Some(BodyExpressionShapeV1::Me {
            receiver: BodyMeReceiverV1::Lexical(me),
            ..
        }) if ledger
            .binding(*me)
            .is_some_and(|record| record.kind() == BindingKindV1::Receiver)
    )
}

/// Resolves the tracked-field receiver for one call site: either
/// `me.<field>` directly, or a local alias whose sole initializer is
/// `me.<field>`.
fn field_receiver(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    aliases: &BTreeMap<BindingRefV1, Box<str>>,
    fields: &BTreeSet<Box<str>>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
) -> Option<Box<str>> {
    if let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(call.receiver_site())
    {
        if is_self_receiver(ledger, shape, object) && fields.contains(field.as_ref()) {
            return Some(field.clone());
        }
        return None;
    }
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        call.receiver()
    else {
        return None;
    };
    aliases.get(&binding).cloned()
}

/// `me.<field>` aliases for one method: bindings whose single initializer
/// is a `me.<field>` read on a tracked field, plus the bindings initialized
/// from bare `me` (any field use on those vetoes the touched field).
fn method_aliases(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    fields: &BTreeSet<Box<str>>,
    census: &mut BTreeMap<Box<str>, FieldUseCensusV1>,
) -> (
    BTreeMap<BindingRefV1, Box<str>>,
    BTreeSet<SourceExprSiteV1>,
    BTreeSet<BindingRefV1>,
) {
    let mut aliases: BTreeMap<BindingRefV1, Box<str>> = BTreeMap::new();
    let mut alias_sites: BTreeSet<SourceExprSiteV1> = BTreeSet::new();
    let mut duplicate: BTreeSet<BindingRefV1> = BTreeSet::new();
    let mut me_aliases: BTreeSet<BindingRefV1> = BTreeSet::new();
    for relation in ledger.initializer_relations() {
        let Some(site) = relation.initializer_site() else {
            continue;
        };
        match shape.expression_shape(site) {
            Some(BodyExpressionShapeV1::FieldAccess { object, field, .. })
                if is_self_receiver(ledger, shape, object) && fields.contains(field.as_ref()) =>
            {
                if aliases.insert(relation.binding(), field.clone()).is_some() {
                    duplicate.insert(relation.binding());
                }
                alias_sites.insert(site.clone());
            }
            Some(BodyExpressionShapeV1::Me { .. }) => {
                me_aliases.insert(relation.binding());
            }
            _ => {}
        }
    }
    for binding in duplicate {
        if let Some(field) = aliases.remove(&binding) {
            census.entry(field).or_default().vetoed = true;
        }
    }
    for source in shape.assignment_sources() {
        let Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) =
            ledger.assignment_target(source.target_site())
        else {
            continue;
        };
        if let Some(field) = aliases.remove(binding) {
            census.entry(field).or_default().vetoed = true;
        }
        if me_aliases.contains(binding) {
            for field in fields {
                census.entry(field.clone()).or_default().vetoed = true;
            }
        }
    }
    (aliases, alias_sites, me_aliases)
}

/// Census one selected method of the Box.
pub(super) fn capture_method_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    method: FieldUseMethodV1,
    fields: &BTreeSet<Box<str>>,
    census: &mut BTreeMap<Box<str>, FieldUseCensusV1>,
    owns_fields: bool,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
    let Some(shape) = input.body_shape() else {
        return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
    };
    if owns_fields && ledger.bindings().any(|(binding, record)| record.kind() == BindingKindV1::Receiver && !ledger.uncaptured_binding(binding)) {
        for field in fields { census.entry(field.clone()).or_default().captured_receiver = true; }
    }
    let (aliases, alias_sites, me_aliases) = method_aliases(&ledger, shape, fields, census);
    for (binding, field) in &aliases { census.entry(field.clone()).or_default().aliases.insert(*binding); }
    let mut store_sites: BTreeSet<SourceExprSiteV1> = BTreeSet::new();
    let mut receiver_sites: BTreeSet<SourceExprSiteV1> = BTreeSet::new();
    for source in shape.assignment_sources() {
        let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
            shape.expression_shape(source.target_site())
        else {
            continue;
        };
        if is_self_receiver(&ledger, shape, object) && fields.contains(field.as_ref()) {
            store_sites.insert(source.target_site().clone());
            census
                .entry(field.clone())
                .or_default()
                .stores
                .push((method, source.value_site().clone()));
        }
    }
    for (_, call) in ledger.method_calls() {
        let Some(field) = field_receiver(&ledger, shape, &aliases, fields, call) else {
            continue;
        };
        receiver_sites.insert(call.receiver_site().clone());
        let original = input.function().method_call_source_v1(call.site())
            .ok_or(OrdinaryNewCoSealIssueV1::BatchLoan)?;
        census.entry(field).or_default().calls.push((method, std::sync::Arc::clone(original)));
    }

    // Every `me.<field>` occurrence must be accounted for; every alias use
    // must be an allowed receiver; every `me`-alias field use vetoes.
    for expression in shape.expressions() {
        match expression {
            BodyExpressionShapeV1::FieldAccess {
                site, object, field,
            } => {
                if fields.contains(field.as_ref()) && (!owns_fields || !is_self_receiver(&ledger, shape, object)) {
                    census.entry(field.clone()).or_default().foreign = true;
                }
                if is_self_receiver(&ledger, shape, object)
                    && fields.contains(field.as_ref())
                    && !store_sites.contains(site)
                    && !receiver_sites.contains(site)
                    && !alias_sites.contains(site)
                {
                    census.entry(field.clone()).or_default().vetoed = true;
                }
                if let Some(BodyExpressionShapeV1::Variable { resolved, .. }) =
                    shape.expression_shape(object)
                {
                    if let ResolvedLexicalRefV1::Local(binding) = resolved {
                        if me_aliases.contains(binding) && fields.contains(field.as_ref()) {
                            census.entry(field.clone()).or_default().vetoed = true;
                        }
                    }
                }
            }
            BodyExpressionShapeV1::Variable { site, resolved } => {
                let ResolvedLexicalRefV1::Local(binding) = resolved else {
                    continue;
                };
                if let Some(field) = aliases.get(binding) {
                    if !receiver_sites.contains(site) {
                        census.entry(field.clone()).or_default().vetoed = true;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}


/// Borrow the lowering input for one census method: the batch loan for a
/// selected slot, or the constructor row's own sealed input for `birth`.
pub(super) fn with_method_input_v1<R>(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    birth: Option<&super::VerifiedInstanceConstructorSemanticRowV1>,
    source: &crate::ast::ASTNode,
    method: FieldUseMethodV1,
    f: impl FnOnce(ResolvedFunctionLoweringInputV1<'_>) -> R,
) -> Result<R, OrdinaryNewCoSealIssueV1> {
    match method {
        FieldUseMethodV1::Selected { slot, .. } => batch
            .with_lowering_input(slot, f)
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan),
        FieldUseMethodV1::BirthRow => {
            let row = birth.ok_or(OrdinaryNewCoSealIssueV1::BatchLoan)?;
            let input = row
                .lowering_input(source)
                .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
            Ok(f(input))
        }
    }
}
