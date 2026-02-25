# Phase 211: JsonParser 次の 1 手（中規模ループ候補選定）

**Phase**: 211
**Date**: 2025-12-09
**Status**: 🎯 設計フェーズ（コード実装なし）
**Prerequisite**: Phase 210 完了（軽量ループ 3 本実戦成功）

---

## 🎯 Phase 211 の目的

Phase 210 で「軽量ループ 3 本」が完全成功したため、次は **「中規模の複雑さを持つループ 1 本」** を選び、既存の Pattern/P5 boxes をどう組み合わせるか **設計のみ** 行う。

### 📋 作業範囲（明確化）

- ✅ **やること**: ループ 1 本選定 → Pattern/boxes マッピング → 組み合わせ戦略設計
- ❌ **やらないこと**: コード実装、ハーネス作成、テスト実行
- 🎯 **成果物**: Phase 212+ で実装する際の「設計図」

---

## Task 211-1: 中規模ループ候補の選定

### 候補 A: `_parse_string` 簡略版

**元の仕様** (Phase 181 より):
```hako
_parse_string(pos) {
    local i = pos
    local escaped = 0  // LoopBodyLocal (フラグ)
    local buf = new ArrayBox()  // Buffer構築

    loop(i < len) {
        local ch = s.char_at(i)
        if ch == quote and escaped == 0 { break }  // 終了条件
        if ch == backslash {
            escaped = 1  // フラグ切り替え
        } else {
            if escaped == 1 {
                buf.append(escape_char(ch))  // エスケープ処理
                escaped = 0
            } else {
                buf.append(ch)
            }
        }
        i = i + 1
    }
    return buf.to_string()
}
```

**簡略版スコープ** (Phase 211 用):
- ✅ `escaped` フラグ（LoopBodyLocal の if 分岐）
- ✅ `buf` バッファ構築（ArrayBox.append）
- ❌ `escape_char()` 詳細処理（Phase 211 では省略 → "X" で代用）
- ❌ StringBox.to_string()（単純化のため最終 return は buf のまま）

**複雑さの軸**:
- **A軸 (更新)**: `i = i + 1` （Simple）+ `escaped` フラグ切り替え（IfPHI 必要）
- **B軸 (脱出)**: `break` （Pattern 2 Break）
- **C軸 (条件)**: `ch == quote and escaped == 0` （Multi-condition）
- **D軸 (変数)**: `i`, `escaped`, `buf` （3 carriers）

### 候補 B: selfhost if-sum パターン

**元の仕様** (Phase 181 より):
```hako
// FuncScannerBox._sum_def_count() の簡略版
_sum_def_count(defs) {
    local sum = 0
    local i = 0
    loop(i < defs.len()) {
        local item = defs.get(i)
        if item != null {
            sum = sum + 1  // 条件付き加算
        }
        i = i + 1
    }
    return sum
}
```

**複雑さの軸**:
- **A軸 (更新)**: `sum = sum + 1` （条件内）+ `i = i + 1` （無条件）
- **B軸 (脱出)**: なし（自然終了）
- **C軸 (条件)**: `item != null` （Simple）
- **D軸 (変数)**: `sum`, `i` （2 carriers）

---

## Task 211-2: Pattern/Boxes マッピング（候補ごと）

### 候補 A マッピング: `_parse_string` 簡略版

| 軸 | 要求 | 既存 Pattern/Box | Phase 210 時点の対応状況 |
|---|-----|----------------|----------------------|
| **A軸** | `i = i + 1` + `escaped` フラグ | Pattern 2 + IfPHI | ✅ Phase 210 で multi-carrier 確認済み |
| **B軸** | `break` | Pattern 2 Break | ✅ Phase 210 で動作確認済み |
| **C軸** | `ch == quote and escaped == 0` | ConditionLowerer + Multi-condition | ✅ Phase 169 で `and` 対応済み |
| **D軸** | 3 carriers (`i`, `escaped`, `buf`) | CarrierInfo + Multi-carrier | ✅ Phase 210 で 2-carrier 確認済み（3-carrier は未テスト） |

**特殊要素**:
- **LoopBodyLocal**: `escaped` はループ内 if 分岐で更新される「状態フラグ」
  - Phase 171 Trim Pattern では「ループ末尾で代入→Carrier 昇格」だったが、今回は **「if 分岐内で更新→PHI 必要」**
  - 既存 IfPHI ロジック（Phase 61）で対応可能か要検証

