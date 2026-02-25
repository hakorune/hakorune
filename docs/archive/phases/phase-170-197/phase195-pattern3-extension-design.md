# Phase 195: Pattern 3 拡張（if-in-loop + マルチキャリア）

**Status**: Design Phase
**Date**: 2025-12-09
**Prerequisite**: Phase 194 complete (JsonParser deployment & validation)

---

## 目的

**Pattern 3（If-Else PHI）の対応範囲を拡張**し、複数キャリアの条件付き更新を JoinIR で扱えるようにする。

**スコープ**:
- ✅ if 内で完結する複数キャリア更新（2-3個程度）
- ❌ ConditionEnv 拡張は行わない（外部ローカル・digits 系は Phase 200+ 保留）
- ❌ LoopBodyLocal + MethodCall 混在は Phase 195+ に延期

---

## Task 195-1: 対象ループの絞り込み（doc-only）

### 目標
JsonParser / selfhost から「P3 で攻めたいループ」を **1-2 本だけ**選定し、AST 構造を詳細に分析する。

### 候補ループ

#### 候補 1: JsonParser `_parse_string` の簡易版（優先度: 高）

**ループ構造**:
```nyash
// _parse_string (escape 処理の簡略版)
static box JsonParser {
    _parse_string(s, start) {
        local buffer = ""
        local escaped = false
        local i = start

        loop(i < s.length() and s[i] != '"') {
            local ch = s[i]

            if(ch == '\\') {
                escaped = true           // ← Carrier 1 update
            } else {
                buffer = buffer + ch     // ← Carrier 2 update
                escaped = false          // ← Carrier 1 update
            }

            i = i + 1
        }

        return buffer
    }
}
```

**AST 構造**:
```
Loop {
    condition: BinOp(i < len AND s[i] != '"')
    body: [
        LocalVar { name: "ch", init: ArrayAccess(s, i) },  // body-local
        If {
            condition: Compare(ch == '\\'),
            then: [
                Assign { lhs: "escaped", rhs: BoolLiteral(true) }
            ],
            else: [
                Assign { lhs: "buffer", rhs: BinOp(buffer + ch) },
                Assign { lhs: "escaped", rhs: BoolLiteral(false) }
            ]
        },
        Assign { lhs: "i", rhs: BinOp(i + 1) }
    ]
}
```

**キャリア分析**:
| Carrier | Type | Update Pattern | Then Branch | Else Branch |
|---------|------|----------------|-------------|-------------|
| `escaped` | BoolBox | Conditional flag | `true` | `false` |
| `buffer` | StringBox | StringAppend | (unchanged) | `buffer + ch` |
| `i` | IntegerBox | CounterLike | `i + 1` | `i + 1` |

**P3 で扱う範囲**:
- ✅ `escaped`: 条件フラグ（両分岐で定義）
- ✅ `buffer`: StringAppend（else のみ更新、then は不変）
- ✅ `i`: ループ外で統一更新（P3 の外で処理）

**制約**:
- `ch` は body-local 変数（Phase 191 で対応済み）
- `s[i]` の配列アクセスは ConditionEnv で解決（`s` は outer local → Phase 200+）
- **この Phase では `ch = "x"` のような定数で代替してテスト**

#### 候補 2: selfhost の if-sum パターン（優先度: 中）

**ループ構造**:
```nyash
// selfhost: 条件付き集計
static box Aggregator {
    sum_positive(array) {
        local sum = 0
        local count = 0
        local i = 0

        loop(i < array.length()) {
            if(array[i] > 0) {
                sum = sum + array[i]      // ← Carrier 1 update
                count = count + 1         // ← Carrier 2 update
            }
            i = i + 1
        }

        return sum
    }
}
```

**AST 構造**:
```
Loop {
    condition: Compare(i < array.length())
    body: [
        If {
            condition: Compare(array[i] > 0),
            then: [
                Assign { lhs: "sum", rhs: BinOp(sum + array[i]) },
                Assign { lhs: "count", rhs: BinOp(count + 1) }
            ],
            else: []  // 空（更新なし）
        },
        Assign { lhs: "i", rhs: BinOp(i + 1) }
    ]
}
```

