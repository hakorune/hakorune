//! Passive returned-object field provenance. No field read, Home or call executes.
use super::*;
use super::call_witness::OriginalLocalCallSourceV1;
use super::child_relation::SourceConstructionChildRelationV1;
use super::super::lexical_instance_call::LexicalInstanceCallSourceTargetV1;
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use std::rc::Rc;

#[derive(Debug, Clone)]
struct FieldHopV1 {
    read: OwnedExprSiteV1,
    receiver: crate::mir::resolved_semantics::SourceExprSiteV1,
    local: OriginalLocalCallSourceV1,
    call: ObjectCallSourceLoanV1,
    destination: BindingRefV1,
    field: Option<CanonicalFieldRefV1>,
}
impl PartialEq for FieldHopV1 {
    fn eq(&self, other: &Self) -> bool {
        self.read == other.read && self.receiver == other.receiver && self.local == other.local && self.destination == other.destination
            && self.field == other.field && self.call.source() == other.call.source()
            && self.call.result_class() == other.call.result_class()
            && same_roots(self.call.roots(), other.call.roots())
    }
}
impl Eq for FieldHopV1 {}

#[derive(Debug, Clone)]
struct FieldAlternativeV1 {
    hop: usize,
    wrapper: Rc<ResultOriginWitnessV1>,
    relation: Rc<SourceConstructionChildRelationV1>,
    child: Rc<ResultOriginWitnessV1>,
}
impl PartialEq for FieldAlternativeV1 {
    fn eq(&self, other: &Self) -> bool {
        self.hop == other.hop && Rc::ptr_eq(&self.wrapper, &other.wrapper)
            && Rc::ptr_eq(&self.relation, &other.relation) && Rc::ptr_eq(&self.child, &other.child)
    }
}
impl Eq for FieldAlternativeV1 {}

/// Immutable call-site substitution, retaining every original alternative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReturnedFieldSourceLoanV1 {
    read: OwnedExprSiteV1,
    field: CanonicalFieldRefV1,
    child: CanonicalObjectIdV1,
    hops: Box<[FieldHopV1]>,
    alternatives: Box<[FieldAlternativeV1]>,
    leaves: Box<[(OwnedExprSiteV1, ResultValueOriginV1)]>,
    class: Option<Box<str>>,
    nullable: bool,
}
impl ReturnedFieldSourceLoanV1 {
    pub(crate) fn read(&self) -> &OwnedExprSiteV1 { &self.read }
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(crate) fn child(&self) -> CanonicalObjectIdV1 { self.child }
    pub(crate) fn class(&self) -> Option<&str> { self.class.as_deref() }
    pub(crate) fn nullable(&self) -> bool { self.nullable }
    pub(crate) fn parent(&self) -> BindingRefV1 { self.hops[0].destination }
    pub(crate) fn acquisition(&self) -> &OwnedExprSiteV1 { &self.hops[0].local.call }
    pub(crate) fn receiver_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 { &self.hops[0].receiver }

    /// Borrow the original LOCAL acquisitions, never a reconstructed receipt.
    pub(in crate::mir::normal_callable_semantic_package) fn acquisition_sources_v1(
        &self,
    ) -> impl Iterator<Item = (&ObjectCallSourceLoanV1, BindingRefV1)> {
        self.hops.iter().map(|hop| (&hop.call, hop.destination))
    }

    pub(crate) fn recheck_writers_v1(
        &self, fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    ) -> Result<(), &'static str> {
        for alternative in &self.alternatives {
            let wrapper = self.hops.get(alternative.hop).ok_or("returned-field/hop-missing")?;
            let writer = alternative.relation.provided_field_source_v1(wrapper.call.class())?;
            fields.recheck_original_write_source_v1(writer)?;
        }
        Ok(())
    }
}
fn same_roots(a: &[Rc<ResultOriginWitnessV1>], b: &[Rc<ResultOriginWitnessV1>]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| Rc::ptr_eq(a, b))
}

