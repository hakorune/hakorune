---
Status: Active
Date: 2026-04-04
---

# 57x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `57xA inventory/classify` | landed | lock the exact residual rust-vm surfaces before any removal |
| 2 | `57xB audit/prep` | active | prove caller-zero or replacement coverage for removal candidates |
| 3 | `57xC removal wave` | queued | remove only the delete-ready residue |
| 4 | `57xD closeout` | queued | prove and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `57xA1` | landed | residual rust-vm delete-ready inventory lock |
| `57xA2` | landed | keep/delete/archive classification freeze |
| `57xB1` | landed | caller-zero audit |
| `57xB2` | active | removal candidate prep |
| `57xC1` | queued | removal wave |
| `57xD1` | queued | proof / closeout |

## Inventory Snapshot

| Surface | Class | Read as |
| --- | --- | --- |
| `src/runner/modes/vm.rs` | `keep-now` | broad rust-vm proof/oracle owner still has explicit callers |
| `src/runner/modes/vm_fallback.rs` | `keep-now` | explicit compat fallback keep |
| `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | `keep-now` | explicit Program(JSON) compat bridge |
| `lang/src/runner/stage1_cli/core.hako` | `keep-now` | raw compat hold line |
| `tools/selfhost/run_stageb_compiler_vm.sh` | `keep-now` | proof-only Stage-B gate |
| residual archive/manual-smoke wrappers | `archive-later` | historical or proof/manual evidence, not delete-ready source |

## Current Front

| Item | State |
| --- | --- |
| Now | `57xB2 removal candidate prep` |
| Blocker | `none` |
| Next | `57xC1 removal wave` |
| After Next | `57xD1 proof / closeout` |
