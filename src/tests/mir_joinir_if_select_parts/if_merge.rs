use super::helpers::{
    create_if_merge_multiple_pattern_mir, create_if_merge_simple_pattern_mir,
    strict_if_env_guard,
};
use crate::mir::join_ir::lowering::try_lower_if_to_joinir;
use crate::tests::helpers::joinir_env;

#[test]
fn test_if_merge_simple_pattern() {
    use crate::mir::join_ir::JoinInst;

    let _env = strict_if_env_guard();
    joinir_env::set_if_select_on();

    let func = create_if_merge_simple_pattern_mir();
    let entry_block = func.entry_block;
    let result = try_lower_if_to_joinir(&func, entry_block, true, None);

    assert!(
        result.is_some(),
        "Expected simple 2-variable pattern to be lowered to IfMerge"
    );

    if let Some(JoinInst::IfMerge {
        cond,
        merges,
        k_next,
    }) = result
    {
        eprintln!("✅ Simple pattern (2 vars) successfully lowered to IfMerge");
        eprintln!(
            "   cond: {:?}, merges: {} pairs, k_next: {:?}",
            cond,
            merges.len(),
            k_next
        );
        assert_eq!(merges.len(), 2, "Expected 2 MergePairs for x and y");
        assert!(
            k_next.is_none(),
            "Phase 33-7 constraint: k_next should be None"
        );
    } else {
        panic!("Expected JoinInst::IfMerge, got {:?}", result);
    }

    joinir_env::set_if_select_off();
}

#[test]
fn test_if_merge_multiple_pattern() {
    use crate::mir::join_ir::JoinInst;

    let _env = strict_if_env_guard();
    joinir_env::set_if_select_on();

    let func = create_if_merge_multiple_pattern_mir();
    let entry_block = func.entry_block;
    let result = try_lower_if_to_joinir(&func, entry_block, true, None);

    assert!(
        result.is_some(),
        "Expected multiple 3-variable pattern to be lowered to IfMerge"
    );

    if let Some(JoinInst::IfMerge {
        cond,
        merges,
        k_next,
    }) = result
    {
        eprintln!("✅ Multiple pattern (3 vars) successfully lowered to IfMerge");
        eprintln!(
            "   cond: {:?}, merges: {} pairs, k_next: {:?}",
            cond,
            merges.len(),
            k_next
        );
        assert_eq!(merges.len(), 3, "Expected 3 MergePairs for x, y, and z");
        assert!(
            k_next.is_none(),
            "Phase 33-7 constraint: k_next should be None"
        );
    } else {
        panic!("Expected JoinInst::IfMerge, got {:?}", result);
    }

    joinir_env::set_if_select_off();
}
