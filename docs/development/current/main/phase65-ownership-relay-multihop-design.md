# Phase 65: Ownership-Relay Multihop Design

## Overview

Phase 60/64で導入した`relay_path.len() > 1` Fail-Fastを、**段階的に**撤去できるだけの設計を固める（**Phase 65は設計のみ、実装はPhase 66以降**）。

「読むのは自由、管理は直下だけ」の原則を保ちながら、**祖先owned変数への更新が複数階層を跨ぐケース（multihop relay）** を安全に扱う。

---

## Core Definitions (Phase 56からの拡張)

### 1. Relay（中継）の基本定義

**Relay**: 内側スコープが**祖先owned**変数を更新する場合、その更新責務をownerへ昇格させる仕組み。

```nyash
loop outer {
    local total = 0    // owned by outer
    loop inner {
        total++        // relay to outer (inner doesn't own total)
    }
}
// outer の exit PHI で total を merge
```

**不変条件（Phase 56）**:
- `relay_writes = writes \ owned` （書き込まれたが所有されていない変数）
- 各relay変数は唯一のowner scopeを持つ
- Owner以外のスコープは「中継」責務のみ（merge責務はownerのみ）

### 2. Multihop Relay（新定義）

**Multihop Relay**: 内側スコープから祖先ownerまでの**複数階層を跨ぐ**relay。

**例**（3階層）:
```nyash
loop L1 {
    local counter = 0    // owned by L1
    loop L2 {
        loop L3 {
            counter++    // relay L3 → L2 → L1 (multihop)
        }
    }
}
```

**`relay_path` の意味論**:
- `RelayVar.relay_path: Vec<ScopeId>` は**内→外の順**でスコープIDを列挙
- 上記の例では `relay_path = [L3, L2]` （L1はownerなので含まない）
- **Invariant**: `relay_path`の末尾の次がowner（`relay_path.is_empty()` ならwriter自身の親がowner）

**責務の分離**:
| 層 | 責務 |
|---|---|
| **Analyzer** | `relay_path`を**宣言**（どのスコープ経由でrelayするか） |
| **Lowering** | `relay_path`を**実装**（各boundaryでcarrierに昇格、loop_step引数として伝播） |

---

## Multihop Relay の意味論

### 2.1. 中継境界での扱い

**原則**: 各中間ループは、relay変数を**loop_step引数として素通し**する。

**例** (L3 → L2 → L1):
```rust
// L3 loop_step signature:
fn l3_loop_step(counter: i64, ...) -> (i64, bool) {
    // counter を受け取り、+1 して返す
    (counter + 1, true)
}

// L2 loop_step signature:
fn l2_loop_step(counter: i64, ...) -> (i64, bool) {
    // L3の結果を受け取り、そのまま返す（中継）
    let (counter_out, _) = l3_loop_step(counter, ...);
    (counter_out, true)
}

// L1 (owner) exit PHI:
// counter の最終値を merge
```

**Key Design Decision**:
- 中間ループ（L2）は**merge責務を持たない**（素通しのみ）
- Owner（L1）の**exit PHI**でのみmerge
- これにより「読むのは自由、管理は直下だけ」を維持

### 2.2. Carrier化のタイミング

**Question**: 各中間境界でcarrierに昇格するか、一度だけにするか？

**Decision**: **各境界で段階的にcarrier化** （理由：JoinIRの境界設計と整合）

**Rationale**:
- JoinIRの各loop boundar は独立した`loop_step`関数を持つ
- 境界を跨ぐたびに、その時点の変数状態を引数として伝播
- 「一度だけ」にすると、中間ループのsignatureが複雑化（どこで昇格したかを追跡する必要）
- **段階的昇格**なら、各ループは「自分が中継するrelay変数」だけを意識すればよい

**実装**:
```rust
// L3 (innermost writer)
// relay_path = [L3, L2] → L3 exit で counter を carrier として返す
let (counter_from_l3, _) = l3_loop_step(...);

// L2 (intermediate relay)
// L2 も counter を carrier として受け取り、返す
let (counter_from_l2, _) = l2_loop_step(counter_from_l3, ...);

// L1 (owner)
// counter を merge（PHI）
let counter_final = phi(counter_init, counter_from_l2);
```

