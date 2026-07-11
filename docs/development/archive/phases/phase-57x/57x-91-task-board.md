---
Status: Landed
Date: 2026-04-04
---

# 57x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `57xA inventory/classify` | landed | lock the exact residual rust-vm surfaces before any removal |
| 2 | `57xB audit/prep` | landed | prove caller-zero or replacement coverage for removal candidates |
| 3 | `57xC removal wave` | landed | remove only the delete-ready residue |
| 4 | `57xD closeout` | landed | prove and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `57xA1` | landed | residual rust-vm delete-ready inventory lock |
| `57xA2` | landed | keep/delete/archive classification freeze |
| `57xB1` | landed | caller-zero audit |
| `57xB2` | landed | removal candidate prep |
| `57xC1` | landed | removal wave |
| `57xD1` | landed | proof / closeout |

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
| Now | `phase-57x landed` |
| Blocker | `none` |
| Next | `58xA1 successor lane inventory lock` |
| After Next | `58xA2 candidate lane ranking` |
