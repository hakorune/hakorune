---
Status: closed__2026-09-22__WarningWrapInProgramRetireI141
Task: MIRBUILDER-WARNING-WRAP-IN-PROGRAM-RETIRE-I141
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i140-2026-09-22.md
Implementation permission: true; one production-zero test facade
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I142
---

# MirBuilder warning wrap-in-program retirement I141

## Six-line brief

```text
Decision: delete the unused wrap_in_program helper and construct the same
  ASTNode::Program directly in its test-only caller.
Source authority + canonical issuer: the existing ASTNode::Program shape and
  function-session checkpoint test; no production helper authority exists.
Non-authority: warning text alone, historical inventory rows, or a new AST
  wrapper/fallback.
Fail-fast boundary: any production caller, public reachable caller, changed
  test behavior, focused red, or stale active fixture reference.
Smallest next slice: remove the helper, update its one test caller and the
  current ingress fixture, then run the existing checkpoint guard.
Non-claims: no function-lowering route change, parser change, semantic
  receipt, backend change, or LegacyCallV0 retirement.
```

## Finite caller census and delete set

Boundary: `src/mir/builder/calls/function_lowering.rs:wrap_in_program` ->
`src/mir/builder/calls/function_session_tests.rs:run_injected_checkpoint`.

The source census is complete: the helper definition and one `#[cfg(test)]`
caller are the only `src/` references. The `calls` module is private inside
`MirBuilder`; no external API is exposed. Production lowering uses the
existing session/draft owners and has no helper call.

Delete exactly:

1. `wrap_in_program` from `function_lowering.rs`;
2. its one test-only call, replaced by the equivalent `ASTNode::Program`;
3. the stale `function-body-program-wrapper` evidence in
   `tools/checks/fixtures/resolved_lowering_ingress_inventory_v1.json`.

Historical archived inventories remain unchanged. No production lowering
route or AST shape is changed.

## Acceptance

- the existing `function_session_tests` checkpoint suite remains green;
- quick library check and test-binary build show the measured warning counts;
- `cargo fmt --all -- --check`, pointer guard, and `git diff --check` pass;
- `rg` confirms no `wrap_in_program` source or active fixture reference;
- no `#[allow]`, test deletion, or fallback is introduced.

## Closeout receipt

The helper and stale active ingress-fixture seam were deleted. The one
test-only caller now constructs the identical `ASTNode::Program` with
`Span::unknown()` directly. Source and active tools census are empty for
`wrap_in_program`; historical design snapshots remain unchanged.

Measured local evidence:

- `function_session_tests` focused suite: **6/6**;
- `cargo fmt --all -- --check`: **pass**;
- `CARGO_BUILD_JOBS=4 cargo check --profile quick --lib`: **pass**, lib
  warnings **1,676** (I140 baseline 1,677);
- `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib --no-run`: **pass**,
  lib-test warnings **544**;
- `current_state_pointer_guard.sh`: **pass**; `git diff --check`: **pass**.

No production lowering route or AST semantics changed.
