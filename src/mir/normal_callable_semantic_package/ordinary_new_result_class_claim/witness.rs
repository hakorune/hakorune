//! Immutable source derivations shared by the existing result solver.
//! This graph records source identity; it issues no acquired Home or runtime tag.
use super::ResultValueOriginV1;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ResultFormalSubstitutionV1 {
    pub(crate) argument_site: OwnedExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) callee_ordinal: u32,
    pub(crate) caller_ordinal: u32,
}

#[derive(Debug)]
pub(crate) enum ResultWitnessStepV1 {
    NullLiteral,
    FreshConstruction,
    MapStored { source: Rc<MapStoredResultSourceV1> },
    Formal {
        binding: BindingRefV1,
        ordinal: u32,
    },
    Call {
        site: OwnedExprSiteV1,
        key: CanonicalSameModuleCallableKeyV1,
        callee: Rc<ResultOriginWitnessV1>,
        substitution: Option<ResultFormalSubstitutionV1>,
    },
}

#[derive(Debug)]
pub(crate) struct ResultOriginWitnessV1 {
    pub(super) site: OwnedExprSiteV1,
    pub(super) origin: ResultValueOriginV1,
    pub(super) step: ResultWitnessStepV1,
}
impl ResultOriginWitnessV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn origin(&self) -> &ResultValueOriginV1 {
        &self.origin
    }
    pub(crate) fn step(&self) -> &ResultWitnessStepV1 {
        &self.step
    }
    pub(super) fn formal_ordinal(&self) -> Option<u32> {
        match &self.step {
            ResultWitnessStepV1::Formal { ordinal, .. } => Some(*ordinal),
            ResultWitnessStepV1::Call { substitution, .. } => {
                substitution.as_ref().map(|row| row.caller_ordinal)
            }
            _ => None,
        }
    }
}

/// Passive same-field result derivation. This retains original get/has/set and
/// New evidence; it grants no completed Call, committed Home or physical borrow.
#[derive(Debug)]
pub(crate) struct MapStoredResultSourceV1 {
    pub(super) receiver: Rc<crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1>,
    pub(super) key: BindingRefV1,
    pub(super) ordinal: u32,
    pub(super) class: Box<str>,
    pub(super) object: hakorune_mir_defs::CanonicalObjectIdV1,
    pub(super) returns: std::collections::BTreeMap<OwnedExprSiteV1, MapStoredReturnV1>,
    pub(super) uses: super::super::container_field_uses::FieldUseCensusV1,
}
impl PartialEq for MapStoredResultSourceV1 {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self, other) }
}
impl Eq for MapStoredResultSourceV1 {}

#[derive(Debug)]
pub(super) enum MapStoredReturnV1 {
    Present { get: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
        has: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1> },
    InstalledLocal { binding: BindingRefV1,
        construction: crate::mir::resolved_semantics::ResolvedNewExpressionSourceV1,
        set: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
        intervening: Box<[DisjointFieldAppendSourceV1]> },
}
#[derive(Debug)]
pub(super) struct DisjointFieldAppendSourceV1 {
    source: Rc<super::super::field_write_claim::DeclaredFieldValueSourceV1>,
    call: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
}
impl MapStoredResultSourceV1 {
    /// Logical support from the original slot; this never issues a Call ABI.
    pub(crate) fn result_support_v1(&self) -> crate::mir::instruction::BorrowedResultSupportV1 {
        crate::mir::instruction::BorrowedResultSupportV1::ReceiverFieldSlot {
            field: self.receiver.field(), key_ordinal: self.ordinal, object: self.object,
        }
    }
    pub(crate) fn key_binding_v1(&self) -> BindingRefV1 { self.key }
    pub(crate) fn corroborates_roots_v1(&self, owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        roots: &[Rc<ResultOriginWitnessV1>],
    ) -> bool {
        let sites: std::collections::BTreeSet<_> = roots.iter().map(|root| root.site()).collect();
        self.receiver.receiver().owner() == owner && self.key.owner() == owner
            && !roots.is_empty() && roots.len() == self.returns.len() && sites == self.returns.keys().collect()
            && roots.iter().all(|root| root.site().owner() == owner && root.origin() == &self.origin()
                && matches!(root.step(), ResultWitnessStepV1::MapStored { source } if std::ptr::eq(source.as_ref(), self)))
    }
    /// SAME original get; the source census also retains its guarding has.
    pub(crate) fn present_get_v1(&self, value: &OwnedExprSiteV1)
        -> Option<&std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>> {
        match self.returns.get(value)? {
            MapStoredReturnV1::Present { get, .. } => Some(get),
            MapStoredReturnV1::InstalledLocal { .. } => None,
        }
    }
    /// Exact retained miss-path source, selected by its original set or Return.
    pub(crate) fn installed_local_v1(&self, at: &OwnedExprSiteV1)
        -> Option<(BindingRefV1, &crate::mir::resolved_semantics::ResolvedNewExpressionSourceV1,
            &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>)> {
        let mut rows = self.returns.iter().filter_map(|(value, row)| match row {
            MapStoredReturnV1::InstalledLocal { binding, construction, set, .. }
                if value == at || set.owner() == at.owner() && set.site() == at.site() => Some((*binding, construction, set)),
            _ => None,
        });
        let found = rows.next()?;
        rows.next().is_none().then_some(found)
    }
    pub(crate) fn receiver_v1(&self)
        -> &crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1 {
        &self.receiver
    }
    pub(super) fn origin(&self) -> ResultValueOriginV1 {
        ResultValueOriginV1::MapHeld { field: self.receiver.field(), key: self.key,
            object: self.object, class: self.class.clone() }
    }
}

