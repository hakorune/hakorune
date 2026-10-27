use super::*;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
use std::rc::Rc;

fn issue(leaf: &str, body: &str) -> Result<VerifiedNormalCallableSemanticPackageV1, String> {
    let source = format!("box Transport {{ birth() {{ }} leaf(p) {{ {leaf} }}
        bridge(p) {{ local recv = new Transport() {body} }}
        wrap(p) {{ local recv = new Transport() local out = recv.bridge(p) return out }} }}
        static box Main {{ main() {{ local recv = new Transport() local out = recv.wrap(null) return 0 }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .map_err(|error| format!("{error:?}"))
}

#[test]
fn call_result_source_rejects_rebind_non_i64_and_ungrounded_cycles() {
    assert!(issue("return 7", "local out = recv.leaf(p) return out").is_ok());
    for (leaf, body) in [
        ("return 7", "local out = recv.leaf(p) out = 8 return out"),
        ("return null", "local out = recv.leaf(p) return out"),
        (
            "if p == null { return null } return new Transport()",
            "local out = recv.leaf(p) return out",
        ),
        (
            "local recv = new Transport() local out = recv.bridge(p) return out",
            "local out = recv.leaf(p) return out",
        ),
        (
            "return 7",
            "if p == null { return 0 } local out = recv.bridge(p) return out",
        ),
        ("return", "local out = recv.leaf(p) return out"),
        ("return 7", "local out = recv.bridge(p) return out"),
    ] {
        let error = issue(leaf, body)
            .err()
            .expect("selected source result is unproven");
        assert!(error.contains("borrowed-result/"), "{leaf}/{body}: {error}");
    }
    for (birth, update, actual, expected) in [
        ("me.n = capacity", "me.n = me.n + step", "0", true),
        ("me.n = 0", "me.n = me.n - step", "true", true),
        ("me.n = capacity", "me.n = me.n + step", "true", false),
        ("me.n = me.n", "me.n = me.n + step", "0", false),
        ("me.n = 0", "me.n = true", "0", false),
        ("me.n = 0", "step = true me.n = step", "0", false),
    ] {
        let text = format!("box Counter {{ n: i64 birth(capacity) {{ {birth} }}
            bump(step: i64) {{ {update} return 0 }} read() {{ return me.n }} }}
            static box Main {{ main() {{ local c = new Counter({actual}) return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text)
            .expect("original counter field/result source cohort");
        let owner = package.parameter_contracts.iter().find(|row|
            matches!(package.selected.key_for_batch_slot(row.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.owner() == "Counter" && key.name() == "read"))
            .unwrap().owner;
        let result = package.ordinary_new_claim_ledger.borrowed_i64_results.get(&owner);
        assert_eq!(result.is_some_and(|result| result.is_ok()), expected, "{birth}/{update}/{actual}: {result:?}");
        if let Some(Ok(proof)) = result {
            assert_eq!(proof.class, BorrowedResultClassV1::I64);
            proof.require_source_sealed_v1().unwrap();
            assert_eq!(proof.expressions.len(), 1);
            assert!(proof.field_component.as_ref().unwrap().permits(&proof.expressions[0]));
        }
    }

    // Original field → New actual → Birth formal shares the same closure.
    // Every New and every later writer remains conjunctive; no cyclic base.
    for (seed, update, extra, expected) in [
        ("7", "", "", true),
        ("7", "me.n = me.n + 1", "", true),
        ("true", "", "", false),
        ("7", "me.n = true", "", false),
        ("7", "", "local bad = new Handle(true)", false),
        ("7", "", "local peer = new Handle(me.n)", true),
    ] {
        let text = format!("box Handle {{ n: i64 birth(value) {{ me.n = value }} read() {{ return me.n }} }}
            box Page {{ n: i64 birth(value) {{ me.n = value }}
                update() {{ {update} return 0 }}
                make() {{ local h = new Handle(me.n) {extra} return 0 }} }}
            static box Main {{ main() {{ local p = new Page({seed}) return 0 }} }}");
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let owner = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                if key.owner() == "Handle" && key.name() == "read")).unwrap().owner;
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let results = &ledger.borrowed_i64_results;
        assert_eq!(results[&owner].is_ok(), expected, "{seed}/{update}/{extra}: {:?}", results[&owner]);
        if expected {
            let component = results[&owner].as_ref().unwrap().field_component.as_ref().unwrap();
            let field = &component.original.sources[&("Handle".into(), "n".into())];
            let ingress = ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap();
            recheck_field_source_v1(results, ingress, component, field, "n").unwrap();
            for erase in [false, true] {
                ingress.with_replaced_original_new_evidence_for_test(erase, |ingress| {
                    assert!(recheck_field_source_v1(results, ingress, component, field, "n").is_err(),
                        "equal foreign or erased New census cannot reuse the field closure");
                });
            }
            recheck_field_source_v1(results, ingress, component, field, "n").unwrap();
        }
    }
    let cycle = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Handle { n: i64 birth(value) { me.n = value }
         clone() { local h = new Handle(me.n) return 0 } read() { return me.n } }
         static box Main { main() { return 0 } }").unwrap();
    let context = cycle.ordinary_new_claim_ledger.borrowed_i64_results.field_candidates.as_ref().unwrap();
    assert!(!context.grounded_bases(None).contains(&("Handle".into(), "n".into())),
        "a sole self-copy New actual has no productive numeric root");

    // Array-get locals use the existing exact read issuer, not generic local
    // inference. Nonzero Birth ordinal, all stores and local stability matter.
    for (stored, update, expected) in [("7", "", true), ("true", "", false), ("7", "value = true", false)] {
        let text = format!("box Handle {{ n: i64 birth(ignored, value) {{ me.n = value }} read() {{ return me.n }} }}
            box Page {{ values: ArrayBox = new ArrayBox() birth() {{ }}
                seed() {{ me.values.set(0, {stored}) return 0 }}
                make() {{ local value = me.values.get(0) {update} local h = new Handle(true, value) return 0 }} }}
            static box Main {{ main() {{ return 0 }} }}");
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let owner = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                if key.owner() == "Handle" && key.name() == "read")).unwrap().owner;
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let results = &ledger.borrowed_i64_results;
        assert_eq!(results[&owner].is_ok(), expected, "Array={stored}/{update}: {:?}", results[&owner]);
        if expected {
            let component = results[&owner].as_ref().unwrap().field_component.as_ref().unwrap();
            let field = &component.original.sources[&("Handle".into(), "n".into())];
            let ingress = ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap();
            recheck_field_source_v1(results, ingress, component, field, "n").unwrap();
            ingress.with_replaced_original_new_evidence_for_test(true, |ingress| {
                assert!(recheck_field_source_v1(results, ingress, component, field, "n").is_err());
            });
        }
    }

    // Non-Me writes keep their original source; class agreement and signed
    // value evidence are separate conjuncts, even for a usize declaration.
    // Nullable forwarding matches the original resizeInPlace source cohort.
    for (receiver_type, value_type, actual, extra, expected) in [
        ("", "", "item", "", true),
        (": Item", "", "item", "", true),
        (": Item", ": usize", "item", "", false),
        (": Item", "", "other", "", false),
        ("", "", "item", "unknown(q) { q.n = true return 0 }", false),
        ("", "", "item", "cycle(q, v) { if q == null { return 0 } if v <= 0 { return 0 } q.n = v local w = new Writer() return w.cycle(q, v) }", false),
    ] {
        let text = format!("box Item {{ n: usize birth(n) {{ me.n = n }} read() {{ return me.n }} }}
            box Other {{ n: usize birth() {{ me.n = 0 }} }}
            box Writer {{ birth() {{ }} write(p{receiver_type}, q{receiver_type}, value{value_type}) {{
                if p == null {{ return null }} if q == null {{ return null }} if value <= 0 {{ return null }}
                p.n = value q.n = value return p }} {extra} }}
            static box Main {{ main() {{ local item = new Item(3) local other = new Other()
                local writer = new Writer() local ignored = writer.write({actual}, item, 7) return 0 }} }}");
        let issued = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text);
        let mut package = match issued {
            Ok(package) => package,
            Err(issue) => { assert!(!expected, "{receiver_type}/{value_type}/{actual}: {issue:?}"); continue; }
        };
        let owner = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.owner() == "Item" && key.name() == "read")).unwrap().owner;
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let results = &ledger.borrowed_i64_results;
        if expected {
            let context = results.field_candidates.as_ref().unwrap();
            let key = ("Item".into(), "n".into());
            let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            let contract = package.parameter_contracts.iter().find(|row| matches!(
                package.selected.key_for_batch_slot(row.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.owner() == "Writer" && key.name() == "write")).unwrap();
            for parameter in &contract.parameters[..2] {
                assert_eq!(ingress.original_object_class_for_test(parameter.binding), Some("Item"));
            }
            assert_eq!(context.sources[&key].writers().count(), 3, "Birth and both original formal writers");
            assert!(context.bases.contains(&key), "original New supplies a productive Birth");
            for writer in context.writers[&key].iter().filter(|writer| writer.birth.is_none()) {
                assert_eq!(writer.expression.class(&BTreeSet::new(), &BTreeMap::new()), Some(BorrowedResultClassV1::I64), "non-Me value not I64: {writer:?}");
            }
        }
        assert_eq!(results[&owner].is_ok(), expected, "{receiver_type}/{value_type}/{actual}/{extra}: {:?}", results[&owner]);
        if expected {
            let component = results[&owner].as_ref().unwrap().field_component.as_ref().unwrap();
            let field = &component.original.sources[&("Item".into(), "n".into())];
            assert_eq!(field.declared_type_name(), Some("usize"));
            assert!(field.writers().enumerate().any(|(ordinal, writer)| !writer.is_attributed() && field.needs_formal_target(ordinal)),
                "conditional formal source is not rewritten as Me");
            let ingress = ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap();
            let context = &component.original;
            let key = ("Item".into(), "n".into());
            let mut targets = context.writers[&key].iter().filter(|writer| writer.target.is_some());
            let first = targets.next().unwrap();
            let second = targets.next().unwrap();
            assert_ne!(first.target.as_ref().unwrap().receiver.binding(), second.target.as_ref().unwrap().receiver.binding());
            let swapped = FieldWriterV1 { ordinal: first.ordinal, expression: Rc::clone(&first.expression), birth: None,
                target: Some(FormalFieldWriterTargetV1 { original: first.target.as_ref().unwrap().original.clone(),
                    receiver: second.target.as_ref().unwrap().receiver.clone() }) };
            assert!(!context.target_ready(&key, &swapped, Some(ingress)), "same-class formal cannot replace the original writer receiver");
            recheck_field_source_v1(results, ingress, component, field, "n").unwrap();
            ingress.with_replaced_original_new_evidence_for_test(true, |ingress| {
                assert!(recheck_field_source_v1(results, ingress, component, field, "n").is_err());
            });
        }
    }

    // A copied Birth value survives later changes to its original field.
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Counter { a: i64 b: i64 birth() { me.a = 1 me.b = me.a }
         alter() { me.a = true return 0 } read() { return me.b } }
         static box Main { main() { local c = new Counter() return 0 } }").unwrap();
    let owner = package.parameter_contracts.iter().find(|row|
        matches!(package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == "read")).unwrap().owner;
    package.ordinary_new_claim_ledger.borrowed_i64_results[&owner].as_ref().unwrap().require_source_i64_v1().unwrap();

    let mut foreign_sources = original::prepare_original_returns_v1(package.batch(), &package.selected, &package.parameter_contracts);
    let foreign = foreign_sources.remove(&owner).unwrap().unwrap().unwrap();
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let component = Rc::clone(ledger.borrowed_i64_results.field_component.as_ref().unwrap().as_ref().unwrap());
    ledger.borrowed_i64_results.require_field_component_v1(&component).unwrap();
    let retained = std::mem::replace(&mut ledger.borrowed_i64_results.field_component, Ok(None));
    assert!(ledger.borrowed_i64_results.require_field_component_v1(&component).unwrap_err().contains("field-component-unsealed"));
    ledger.borrowed_i64_results.field_component = Err("test/cohort-ingress-refused".into());
    assert_eq!(ledger.borrowed_i64_results.require_field_component_v1(&component).unwrap_err(), "test/cohort-ingress-refused");
    ledger.borrowed_i64_results.field_component = retained;
    let original = std::mem::replace(&mut ledger.borrowed_i64_results.field_candidates, Err("test/original-cohort-lost".into()));
    assert_eq!(ledger.borrowed_i64_results.require_field_component_v1(&component).unwrap_err(), "test/original-cohort-lost");
    ledger.borrowed_i64_results.field_candidates = original;
    let proof = ledger.borrowed_i64_results.get_mut(&owner).unwrap().as_mut().unwrap();
    let expressions = proof.expressions.clone();
    proof.source_facts_mut_for_test().expressions = Box::new([]);
    assert!(proof.require_source_sealed_v1().unwrap_err().contains("original-return-plan-identity"));
    proof.source_facts_mut_for_test().expressions = expressions;
    proof.source_facts_mut_for_test().original_return = Some(foreign);
    assert!(proof.require_source_sealed_v1().unwrap_err().contains("original-return-plan-identity"));
    for body in ["if true { return me.n } return", "return me.n"] {
        let text = format!("box Counter {{ n: i64 birth() {{ me.n = true }} read() {{ {body} }} }}
            static box Main {{ main() {{ local c = new Counter() return 0 }} }}");
        let issued = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text);
        if body.contains("if true") {
            assert!(format!("{:?}", issued.unwrap_err()).contains("ReturnClassificationInvariant"),
                "mixed bare/value Return is already refused by the earlier physical header");
            continue;
        }
        let package = issued.unwrap();
        let owner = package.parameter_contracts.iter().find(|row|
            matches!(package.selected.key_for_batch_slot(row.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == "read")).unwrap().owner;
        let originals = original::prepare_original_returns_v1(package.batch(), &package.selected, &package.parameter_contracts);
        assert!(originals[&owner].as_ref().unwrap().is_some(), "passive field requirement survives invalid result/Completion");
        assert!(package.ordinary_new_claim_ledger.borrowed_i64_results[&owner].is_err());
    }

    for operand in [false, true] {
        let read = if operand { "return me.n < me.seed()" } else { "return me.n" };
        let text = format!("box Counter {{ n: i64 m: i64 birth() {{ me.n = 0 me.m = 0 }}
            seed() {{ return 7 }} alter() {{ me.m = me.seed() return 0 }}
            bump() {{ me.n = me.m return 0 }} read() {{ {read} }} }}
            static box Main {{ main() {{ local c = new Counter() return 0 }} }}");
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let named = |name: &str| package.parameter_contracts.iter().find(|row|
            matches!(package.selected.key_for_batch_slot(row.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == name)).unwrap().owner;
        let read_owner = named("read");
        let seed_owner = named("seed");
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let proof = ledger.borrowed_i64_results[&read_owner].as_ref().unwrap();
        assert_eq!(proof.class, if operand { BorrowedResultClassV1::Bool } else { BorrowedResultClassV1::I64 });
        let edges = source_dependencies_v1(proof, ingress).unwrap();
        assert!(edges.iter().any(|(target, role)| target.callee_owner() == seed_owner && *role == SourceDependencyRoleV1::FieldWriter));
        assert_eq!(edges.iter().any(|(_, role)| *role == SourceDependencyRoleV1::ExpressionOperand), operand);
        assert!(grounded_source_results_with_ingress_v1(&ledger.borrowed_i64_results, ingress).contains(&read_owner));
        let field_leaf = if !operand {
            let expression = &proof.expressions[0];
            let receiver = proof.original_return.as_ref().unwrap().receiver();
            let site = expression.plan().nodes()[0].site().clone();
            let crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Shape(
                BodyExpressionShapeV1::FieldAccess { object, .. }) = expression.plan().nodes()[0].source() else { panic!("original field") };
            let source = Rc::clone(&proof.field_component.as_ref().unwrap().original.sources[&("Counter".into(), "n".into())]);
            let owned = OwnedExprSiteV1::new(read_owner, site);
            let loan = super::super::expression_field::ExpressionFieldSourceLoanV1::issue(
                &ledger.borrowed_i64_results, ingress, Rc::clone(expression.plan()), source,
                &owned, object, receiver, receiver, "n").unwrap();
            Some((loan, Rc::clone(expression.plan()), owned, object.clone(), receiver))
        } else { None };
        ledger.borrowed_i64_results.insert(seed_owner, Err(freeze("test/lost-field-writer-callee")));
        let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        assert!(!grounded_source_results_with_ingress_v1(&ledger.borrowed_i64_results, ingress).contains(&read_owner));
        if let Some((loan, plan, site, object, receiver)) = field_leaf {
            assert!(loan.recheck(&ledger.borrowed_i64_results, ingress, &plan,
                &site, &object, receiver, receiver, loan.field()).is_err(),
                "standalone field source must re-demand the lost writer callee");
        }
    }

    // Original field context exists without any getter or successful result row.
    for (birth, expected) in [("me.n = 0", true), ("me.n = true", false), ("me.n = me.n", false)] {
        let text = format!("box Counter {{ n: i64 birth() {{ {birth} }} }}
            static box Main {{ main() {{ local c = new Counter() print(\"n=\" + c.n) return 0 }} }}");
        let issued = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text);
        if !expected {
            assert!(format!("{:?}", issued.err().expect("nonnumeric Print must refuse source qualification"))
                .contains("integer-field/source-missing"), "{birth}");
            continue;
        }
        let package = issued.unwrap();
        let cohort = &package.ordinary_new_claim_ledger.borrowed_i64_results;
        let context = cohort.field_candidates.as_ref().expect("original field obligations retained");
        let component = cohort.field_component.as_ref().unwrap().as_ref().expect("single cohort closure");
        assert!(Rc::ptr_eq(context, &component.original));
        assert_eq!(component.fields.contains(&("Counter".into(), "n".into())), expected, "{birth}");
        let empty = BorrowedResultCohortV1::prepared(BTreeMap::new(), Rc::clone(context));
        let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let (plan, foreign_plan, site, object, receiver) = package.batch().declarations().find_map(|declaration| {
            package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                let shape = input.body_shape()?;
                let statement = shape.statements().iter().find(|row| matches!(row,
                    crate::mir::resolved_semantics::BodyStatementShapeV1::Print { .. }))?;
                let ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
                let prepare = || crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
                    &ledger, shape, statement.site()).unwrap().unwrap();
                let plan = prepare();
                let (site, object) = plan.nodes().iter().find_map(|node| match node.source() {
                    crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Shape(
                        BodyExpressionShapeV1::FieldAccess { object, .. }) => Some((node.site().clone(), object.clone())),
                    _ => None,
                }).unwrap();
                let Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(receiver)) = input.function().variable_ref(&object)
                    else { panic!("original local receiver") };
                Some((plan, prepare(), OwnedExprSiteV1::new(input.owner(), site), object, receiver))
            }).unwrap()
        }).expect("original getter-free Print expression");
        let staged = package.ordinary_new_claim_ledger.field_reads.borrow().len();
        let leaf = context.sources.get(&("Counter".into(), "n".into())).map(|source|
            super::super::expression_field::ExpressionFieldSourceLoanV1::issue(
                cohort, ingress, Rc::clone(&plan), Rc::clone(source), &site, &object, receiver, receiver, "n"));
        assert_eq!(leaf.as_ref().is_some_and(Result::is_ok), expected, "getter-free Print field: {birth}");
        if let Some(Ok(leaf)) = leaf {
            assert!(leaf.matches_declaration(leaf.field(), Some("i64")));
            assert!(!leaf.matches_declaration(leaf.field(), None));
            assert!(!leaf.matches_declaration(leaf.field(), Some("usize")));
            leaf.recheck(cohort, ingress, &plan, &site, &object, receiver, receiver, leaf.field()).unwrap();
            assert!(leaf.recheck(cohort, ingress, &foreign_plan, &site, &object, receiver, receiver, leaf.field())
                .unwrap_err().contains("integer-field/source-identity"));
        }
        assert_eq!(package.ordinary_new_claim_ledger.field_reads.borrow().len(), staged,
            "source-only leaf preparation never publishes a partial frame/read");
        let (_, recomputed) = grounded_with_fields_v1(&empty, Some(ingress));
        let recomputed = recomputed.unwrap();
        assert_eq!(recomputed.fields.contains(&("Counter".into(), "n".into())), expected,
            "field authority cannot depend on successful result rows");
        assert!(cohort.require_field_component_v1(&recomputed).unwrap_err().contains("field-component-identity"),
            "equal recomputed fields do not replace the SAME retained component");
        let refused = pending::seal_pending_results_v1(empty, &package.parameter_contracts,
            &Err("test/original-ingress-refused".into()), package.batch(), package.instance_constructors());
        assert!(Rc::ptr_eq(context, refused.field_candidates.as_ref().unwrap()));
        assert_eq!(refused.field_component.as_ref().unwrap_err(), "test/original-ingress-refused");
    }

}

#[test]
fn composed_result_corroboration_rejects_foreign_dependencies_and_propagates_callee_drift() {
    for change in 0..5 {
        let mut package = issue("return 7", "local out = recv.leaf(p) return out").unwrap();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let original: Vec<_> = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .incoming
            .iter()
            .map(|row| Ok(Some(row.source.require_instance().unwrap().clone())))
            .collect();
        ledger.corroborate_borrowed_result_cohort_v1(&original, &package.result_contracts);
        let callers: Vec<_> = ledger
            .borrowed_i64_results
            .iter()
            .filter_map(|(owner, row)| {
                row.as_ref()
                    .ok()
                    .filter(|row| !row.dependencies.is_empty())
                    .map(|_| *owner)
            })
            .collect();
        assert_eq!(callers.len(), 2, "two-hop chain");
        let caller = *callers
            .iter()
            .find(|owner| {
                let dependency = &ledger.borrowed_i64_results[owner]
                    .as_ref()
                    .unwrap()
                    .dependencies[0];
                ledger.borrowed_i64_results[&dependency.callee_owner()]
                    .as_ref()
                    .unwrap()
                    .dependencies
                    .is_empty()
            })
            .expect("bridge directly depends on leaf");
        assert!(
            ledger.borrowed_i64_results[&caller]
                .as_ref()
                .unwrap()
                .contract_corroborated
        );
        let proof = ledger
            .borrowed_i64_results
            .get_mut(&caller)
            .unwrap()
            .as_mut()
            .unwrap();
        let callee = proof.dependencies[0].callee_owner();
        match change {
            0 => proof.source_facts_mut_for_test().dependencies[0].target_batch_slot += 1,
            1 => {
                proof.source_facts_mut_for_test().dependencies[0].argument_sites[0] =
                    proof.dependencies[0].receiver_site.clone()
            }
            2 => proof.source_facts_mut_for_test().dependencies[0].callee_owner = caller,
            3 => {
                let leaf = ledger
                    .borrowed_i64_results
                    .get_mut(&callee)
                    .unwrap()
                    .as_mut()
                    .unwrap();
                leaf.source_facts_mut_for_test().returns[0] = original[0]
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .call_site()
                    .clone();
            }
            _ => {
                ledger
                    .borrowed_i64_results
                    .insert(callee, Err(freeze("test/missing-result")));
            }
        }
        ledger.corroborate_borrowed_result_cohort_v1(&original, &package.result_contracts);
        for owner in callers {
            assert!(ledger.borrowed_i64_results[&owner]
                .as_ref()
                .unwrap_err()
                .contains("borrowed-result/result-contract-mismatch"));
        }
        assert!(ledger.borrowed_i64_results[&caller]
            .as_ref()
            .unwrap_err()
            .contains("borrowed-result/result-contract-mismatch"));
    }
}

#[test]
fn direct_call_result_source_rejects_non_i64_and_ungrounded_cycles() {
    assert!(issue("return 7", "return recv.leaf(p)").is_ok());
    for (leaf, body) in [
        ("return null", "return recv.leaf(p)"),
        ("return", "return recv.leaf(p)"),
        (
            "if p == null { return null } return new Transport()",
            "return recv.leaf(p)",
        ),
        ("return 7", "return recv.bridge(p)"),
        (
            "local recv = new Transport() return recv.bridge(p)",
            "return recv.leaf(p)",
        ),
        (
            "return 7",
            "if p == null { return 0 } return recv.bridge(p)",
        ),
    ] {
        let error = issue(leaf, body)
            .err()
            .expect("direct result lacks an I64 proof");
        assert!(error.contains("borrowed-result/"), "{leaf}/{body}: {error}");
    }
}

#[test]
fn direct_call_result_source_rejects_nonborrowed_callee_annotation() {
    let source = "box Transport { birth() { } leaf(q: i64): i64 { return q }
        bridge(p) { local recv = new Transport() return recv.leaf(7) } }
        static box Main { main() { local recv = new Transport() local out = recv.bridge(null) return 0 } }";
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source)
        .err().expect("nonborrowed callee has no borrowed incoming source proof");
    assert!(
        format!("{error:?}").contains("borrowed-result/source-not-i64"),
        "{error:?}"
    );
}

#[test]
fn direct_call_result_source_retains_terminal_relations_for_declared_borrows() {
    use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
    for annotated in [true, false] {
        let result = if annotated { ": i64" } else { "" };
        let source = format!("box Item {{ value: i64 birth() {{ me.value = 5 }} }}
            box Transport {{ flag: i64 birth() {{ me.flag = 0 }}
                read(p: Item){result} {{ if p == null {{ return 7 }} return p.value }}
                bridge(p: Item){result} {{ local recv = new Transport() return recv.read(p) }}
                wrap(p: Item){result} {{ local recv = new Transport() return recv.bridge(p) }} }}
            static box Main {{ main() {{ local recv = new Transport() local item = new Item() return recv.wrap(item) }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        for (owner, proof) in ledger.borrowed_i64_results.iter() {
            if proof.as_ref().unwrap().dependencies.is_empty() {
                continue;
            }
            let relation = ledger.sole_terminal_relation_for_owner(*owner);
            let completion = ledger.completion_for_owner(*owner);
            assert!(
                matches!(relation, Some(TerminalRelationV1::Call(_))),
                "annotated={annotated} owner={owner:?} relation={relation:?} completion={completion:?}"
            );
            let flow = ledger
                .completion_for_owner(*owner)
                .unwrap()
                .cleanup()
                .root_flow()
                .unwrap();
            assert!(
                flow.all_exits_ready(),
                "annotated={annotated} owner={owner:?} flow={flow:?}"
            );
        }
    }
}

#[test]
fn bool_source_composition_keeps_exact_class_without_i64_or_invoke_permission() {
    for body in [
        "local out = recv.leaf(p) return out",
        "return recv.leaf(p)",
        "if p == null { return false } return recv.leaf(p)",
    ] {
        let package = issue("return true", body).expect("original Bool source chain");
        let ledger = &package.ordinary_new_claim_ledger;
        assert_eq!(ledger.borrowed_i64_results.len(), 3);
        for proof in ledger.borrowed_i64_results.values() {
            let proof = proof.as_ref().expect("grounded original Bool returns");
            assert_eq!(proof.class, BorrowedResultClassV1::Bool);
            proof.require_source_sealed_v1().unwrap();
            assert!(
                proof.require_source_i64_v1().is_err(),
                "Bool never acquires I64 source law"
            );
        }
        for slot in ledger.lexical_instance_calls.borrow().values() {
            if let LexicalInstanceCallDispositionSlotV1::Ready(row) = slot {
                assert_eq!(
                    row.result(),
                    None,
                    "source Bool has no executable Invoke ABI yet"
                );
                assert!(ledger.borrowed_call_actuals_v1(row).is_err());
            }
        }
    }
}

#[test]
fn source_scalar_composition_refuses_mixed_direct_and_call_classes() {
    for (leaf, body) in [
        (
            "return true",
            "if p == null { return 0 } return recv.leaf(p)",
        ),
        (
            "return 7",
            "if p == null { return false } return recv.leaf(p)",
        ),
    ] {
        let error = issue(leaf, body)
            .err()
            .expect("mixed scalar source refuses");
        assert!(error.contains("borrowed-result/"), "{error}");
    }
}

#[test]
fn provider_static_original_base_keeps_source_evidence_through_ready() {
    for suffix in ["", "me.child.seed()"] {
    for (body, expected) in [
        ("if kind == 0 { return 8 } if kind == 1 { return 4 } return 0", true),
        ("return true", false),
    ] {
        let text = format!("static box Layout {{ capacity(kind) {{ {body} }} }}
            box Page {{ n: i64 birth(capacity) {{ me.n = capacity }} read() {{ return me.n }} seed() {{ return 0 }} }}
            box Parent {{ child: Page = new Page(Layout.capacity(0)) birth() {{ {suffix} }} }}
            static box Main {{ main() {{ return 0 }} }}");
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text)
            .unwrap_or_else(|issue| panic!("{body}: {issue:?}"));
        if !suffix.is_empty() && expected {
            let parent = package.instance_constructors().rows().iter().find(|row| row.box_name() == "Parent").unwrap();
            assert!(matches!(parent.construction(), Err(crate::mir::normal_callable_semantic_package::ConstructionUnavailableV1::BodyCoverageUnsupported)));
        }
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let context = ledger.borrowed_i64_results.field_candidates.as_ref().unwrap();
        let parameter = context.writers.get(&("Page".into(), "n".into())).and_then(|rows|
            rows.iter().find_map(|row| match &row.birth { Some(BirthBaseV1::Parameter(basis)) => Some(basis), _ => None }));
        assert_eq!(parameter.is_some(), expected, "{body}");
        if let Some(basis) = parameter {
            assert!(basis.i64_source());
            let source = ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap();
            assert!(basis.ready(source));
            for erase_rows in [false, true] {
                source.with_replaced_original_new_evidence_for_test(erase_rows, |source| {
                    assert!(!source.candidate_integer_agreement(basis.binding()));
                    assert!(!source.candidate_i64_source_agreement(basis.binding()));
                    assert!(!basis.ready(source), "cached basis survived removed provider evidence");
                });
                assert!(basis.ready(source), "restored SAME evidence remains valid");
            }
        }
    }
    }
}

#[test]
fn original_return_field_requests_keep_nested_expression_plans_before_qualification() {
    for (body, requested) in [
        ("return me.child.n", true),
        ("return me.child.n + me.child.n", true),
        ("if true { return me.child.n + 1 } return me.child.n", true),
        ("return 1 + 2", false),
        ("local other = new Leaf() return other.n", false),
    ] {
        // A Bool Birth store deliberately prevents numeric qualification.
        // Passive original source requests must survive that refusal.
        let text = format!("box Leaf {{ n: i64 birth() {{ me.n = true }} }}
            box Parent {{ child: Leaf = new Leaf() birth() {{ }} read() {{ {body} }} }}
            static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let contract = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                if key.owner() == "Parent" && key.name() == "read")).unwrap();
        let originals = original::prepare_original_returns_v1(package.batch(), &package.selected, &package.parameter_contracts);
        let loan = originals[&contract.owner].as_ref().unwrap();
        // Computed non-field Returns also retain passive plans now; only
        // the original field request belongs to this field-authority test.
        let field_loan = loan.as_ref().filter(|loan| loan.field_plans().next().is_some());
        assert_eq!(field_loan.is_some(), requested, "{body}");
        if let Some(loan) = field_loan {
            assert_eq!(loan.field_plans().count(), if body.contains("if true") { 2 } else { 1 });
            for plan in loan.field_plans() {
                assert!(Rc::ptr_eq(loan.plan(plan.nodes()[0].site()).unwrap(), plan));
            }
            assert!(package.ordinary_new_claim_ledger.borrowed_i64_results[&contract.owner].is_err(),
                "passive source request cannot grant numeric result: {body}");
        }
    }
    for (body, class) in [
        ("if true { return false } return true", BorrowedResultClassV1::Bool),
        ("if true { return 7 } return 1", BorrowedResultClassV1::I64),
    ] {
        let text = format!("box Counter {{ birth() {{ }} read(p) {{ {body} }} }}
            static box Main {{ main() {{ local c = new Counter() local s = \"text\" c.read(s.substring(0, s.length())) return 0 }} }}");
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let owner = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == "read")).unwrap().owner;
        let results = &mut Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap().borrowed_i64_results;
        let proof = results.get_mut(&owner).unwrap().as_mut().unwrap();
        assert_eq!(proof.class, class);
        assert_eq!(proof.returns.len(), 2);
        assert!(proof.original_return.as_ref().unwrap().is_literal_scalar_v1());
        proof.verify_original_return_v1().unwrap();
        // A shared original loan must not make a lost Return invisible merely
        // because this source has no field expressions to corroborate.
        proof.source_facts = Rc::new((*proof.source_facts).clone());
        proof.source_facts_mut_for_test().returns = proof.returns[..1].to_vec().into_boxed_slice();
        assert!(proof.verify_original_return_v1().is_err());
    }
}

#[test]
fn original_field_request_rejects_lost_retained_return_before_lending() {
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Counter { n: i64 birth() { me.n = 0 } read() { return me.n } }
         static box Main { main() { local c = new Counter() return 0 } }").unwrap();
    let contract = package.parameter_contracts.iter().find(|row| matches!(
        package.selected.key_for_batch_slot(row.batch_slot),
        Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == "read")).unwrap().clone();
    let mut results = std::mem::take(&mut Rc::get_mut(&mut package.ordinary_new_claim_ledger)
        .unwrap().borrowed_i64_results);
    let proof = results[&contract.owner].as_ref().unwrap();
    let receiver = proof.original_return.as_ref().unwrap().receiver();
    let site = OwnedExprSiteV1::new(contract.owner, proof.expressions[0].plan().nodes()[0].site().clone());
    let crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Shape(
        BodyExpressionShapeV1::FieldAccess { object, .. }) = proof.expressions[0].plan().nodes()[0].source()
        else { panic!("original field") };
    let object = object.clone();
    for erase in [false, true] {
        if erase {
            results.get_mut(&contract.owner).unwrap().as_mut().unwrap()
                .source_facts_mut_for_test().original_return = None;
        }
        let loan = package.batch().with_lowering_input(contract.batch_slot, |input| {
            let class = package.batch().ordinary_box_coverage().row_for("Counter").unwrap().unwrap();
            super::super::integer_field::lend_integer_field_v1(&results,
                package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap(),
                input, package.instance_constructors(), Some((receiver, class)),
                &site, &object, receiver, receiver, "n")
        }).unwrap();
        if erase {
            assert!(loan.unwrap_err().contains("integer-field/original-return-missing"));
        } else { assert!(loan.unwrap().is_some()); }
    }
}

#[test]
fn nested_result_fields_keep_object_identity_separate_from_numeric_writers() {
    for (body, text_only) in [
        ("local out = \"雪\" return out", true),
        ("local out = \"雪\" out = \"🐱\" return out", true),
        ("local out = \"\" local i = 0 loop(i < 2) { out = out + \"x\" i = i + 1 } return out", true),
        ("local out = 7 return out", false),
        ("local out = \"雪\" out = 7 return out", false),
    ] {
        let text = format!("box Counter {{ birth() {{ }} read() {{ {body} }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let owner = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.owner() == "Counter" && key.name() == "read")).unwrap().owner;
        let original = ledger.borrowed_i64_results.original_return_source_v1(&owner).unwrap().unwrap();
        assert_eq!(original.is_text_return_v1(ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap()), text_only, "{body}");
        let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        assert!(!original.is_text_return_v1(foreign.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap()),
            "an equal-shaped foreign flow cannot lend this local Return class");
    }
    // All original writer RHSs, rather than only Birth, decide this class.
    for (replacement, expected) in [("\"雪\"", true), ("7", false)] {
        let text = format!("box Page {{ init {{ value }} birth() {{ me.value = \"\" }}
            change() {{ me.value = {replacement} }} }}
            static box Main {{ main() {{ local page = new Page() return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let field = ledger.field_value_source("Page", "value").unwrap().as_ref().unwrap();
        assert_eq!(field.writers().count(), 2);
        let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let sum = ingress.source_field_value_sum_v1(&ledger.field_write_claims, field).expect("all writer RHSs close");
        assert_eq!(sum.text_only(), expected, "replacement={replacement}: {sum:?}");
        let text_source = crate::mir::normal_callable_semantic_package::TextFieldValueSourceV1::lend(
            ingress, &ledger.field_write_claims, field);
        assert_eq!(text_source.is_some(), expected);
        if let Some(source) = &text_source {
            assert_eq!(source.field(), field.field());
            assert!(source.recheck(ingress, &ledger.field_write_claims, "value"));
            assert!(!source.recheck(ingress, &ledger.field_write_claims, "other"));
        }
        if expected {
            let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
            let other = &foreign.ordinary_new_claim_ledger;
            assert!(!text_source.as_ref().unwrap().recheck(ingress, &other.field_write_claims, "value"));
            let catalog = crate::mir::callable_parameter_contract::issue_callable_parameter_contract_v1(package.batch()).unwrap();
            let entries = crate::mir::resolved_semantics::CallableHomeAbiIssuerV1::issue_source_entry_home_catalog_v1(package.batch(), &catalog).unwrap();
            let contract = package.parameter_contracts.iter().find(|row| matches!(
                package.selected.key_for_batch_slot(row.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                    if key.owner() == "Page" && key.name() == "change")).unwrap();
            let home = entries.for_batch_slot(contract.batch_slot).unwrap();
            let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                &package.selected, package.batch(), Some(home), contract.batch_slot).unwrap();
            let entry = package.batch().with_lowering_input(contract.batch_slot, |input| {
                let assignment = &input.body_shape().unwrap().assignment_sources()[0];
                let original = ledger.field_write_claims.original_write_source_v1(input.owner(), assignment.target_site()).unwrap().unwrap();
                let root = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::ExpressionFieldRootSourceV1::issue_entry_v1(
                    package.instance_constructors(), receiver, home, input.owner(), receiver.0, field.field()).unwrap();
                crate::mir::normal_callable_semantic_package::EntryFieldWriteSourceV1::issue(
                    input, original, root, field.field(), None,
                    crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::EntryFieldWriteValueSourceV1::Text(
                        text_source.as_ref().unwrap().clone())).unwrap()
            }).unwrap();
            entry.recheck(ledger).unwrap();
            assert!(entry.recheck(other).is_err());
            let completion = ledger.completion_source_for_owner_v1(contract.owner).unwrap();
            let plan = completion.cleanup().root_flow().unwrap().expression_plans().iter()
                .find(|plan| plan.field_write_frame().is_some()).unwrap();
            assert!(plan.field_write_frame().unwrap().entry().unwrap().text_value().is_some());
            assert!(plan.field_write_rhs().unwrap().is_consistent(entry.source()));
            assert!(std::ptr::eq(plan.evaluation().unwrap(), plan.field_write_rhs().unwrap().evaluation().unwrap()));
            assert!(ingress.source_field_value_sum_v1(&other.field_write_claims,
                other.field_value_source("Page", "value").unwrap().as_ref().unwrap()).is_none());
        }
    }
    for (birth, extra, returned, expected) in [
        ("0", "", "me.child.n + me.child.n", true),
        ("0", "me.child.seed()", "me.child.n + me.child.n", true),
        ("true", "", "me.child.n + me.child.n", false),
        ("0", "", "me.child + 1", false),
    ] {
        let text = format!("box Leaf {{ n: i64 birth() {{ me.n = {birth} }} seed() {{ return 0 }} }}
            box Parent {{ child: Leaf = new Leaf() birth() {{ {extra} }}
                read() {{ return {returned} }} }}
            static box Main {{ main() {{ local leaf = new Leaf() return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let owner = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                if key.owner() == "Parent" && key.name() == "read")).unwrap().owner;
        let context = package.ordinary_new_claim_ledger.borrowed_i64_results.field_candidates.as_ref().unwrap();
        let original_objects = context.object_sources();
        assert!(Rc::ptr_eq(original_objects, package.ordinary_new_claim_ledger.field_write_claims.object_sources()));
        assert!(original_objects[&("Parent".into(), "child".into())].field().is_some());
        if !extra.is_empty() {
            assert!(package.instance_constructors().rows().iter().find(|row| row.box_name() == "Parent")
                .unwrap().construction().is_err(), "source class is independent of whole Birth admission");
        }
        let result = &package.ordinary_new_claim_ledger.borrowed_i64_results[&owner];
        assert_eq!(result.is_ok(), expected, "{birth}/{extra}: {result:?}");
        if expected {
            let proof = result.as_ref().unwrap();
            proof.require_source_i64_v1().unwrap();
            let expression = &proof.expressions[0];
            let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            assert!(ingress.source_field_value_sum_v1(&package.ordinary_new_claim_ledger.field_write_claims,
                &context.sources()[&("Leaf".into(), "n".into())]).is_some_and(|sum| sum.signed_only()));
            assert_eq!(expression.fields().cloned().collect::<BTreeSet<_>>(),
                BTreeSet::from([("Leaf".into(), "n".into())]));
            assert!(!proof.field_component.as_ref().unwrap().fields.contains(&("Parent".into(), "child".into())),
                "intermediate object Identity must never enter numeric closure");
            assert!(expression.fields_match(context.sources(), original_objects));
            let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
            assert!(!expression.fields_match(context.sources(),
                foreign.ordinary_new_claim_ledger.field_write_claims.object_sources()),
                "equal-shaped foreign object source cannot replace original proof");
            let mut erased = original_objects.as_ref().clone();
            erased.remove(&("Parent".into(), "child".into()));
            assert!(!expression.fields_match(context.sources(), &Rc::new(erased)),
                "an erased original object edge cannot retain numeric result credit");
        }
    }
    // A get keeps its Null alternative unless the same original count guard
    // proves the constant index on the surviving path. This is source proof,
    // independent of whether an unused helper has a physical Return packet.
    for (body, expected) in [
        ("if me.values.length() == 0 { return \"\" } return me.values.get(0)", true),
        ("if me.values.length() <= 2 { return \"\" } return me.values.get(2)", true),
        ("return me.values.get(0)", false),
        ("if me.other.length() == 0 { return \"\" } return me.values.get(0)", false),
        ("if me.values.length() == 0 { local unused = 0 } return me.values.get(0)", false),
        ("if me.values.length() == 0 { return \"\" } me.values.push(\"y\") return me.values.get(0)", false),
        ("if me.values.length() == 0 { return \"\" } return me.values.get(1)", false),
        ("if me.values.length() == 0 { return \"\" } local i = 0 loop(i < 1) { local v = me.values.get(0) me.values.push(\"y\") i = i + 1 } return \"\"", true),
    ] {
        let text = format!("box Page {{ init {{ values, other }} birth() {{ me.values = new ArrayBox() me.other = new ArrayBox() }}
            seed() {{ me.values.push(\"x\") me.other.push(\"x\") }} read() {{ {body} }} }}
            static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let owner = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key.owner() == "Page" && key.name() == "read")).unwrap().owner;
        let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let original = ledger.borrowed_i64_results.original_return_source_v1(&owner).unwrap().unwrap();
        assert_eq!(original.is_text_return_v1(ingress), expected, "{body}");
        for (site, method) in package.batch().with_lowering_input(
            package.parameter_contracts.iter().find(|row| row.owner == owner).unwrap().batch_slot,
            |input| input.function().method_calls().map(|(site, method)| (site.clone(), method.selector().to_owned())).collect::<Vec<_>>(),
        ).unwrap() {
            if method != "get" { continue; }
            if body.contains("loop(") {
                let core = ingress.ordinary_core_source_v1(&OwnedExprSiteV1::new(owner, site)).unwrap();
                assert!(!core.operands().field_array_access_v1().unwrap().get_is_in_bounds_v1(),
                    "a guard outside a repeated container does not certify each iteration");
            }
        }
    }
}

#[test]
fn forwarded_constructor_field_preserves_original_object_class() {
    let program = "box Handle { birth() {} } box Maker { birth() {} make(flag) { if flag == 0 { return null } return new Handle() } } box Chunk { init { handle } birth(h) { me.handle = h } } static box Main { main() { local m = new Maker() local h = m.make(1) if h == null { return 0 } local c = new Chunk(h) return 0 } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(program).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger.field_value_source("Chunk", "handle").unwrap().as_ref().unwrap();
    let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let sum = ingress.source_field_value_sum_v1(&ledger.field_write_claims, source)
        .expect("the exact constructor input closes the complete field writer sum");
    assert_eq!(sum.nullable_object_class_v1(), Some(("Handle", false)));
}

#[test]
fn direct_map_receiver_numeric_writes_close_the_original_callee_effects() {
    for (birth, update, accepted) in [("0", "me.value = me.value + 1", true),
        ("\"\"", "me.value = \"changed\"", false)] {
        let program = format!("box Item {{ init {{ value }} birth() {{ me.value = {birth} }} read() {{ {update} return 1 }} }}
            box Holder {{ init {{ stats }} birth() {{ me.stats = new MapBox() }}
              find(key) {{ if me.stats.has(key) {{ return me.stats.get(key) }} local item = new Item() me.stats.set(key, item) return item }}
              probe(key) {{ local item = me.stats.get(key) if item == null {{ return 0 }} local n = item.read() return n }} }}
            static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&program).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let targets: Vec<_> = ledger.lexical_instance_calls.borrow().values().filter_map(|slot| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row) => Some(row.source_target()),
            LexicalInstanceCallDispositionSlotV1::SourcePending(source) => Some(source),
            _ => None,
        }.filter(|source| source.target().owner() == "Item" && source.target().name() == "read"
            && source.direct_map_receiver_source_v1().is_some()).cloned()).collect();
        assert_eq!(targets.len(), 1);
        let completion = ledger.completion_source_for_owner_v1(targets[0].callee_owner()).unwrap();
        let flow = completion.cleanup().root_flow().unwrap();
        let plans: Vec<_> = flow.expression_plans().iter().filter(|plan| plan.field_write_frame().is_some()).collect();
        assert_eq!(plans.len(), 1);
        let entry = plans[0].field_write_frame().unwrap().entry().unwrap();
        assert!(plans[0].field_write_rhs().is_some());
        entry.recheck(ledger).unwrap();
        assert_eq!(entry.is_numeric_v1(), accepted);
        assert_eq!(ledger.check_direct_map_numeric_effects_v1(&targets[0]).is_ok(), accepted);
    }
}

#[test]
fn array_set_and_push_join_original_element_sources() {
    use crate::mir::normal_callable_semantic_package::SourceValueAlternativeV1;
    let source = "box Sink { birth() {} accept(value) { return 0 } }
        box Page { init { values } birth() { me.values = new ArrayBox() }
          seed() { me.values.push(\"x\") me.values.set(0, 7) }
          read() { local sink = new Sink() local value = me.values.get(0) local out = sink.accept(value) return 0 } }
        static box Main { main() { local page = new Page() page.seed() local out = page.read() return 0 } }";
    for source in [source.to_string(), source.replace("push(\"x\")", "push(true)")] {
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&source).unwrap();
        let contract = package.parameter_contracts.iter().find(|row| matches!(
            package.selected.key_for_batch_slot(row.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
                if key.owner() == "Sink" && key.name() == "accept"
        )).expect("original sink contract");
        let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let sum = ingress.source_value_sum_v1(contract.parameters[0].binding).expect("closed original element sum");
        assert!(sum.alternatives().contains(&SourceValueAlternativeV1::Text), "strict out-of-bounds can produce Text");
        if source.contains("push(true)") { assert!(sum.alternatives().contains(&SourceValueAlternativeV1::Bool)); }
        assert!(sum.alternatives().contains(&SourceValueAlternativeV1::SignedI64));
        assert!(sum.alternatives().contains(&SourceValueAlternativeV1::Null), "out-of-bounds remains nullable");
    }
}

#[test]
fn array_write_uses_original_formal_field_source() {
    use crate::mir::normal_callable_semantic_package::SourceValueAlternativeV1;
    let source = "box Handle { block_id: i64 birth() { me.block_id = 7 } }
        box Sink { birth() {} accept(value) { return 0 } }
        box Page { init { values } birth() { me.values = new ArrayBox() }
          seed(handle: Handle): i64 { if handle == null { return 0 } me.values.push(\"x\") me.values.set(0, handle.block_id) return 0 }
          qualified(): i64 { local h = new Handle() local out = me.seed(h) return 0 }
          read() { local sink = new Sink() local value = me.values.get(0) local out = sink.accept(value) return 0 } }
        static box Main { main() { return 0 } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    let contract = package.parameter_contracts.iter().find(|row| matches!(
        package.selected.key_for_batch_slot(row.batch_slot),
        Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key))
            if key.owner() == "Sink" && key.name() == "accept"
    )).expect("original sink contract");
    let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let sum = ingress.source_value_sum_v1(contract.parameters[0].binding).expect("closed formal-field element sum");
    assert!(sum.alternatives().contains(&SourceValueAlternativeV1::SignedI64));
    assert!(sum.alternatives().contains(&SourceValueAlternativeV1::Text));
    assert!(sum.alternatives().contains(&SourceValueAlternativeV1::Null));
}
