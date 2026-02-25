# Phase 245B: num_str Carrier Design Document

---
**Phase 26-45 Completion**: このフェーズで設計した機能は Phase 43/245B で実装完了。最終状態は [PHASE_43_245B_NORMALIZED_COMPLETION.md](./PHASE_43_245B_NORMALIZED_COMPLETION.md) を参照。
---

**Status**: Implemented (Phase 245B-IMPL)
**Target**: `_parse_number` loop の `num_str` 文字列キャリア対応
**Scope**: Pattern 2 (loop with break) + 既存インフラ活用
**Notes**: jsonparser_parse_number_real フィクスチャを Structured→Normalized→MIR(direct) で実装し、`num_str = num_str + ch` を LoopState キャリアとして dev テスト固定済み。

---

## 1. num_str の役割

### 1.1 現在の `_parse_number` ループ構造

```nyash
fn _parse_number(s, p) {
    local num_str = ""
    local len = s.length()
    loop(p < len) {
        local ch = s.substring(p, p + 1)
        if not _is_digit(ch) {
            break
        }
        num_str = num_str + ch   // ← StringAppend carrier update
        p = p + 1
    }
    return num_str  // or return { num_str, p }
}
```

### 1.2 num_str の役割定義

| 観点 | 決定 |
|------|------|
| **主目的** | digit を連結する文字列バッファ |
| **スコープ** | `_parse_number` 専用（245B では） |
| **共有可能性** | 将来 `_atoi` / `_atof_loop` と共通化可能 |
| **初期値** | 空文字 `""` |
| **更新パターン** | Append のみ（削除・置換なし） |

### 1.3 将来の拡張候補（245B では非対象）

- `_atoi`: `num_str` → `result: Integer` への変換ループ
- `_atof_loop`: 小数部・指数部を含む浮動小数点パース
- これらとの整合性は **Phase 246+** で検討

---

## 2. 許可する UpdateExpr パターン

### 2.1 最小セット（245B 対象）

```
num_str = num_str + ch
```

| 要素 | 制約 |
|------|------|
| **左辺** | `num_str`（LoopState carrier） |
| **右辺** | `BinaryOp(Add, num_str, ch)` のみ |
| **ch** | body-local または captured 変数 |

### 2.2 許可パターンの形式定義

```rust
// 許可: num_str = num_str + ch
UpdatePattern::StringAppend {
    carrier: "num_str",
    append_source: Variable("ch"),
}

// 内部表現
BinaryOp {
    op: Add,
    lhs: Variable { name: carrier_name },  // 同じキャリア
    rhs: Variable { name: append_var },    // body-local/captured
}
```

### 2.3 非対象パターン（将来フェーズ候補）

| パターン | 理由 | 候補フェーズ |
|----------|------|-------------|
| `ch + num_str` | 逆順 concat | Phase 246 |
| `num_str + num_str` | 自己 concat | Phase 247 |
| `a + b + c` | 三項以上 | Phase 247 |
| `num_str.append(ch)` | MethodCall | Phase 248 |

### 2.4 CarrierUpdateEmitter への統合案

**Option A**: 既存 UpdateExpr に `StringConcat` variant 追加

```rust
pub enum UpdateExpr {
    // 既存
    Increment { carrier: String, amount: i64 },
    Accumulate { carrier: String, source: ValueId },

    // 新規追加 (245B)
    StringAppend { carrier: String, append_source: String },
}
```

**Option B**: Generic `BinaryUpdate` で統一

```rust
pub enum UpdateExpr {
    BinaryUpdate {
        carrier: String,
        op: BinaryOperator,
        rhs: UpdateRhs,
    },
}

pub enum UpdateRhs {
    Variable(String),
    Constant(ConstValue),
    Carrier(String),
}
```

**推奨**: Option A（最小変更、明示的）

---

## 3. num_str キャリアの Contract / Invariant

### 3.1 Loop Entry Contract

```
PRE:
  - num_str = "" (空文字で初期化済み)
  - p = 開始位置 (valid index)
  - s = 対象文字列 (immutable)
```

### 3.2 Loop Iteration Invariant

```
INV:
  - num_str = s[start_p..p] の digit 部分文字列
  - p は monotonic increasing (p' > p)
  - num_str.length() == p - start_p
```

### 3.3 Loop Exit Contract

```
POST (break):
  - num_str = parse された digit 列
  - p = 最初の non-digit 位置
  - num_str == s[start_p..p]

POST (natural exit):
  - num_str = s[start_p..len] 全て digit
  - p == len
```

### 3.4 ExitMeta 構造

