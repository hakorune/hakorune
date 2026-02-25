# Phase 257: Loop with Early Return Pattern

Status: Completed (2025-12-20)
Scope: Pattern6（ScanWithInit）拡張で reverse scan + early return を受理する
Related:
- Phase 256 完了（Pattern2 boundary SSOT 化、entry_param_mismatch 根治）
- North star: `docs/development/current/main/design/join-explicit-cfg-construction.md`

## Current Status (SSOT)

- Former first FAIL: `json_lint_vm / StringUtils.last_index_of/2`（P0/P1で解消）
- Current first FAIL: `json_lint_vm / StringUtils.is_integer/1`（Phase 259）
- Result: Pattern6 reverse scan + PHI/CFG 安定化（P0/P1）により quick が次へ進む

---

## Background

### 失敗詳細

**テスト**: json_lint_vm (quick profile)
**エラー**: `[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern`
**関数**: `StringUtils.last_index_of/2`

#### エラーメッセージ全体

```
[phase143/debug] Attempting loop_true_if_break/continue pattern (P0/P1)
[phase143/debug] Pattern out-of-scope: NotLoopTrue
[trace:dev] phase121/shadow: shadow=skipped signature_basis=kinds=Block,Stmt(local(i)),Loop,Block,If,Block,Stmt(return(value)),Stmt(assign(i)),Stmt(return(value));exits=return;writes=i;reads=ch,i,s;caps=If,Loop,Return;conds=(var:i >= lit:int:0)|(other:MethodCall == var:ch)
[trace:dev] loop_canonicalizer: Function: StringUtils.last_index_of/2
[trace:dev] loop_canonicalizer:   Skeleton steps: 0
[trace:dev] loop_canonicalizer:   Carriers: 0
[trace:dev] loop_canonicalizer:   Has exits: false
[trace:dev] loop_canonicalizer:   Decision: FAIL_FAST
[trace:dev] loop_canonicalizer:   Missing caps: [ConstStep]
[trace:dev] loop_canonicalizer:   Reason: Phase 143-P2: Loop does not match read_digits(loop(true)), skip_whitespace, parse_number, continue, parse_string, or parse_array pattern
[ERROR] ❌ MIR compilation error: [joinir/freeze] Loop lowering failed: JoinIR does not support this pattern, and LoopBuilder has been removed.
Function: StringUtils.last_index_of/2
Hint: This loop pattern is not supported. All loops must use JoinIR lowering.
```

### 期待される動作

`StringUtils.last_index_of(s, ch)` が正常にコンパイルされ、文字列内の最後の文字位置を返すこと。

### 実際の動作

Loop canonicalizer が `ConstStep` を要求しているが、このループは早期リターンを持つため Pattern2Break として認識されていない。

### 最小再現コード

```nyash
// apps/tests/phase257_p0_last_index_of_min.hako
static box Main {
    main() {
        local s = "hello world"
        local ch = "o"

        // Find last occurrence of ch in s
        local i = s.length() - 1

        loop(i >= 0) {
            if s.substring(i, i + 1) == ch {
                return i  // Early return when found
            }
            i = i - 1  // Backward scan
        }

        return -1  // Not found
    }
}
```

### 分析

#### ループ構造

1. **条件**: `i >= 0` (backward scan)
2. **ボディ**:
   - If branch: マッチング検出時
     - `return i` で早期リターン
   - After if: マッチなし
     - `i = i - 1` で 1 つ戻る（定数ステップ）
3. **特徴**:
   - **Early return**: break ではなく return を使用
   - **Backward scan**: `i--` で逆方向スキャン
   - **Capabilities**: `caps=If,Loop,Return` (no Break)
   - **Exits**: `exits=return` (両方の return を検出)

#### Pattern6（index_of）との比較

