---
Status: Landed
Decision: provisional
Date: 2026-04-03
Scope: `phase-34x stage0 shell residue split` の concrete queue と evidence command をまとめる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-34x/README.md
  - docs/development/current/main/phases/phase-34x/34x-90-stage0-shell-residue-split-ssot.md
---

# 34x-91 Task Board

## Current Queue

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `34xA residue owner lock` | landed | exact shell residue/owner split first |
| 2 | `34xB child runner thinning` | landed | process/shell helper gets thinner before new runtime work |
| 3 | `34xC stage1 raw compat narrowing` | landed | raw compat branch stays narrow |
| 4 | `34xD direct core handoff` | landed | direct `MIR(JSON)` owner is `core_executor` |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `34xA1` | landed | `child.rs` exact residue lock |
| 2 | `34xA2` | landed | `stage1_cli/core.hako` exact residue lock |
| 3 | `34xA3` | landed | `core_executor` takeover seam lock |
| 4 | `34xB1` | landed | split spawn/timeout/capture from `child.rs` |
| 5 | `34xC1` | landed | `run_program_json` no-widen lock |
| 6 | `34xD1` | landed | direct `MIR(JSON)` proof path |

## Evidence Commands

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
git diff --check
rg -n 'run_ny_program_capture_json_v0|run_program_json|_run_raw_request|execute_mir_json_text|execute_loaded_mir_module' \
  src/runner/modes/common_util/selfhost/child.rs \
  lang/src/runner/stage1_cli/core.hako \
  src/runner/core_executor.rs \
  docs/development/current/main/phases/phase-32x \
  docs/development/current/main/phases/phase-34x
cargo check --bin hakorune
cargo test --manifest-path Cargo.toml execute_mir_json_text_ -- --nocapture
```

## Current Result

- current front:
  - `landed precursor`
- worker-confirmed residue concentration:
  - `child.rs::run_ny_program_capture_json_v0` owns spawn / timeout / stdout-stderr capture / first-line JSON extraction
  - `selfhost.rs` is the shared v0 caller; `stage_a_compat_bridge.rs` is the MIR-only selector caller
  - `stage1_cli/core.hako::run_program_json` and `_run_raw_request` own the raw compat residue and must stay narrow
  - `run_program_json` / `_mode_run` / `_run_raw_request` are now explicitly no-widen: no thread/runtime capability work lands there
  - `dispatch_env_mode` / `dispatch_emit` / `dispatch_run` are the thin dispatch-side callers; `stage1_main` stays dispatcher-only
  - `core_executor::execute_mir_json_text` and `execute_loaded_mir_module` are the direct MIR(JSON) owner seam; `execute_json_artifact` remains the family classifier
  - `child.rs` now splits private helper ownership into command setup, capture wiring, timeout/wait, output readback, and JSON-line selection while public selectors stay unchanged
  - proof is pinned by `execute_mir_json_text_accepts_direct_mir_fixture` and `execute_mir_json_text_rejects_program_json_direct_input`
  - successor lane is `phase-35x stage-a compat route thinning`
