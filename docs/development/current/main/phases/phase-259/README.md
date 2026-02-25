Status: ✅ P0 Complete
Scope: `StringUtils.is_integer/1`（nested-if + loop）を JoinIR で受理して `--profile quick` を進める。
Related:
- Now: `docs/development/current/main/10-Now.md`
- Phase 258: `docs/development/current/main/phases/phase-258/README.md`
- Design goal: `docs/development/current/main/design/join-explicit-cfg-construction.md`

# Phase 259: `StringUtils.is_integer/1` (nested-if + loop)

## P0 Result (2025-12-21)

- **is_integer/1**: Pattern8 で認識・実行成功 ✅
- **VM smoke test**: `[PASS] phase259_p0_is_integer_vm` ✅
- **Exit code**: 7（is_integer("123") == true）
- **json_lint_vm**: まだ FAIL（別問題: nested-loop with break / Pattern2）

### Key Fixes Applied

1. `expr_result = Some(join_exit_value)` - Pattern7 style で明示設定
2. `loop_var_name = Some(parts.loop_var.clone())` - merge_entry_block 選択用
3. `loop_invariants = [(me, me_host), (s, s_host)]` - PHI-free 不変量パラメータ
4. `skipped_entry_redirects` - k_exit のスキップ時ブロック参照リダイレクト

## Current Status (SSOT)

- ✅ is_integer/1 は Pattern8 で解決
- ❌ json_lint_vm は別の nested-loop with break パターンで失敗中（Phase 260+）

### Next FAIL (Phase 260+)

- **Function**: `Main.main/0` in `apps/examples/json_lint/main.hako`
- **Error**: `[cf_loop/pattern2] Failed to extract break condition from loop body`
- **Pattern**: Nested loop（外側 `loop(i < cases.length())` 内で内側 `loop(j < valid.length())` + break）
- **AST Structure**:
  ```
  loop(i < cases.length()) {
    local s = ...
    local ok = 0
    local j = 0
    loop(j < valid.length()) {  // ← 内側ループ
      if (s == valid.get(j)) {
        ok = 1
        break  // ← Pattern2 が抽出失敗
      }
      j = j + 1
    }
    if (ok == 1) { print("OK") } else { print("ERROR") }
    i = i + 1
  }
  ```
- **Reproduce**:
  ```bash
  ./target/release/hakorune --backend vm apps/examples/json_lint/main.hako
  ```
- Shape summary（ログ由来）:
  - prelude: nested-if to compute `start` (handles leading `"-"`)
  - loop: `loop(i < s.length()) { if not this.is_digit(s.substring(i, i+1)) { return false } i = i + 1 }`
  - post: `return true`
  - caps: `If,Loop,NestedIf,Return`

## Goal

- `StringUtils.is_integer/1` を JoinIR で受理し、quick の first FAIL を次へ進める

## Proposed Approach (P0)

**P0 Design Decision: Pattern8（新規）採用**

### Why Pattern8?

Pattern6（index_of系）は "見つける" scan（返り値: 整数 i or -1）で、is_integer は "全部検証する" predicate scan（返り値: 真偽値 true/false）。役割が異なるため、Pattern8 として分離した。

### Pattern8 vs Pattern6

| | Pattern6 (index_of系) | Pattern8 (is_integer系) |
|---|---|---|
| 役割 | "見つける" scan | "全部検証する" predicate scan |
| Match形 | `substring(...) == needle` | `not predicate(ch)` → early exit |
| 返り値 | Integer (i or -1) | Boolean (true/false) |
| Exit PHI | `i`（ループ状態変数） | `ret_bool`（検証結果） |
| Carriers | [i] (LoopState) | [] (empty, expr_result のみ) |

### JoinIR Contract

- **jump_args_layout**: ExprResultPlusCarriers（carriers=0）
- **expr_result**: Some(join_exit_value) - ret_bool from k_exit (pipeline handling)
- **exit_bindings**: Empty（carriers なし）
- **SSOT**: `join_inputs = entry_func.params.clone()`
- **Me receiver**: Passed as param [i, me, s] (by-name 禁止)

### 受理形（P0固定）

```nyash
loop(i < s.length()) {
    if not this.is_digit(s.substring(i, i + 1)) {
        return false
    }
    i = i + 1
}
return true
```

- prelude の start 計算は許可（ただし i_init = start で渡す）
- predicate は Me method call（this.is_digit）のみ
- step は 1 固定