**キャリア分析**:
| Carrier | Type | Update Pattern | Then Branch | Else Branch |
|---------|------|----------------|-------------|-------------|
| `sum` | IntegerBox | NumberAccumulation | `sum + array[i]` | (unchanged) |
| `count` | IntegerBox | CounterLike | `count + 1` | (unchanged) |
| `i` | IntegerBox | CounterLike | `i + 1` | `i + 1` |

**P3 で扱う範囲**:
- ✅ `sum`: NumberAccumulation（then のみ更新）
- ✅ `count`: CounterLike（then のみ更新）
- ✅ `i`: ループ外で統一更新

**制約**:
- `array[i]` の配列アクセスは ConditionEnv で解決（`array` は outer local → Phase 200+）
- **この Phase では `i` のような既存パラメータで代替してテスト**

### Phase 195 での選定

**優先順位 1**: 候補 1（_parse_string 簡易版）
- 理由: JsonParser の実戦コード、flag + buffer の2キャリア
- 簡略化: `ch = "x"` 定数で配列アクセス回避

**優先順位 2**: 候補 2（if-sum）
- 理由: selfhost の典型パターン、sum + count の2キャリア
- 簡略化: `i` のみで配列アクセス回避

**成果物**:
- 選定したループの詳細 AST 構造記録（本セクション）
- キャリア分析表（UpdateKind 分類済み）

---

## Task 195-2: LoopUpdateSummary / CarrierInfo の設計整理

### 現状の P3 サポート（Phase 170-189）

**既存の P3 は単一キャリアのみ対応**:
```rust
// 既存の P3 ケース
if(cond) {
    sum = sum + i  // ← 単一キャリア "sum"
} else {
    sum = sum - i
}
```

**LoopUpdateSummary / CarrierInfo の構造**:
```rust
pub struct CarrierInfo {
    pub carriers: Vec<String>,           // キャリア名リスト
    pub updates: HashMap<String, ASTNode>,  // name → update式
}

pub struct LoopUpdateSummary {
    pub kind: UpdateKind,  // CounterLike | NumberAccumulation | ...
    // ...
}
```

### Phase 195 で扱う「複数キャリア + 条件付き更新」

**拡張要件**:
```rust
// Phase 195 の P3 ケース
if(cond) {
    escaped = true      // ← Carrier 1
    // buffer は更新なし（不変）
} else {
    buffer = buffer + ch  // ← Carrier 2
    escaped = false     // ← Carrier 1
}
```

**設計原則**:
1. **両分岐で同じ Carrier が必ず定義**されること（PHI 生成の前提）
   - 片方の分岐で更新なし（不変）の場合、明示的に `carrier = carrier` を挿入
   - OR: PHI 生成時に「更新なし = 前の値を使う」として扱う

2. **各 Carrier の update 式は既存 UpdateKind 範囲内**:
   - CounterLike: `count + 1`, `count - 1`
   - NumberAccumulation: `sum + i`, `sum * base + addend`
   - StringAppend: `buffer + ch`
   - BoolFlag: `true`, `false`（新規 UpdateKind 候補）

3. **CarrierInfo は複数 Carrier を同時に保持**:
   ```rust
   CarrierInfo {
       carriers: vec!["escaped", "buffer"],
       updates: {
           "escaped": ...,  // then/else で異なる式
           "buffer": ...,
       }
   }
   ```

### 制約の明確化

**P3 で扱う Carrier の制約** (Phase 195):
- ✅ if-else の**両分岐で同じ Carrier が定義**される（明示的 or 不変）
- ✅ 各 update 式は**既存 UpdateKind に対応**する
- ❌ MethodCall を含む update は Phase 193 の制約に従う（ループパラメータのみ）
- ❌ 外部ローカル変数（`digits` 等）は Phase 200+ に保留