impl OrdinaryNewResultClassClaimsV1 {
    /// The immutable capsule keeps source navigation/substitution. Current
    /// owners must still lend every original target, root, relation and writer.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_returned_field_graph_v1<'a>(
        &self, source: &ReturnedFieldSourceLoanV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        lookup: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<&'a LexicalInstanceCallSourceTargetV1>, &'static str>,
    ) -> Result<(), &'static str> {
        self.recheck_returned_field_graph_using_v1(source, fields, &mut |site, loan| {
            let target = lookup(site)?.ok_or("returned-field/current-target-missing")?;
            self.recheck_object_call_roots_v1(target, loan)
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_returned_field_graph_using_v1(
        &self, source: &ReturnedFieldSourceLoanV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        check_target: &mut impl FnMut(&OwnedExprSiteV1, &ObjectCallSourceLoanV1) -> Result<(), &'static str>,
    ) -> Result<(), &'static str> {
        source.recheck_writers_v1(fields)?;
        for (index, hop) in source.hops.iter().enumerate() {
            check_target(&hop.local.call, &hop.call)?;
            let field = hop.field.ok_or("returned-field/current-field-missing")?;
            let mut retained = source.alternatives.iter().filter(|row| row.hop == index);
            for root in hop.call.roots() {
                let mut relations = self.construction_child_relation_loans()
                    .filter(|row| row.outer() == root.site() && row.field() == field);
                let relation = relations.next().ok_or("returned-field/current-relation-missing")?;
                if relations.next().is_some() { return Err("returned-field/current-relation-duplicate"); }
                for child in relation.witnesses() {
                    let original = retained.next().ok_or("returned-field/current-alternative-extra")?;
                    if !Rc::ptr_eq(&original.wrapper, root) || !Rc::ptr_eq(&original.relation, relation)
                        || !Rc::ptr_eq(&original.child, child) {
                        return Err("returned-field/current-alternative-drift");
                    }
                }
            }
            if retained.next().is_some() { return Err("returned-field/current-alternative-missing"); }
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_returned_field_source_v1<'a>(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        source: &ReturnedFieldSourceLoanV1,
        lookup: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<&'a LexicalInstanceCallSourceTargetV1>, &'static str>,
    ) -> Result<(), &'static str> {
        source.recheck_writers_v1(fields)?;
        match self.returned_field_source_v1(batch, selected, constructors, source.read(), lookup)? {
            Some(rebuilt) if &rebuilt == source => Ok(()),
            _ => Err("returned-field/retained-source-drift"),
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn returned_field_source_v1<'a>(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        read: &OwnedExprSiteV1,
        lookup: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<&'a LexicalInstanceCallSourceTargetV1>, &'static str>,
    ) -> Result<Option<ReturnedFieldSourceLoanV1>, &'static str> {
        // Other source owners retain their own original classifier protocol.
        if !batch.declarations().any(|row| row.owner() == read.owner()) { return Ok(None); }
        let mut walk = FieldWalkV1 { facts: self, batch, selected, constructors, lookup,
            active: BTreeSet::new(), hops: Vec::new(), alternatives: Vec::new(), leaves: Vec::new() };
        let Some((field, child)) = walk.field(read)? else { return Ok(None); };
        let mut class: Option<Box<str>> = None;
        let mut nullable = false;
        for (_, origin) in &walk.leaves {
            match origin {
                ResultValueOriginV1::Null => nullable = true,
                ResultValueOriginV1::Fresh(name) => {
                    if class.as_ref().is_some_and(|old| old != name) { return Err("returned-field/leaf-class-drift"); }
                    class = Some(name.clone());
                }
                _ => return Err("returned-field/unclosed-formal"),
            }
        }
        if walk.leaves.is_empty() { return Err("returned-field/empty-leaves"); }
        Ok(Some(ReturnedFieldSourceLoanV1 { read: read.clone(), field, child,
            hops: walk.hops.into(), alternatives: walk.alternatives.into(),
            leaves: walk.leaves.into(), class, nullable }))
    }
}

