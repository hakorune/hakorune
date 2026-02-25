# Phase 247-EX: DigitPos Dual-Value Design

Status: Active
Scope: DigitPos パターンを「1つの質問→2つの値」に整理し、break 条件と NumberAccumulation 両方に対応。

## 1. DigitPos の二重値化モデル

### 1.1 問題の本質

**Phase 224 の現状**:
```rust
// Phase 224: digit_pos (integer) → is_digit_pos (boolean) に変換
digit_pos = digits.indexOf(ch)  // -1 or 0-9
  ↓ (DigitPosPromoter)
is_digit_pos (boolean carrier)  // indexOf() の成否のみ
```

**何が失われるか**:
- indexOf() の戻り値（0-9 の digit 値）が消える
- break 条件 `digit_pos < 0` は `!is_digit_pos` で対応できる（Phase 224-E で実装済み）
- しかし NumberAccumulation `result = result * 10 + digit_pos` で digit 値が必要！

**具体例**:
```nyash
// _atoi ループ (_parse_number も同様)
loop(i < n) {
    local ch = s.substring(i, i+1)
    local digit_pos = digits.indexOf(ch)  // -1 (not found) or 0-9 (digit value)

    if digit_pos < 0 { break }            // Break 用途: 成否判定（boolean）

    v = v * 10 + digit_pos                // Accumulation 用途: digit 値（integer）
    i = i + 1
}
```

### 1.2 解決: 二重値化アーキテクチャ

```
digit_pos = digits.indexOf(ch)
  ↓ (DigitPosPromoter 拡張)

Output A: is_digit_pos (boolean carrier)  ← break 条件用
Output B: digit_value (integer carrier)   ← accumulation 用
```

**質問と出力の対応**:
- **質問**: この ch は digits に含まれているか？ index はいくつか？
- **出力A**: `is_digit_pos: bool` - 含まれているか（condition 側）
- **出力B**: `digit_value: i64` - index 値（accumulation 側）

**両方を CarrierInfo に含める**:
```rust
// Phase 247-EX: DigitPos dual-value promotion
CarrierInfo {
    carriers: vec![
        CarrierVar {
            name: "is_digit_pos",
            role: CarrierRole::ConditionOnly,  // Exit PHI 不要
            init: CarrierInit::BoolConst(false),
        },
        CarrierVar {
            name: "digit_value",
            role: CarrierRole::LoopState,      // Exit PHI 必要
            init: CarrierInit::FromHost,
        },
    ],
    promoted_loopbodylocals: vec!["digit_pos".to_string()],
}
```

---

## 2. 箱の責務分割

### 2.1 DigitPosPromoter (拡張)

**Phase 224 の責務**:
- Input: `digit_pos = digits.indexOf(ch)` (LoopBodyLocal)
- Output: `is_digit_pos` (boolean carrier)

**Phase 247-EX の拡張**:
- Input: 同上
- Output:
  - `is_digit_pos` (boolean carrier) - 既存
  - `digit_value` (integer carrier) - **新規追加**

**実装方針**:
```rust
// Phase 247-EX: Dual-value carrier creation
let promoted_carrier_bool = CarrierVar {
    name: format!("is_{}", var_in_cond),      // "is_digit_pos"
    host_id: ValueId(0),
    join_id: None,
    role: CarrierRole::ConditionOnly,
    init: CarrierInit::BoolConst(false),
};

let promoted_carrier_int = CarrierVar {
    name: format!("{}_value", var_in_cond),   // "digit_pos_value" or "digit_value"
    host_id: ValueId(0),
    join_id: None,
    role: CarrierRole::LoopState,
    init: CarrierInit::FromHost,
};

let mut carrier_info = CarrierInfo::with_carriers(
    "__dummy_loop_var__".to_string(),
    ValueId(0),
    vec![promoted_carrier_bool, promoted_carrier_int],
);
```

### 2.2 CarrierInfo

