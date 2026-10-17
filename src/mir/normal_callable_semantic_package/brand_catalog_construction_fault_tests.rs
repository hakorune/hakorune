//! Source authority for partial Birth cleanup, independent of MIR order.
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
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let compile = |text: &str| {
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                text, crate::parser::ParserBuildConfig::default()).unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("source lost") };
            crate::mir::MirCompiler::with_options(false).compile_normal(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()))
        };
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
    });
}