struct FieldWalkV1<'a, 't> {
    facts: &'a OrdinaryNewResultClassClaimsV1,
    batch: &'a VerifiedResolvedCallableSemanticBatchV1,
    selected: &'a crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    lookup: &'a mut dyn FnMut(&OwnedExprSiteV1) -> Result<Option<&'t LexicalInstanceCallSourceTargetV1>, &'static str>,
    active: BTreeSet<(OwnedExprSiteV1, CanonicalFieldRefV1)>,
    hops: Vec<FieldHopV1>,
    alternatives: Vec<FieldAlternativeV1>,
    leaves: Vec<(OwnedExprSiteV1, ResultValueOriginV1)>,
}
impl<'a, 't> FieldWalkV1<'a, 't> {
    fn slot(&self, site: &OwnedExprSiteV1) -> Result<u32, &'static str> {
        let mut rows = self.batch.declarations().filter(|row| row.owner() == site.owner());
        let row = rows.next().ok_or("returned-field/source-owner")?;
        if rows.next().is_some() { return Err("returned-field/source-owner-duplicate"); }
        Ok(row.batch_slot())
    }
    fn field(&mut self, read: &OwnedExprSiteV1) -> Result<Option<(CanonicalFieldRefV1, CanonicalObjectIdV1)>, &'static str> {
        let parts = self.batch.with_lowering_input(self.slot(read)?, |input| {
            let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
                input.body_shape().and_then(|shape| shape.expression_shape(read.site())) else { return Ok(None); };
            let Some(ResolvedLexicalRefV1::Local(parent)) = input.function().variable_ref(object)
                else { return Ok(None); };
            if !matches!(input.function().binding(parent).map(|row| row.kind()), Some(BindingKindV1::Local { .. })) {
                return Ok(None);
            }
            let mut initializers = input.function().expression_source().initializers().filter(|row| row.binding() == parent);
            let Some(initializer) = initializers.next() else { return Ok(None); };
            if initializers.next().is_some() { return Err("returned-field/parent-initializer-duplicate"); }
            let Some(site) = initializer.initializer_site() else { return Ok(None); };
            Ok(Some((parent, object.clone(), field.clone(), OwnedExprSiteV1::new(read.owner(), site.clone()))))
        }).map_err(|_| "returned-field/source-input")??;
        let Some((parent, receiver, name, acquisition)) = parts else { return Ok(None); };
        let Some(target) = (self.lookup)(&acquisition)? else { return Ok(None); };
        let Some(OrdinaryNewResultClassV1::Object(wrapper)) = self.facts.get(target.target()) else { return Ok(None); };
        let wrapper_source = self.batch.ordinary_box_coverage().row_for(wrapper)
            .map_err(|_| "returned-field/wrapper-class")?.ok_or("returned-field/wrapper-class")?;
        let wrapper_object = self.constructors.object_for(&wrapper_source).map_err(|_| "returned-field/wrapper-object")?;
        if !self.facts.construction_child_relation_loans().any(|row|
            row.field().object() == wrapper_object && row.original_field_name_v1() == Ok(name.as_ref())) {
            return Ok(None);
        }
        let call = self.facts.object_call_source_v1(self.batch, self.selected, target)?;
        let local = self.facts.original_local_object_target_source_v1(self.batch, self.selected, target, parent, &call)?;
        let mut chosen = None;
        let mut selected = Vec::new();
        for root in call.roots() {
            if !matches!(root.step(), ResultWitnessStepV1::FreshConstruction) { return Err("returned-field/wrapper-forward-root"); }
            let mut rows = self.facts.construction_child_relation_loans().filter(|row|
                row.outer() == root.site() && row.original_field_name_v1() == Ok(name.as_ref()));
            let Some(relation) = rows.next() else { return Err("returned-field/field-alternative-missing"); };
            if rows.next().is_some() { return Err("returned-field/field-relation-duplicate"); }
            relation.provided_field_source_v1(wrapper)?;
            let mut births = self.constructors.rows().iter().filter(|row|
                row.source_id().same_as(relation.constructor_source()));
            let birth = births.next().ok_or("returned-field/constructor-source-missing")?;
            if births.next().is_some() || birth.forest().roots() != [relation.constructor_owner()] {
                return Err("returned-field/constructor-source-drift");
            }
            let coordinates = (relation.field(), relation.child());
            if chosen.is_some_and(|old| old != coordinates) { return Err("returned-field/field-relation-drift"); }
            chosen = Some(coordinates);
            selected.push((root.clone(), relation.clone()));
        }
        let (field, child) = chosen.ok_or("returned-field/wrapper-outcomes-empty")?;
        if !self.active.insert((acquisition.clone(), field)) { return Err("returned-field/initializer-cycle"); }
        let hop = self.hops.len();
        self.hops.push(FieldHopV1 { read: read.clone(), receiver, local: local.clone(), call, destination: parent, field: Some(field) });
        for (wrapper, relation) in selected {
            for witness in relation.witnesses() {
                self.alternatives.push(FieldAlternativeV1 { hop, wrapper: wrapper.clone(), relation: relation.clone(), child: witness.clone() });
                match witness.origin() {
                    ResultValueOriginV1::ForwardFormal { ordinal } => {
                        let actual = local.actuals.get(*ordinal as usize).ok_or("returned-field/formal-ordinal")?;
                        self.actual(&actual.site, child)?;
                    }
                    origin => self.leaf(witness.site(), origin, child)?,
                }
            }
        }
        self.active.remove(&(acquisition, field));
        Ok(Some((field, child)))
    }
    fn leaf(&mut self, site: &OwnedExprSiteV1, origin: &ResultValueOriginV1, expected: CanonicalObjectIdV1) -> Result<(), &'static str> {
        if let ResultValueOriginV1::Fresh(class) = origin {
            let source = self.batch.ordinary_box_coverage().row_for(class)
                .map_err(|_| "returned-field/child-class")?.ok_or("returned-field/child-class")?;
            if self.constructors.object_for(&source).map_err(|_| "returned-field/child-object")? != expected {
                return Err("returned-field/actual-class-drift");
            }
        } else if !matches!(origin, ResultValueOriginV1::Null) { return Err("returned-field/unclosed-actual"); }
        self.leaves.push((site.clone(), origin.clone()));
        Ok(())
    }
    fn actual(&mut self, site: &OwnedExprSiteV1, expected: CanonicalObjectIdV1) -> Result<(), &'static str> {
        let slot = self.slot(site)?;
        let null = self.batch.with_lowering_input(slot, |input|
            matches!(input.function().expression_source().literal(site.site()), Some(ResolvedLiteralSourceV1::Null)))
            .map_err(|_| "returned-field/actual-input")?;
        if null { return self.leaf(site, &ResultValueOriginV1::Null, expected); }
        if let Some((_, child)) = self.field(site)? {
            return if child == expected { Ok(()) } else { Err("returned-field/substitution-child-drift") };
        }
        Err("returned-field/actual-source-unavailable")
    }
}
