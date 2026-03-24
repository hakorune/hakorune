---
Status: Active
Decision: accepted
Date: 2026-03-25
Scope: `Program(JSON v0)` bootstrap boundary を retire target として固定し、repo-wide external/bootstrap boundary を `MIR(JSON v0)` に統一する separate phase owner。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/phases/phase-29ch/README.md
  - docs/development/current/main/phases/phase-29ch/29ch-10-mir-direct-bootstrap-unification-checklist.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - docs/development/current/main/phases/phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md
  - docs/development/current/main/phases/phase-29ci/P5-STAGEB-MALFORMED-PROGRAM-JSON.md
  - docs/development/current/main/phases/phase-29cj/README.md
  - src/stage1/program_json_v0/README.md
  - src/runner/stage1_bridge/README.md
---

# Phase 29ci: Program JSON v0 Bootstrap Boundary Retirement

## Goal

`phase-29ch` で `temporary bootstrap boundary` に縮退した

- `src/stage1/program_json_v0.rs` cluster
- `src/runner/stage1_bridge/**` future-retire lane

を、authority migration と混ぜずに separate phase として retire する。

この phase は `MIR-direct bootstrap unification` ではない。
`phase-29ch` が固定した authority を前提に、bootstrap-only JSON v0 boundary の caller / owner / delete order を扱う。
execution-lane reading では、この phase は stage1 bridge/proof boundary だけを扱い、distribution policy は持たない。

## Status Reading

- current status は `reopen W7 active`。
- この phase の current goal は `Program(JSON v0)` の hard delete ではない。
- current repo では:
  - `Program(JSON v0)` = compat/internal/bootstrap-only keep + retire target
  - `MIR(JSON v0)` = sole external/bootstrap boundary
- この phase の fixed order を完了する前に、`JSON v0 は repo-wide で撤退済み` と読まない。

## Entry Conditions

1. `phase-29ch` の done judgment が green
   - reduced bootstrap proof can be explained without JSON v0 route authority
   - bridge is documented as `temporary bootstrap boundary` only
2. proof bundle is green on the current authority contract
   - Stage1/Stage2 rebuild
   - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
   - `tools/selfhost_identity_check.sh --mode {smoke,full} --skip-build`
3. `Program(JSON v0)` retirement work is not mixed back into `phase-29ch`

## Fixed Order

1. reclassify the remaining JSON v0 consumers into `public/deprecate-now`, `internal-compat-keep`, and `delete-ready-later`
2. retire public/bootstrap boundary reading first
3. keep internal compat routes explicit and non-public
4. keep proof bundle green after each retirement slice
5. only after caller inventory is empty, consider deleting the boundary itself

## P0 Inventory

- exact caller / owner matrices live in the P0/P1/P2/P3/P5 docs
- current focus is narrow and operational:
  - wrapper/helper retirement is landed
  - raw direct `stage1_cli.hako emit program-json` is diagnostics-only evidence
  - explicit env-route compat probes and raw compat flags stay alive
  - `stage1_cli.hako` / `launcher.hako` route orchestration thinning is landed
  - `tools/hakorune_emit_mir.sh` helper-local splits are landed: Stage-B Program(JSON) production and direct-emit fallback policy
  - next cleanup slice moves to `tools/selfhost/selfhost_build.sh` and starts with the Program(JSON)->MIR->EXE consumer path
- keep this README as the phase entry point, not the evidence log

## Current Retirement Targets

- public/bootstrap boundary first:
  - wrapper/helper surface `tools/selfhost/run_stage1_cli.sh emit program-json` (landed)
  - wrapper/helper surface `tools/selfhost/selfhost_build.sh --json` (landed)
  - exact smoke/docs that still present those wrappers as live
- raw direct diagnostics pin:
  - `lang/src/runner/stage1_cli.hako` raw `emit program-json` lane is retire-only / diagnostics-only
  - `tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh` pins that retired lane as an absence proof
- raw compat keep after wrapper retirement:
  - CLI `--emit-program-json-v0`
  - CLI `--hako-emit-program-json`
  - CLI `--program-json-to-mir`
  - Stage1 bridge explicit `emit-program-json-v0` route
- compat/internal keep after that:
  - `src/stage1/program_json_v0.rs` cluster
  - `src/runner/stage1_bridge/program_json/**`
  - `src/runner/stage1_bridge/program_json_entry/**`
  - `.hako` live/bootstrap callers
  - compiled-stage1 / shell callers that still terminate in MIR

## Non-goals

- reopening `phase-29cg` solved reduction buckets
- re-arguing `phase-29ch` authority migration
- widening compat keep or raw direct `stage1-cli` authority

## Acceptance

- retirement work can be explained without mixing authority migration back into `phase-29ch`
- remaining JSON v0 consumers are inventoried with exact owners and boundary class
- public/bootstrap docs and CLI/help read `MIR(JSON)` as the only supported boundary
- at least one compat-only Program(JSON) route remains green and explicitly marked non-public
- wrapper/public helper retirement is pinned by exact smoke and explicit compat probe
- raw direct `stage1_cli.hako` `emit program-json` lane is pinned as retired diagnostics-only evidence
- hard delete is not started in the same wave

## Next Phase Pointer

- next Rust-owned retirement wave:
  - `docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md`