**Phase 247-EX で含まれるもの**:
```rust
CarrierInfo {
    carriers: vec![
        // Boolean carrier (condition 用)
        CarrierVar {
            name: "is_digit_pos",
            role: ConditionOnly,
            init: BoolConst(false),
        },
        // Integer carrier (accumulation 用)
        CarrierVar {
            name: "digit_value",
            role: LoopState,
            init: FromHost,
        },
    ],
    promoted_loopbodylocals: vec!["digit_pos"],
}
```

**ScopeManager による解決**:
- Break 条件: `digit_pos` → `is_digit_pos` (ConditionEnv)
- Update 式: `digit_pos` → `digit_value` (UpdateEnv)

### 2.3 ConditionEnv / ExprLowerer

**Phase 224-E の既存動作**:
```rust
// Break condition: digit_pos < 0
//   ↓ (DigitPosConditionNormalizer)
// !is_digit_pos
```

**Phase 247-EX で不変**:
- Break 条件は引き続き `is_digit_pos` を参照
- ConditionEnv は `digit_pos` → `is_digit_pos` を解決

### 2.4 UpdateEnv / NumberAccumulation

**Phase 247-EX の新規動作**:
```rust
// Update expression: v = v * 10 + digit_pos
//   ↓ (UpdateEnv resolution)
// v = v * 10 + digit_value
```

**UpdateEnv の変更**:
- `digit_pos` 参照時に `digit_value` を解決
- NumberAccumulation 検出時に `digit_var: "digit_pos"` → `"digit_value"` に変換

**CarrierUpdateEmitter の変更**:
- `UpdateRhs::NumberAccumulation { digit_var }` で `digit_var` を UpdateEnv から解決
- UpdateEnv が既に `digit_pos` → `digit_value` を解決済みと仮定

---

## 3. 影響範囲（パターン別）

### 3.1 Pattern2: _parse_number

**ループ構造**:
```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)
    local digit_pos = digits.indexOf(ch)

    if digit_pos < 0 { break }  // Break 条件

    num_str = num_str + ch      // StringAppend (digit_pos 不使用)
    p = p + 1
}
```

**変数の役割**:
| Variable | Type | Usage | Carrier Type |
|----------|------|-------|--------------|
| `p` | position | loop counter | LoopState |
| `num_str` | string | accumulator | LoopState |
| `ch` | string | LoopBodyLocal | - |
| `digit_pos` | integer | LoopBodyLocal | → `is_digit_pos` (ConditionOnly) + `digit_value` (unused) |

**Phase 247-EX での扱い**:
- Header: `p < s.length()` (ExprLowerer)
- Break: `digit_pos < 0` → `!is_digit_pos` (ConditionEnv + Normalizer)
- Update:
  - `p = p + 1` (Increment)
  - `num_str = num_str + ch` (StringAppend)
- **digit_value は生成されるが使用されない**（無駄だが害なし）

### 3.2 Pattern2: _atoi

**ループ構造**:
```nyash
loop(i < n) {
    local ch = s.substring(i, i+1)
    local pos = digits.indexOf(ch)

    if pos < 0 { break }        // Break 条件

    v = v * 10 + pos            // NumberAccumulation (pos 使用！)
    i = i + 1
}
```

**変数の役割**:
| Variable | Type | Usage | Carrier Type |
|----------|------|-------|--------------|
| `i` | position | loop counter | LoopState |
| `v` | integer | number accumulator | LoopState |
| `ch` | string | LoopBodyLocal | - |
| `pos` | integer | LoopBodyLocal | → `is_pos` (ConditionOnly) + `pos_value` (LoopState) |

**Phase 247-EX での扱い**:
- Header: `i < n` (ExprLowerer)
- Break: `pos < 0` → `!is_pos` (ConditionEnv + Normalizer)
- Update:
  - `i = i + 1` (Increment)
  - `v = v * 10 + pos` → `v = v * 10 + pos_value` (NumberAccumulation)
- **両方の carrier が必要！**

