---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify Stage1 contract callers before any Program(JSON v0) delete-last work.
Related:
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/selfhost/lib/stage1_contract.sh
  - tools/selfhost/compat/run_stage1_cli.sh
  - tools/selfhost/run_stage1_cli.sh
  - tools/selfhost/lib/identity_routes.sh
  - tools/selfhost/mainline/build_stage1.sh
---

# P64 Stage1 Contract Caller Ownership Split

## Goal

Keep Stage1 contract cleanup from accidentally treating a live shell contract
as a removable Program(JSON v0) residue.

## Decision

- `tools/selfhost/lib/stage1_contract.sh` is the Stage1 shell contract SSOT.
  It owns env injection, emit output validation, direct emit helpers, and
  bootstrap capability verification.
- `tools/selfhost/compat/run_stage1_cli.sh` is a compatibility wrapper around
  that contract. It is not a second authority route.
- `tools/selfhost/run_stage1_cli.sh` is only the top-level shim to the compat
  wrapper. It is owned by the wrapper and should not be counted as a separate
  Program(JSON v0) keeper.
- `tools/lib/program_json_v0_compat.sh` cannot be deleted while
  `stage1_contract_exec_direct_emit_mode ... emit-program ...` remains live.

## Active Caller Classes

| Caller class | Examples | Reading |
| --- | --- | --- |
| build/bootstrap | `tools/selfhost/mainline/build_stage1.sh` | uses Stage1 contract helpers for capability probes and direct emit checks |
| identity/proof | `tools/selfhost/lib/identity_routes.sh`, `tools/selfhost_identity_check.sh` | keeps exact route validation centralized |
| compatibility wrapper | `tools/selfhost/compat/run_stage1_cli.sh`, `tools/selfhost/run_stage1_cli.sh` | wrapper/shim around the contract, not a new authority |
| dev/probe | phase29ch/phase29cg probes | diagnostics only; callers still keep the contract live |
| smoke | `phase29bq_selfhost_stage1_contract_smoke_vm.sh` | contract pin, requires a prebuilt stage1-cli artifact |

## Non-goals

- do not archive `run_stage1_cli.sh`
- do not delete `program_json_v0_compat.sh`
- do not re-enable retired wrapper routes such as `emit program-json` or
  `emit mir-json --from-program-json`

## Acceptance

```bash
bash -n tools/selfhost/lib/stage1_contract.sh tools/selfhost/compat/run_stage1_cli.sh tools/selfhost/run_stage1_cli.sh tools/selfhost/lib/identity_routes.sh tools/selfhost/mainline/build_stage1.sh
bash tools/selfhost/compat/run_stage1_cli.sh --help
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
