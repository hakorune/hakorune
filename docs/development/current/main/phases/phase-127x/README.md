# Phase 127x: compat route raw vm cut prep

- 目的: compat boundary smoke を raw `vm-route/*` tag 依存から route-first contract へ移し、`runtime-route compat` の temp-MIR handoff を安定化する。
- 到達:
  - `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
- success:
  - compat boundary smoke は raw `vm-route/pre-dispatch` / `lane=vm-compat-fallback` を contract pin しない
  - route-first tag (`runtime_route=compat`, `mode=stage-a-compat`) と explicit fallback env だけで compat keep を説明できる
  - `run.sh --runtime --runtime-route compat` が route-first smoke bundle で green

## Decision Now

- `phase-127x` is landed
- compat temp-MIR handoff is green again because the helper receives the parser-EXE preference env internally
- the next source seam is `stage1_bridge`:
  - `src/runner/stage1_bridge/plan.rs`
  - `src/runner/stage1_bridge/args.rs`
  - `src/runner/stage1_bridge/env/stage1_aliases.rs`
  - `src/config/env/stage1.rs`
  - `src/runner/stage1_bridge/direct_route/mod.rs`

## Next

1. inventory which `stage1_bridge` helper still keeps `backend=vm`
2. decide whether direct-route callers can stay route-first only
3. keep compat fallback explicit while shrinking the bridge hint chain
4. then move to `phase-128x stage1 bridge vm gate softening`