**例（OK）**:
```nyash
if(ch == '\\') {
    escaped = true           // ✅ BoolFlag
} else {
    buffer = buffer + ch     // ✅ StringAppend
    escaped = false          // ✅ BoolFlag
}
```

**例（NG - Phase 195 範囲外）**:
```nyash
if(cond) {
    digit = digits.indexOf(ch)  // ❌ 外部ローカル (Phase 200+)
    sum = sum + digit
}
```

### 成果物
- 複数キャリア対応の設計原則（本セクション）
- UpdateKind 拡張候補（BoolFlag）の検討
- CarrierInfo 構造の拡張仕様（擬似コード）

---

## Task 195-3: Pattern3 lowerer の設計更新

### 現状の P3 lowerer（Phase 170-189）

**単一キャリアの処理フロー**:
```
1. If-Else AST を検出
2. Then/Else 各分岐で carrier 更新式を抽出
3. JoinIR で Then/Else ブロック生成
4. Merge 点で PHI 命令生成（1つの carrier のみ）
5. ExitLine で PHI 結果を variable_map に接続
```

### Phase 195 での拡張設計

#### 1. 複数 Carrier の同時処理

**設計案**: 複数 PHI を同じ Merge 点で生成

```rust
// 擬似コード: Pattern3 lowerer 拡張

// Step 1: If-Else で更新される全 Carrier を収集
let carriers_in_then = extract_carriers(&if_node.then_branch);
let carriers_in_else = extract_carriers(&if_node.else_branch);
let all_carriers = carriers_in_then.union(&carriers_in_else);

// Step 2: 各 Carrier について Then/Else の update 式を取得
let mut carrier_updates = HashMap::new();
for carrier in all_carriers {
    let then_update = get_update_or_unchanged(&if_node.then_branch, carrier);
    let else_update = get_update_or_unchanged(&if_node.else_branch, carrier);
    carrier_updates.insert(carrier, (then_update, else_update));
}

// Step 3: JoinIR で Then/Else ブロック生成（複数 update を emit）
let then_block = emit_then_branch(&carrier_updates);
let else_block = emit_else_branch(&carrier_updates);

// Step 4: Merge 点で複数 PHI 生成
let merge_block = create_merge_block();
for (carrier, (then_val, else_val)) in carrier_updates {
    let phi_result = emit_phi(merge_block, then_val, else_val);
    // ExitLine で variable_map に接続
    exit_line.connect(carrier, phi_result);
}
```

#### 2. PhiGroupBox vs ExitLine/CarrierInfo 拡張

**Option A: PhiGroupBox（新規箱）**
```rust
pub struct PhiGroupBox {
    pub phis: Vec<PhiInfo>,  // 複数 PHI の束
}

pub struct PhiInfo {
    pub carrier_name: String,
    pub then_value: ValueId,
    pub else_value: ValueId,
    pub result: ValueId,
}
```

**メリット**:
- 複数 PHI の関係性を明示的に管理
- 単一責任の原則（PHI グループ専用）

**デメリット**:
- 新規箱の追加（複雑度増加）
- 既存の ExitLine との統合が必要

**Option B: ExitLine/CarrierInfo 拡張（既存箱再利用）**
```rust
// 既存の ExitLine を拡張
pub struct ExitLine {
    pub phi_connections: HashMap<String, ValueId>,  // carrier → PHI result
}

// CarrierInfo は既に複数 Carrier 対応
pub struct CarrierInfo {
    pub carriers: Vec<String>,
    pub updates: HashMap<String, ASTNode>,
}
```

**メリット**:
- 新規箱なし（既存インフラ再利用）
- ExitLine が既に複数 Carrier 接続をサポート

**デメリット**:
- ExitLine の責務が拡大

**Phase 195 での判断**:
→ **Option B（既存箱拡張）を採用**

**理由**:
- ExitLine は既に「variable_map への接続」を担当
- 複数 Carrier → 複数 PHI は自然な拡張
- 新規箱を作る必要性が低い（YAGNI 原則）

