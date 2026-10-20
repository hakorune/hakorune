//! Original owning Object LOCAL input demand, independent of result execution.
use super::call_witness::OriginalLocalCallSourceV1;
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::{AppMainSourceRefV1, LexicalInstanceCallSourceTargetV1};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct OwningObjectLocalInputSourceLoanV1 {
    source: OriginalLocalCallSourceV1,
    destination: BindingRefV1,
    class: Box<str>,
    nullable: bool,
    roots: Box<[Rc<ResultOriginWitnessV1>]>,
}
impl PartialEq for OwningObjectLocalInputSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.destination == other.destination
            && self.class == other.class && self.nullable == other.nullable && self.roots.len() == other.roots.len()
            && self.roots.iter().zip(&other.roots).all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
impl Eq for OwningObjectLocalInputSourceLoanV1 {}
impl OwningObjectLocalInputSourceLoanV1 {
    pub(crate) fn nullable(&self) -> bool { self.nullable }
    pub(crate) fn call(&self) -> &crate::mir::resolved_semantics::OwnedExprSiteV1 { &self.source.call }
    pub(crate) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 { &self.source.key }
    pub(crate) fn callee_owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 { self.source.callee_owner }
    pub(crate) fn receiver(&self) -> Option<&SourceExprSiteV1> { self.source.receiver.as_ref() }
    pub(crate) fn destination(&self) -> BindingRefV1 { self.destination }
    pub(crate) fn declaration(&self) -> &crate::mir::resolved_semantics::SourceBindingSiteV1 { &self.source.declaration }
    pub(crate) fn matches_arguments_v1(&self, sites: &[SourceExprSiteV1]) -> bool {
        self.source.actuals.len() == sites.len() && self.source.actuals.iter().zip(sites)
            .all(|(a, b)| a.site.owner() == self.call().owner() && a.site.site() == b)
    }
    pub(crate) fn matches_target(&self, target: &LexicalInstanceCallSourceTargetV1) -> bool {
        self.call() == target.call_site() && self.source.key == *target.target()
            && self.source.callee_owner == target.callee_owner()
            && self.source.receiver.as_ref() == Some(target.receiver_site())
            && self.matches_arguments_v1(target.argument_sites())
    }
}
impl OrdinaryNewResultClassClaimsV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn owning_object_local_input_source_v1(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        reference: &super::super::lexical_instance_call::CallTargetReferenceV1, destination: BindingRefV1,
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<Option<OwningObjectLocalInputSourceLoanV1>, &'static str> {
        let source = self.original_prepared_local_call_source_v1(batch, selected, reference,
            destination, parameters, app_main)?;
        let (class, nullable) = match self.get(&source.key) {
            Some(OrdinaryNewResultClassV1::Object(class)) => (class, false),
            Some(OrdinaryNewResultClassV1::NullableObject(class)) => (class, true),
            _ => return Ok(None),
        };
        let roots = self.checked_original_source_roots_v1(&source.key, source.callee_owner)?;
        if roots.is_empty() || roots.iter().any(|root| match root.origin() {
            ResultValueOriginV1::Null => !nullable,
            ResultValueOriginV1::Fresh(name) => name != class,
            _ => true,
        }) { return Err("nullable-input/original-roots"); }
        Ok(Some(OwningObjectLocalInputSourceLoanV1 {
            source, destination, class: class.clone(), nullable,
            roots: roots.into_iter().map(Rc::clone).collect(),
        }))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_owning_object_local_input_roots_v1(
        &self, loan: &OwningObjectLocalInputSourceLoanV1,
    ) -> Result<(), &'static str> {
        let expected = if loan.nullable { OrdinaryNewResultClassV1::NullableObject(loan.class.clone()) }
            else { OrdinaryNewResultClassV1::Object(loan.class.clone()) };
        if self.get(&loan.source.key) != Some(&expected) {
            return Err("nullable-input/result-class-drift");
        }
        let roots = self.checked_original_source_roots_v1(&loan.source.key, loan.source.callee_owner)?;
        if roots.len() != loan.roots.len() || roots.iter().zip(&loan.roots).any(|(a, b)| !Rc::ptr_eq(a, b)) {
            return Err("nullable-input/original-roots-drift");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owning_object_local_input_rejects_lost_replaced_and_reissued_roots() {
        for (caller, text) in [
            ("Maker", "box Token {} box Maker { make(size: i64, tag: i64) { if size <= 0 { return null } return new Token() }
                relay(size: i64) { local out = me.make(size, 7) return out } }
                static box Main { main() { return 0 } }"),
            ("Caller", "box Token {} box Maker { make(size: i64, tag: i64) { if size <= 0 { return null } return new Token() } }
                box Caller { relay(builder, size: i64) { local out = builder.make(size, 7) return out }
                    drive(size: i64) { local builder = new Maker() return me.relay(builder, size) } }
                static box Main { main() { return 0 } }"),
        ] {
            super::super::with_source_claims_fixture(text, |batch, selected, constructors, parameters, main, writers| {
                let (fields, _, facts, _) = super::super::super::coseal_issue::source_claims::prepare_source_claims(
                    batch, selected, main, constructors, parameters, std::rc::Rc::clone(writers), None,).unwrap();
                let key = CanonicalSameModuleCallableKeyV1::instance_box_method(caller, "relay", if caller == "Maker" { 1 } else { 2 });
                let slot = selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
                let (call, destination) = batch.with_lowering_input(slot, |input| {
                    let row = input.function().expression_source().initializers().next().unwrap();
                    (crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), row.initializer_site().unwrap().clone()), row.binding())
                }).unwrap();
                let names: Vec<Box<str>> = batch.ordinary_box_coverage().rows().iter()
                    .map(|row| row.name().into()).collect();
                let mut new_classes = BTreeMap::new();
                for row in batch.declarations() {
                    batch.with_lowering_input(row.batch_slot(), |input| {
                        for construction in input.function().expression_source().constructions() {
                            new_classes.insert(crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), construction.site().clone()),
                                construction.class().into());
                        }
                    }).unwrap();
                }
                use super::super::super::lexical_instance_call::{CallTargetReferenceV1, PreparedSourceCallNeedV1};
                let needs = super::super::super::lexical_instance_call::prepare_lexical_source_targets_v1(
                    batch, selected, &new_classes, &names, &fields, &facts, parameters, main.map(|main| main.caller_source_ref_v1()), &mut |_, _, _| Ok(None), None).unwrap();
                let target = needs.iter().filter_map(|row| row.as_ref().unwrap().as_ref()).find_map(|need| match need {
                    PreparedSourceCallNeedV1::Lexical(target) if target.call_site() == &call => Some(target),
                    _ => None,
                }).expect("canonical self or parameter receiver target");
                let reference = CallTargetReferenceV1::from_target(target);
                let loan = facts.owning_object_local_input_source_v1(batch, selected,
                    parameters, &reference, destination, None).unwrap().unwrap();
                assert!(loan.matches_target(target));
                for mutation in 0..6 {
                    let mut changed = reference.clone();
                    match mutation {
                        0 => changed.receiver_site = changed.argument_sites[0].clone(),
                        1 => changed.argument_sites.swap(0, 1),
                        2 => changed.argument_sites = Box::new([]),
                        3 => changed.target = CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "other", 2),
                        4 => changed.target_batch_slot = u32::MAX,
                        _ => changed.callee_owner = call.owner(),
                    }
                    assert!(facts.owning_object_local_input_source_v1(batch, selected,
                        parameters, &changed, destination, None).is_err(), "original reference mutation {mutation}");
                }
                facts.recheck_owning_object_local_input_roots_v1(&loan).unwrap();
                assert!(loan.roots.len() >= 2);
                for mutation in 0..5 {
                    let mut changed = loan.clone();
                    match mutation {
                        0 => changed.roots = Box::new([]),
                        1 => changed.roots[0] = changed.roots[1].clone(),
                        2 => {
                            let ordinal = changed.roots.iter().position(|root|
                                matches!(root.origin(), ResultValueOriginV1::Null)).unwrap();
                            changed.roots[ordinal] = Rc::new(ResultOriginWitnessV1 {
                                site: changed.roots[ordinal].site().clone(),
                                origin: ResultValueOriginV1::Null,
                                step: ResultWitnessStepV1::NullLiteral,
                            });
                        }
                        3 => changed.nullable = !changed.nullable,
                        _ => changed.class = "Other".into(),
                    }
                    assert!(facts.recheck_owning_object_local_input_roots_v1(&changed)
                        .is_err(), "original immutable root identity mutation {mutation}");
                }
            });
        }
    }
}
