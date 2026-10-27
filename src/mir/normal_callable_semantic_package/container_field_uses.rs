//! Shared original field-use and payload-class source owner. Observation and
//! class qualification grant no Home, slot lifetime or executable operation.
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
    for (binding, field) in &aliases {
        let row = census.entry(field.clone()).or_default();
        row.aliases.insert(*binding);
        if !ledger.uncaptured_binding(*binding) { row.captured_receiver = true; }
    }
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

/// Observe a field across the complete original batch and constructor-only
/// Birth. Consumers decide container kind, payload class and lifetime rules.
pub(super) fn capture_box_field_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    birth: Option<&super::VerifiedInstanceConstructorSemanticRowV1>,
    owner_class: &str,
    field: &str,
) -> Option<FieldUseCensusV1> {
    let tracked = BTreeSet::from([Box::<str>::from(field)]);
    let mut census = BTreeMap::from([(Box::<str>::from(field), FieldUseCensusV1::default())]);
    let birth_owner = birth?.lowering_input(batch.source_ast()).ok()?.owner();
    let mut birth_seen = false;
    for declaration in batch.declarations() {
        let own = matches!(selected.key_for_batch_slot(declaration.batch_slot()),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                if key.owner() == owner_class && matches!(key.namespace(),
                    hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                    | hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor));
        let is_birth = birth_owner == declaration.owner();
        birth_seen |= is_birth;
        batch.with_lowering_input(declaration.batch_slot(), |input| {
            let method = FieldUseMethodV1::Selected { slot: declaration.batch_slot(), is_birth };
            capture_method_v1(input, method, &tracked, &mut census, own).ok()
        }).ok()??;
    }
    if !birth_seen {
        let input = birth?.lowering_input(batch.source_ast()).ok()?;
        capture_method_v1(input, FieldUseMethodV1::BirthRow, &tracked, &mut census, true).ok()?;
    }
    census.remove(field)
}

