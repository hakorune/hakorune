---
Status: SSOT
Date: 2026-04-03
Scope: main ラインの current summary と正本リンクだけを置く薄い mirror/dashboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
---

# Self Current Task — Now (main)

## Purpose

- この文書は docs 側の薄い mirror/dashboard。
- current summary と正本リンクだけを置く。
- landed history や inventory detail は `CURRENT_TASK` と phase docs に逃がす。

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`
- layout contract: `docs/development/current/main/DOCS_LAYOUT.md`

## Current

- lane: `phase-45x vm residual cleanup`
- `45xA1` landed: residual vm owner inventory lock
- `45xA2` active: proof-only keep boundary freeze
- `45xB1` next: vm.rs broad owner shrink
- `44xE1` landed: proof / closeout
- `phase-30x` landed: backend roles and docs/artifact/smoke ownership are settled
- `phase-31x` landed: low-blast engineering rehome and shim drain are complete
- `phase-32x` landed: mixed-owner source/smoke split and raw default/token defer are fixed
- `phase-33x` landed: helper-family path truth and broad keep gates are fixed
- `34xA1` landed: `child.rs` shell/process/capture ownership is fixed around `run_ny_program_capture_json_v0`
- `34xA2` landed: `stage1_cli/core.hako` raw compat residue is fixed around `run_program_json` / `_run_raw_request`
- `34xA3` landed: `core_executor` is fixed as the direct MIR(JSON) owner seam
- `34xB1` landed: `child.rs` shell residue is mechanically split into private route-neutral helpers
- `34xC1` landed: raw compat lane is explicitly no-widen; thread/runtime capability work is barred from `run_program_json`
- `34xD1` landed: direct MIR(JSON) handoff is proof-pinned by `execute_mir_json_text_*` tests
- `35xA1` landed: captured Stage-A payload resolution moved into `stage_a_compat_bridge::resolve_captured_payload_to_mir(...)`
- `35xA2` landed: `selfhost.rs` delegates Stage-A child spawn/setup to `stage_a_route.rs` and stays orchestration/terminal-accept only
- `35xB1` landed: Program(JSON v0) compat lane is fixed as explicit/no-widen in `stage_a_policy.rs` and `stage_a_compat_bridge.rs`
- `35xC1` landed: direct-vs-compat Stage-A route is proof-pinned by evidence commands and focused tests
- `36xA1` landed: `source_prepare.rs` now owns source extension gate / source read / using merge / preexpand / tmp staging
- `36xA2` landed: `selfhost.rs` is explicitly route ordering / macro gate / terminal accept owner
- `36xB1` landed: `raw_subcommand_emit_mir.hako` now owns raw `emit mir-json` request/materialize/emit glue
- `36xB2` landed: `raw_subcommand_run.hako` now owns raw `run` request/script-args env/Program(JSON) materialization glue
- `36xC1` landed: proof/closeout evidence is fixed; raw bridge split does not reopen compat ownership
- `40xA1` landed: archive candidate caller inventory is fixed
- `40xA2` landed: route classes are fixed as `must-split-first`, `proof-only keep`, `compat keep`, `archive-later`, and `direct-owner target`
- active micro task: `44xA2 selfhost_build_stageb.sh direct/core-first cutover`
- next micro task: `44xB1 selfhost_run_routes.sh runtime default cutover`
- plain reading:
  - if a bootstrap route stays on `--backend vm`, new capability work still tends to imply `rust-vm` support
  - `phase-41x` hardened the remaining direct/core mainline and kept vm as proof/compat keep
- `phase-42x` is landed; it starved day-to-day callers away from vm-gated routes and moved owner pressure toward direct/core seams
- `phase-43x` is landed; it selected `phase-44x stage0 direct/core follow-up` as the highest-leverage successor lane
- `phase-44x` is landed; it keeps proof-only VM gates explicit and closes the lane
- `phase-45x` is active; it shrinks residual vm owner surfaces while keeping direct/core mainline canonical
  - `tools/selfhost/lib/selfhost_build_stageb.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `stage0_capture.rs` is already route-neutral
  - success means day-to-day stage0/selfhost defaults stay direct/core-first and VM gates stay explicit proof/fallback only
  - failure means new features drifting back into `--backend vm`, stage1 compat, or raw routes
- post-`39xD1`: stage0 vm archive candidate selection for remaining bootstrap surfaces
- landed first cleanup move: `tools/archive/legacy-selfhost/stage1_embedded_smoke.sh`
- `37xD1` evidence:
  - `cargo check --bin hakorune` PASS
  - `git diff --check` PASS
  - `bash tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh` PASS
  - `bash tools/selfhost/stage1_mainline_smoke.sh --bin target/selfhost/hakorune.stage1_cli.stage2 apps/tests/hello_simple_llvm.hako` PASS
- inherited red outside D1 acceptance:
  - `bash tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh`
  - upstream Stage-B source-route red: `Undefined variable: StageBMod`
- backend reading:
  - `llvm/exe` = `product`
  - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
  - `vm-hako` = `reference/conformance`
  - `wasm` = `experimental`
- raw backend default/token rewrite stays deferred beyond `phase-42x`
- source/smoke cleanup rule:
  - `split/rehome/drain -> delete`
- vm thinning rule:
  - `move owner to hakorune binary direct/core routes -> freeze proof-only vm gates -> archive drained vm-facing shims`
- speed rule:
  - temporary smoke red is acceptable inside `37xA` / `37xB`
  - keep `cargo check --bin hakorune` and `git diff --check` green

## Read Next

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read `docs/development/current/main/phases/phase-45x/README.md`
4. read `docs/development/current/main/phases/phase-45x/45x-90-vm-residual-cleanup-ssot.md`
5. read `docs/development/current/main/phases/phase-45x/45x-91-task-board.md`
