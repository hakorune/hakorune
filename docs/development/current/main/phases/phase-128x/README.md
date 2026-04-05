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
  - binary-only direct-route vm gate remains explicit legacy and isolated

## Decision Now

- `phase-127x` is landed
- `phase-128x` landed with the `stage1_bridge` backend-hint chain narrowed
- default `stage1_cli_env.hako` child path no longer forwards backend hints
- binary-only direct-route vm gate remains an explicit legacy contract
- `phase-129x` follows the remaining public vm gate / orchestrator surfaces

## Next

1. inventory remaining public `vm` wording in CLI/help/docs
2. decide whether any public `--backend vm` callsites can be demoted without breaking explicit legacy keep/debug callers
3. keep the direct-route legacy gate isolated
4. then move to the next vm public-gate cleanup lane
