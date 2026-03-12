//! Phase 67: P3-C GenericTypeResolver 実利用テスト
//!
//! ArrayBox.get / MapBox.get などのジェネリック型メソッドの
//! 型推論が GenericTypeResolver 経由で正しく動作することを検証。
//!
//! # テスト方針
//!
//! 1. MIR を直接構築して if 文 + ArrayBox.get パターンを作成
//! 2. lifecycle.rs での型推論が正しく動作することを確認
//! 3. A/B テスト：旧経路と新経路で同じ結果になることを検証

use crate::mir::join_ir::lowering::generic_type_resolver::GenericTypeResolver;
use crate::mir::join_ir::lowering::type_hint_policy::TypeHintPolicy;
use crate::mir::MirType;

/// Phase 67-1: GenericTypeResolver の基本判定テスト
#[test]
fn phase67_generic_type_resolver_is_generic_method() {
    let array_type = MirType::Box("ArrayBox".to_string());
    let map_type = MirType::Box("MapBox".to_string());

    // P3-C 対象メソッド（ジェネリック型）
    assert!(GenericTypeResolver::is_generic_method(&array_type, "get"));
    assert!(GenericTypeResolver::is_generic_method(&array_type, "pop"));
    assert!(GenericTypeResolver::is_generic_method(&map_type, "get"));

    // P3-A/P3-B 対象（非ジェネリック）
    assert!(!GenericTypeResolver::is_generic_method(&array_type, "size"));
    assert!(!GenericTypeResolver::is_generic_method(&array_type, "push"));
    assert!(!GenericTypeResolver::is_generic_method(&map_type, "has"));
}

/// Phase 67-1: TypeHintPolicy と GenericTypeResolver の連携テスト
#[test]
fn phase67_type_hint_policy_p3c_integration() {
    // P1/P2/P3-A/P3-B 対象関数は P3-C ではない
    assert!(!TypeHintPolicy::is_p3c_target("IfSelectTest.simple/0")); // P1
    assert!(!TypeHintPolicy::is_p3c_target("IfMergeTest.simple/0")); // P2
    assert!(!TypeHintPolicy::is_p3c_target("read_quoted_from/1")); // P3-A
    assert!(!TypeHintPolicy::is_p3c_target("NewBoxTest.array/0")); // P3-B

    // 一般関数は P3-C 候補
    assert!(TypeHintPolicy::is_p3c_target("ArrayProcessor.process/1"));
    assert!(TypeHintPolicy::is_p3c_target(
        "GenericTypeGetTest.array_get/0"
    ));

    // is_target と is_p3c_target は排他的
    let p3c_func = "GenericTypeGetTest.array_get/0";
    assert!(!TypeHintPolicy::is_target(p3c_func));
    assert!(TypeHintPolicy::is_p3c_target(p3c_func));
}

/// Phase 67-3: A/B テスト準備 - 型推論経路の一致確認
///
/// GenericTypeResolver::resolve_from_phi() と
/// if_phi::infer_type_from_phi() が同じ結果を返すことを確認
#[test]
fn phase67_ab_test_resolve_from_phi_equivalence() {
    use crate::mir::{
        BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, ValueId,
    };
    use std::collections::BTreeMap;

    // 簡単な PHI を含む MIR を構築
    // Block 0 (entry): branch to Block 1 or Block 2
    // Block 1 (then): v2 = 42, jump to Block 3
    // Block 2 (else): v3 = 0, jump to Block 3
    // Block 3 (merge): v4 = phi(v2, v3), return v4

    let sig = FunctionSignature {
        name: "GenericTypeGetTest.simple_phi/0".into(),
        params: vec![],
        return_type: MirType::Unknown, // 型推論対象
        effects: EffectMask::PURE,
    };
    let mut f = MirFunction::new(sig, BasicBlockId::new(0));

    let entry = BasicBlockId::new(0);
    let then_bb = BasicBlockId::new(1);
    let else_bb = BasicBlockId::new(2);
    let merge_bb = BasicBlockId::new(3);

    // Block 構築
    f.add_block(BasicBlock::new(then_bb));
    f.add_block(BasicBlock::new(else_bb));
    f.add_block(BasicBlock::new(merge_bb));

    // Entry: condition + branch
    let cond = f.next_value_id();
    f.get_block_mut(entry)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: cond,
            value: ConstValue::Bool(true),
        });
    f.get_block_mut(entry)
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: cond,
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
        });

    // Then: v2 = 42
    let v2 = f.next_value_id();
    f.get_block_mut(then_bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: v2,
            value: ConstValue::Integer(42),
        });
    f.get_block_mut(then_bb)
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: merge_bb,
            edge_args: None,
        });

    // Else: v3 = 0
    let v3 = f.next_value_id();
    f.get_block_mut(else_bb)
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: v3,
            value: ConstValue::Integer(0),
        });
    f.get_block_mut(else_bb)
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: merge_bb,
            edge_args: None,
        });

    // Merge: v4 = phi(v2 from then, v3 from else)
    let v4 = f.next_value_id();
    f.get_block_mut(merge_bb)
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: v4,
            inputs: vec![(then_bb, v2), (else_bb, v3)],
            type_hint: None, // P3-C: type_hint なし
        });
    f.get_block_mut(merge_bb).unwrap().terminator =
        Some(MirInstruction::Return { value: Some(v4) });

    // 型情報を設定
    let mut types: BTreeMap<ValueId, MirType> = BTreeMap::new();
    types.insert(v2, MirType::Integer);
    types.insert(v3, MirType::Integer);

    // Phase 84-5: if_phi.rs 削除後、GenericTypeResolver のみをテスト
    let result = GenericTypeResolver::resolve_from_phi(&f, v4, &types);

    assert_eq!(
        result,
        Some(MirType::Integer),
        "GenericTypeResolver should infer Integer type from PHI"
    );
}
