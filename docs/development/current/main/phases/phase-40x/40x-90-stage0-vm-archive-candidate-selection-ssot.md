---
Status: Landed
Date: 2026-04-03
Scope: stage0/bootstrap lane の remaining vm-rust / vm-gated surface を archive candidate / keep / rehome / delete-ready に分類する。
---

# 40x-90 Stage0 VM Archive Candidate Selection SSOT

## Plain Reading

- problem: every live vm-gated bootstrap route is still a feature obligation on `rust-vm`
- target: move stage0/bootstrap mainline ownership toward `hakorune` binary direct/core routes
- result: vm routes become `proof-only keep`, `compat keep`, or `archive-later`, instead of staying broad execution owners
- `40xB1` landed: the small proof-only VM gate set is frozen as `do-not-grow`.

## Success / Failure Rails

### Success

- keep only a small proof-only VM gate set
- split or starve `selfhost_build.sh` / `build.rs` as the decisive mixed owners
- stop new features from landing on `--backend vm`, stage1 compat, or raw routes
- shrink `vm.rs` only after caller drain proves it is no longer a broad owner

### Failure

- selfhost/bootstrap mainline still runs through `--backend vm`
- stage1 compat or raw routes absorb new capability work
- proof-only VM gates drift back into day-to-day mainline

## Macro Reading

| Wave | Status | Read as |
| --- | --- | --- |
| `40xA archive candidate inventory` | landed | new feature work がまだ `rust-vm` を引きずる route を exact に inventory する |
| `40xB keep/archive classification` | landed | route を `proof-only keep` / `compat keep` / `archive-later` / `direct-owner target` / `must-split-first` に分ける |
| `40xC archive/delete sweep` | landed | drained vm-facing shims と stale compat wrappers を live surface から外した |
| `40xD closeout` | landed | `rust-vm` を mainline owner ではなく proof/compat keep として handoff した |

## Candidate Reading

| Path | State | Reading |
| --- | --- | --- |
| `tools/selfhost/selfhost_build.sh` | mixed | Stage-B producer / direct MIR / EXE artifact / dispatcher が同居する bootstrap owner surface |
| `tools/selfhost/run_stageb_compiler_vm.sh` | vm gate | explicit Stage-B VM gate; archive candidate selection では keep boundary の確認対象 |
| `tools/selfhost/run.sh` | outer facade | `stage-a|exe` facade だが runtime route はまだ vm-dependent |
| `tools/bootstrap_selfhost_smoke.sh` | deleted | top-level shim was drained and removed; callers now land on `tools/selfhost/bootstrap_selfhost_smoke.sh` |
| `tools/plugin_v2_smoke.sh` | deleted | top-level shim was drained and removed; callers now land on `tools/plugins/plugin_v2_smoke.sh` |
| `tools/selfhost/selfhost_vm_smoke.sh` | explicit keep | VM path parity proof gate |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | explicit keep | stage3 acceptance proof gate |
| `src/runner/modes/common_util/selfhost/child.rs` | thin helper | shell capture helper; caller-sensitive |
| `src/runner/modes/vm.rs` | engineering keep | stage0/oracle keep until archive classification proves otherwise |
| `lang/src/runner/stage1_cli/core.hako` | compat keep | raw compat no-widen lane |
| `src/runner/core_executor.rs` | direct owner | already-materialized MIR(JSON) execution owner |
| `tools/selfhost/stage1_mainline_smoke.sh` | direct proof | current mainline proof smoke |
| `tools/stage1_smoke.sh` | archived | legacy embedded bridge smoke archived in phase-38x |
| `src/runner/build.rs` | mixed | source-side build owner; product/engineering split remains decisive for vm thinning |

## State Reading

| State | Read as |
| --- | --- |
| `mixed` | first migration target; still too many owners in one surface |
| `vm gate` | explicit keep candidate; do not grow it |
| `compat keep` | legacy/raw contract keep; do not attach new capabilities |
| `archive-later shim` | not a real owner; drain callers first and then archive |
| `explicit keep` | proof/acceptance route that stays live for now |
| `thin helper` | low-level helper; caller drain matters more than direct deletion |
| `direct owner` | where new capability work should converge |

## Inventory Targets (40xA1)

| Surface | Inventory focus |
| --- | --- |
| `tools/selfhost/selfhost_build.sh` | mixed owner seams: Stage-B producer / direct MIR / EXE artifact / dispatcher |
| `tools/selfhost/run_stageb_compiler_vm.sh` | explicit VM gate lines and current caller families |
| `tools/selfhost/run.sh` | outer facade modes and remaining vm-dependent edges |
| `tools/selfhost/bootstrap_selfhost_smoke.sh` | canonical bootstrap proof home and caller drain target |
| `tools/plugins/plugin_v2_smoke.sh` | canonical plugin proof home and caller drain target |
| `tools/selfhost/selfhost_vm_smoke.sh` | explicit VM proof boundary |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | stage3 acceptance boundary |
| `src/runner/modes/common_util/selfhost/child.rs` | thin helper boundary vs caller drain |
| `src/runner/modes/vm.rs` | engineering keep justification |
| `lang/src/runner/stage1_cli/core.hako` | compat keep boundary |

## Caller Inventory Snapshot (40xA1 landed)

