/// Phase 26-A-5: 統合テスト - ValueId型安全化の完全動作確認
///
/// GUARDバグ再現防止の完全検証:
/// - パラメータ型自動登録（Phase 26-A-3）
/// - is_parameter型安全判定（Phase 26-A-4）
/// - 実際のMIRビルド環境での動作確認
use crate::mir::MirBuilder;
use hakorune_mir_core::{MirValueKind, ValueId};

/// GUARD checkバグ完全再現防止テスト
///
/// ## 背景
///
/// loop_builder.rs で以下のバグがあった：
/// ```rust
/// // ❌ ValueId(0) を「常に未初期化」と誤判定
/// for (name, value) in &current_vars {
///     if value.0 == 0 {  // ← Parameter s=ValueId(0) も弾く！
///         return Ok(ValueId(0));
///     }
/// }
/// ```
///
/// ## 検証内容
///
/// Phase 26-A実装により、ValueId(0)でも正しくパラメータと判定できるようになった
#[test]
fn test_guard_bug_prevention_full_flow() {
    // MirBuilder作成
    let mut builder = MirBuilder::new();

    // Phase 26-A-3: パラメータ型を手動登録（実際は setup_function_params() で自動登録）
    // skip_whitespace(s, idx) を想定
    let s_value_id = ValueId(0);
    let idx_value_id = ValueId(1);

    builder.register_value_kind(s_value_id, MirValueKind::Parameter(0));
    builder.register_value_kind(idx_value_id, MirValueKind::Parameter(1));

    // ローカル変数（Temporary）
    let local_value_id = ValueId(2); // 未登録 → デフォルトTemporary扱い

    // Phase 26-A-2: get_value_kind() で型情報を取得可能
    let s_kind = builder.get_value_kind(s_value_id);
    let idx_kind = builder.get_value_kind(idx_value_id);

    // Phase 26-A-4: is_value_parameter() で型安全判定
    // ✅ GUARD checkバグ修正: ValueId(0) でも正しくパラメータ判定！
    assert!(
        builder.is_value_parameter(s_value_id),
        "ValueId(0) はパラメータ s であり、未初期化ではない！"
    );
    assert!(
        builder.is_value_parameter(idx_value_id),
        "ValueId(1) はパラメータ idx"
    );
    assert!(
        !builder.is_value_parameter(local_value_id),
        "ValueId(2) はパラメータではない"
    );

    // 型情報の詳細検証
    assert_eq!(
        s_kind,
        Some(MirValueKind::Parameter(0)),
        "s は Parameter(0) として登録されている"
    );
    assert_eq!(
        idx_kind,
        Some(MirValueKind::Parameter(1)),
        "idx は Parameter(1) として登録されている"
    );
}

/// 複雑なパラメータパターンのテスト
///
/// インスタンスメソッドの暗黙的 receiver (me) も含む
#[test]
fn test_instance_method_parameters() {
    let mut builder = MirBuilder::new();

    // インスタンスメソッド: Box.process(arg1, arg2)
    // → 実際のパラメータ: (me, arg1, arg2)
    builder.register_value_kind(ValueId(0), MirValueKind::Parameter(0)); // me
    builder.register_value_kind(ValueId(1), MirValueKind::Parameter(1)); // arg1
    builder.register_value_kind(ValueId(2), MirValueKind::Parameter(2)); // arg2

    // receiver (me) = ValueId(0)
    // arg1 = ValueId(1)
    // arg2 = ValueId(2)
    assert!(builder.is_value_parameter(ValueId(0)), "receiver me");
    assert!(builder.is_value_parameter(ValueId(1)), "arg1");
    assert!(builder.is_value_parameter(ValueId(2)), "arg2");
    assert!(!builder.is_value_parameter(ValueId(3)), "not parameter");

    // 型情報確認
    assert_eq!(
        builder.get_value_kind(ValueId(0)),
        Some(MirValueKind::Parameter(0))
    );
    assert_eq!(
        builder.get_value_kind(ValueId(1)),
        Some(MirValueKind::Parameter(1))
    );
    assert_eq!(
        builder.get_value_kind(ValueId(2)),
        Some(MirValueKind::Parameter(2))
    );
}

/// ループ内でのパラメータ/ローカル変数の区別テスト
///
/// loop_builder.rs の実際のユースケース
#[test]
fn test_loop_parameter_vs_local_distinction() {
    let mut builder = MirBuilder::new();

    // Phase 26-A-2: new_typed_value() で各種ValueId作成
    let limit = builder.new_typed_value(MirValueKind::Parameter(0)); // パラメータ
    let i = builder.new_typed_value(MirValueKind::Local(0)); // ローカル変数
    let sum = builder.new_typed_value(MirValueKind::Local(1)); // ローカル変数
    let carrier = builder.new_typed_value(MirValueKind::LoopCarrier); // ループキャリア

    // パラメータ判定
    assert!(
        builder.is_value_parameter(limit.value_id()),
        "limit はパラメータ"
    );
    assert!(
        !builder.is_value_parameter(i.value_id()),
        "i はローカル変数"
    );
    assert!(
        !builder.is_value_parameter(sum.value_id()),
        "sum はローカル変数"
    );
    assert!(
        !builder.is_value_parameter(carrier.value_id()),
        "carrier はループキャリア"
    );

    // 型情報確認
    assert_eq!(
        builder.get_value_kind(limit.value_id()),
        Some(MirValueKind::Parameter(0))
    );
    assert!(builder.get_value_kind(i.value_id()).unwrap().is_local());
    assert!(builder
        .get_value_kind(carrier.value_id())
        .unwrap()
        .is_loop_carrier());
}

/// パラメータなし関数のテスト
#[test]
fn test_no_parameters_function() {
    let builder = MirBuilder::new();

    // 関数: main() - パラメータなし
    // ValueId(0) からすべて未登録（デフォルトTemporary扱い）
    assert!(!builder.is_value_parameter(ValueId(0)));
    assert!(!builder.is_value_parameter(ValueId(1)));
    assert!(!builder.is_value_parameter(ValueId(2)));
}
