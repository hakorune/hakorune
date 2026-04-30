---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: narrow the Stage1 explicit Program(JSON) compat shell surface so only `stage1_contract_exec_program_json_compat()` remains as the live helper entry.
Related:
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - tools/selfhost/lib/stage1_contract.sh
  - tools/dev/phase29ch_program_json_explicit_mode_gate_probe.sh
---

# P25 Stage1 Contract Compat Helper Narrow

## Goal

Keep the explicit Program(JSON) compat helper surface singular.

After P24, `tools/selfhost/lib/stage1_contract.sh` still had a probe-oriented
generic text helper alongside the actual live compat helper. This slice moves
the probe to `stage1_contract_exec_mode()` / `stage1_contract_exec_program_json_compat()`
and removes the extra helper entry.

## Decision

- keep `stage1_contract_exec_program_json_compat()` as the only live explicit
  Program(JSON) compat helper
- keep the compat mode / sentinel entry constants in `stage1_contract.sh`
- remove `stage1_contract_exec_program_json_text()`
- keep the explicit-mode probe green by calling `stage1_contract_exec_mode()`
  directly for the rejected legacy alias case

## Non-goals

- do not touch `tools/lib/program_json_v0_compat.sh`
- do not change `run_stage1_cli.sh` behavior
- do not change Stage-B artifact probes or `phase29bq` fixture producers
- do not touch Rust-side delete-last compat surfaces

## Acceptance

```bash
bash -n tools/selfhost/lib/stage1_contract.sh \
  tools/dev/phase29ch_program_json_explicit_mode_gate_probe.sh
if [ -x target/selfhost/hakorune.stage1_cli ] && [ -x target/selfhost/hakorune.stage1_cli.stage2 ]; then
  bash tools/dev/phase29ch_program_json_explicit_mode_gate_probe.sh
fi
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