| Surface | Caller inventory | Reading |
| --- | --- | --- |
| `tools/bootstrap_selfhost_smoke.sh` | `Makefile`, `docs/guides/selfhost-pilot.md`, `dev/selfhosting/README.md`, phase-30x/31x/38x/40x docs | deleted top-level shim; callers now point at `tools/selfhost/bootstrap_selfhost_smoke.sh` |
| `tools/plugin_v2_smoke.sh` | `src/runner/modes/common_util/plugin_guard.rs`, `tools/selfhost/README.md`, phase-29cc/30x/31x/38x/40x docs | deleted top-level shim; callers now point at `tools/plugins/plugin_v2_smoke.sh` |
| `tools/selfhost/selfhost_build.sh` | `tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh`, `selfhost_build_{binop,return}_vm.sh`, `selfhost_build_exe_return.sh`, `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`, `tools/selfhost/README.md`, `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` | mixed owner; producer/direct-run/exe-artifact/dispatcher all live |
| `tools/selfhost/run_stageb_compiler_vm.sh` | `tools/selfhost/run.sh`, phase29bq / 29cc stageb smokes, parser trace, `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` | explicit VM gate; keep frozen |
| `tools/selfhost/run.sh` | selfhost README, script index, README, README.ja, quickstart, stage2 bridge smoke, stage3 accept smoke, phase29bq/29y/parser smokes | outer facade; vm-dependent edges remain |
| `tools/selfhost/selfhost_vm_smoke.sh` | `Makefile`, `README.md`, `README.ja.md`, `tools/selfhost/README.md`, phase-31x/37x/39x/40x docs | explicit VM proof boundary; keep |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | `docs/guides/selfhost-pilot.md`, `docs/guides/exceptions-stage3.md`, `tools/selfhost/README.md`, `tools/selfhost/stage3_same_result_check.sh`, current phase docs | stage3 acceptance boundary; keep |
| `src/runner/modes/common_util/selfhost/child.rs` | `src/runner/selfhost.rs`, `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | thin helper; caller-sensitive |
| `src/runner/modes/vm.rs` | current backend docs / vm-lane docs | engineering keep |
| `lang/src/runner/stage1_cli/core.hako` | stage1 compat docs / raw compat callers | compat keep |
| `src/runner/core_executor.rs` | `src/runner/mod.rs` direct handoff | direct owner |
| `src/runner/build.rs` | `src/runner/mod.rs`, `build_product.rs`, `build_engineering.rs` | mixed build owner; decisive split input |

## Drain Map (40xB2)

| Caller family | Drain target | Read as |
| --- | --- | --- |
| `tools/selfhost/bootstrap_selfhost_smoke.sh` callers | `tools/selfhost/stage1_mainline_smoke.sh` / `tools/selfhost/run_stage1_cli.sh` | mainline proof points at the direct/core route; the top-level shim is deleted |
| `tools/plugins/plugin_v2_smoke.sh` callers | `tools/plugins/plugin_v2_smoke.sh` | plugin proof points at the canonical plugin home |
| `tools/selfhost/run.sh` callers | `tools/selfhost/run_stage1_cli.sh` / `tools/selfhost/stage1_mainline_smoke.sh` | outer facade can stay, but direct mainline should land inward |
| `tools/selfhost/selfhost_build.sh` callers | `tools/selfhost/selfhost_build.sh` split outputs (`build_product.rs` / `build_engineering.rs`) | mixed owner is not a shim, but callers should stop assuming one vm-shaped route |
| `src/runner/modes/common_util/selfhost/child.rs` callers | `src/runner/core_executor.rs` / `stage_a_route.rs` / `stage_a_compat_bridge.rs` | caller-sensitive helper should drain toward direct/core owners, not gain new vm growth |

## Classification Snapshot (40xA2 landed)

| Surface | Bucket | Rule |
| --- | --- | --- |
| `tools/selfhost/selfhost_build.sh` | `must-split-first` | biggest mixed owner in the bootstrap lane; do not add more vm-only branches here |
| `src/runner/build.rs` | `must-split-first` | source-side product/engineering build owner still decides whether feature work leaks back into vm |
| `tools/selfhost/run_stageb_compiler_vm.sh` | `proof-only keep` | explicit Stage-B VM gate; freeze and do not grow |
| `tools/selfhost/selfhost_vm_smoke.sh` | `proof-only keep` | VM parity proof only |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | `proof-only keep` | stage3 acceptance proof only until direct coverage replaces it |
| `lang/src/runner/stage1_cli/core.hako` | `compat keep` | raw compat lane; no new capability work |
| `tools/bootstrap_selfhost_smoke.sh` | `deleted` | top-level shim drained and removed |
| `tools/plugin_v2_smoke.sh` | `deleted` | top-level shim drained and removed |
| `tools/selfhost/run.sh` | `direct-owner target behind facade` | facade may stay, but live owner must move inward to direct/core routes |
| `src/runner/modes/common_util/selfhost/child.rs` | `direct-owner target behind caller drain` | thin helper; caller drain comes before shrink |
| `src/runner/core_executor.rs` | `direct owner` | converge new capability work here |
| `tools/selfhost/run_stage1_cli.sh` / `tools/selfhost/stage1_mainline_smoke.sh` | `direct owner/proof` | current mainline direct route |
| `src/runner/modes/vm.rs` | `engineering keep; shrink later` | broad owner today, but no longer a place for new features |

## Current Front

- active macro wave: `41xB route hardening`
- active micro task: `41xB2 run.sh facade trim`
- next queued micro task: `41xC1 vm.rs proof/oracle shrink`
