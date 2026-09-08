//! Actual callable Local lowering from one source issuance, below the install Stop.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::instruction::{InvokeOperation, MapInvokeOperation};
use crate::mir::{MirBuilder, MirInstruction};

#[test]
fn map_callable_dependency_preserves_opaque_local_and_alias_identity() {
    for body in [
        "local m = %{} return 30",
        "local m = %{} local alias = m local again = alias return 30",
        "local a = new Page() local m = %{\"a\" => a} return 30",
        "local m = %{} local a = new Page() return 30",
    ] {
        let source = format!("box Page {{}} static box Main {{ main() {{ {body} }} }}");
        let package = issue(&source).unwrap();
        let main = package
            .declaration_catalog()
            .source_backed_app_main()
            .unwrap();
        let declaration = package
            .batch()
            .declarations()
            .find(|row| row.identity().same_as(main.parser_identity()))
            .unwrap();
        let mut builder = MirBuilder::new();
        let function = package
            .batch()
            .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
                builder.lower_map_dependency_for_test(
                    input,
                    SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                    main.parser_identity(),
                    identity.method_source_observation().cloned(),
                    std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                )
            })
            .unwrap()
            .unwrap_or_else(|e| panic!("{body}: {e}"));
        let mut maps = Vec::new();
        for block in function.blocks.values() {
            for instruction in block.all_instructions() {
                if let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction {
                    if function.blocks[invoke_block].all_instructions().any(|i| {
                        matches!(
                            i,
                            MirInstruction::Invoke {
                                operation: InvokeOperation::Map(MapInvokeOperation::New),
                                ..
                            }
                        )
                    }) {
                        maps.push(*dst);
                    }
                }
            }
        }
        assert_eq!(maps.len(), 1, "{body}");
        assert!(package.ordinary_new_claim_ledger.map_demands_consumed());
        crate::mir::verification::MirVerifier::new_strict()
            .verify_function(&function)
            .unwrap_or_else(|e| panic!("{body}: {e:?}"));
        package
            .ordinary_new_claim_ledger
            .validate_finalized_new_root(&function)
            .unwrap_or_else(|e| panic!("{body}: root validation: {e}"));
        let mut drifted = function.clone();
        let mut changed = false;
        for block in drifted.blocks.values_mut() {
            for instruction in &mut block.instructions {
                if matches!(instruction, MirInstruction::InvokeNormalResult { dst, .. }
                    if *dst == maps[0])
                {
                    *instruction = MirInstruction::Copy {
                        dst: maps[0],
                        src: maps[0],
                    };
                    changed = true;
                }
            }
        }
        assert!(changed);
        assert!(package
            .ordinary_new_claim_ledger
            .validate_new_emissions(declaration.owner(), &drifted)
            .is_err());

        assert!(
            !function
                .blocks
                .values()
                .flat_map(|b| b.all_instructions())
                .any(|i| matches!(i, MirInstruction::Copy { src, .. } if maps.contains(src))),
            "{body}"
        );
    }
}

#[test]
fn map_physical_preflight_rejects_foreign_site_and_pending_install() {
    let source = "static box Main { main() { local m = %{} local other = 1 return 30 } }";
    let package = issue(source).unwrap();
    let foreign = issue(source).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let map = ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap()
        .maps()[0]
        .complete()
        .unwrap();
    let foreign_completion = foreign.ordinary_new_claim_ledger.root_completion_for_test();
    let foreign_site = foreign_completion.cleanup().root_flow().unwrap().maps()[0].site();
    let declaration = package
        .batch()
        .declarations()
        .find(|d| d.owner() == map.site().owner())
        .unwrap();
    package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            let relation = input
                .function()
                .expression_source()
                .initializers()
                .find(|r| r.initializer_site() == Some(map.site().site()))
                .unwrap();
            let other = input
                .function()
                .expression_source()
                .initializers()
                .find(|r| r.binding() != relation.binding())
                .unwrap();
            assert!(ledger.begin_map_emission(foreign_site, relation).is_err());
            assert!(ledger
                .begin_map_emission(map.site(), other)
                .unwrap_err()
                .contains("map-initializer-source-drift"));
            assert!(!ledger.map_demands_consumed());
            ledger.begin_map_emission(map.site(), relation).unwrap();
            assert!(ledger
                .begin_map_emission(map.site(), relation)
                .unwrap_err()
                .contains("map-duplicate-emission"));
            let crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, ordinal } =
                relation.declaration_site()
            else {
                panic!("source local");
            };
            let value = crate::mir::ValueId(99);
            assert!(!ledger.map_initializer_matches(map.site(), relation.binding(), value));
            assert!(ledger
                .complete_local_installation(
                    input.owner(),
                    statement.node(),
                    &[(relation.binding(), *ordinal, value, value)]
                )
                .unwrap_err()
                .contains("local-initializer-mismatch"));
            assert!(!ledger.is_installed_map_binding(relation.binding(), value));
            assert!(!ledger.map_demands_consumed());
        })
        .unwrap();
}