```rust
ExitMeta {
    exit_values: vec![
        ("p".to_string(), p_final),           // loop counter
        ("num_str".to_string(), num_str_final), // string carrier
    ],
}
```

---

## 4. テストケース

### 4.1 E2E テスト（正常系）

| 入力 | 期待 num_str | 期待 RC |
|------|-------------|---------|
| `"0"` | `"0"` | 0 |
| `"42"` | `"42"` | 0 |
| `"123456"` | `"123456"` | 0 |
| `"007"` | `"007"` | 0 (先頭 0 許容) |

### 4.2 E2E テスト（部分マッチ系）

| 入力 | 期待 num_str | 期待 p | 備考 |
|------|-------------|--------|------|
| `"7z"` | `"7"` | 1 | 1文字で break |
| `"123abc"` | `"123"` | 3 | 3文字で break |
| `"abc"` | `""` | 0 | 即 break |

### 4.3 JoinIR 構造テスト

```rust
#[test]
fn test_parse_number_string_carrier() {
    // Given: _parse_number loop
    // When: JoinIR generated
    // Then:
    //   - num_str is in CarrierInfo with role=LoopState
    //   - UpdateExpr::StringAppend for num_str exists
    //   - ExitMeta contains num_str exit value
}
```

### 4.4 UpdateExpr 検証テスト

```rust
#[test]
fn test_string_append_update_expr() {
    // Given: num_str = num_str + ch
    // When: UpdateExpr extracted
    // Then:
    //   - UpdateExpr::StringAppend { carrier: "num_str", append_source: "ch" }
    //   - Exactly 1 StringAppend for num_str
}
```

---

## 5. 制約と非目標

### 5.1 Phase 245B の制約

| 制約 | 理由 |
|------|------|
| `_parse_number` のみ対象 | スコープ限定、段階的実装 |
| 新パターン追加なし | P2 + 既存インフラ活用 |
| by-name hardcode 禁止 | CarrierInfo/UpdateExpr で区別 |
| StringAppend 1形式のみ | 最小セットから開始 |

### 5.2 非目標（245B では実装しない）

- `_atoi` / `_atof_loop` との共通化
- Pattern 3/4 への文字列キャリア拡張
- MethodCall 形式の append (`num_str.append(ch)`)
- 逆順 concat (`ch + num_str`)
- 三項以上の concat

### 5.3 後続フェーズ候補

| フェーズ | 内容 |
|---------|------|
| **246** | `_atoi` / `_atof_loop` との整合性 |
| **247** | Pattern 3/4 に文字列キャリア拡張 |
| **248** | MethodCall append 対応 |
| **249** | 逆順・三項 concat 対応 |

---

## 6. 実装ロードマップ

### 6.1 Phase 245B-1: UpdateExpr 拡張

1. `UpdateExpr::StringAppend` variant 追加
2. CarrierUpdateEmitter に StringAppend 検出ロジック
3. ユニットテスト 3-5 件

### 6.2 Phase 245B-2: CarrierInfo 統合

1. String 型キャリアの role=LoopState 対応
2. ExitMeta に string carrier 含める
3. Header/Exit PHI に string value 通す

### 6.3 Phase 245B-3: Pattern 2 統合

1. `loop_with_break_minimal.rs` で StringAppend 処理
2. E2E テスト実装
3. `_parse_number` テストケース通す

### 6.4 完了条件

- [ ] `cargo build --release` 成功
- [ ] 全テスト PASS（911+）
- [ ] `_parse_number` E2E 4+ ケース PASS
- [ ] UpdateExpr::StringAppend ユニットテスト PASS
- [ ] by-name hardcode なし

---

## 7. リスク評価

| リスク | 影響度 | 軽減策 |
|--------|--------|--------|
| String PHI の型不整合 | 中 | 既存 String carrier テスト確認 |
| UpdateExpr 検出失敗 | 中 | 段階的検出ロジック |
| Pattern 2 退行 | 低 | 911 テスト PASS 維持 |
| 将来の拡張困難 | 低 | Option A 設計で拡張可能 |

---

## 8. 承認チェックリスト

- [ ] num_str の役割（Section 1）確認
- [ ] UpdateExpr パターン（Section 2）確認
- [ ] Contract/Invariant（Section 3）確認
- [ ] テストケース（Section 4）確認
- [ ] 制約と非目標（Section 5）確認
- [ ] 実装ロードマップ（Section 6）確認

**承認後、Phase 245B-1 実装開始！**

---

*Document created: Phase 245B Design*
*Author: Claude Code Session*
*Date: 2025-12-11*
