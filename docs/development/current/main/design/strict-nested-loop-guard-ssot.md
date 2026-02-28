Status: SSOT
Scope: JoinIR composer `strict_nested_loop_guard` (Phase-agnostic)
Related:
- Entry: `docs/development/current/main/10-Now.md`
- Debug contract: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`

# strict-nested-loop-guard SSOT

## Purpose

Fail-fast when a nested loop is detected but there is no plan/composer support
for that shape. This prevents silent fallback or mis-lowering in strict/dev modes.

## Trigger (exact)

Guard fires when **both** are true:

1) `nested_loop == true` (from facts or `detect_nested_loop(ctx.body)`), and  
2) `strict_nested_loop_guard` allowlist 判定に一致しない。

## Responsibility (SSOT)

Only `composer/shadow_adopt.rs` owns this guard. No other layer should duplicate
its logic or maintain a separate allowlist.

## Allowlist (actual contract)

- `generic_loop_v0` または `generic_loop_v1` facts が存在する形
- `Pattern4Continue accept-min1` 形（strict-nested 最小受理）
  - `pattern_kind == Pattern4Continue`
  - `exit_usage`: continueあり / breakなし / returnなし
  - `pattern4_continue` factsあり
  - `carrier_updates.len() == 1`
  - `condition`: `<loop_var> < <int>`
  - `continue_condition`: `<loop_var> >= <int>`
  - `continue_condition` の下限値 `> condition` の上限値（continue が到達不能）
  - `loop_increment`: `<loop_var> + 1`

## Log contract (dev/debug only)

- Tag: `[plan/freeze:nested_loop_guard]`
- Emitted only when `joinir_dev::debug_enabled()` is true.
- Format (single line, stable fields):
  - `func=<...> span=<...> plan=<...> pattern=<...> depth=<...>`

## Failure contract

- Error: `[plan/freeze:unstructured] nested loop requires plan/composer support: <plan> not in strict_nested_loop_guard allowlist`
- No fallback. The guard must remain fail-fast.