/// Resolve one producer's original Return census after the SAME field claims,
/// before SUM/Completion or any executable result selection. A failed selected
/// Map shape stays unavailable rather than borrowing generic owning semantics.
pub(super) fn map_stored_result_source_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    row: &super::ResultClassDraftRowV1,
    batch: &super::VerifiedResolvedCallableSemanticBatchV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    entries: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
) -> Option<Rc<MapStoredResultSourceV1>> {
    use crate::mir::resolved_semantics::{BodyExpressionShapeV1 as Shape, ResolvedLexicalRefV1 as Ref,
        BindingKindV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1 as Part};
    use super::super::container_field_uses::{FieldUseCensusV1, FieldUseMethodV1, capture_method_v1, with_method_input_v1};
    let shape = input.body_shape()?;
    let (get_site, _) = row.exits.iter().find(|(_, exit)| matches!(exit,
        super::ResultClassExitDraftV1::ForwardCall(site)
            if input.function().method_call(site).is_some_and(|call| call.selector() == "get" && call.arity() == 1)))?;
    let first = input.function().method_call_source_v1(get_site.site())?;
    let Shape::FieldAccess { field: name, .. } = shape.expression_shape(first.receiver_site())? else { return None };
    let entry = entries.for_batch_slot(row.batch_slot)?;
    let receiver_proof = super::super::entry_receiver_box_proof(selected, batch, Some(entry), row.batch_slot)?;
    let receiver = Rc::new(crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1::issue(
        input, &row.key, first, constructors, fields, receiver_proof, entry).ok()?);
    let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
    let symbolic_key = |call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1| {
        let site = call.arguments().first()?.site();
        let Ref::Local(binding) = input.function().variable_ref(site)? else { return None };
        let BindingKindV1::Parameter { index } = input.function().binding(binding)?.kind() else { return None };
        (super::binding_is_unrebound(input.function(), binding) && ledger.uncaptured_binding(binding))
            .then_some((binding, index))
    };
    let (key, ordinal) = symbolic_key(first)?;
    let tracked = std::collections::BTreeSet::from([name.clone()]);
    let mut census = std::collections::BTreeMap::from([(name.clone(), FieldUseCensusV1::default())]);
    let birth = constructors.birth_row_for(receiver_proof.1).ok()?;
    let mut birth_seen = false;
    let birth_owner = birth?.lowering_input(batch.source_ast()).ok()?.owner();
    for declaration in batch.declarations() {
        let own = matches!(selected.key_for_batch_slot(declaration.batch_slot()),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                if key.owner() == row.key.owner() && matches!(key.namespace(),
                    hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                    | hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor));
        let is_birth = birth_owner == declaration.owner();
        birth_seen |= is_birth;
        batch.with_lowering_input(declaration.batch_slot(), |method_input| {
            let method = FieldUseMethodV1::Selected { slot: declaration.batch_slot(), is_birth };
            capture_method_v1(method_input, method, &tracked, &mut census, own).ok()?;
            Some(())
        }).ok()??;
    }
    if !birth_seen {
        let birth = birth?;
        let birth_input = birth.lowering_input(batch.source_ast()).ok()?;
        capture_method_v1(birth_input, FieldUseMethodV1::BirthRow, &tracked, &mut census, true).ok()?;
    }
    let uses = census.remove(name)?;
    if uses.vetoed || uses.foreign || uses.captured_receiver || !uses.aliases.is_empty() || uses.stores.len() != 1
        || !uses.stores[0].0.is_birth() { return None; }
    // Validate the final WHOLE census, including constructor-only Birth.
    let mut payload: Option<(Box<str>, hakorune_mir_defs::CanonicalObjectIdV1)> = None;
    for (method, call) in &uses.calls {
        with_method_input_v1(batch, birth, batch.source_ast(), *method, |method_input| {
            match (call.selector(), call.arity()) {
                ("has", 1) | ("get", 1) => {},
                ("set", 2) => {
                    let construction = original_payload_new_v1(method_input, call.arguments()[1].site())?;
                    let source = batch.ordinary_box_coverage().row_for(construction.class()).ok()??;
                    let found = (Box::<str>::from(construction.class()), constructors.object_for(source).ok()?);
                    if payload.as_ref().is_some_and(|old| old != &found) { return None; }
                    payload = Some(found);
                }
                _ => return None,
            }
            Some(())
        }).ok()??;
    }
    let (class, object) = payload?;
    let calls: Vec<_> = uses.calls.iter().filter(|(_, call)| call.owner() == input.owner())
        .map(|(_, call)| call).collect();
    let mut returns = std::collections::BTreeMap::new();
    for (value, exit) in &row.exits {
        let proof = match exit {
            super::ResultClassExitDraftV1::ForwardCall(site) => {
                let get = calls.iter().find(|call| call.site() == site && call.selector() == "get" && call.arity() == 1)?;
                if symbolic_key(get) != Some((key, ordinal)) { return None; }
                // The direct first statement of the true arm has no intervening
                // Map mutation. The exact same has(receiver,key) is retained.
                let segments = value.site().node().segments();
                let i = segments.iter().position(|part| matches!(part, Part::IfThen(0)))?;
                if segments.get(i + 1) != Some(&Part::Value) || i + 2 != segments.len() { return None; }
                let mut condition = segments[..i].to_vec(); condition.push(Part::IfCondition);
                let condition = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(condition));
                let has = calls.iter().find(|call| call.site() == &condition && call.selector() == "has" && call.arity() == 1)?;
                if symbolic_key(has) != Some((key, ordinal)) { return None; }
                MapStoredReturnV1::Present { get: std::sync::Arc::clone(get), has: std::sync::Arc::clone(has) }
            }
            super::ResultClassExitDraftV1::ForwardLocal(binding) => {
                if !ledger.uncaptured_binding(*binding) { return None; }
                let init = super::sole_initializer_site(input.function(), *binding)?;
                let construction = original_payload_new_v1(input, &init)?;
                if construction.class() != class.as_ref() { return None; }
                let [Part::Body(return_index), Part::Value] = value.site().node().segments() else { return None };
                let [Part::Body(init_index), Part::Initializer(_)] = init.node().segments() else { return None };
                let mut stores = calls.iter().filter(|call| call.selector() == "set" && call.arity() == 2
                    && symbolic_key(call) == Some((key, ordinal))
                    && input.function().variable_ref(call.arguments()[1].site()) == Some(Ref::Local(*binding)));
                let set = stores.next()?;
                if stores.next().is_some() { return None; }
                let [Part::Body(set_index)] = set.site().node().segments() else { return None };
                if !(*init_index < *set_index && *set_index < *return_index) { return None; }
                // No later mutation may replace the support for this alias.
                if calls.iter().any(|call| call.selector() == "set" && call.site() != set.site()
                    && match call.site().node().segments().first() {
                        Some(Part::Body(index)) => *set_index <= *index && *index <= *return_index,
                        _ => true,
                    }) { return None; }
                let in_interval = |site: &crate::mir::resolved_semantics::SourceExprSiteV1| matches!(site.node().segments().first(),
                    Some(Part::Body(index)) if *set_index < *index && *index < *return_index);
                if ledger.direct_call_observations().any(|(site, _)| in_interval(site)) { return None; }
                // Construction or opaque expression effects inside the interval
                // are not erased by the field-only call census.
                if input.function().expression_source().constructions().any(|new| in_interval(new.site())) { return None; }
                let mut intervening = Vec::new();
                for (_, call) in ledger.method_calls().filter(|(_, call)| in_interval(call.site())) {
                    let Shape::FieldAccess { object, field, .. } = shape.expression_shape(call.receiver_site())? else { return None };
                    if input.function().variable_ref(object) != Some(Ref::Local(receiver.receiver()))
                        || call.selector() != "push" || call.arity() != 1 { return None; }
                    let source = fields.value_source(row.key.owner(), field)?.as_ref().ok()?;
                    if source.field() == receiver.field() || !source.same_inventory(fields) || source.writers().count() != 1 { return None; }
                    let store = source.birth_store(constructors, 0).ok()??;
                    if !matches!(store.rhs(), crate::mir::normal_callable_semantic_package::ConstructionStoreRhsV1::ProviderConstruction {
                        class, object: None, .. } if class.as_ref() == "ArrayBox") { return None; }
                    let original = input.function().method_call_source_v1(call.site())?;
                    intervening.push(DisjointFieldAppendSourceV1 { source: Rc::clone(source), call: std::sync::Arc::clone(original) });
                }
                if shape.effects().iter().filter(|effect| in_interval(&effect.site)).any(|effect|
                    effect.kind != crate::mir::resolved_semantics::BodyEffectKindV1::Call
                        || !intervening.iter().any(|proof| proof.call.site() == &effect.site)) { return None; }
                MapStoredReturnV1::InstalledLocal { binding: *binding, construction, set: std::sync::Arc::clone(set), intervening: intervening.into() }
            }
            _ => return None,
        };
        if returns.insert(value.clone(), proof).is_some() { return None; }
    }
    Some(Rc::new(MapStoredResultSourceV1 { receiver, key, ordinal, class, object, returns, uses }))
}