#### 3. 更新なし（不変）の扱い

**ケース**: 片方の分岐で Carrier が更新されない

```nyash
if(ch == '\\') {
    escaped = true
    // buffer は更新なし（不変）
} else {
    buffer = buffer + ch
    escaped = false
}
```

**設計案**: PHI 生成時に「前の値」を使用

```rust
// Then 分岐で buffer 更新なし
let then_buffer_value = previous_buffer_value;  // ← ループ header の PHI param

// Else 分岐で buffer 更新あり
let else_buffer_value = emit_string_append(...);

// Merge 点で PHI
let buffer_phi = emit_phi(merge, then_buffer_value, else_buffer_value);
```

**実装詳細**:
- `get_update_or_unchanged()` 関数で検出
- 更新なし → `ValueId` として「前の値」を返す
- PHI 生成時に自動的に接続

### 成果物
- Pattern3 lowerer の擬似コード（本セクション）
- PhiGroupBox vs ExitLine 拡張の判断（Option B 採用）
- 更新なし（不変）の扱い方設計

---

## Task 195-4: 実装スコープの決定（どこまでやるか）

### Phase 195-impl の範囲

**✅ Phase 195-impl で実装する**:
1. **複数 Carrier の P3 処理**（2-3個程度）
   - `escaped` + `buffer` のような flag + accumulation
   - `sum` + `count` のような accumulation + counter

2. **既存 UpdateKind 範囲内の update**:
   - CounterLike: `count + 1`
   - NumberAccumulation: `sum + i`
   - StringAppend: `buffer + ch`
   - BoolFlag: `true`, `false`（新規 UpdateKind 追加候補）

3. **両分岐での定義確認**:
   - Then/Else で同じ Carrier が定義されるケース
   - 更新なし（不変）の場合は自動的に「前の値」を使用

4. **E2E テスト**:
   - 簡易版 _parse_string（`ch = "x"` 定数版）
   - 簡易版 if-sum（配列アクセスなし版）

**❌ Phase 195-impl で実装しない**:
1. **LoopBodyLocal + MethodCall 混在**:
   - `local ch = s[i]; if(...) { buf += ch }`
   - → Phase 191（body-local init）と Phase 193（MethodCall）の組み合わせ
   - → Phase 195+ に延期（複雑度高い）

2. **外部ローカル変数（ConditionEnv 拡張）**:
   - `local digits = "012..."; digit = digits.indexOf(ch)`
   - → Phase 200+ に保留（設計判断済み）

3. **ネストした If**:
   - `if(...) { if(...) { ... } }`
   - → Phase 196+ に延期（P3 の P3）

### ゴールの明確化

**Phase 195-impl のゴール**:
> 「Named carrier が 2-3 個あっても P3 lowerer が壊れない」こと。
>
> JsonParser/simple selfhost で「flag + count」くらいの例が通ること。

**成功基準**:
- [ ] 簡易版 _parse_string が JoinIR で動作（escaped + buffer）
- [ ] 簡易版 if-sum が JoinIR で動作（sum + count）
- [ ] 既存テスト（phase190-194）が退行しない
- [ ] ドキュメント更新（Implementation Status セクション追加）

### テストケース設計

#### テスト 1: flag + buffer (BoolFlag + StringAppend)

**ファイル**: `apps/tests/phase195_flag_buffer.hako`

```nyash
static box Main {
    main() {
        local buffer = ""
        local escaped = false
        local i = 0

        loop(i < 3) {
            local ch = "a"  // ← 定数（配列アクセス回避）

            if(i == 1) {
                escaped = true
            } else {
                buffer = buffer + ch
                escaped = false
            }

            i = i + 1
        }

        print(buffer)  // Expected: "aa" (i=0,2 で追加)
        return 0
    }
}
```

#### テスト 2: sum + count (NumberAccumulation + CounterLike)

**ファイル**: `apps/tests/phase195_sum_count.hako`