### 2.3. relay_path の正規化

**Invariant**:
- `relay_path` は**必ずLoopスコープのみ**を含む（Function/Block/Ifは含まない）
- 順序は**内→外**（writer自身は含まない、ownerも含まない）
- 空の`relay_path`は**1-hop relay**（writer直下の親がowner）

**Validation**:
- Analyzerは`relay_path`生成時に上記を検証
- Loweringは`relay_path.len() == 0`と`relay_path.len() > 0`の両方に対応

---

## Merge Relay の意味論

### 3.1. 問題定義

**同一変数に対して、複数のinner loopが更新するケース**:

```nyash
loop outer {
    local total = 0
    if a {
        loop inner1 { total++ }  // relay to outer
    }
    if b {
        loop inner2 { total-- }  // relay to outer
    }
}
// outer の exit PHI で merge
```

**Question**: これは許可するか、Fail-Fastで弾くか？

### 3.2. Design Decision: **PERMIT with Owner Merge**

**決定**: **許可する**（ただしownerのexit PHIでのみmerge）

**Rationale**:
- 複数のinner loopが同じ祖先owned変数を更新するのは**合理的なパターン**
- 例: 条件分岐ごとに異なるループで集計値を更新
- 禁止すると、ユーザーが不自然な回避策（変数を分ける等）を強いられる

**不変条件**:
- Owner scopeの**exit PHI**で全ての分岐からの値をmerge
- 中間ループは**merge責務を持たない**（relay変数を引数として受け渡すだけ）

**実装方針**（JoinIR層）**:
- Owner loopの`loop_step`が、複数のinner loopから返されたrelay変数を受け取る
- Exit PHIで全分岐の最終値をmerge
- 中間ループは「素通し」のみ

**Fail-Fast vs Permit 判定**:
| ケース | 判定 | 理由 |
|---|---|---|
| 同一ownerへの複数relay | **PERMIT** | Owner exit PHIでmerge可能 |
| 異なるownerへのrelay | **Fail-Fast** | Ownershipが曖昧（設計エラー） |
| Relay pathの不整合 | **Fail-Fast** | Analyzer実装バグ（検出すべき） |

### 3.3. Merge Relay 代表例

**例1**: 条件分岐での複数inner loop
```nyash
loop outer {
    local sum = 0
    if phase == 1 {
        loop { sum = sum + data[i] }  // relay to outer
    }
    if phase == 2 {
        loop { sum = sum * 2 }        // relay to outer (different loop)
    }
}
// outer exit PHI: sum = phi(sum_init, sum_from_phase1, sum_from_phase2)
```

**例2**: Multihop + Merge
```nyash
loop L1 {
    local total = 0
    loop L2 {
        loop L3a { total++ }  // relay L3a → L2 → L1
        loop L3b { total-- }  // relay L3b → L2 → L1
    }
}
// L2 は両方のrelay を受け取り、L1 へ転送
// L1 exit PHI で merge
```

---

## Fail-Fast 解除条件

### 4.1. 現状の Fail-Fast

**Phase 58-59 実装**:
```rust
if !plan.relay_writes.is_empty() {
    return Err("relay_writes not supported in this phase");
}
```

**Phase 64 実装**:
```rust
if relay_var.relay_path.len() > 1 {
    return Err("Multi-hop relay not yet supported");
}
```

### 4.2. Phase 65 の方針

**Phase 65は設計のみ** → **Fail-Fastは維持**（解除はPhase 66以降）

### 4.3. Phase 66 解除条件（事前設計）

**解除のための受け入れ基準**:

1. **ユニットテスト**: `plan_to_p2_inputs()` / `plan_to_p3_inputs()` でmultihop relayを含むOwnershipPlanを直接組み立てて変換を検証
2. **統合テスト**: 実際のAST（3階層loop + relay）からOwnershipPlan生成 → lowering inputs変換を検証
3. **既存回帰テスト**: Phase 64のP3統合テストが全てPASS（既存1-hop relayに影響なし）
4. **観測可能性**: Multihop relay発生時に`NYASH_TRACE_VARMAP=1`でログ出力（中継境界を可視化）
5. **偽陽性の上限**: Fail-Fast解除後も、不正なrelay_path（異なるowner等）は検出してErr返却

