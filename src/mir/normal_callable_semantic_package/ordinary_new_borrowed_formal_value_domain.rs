//! One incoming/Forward fixed point, projected to object and integer agreements.
//! Integer agreement is source evidence, never a physical payload projection.
use super::*;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::resolved_semantics::{
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceBindingSiteV1,
};

enum FormalDomainV1 {
    Integer,
    Object(BorrowedFormalObjectViewV1),
}

pub(super) fn resolve_formal_domains_v1(
    seeds: &BTreeMap<BindingRefV1, Vec<FormalActualSeedV1>>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
) -> (
    BTreeMap<BindingRefV1, BorrowedFormalObjectViewV1>,
    BTreeSet<BindingRefV1>,
) {
    // None is a final refusal. Pending Forward never proves a domain.
    let mut resolved: BTreeMap<BindingRefV1, Option<FormalDomainV1>> = BTreeMap::new();
    loop {
        let mut progressed = false;
        for (formal, rows) in seeds {
            if resolved.contains_key(formal) {
                continue;
            }
            let mut class: Option<Box<str>> = None;
            let mut integer = false;
            let mut null = false;
            let mut blocked = false;
            let mut declined = false;
            for seed in rows {
                match seed {
                    FormalActualSeedV1::Null => null = true,
                    FormalActualSeedV1::Integer => integer = true,
                    FormalActualSeedV1::Class(name) => {
                        declined |= class.as_ref().is_some_and(|prev| prev != name);
                        class = class.or_else(|| Some(name.clone()));
                    }
                    FormalActualSeedV1::Forward(origin) => match resolved.get(origin) {
                        None => blocked = true,
                        Some(None) => declined = true,
                        Some(Some(FormalDomainV1::Integer)) => integer = true,
                        Some(Some(FormalDomainV1::Object(view))) => {
                            declined |= class.as_ref().is_some_and(|prev| *prev != view.class);
                            class = class.or_else(|| Some(view.class.clone()));
                        }
                    },
                    FormalActualSeedV1::ForwardIntegerOnly(origin) => match resolved.get(origin) {
                        None => blocked = true,
                        Some(Some(FormalDomainV1::Integer)) => integer = true,
                        Some(_) => declined = true,
                    },
                    FormalActualSeedV1::Conflict => declined = true,
                }
            }
            declined |= integer && (null || class.is_some());
            if declined {
                resolved.insert(*formal, None);
                progressed = true;
            } else if !blocked {
                let domain = if integer {
                    Some(FormalDomainV1::Integer)
                } else {
                    class
                        .and_then(|name| object_view_for(batch, constructors, &name))
                        .map(FormalDomainV1::Object)
                };
                resolved.insert(*formal, domain);
                progressed = true;
            }
        }
        if !progressed {
            for formal in seeds.keys() {
                resolved.entry(*formal).or_insert(None);
            }
            break;
        }
    }
    let mut objects = BTreeMap::new();
    let mut integers = BTreeSet::new();
    for (formal, domain) in resolved {
        match domain {
            Some(FormalDomainV1::Integer) => {
                integers.insert(formal);
            }
            Some(FormalDomainV1::Object(view)) => {
                objects.insert(formal, view);
            }
            None => {}
        }
    }
    (objects, integers)
}

