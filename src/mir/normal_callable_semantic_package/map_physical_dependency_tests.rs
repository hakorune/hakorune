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
