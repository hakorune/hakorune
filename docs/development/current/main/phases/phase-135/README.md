# Phase 135: ConditionLoweringBox allocator SSOT（ValueId 衝突の根治）

## Status
- 状態: ✅ P0 + P1 実装完了
- スコープ: JoinIR の条件 lowering が `JoinValueSpace` と同一の allocator（SSOT）を使うことを保証 + contract_checks で Fail-Fast 強化

## Problem
`apps/tests/phase133_json_skip_whitespace_min.hako` などで `--verify` が失敗し、MIR に以下の SSA 破綻が出ることがあった:
- `Value %13/%14 defined multiple times`（ループ header PHI dst が後続命令で上書きされる）
- `Value %18 defined multiple times`（同一 JoinIR ValueId への alias binding が Copy を重複注入する）

## Root Cause
1. `ExprLowerer` が `ConditionLoweringBox` 実装で `ConditionContext.alloc_value` を無視し、内部カウンタで ValueId を発行していた。  
   → JoinIR 内で `main()` params（例: `ValueId(1000), ValueId(1001)`）と衝突し、merge の remap で header PHI dst に書き込む命令が生成される。
2. `JoinInlineBoundary` の `condition_bindings` に同一 `join_value` が複数名で登録される場合があり、entry block への Copy 注入が同じ `dst` に重複する。  
   → MIR SSA を破壊する（`copy dst` が 2 回発生）。

## Fix
- `ConditionLoweringBox` は `ConditionContext.alloc_value`（SSOT allocator）を必ず使う。
  - `ConditionLoweringBox::lower_condition` は `&mut ConditionContext` を受け取る（allocator の正当な可変借用のため）。
  - `condition_lowerer::lower_condition_to_joinir` は `&mut dyn FnMut() -> ValueId` を受理する。
- `BoundaryInjector` は `condition_bindings` 注入を `dst` で重複排除し、異なる source が同一 dst に来る場合は Fail-Fast。

## Acceptance (P0)
- `./target/release/hakorune --verify apps/tests/phase133_json_skip_whitespace_min.hako` が PASS
- `./target/release/hakorune --dump-mir apps/tests/phase133_json_skip_whitespace_min.hako` のループ header で PHI dst の再定義がない

---

## P1: Contract Checks 強化（Fail-Fast）

### 目的
Phase 135 P0 の根治を「二度と破れない」ように、JoinIR merge 時の contract violation を早期検出する Fail-Fast を追加。

### 実装内容

#### Step 1: `verify_condition_bindings_consistent`
**場所**: `src/mir/builder/control_flow/joinir/merge/contract_checks.rs`

**契約**:
- condition_bindings は alias（同一 join_value に複数名）を許容
- ただし、同一 join_value が異なる host_value に紐付く場合は即座に Fail-Fast

**検出例**:
```text
[JoinIRVerifier/Phase135-P1] condition_bindings conflict:
  join_value ValueId(104) mapped to both ValueId(12) and ValueId(18)
  Contract: Same join_value can have multiple names (alias) but must have same host_value.
```

#### Step 2: `verify_header_phi_dsts_not_redefined`
**場所**: `src/mir/builder/control_flow/joinir/merge/contract_checks.rs`

**契約**:
- Loop header PHI dst ValueIds は、PHI 以外の命令で dst として再利用してはいけない
- 違反すると MIR SSA 破壊（PHI dst overwrite）

**検出例**:
```text
[JoinIRVerifier/Phase135-P1] Header PHI dst ValueId(14) redefined by non-PHI instruction in block 3:
  Instruction: Call { dst: Some(ValueId(14)), ... }
  Contract: Header PHI dsts must not be reused as dst in other instructions.
```

### 統合
`src/mir/builder/control_flow/joinir/merge/mod.rs` の `verify_joinir_contracts()` に統合：
1. Step 1 を merge 前に実行（boundary 検証）
2. Step 2 を merge 後に実行（func 検証、header PHI dst set を収集）

### Acceptance (P1)
- ✅ `cargo build --release` 成功
- ✅ `phase135_trim_mir_verify.sh` - PASS
- ✅ 回帰テスト: `phase132_exit_phi_parity.sh` - 3/3 PASS
- ✅ 回帰テスト: `phase133_json_skip_whitespace_llvm_exe.sh` - PASS

### 効果
- **予防**: 今後の Box 実装で allocator SSOT 違反や boundary 矛盾を即座に検出
- **明示的エラー**: `--verify` の汎用的なエラーではなく、Phase 135 固有の契約違反メッセージを出力
- **二度と破れない**: debug build で必ず検出されるため、CI/CD で確実に防げる