| Feature | Pattern6/index_of | last_index_of |
|---------|-------------------|---------------|
| 走査方向 | forward（`i = i + 1`） | reverse（`i = i - 1`） |
| 早期終了 | 見つかったら return / exit PHI | 見つかったら return |
| 通常終了 | not-found return（例: `-1`） | not-found return（`-1`） |
| JoinIR | `main/loop_step/k_exit` | 同じ語彙で表現できる |

#### 提案される実装アプローチ

**Option A: Pattern6 拡張（推奨 / 最小差分）**

Pattern6（ScanWithInit）を “scan direction” を持つ形に一般化する（forward / reverse）。

1. **検出条件**:
   - init: `i = s.length() - 1`（reverse）または `i = 0`（forward）
   - loop cond: `i >= 0`（reverse）または `i < bound`（forward）
   - body: `if s.substring(i, i + 1) == ch { return i }` + step update
   - step: `i = i - 1`（reverse const step）または `i = i + 1`（forward const step）

2. **JoinIR lowering**:
   - Pattern6 の語彙（`main/loop_step/k_exit`）を維持
   - scan direction に応じて:
     - stop 判定（`i < 0` or `i >= bound`）
     - const step（`+1` / `-1`）
   - return は既存の “exit PHI + post-loop guard” を必要最小で再利用する（DCE回避は既存Box優先）

3. **Boundary construction**:
   - Phase 256 系の契約（`JumpArgsLayout` / contract checks）に従い、推測しない
   - `expr_result` は `return i` の経路でのみ使用（not-found は `-1`）

---

## Progress

### P0（完了）

- Pattern6 を双方向 scan に拡張（forward/reverse）
  - reverse scan 用 lowerer を追加（`scan_with_init_reverse.rs`）
  - `apps/tests/phase257_p0_last_index_of_min.hako` を追加
- ただし当初は「PHI predecessor mismatch」が露出したため、P1 でインフラ不変条件を固定した

### P1（完了）

P1 で以下を実装し、runtime の `phi pred mismatch` を compile-time 側で捕捉・根治した:

- Pattern6 の誤検出防止（detect/extract SSOT）
- MIR verifier へ PHI predecessor 検証を追加（fail-fast）
- loop header PHI の entry edge を CFG から復元（self pred 根治）
- smoke の false positive を抑止（`--verify` + VM error 検出）

---

## Progress

### P0（完了）

- Pattern6 を双方向 scan に拡張（forward/reverse）
  - reverse scan 用 lowerer を追加
  - fixture: `apps/tests/phase257_p0_last_index_of_min.hako`

### P1（完了）

- Pattern6 の誤検出を防止（detect/extract SSOT 化）
  - `index_of_string/2` など “近いが別形” を `Ok(None)` で fall-through させる
- MIR verifier に PHI predecessor 検証を追加（fail-fast）
  - unreachable pred は除外して現実的に運用
- loop header PHI の entry edge を CFG から復元（self pred 根治）
  - merge 内で `terminator` 直書きにより `successors` が同期されないケースを補正
  - finalize 時点で host entry jump が未設定なため、host entry predecessor を明示的に補う
- smoke の false positive を抑止（`--verify` + VM error 検出）

## Next (Phase 258 proposal)

- `StringUtils.is_integer/1` の loop を JoinIR で受理する（caps=If,Loop,NestedIf,Return）
  - ループ形: `loop(i < s.length()) { if not is_digit(...) return false; i=i+1 } return true`
  - 前処理に nested-if がある（`start` の決定）

**Option B: Pattern8_ReverseScanReturn 新設**

Pattern6 を触らずに、reverse scan 専用パターンとして箱を追加する。
（影響範囲は狭いが、scan 系が分裂する）

**Option C: Normalization（将来）**

return/break を正規化して共通語彙へ落とし、JoinIR patterns を縮退させる。
（Phase 257 ではやらない）

### 実装計画

#### 推奨方針

**Option A**（Pattern6 拡張）を推奨。

