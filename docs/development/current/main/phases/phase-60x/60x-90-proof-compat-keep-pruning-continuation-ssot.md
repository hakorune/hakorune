---
Status: Landed
Date: 2026-04-04
Scope: continue pruning explicit proof/compat keeps after route/default/help retirement narrowed the outer rust-vm surface.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-59x/README.md
---

# 60x-90 Proof/Compat Keep Pruning Continuation SSOT

## Intent

- keep attacking the explicit proof/compat keep bucket without reopening route/default affordances
- prefer wording/contract/seam narrowing over premature source deletion
- leave `vm-hako` reference/conformance surfaces untouched in this lane

## Highest-Leverage Keep Surfaces

- `tools/selfhost/lib/selfhost_run_routes.sh`
  - `stage-a-compat` remains an explicit compat-only route and should stay narrow
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - still owns Program(JSON v0) compat bridge logic
- `src/runner/modes/vm_fallback.rs`
  - explicit compat fallback interpreter keep
- `lang/src/runner/stage1_cli/core.hako`
  - raw compat hold line
- proof smoke wrappers
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
  - `tools/plugins/plugin_v2_smoke.sh`

## Inventory Lock

- `tools/selfhost/run_stageb_compiler_vm.sh`
  - classification: `proof-only keep`
  - current callers: `tools/selfhost/lib/selfhost_run_routes.sh` direct proof path, Stage-B proof smokes, parser trace proof smoke
  - lock: keep explicit, keep guarded by `NYASH_SELFHOST_STAGEB_PROOF_ONLY=1`, do not widen ordinary callers
- `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - classification: `proof-only keep`
  - current role: c0/c1/c1' bootstrap parity smoke
  - lock: keep as proof smoke only; do not reintroduce it as a default bootstrap route
- `tools/selfhost/selfhost_smoke.sh`
  - classification: `proof-only keep`
  - current role: explicit selfhost emit/compare proof smoke
  - lock: keep explicit; do not grow it into a broad day-to-day smoke matrix
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`
  - classification: `proof-only keep`
  - current role: stage3 acceptance contract bridging `--direct` producer and vm compat consumer
  - lock: keep explicit; do not let it become a general runtime gate
- `tools/plugin_v2_smoke.sh`
  - classification: `proof-only keep`
  - current role: plugin-host compatibility proof smoke
  - lock: keep as plugin proof only; do not route ordinary plugin guidance through it
- `tools/selfhost/lib/selfhost_run_routes.sh` `stage-a-compat`
  - classification: `compat keep`
  - current role: explicit shell compat entry that still routes to `--backend vm`
  - lock: keep explicit and non-default; do not widen aliases beyond `stage-a`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - classification: `compat keep`
  - current role: payload-family resolution plus explicit Program(JSON)->MIR fallback
  - lock: keep narrow; future pruning should split direct MIR acceptance from Program(JSON) fallback instead of widening this bridge
- `src/runner/modes/vm_fallback.rs`
  - classification: `compat keep`
  - current role: explicit `NYASH_VM_USE_FALLBACK=1` interpreter lane
  - lock: keep explicit and guarded; prune internal helper ownership before any deletion claim
- `lang/src/runner/stage1_cli/core.hako`
  - classification: `compat keep`
  - current role: raw Program(JSON) hold line with `vm|pyvm` accept and `llvm` reject
  - lock: keep thin and non-growing; no new backend policy or capability work lands here

## Boundary Freeze

- proof-only keeps:
  - remain opt-in or proof-script-only
  - stay off default runtime/bootstrap/selfhost routes
  - may be narrowed or archived later, but are not deleted in `60x`
- compat keeps:
  - remain explicit and non-default
  - accept only current compat payload/bridge contracts
  - must not receive new mainline capability work
- out of scope in `60x`:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_hako.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - these stay in the later `61x -> 62x -> 63x` retirement corridor

## Landed This Lane

- `60xB1 stage-a compat seam pruning`
  - `src/runner/modes/common_util/selfhost/stage_a_route.rs`
    - direct MIR acceptance stays here
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
    - narrowed to Program(JSON) compat fallback only
  - read-as:
    - mainline-like MIR acceptance no longer lives inside the compat bridge
    - the compat seam is thinner without changing shell/runtime contracts
- `60xB2 vm_fallback/core.hako keep pruning continuation`
  - `src/runner/modes/vm_fallback.rs`
    - removed caller-zero helper `execute_vm_fallback_from_ast(...)`
  - `lang/src/runner/stage1_cli/core.hako`
    - kept unchanged as the thin raw compat hold line
  - read-as:
    - the explicit compat fallback keep shrinks by deletion first
    - `core.hako` remains present because the raw Program(JSON) boundary still has live compat callers

## Proof

- `cargo check --bin hakorune` PASS
- `git diff --check` PASS
- `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh` PASS
- `bash tools/selfhost/selfhost_smoke.sh` PASS
- `bash tools/selfhost/bootstrap_selfhost_smoke.sh` PASS

## Handoff

- `60x` closes as a pruning lane only.
- successor lane:
  - `61x residual rust-vm caller-zero audit rerun`

## Boundaries

- do not reopen CLI/default/backend selection work from phase-59x
- do not mix `vm-hako` reference/conformance cleanup into this lane
- do not claim delete-ready broad rust-vm source until caller-zero facts materially change

## Successor Corridor Lock

- after `60x`, the rust-vm retirement corridor is fixed as:
  1. `61x residual rust-vm caller-zero audit rerun`
  2. `62x rust-vm delete-ready removal wave`
  3. `63x rust-vm final retirement decision`
- `60x` is therefore a pruning lane, not a final-retirement claim
