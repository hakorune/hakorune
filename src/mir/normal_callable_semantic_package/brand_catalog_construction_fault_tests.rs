//! Source authority for partial Birth cleanup, independent of MIR order.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::super::OwnedFieldChildKindV1;
use super::issue_with_brand_catalog;

#[test]
fn construction_store_fault_inventory_is_prior_committed_source_order() {
    for class in ["Inner", "ArrayBox"] {
        let source = format!(
            "box Inner {{ value: i64 = 0 birth() {{ }} }}
             box Holder {{
                 left: {class} = new {class}()
                 right: {class} = new {class}()
                 marker: i64 = 0
                 birth() {{ }}
             }}
             static box Main {{ main() {{ local holder = new Holder() return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source).unwrap();
        let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        let [claim] = claims.as_slice() else {
            panic!("one Holder claim: {claims:?}")
        };
        let plan = claim.construction().as_ref().unwrap();
        let stores = plan.stores();
        assert_eq!(stores.len(), 3);
        for (store, expected) in stores.iter().zip([vec![], vec![0], vec![1, 0]]) {
            let actual: Vec<_> = store
                .fault_discharge()
                .iter()
                .map(|child| child.field.declaration_ordinal())
                .collect();
            assert_eq!(actual, expected);
            for child in store.fault_discharge() {
                assert_eq!(
                    matches!(child.kind, OwnedFieldChildKindV1::Array),
                    class == "ArrayBox"
                );
            }
        }
    }
}

#[test]
fn caller_provided_object_store_seals_prior_owned_cleanup() {
    let package = issue_with_brand_catalog(
        "box Inner { value: i64 = 0 birth() { } }
         box Holder { left: Inner = new Inner() right: ArrayBox = new ArrayBox()
             inner: Inner marker: i64
             birth(inner) { me.inner = inner me.marker = 1 } }
         static box Main { main() {
             local inner = new Inner() local holder = new Holder(inner) return 0
         } }",
    )
    .expect("valid source retains caller-provided object construction");
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claim = rows
        .values()
        .find(|claim| claim.class() == "Holder")
        .expect("Holder source claim");
    let plan = claim.construction().as_ref().unwrap();
    assert_eq!(plan.stores().len(), 4);
    for (store, expected) in plan.stores().iter().zip([vec![], vec![0], vec![1, 0], vec![1, 0]]) {
        assert_eq!(store.fault_discharge().iter().map(|child|
            child.field.declaration_ordinal()).collect::<Vec<_>>(), expected);
    }
    assert!(matches!(plan.stores()[2].rhs(),
        super::super::ConstructionStoreRhsV1::Parameter { provided: Some(_), .. }));
    let children = claim.children().expect("Normal parent End keeps all residences");
    assert_eq!(children.iter().map(|child| child.field.declaration_ordinal())
        .collect::<Vec<_>>(), vec![0, 1, 2]);
    let duplicated = issue_with_brand_catalog(
        "box Inner { value: i64 = 0 birth() { } }
         box Holder { left: Inner right: Inner
             birth(inner) { me.left = inner me.right = inner } }
         static box Main { main() { local inner = new Inner()
             local holder = new Holder(inner) return 0 } }",
    ).unwrap();
    assert!(duplicated.ordinary_new_claim_ledger.pending_claims_for_test().values()
        .find(|claim| claim.class() == "Holder").unwrap().construction().is_err(),
        "one provided Home cannot become two owning fields");

}

#[test]
fn allocation_only_local_new_releases_home_without_initializing_fields() {
    use crate::mir::{MirInstruction, instruction::InvokeOperation};
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    // A later allocation belongs to read-time cleanup, even though the
    // earlier writer that proves field readiness did not yet observe it.
    for expression in ["p.left", "p.left + 1", "p.left - 1", "p.left / 2"] {
        let text = format!("box Pair {{ left: i64 right: i64 }} static box Main {{ main() {{
            local p = new Pair() p.left = 7 local q = new Pair()
            local x = {expression} return x }} }}");
        let package = issue_with_brand_catalog(&text).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let completion = ledger.root_completion_for_test();
        let flow = completion.cleanup().root_flow().unwrap();
        let required: Vec<_> = flow.source_expression_fields().collect();
        let [required] = required.as_slice() else { panic!("one original read: {expression}") };
        let writer = required.request().owned_ready.as_ref().unwrap().frame();
        assert_eq!(writer.prior_homes(), &[LiveHomeV1::Stable(required.request().home)]);
        let claims = ledger.pending_claims_for_test();
        let homes: Vec<_> = claims.values().map(|claim| LiveHomeV1::Stable(claim.home_prefix().as_ref().unwrap().destination())).collect();
        assert_eq!(required.prior_homes(), homes.as_slice(), "current allocation order: {expression}");
        assert_eq!(required.prior_homes().len(), 2, "q must join read Fault cleanup");
        let original = flow.expression_plans().iter().find(|plan| std::rc::Rc::ptr_eq(plan.source(), required.plan())).unwrap();
        let loans: Vec<_> = original.scalar_operations().collect();
        assert_eq!(loans.len(), usize::from(expression.contains('-') || expression.contains('/')));
        assert!(loans.iter().all(|loan| std::rc::Rc::ptr_eq(loan.plan(), required.plan())));
    }
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let request = |text: &str| {
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                text, crate::parser::ParserBuildConfig::default()).unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("source lost") };
            crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default())
        };
        let compile = |text: &str| crate::mir::MirCompiler::with_options(false).compile_normal(request(text));
        let result = compile("box Pair { left: i64 right: i64 } static box Main { main() { local p = new Pair() return 0 } }").unwrap();
        assert!(result.verification_result.is_ok(), "{:?}", result.verification_result);
        let main = result.module.get_function("main").unwrap();
        let operations: Vec<_> = main.blocks.values().flat_map(|block| block.all_instructions())
            .filter_map(|row| match row { MirInstruction::Invoke { operation, .. } => Some(operation), _ => None }).collect();
        assert_eq!(operations.iter().filter(|op| matches!(op, InvokeOperation::NewBox { .. })).count(), 1);
        assert_eq!(operations.iter().filter(|op| matches!(op, InvokeOperation::HomeRelease { .. })).count(), 1);
        assert!(!operations.iter().any(|op| matches!(op, InvokeOperation::ReclaimUnpublished { .. }
            | InvokeOperation::FieldSet { .. } | InvokeOperation::FieldGet { .. })));
        for suffix in ["return p.left", "local x = p.left return 0", "return p.left + 1",
            "local q = p return q.left", "local q = p local r = q return r.left",
            "local q = null q = p return q.left", "return p.left.missing",
            "print(p.left) return 0", "if p.left > 0 { return 1 } return 0",
            "if p.left { return 1 } return 0",
            "if (p.left * 2) > 0 { return 1 } return 0",
            "if (p.left & 1) > 0 { return 1 } return 0",
            "local q = p if q.left > 0 { return 1 } return 0"] {
            let text = format!("box Pair {{ left: i64 right: i64 }} static box Main {{ main() {{ local p = new Pair() {suffix} }} }}");
            assert!(compile(&text).is_err(), "allocation never makes an unwritten field readable: {suffix}");
        }
        let mut failures = Vec::new();
        for suffix in ["p.left = 7 return p.left", "p.left = 7 p.left = 9 return p.left",
            "p.left = 7 p.right = 11 return p.left + p.right",
            "p.left = 7 local x = p.left return x",
            "p.left = 7 local x = p.left + 1 return x",
            "p.left = 7 local q = new Pair() local x = p.left + 1 return x",
            "p.left = 7 p.right = p.left + 1 return p.right",
            "p.left = 7 local q = new Pair() p.right = p.left + 1 return p.right"] {
            let text = format!("box Pair {{ left: i64 right: i64 }} static box Main {{ main() {{ local p = new Pair() {suffix} }} }}");
            let result = match compile(&text) {
                Ok(result) => result,
                Err(error) => { failures.push(format!("MIR {suffix}: {error}")); continue; },
            };
            assert!(result.verification_result.is_ok(), "{suffix}: {:?}", result.verification_result);
            let published = crate::mir::MirCompiler::with_options(false).compile_normal_with_published(request(&text),
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "{suffix}: {verification:?}");
                    view.issue_lifecycle_physical_abi_input()?;
                    if suffix.contains("local x") || suffix.contains("p.right = p.left") {
                        let source = view.retained_root_source().expect("original read source");
                        let function = view.module().get_function("main").unwrap();
                        let (read_block, read) = function.blocks.iter().find_map(|(block, body)|
                            body.all_instructions().find(|row| matches!(row,
                                MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. }, .. }))
                                .map(|row| (*block, row.clone()))).unwrap();
                        let MirInstruction::Invoke { normal_landing, fault_landing, fault_frame, .. } = read
                            else { unreachable!() };
                        for mutation in ["skip-cleanup", "extra-cleanup", "wrong-frame"] {
                            let mut changed = function.clone();
                            match mutation {
                                "skip-cleanup" => {
                                    let body = changed.blocks.get_mut(&fault_landing).unwrap();
                                    body.set_terminator(MirInstruction::Jump { target: normal_landing, edge_args: None });
                                    body.successors = body.successors_from_terminator().into_iter().collect();
                                },
                                "extra-cleanup" => {
                                    let body = changed.blocks.get_mut(&fault_landing).unwrap();
                                    body.instructions.push(body.terminator.clone().unwrap());
                                },
                                "wrong-frame" => {
                                    let MirInstruction::Invoke { fault_frame: actual, .. } = changed.blocks
                                        .get_mut(&read_block).unwrap().terminator.as_mut().unwrap() else { panic!("read Invoke") };
                                    *actual = crate::mir::ValueId(fault_frame.0 + 1000);
                                },
                                _ => unreachable!(),
                            }
                            assert!(source.with_finished_owned_field_writes_v1(source.owner(), &changed,
                                |_| Ok(())).is_err(), "{suffix}: source rejects {mutation}");
                        }
                    }
                    Ok(())
                });
            if let Err(error) = published { failures.push(format!("published {suffix}: {error}")); }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    });
}