```nyash
static box Main {
    main() {
        local sum = 0
        local count = 0
        local i = 0

        loop(i < 5) {
            if(i > 2) {
                sum = sum + i      // i=3,4 で加算
                count = count + 1
            }
            i = i + 1
        }

        print(sum)    // Expected: 7 (3+4)
        print(count)  // Expected: 2
        return 0
    }
}
```

---

## Task 195-5: CURRENT_TASK / overview 更新

### CURRENT_TASK.md 更新内容

```markdown
## Phase 195: Pattern 3 拡張（設計フェーズ）(完了予定: 2025-12-XX)

**目的**: P3（If-Else PHI）を複数キャリア対応に拡張する設計

**タスク**:
- [ ] 195-1: 対象ループ絞り込み（_parse_string 簡易版、if-sum）
- [ ] 195-2: LoopUpdateSummary/CarrierInfo 設計整理（複数キャリア対応）
- [ ] 195-3: Pattern3 lowerer 設計更新（PhiGroup vs ExitLine 拡張判断）
- [ ] 195-4: 実装スコープ決定（2-3キャリア、既存 UpdateKind 範囲内）
- [ ] 195-5: ドキュメント更新（本項目 + overview 更新）

**設計判断**:
- ✅ ExitLine 拡張で対応（PhiGroupBox は作らない）
- ✅ 両分岐での Carrier 定義確認（更新なし = 前の値使用）
- ❌ ConditionEnv 拡張なし（Phase 200+ 保留）
- ❌ LoopBodyLocal + MethodCall 混在は Phase 195+ 延期

**期待成果**:
- phase195-pattern3-extension-design.md（完全設計書）
- Phase 195-impl の実装スコープ明確化
- JsonParser カバレッジ 40% → 60% への道筋
```

### joinir-architecture-overview.md 更新内容

Section 7.2 "残タスク" に追記:

```markdown
- [ ] **Phase 195**: Pattern 3 拡張（複数キャリア対応）
  - 設計フェーズ: P3 で 2-3 個の Carrier を同時処理
  - ExitLine 拡張で複数 PHI 生成（PhiGroupBox は不要）
  - 対象: _parse_string（flag+buffer）、if-sum（sum+count）
  - Phase 195-impl で実装予定
```

---

## 成功基準（設計フェーズ）

- [x] 対象ループ選定完了（_parse_string 簡易版、if-sum）
- [x] キャリア分析表作成（UpdateKind 分類済み）
- [x] 複数キャリア対応の設計原則明確化
- [x] Pattern3 lowerer の擬似コード作成
- [x] PhiGroupBox vs ExitLine 拡張の判断（Option B 採用）
- [x] 実装スコープ決定（Phase 195-impl 範囲明確化）
- [x] テストケース設計（2ケース）
- [x] ドキュメント更新計画作成

---

## 関連ファイル