### 3.3 Pattern2: _atof_loop [Future]

**ループ構造** (想定):
```nyash
loop(i < n) {
    local ch = s.substring(i, i+1)
    local digit_pos = digits.indexOf(ch)

    if digit_pos < 0 { break }

    result = result + digit_pos * place_value
    place_value = place_value / 10
    i = i + 1
}
```

**Phase 247-EX での扱い**:
- 同じ二重値化モデルが適用可能
- `digit_value` を乗算・累積に使用

---

## 4. 実装アーキテクチャ

### 4.1 データフロー図

```
DigitPosPromoter
  ├→ is_digit_pos (boolean)
  │    ↓
  │   ConditionEnv (Phase 245C capture)
  │    ├→ ExprLowerer (break condition)
  │    └→ is_digit_pos → boolean in scope
  │
  └→ digit_value (integer)
       ↓
      UpdateEnv
       ├→ digit_pos reference → digit_value (integer)
       └→ NumberAccumulation pattern resolution
```

### 4.2 命名規則

**Phase 247-EX の命名戦略**:
```rust
// Option A: Separate naming (推奨)
is_digit_pos  // boolean carrier (condition 用)
digit_value   // integer carrier (accumulation 用)

// Option B: Prefix naming
is_digit_pos          // boolean
digit_pos_value       // integer (冗長だが明示的)

// Option C: Short naming
is_digit_pos          // boolean
digit_pos             // integer (元の名前を保持、混乱の恐れ)
```

**採用**: Option A（Separate naming）
- 理由: 用途が明確、混乱が少ない、既存の `is_*` 命名規則と整合

### 4.3 ScopeManager の解決ルール

**ConditionEnv (break 条件)**:
```rust
// digit_pos 参照 → is_digit_pos (boolean carrier)
env.get("digit_pos") → Some(is_digit_pos_value_id)
```

**UpdateEnv (update 式)**:
```rust
// digit_pos 参照 → digit_value (integer carrier)
env.resolve("digit_pos") → Some(digit_value_value_id)
```

**実装方針**:
1. CarrierInfo に両方の carrier を登録
2. ScopeManager が context 依存で正しい carrier を返す
   - ConditionEnv: `promoted_loopbodylocals` に含まれる名前 → `is_*` carrier
   - UpdateEnv: `promoted_loopbodylocals` に含まれる名前 → `*_value` carrier

---

## 5. 責務の明確化

| Component | Input | Output | 用途 |
|-----------|-------|--------|------|
| **DigitPosPromoter** | indexOf() result | `is_digit_pos` + `digit_value` | 二重値化 |
| **CarrierInfo** | DigitPos values | Both as LoopState | キャリア登録 |
| **ConditionEnv** | Promoted carriers | `is_digit_pos` (bool) | Break 条件 |
| **UpdateEnv** | Promoted carriers | `digit_value` (i64) | Accumulation |
| **ExprLowerer** | Condition AST | boolean ValueId | Lowering |
| **DigitPosConditionNormalizer** | `digit_pos < 0` | `!is_digit_pos` | AST 変換 |
| **CarrierUpdateEmitter** | UpdateExpr | JoinIR instructions | Update 生成 |

**Phase 247-EX の変更箇所**:
- ✅ **DigitPosPromoter**: 2つの carrier を生成
- ✅ **CarrierInfo**: 両方を含める
- ✅ **ConditionEnv**: `digit_pos` → `is_digit_pos` 解決（既存）
- 🆕 **UpdateEnv**: `digit_pos` → `digit_value` 解決（新規）
- ✅ **DigitPosConditionNormalizer**: `digit_pos < 0` → `!is_digit_pos`（既存）
- 🆕 **CarrierUpdateEmitter**: `digit_value` を UpdateEnv から解決（新規）

---

## 6. テスト戦略

### 6.1 DigitPos Promoter 単体テスト

