---
Status: Active
Date: 2026-04-05
Scope: rerun selfhost proof/compat callers after the top-level wrapper policy freeze.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-91x/README.md
  - lang/src/runner/README.md
---

# 92x-90 Selfhost Proof/Compat Caller Rerun SSOT

## Purpose

- rerun proof/compat callers against the canonical wrapper homes
- confirm the top-level `.hako` wrappers remain thin public/front-door keeps
- keep proof/compat reruns explicit and non-growing

## In Scope

- `tools/selfhost/mainline/stage1_mainline_smoke.sh`
- `tools/selfhost/proof/run_stageb_compiler_vm.sh`
- `tools/selfhost/proof/selfhost_smoke.sh`
- `tools/selfhost/proof/selfhost_vm_smoke.sh`
- `tools/selfhost/proof/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/proof/selfhost_stage3_accept_smoke.sh`
- `tools/selfhost/compat/run_stage1_cli.sh`
- `tools/selfhost/run.sh`
- `lang/src/runner/README.md`

## Caller Inventory

### Canonical Proof Callers

- `tools/selfhost/mainline/stage1_mainline_smoke.sh`
- `tools/selfhost/proof/run_stageb_compiler_vm.sh`
- `tools/selfhost/proof/selfhost_smoke.sh`
- `tools/selfhost/proof/selfhost_vm_smoke.sh`
- `tools/selfhost/proof/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/proof/selfhost_stage3_accept_smoke.sh`

### Canonical Compat Callers

- `tools/selfhost/compat/run_stage1_cli.sh`
- `tools/selfhost/run.sh`

### Canonical Wrapper Homes

- `lang/src/runner/compat/stage1_cli.hako`
- `lang/src/runner/facade/runner_facade.hako`
- `lang/src/runner/entry/launcher_native_entry.hako`
- `lang/src/runner/entry/stage1_cli_env_entry.hako`
- `lang/src/runner/launcher.hako`
- `lang/src/runner/stage1_cli.hako`
- `lang/src/runner/stage1_cli_env.hako`

## Ranked Rerun Set

1. `tools/selfhost/mainline/stage1_mainline_smoke.sh`
   - minimal proof that the canonical Stage1 mainline wrapper still emits MIR(JSON) through the thin compat path
2. `tools/selfhost/compat/run_stage1_cli.sh`
   - explicit compat owner rerun for the canonical Stage1 CLI surface
3. `tools/selfhost/run.sh --runtime --runtime-mode exe --input apps/examples/string_p0.hako`
   - top-level front-door rerun for the current mainline runtime route
4. `tools/selfhost/proof/run_stageb_compiler_vm.sh`
   - explicit proof-only rerun for the VM Stage-B gate

## Deferred Broader Callers

- `tools/selfhost/proof/selfhost_smoke.sh`
- `tools/selfhost/proof/selfhost_vm_smoke.sh`
- `tools/selfhost/proof/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/proof/selfhost_stage3_accept_smoke.sh`

The broader proof callers stay parked unless one of the ranked reruns exposes a regression that requires widening the scope.

## Proof Refresh

- PASS: `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- PASS: `bash tools/selfhost/compat/run_stage1_cli.sh --bin target/selfhost/hakorune.stage1_cli.stage2 emit mir-json apps/tests/hello_simple_llvm.hako`
- PASS: `bash tools/selfhost/run.sh --runtime --runtime-mode exe --input apps/examples/string_p0.hako --timeout-secs 20`
- PASS: `NYASH_SELFHOST_STAGEB_PROOF_ONLY=1 bash tools/selfhost/proof/run_stageb_compiler_vm.sh --source-file apps/tests/hello_simple_llvm.hako --timeout-secs 20`

Notes:
- the compat wrapper emits warnings about plugins being disabled, but exits 0 and produces MIR(JSON) with the `functions` marker
- the proof-only Stage-B gate remains explicit and non-growing

## Closeout Notes

- ranked caller rerun passed without widening scope
- broader proof callers stayed deferred because the canonical wrapper homes remained thin
- no blocker remains for handoff

## Policy

- proof/compat reruns should stay explicit and non-growing
- top-level wrappers remain thin public/front-door keeps
- canonical homes stay under `compat/`, `facade/`, and `entry/`

## Out of Scope

- rust-vm retirement
- vm-hako reference/conformance lane
- archive/deletion sweeps
- new wrapper ownership growth