### 設計対象
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if.rs`（Phase 195-impl で実装）
- `src/mir/join_ir/lowering/loop_update_summary.rs`（BoolFlag UpdateKind 追加候補）
- `src/mir/join_ir/lowering/carrier_info.rs`（複数 Carrier 対応確認）

### テストファイル（Phase 195-impl で作成）
- `apps/tests/phase195_flag_buffer.hako`（BoolFlag + StringAppend）
- `apps/tests/phase195_sum_count.hako`（NumberAccumulation + CounterLike）

### ドキュメント
- `docs/development/current/main/phase195-pattern3-extension-design.md`（本ファイル）
- `docs/development/current/main/joinir-architecture-overview.md`（更新予定）
- `CURRENT_TASK.md`（Phase 195 設計タスク追加）

---

## 次のステップ

### Phase 195-impl: Pattern3 拡張実装

設計書（本ファイル）に基づいて実装:
1. Pattern3 lowerer に複数 Carrier 処理追加
2. ExitLine で複数 PHI 接続
3. BoolFlag UpdateKind 追加（必要に応じて）
4. E2E テスト（phase195_flag_buffer.hako, phase195_sum_count.hako）
5. ドキュメント更新（Implementation Status セクション）

### Phase 196+: 候補

- Pattern 3 のネスト対応（if-in-if）
- LoopBodyLocal + MethodCall + P3 の統合
- JsonParser の _parse_string 完全版（配列アクセス対応 = Phase 200+）

---

## 設計原則（Phase 195）

1. **既存箱再利用**:
   - PhiGroupBox を作らず、ExitLine/CarrierInfo を拡張
   - YAGNI（You Aren't Gonna Need It）原則

2. **段階的拡張**:
   - 単一キャリア（Phase 170-189）→ 複数キャリア（Phase 195）
   - 2-3 個程度に限定（無理に全部対応しない）

3. **Fail-Fast 継承**:
   - ConditionEnv 拡張なし（Phase 200+ 保留）
   - 複雑なパターンは Phase 195+ に延期

4. **箱理論の実践**:
   - 設計フェーズで構造を固める
   - 実装フェーズは lowerer のみに集中
   - ドキュメント駆動開発

---

## Implementation Status

**完了日**: 2025-12-09
**状態**: Lowerer/ExitLine 側は完了、JoinIR→MIR 変換バグにより E2E ブロック

### 実装サマリ

**Phase 195-impl で実装したこと**:

1. **`loop_with_if_phi_minimal.rs`** (+91行):
   - multi-carrier PHI 生成を追加（sum + count の 2 キャリア対応）
   - 各キャリアについて then/else 値を収集し、個別に PHI を生成
   - JoinIR 上で正しい PHI 構造を出力

2. **`pattern3_with_if_phi.rs`** (+68行):
   - 単一キャリア（後方互換）と複数キャリアを動的に扱うよう拡張
   - exit_bindings に複数キャリアを載せる処理を追加

3. **ExitLine / LoopExitBinding / CarrierVar.join_id**:
   - **変更不要！** 既存インフラをそのまま利用できた
   - Phase 170-189 で整備された CarrierVar.join_id が multi-carrier を想定していた
   - YAGNI 原則の正しさが証明された

### Blocker: Nested Select→Branch+Phi 変換バグ

**問題の概要**:
- JoinIR 側では正しい PHI が生成されている
- MIR 変換時に PHI inputs が undefined を参照するようになる

**具体例**:
```
JoinIR (正しい):
  ValueId(20) = phi [(BasicBlockId(3), ValueId(14)), (BasicBlockId(4), ValueId(18))]

MIR (壊れている):
  bb10:
      %27 = phi [%28, bb8], [%32, bb9]  // ← %28, %32 は undefined
```

**原因分析**:
- `joinir_block.rs` の Select 命令処理で、Nested Select（if-else が複数キャリアを持つ場合）の変換が壊れている
- Select → 3ブロック + 1 PHI の展開時に、block ID マッピングまたは ValueId マッピングが不整合
- Pattern3 の multi-carrier 対応自体は問題なく、bridge 層の既存バグ

**対応方針**:
- Phase 195 の scope からは外す（Lowerer/ExitLine の責務は完了）
- Phase 196 として Select 展開バグの調査・修正を別タスク化

### テスト状況

**後方互換テスト**:
- 単一キャリア P3 テスト（loop_if_phi.hako 等）: 退行確認が必要
- Pattern1/2/4 代表テスト: 影響なし（Select 変換に依存しないパターン）

**multi-carrier E2E テスト**:
- `phase195_sum_count.hako`: JoinIR 生成は正しい、MIR 変換でブロック

### 次のステップ

**Phase 196: Select 展開/変換バグ調査＆修正**（別タスク）
- `select_expansion` / `instruction_rewriter` の責務を doc に整理
- 「1 Select = 3 ブロック + 1 PHI」変換インタフェースを明文化
- block reuse / block ID マッピングの切り分け

**Phase 195 完了時点での成果**:
- P3 Lowerer は複数キャリア対応完了 ✅
- ExitLine/CarrierVar は既に対応済み（変更不要）✅
- JoinIR→MIR bridge の Select バグは別 Issue として分離 ✅
Status: Historical
