# Smoke Inventory Triage (2026-02-26)

## Scope

- Target: `tools/smokes/v2/profiles/integration/apps`
- Source report: `tools/checks/smoke_inventory_report.sh`

## Current Snapshot

- total: `344` (`Include archive: 0` default)
- total: `326` (`Include archive: 0` default)
- referenced: `155`
- orphan candidate: `171`
- orphan wrapper candidate: `0`
- with archive included: total `365`, orphan `210`
- suffix breakdown:
  - `vm`: `261`
  - `llvm_exe`: `35`
  - `other`: `30`

## This Round (completed)

1. Removed alias-only wrappers with zero inbound references:
   - `phase29y_hako_emit_mir_binary_only_block_vm.sh`
   - `phase29y_hako_run_binary_only_block_vm.sh`
   - `vm_hako_caps_const_void_block_vm.sh`
   - `vm_hako_caps_file_close_block_vm.sh`
   - `vm_hako_caps_file_read_block_vm.sh`
2. Fixed lane blocker root cause:
   - issue was executable-bit drift (not spec mismatch)
   - restored `+x` on phase29y gate chain scripts/checks/selfhost scripts
3. Verified:
   - `phase29y_lane_gate_quick_vm.sh` PASS
   - `phase29y_lane_gate_vm.sh` PASS
4. Batch-A done:
   - removed 17 orphan `phase29z_vm_hako_*_parity_vm.sh` wrappers
   - commit: `e5fc306a0`
5. X56 gate drift fix:
   - `phase29x_vm_hako_s6_vocab_guard.sh` now tracks `src/runner/modes/vm_hako/subset_check.rs`
   - S6 parity gate reject step aligned to `phase29z_vm_hako_s5_newclosure_probe_vm.sh`
   - commit: `b55825c74`
6. Batch-B (archive isolation) done:
   - moved orphan-heavy families to `tools/smokes/v2/profiles/integration/apps/archive/`
   - families: `phase100`, `phase103`, `phase104`, `phase107`, `phase118` (21 files)
   - inventory default now excludes archive (`SMOKE_INVENTORY_INCLUDE_ARCHIVE=0`)
7. Batch-C (archive isolation, selective) done:
   - moved 18 additional orphan scripts to archive after zero-inbound-ref check
   - families: `phase143` (P2 bc/cb), `phase145` (P2 compound expr), `phase252` (p0 break cond), `phase275` (eq/plus), `phase284` (p1 return), `phase286` (pattern1/2 frag)
   - kept `phase259_p0_is_integer_llvm_exe.sh` in active due doc reference
   - verification: `phase29y_lane_gate_quick_vm.sh` PASS

## Findings

- `tools/checks` hardcoded references to `integration/apps` were scanned; no missing file references found.
- current blockers were caused by file mode drift (`100644` for executable scripts).
- many orphan candidates are standalone fixtures (`phase100/103/104/107/118...`) and need policy decision before removal.

## Next Batches (ordered)

1. Batch-D (archive or retire):
   - continue zero-inbound-ref archive moves for remaining orphan families (`phase136/137/146/285/...`)
   - skip any script referenced from `docs/development/current/main/**` until docs pointers are cleaned
2. Batch-E (gate-pack consolidation):
   - optionally re-promote selected archive groups into explicit gate packs when needed
   - keep fixture coverage while limiting top-level app-script fan-out
   - scripts still orphan after Batch-A/B
   - move to archive or remove after docs pointer update

## Guardrails

- 1 batch = 1 commit + gate verification.
- do not remove scripts that appear in `tools/checks/*guard*.sh` or `phase29y/phase29x` entry chains.
- if script is orphan by report but used manually, move under archived/experimental namespace instead of silent deletion.
