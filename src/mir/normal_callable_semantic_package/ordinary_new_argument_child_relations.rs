//! Original child-relation loans ride the existing New argument rows.
//! This join issues neither Home acquisition nor physical field permission.
use super::*;
use crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreRhsV1;
use crate::mir::normal_callable_semantic_package::OrdinaryNewConstructorDispositionV1;
#[cfg(test)]
use crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentV1;
use std::rc::Rc;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::coseal_issue) fn install_result_classes_with_argument_loans_v1(
    ledger: &mut OrdinaryNewClaimLedgerV1,
    facts: result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    for claim in ledger.claims.get_mut().values_mut() {
        retain_argument_loans(&mut claim.core, &facts)?;
    }
    for claim in ledger.result_claims.get_mut().values_mut() {
        retain_argument_loans(&mut claim.core, &facts)?;
    }
    ledger.callable_result_classes = facts;
    Ok(())
}

fn retain_argument_loans(
    core: &mut OrdinaryNewClaimCoreV1,
    facts: &result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    let relations: Vec<_> = facts
        .construction_child_relation_loans()
        .filter(|relation| relation.outer() == &core.site)
        .collect();
    if relations.is_empty() {
        return Ok(());
    }
    // Passive child identity can ride an original New actual before its
    // physical construction plan is executable. Dynamic fields are sealed
    // against the retained source structure; typed fields still require the
    // executable plan below.
    let Ok(arguments) = &mut core.argument_rows else {
        return Ok(());
    };
    let mismatch = || OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
        site: core.site.clone(),
        class: core.class.clone(),
        arity: core.arity,
    };
    let OrdinaryNewConstructorDispositionV1::Birth(birth) = &core.constructor else {
        return Err(mismatch());
    };
    let mut retained_relations = Vec::new();
    for relation in &relations {
        if relation.field().object() != core.object
            || relation.formal_ordinal() as usize >= core.arity
        {
            return Err(mismatch());
        }
        if relation.is_unqualified_dynamic_v1() {
            let source = core.construction_source.as_ref().map_err(|_| mismatch())?;
            let (constructor, owner) = source.constructor().ok_or_else(mismatch)?;
            if !constructor.same_as(birth.source_id())
                || !relation.constructor_source().same_as(constructor)
                || relation.constructor_owner() != *owner
                || relation.formal_binding().owner() != *owner
                || relation.unqualified_field_source_v1(&core.class).is_err()
            {
                return Err(mismatch());
            }
            retained_relations.push(*relation);
            continue;
        }
        let Ok(plan) = &core.construction else {
            continue;
        };
        let (source, owner) = plan.constructor().ok_or_else(mismatch)?;
        if !relation.constructor_source().same_as(source)
            || !relation.constructor_source().same_as(birth.source_id())
            || relation.constructor_owner() != *owner || relation.formal_binding().owner() != *owner
            || plan.object() != core.object
            || plan.stores().iter().filter(|store| store.field() == relation.field()
                && matches!(store.rhs(), ConstructionStoreRhsV1::Parameter { binding, provided: Some(child), .. }
                    if *binding == relation.formal_binding() && *child == relation.child())).count() != 1
        { return Err(mismatch()); }
        retained_relations.push(*relation);
    }
    if retained_relations.is_empty() {
        return Ok(());
    }
    for relation in &retained_relations {
        let matches = arguments
            .iter()
            .filter(|argument| {
                argument.owner() == core.site.owner()
                    && argument.new_site() == &core.site
                    && argument.ordinal() == relation.formal_ordinal()
                    && argument.site() == relation.actual().site()
            })
            .count();
        if matches != 1 {
            return Err(mismatch());
        }
    }
    for argument in arguments.iter_mut() {
        let retained = retained_relations
            .iter()
            .filter(|relation| relation.formal_ordinal() == argument.ordinal())
            .map(|relation| Rc::clone(relation))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        argument.retain_child_relations(retained);
    }
    Ok(())
}

/// Extend the existing source-identity owner with unavailable and drift cases.
#[cfg(test)]
fn check_unavailable_argument_loans_v1(
    core: &mut OrdinaryNewClaimCoreV1,
    facts: &result_class_claim::OrdinaryNewResultClassClaimsV1,
) {
    use crate::mir::normal_callable_semantic_package::instance_construction::ConstructionUnavailableV1;
    let before_arguments = core.argument_rows.clone();
    let original_plan = std::mem::replace(
        &mut core.construction,
        Err(ConstructionUnavailableV1::OverrideUnsupported),
    );
    assert!(retain_argument_loans(core, facts).is_ok());
    assert_eq!(
        core.construction,
        Err(ConstructionUnavailableV1::OverrideUnsupported)
    );
    assert_eq!(core.argument_rows, before_arguments);
    core.construction = original_plan;
    let unavailable = SelectedNewArgumentUnavailableV1::SourceMismatch {
        new_site: core.site.clone(),
    };
    let original_arguments = std::mem::replace(&mut core.argument_rows, Err(unavailable.clone()));
    assert!(retain_argument_loans(core, facts).is_ok());
    assert_eq!(core.argument_rows, Err(unavailable));
    core.argument_rows = original_arguments;
    let arguments = core.argument_rows.as_mut().unwrap();
    let index = arguments
        .iter()
        .position(|argument| !argument.child_relations().is_empty())
        .unwrap();
    let source = &arguments[index];
    let wrong = OrdinaryNewTrivialArgumentV1::new(
        source.owner(),
        source.new_site().clone(),
        source.ordinal() + 1,
        source.site().clone(),
        source.kind().clone(),
    );
    let original = std::mem::replace(&mut arguments[index], wrong);
    assert!(retain_argument_loans(core, facts).is_err());
    core.argument_rows.as_mut().unwrap()[index] = original;
}

#[cfg(test)]
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn check_argument_child_relation_loans_for_test(
        &self,
        site: &OwnedExprSiteV1,
    ) {
        let facts = &self.callable_result_classes;
        let mut claims = self.result_claims.borrow_mut();
        let core = &mut claims
            .get_mut(site)
            .expect("original pending returned-New")
            .core;
        let mut retained = 0;
        for argument in core.argument_rows.as_ref().unwrap().iter() {
            for loan in argument.child_relations() {
                assert_eq!(loan.outer(), site);
                assert_eq!(loan.formal_ordinal(), argument.ordinal());
                assert_eq!(loan.actual().site(), argument.site());
                let original = facts
                    .construction_child_relation_loans()
                    .find(|original| original.outer() == site && original.field() == loan.field())
                    .unwrap();
                assert!(Rc::ptr_eq(original, loan));
                let cloned = argument.clone();
                assert_eq!(cloned, *argument);
                assert!(cloned
                    .child_relations()
                    .iter()
                    .any(|same| Rc::ptr_eq(same, loan)));
                retained += 1;
            }
        }
        assert!(
            retained > 0,
            "original constructor argument retains its child relation"
        );
        check_unavailable_argument_loans_v1(core, facts);
    }
}
