//! Source/physical corruption and the real recursive lowering boundary.
use super::*;
use crate::mir::{BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirType, ValueId};

#[test]
fn owned_rhs_projection_rejects_wrong_literal_and_unowned_effects() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Pair { left: i64 right: i64 } static box Main { main() { local p = new Pair() p.left = 7 p.right = 20 return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let claims = ledger.pending_claims_for_test();
    let owner = claims.values().next().unwrap().site().owner();
    let recipe = ledger.completion_for_owner(owner).unwrap().cleanup().root_flow().unwrap()
        .expression_plans().iter().find_map(|plan| plan.field_write_rhs()).unwrap();
    let block = BasicBlockId(0);
    let mut function = MirFunction::new(FunctionSignature { name: "rhs".into(), params: vec![],
        return_type: MirType::Integer, effects: EffectMask::PURE }, block);
    let make_row = |value| (block, MirInstruction::Const { dst: ValueId(1), value: ConstValue::Integer(value) });
    let make_node = |row, entry, exit| vec![FieldWriteRhsNodeV1 { node: 0, value: ValueId(1),
        entry: Cursor { block, instruction: entry }, exit: Cursor { block, instruction: exit }, proof: FieldWriteRhsProofV1::Literal(row) }];
    let row = make_row(7);
    function.blocks.get_mut(&block).unwrap().instructions.push(row.1.clone());
    let rhs = EmittedFieldWriteRhsV1::new(Rc::clone(recipe), make_node(row.clone(), 0, 1), ValueId(1), ledger, &function).unwrap();
    function.blocks.get_mut(&block).unwrap().instructions.insert(0,
        MirInstruction::Const { dst: ValueId(2), value: ConstValue::Integer(99) });
    assert!(rhs.validate_lowering(ledger, &function).unwrap_err().contains("unowned-effect"));
    function.blocks.get_mut(&block).unwrap().instructions.remove(0);
    function.blocks.get_mut(&block).unwrap().instructions[0] = make_row(9).1;
    let error = EmittedFieldWriteRhsV1::new(Rc::clone(recipe), make_node(make_row(9), 0, 1), ValueId(1), ledger, &function).unwrap_err();
    assert!(error.contains("literal-value"), "{error}");
    function.blocks.get_mut(&block).unwrap().instructions[0] = row.1.clone();
    function.blocks.get_mut(&block).unwrap().instructions.push(MirInstruction::Const { dst: ValueId(2), value: ConstValue::Integer(99) });
    let error = EmittedFieldWriteRhsV1::new(Rc::clone(recipe), make_node(row.clone(), 0, 2), ValueId(1), ledger, &function).unwrap_err();
    assert!(error.contains("unowned-effect"), "{error}");
    let error = EmittedFieldWriteRhsV1::new(Rc::clone(recipe), make_node(row, 2, 1), ValueId(1), ledger, &function).unwrap_err();
    assert!(error.contains("interval-cursor"), "{error}");
}

#[test]
fn owned_rhs_original_recursive_writers_reach_the_explicit_consumer_fence() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        for rhs in ["7", "1 + 6", "9 - 2", "14 / 2", "1 * 7", "15 % 8", "7 % 0", "7 & 3", "7 | 3", "7 ^ 3", "1 << 2", "8 >> 1", "n + 2", "(1 + 6) - (8 / 2)"] {
            let text = format!("box Pair {{ left: i64 right: i64 }} static box Main {{ main() {{ local n = 5 local p = new Pair() p.left = {rhs} p.right = 20 return 0 }} }}");
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(&text,
                crate::parser::ParserBuildConfig::default()).unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("original source lost") };
            let error = crate::mir::MirCompiler::with_options(false).compile_normal(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default())).unwrap_err();
            assert!(error.contains("owned-field-write/physical-consumer-pending"), "{rhs}: {error}");
        }
    });
}
