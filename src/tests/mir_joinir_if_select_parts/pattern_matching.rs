use super::helpers::{create_local_pattern_mir, create_simple_pattern_mir};
use crate::mir::join_ir::lowering::try_lower_if_to_joinir;
use crate::tests::helpers::joinir_env;

/// Phase 33-3: 統合パターンマッチングテスト（env 競合回避）
///
/// env 変数を触る4つのテストを1つにまとめて、並列実行でのレースを防ぐ。
/// 順番に: simple/local/disabled/wrong_name を確認する。
#[test]
fn test_if_select_pattern_matching() {
    use crate::tests::helpers::joinir_env::clear_joinir_flags;

    clear_joinir_flags();

    joinir_env::set_if_select_on();

    let func = create_simple_pattern_mir();
    let entry_block = func.entry_block;
    let result = try_lower_if_to_joinir(&func, entry_block, true, None);

    assert!(
        result.is_some(),
        "Expected simple pattern to be lowered to Select"
    );

    if let Some(crate::mir::join_ir::JoinInst::Select {
        dst,
        cond,
        then_val,
        else_val,
        ..
    }) = result
    {
        eprintln!("✅ Simple pattern successfully lowered to Select");
        eprintln!(
            "   dst: {:?}, cond: {:?}, then: {:?}, else: {:?}",
            dst, cond, then_val, else_val
        );
    } else {
        panic!("Expected JoinInst::Select, got {:?}", result);
    }

    let func = create_local_pattern_mir();
    let entry_block = func.entry_block;
    let result = try_lower_if_to_joinir(&func, entry_block, true, None);

    assert!(
        result.is_some(),
        "Expected local pattern to be lowered to Select"
    );

    if let Some(crate::mir::join_ir::JoinInst::Select {
        dst,
        cond,
        then_val,
        else_val,
        ..
    }) = result
    {
        eprintln!("✅ Local pattern successfully lowered to Select");
        eprintln!(
            "   dst: {:?}, cond: {:?}, then: {:?}, else: {:?}",
            dst, cond, then_val, else_val
        );
    } else {
        panic!("Expected JoinInst::Select, got {:?}", result);
    }

    clear_joinir_flags();

    let func = create_simple_pattern_mir();
    let entry_block = func.entry_block;
    let result = try_lower_if_to_joinir(&func, entry_block, false, None);

    if result.is_some() {
        eprintln!("✅ If/Select lowering works under structure-first routing (core always on, no toggles)");
    } else {
        eprintln!("ℹ️ If/Select lowering skipped when toggle bundle is cleared (no dev flags)");
    }

    joinir_env::set_if_select_on();

    let mut func = create_simple_pattern_mir();
    func.signature.name = "WrongName.test/1".to_string();
    let entry_block = func.entry_block;
    let result = try_lower_if_to_joinir(&func, entry_block, true, None);

    assert!(
        result.is_none(),
        "Expected None for non-IfSelectTest functions"
    );

    eprintln!("✅ Function name filter working correctly");

    clear_joinir_flags();
}
