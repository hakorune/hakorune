---
Status: Landed
Date: 2026-04-24
Scope: Thin the Stage-B entry adapter by moving trace ownership to the existing Stage-B trace box.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-131-hotline-core-method-contract-task-plan.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_trace_box.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-152 Stage-B Trace Adapter Thinning Card

## Goal

Start HCM-8 Stage-B thin-adapter work with the smallest BoxShape-only slice:

```text
compiler_stageb.hako inline StageBTraceBox
  -> lang.compiler.entry.stageb_trace_box
```

This removes a duplicated trace helper from the Stage-B entry file without
changing parser authority, body extraction, same-source defs injection, or
generic-method/CoreMethod lowering.

## Design

`compiler_stageb.hako` should import the existing trace box and keep only entry
adapter/orchestration logic:

```text
using lang.compiler.entry.stageb_trace_box as StageBTraceBox
```

The existing `stageb_trace_box.hako` remains the trace SSOT for
`HAKO_STAGEB_TRACE=1` logging. This card deliberately does not touch the older
direct `print("[stageb/...")` diagnostics; those need a separate trace cleanup
card because some are error/freeze markers rather than trace-only logs.

## Boundary

- BoxShape only.
- No parser behavior change.
- No Stage-B args/source resolver split in this card.
- No same-source defs or JSON fragment extraction in this card.
- No CoreMethodContract, `.inc`, or runtime lowering changes.

## Implementation

- Removed the inline `StageBTraceBox` definition from
  `compiler_stageb.hako`.
- Added `using lang.compiler.entry.stageb_trace_box as StageBTraceBox` to the
  Stage-B entry adapter.
- Kept direct Stage-B diagnostic/error prints unchanged; they are not pure trace
  helper duplication.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