/// Direct declared parameter evidence only; no scalar-local alias inference.
pub(super) fn declared_integer_seed_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    binding: BindingRefV1,
) -> bool {
    let mut owners = contracts.iter().filter(|row| row.owner == input.owner());
    let Some(contract) = owners.next() else {
        return false;
    };
    let function = input.function();
    let mut formals = contract
        .parameters
        .iter()
        .filter(|row| row.binding == binding);
    let Some(formal) = formals.next() else {
        return false;
    };
    if owners.next().is_some() || formals.next().is_some()
        || function.owner() != input.owner() || binding.owner() != input.owner()
        || contract.parameters.get(formal.ordinal as usize).is_none_or(|row| row.binding != binding)
        || formal.kind != CallableParameterContractKindV1::ExactTrivial(ExactTrivialParameterAbiV1::I64)
        || function.declaration_binding(&SourceBindingSiteV1::Parameter { index: formal.ordinal }) != Some(binding)
        || function.assignment_targets().any(|(_, target)| matches!(target, ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding))
    { return false }
    for (_, owner) in input.forest().semantic_owners() {
        if owner.assignment_targets().any(|(_, target)| matches!(target, ResolvedAssignmentTargetV1::UpvarRebind(upvar) if upvar.source() == binding))
            || owner.variable_refs().any(|(_, reference)| matches!(reference, ResolvedLexicalRefV1::Upvar(upvar) if upvar.source() == binding))
        { return false }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::SelectedNormalCallableKeyV1;
    use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;

    fn domain_package(
        main: &str,
        probe: &str,
    ) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
        issue_with_brand_catalog(&format!(
            "box Token {{}} box Transport {{ probe(p) {{ {probe} }} sink(q) {{ return 0 }} }} static box Main {{ main() {{ {main} }} }}"
        )).unwrap()
    }

    fn formal(
        package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
        selector: &str,
    ) -> BindingRefV1 {
        let key = SelectedNormalCallableKeyV1::Cataloged(
            CanonicalSameModuleCallableKeyV1::instance_box_method("Transport", selector, 1),
        );
        let slot = package.selected.batch_slot(&key).unwrap();
        package
            .parameter_contracts
            .iter()
            .find(|row| row.batch_slot == slot)
            .unwrap()
            .parameters[0]
            .binding
    }

    #[test]
    fn shared_incoming_domain_proves_literal_and_forwarded_integer_without_object_view() {
        for actual in ["7", "-7", "0"] {
            let package = domain_package(
                &format!("local recv = new Transport() local out = recv.probe({actual}) return 0"),
                "local alias = p local recv = new Transport() local out = recv.sink(alias) return 0",
            );
            let ingress = package
                .ordinary_new_claim_ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            for selector in ["probe", "sink"] {
                let binding = formal(&package, selector);
                assert!(
                    ingress.formal_integer_agreement(binding),
                    "{selector}: {actual}"
                );
                assert!(ingress.formal_object_view(binding).is_none());
            }
        }
    }

    #[test]
    fn shared_incoming_domain_refuses_mixed_null_bool_object_and_unproved_local() {
        for tail in [
            "local b = recv.probe(null)",
            "local b = recv.probe(true)",
            "local token = new Token() local b = recv.probe(token)",
            "local n = 8 local b = recv.probe(n)",
        ] {
            let package = domain_package(
                &format!("local recv = new Transport() local a = recv.probe(7) {tail} return 0"),
                "return 0",
            );
            let ingress = package
                .ordinary_new_claim_ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            let binding = formal(&package, "probe");
            assert!(!ingress.formal_integer_agreement(binding), "{tail}");
            assert!(ingress.formal_object_view(binding).is_none(), "{tail}");
            assert_eq!(
                ingress
                    .incoming
                    .iter()
                    .filter(|row| row.callee == binding.owner())
                    .count(),
                2,
                "never omit the conflicting original caller"
            );
        }
    }

    #[test]
    fn shared_incoming_domain_preserves_direct_null_object_exception_and_forward_refusal() {
        let package = domain_package("local recv = new Transport() local token = new Token() local a = recv.probe(token) local b = recv.probe(null) return 0", "return 0");
        let ingress = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let binding = formal(&package, "probe");
        assert_eq!(
            ingress.formal_object_view(binding).unwrap().class(),
            "Token"
        );
        assert!(!ingress.formal_integer_agreement(binding));
        let package = domain_package("local recv = new Transport() local a = recv.probe(null) return 0", "local recv = new Transport() local token = new Token() local a = recv.sink(token) local b = recv.sink(p) return 0");
        let ingress = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        for selector in ["probe", "sink"] {
            let binding = formal(&package, selector);
            assert!(ingress.formal_object_view(binding).is_none());
            assert!(!ingress.formal_integer_agreement(binding));
        }
    }

    #[test]
    fn shared_incoming_domain_does_not_invent_integer_from_empty_or_unresolved_cycle() {
        let package = domain_package("return 0", "return 0");
        let p = formal(&package, "probe");
        let q = formal(&package, "sink");
        for seeds in [
            BTreeMap::from([(p, vec![])]),
            BTreeMap::from([
                (p, vec![FormalActualSeedV1::Forward(q)]),
                (q, vec![FormalActualSeedV1::Forward(p)]),
            ]),
            BTreeMap::from([
                (
                    p,
                    vec![FormalActualSeedV1::Integer, FormalActualSeedV1::Forward(q)],
                ),
                (q, vec![FormalActualSeedV1::Forward(p)]),
            ]),
        ] {
            let (objects, integers) =
                resolve_formal_domains_v1(&seeds, package.batch(), &package.instance_constructors);
            assert!(objects.is_empty());
            assert!(
                integers.is_empty(),
                "a pending cycle is not a completed agreement"
            );
        }
    }

    #[test]
    fn shared_incoming_domain_invalid_class_declines_dependent_forward() {
        let package = domain_package("return 0", "return 0");
        let p = formal(&package, "probe");
        let q = formal(&package, "sink");
        let seeds = BTreeMap::from([
            (p, vec![FormalActualSeedV1::Class("Absent".into())]),
            (
                q,
                vec![
                    FormalActualSeedV1::Forward(p),
                    FormalActualSeedV1::Class("Token".into()),
                ],
            ),
        ]);
        let (objects, integers) =
            resolve_formal_domains_v1(&seeds, package.batch(), &package.instance_constructors);
        assert!(objects.is_empty());
        assert!(integers.is_empty());
    }

    #[test]
    fn candidate_integer_forward_never_propagates_source_object_authority() {
        let package = domain_package(
            "local recv = new Transport() local out = recv.probe(7) return 0",
            "return 0",
        );
        let p = formal(&package, "probe");
        let q = formal(&package, "sink");
        let seeds = BTreeMap::from([
            (p, vec![FormalActualSeedV1::Class("Token".into())]),
            (q, vec![FormalActualSeedV1::ForwardIntegerOnly(p)]),
        ]);
        let (objects, integers) =
            resolve_formal_domains_v1(&seeds, package.batch(), &package.instance_constructors);
        assert!(objects.contains_key(&p));
        assert!(!objects.contains_key(&q));
        assert!(integers.is_empty());
    }

    #[test]
    fn declared_integer_seed_requires_stable_direct_parameter_and_original_ordinal() {
        for (body, expected) in [
            ("return 0", true),
            ("p = true return 0", false),
            ("local closure = fn() { return p } return 0", false),
            ("local closure = fn() { p = true return 0 } return 0", false),
        ] {
            let package = issue_with_brand_catalog(&format!(
                "static box Main {{ helper(p: i64) {{ {body} }} main() {{ return 0 }} }}"
            ))
            .unwrap();
            let key = SelectedNormalCallableKeyV1::Cataloged(
                CanonicalSameModuleCallableKeyV1::static_box_method("Main", "helper", 1),
            );
            let slot = package.selected.batch_slot(&key).unwrap();
            let contract = package
                .parameter_contracts
                .iter()
                .find(|row| row.batch_slot == slot)
                .unwrap();
            let binding = contract.parameters[0].binding;
            package.batch().with_lowering_input(slot, |input| {
                assert_eq!(declared_integer_seed_v1(input, &package.parameter_contracts, binding), expected, "{body}");
                let bad = OwnedCallableParameterContractDeclarationV1 {
                    owner: contract.owner, batch_slot: contract.batch_slot, mode: contract.mode,
                    parameters: vec![crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1 {
                        ordinal: 1, binding, kind: contract.parameters[0].kind.clone(),
                    }].into_boxed_slice(),
                };
                assert!(!declared_integer_seed_v1(input, &[bad], binding));
            }).unwrap();
        }
    }

    #[test]
    fn shared_incoming_domain_uses_direct_declared_integer_from_outside_borrowed_profile() {
        let package = issue_with_brand_catalog(
            "box Transport { probe(p) { return 0 } } static box Main { send(size: i64) { local recv = new Transport() local out = recv.probe(size) return 0 } main() { return Main.send(7) } }",
        ).unwrap();
        let binding = formal(&package, "probe");
        let ingress = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        assert!(ingress.formal_integer_agreement(binding));
        assert!(ingress.formal_object_view(binding).is_none());
        assert_eq!(ingress.incoming.len(), 1);
        assert!(
            !ingress.contains_definition_for_test(ingress.incoming[0].call.owner()),
            "direct declared scalar evidence is distinct from an opaque caller profile"
        );
    }

    #[test]
    fn declared_integer_scalar_alias_never_inherits_unproven_parameter_agreement() {
        for alias_body in ["local alias = p", "local alias = p alias = true"] {
            let package = issue_with_brand_catalog(&format!(
                "static box Main {{ helper(p: i64) {{ {alias_body} local observed = alias return 0 }} main() {{ return 0 }} }}"
            )).unwrap();
            let key = SelectedNormalCallableKeyV1::Cataloged(
                CanonicalSameModuleCallableKeyV1::static_box_method("Main", "helper", 1),
            );
            let slot = package.selected.batch_slot(&key).unwrap();
            package
                .batch()
                .with_lowering_input(slot, |input| {
                    let initializer = input
                        .function()
                        .expression_source()
                        .initializers()
                        .next()
                        .unwrap();
                    assert!(!declared_integer_seed_v1(
                        input,
                        &package.parameter_contracts,
                        initializer.binding()
                    ));
                    let observed = input
                        .function()
                        .expression_source()
                        .initializers()
                        .last()
                        .unwrap();
                    let site = observed.initializer_site().unwrap();
                    assert_eq!(
                        super::super::arg_site_binding(input, site),
                        Some(initializer.binding())
                    );
                    let source = classify_actual_seed(
                        input,
                        package.batch(),
                        &package.selected,
                        &package.ordinary_new_claim_ledger.callable_result_classes,
                        input.owner(),
                        &[],
                        None,
                        &BTreeMap::new(),
                        &package.parameter_contracts,
                        site,
                    );
                    // Read the original alias-use site, including the rebound case.
                    // No declared scalar fact is inherited through this alias.
                    assert!(matches!(source, FormalActualSeedV1::Conflict));
                })
                .unwrap();
        }
    }
}