**既存テスト** (loop_body_digitpos_promoter.rs):
- ✅ Phase 224: `is_digit_pos` 側の生成を確認
- 🆕 Phase 247-EX: `digit_value` 側も生成されることを確認

**新規テスト**:
```rust
#[test]
fn test_digitpos_dual_value_carriers() {
    // digit_pos = indexOf(ch) → is_digit_pos + digit_value
    let result = DigitPosPromoter::try_promote(req);
    match result {
        Promoted { carrier_info, .. } => {
            assert_eq!(carrier_info.carriers.len(), 2);

            // Boolean carrier
            let bool_carrier = &carrier_info.carriers[0];
            assert_eq!(bool_carrier.name, "is_digit_pos");
            assert_eq!(bool_carrier.role, CarrierRole::ConditionOnly);

            // Integer carrier
            let int_carrier = &carrier_info.carriers[1];
            assert_eq!(int_carrier.name, "digit_value");
            assert_eq!(int_carrier.role, CarrierRole::LoopState);
        }
        _ => panic!("Expected Promoted"),
    }
}
```

### 6.2 _parse_number (Pattern2) E2E テスト

**テストファイル**: `apps/tests/phase189_parse_number_mini.hako`

**期待される動作**:
- Break 条件: `digit_pos < 0` が `is_digit_pos` で動作
- `digit_value` carrier は生成されるが未使用（害なし）
- String 累積（`num_str`）は別の carrier で処理

**検証コマンド**:
```bash
NYASH_JOINIR_DEBUG=1 ./target/release/hakorune apps/tests/phase189_parse_number_mini.hako
```

**期待出力**:
```
[digitpos_promoter] A-4 DigitPos pattern promoted: digit_pos → is_digit_pos + digit_value
[pattern2/lowering] Using promoted carrier: is_digit_pos (condition)
[pattern2/lowering] Carrier 'digit_value' registered but unused
```

### 6.3 _atoi (Pattern2) E2E テスト

**テストファイル**: `apps/tests/phase189_atoi_mini.hako` または `tests/phase246_json_atoi.rs`

**期待される動作**:
- Break 条件: `pos < 0` が `is_pos` で動作
- NumberAccumulation: `v = v * 10 + pos` が `v = v * 10 + pos_value` で動作
- 両方の carrier が正しく使用される

**検証コマンド**:
```bash
NYASH_JOINIR_DEBUG=1 cargo test --release phase246_json_atoi -- --nocapture
```

**期待出力**:
```
[digitpos_promoter] A-4 DigitPos pattern promoted: pos → is_pos + pos_value
[pattern2/lowering] Using promoted carrier: is_pos (condition)
[pattern2/lowering] Using promoted carrier: pos_value (accumulation)
[carrier_update] NumberAccumulation: digit_var='pos' resolved to pos_value
```

### 6.4 Regression テスト

**対象**:
- Phase 224 系テスト（DigitPos promotion 基本動作）
- Phase 245 系テスト（_parse_number 系）
- Phase 246 系テスト（_atoi 系）

**確認項目**:
- 既存の `is_digit_pos` carrier が引き続き動作
- 新規の `digit_value` carrier が追加されても退行なし

---

## 7. 成功条件

### 7.1 ビルド

- [ ] `cargo build --release` 成功（0 errors, 0 warnings）

### 7.2 単体テスト

- [ ] DigitPosPromoter テスト: 2つの carrier 生成を確認
- [ ] CarrierInfo テスト: 両方の carrier が含まれることを確認
- [ ] UpdateEnv テスト: `digit_pos` → `digit_value` 解決を確認

### 7.3 E2E テスト

- [ ] _parse_number: `digit_value` 未使用でも正常動作
- [ ] _atoi: `digit_value` を NumberAccumulation で使用
- [ ] Phase 224/245/246 既存テスト: 退行なし

### 7.4 ドキュメント

- [ ] このドキュメント完成
- [ ] 箱の責務が明確に書かれている
- [ ] テスト戦略が具体的

---

## 8. 将来への展開