- **Buffer 構築**: `buf.append(ch)` は BoxCall だが、JoinIR では BoxCall は Opaque 扱い
  - Phase 210 で BoxCall 自体は問題なし（既存パターンで動作）

**Phase 211 での設計焦点**:
1. `escaped` フラグを Carrier として扱うか、LoopBodyLocal+IfPHI で扱うか
2. 3-carrier (i, escaped, buf) の PHI 配線が既存ロジックで通るか

### 候補 B マッピング: selfhost if-sum パターン

| 軸 | 要求 | 既存 Pattern/Box | Phase 210 時点の対応状況 |
|---|-----|----------------|----------------------|
| **A軸** | `sum = sum + 1` (条件内) + `i = i + 1` | Pattern 1 + IfPHI | ✅ IfPHI は Phase 61 で実装済み |
| **B軸** | なし（自然終了） | Pattern 1 Simple | ✅ Phase 210 で確認済み |
| **C軸** | `item != null` | ConditionLowerer | ✅ 比較演算子対応済み |
| **D軸** | 2 carriers (`sum`, `i`) | CarrierInfo | ✅ Phase 210 で動作確認済み |

**特殊要素**:
- **条件付き更新**: `sum = sum + 1` が if ブロック内
  - Phase 61 IfPHI で対応可能（ループ内 if は Merge 経由で Carrier に PHI 接続）

**Phase 211 での設計焦点**:
1. ループ内 if の `sum` 更新が IfPHI → Loop Header PHI に正しく接続されるか確認

---

## Task 211-3: 推奨候補の選定と組み合わせ戦略

### 🎯 推奨: 候補 B (`selfhost if-sum`) を Phase 211 で選定

**理由**:
1. **既存 boxes で完全カバー可能**
   - Pattern 1 Simple + IfPHI + Multi-carrier（すべて Phase 210 で動作確認済み）
   - 新規要素: 「ループ内 if の条件付き更新」のみ

2. **検証価値が高い**
   - Phase 61 IfPHI が「ループ内 if」でも正しく動作するか実戦確認
   - selfhost 実用パターン（`_sum_def_count` 等）の代表例

3. **Phase 212 実装が軽量**
   - ハーネス作成が簡単（ArrayBox.get + null チェック）
   - デバッグが容易（条件分岐 1 箇所のみ）

**候補 A を Phase 212 以降に回す理由**:
- 3-carrier は Phase 210 で未テスト（2-carrier までしか確認していない）
- `escaped` フラグの LoopBodyLocal+IfPHI 処理が複雑
- Phase 211 で「ループ内 if 更新」を先に確認してから、Phase 212+ で 3-carrier に進む方が安全

---

## Task 211-4: Boxes 組み合わせ設計（候補 B: if-sum）

### 使用する既存 Boxes

| Box 名 | 役割 | Phase 210 確認状況 |
|-------|-----|------------------|
| **LoopPatternRouter** | Pattern 1 ルーティング | ✅ Phase 210 で動作確認 |
| **SimpleWhileMinimal** | Pattern 1 lowering | ✅ Phase 210 で動作確認 |
| **ConditionLowerer** | `item != null` → JoinIR | ✅ Phase 169/210 で確認 |
| **CarrierInfo** | `sum`, `i` の metadata 管理 | ✅ Phase 210 で確認 |
| **IfPhiContext** | ループ内 if の PHI 生成 | ⚠️ Phase 61 実装済みだが、ループ内 if での実戦は未確認 |
| **JoinValueSpace** | ValueId 割り当て | ✅ Phase 210 で region 分離確認 |

### 処理フロー設計（Phase 212 実装時の想定）

