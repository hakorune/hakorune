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
2) `outcome.plan` is **not** in the allowlist below.

## Responsibility (SSOT)

Only `composer/shadow_adopt.rs` owns this guard. No other layer should duplicate
its logic or maintain a separate allowlist.

## Allowlist (DomainPlan)

- `GenericLoopV0`
- `GenericLoopV1`
- `LoopTrueBreakContinue`
- `LoopCondBreakContinue`
- `LoopScanMethodsV0`
- `LoopCondContinueOnly`
- `LoopScanPhiVarsV0`

## Log contract (dev/debug only)

- Tag: `[plan/freeze:nested_loop_guard]`
- Emitted only when `joinir_dev::debug_enabled()` is true.
- Format (single line, stable fields):
  - `func=<...> span=<...> plan=<...> pattern=<...> depth=<...>`

## Failure contract

- Error: `[plan/freeze:unstructured] nested loop requires plan/composer support: <plan> not in strict_nested_loop_guard allowlist`
- No fallback. The guard must remain fail-fast.
