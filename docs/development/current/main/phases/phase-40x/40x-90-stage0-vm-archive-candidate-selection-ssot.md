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

## Current Front

- active macro wave: `40xA archive candidate inventory`
- active micro task: `40xA1 archive candidate inventory`
- next queued micro task: `40xA2 keep/archive classification`