理由:
- 既に index_of/find 系で “scan + not-found return” を Pattern6 が担っている
- last_index_of はその自然な派生（reverse scan + const step -1）
- Pattern2（break 前提）を膨らませずに済む

#### P0 タスク

1) **Fixture & integration smokes**（完了）
   - `apps/tests/phase257_p0_last_index_of_min.hako`
   - `tools/smokes/v2/profiles/integration/apps/phase257_p0_last_index_of_vm.sh`
   - `tools/smokes/v2/profiles/integration/apps/phase257_p0_last_index_of_llvm_exe.sh`

2) **Pattern6 detector/extractor 拡張（reverse scan）**
   - `src/mir/builder/control_flow/joinir/patterns/pattern6_scan_with_init.rs`
   - reverse scan 形を accept し、parts（init/cond/step/return/not-found）を抽出

3) **Pattern6 lowerer 拡張**
   - reverse scan の stop 判定・step を JoinIR へ落とす（語彙は既存 Pattern6 を維持）

4) **検証**
   - `bash tools/smokes/v2/profiles/integration/apps/phase257_p0_last_index_of_vm.sh`
   - `./tools/smokes/v2/run.sh --profile quick`（最初の FAIL が次へ進む）

### 注意（P0ではやらない）

- 正規化での return/break 統一（Phase 257 ではやらない）
- scan 系の大改造（まずは reverse scan の受理を最小差分で固定）

---

## 備考

- Phase 256 で Pattern2Break の境界構築が SSOT 化済み（entry_param_mismatch 根治）
- この知見を活かし、Pattern2Return も同じ原則で実装する
- "Early exit" として break と return を統一的に扱うことで、将来の拡張性も高まる

---

## 進捗（P0）

### ✅ 実装完了 (2025-12-20)

**実装内容**:
1. ✅ `ScanDirection` enum 追加 (Forward/Reverse)
2. ✅ `ScanParts` 構造体拡張（scan_direction フィールド追加）
3. ✅ `is_const_step_pattern()` 更新（`i + 1` / `i - 1` 両対応）
4. ✅ `extract_scan_with_init_parts()` 更新（forward/reverse 両検出）
   - Forward: `i < s.length()`, step +1
   - Reverse: `i >= 0`, step -1
5. ✅ `lower_scan_with_init_reverse()` 新規作成
6. ✅ Pattern6 lowering 分岐実装（scan direction に応じて適切な lowerer 選択）

**ファイル変更**:
- `src/mir/builder/control_flow/joinir/patterns/pattern6_scan_with_init.rs`
- `src/mir/join_ir/lowering/scan_with_init_reverse.rs` (新規)
- `src/mir/join_ir/lowering/mod.rs`

**ビルド状況**:
- ✅ コンパイル成功（0エラー）
- ✅ コード品質維持（既存パターン踏襲、SSOT原則遵守）

### 🔍 既知の問題（実装前から存在）

**PHI predecessor mismatch bug**:
- Error: `phi pred mismatch at ValueId(X): no input for predecessor BasicBlockId(Y)`
- Pattern6 forward scan (phase254_p0_index_of) でも同じエラー発生
- **Phase 257 P0 実装前から存在** していたバグ
- phase254 テストは期待終了コード=1 のため "PASS" と表示されるが、実際はエラー終了
- **Scope外**: Pattern6 全体の PHI 生成バグであり、Phase 257 P0 では修正しない

### 次のアクション

#### Phase 257 P1（将来）
- PHI predecessor mismatch bug 修正（JoinIR merger の PHI ノード生成ロジック）
- Pattern6 全体のテスト整備（forward/reverse 両方の正常動作確認）

#### Phase 258（次フェーズ）
- quick profile の次の FAIL へ進む
- last_index_of が通った後の最初のエラーを調査

---

**最終更新**: 2025-12-20 (Phase 257 P0 実装完了、PHI bug は既知の問題として文書化)
