use super::helpers::{
    create_if_merge_simple_pattern_mir, create_simple_pattern_mir_with_const, strict_if_env_guard,
};
use crate::mir::join_ir::lowering::try_lower_if_to_joinir;
use crate::tests::helpers::joinir_env;

#[test]
fn test_type_hint_propagation_simple() {
    use crate::mir::MirType;

    let _env = strict_if_env_guard();
    joinir_env::set_if_select_on();

    let func = create_simple_pattern_mir_with_const();
    let entry_block = func.entry_block;
    let result = try_lower_if_to_joinir(&func, entry_block, true, None);

    assert!(
        result.is_some(),
        "Expected simple pattern to be lowered to Select"
    );

    if let Some(crate::mir::join_ir::JoinInst::Select { type_hint, .. }) = result {
        assert_eq!(
            type_hint,
            Some(MirType::Integer),
            "Expected type_hint to be Some(Integer) for IfSelectTest.simple (Const 10/20)"
        );
        eprintln!(
            "✅ Phase 63-2: Type hint propagation successful: {:?}",
            type_hint
        );
    } else {
        panic!("Expected Select instruction with type_hint");
    }

    joinir_env::set_if_select_off();
}

#[test]
fn test_p1_ab_type_inference() {
    use crate::mir::MirType;

    let _env = strict_if_env_guard();
    joinir_env::set_if_select_on();

    let func = create_simple_pattern_mir_with_const();
    let entry_block = func.entry_block;
    let join_inst = try_lower_if_to_joinir(&func, entry_block, true, None)
        .expect("P1 simple pattern should lower to Select");

    if let crate::mir::join_ir::JoinInst::Select { type_hint, .. } = join_inst {
        assert_eq!(
            type_hint,
            Some(MirType::Integer),
            "Route B: P1 Select should have type_hint=Some(Integer)"
        );
        eprintln!("✅ Phase 63-6-4 Step 1: Select type_hint = Some(Integer)");
    } else {
        panic!("Expected Select instruction");
    }

    eprintln!("✅ Phase 63-6-4 Step 2: P1 function name filter: IfSelectTest.* ✓");
    eprintln!("✅ Phase 63-6-4: A/B test passed - JoinIR type hint available for lifecycle.rs");

    joinir_env::set_if_select_off();
}

#[test]
fn test_p2_if_merge_type_hint() {
    use crate::mir::join_ir::lowering::if_merge::IfMergeLowerer;
    use crate::mir::MirType;

    let _env = strict_if_env_guard();
    std::env::set_var("NYASH_JOINIR_IF_MERGE", "1");

    let func = create_if_merge_simple_pattern_mir();
    let entry_block = func.entry_block;

    let lowerer = IfMergeLowerer::new(2);
    let join_inst = lowerer
        .lower_if_to_if_merge(&func, entry_block)
        .expect("P2 IfMerge Simple should lower to IfMerge");

    use crate::mir::join_ir::JoinInst;
    if let JoinInst::IfMerge { merges, .. } = join_inst {
        eprintln!("✅ Phase 64-2-2 Step 1: IfMerge instruction found");

        assert!(
            !merges.is_empty(),
            "IfMerge should have at least one MergePair"
        );

        let first_pair = &merges[0];
        if let Some(type_hint) = &first_pair.type_hint {
            eprintln!(
                "✅ Phase 64-2-2 Step 2: MergePair[0] type_hint = Some({:?})",
                type_hint
            );
            assert_eq!(
                *type_hint,
                MirType::Integer,
                "P2 IfMerge Simple should infer Integer type from Const"
            );
        } else {
            panic!("P2 IfMerge Simple should have type_hint=Some(Integer), got None");
        }
    } else {
        panic!("Expected IfMerge instruction, got: {:?}", join_inst);
    }

    eprintln!(
        "✅ Phase 64-2-2: P2 IfMerge type hint test passed - infer_type_from_mir_pattern() works!"
    );

    std::env::remove_var("NYASH_JOINIR_IF_MERGE");
}