**段階的解除の流れ**（Phase 66実装時）:
1. `plan_to_p2_inputs()` / `plan_to_p3_inputs()` のFail-Fastを条件付き解除
2. `relay_path.len() > 0` の場合、各中間スコープをloop_step引数に追加（carrier化）
3. Owner scopeのexit PHIでmerge
4. 回帰テストでvalidation

---

## 実装箇所の特定（Phase 66 に向けて）

### 5.1. Analyzer側（Phase 57）

**現状**: `OwnershipAnalyzer::analyze_json()` は既に`relay_path`を生成済み

**変更不要** → Phase 57実装で正しく`relay_path`を構築している

**検証項目**:
- `relay_path`が内→外の順で正しく列挙されているか
- Owner scopeが`relay_path`に含まれていないか
- 空の`relay_path`（1-hop）が正しく処理されているか

### 5.2. plan_to_lowering.rs（Phase 58-59）

**現状のFail-Fast**:
```rust
// src/mir/join_ir/ownership/plan_to_lowering.rs
pub fn plan_to_p2_inputs(...) -> Result<P2LoweringInputs, String> {
    if !plan.relay_writes.is_empty() {
        return Err("relay not supported".to_string());
    }
    // ...
}
```

**Phase 66 変更内容**:
1. Fail-Fastを条件付き解除（`relay_path.len() > THRESHOLD`等）
2. `relay_path`を反復して、各中間スコープ用のcarrier追加
3. Owner scope用のmerge point（exit PHI）情報を生成

**疑似コード**:
```rust
pub fn plan_to_p2_inputs(...) -> Result<P2LoweringInputs, String> {
    let mut carriers = vec![];

    // Owned + written
    for var in plan.carriers() {
        carriers.push(CarrierVar { name: var.name.clone(), ... });
    }

    // Relay variables
    for relay_var in &plan.relay_writes {
        // Multihop support (Phase 66)
        if relay_var.relay_path.len() > 1 {
            // For each intermediate scope in relay_path
            for &scope_id in &relay_var.relay_path {
                // Add carrier for this intermediate scope
                // (詳細は Phase 66 実装時に設計)
            }
        }

        // Owner scope gets merge responsibility
        // (exit PHI generation - 詳細は lowering 層の仕事)
    }

    Ok(P2LoweringInputs { carriers, ... })
}
```

### 5.3. Pattern2/3 Lowering側（Phase 64）

**現状のFail-Fast** (`pattern3_with_if_phi.rs`):
```rust
fn check_ownership_plan_consistency(...) -> Result<(), String> {
    for relay_var in &plan.relay_writes {
        if relay_var.relay_path.len() > 1 {
            return Err("Multi-hop relay not supported".to_string());
        }
    }
    Ok(())
}
```

**Phase 66 変更内容**:
1. Multihop relay受け入れ（Fail-Fast解除）
2. `relay_path`を使って、各中間ループのloop_step signatureを調整
3. Owner loopのexit PHIでrelay変数をmerge

**実装の要点**:
- 各中間ループは`relay_var.name`をloop_step引数に追加
- Owner loopは全分岐からのrelay変数をexit PHIでmerge
- Merge時の順序はcarrier order SSOT（Phase 67以降の課題）

---

## 禁止事項（by-name分岐の排除）

### 6.1. 原則: 構造ベース設計

**Phase 65の目的**: multihop relayを**構造的に**扱う設計を固める。

**禁止**:
- ❌ 変数名による特別扱い（`if var.name == "sum" { ... }`）
- ❌ Dev-only name guard（`NYASH_ALLOW_RELAY_VAR=sum`等）
- ❌ 「黙って最後を採用」型の暗黙merge（不変条件違反）