/// Normal payload class only. Slot lifetime, nonnull and executable reads
/// remain with their existing acquisition and field-read owners.
#[derive(Debug)]
pub(crate) struct MapFieldPayloadSourceV1 {
    pub(super) source: std::rc::Rc<super::field_write_claim::DeclaredFieldValueSourceV1>,
    pub(super) class: Box<str>,
    pub(super) object: hakorune_mir_defs::CanonicalObjectIdV1,
    pub(super) uses: FieldUseCensusV1,
}
impl MapFieldPayloadSourceV1 {
    pub(crate) fn payload_class_v1(&self) -> &str { &self.class }
    pub(crate) fn payload_object_v1(&self) -> hakorune_mir_defs::CanonicalObjectIdV1 { self.object }
    pub(super) fn same_source_v1(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.source, &other.source) && self.class == other.class
            && self.object == other.object && self.uses.stores == other.uses.stores
            && self.uses.aliases == other.uses.aliases && self.uses.foreign == other.uses.foreign
            && self.uses.captured_receiver == other.uses.captured_receiver && self.uses.vetoed == other.uses.vetoed
            && self.uses.calls.len() == other.uses.calls.len()
            && self.uses.calls.iter().zip(&other.uses.calls).all(|((a, call), (b, original))|
                a == b && std::sync::Arc::ptr_eq(call, original))
    }
    pub(super) fn issue(
        batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
        selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        constructors: &super::VerifiedInstanceConstructorSemanticBatchV1,
        claims: &super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        owner_class: &str, field: &str,
    ) -> Option<Self> {
        let source = claims.value_source(owner_class, field)?.as_ref().ok()?;
        let writers: Vec<_> = source.writers().collect();
        let store = source.birth_store(constructors, 0).ok()??;
        if !source.same_inventory(claims) || source.declared_type_name().is_some()
            || writers.len() != 1 || !writers[0].in_birth()
            || store.mode() != Ok(crate::mir::instruction::FieldSetModeV1::DynamicMapInitialize)
            || !matches!(store.rhs(), crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1::ProviderConstruction {
                object: None, class, .. } if class.as_ref() == "MapBox") { return None; }
        let owner = batch.ordinary_box_coverage().row_for(owner_class).ok()??;
        let birth = constructors.birth_row_for(owner).ok()?;
        let uses = capture_box_field_v1(batch, selected, birth, owner_class, field)?;
        if uses.vetoed || uses.foreign || uses.captured_receiver || !uses.aliases.is_empty()
            || uses.stores.len() != 1 || !uses.stores[0].0.is_birth() { return None; }
        let mut payload = None;
        for (method, call) in &uses.calls {
            with_method_input_v1(batch, birth, batch.source_ast(), *method, |input| {
                match (call.selector(), call.arity()) {
                    ("has", 1) | ("get", 1) | ("delete", 1) | ("keys", 0) => {},
                    ("set", 2) => {
                        let construction = original_payload_new_v1(input, call.arguments()[1].site())?;
                        let declaration = batch.ordinary_box_coverage().row_for(construction.class()).ok()??;
                        let found = (Box::<str>::from(construction.class()), constructors.object_for(declaration).ok()?);
                        if payload.as_ref().is_some_and(|old| old != &found) { return None; }
                        payload = Some(found);
                    }
                    _ => return None,
                }
                Some(())
            }).ok()??;
        }
        let (class, object) = payload?;
        Some(Self { source: std::rc::Rc::clone(source), class, object, uses })
    }
    /// Exact original get in the shared whole field-use census.
    pub(super) fn original_get_at_v1(&self, at: &crate::mir::resolved_semantics::OwnedExprSiteV1)
        -> Option<&std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>> {
        let mut calls = self.uses.calls.iter().filter_map(|(_, call)|
            (call.owner() == at.owner() && call.site() == at.site()
                && call.selector() == "get" && call.arity() == 1).then_some(call));
        let call = calls.next()?;
        calls.next().is_none().then_some(call)
    }
    pub(super) fn original_get_v1(&self, call: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>) -> bool {
        self.uses.calls.iter().filter(|(_, original)| original.owner() == call.owner()
            && original.site() == call.site()).count() == 1
            && self.original_get_at_v1(&crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                call.owner(), call.site().clone()))
                .is_some_and(|original| std::sync::Arc::ptr_eq(original, call))
    }
}
pub(super) fn original_payload_new_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, value: &SourceExprSiteV1,
) -> Option<crate::mir::resolved_semantics::ResolvedNewExpressionSourceV1> {
    let function = input.function();
    let site = if function.expression_source().construction(value).is_some() { value.clone() }
        else {
            let ResolvedLexicalRefV1::Local(binding) = function.variable_ref(value)? else { return None };
            if !input.forest().callable_source_ledger(input.owner()).ok()?.uncaptured_binding(binding) { return None; }
            super::result_class_claim::sole_initializer_site(function, binding)?
        };
    function.expression_source().construction(&site).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_alias_capture_stays_visible_to_the_original_census() {
        for (returned, captured) in [("ids", true), ("1", false)] {
            let text = format!("box Page {{ init {{ values }} probe() {{ local ids = me.values
                local f = fn() {{ return {returned} }} ids.push(\"x\") return 0 }} }}
                static box Main {{ main() {{ return 0 }} }}");
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                &text, crate::parser::ParserBuildConfig::default()).unwrap();
            let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", ||
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap());
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
                panic!("original source");
            };
            let mut resolver = crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(71).unwrap();
            let batch = crate::mir::callable_semantic_batch::issue_resolved_callable_semantic_batch_v1(
                &mut resolver, source).unwrap();
            let mut checked = 0;
            for declaration in batch.declarations() {
                batch.with_lowering_input(declaration.batch_slot(), |input| {
                    if !input.function().method_calls().any(|(_, call)| call.selector() == "push") { return; }
                    let fields = BTreeSet::from([Box::<str>::from("values")]);
                    let mut census = BTreeMap::new();
                    capture_method_v1(input, FieldUseMethodV1::Selected { slot: declaration.batch_slot(), is_birth: false },
                        &fields, &mut census, true).unwrap();
                    assert_eq!(census["values"].captured_receiver, captured, "return {returned}");
                    checked += 1;
                }).unwrap();
            }
            assert_eq!(checked, 1);
        }
    }
}
