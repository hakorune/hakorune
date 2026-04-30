---
Status: SSOT
Decision: settled
Date: 2026-04-02
Scope: backend surface simplification の role taxonomy、fixed order、dangerous early flips を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-30x/README.md
  - docs/development/current/main/phases/phase-30x/30x-91-task-board.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/artifact-policy-ssot.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
---

# 30x-90 Backend Surface Simplification

## Goal

- backend を `product / engineering / reference / experimental` の 4 役に固定する。
- `llvm/exe` を user-facing main に寄せる。
- `rust-vm` は engineering(stage0/bootstrap + tooling keep) lane として explicit keep にする。
- `vm-hako` は reference/conformance lane として残し、mainline と混ぜない。
- `wasm` は experimental target として扱い、promotion 判定を分離する。

## Fixed Role Taxonomy

| Surface | Role | Fixed reading |
| --- | --- | --- |
| `llvm/exe` / `ny-llvm` / `ny-llvmc` | `product` | daily mainline / CI / distribution target |
| `rust-vm` (`--backend vm`) | `engineering` | stage0/bootstrap / recovery / compat / tooling keep |
| `vm-hako` | `reference` | semantic witness / conformance / debug |
| `wasm` / `--compile-wasm` | `experimental` | feature-gated compile target |

## Fixed Rules

- `rust-vm` を phase 冒頭で剥がさない。
- raw CLI backend token/default は `30xF` まで変えない。
- `vm-hako` は reference lane のままにし、co-main にしない。
- `wasm` は experimental のままにし、promotion は別 gate を要求する。
- selfhost/bootstrap/plugin/macro/smoke orchestration の `--backend vm` 直打ちは inventory 後にしか触らない。

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `30xA role taxonomy lock` | landed | docs と mirrors の backend role labels を揃える | active lane と role-first reading が root docs で一致する |
| `30xB smoke taxonomy split` | landed | smoke を `product / engineering / reference / experimental` の見え方へ寄せる | role-first buckets と suites の方針が固定される |
| `30xC rust-vm dependency inventory` | landed | internal `--backend vm` pressure を category ごとに固定する | bootstrap/selfhost/plugin/macro/smoke/doc の pressure map が揃う |
| `30xD dangerous-early-flip lock` | landed | 先に変えると壊れる launcher/default/orchestrator を固定する | early-flip denylist が task board で explicit |
| `30xE user-facing main switch prep` | landed | README/help/examples を `llvm/exe` first に寄せる準備をする | default を変えずに main narrative だけ切り替える差分範囲が固まる |
| `30xF backend default decision gate` | landed | CLI default/backend flip の可否を最後に判定する | raw default は据え置き、ownership flip を phase-30x の結論にする |
| `30xG legacy disposition sweep` | landed | manual residue / stale snapshot / old compare helpers を archive か delete に寄せる | open residue is settled as archive/delete/explicit keep |

## Micro Tasks

### `30xA` role taxonomy lock

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xA1` | landed | root mirrors use the same role-first labels | `CURRENT_TASK`, `05`, `10`, and `15` read `product / engineering / reference / experimental` |
| `30xA2` | landed | design role SSOT alignment | `artifact-policy` and `execution-lanes` agree on the same four-role reading |

### `30xB` smoke taxonomy split

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xB1` | landed | reference smoke lock | `vm-hako` suites/readmes read as `reference`, not active mainline |
| `30xB2` | landed | experimental smoke lock | `wasm` suites/readmes read as `experimental`, not co-main |
| `30xB3` | landed | product/probe boundary lock | `llvm/exe` product lane and `llvmlite` compat/probe keep are not mixed |
| `30xB4` | landed | matrix/guide cleanup | smoke discovery docs and matrix config use the same role-first reading |