```
1. LoopPatternRouter が Pattern 1 を検出
   ↓
2. SimpleWhileMinimal が呼び出される
   ↓
3. CarrierInfo が `sum`, `i` を carrier として登録
   ↓
4. Loop Header PHI 生成:
   - PHI(sum): entry=0, back_edge=sum_updated
   - PHI(i): entry=0, back_edge=i_updated
   ↓
5. ConditionLowerer が `i < defs.len()` を JoinIR に変換
   ↓
6. ループ本体:
   - `local item = defs.get(i)` → JoinIR BoxCall (Opaque)
   - `if item != null { ... }` → IfPhiContext 起動
     ↓
     6a. IfPhiContext が if ブロック内の `sum = sum + 1` を処理
         - then ブロック: sum_updated = sum_current + 1
         - else ブロック: sum_updated = sum_current （変更なし）
         - Merge 点: PHI(sum_updated) ← [then: sum+1, else: sum]
     ↓
   - `i = i + 1` → 無条件更新
   ↓
7. Loop Back Edge:
   - sum_updated → Header PHI(sum) の back_edge
   - i_updated → Header PHI(i) の back_edge
   ↓
8. Exit PHI:
   - PHI(sum_final): loop_exit ← Header PHI(sum)
   - PHI(i_final): loop_exit ← Header PHI(i)
```

### 重要な設計ポイント

**IfPhiContext の責務**:
- ループ内 if の **Merge 点で PHI 生成** → この PHI が Loop Header PHI の back_edge に接続される
- Phase 61 実装時は「ループ外 if」を想定していたが、**ループ内 if でも同じロジックが適用できる** はず

**検証ポイント（Phase 212 で確認）**:
1. IfPhiContext がループ内 if を正しく検出するか
2. Merge PHI が Header PHI の back_edge に正しく接続されるか
3. `sum` の ValueId が Param region (100-999) に割り当てられるか（Phase 201/205 要件）

---

## Task 211-5: Phase 212+ 実装スコープ定義

### Phase 212: if-sum ハーネス実装・実行

**スコープ**:
- ✅ `apps/tests/phase212_if_sum_min.hako` 作成
- ✅ 実行 → 観測（Phase 210 と同じ Fail-Fast 戦略）
- ✅ IfPhiContext のループ内 if 動作確認
- ✅ phase212-if-sum-observation.md にログ記録

**期待される成果**:
- ループ内 if の条件付き更新が正しく動作
- IfPhiContext → Header PHI 接続が正常
- → **「ループ内 if + multi-carrier」パターンが実戦確認済み** になる

### Phase 213+: 段階的拡張（候補 A 等）

**Phase 213**: 3-carrier テスト（`_parse_string` 簡略版の前段階）
- 候補: `i`, `sum`, `count` の 3-carrier ループ（ダミー処理）
- 目的: 3-carrier の PHI 配線が既存ロジックで通るか確認

**Phase 214**: `_parse_string` 簡略版（`escaped` フラグ + `buf` バッファ）
- 候補 A の実装
- 条件: Phase 213 で 3-carrier が成功していること

**Phase 215+**: 残りの JsonParser ループ（Phase 181 inventory より）
- `_read_array`, `_read_object` 等の再帰呼び出しパターン
- `_parse_hex` 等の特殊処理

---

## 📊 Phase 211 の成果物（このドキュメント）

### ✅ 達成したこと

1. **候補選定**: 候補 B (`selfhost if-sum`) を Phase 212 実装対象に選定
2. **Pattern/Boxes マッピング**: 既存 boxes で完全カバー可能と確認
3. **組み合わせ戦略**: IfPhiContext → Header PHI 接続フローを設計
4. **Phase 212+ スコープ**: 段階的拡張計画を定義

### 🎯 Phase 212 への引き継ぎ事項

- **実装対象**: `apps/tests/phase212_if_sum_min.hako`（条件付き加算ループ）
- **検証ポイント**: IfPhiContext のループ内 if 動作、Header PHI 接続
- **期待結果**: Phase 210 同様の完全成功（Fail-Fast トリガーなし）

---

## 📝 補足: Phase 210 との差分

| 項目 | Phase 210 | Phase 211 |
|-----|----------|----------|
| **複雑さ** | 軽量（Pattern 1/2 基本形） | 中規模（ループ内 if 更新） |
| **新規要素** | なし（既存確認のみ） | IfPhiContext のループ内適用 |
| **Carrier 数** | 2 まで確認 | 2（Phase 213 で 3 に拡張予定） |
| **アプローチ** | 実戦観測 | 設計のみ（Phase 212 で実装） |

---

**Phase 211 完了条件**: ✅ このドキュメントの作成完了
**次のステップ**: Phase 212（if-sum ハーネス実装・実行）
Status: Active  
Scope: ループ候補選択の設計（JoinIR/JsonParser ライン）
