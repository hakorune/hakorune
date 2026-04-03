---
Status: Active
Date: 2026-04-03
Scope: stage0/bootstrap lane の remaining vm-rust / vm-gated surface を archive candidate / keep / rehome / delete-ready に分類する。
---

# 40x-90 Stage0 VM Archive Candidate Selection SSOT

## Macro Reading

| Wave | Status | Read as |
| --- | --- | --- |
| `40xA archive candidate inventory` | active | remaining vm-rust/bootstrap surfaces を exact に inventory する |
| `40xB keep/archive classification` | queued | live surface を explicit keep / archive-later / delete-ready に分ける |
| `40xC archive/delete sweep` | queued | drained shims と stale compat wrappers を live surface から外す |
| `40xD closeout` | queued | next source lane に handoff する |

## Candidate Reading

| Path | State | Reading |
| --- | --- | --- |
| `tools/selfhost/selfhost_build.sh` | mixed | Stage-B producer / direct MIR / EXE artifact / dispatcher が同居する bootstrap owner surface |
| `tools/selfhost/run_stageb_compiler_vm.sh` | vm gate | explicit Stage-B VM gate; archive candidate selection では keep boundary の確認対象 |
| `tools/selfhost/run.sh` | outer facade | `stage-a|exe` facade だが runtime route はまだ vm-dependent |
| `tools/bootstrap_selfhost_smoke.sh` | archive-later shim | top-level caller drain がまだ残る |
| `tools/plugin_v2_smoke.sh` | archive-later shim | top-level caller drain がまだ残る |
| `tools/selfhost/selfhost_vm_smoke.sh` | explicit keep | VM path parity proof gate |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | explicit keep | stage3 acceptance proof gate |
| `src/runner/modes/common_util/selfhost/child.rs` | thin helper | shell capture helper; caller-sensitive |
| `src/runner/modes/vm.rs` | engineering keep | stage0/oracle keep until archive classification proves otherwise |
| `lang/src/runner/stage1_cli/core.hako` | compat keep | raw compat no-widen lane |
| `src/runner/core_executor.rs` | direct owner | already-materialized MIR(JSON) execution owner |
| `tools/selfhost/stage1_mainline_smoke.sh` | direct proof | current mainline proof smoke |
| `tools/stage1_smoke.sh` | archived | legacy embedded bridge smoke archived in phase-38x |

## Inventory Targets (40xA1)

| Surface | Inventory focus |
| --- | --- |
| `tools/selfhost/selfhost_build.sh` | mixed owner seams: Stage-B producer / direct MIR / EXE artifact / dispatcher |
| `tools/selfhost/run_stageb_compiler_vm.sh` | explicit VM gate lines and current caller families |
| `tools/selfhost/run.sh` | outer facade modes and remaining vm-dependent edges |
| `tools/bootstrap_selfhost_smoke.sh` | top-level shim callers and archive-later readiness |
| `tools/plugin_v2_smoke.sh` | top-level shim callers and archive-later readiness |
| `tools/selfhost/selfhost_vm_smoke.sh` | explicit VM proof boundary |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | stage3 acceptance boundary |
| `src/runner/modes/common_util/selfhost/child.rs` | thin helper boundary vs caller drain |
| `src/runner/modes/vm.rs` | engineering keep justification |
| `lang/src/runner/stage1_cli/core.hako` | compat keep boundary |

## Caller Inventory Snapshot (40xA1 landed)

| Surface | Caller inventory | Reading |
| --- | --- | --- |
| `tools/bootstrap_selfhost_smoke.sh` | `Makefile`, `docs/guides/selfhost-pilot.md`, `dev/selfhosting/README.md`, phase-30x/31x/38x/40x docs | archive-later shim; top-level drain remains |
| `tools/plugin_v2_smoke.sh` | `src/runner/modes/common_util/plugin_guard.rs`, `tools/selfhost/README.md`, phase-29cc/30x/31x/38x/40x docs | archive-later shim; top-level drain remains |
| `tools/selfhost/selfhost_build.sh` | `tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh`, `selfhost_build_{binop,return}_vm.sh`, `selfhost_build_exe_return.sh`, `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`, `tools/selfhost/README.md`, `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` | mixed owner; producer/direct-run/exe-artifact/dispatcher all live |
| `tools/selfhost/run_stageb_compiler_vm.sh` | `tools/selfhost/run.sh`, phase29bq / 29cc stageb smokes, parser trace, `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md` | explicit VM gate; keep frozen |
| `tools/selfhost/run.sh` | selfhost README, script index, README, README.ja, quickstart, stage2 bridge smoke, stage3 accept smoke, phase29bq/29y/parser smokes | outer facade; vm-dependent edges remain |
| `tools/selfhost/selfhost_vm_smoke.sh` | `Makefile`, `README.md`, `README.ja.md`, `tools/selfhost/README.md`, phase-31x/37x/39x/40x docs | explicit VM proof boundary; keep |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | `docs/guides/selfhost-pilot.md`, `docs/guides/exceptions-stage3.md`, `tools/selfhost/README.md`, `tools/selfhost/stage3_same_result_check.sh`, current phase docs | stage3 acceptance boundary; keep |
| `src/runner/modes/common_util/selfhost/child.rs` | `src/runner/selfhost.rs`, `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | thin helper; caller-sensitive |
| `src/runner/modes/vm.rs` | current backend docs / vm-lane docs | engineering keep |
| `lang/src/runner/stage1_cli/core.hako` | stage1 compat docs / raw compat callers | compat keep |
| `src/runner/core_executor.rs` | `src/runner/mod.rs` direct handoff | direct owner |

## Current Front

- active macro wave: `40xA archive candidate inventory`
- active micro task: `40xA2 keep/archive classification`
- next queued micro task: `40xB1 top-level shim caller drain map`
