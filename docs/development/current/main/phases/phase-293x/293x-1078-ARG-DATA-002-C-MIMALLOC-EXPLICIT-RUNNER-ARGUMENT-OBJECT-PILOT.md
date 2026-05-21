# 293x-1078 ARG-DATA-002 C Mimalloc Explicit Runner Argument Object Pilot

Status: landed
Date: 2026-05-21

## Purpose

Refactor the MIMAP-451A explicit C mimalloc runner evidence owner to reduce long
positional argument lists with owner-local context records.

## Scope

- Keep report box fields and stable output lines unchanged.
- Introduce narrow owner-local context records for runner evidence, memory
  evidence, and stop-line evidence.
- Use context records in owner-local `makeReport` / `reject` flows.
- Keep the public evidence entry positional for compatibility; record-local
  carriers still must not cross owner/API boundaries.
- Preserve the existing MIMAP-451A and MIMAP-452A guards.
- Keep MIMAP-454A queued after this BoxShape sidecar.

## Stop Lines

- No new source syntax.
- No `...fields` / spread syntax.
- No named argument syntax.
- No record default value semantics.
- No automatic record-to-box copy semantics.
- No runtime record object materialization.
- No backend route additions.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.

## Validation

Required:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completed

- Added owner-local runner, memory, and stop-line context records to the
  MIMAP-451A explicit C mimalloc runner evidence owner.
- Reduced the repeated `reject` / `makeReport` positional argument transport
  from the full runner payload to three context records.
- Preserved report fields, stable VM output, public call shape, and MIMAP-452A
  diagnostic consumption.

Validated:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh --level L2
```