**許可**:
- ✅ Analyzer が`relay_path`を宣言
- ✅ Lowering がそれを機械的に実装
- ✅ Owner scopeが明示的にmerge（exit PHI）
- ✅ Fail-Fast で不正なパターンを検出（異なるowner等）

### 6.2. Dev-only Name Guardも対象外

**理由**: Multihop relay設計は**本番コードパス**で動作すべき。

Dev-only name guardは：
- テスト用フィクスチャーでのデバッグ補助のみ
- 本番設計には含めない
- Phase 67以降の「carrier order SSOT」で扱う別の問題

---

## 代表ケース

### 7.1. AST例（3階層 multihop）

```nyash
// Example: Nested loop with multihop relay
loop L1 {
    local counter = 0    // owned by L1

    loop L2 {
        local temp = 0   // owned by L2

        loop L3 {
            counter++    // relay L3 → L2 → L1 (multihop)
            temp++       // owned by L2 (no relay)
        }

        print(temp)      // temp は L2 owned
    }

    print(counter)       // counter は L1 owned
}
```

**OwnershipPlan (L3)**:
```rust
OwnershipPlan {
    scope_id: ScopeId(3),  // L3
    owned_vars: [],        // L3は変数を定義していない
    relay_writes: [
        RelayVar {
            name: "counter",
            owner_scope: ScopeId(1),  // L1
            relay_path: [ScopeId(3), ScopeId(2)],  // L3 → L2 → L1
        }
    ],
    captures: [],
    condition_captures: [],
}
```

**OwnershipPlan (L2)**:
```rust
OwnershipPlan {
    scope_id: ScopeId(2),  // L2
    owned_vars: [
        ScopeOwnedVar {
            name: "temp",
            is_written: true,
            is_condition_only: false,
        }
    ],
    relay_writes: [
        RelayVar {
            name: "counter",
            owner_scope: ScopeId(1),  // L1
            relay_path: [ScopeId(2)],  // L2 → L1 (中継)
        }
    ],
    captures: [],
    condition_captures: [],
}
```

**OwnershipPlan (L1)**:
```rust
OwnershipPlan {
    scope_id: ScopeId(1),  // L1
    owned_vars: [
        ScopeOwnedVar {
            name: "counter",
            is_written: true,  // L1がowner（L3からのrelayを受け取る）
            is_condition_only: false,
        }
    ],
    relay_writes: [],  // L1は最外なのでrelay不要
    captures: [],
    condition_captures: [],
}
```

### 7.2. JSON Fixture例（Merge Relay）