fn original_payload_new_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    value: &crate::mir::resolved_semantics::SourceExprSiteV1,
) -> Option<crate::mir::resolved_semantics::ResolvedNewExpressionSourceV1> {
    let function = input.function();
    let site = if function.expression_source().construction(value).is_some() { value.clone() }
        else {
            let crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) = function.variable_ref(value)? else { return None };
            if !input.forest().callable_source_ledger(input.owner()).ok()?.uncaptured_binding(binding) { return None; }
            super::sole_initializer_site(function, binding)?
        };
    function.expression_source().construction(&site).cloned()
}

impl super::OrdinaryNewResultClassClaimsV1 {
    /// Lend the SAME whole original Return derivation, without Completion,
    /// admission or a Home. Every exit must retain this allocation and origin.
    pub(crate) fn map_stored_source_v1(&self, value: &OwnedExprSiteV1)
        -> Result<Option<Rc<MapStoredResultSourceV1>>, &'static str> {
        let mut rows = self.source_rows().filter(|(_, exits)| exits.iter().any(|exit|
            exit.site() == value || exit.witnesses().iter().any(|root|
                matches!(root.step(), ResultWitnessStepV1::MapStored { source }
                    if source.installed_local_v1(value).is_some()))));
        let Some((key, _)) = rows.next() else { return Ok(None) };
        if rows.next().is_some() { return Err("callee-facts-identity"); }
        let roots = self.checked_original_source_roots_v1(key, value.owner())?;
        let Some(first) = roots.first() else { return Ok(None) };
        let ResultWitnessStepV1::MapStored { source } = first.step() else { return Ok(None) };
        if !source.corroborates_roots_v1(value.owner(), &roots.into_iter().cloned().collect::<Vec<_>>()) {
            return Err("callee-facts-identity");
        }
        Ok(Some(Rc::clone(source)))
    }
}