### 8.1 他のパターンへの適用

同じ二重値化モデルを以下に適用可能：

**_atof_loop** (小数パース):
- `digit_pos` → `is_digit_pos` + `digit_value`
- 小数点以下の桁数計算にも使用可能

**汎用的な indexOf パターン**:
- 任意の indexOf() 呼び出しで同様の二重値化を適用
- condition-only 用途と value 用途を自動判定

### 8.2 Pattern3/4 への拡張

**Pattern3 (if-sum)**:
- 複数の条件分岐でも同じ二重値化モデルを適用
- Exit PHI で両方の carrier を適切に扱う

**Pattern4 (continue)**:
- continue 条件でも boolean carrier を使用
- update 式で integer carrier を使用

### 8.3 最適化の可能性

**未使用 carrier の削除**:
- `_parse_number` のように `digit_value` が未使用の場合、最適化で削除可能
- CarrierInfo に "used" フラグを追加して不要な carrier を省略

**命名の統一化**:
- `digit_pos` → `digit_pos` (integer) + `is_digit_pos` (boolean)
- UpdateEnv/ConditionEnv が型情報から自動判定

---

## 9. Box-First 原則の適用

### 9.1 Single Responsibility

- **DigitPosPromoter**: indexOf パターン検出と二重値化のみ
- **CarrierInfo**: Carrier メタデータ保持のみ
- **ConditionEnv**: Condition 側の解決のみ
- **UpdateEnv**: Update 側の解決のみ

### 9.2 Fail-Safe Design

- 未使用の carrier も生成（害なし）
- 解決失敗時は明示的エラー（panic なし）
- 退行テストで既存動作を保証

### 9.3 Boundary Clarity

```
Input:  digit_pos = indexOf(ch)  (AST)
  ↓
Output: is_digit_pos (bool) + digit_value (i64)  (CarrierInfo)
  ↓
Usage:  ConditionEnv → is_digit_pos
        UpdateEnv → digit_value
```

---

## 10. 参考資料

### 10.1 Phase ドキュメント

- **Phase 224**: `phase224-digitpos-promoter-design.md` - DigitPos promotion 基本設計
- **Phase 224-E**: `phase224-digitpos-condition-normalizer.md` - AST 変換設計
- **Phase 245**: `phase245-jsonparser-parse-number-joinir-integration.md` - _parse_number 統合
- **Phase 246**: `phase246-jsonparser-atoi-joinir-integration.md` - _atoi 統合
- **Phase 227**: CarrierRole 導入（LoopState vs ConditionOnly）
- **Phase 228**: CarrierInit 導入（FromHost vs BoolConst）

### 10.2 コード参照

- **DigitPosPromoter**: `src/mir/loop_pattern_detection/loop_body_digitpos_promoter.rs`
- **CarrierInfo**: `src/mir/join_ir/lowering/carrier_info.rs`
- **ConditionEnv**: `src/mir/join_ir/lowering/condition_env.rs`
- **UpdateEnv**: `src/mir/join_ir/lowering/update_env.rs`
- **CarrierUpdateEmitter**: `src/mir/join_ir/lowering/carrier_update_emitter/mod.rs`
- **LoopUpdateAnalyzer**: `src/mir/join_ir/lowering/loop_update_analyzer.rs`

### 10.3 テストファイル

- **_parse_number**: `apps/tests/phase189_parse_number_mini.hako`, `tests/phase245_json_parse_number.rs`
- **_atoi**: `apps/tests/phase189_atoi_mini.hako`, `tests/phase246_json_atoi.rs`
- **DigitPos unit**: `src/mir/loop_pattern_detection/loop_body_digitpos_promoter.rs` (tests module)

---

## 11. Revision History

- **2025-12-11**: Phase 247-EX 設計ドキュメント作成
- **Status**: Active - 設計確定、実装準備完了
- **Scope**: DigitPos 二重値化設計（ExprLowerer + NumberAccumulation ライン）