```json
{
  "type": "Function",
  "name": "test_merge_relay",
  "body": [
    {
      "type": "Local",
      "name": "sum",
      "value": { "type": "Integer", "value": 0 }
    },
    {
      "type": "Loop",
      "condition": { "type": "Variable", "name": "outer_cond" },
      "body": [
        {
          "type": "If",
          "condition": { "type": "Variable", "name": "branch_a" },
          "then": [
            {
              "type": "Loop",
              "condition": { "type": "Variable", "name": "inner_cond_a" },
              "body": [
                {
                  "type": "Assign",
                  "target": "sum",
                  "value": {
                    "type": "BinOp",
                    "op": "Add",
                    "lhs": { "type": "Variable", "name": "sum" },
                    "rhs": { "type": "Integer", "value": 1 }
                  }
                }
              ]
            }
          ]
        },
        {
          "type": "If",
          "condition": { "type": "Variable", "name": "branch_b" },
          "then": [
            {
              "type": "Loop",
              "condition": { "type": "Variable", "name": "inner_cond_b" },
              "body": [
                {
                  "type": "Assign",
                  "target": "sum",
                  "value": {
                    "type": "BinOp",
                    "op": "Sub",
                    "lhs": { "type": "Variable", "name": "sum" },
                    "rhs": { "type": "Integer", "value": 1 }
                  }
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

**Expected OwnershipPlans**:
- **Inner Loop A**: `relay_writes = [RelayVar { name: "sum", owner: outer_loop, relay_path: [inner_loop_a] }]`
- **Inner Loop B**: `relay_writes = [RelayVar { name: "sum", owner: outer_loop, relay_path: [inner_loop_b] }]`
- **Outer Loop**: `owned_vars = [ScopeOwnedVar { name: "sum", is_written: true }]` （両方のrelayを受け取ってmerge）

---

## まとめ：Phase 65 受け入れ基準

**Phase 65完了条件**:

1. ✅ **用語と不変条件の明文化**: Multihop relay, Merge relayの定義と不変条件を文書化
2. ✅ **relay_pathの意味論**: 内→外の順、Loop scopeのみ、段階的carrier化の決定
3. ✅ **Merge relayの扱い**: PERMIT with owner merge、禁止パターンの明文化
4. ✅ **Fail-Fast解除条件**: Phase 66実装時の受け入れ基準（テスト/観測/偽陽性上限）
5. ✅ **実装箇所の特定**: Analyzer（変更不要）、plan_to_lowering（Phase 66変更点）、Pattern lowering（Phase 66変更点）
6. ✅ **禁止事項の明文化**: by-name分岐排除、dev-only name guard対象外
7. ✅ **代表ケースの提供**: 3階層multihop（AST例）、Merge relay（JSON fixture例）

**Phase 66への引き継ぎ**:
- この文書の「5. 実装箇所の特定」セクションを実装ガイドとして使用
- Fail-Fast段階解除の流れに従って実装
- 回帰テスト（既存Phase 64テスト全PASS）を確認しながら進める

---

## Appendix: 用語集

| 用語 | 定義 |
|---|---|
| **Owner** | 変数を定義したスコープ（唯一のownership） |
| **Carrier** | そのスコープがowned AND writtenな変数（loop_step引数として管理） |
| **Capture** | 祖先scopeの変数への read-only アクセス |
| **Relay** | 祖先owned変数への更新を owner へ昇格させる仕組み |
| **Multihop Relay** | 複数階層を跨ぐrelay（relay_path.len() > 1） |
| **Merge Relay** | 複数のinner loopが同一祖先owned変数を更新するケース |
| **relay_path** | 内→外の順でrelayを経由するスコープIDのリスト（writerのLoop scopeは含む / ownerは含まない） |
| **Exit PHI** | Owner loopの出口でrelay変数をmergeするPHI命令 |
| **Fail-Fast** | 不正なパターンを検出して即座にErrを返す設計方針 |

---

## Phase 66 Implementation Status

**Phase 66 Implementation (2025-12-12)**: ✅ **COMPLETED**

Phase 66では `plan_to_p2_inputs_with_relay` の multihop 受理ロジックを実装完了。

### 実装済みチェック

- [x] `plan_to_lowering.rs` の relay_path.len() > 1 制限撤去
- [x] 構造的 Fail-Fast ガード実装:
  - [x] `relay_path.is_empty()` → Err（loop relay は最低 1 hop）
  - [x] `relay_path[0] != plan.scope_id` → Err（この scope が最初の hop）
  - [x] `relay.owner_scope == plan.scope_id` → Err（relay と owned は排他）
  - [x] `owned_vars ∩ relay_writes ≠ ∅` → Err（同名は不変条件違反）
- [x] ユニットテスト追加:
  - [x] `test_relay_multi_hop_accepted_in_with_relay` (multihop 受理)
  - [x] `test_relay_path_empty_rejected_in_with_relay`
  - [x] `test_relay_path_not_starting_at_plan_scope_rejected`
  - [x] `test_relay_owner_same_as_plan_scope_rejected`
  - [x] `test_owned_and_relay_same_name_rejected`
- [x] `ast_analyzer.rs` に 3階層 multihop テスト追加:
  - [x] `multihop_relay_detected_for_3_layer_nested_loops`

### 検証結果

- normalized_dev: 49/49 PASS
- lib tests: 947/947 PASS
- Zero regressions

### 次フェーズ（Phase 70+）

- **Phase 70-A**: Runtime guard 固定 - [phase70-relay-runtime-guard.md](phase70-relay-runtime-guard.md)
- **Phase 70-B+**: 本番 lowering への multihop 完全対応（boundary/exit PHI のmerge実装）
- Merge relay テスト追加（複数 inner loop → 共通 owner）
