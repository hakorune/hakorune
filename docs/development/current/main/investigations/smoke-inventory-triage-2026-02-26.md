# Smoke Inventory Triage (2026-02-26)

## Scope

- Target: `tools/smokes/v2/profiles/integration/apps`
- Source report: `tools/checks/smoke_inventory_report.sh`

## Current Snapshot

- total: `365`
- referenced: `155`
- orphan candidate: `210`
- orphan wrapper candidate: `0`
- suffix breakdown:
  - `vm`: `297`
  - `llvm_exe`: `50`
  - `other`: `35`

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

## Findings

- `tools/checks` hardcoded references to `integration/apps` were scanned; no missing file references found.
- current blockers were caused by file mode drift (`100644` for executable scripts).
- many orphan candidates are standalone fixtures (`phase100/103/104/107/118...`) and need policy decision before removal.

## Next Batches (ordered)

1. Batch-B (gate-pack consolidation):
   - group standalone `phase100/103/104/107/118` into one entry gate (vm + llvm_exe pair packs)
   - keep fixture coverage, reduce top-level script count
2. Batch-C (archive or retire):
   - scripts still orphan after Batch-A/B
   - move to archive or remove after docs pointer update

## Guardrails

- 1 batch = 1 commit + gate verification.
- do not remove scripts that appear in `tools/checks/*guard*.sh` or `phase29y/phase29x` entry chains.
- if script is orphan by report but used manually, move under archived/experimental namespace instead of silent deletion.
