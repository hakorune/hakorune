# Phase 128x: stage1 bridge vm gate softening

- 目的: `stage1_bridge` の backend-hint chain を薄くして、`runtime-route compat` が raw `--backend vm` に依存しないようにする。
- 対象:
  - `src/runner/stage1_bridge/plan.rs`
  - `src/runner/stage1_bridge/args.rs`
  - `src/runner/stage1_bridge/env/stage1_aliases.rs`
  - `src/config/env/stage1.rs`
  - `src/runner/stage1_bridge/direct_route/mod.rs`
  - `src/runner/stage1_bridge/route_exec/direct.rs`
- success:
  - compat boundary smoke is green with route-first contract
  - `stage1_bridge` backend-hint chain is source-backed and narrow
  - default `stage1_cli_env.hako` child paths no longer forward backend hints
  - raw `--backend vm` is no longer treated as a public compat/direct bridge surface

## Decision Now

- `phase-127x` is landed
- `phase-128x` softens the `stage1_bridge` backend-hint chain next
- `stage1_bridge/direct_route/mod.rs` is the first hard-gate seam to inventory

## Next

1. isolate which helper still requires `backend=vm`
2. keep compat fallback explicit while shrinking the bridge hint chain
3. decide whether the binary-only direct-route vm gate should stay as an explicit legacy contract
4. then move to `phase-129x vm orchestrator/public gate follow-up`
