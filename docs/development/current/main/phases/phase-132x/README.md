# Phase 132x: vm default backend decision

- 目的: `args.rs` の default backend が `vm` のままでよいかを、omitted-backend caller inventory を根拠に最後に決める。
- 対象:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/stage1_bridge/direct_route/mod.rs`
  - `tools/stage1_minimal.sh`
  - `tools/stage1_debug.sh`
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_direct_emit_dominance_block_vm.sh`
- success:
  - omitted-backend caller inventory is complete
  - `--backend vm` default is either retained as an explicit legacy keep/debug default or moved only after all dependent callers are updated
  - help/docs wording matches the final decision
  - phase-131x migration lands cleanly into this decision lane

## Decision Now

- the legacy `vm` contract smoke is already archived
- default child path is backend-hint free
- direct-route selection is narrowed
- the legacy emit-mode special-case has been removed from `route_orchestrator.rs`
- the last open question is whether `args.rs` should keep `vm` as the default backend or move to an explicit legacy-only surface

## Next

1. inventory callers that omit `--backend`
2. decide whether `vm` should remain the default backend
3. update help/docs/callers in one shot after the decision
