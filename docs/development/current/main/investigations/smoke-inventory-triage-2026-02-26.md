# Smoke Inventory Triage (2026-02-26)

## Scope

- Target: `tools/smokes/v2/profiles/integration/apps`
- Source report: `tools/checks/smoke_inventory_report.sh`

## Current Snapshot

- total: `344` (`Include archive: 0` default)
- total: `254` (`Include archive: 0` default)
- referenced: `155`
- orphan candidate: `99`
- orphan wrapper candidate: `0`
- with archive included: total `365`, orphan `210`
- suffix breakdown:
  - `vm`: `223`
  - `llvm_exe`: `9`
  - `other`: `22`

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
8. Batch-D (archive isolation + doc-path sync) done:
   - moved 18 additional orphan scripts to archive
   - families: `phase136`, `phase137`, `phase146`, `phase285`
   - updated docs command paths from `apps/...` to `apps/archive/...` for moved scripts
   - verification: `phase29y_lane_gate_quick_vm.sh` PASS
9. Batch-E (archive isolation + doc-path sync) done:
   - moved 16 additional orphan scripts to archive
   - families: `phase102`, `phase113`, `phase114`, `phase115`, `phase116`, `phase117`, `phase121`, `phase128`
   - updated docs command paths from `apps/...` to `apps/archive/...` for moved scripts
   - verification: `phase29y_lane_gate_quick_vm.sh` PASS
10. Batch-F (archive isolation + doc-path sync) done:
   - moved 14 additional orphan scripts to archive
   - families: `phase129`, `phase130`, `phase131`, `phase134`, `phase139`, `phase141`, `phase142`
   - updated docs command paths from `apps/...` to `apps/archive/...` for moved scripts
   - verification: `phase29y_lane_gate_quick_vm.sh` PASS
11. Batch-G (archive isolation + doc-path sync) done:
   - moved 24 additional orphan scripts to archive
   - families: `phase145`, `phase254`, `phase256`, `phase257`, `phase258`, `phase259`, `phase263`, `phase269`, `phase274`, `phase275`, `phase283`, `phase284`
   - updated docs command paths from `apps/...` to `apps/archive/...` for moved scripts
   - verification: `phase29y_lane_gate_quick_vm.sh` PASS

## Findings

- `tools/checks` hardcoded references to `integration/apps` were scanned; no missing file references found.
- current blockers were caused by file mode drift (`100644` for executable scripts).
- many orphan candidates are standalone fixtures (`phase100/103/104/107/118...`) and need policy decision before removal.

## Next Batches (ordered)

1. Batch-H (archive or retire):
   - remaining orphan candidates are mainly `phase29` lineage + utility singletons (`controlflow_probe_vm`, `gate_log_summarizer_vm`, `phase87/92/94/95/96/97/99`)
   - archive non-gate singletons first, then decide `phase29` group by gate contract
2. Batch-I (gate-pack consolidation):
   - optionally re-promote selected archive groups into explicit gate packs when needed
   - keep fixture coverage while limiting top-level app-script fan-out
   - scripts still orphan after Batch-A/B
   - move to archive or remove after docs pointer update

## Guardrails

- 1 batch = 1 commit + gate verification.
- do not remove scripts that appear in `tools/checks/*guard*.sh` or `phase29y/phase29x` entry chains.
- if script is orphan by report but used manually, move under archived/experimental namespace instead of silent deletion.