### `30xC` rust-vm dependency inventory

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xC1` | landed | bootstrap/selfhost inventory | launcher, stage1, selfhost wrappers are grouped explicitly |
| `30xC2` | landed | plugin/macro/tooling inventory | macro child, plugin smoke, and dev tooling are grouped into explicit keep vs archived/manual residue |
| `30xC3` | landed | smoke/test inventory | engineering smoke keeps, mixed orchestrators, and manual residues are separated |
| `30xC4` | landed | docs/help inventory | rewrite targets, engineering keeps, and stale snapshots are separated |

### `30xD` dangerous-early-flip lock

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xD1` | landed | default/dispatch freeze | CLI default and central dispatch are marked `do not flip early` |
| `30xD2` | landed | selfhost/bootstrap freeze | selfhost/stage1 wrappers and scripts are explicit no-touch-first surfaces |
| `30xD3` | landed | plugin/smoke orchestrator freeze | plugin and smoke orchestrators are explicit no-touch-first surfaces |

### `30xE` user-facing main switch prep

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xE1` | landed | README/README.ja prep | root front docs read `llvm/exe` first while `rust-vm` stays engineering keep |
| `30xE2` | landed | CLI/help wording prep | `docs/tools/*` stop reading `vm` as the main narrative and stale help is marked historical |
| `30xE3` | landed | stage1/runtime guide prep | runtime/stage1 guides stop implying `rust-vm` is the product main |
| `30xE4` | landed | vm-hako/wasm wording prep | `vm-hako` stays reference and `wasm` stays experimental in user-facing docs |

### `30xF` backend default decision gate

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xF1` | landed | backend default gate checklist | raw default flip remains blocked even after `30xB-30xE` landed |
| `30xF2` | landed | backend token/default decision | docs-first demotion is enough for phase-30x; raw token/default flip stays deferred |

#### `30xF1` gate checklist result

- landed prerequisites:
  - `30xB` smoke taxonomy split
  - `30xC` rust-vm dependency inventory
  - `30xD` dangerous-early-flip lock
  - `30xE` user-facing main switch prep
- still-blocking surfaces:
  - `src/cli/args.rs`
    - raw backend default is still `vm`
    - help text still says `Backend: vm (default), vm-hako (S0 frame), llvm, interpreter`
  - `src/runner/dispatch.rs`
    - central backend selector still routes `vm` / `vm-hako` / `llvm` as raw tokens
    - unknown-backend and dispatch semantics would all change at once if the default flips now
  - `src/runner/modes/common_util/selfhost/child.rs`
    - child capture still shells out with `--backend vm`
  - engineering wrappers/orchestrators:
    - `tools/selfhost/run.sh`
    - `tools/selfhost/selfhost_build.sh`
    - `tools/bootstrap_selfhost_smoke.sh`
    - `tools/plugin_v2_smoke.sh`
    - `tools/selfhost/selfhost_smoke.sh`
    - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
- result:
  - `30xF1` does **not** permit a raw default flip
  - `30xF2` decides whether docs-first demotion is already enough, or whether a deliberate raw token/default change is worth the blast radius

#### `30xF2` decision result

- phase-30x decision:
  - docs-first demotion is enough
  - raw backend token/default flip stays deferred beyond phase-30x
- fixed reading:
  - `llvm/exe` owns the product narrative through artifacts, docs, and smoke ownership
  - `rust-vm` stays the explicit engineering(stage0/bootstrap + tooling keep) runtime
  - `vm-hako` stays reference/conformance
  - `wasm` stays experimental/monitor-only
- why raw flip is still deferred:
  - `args.rs` and `dispatch.rs` still define the deep runtime token/default semantics
  - selfhost/bootstrap/plugin orchestrators still carry live `--backend vm` contracts
  - changing the default now would mix ownership cleanup with launcher/runtime behavior change
- phase-30x conclusion:
  - ownership flip lands in docs/artifacts/smokes
  - raw default/backend token change is a later, separate gate

### `30xG` legacy disposition sweep

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xG1` | landed | manual smoke residue archive pass | manual residue scripts are either archived or reclassified as explicit engineering keeps |
| `30xG2` | landed | stale help snapshot replacement/archive | `docs/tools/nyash-help.md` is reduced to a thin current stub and the old capture moves to archive |
| `30xG3` | landed | compare/manual helper archive pass | legacy compare/manual helpers such as `tools/smoke_aot_vs_vm.sh` are either kept with explicit engineering meaning or archived |
| `30xG4` | landed | post-switch docs cleanup | root/phase docs stop carrying unsettled residue wording for resolved residues |

#### `30xG2` result

- `docs/tools/nyash-help.md` now stays as a thin current-entry stub
- the old captured help text moved to `docs/archive/tools/nyash-help.md`
- current CLI truth remains:
  - `docs/tools/cli-options.md`
  - `src/cli/args.rs`

#### `30xG3` result

- `tools/smoke_aot_vs_vm.sh` moved to `tools/archive/manual-smokes/smoke_aot_vs_vm.sh`
- root README and README.ja no longer present the old parity helper as a front-door example
- explicit compare/manual keep after `30xG3`:
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`

#### `30xG4` result

- root mirrors and phase docs now read settled residues as:
  - explicit keep
  - rewrite
  - archive
  - delete
- temporary residue wording is no longer used for already-settled manual/doc residues in phase-30x docs
- remaining explicit keep after `30xG4`:
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
- phase-30x is now in closeout review, not active residue triage

## Legacy Disposition Rules

- legacy residue は implicit に残さない。
- residue surface は `rewrite / explicit keep / archive / delete` のどれかで固定する。
- `rust-vm` を使っていても engineering/bootstrap の live contract なら keep に残す。
- product/main narrative から外れた manual residue は archive/delete を優先する。
- delete-ready が出ない場合でも owner-facing docs からは外し、archive か explicit keep に寄せる。

## Current Focus

- active macro wave: `phase-30x landed closeout`
- next queued wave: `none`
- later disposition wave: `none`
- current blocker: `none`
- predecessor lane: `phase-29x backend owner cutover prep` is landed enough and no longer the active docs front

## Internal Pressure Buckets

### Bootstrap / selfhost

- `src/cli/args.rs`
- `src/runner/dispatch.rs`
- `src/runner/modes/common_util/selfhost/child.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `lang/src/runner/stage1_cli/config.hako`
- `lang/src/runner/stage1_cli/raw_subcommand_input.hako`
- `tools/selfhost/run.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `Makefile`

Bootstrap/selfhost findings (`30xC1`):

- `src/cli/args.rs`
  - raw CLI default still reads `--backend vm` as the default token
  - this remains an early-flip denylist surface, not a first-slice edit target
- `src/runner/dispatch.rs`
  - runtime backend selection still exposes `vm`, `vm-hako`, and `llvm`
  - backend token wording stays frozen until `30xF`
- `src/runner/modes/common_util/selfhost/child.rs`
  - selfhost child capture is explicitly `nyash --backend vm <program>`
  - this is bootstrap/runtime glue, not product-mainline evidence
- `lang/src/runner/stage1_cli/core.hako`
  - raw compat route still accepts `vm|pyvm`
  - `llvm` is explicitly retired from this raw stage1 lane
- `tools/selfhost/run.sh`
  - runtime/direct selfhost paths still execute with `--backend vm`
  - this is engineering/bootstrap keep, not a stale path
- `tools/selfhost/selfhost_build.sh`
  - BuildBox and Stage-B wrappers still call `--backend vm`
  - these remain bootstrap/selfhost surfaces, not delete candidates
- `tools/selfhost/run_stageb_compiler_vm.sh`
  - explicit shared Stage-B compiler route on Rust VM core lane
  - keep as bootstrap/selfhost contract surface
- `Makefile`
  - `run-minimal` still uses `--backend vm`
  - keep as engineering quick target; do not flip in this slice

Bootstrap/selfhost archive/delete result (`30xC1`):

- none
- every direct `--backend vm` hit in this bucket still belongs to live bootstrap/selfhost or launcher pressure
- archive/delete review should wait until `30xD` denylist plus `30xE-30xF` main-switch/default decisions

### Plugin / macro / dev tooling

- `src/macro/macro_box_ny.rs`
- `tools/bootstrap_selfhost_smoke.sh`
- `tools/plugin_v2_smoke.sh`
- `tools/archive/manual-smokes/ny_stage1_asi_smoke.sh`
- `tools/archive/manual-smokes/ny_stage3_bridge_accept_smoke.sh`
- `tools/engineering/run_vm_stats.sh`
- `tools/engineering/parity.sh`
- `tools/hako_check.sh`
- `tools/hako_check/deadcode_smoke.sh`
- `tools/archive/manual-smokes/async_smokes.sh`
- `tools/hakorune_emit_mir.sh`

Plugin/macro/tooling findings (`30xC2`):

- `src/macro/macro_box_ny.rs`
  - macro child route still documents `nyash --backend vm ...`
  - keep as engineering/macro tooling pressure; not a product-mainline surface
- `tools/bootstrap_selfhost_smoke.sh`
  - explicit bootstrap smoke on Rust VM lane
  - keep as engineering/bootstrap-tooling surface
- `tools/plugin_v2_smoke.sh`
  - explicit plugin V2 smoke still exercises `--backend vm`
  - keep as engineering/plugin tooling surface
- `tools/engineering/run_vm_stats.sh`
  - explicit VM stats helper and cookbook surface
  - keep as engineering/tooling surface
- `tools/hako_check.sh`
  - current helper/lint route still exercises Rust VM
  - keep as engineering/tooling surface
- `tools/hako_check/deadcode_smoke.sh`
  - deadcode helper smoke is still part of current tooling/docs flow
  - keep as engineering/tooling surface
- `tools/hakorune_emit_mir.sh`
  - current MIR emission helper remains live and docs-referenced
  - keep as engineering/tooling surface
- `tools/archive/manual-smokes/ny_stage1_asi_smoke.sh`
  - archived in `30xG1` from the old root-level manual smoke slot
  - no current live owner remained outside phase docs
- `tools/archive/manual-smokes/ny_stage3_bridge_accept_smoke.sh`
  - archived in `30xG1` from the old root-level manual smoke slot
  - no current live owner remained outside phase docs
- `tools/archive/manual-smokes/async_smokes.sh`
  - archived in `30xG1`; old cranelift/JIT manual helper is no longer a current entrypoint
  - `tools/smokes/jit-migration-plan.md` now reads it as historical, not live keep
- `tools/engineering/parity.sh`
  - parity helper still has current doc references and mixed backend vocabulary
  - keep for now as engineering/tooling pressure; revisit only after `30xD`

Plugin/macro/tooling archive/delete result (`30xC2`):

- none
- live engineering/tooling keeps remain in place
- archived in `30xG1`:
  - `tools/archive/manual-smokes/ny_stage1_asi_smoke.sh`
  - `tools/archive/manual-smokes/ny_stage3_bridge_accept_smoke.sh`
  - `tools/archive/manual-smokes/async_smokes.sh`
- root-level manual residue is reduced; remaining archive/delete work moves to `30xG4`

### Smoke / test

- `tools/selfhost/selfhost_smoke.sh`
- `tools/archive/manual-smokes/cross_backend_smoke.sh`
- `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
- `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
- `tools/selfhost/selfhost_vm_smoke.sh`
- `tools/archive/manual-smokes/selfhost_stage2_smoke.sh`
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- `tools/archive/manual-smokes/smoke_aot_vs_vm.sh`

Smoke/test findings (`30xC3`):

- `tools/selfhost/selfhost_smoke.sh`
  - current selfhosting quickstart still points at this dev smoke
  - keep as engineering/selfhost smoke surface
- `tools/selfhost/selfhost_vm_smoke.sh`
  - current root README/README.ja and `Makefile` still point at this script
  - keep as engineering smoke surface
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`
  - current Stage-3 acceptance guide still points at this bridge/selfhost smoke
  - keep as engineering smoke surface
- `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
  - mixed `vm`/`llvm` aggregator stays a do-not-flip-early orchestrator
  - keep as mixed orchestrator surface; do not archive in `30xC3`
- `tools/archive/manual-smokes/smoke_aot_vs_vm.sh`
  - archived in `30xG3`; old parity helper no longer belongs in root front docs
- `tools/archive/manual-smokes/cross_backend_smoke.sh`
  - archived in `30xG1`; only stale migration-plan material referenced it
- `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
  - explicit test-local manual parity harness with a local README
  - keep as engineering/test-local residue, not archive/delete target in `30xG1`
- `tools/archive/manual-smokes/selfhost_stage2_smoke.sh`
  - archived in `30xG1`; isolated manual selfhost acceptance smoke with no current live owner

Smoke/test archive/delete result (`30xC3`):

- live engineering smoke keeps remain in place
- archived in `30xG1`:
  - `tools/archive/manual-smokes/cross_backend_smoke.sh`
  - `tools/archive/manual-smokes/selfhost_stage2_smoke.sh`
- archived in `30xG3`:
  - `tools/archive/manual-smokes/smoke_aot_vs_vm.sh`
- explicit keep:
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`

### Docs / help / taxonomy

- `README.md`
- `README.ja.md`
- `docs/tools/cli-options.md`
- `docs/tools/nyash-help.md`
- `docs/development/runtime/cli-hakorune-stage1.md`
- `docs/guides/testing-guide.md`
- `docs/development/selfhosting/quickstart.md`
- `docs/guides/selfhost-pilot.md`

Docs/help findings (`30xC4`):

- rewrite in `30xE`:
  - `README.md`
    - still presents `--backend vm` and `selfhost_vm_smoke.sh` as the user-facing default narrative
    - product/main wording should move to `llvm/exe` first
  - `README.ja.md`
    - same main-narrative pressure as the English root README
  - `docs/development/selfhosting/quickstart.md`
    - still says `Use Rust VM by default` and keeps `rust-vm` as the quickstart主語
    - user-facing selfhost quickstart should move to role-first wording in `30xE`
  - `docs/guides/selfhost-pilot.md`
    - still presents pilot runner examples around `--backend vm`
    - user-facing pilot guide should move to role-first wording in `30xE`
- engineering docs keep:
  - `docs/tools/cli-options.md`
    - CLI reference sheet is an engineering/operator surface, not a product main narrative
    - keep and refresh wording later without treating it as main-switch front matter
  - `docs/development/runtime/cli-hakorune-stage1.md`
    - stage1/bootstrap design doc; `backend=vm` here is engineering/stage0 semantics, not product narrative
  - `docs/guides/testing-guide.md`
    - testing/diagnostics guide; `--backend vm` examples belong to engineering lane
- stale help snapshot result:
  - `docs/tools/nyash-help.md`
    - thin current stub in `docs/tools/`
    - historical capture moved to `docs/archive/tools/nyash-help.md`

Docs/help archive/delete result (`30xC4`):

- root README/help rewrites belong to `30xE`, not this inventory slice
- stage1/testing/selfhost guides stay engineering keeps
- stale help snapshot was archived in `30xG2`

Plugin/smoke orchestrator freeze findings (`30xD3`):

- explicit no-touch-first keep:
  - `tools/bootstrap_selfhost_smoke.sh`
  - `tools/plugin_v2_smoke.sh`
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/selfhost_vm_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
  - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
- archived/manual residue set:
  - `tools/archive/manual-smokes/ny_stage1_asi_smoke.sh`
  - `tools/archive/manual-smokes/ny_stage3_bridge_accept_smoke.sh`
  - `tools/archive/manual-smokes/async_smokes.sh`
  - `tools/archive/manual-smokes/cross_backend_smoke.sh`
  - `tools/archive/manual-smokes/selfhost_stage2_smoke.sh`
  - `tools/archive/manual-smokes/smoke_aot_vs_vm.sh`
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`

Plugin/smoke orchestrator freeze result (`30xD3`):

- landed as docs-first only
- no-touch-first orchestrators stay live engineering keeps
- `30xG1` archived low-blast root-level manual smoke residues under `tools/archive/manual-smokes/`
- `30xG3` archived `tools/archive/manual-smokes/smoke_aot_vs_vm.sh`
- `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh` remains an explicit engineering/test-local keep
- remaining helper/archive work moves to `30xG4`

## Dangerous Early Flips

Do not change these before `30xD` freeze plus `30xF` default-gate decisions land.

- `src/cli/args.rs`
- `src/runner/dispatch.rs`
- `src/runner/modes/common_util/selfhost/child.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/bootstrap_selfhost_smoke.sh`
- `tools/plugin_v2_smoke.sh`
- `tools/selfhost/selfhost_smoke.sh`
- `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`

Default/dispatch findings (`30xD1`):

- `src/cli/args.rs`
  - raw CLI `--backend` still defaults to `vm`
  - option help still enumerates `vm`, `vm-hako`, `llvm`, and `interpreter`
  - this token/default surface stays frozen until `30xF`; changing it earlier would blur inventory vs policy
- `src/runner/dispatch.rs`
  - central file dispatch still routes `vm`, `vm-hako`, and `llvm`, with `compile-wasm`/AOT gates adjacent
  - this dispatch surface stays frozen until `30xF`; changing it earlier would mix role taxonomy work with runtime ownership changes

Default/dispatch freeze result (`30xD1`):

- no code changes
- `src/cli/args.rs` and `src/runner/dispatch.rs` are explicit do-not-flip-early surfaces
- raw token/default decision remains blocked on `30xE` plus `30xF`

Bootstrap/selfhost freeze findings (`30xD2`):

- `src/runner/modes/common_util/selfhost/child.rs`
  - child capture hard-codes `nyash --backend vm <program>`
  - this remains bootstrap/runtime glue, not a first-slice backend switch target
- `lang/src/runner/stage1_cli/core.hako`
  - raw compat stage1 route still accepts `vm|pyvm` and explicitly rejects `llvm` on the legacy raw lane
  - this remains a bootstrap/stage1 contract surface, not a user-facing default target
- `tools/selfhost/run.sh`
  - unified selfhost entrypoint still shells out through `--backend vm` in runtime/direct paths
  - this remains a no-touch-first wrapper until `30xE-30xF` clarify the main switch and raw default gate
- `tools/selfhost/selfhost_build.sh`
  - selfhost build wrapper still invokes `--backend vm` for BuildBox/Stage-B steps
  - this remains a bootstrap wrapper surface, not an archive/delete target

Bootstrap/selfhost freeze result (`30xD2`):

- no code changes
- selfhost/stage1 wrappers stay explicit do-not-flip-early surfaces
- bootstrap wrapper/default changes remain blocked on `30xE` plus `30xF`

## Worker Re-Inventory Notes

- keep the docs label `rust-vm`; do not introduce `vm-rust` as the primary docs label in this phase
- `vm-hako` already has an explicit reference/conformance smoke home:
  - `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/README.md`
- `wasm` already has an explicit experimental smoke/tooling home:
  - `tools/smokes/v2/profiles/integration/phase29cc_wsm/README.md`
  - `tools/smokes/v2/lib/wasm_g3_contract.sh`
- current docs/help still over-read `--backend vm` in:
  - `README.md`
  - `README.ja.md`
  - `docs/tools/cli-options.md`
  - `docs/tools/nyash-help.md`
  - `docs/development/runtime/cli-hakorune-stage1.md`
  - `docs/guides/testing-guide.md`

## Exact Read Order

1. `docs/development/current/main/phases/phase-30x/README.md`
2. `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
3. `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
4. `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
5. `docs/development/current/main/design/artifact-policy-ssot.md`
6. `docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md`
