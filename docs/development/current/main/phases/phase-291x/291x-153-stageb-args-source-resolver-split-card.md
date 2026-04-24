---
Status: Landed
Date: 2026-04-24
Scope: Split Stage-B CLI args/source resolution out of the Stage-B entry adapter.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-152-stageb-trace-adapter-thinning-card.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_args_box.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-153 Stage-B Args/Source Resolver Split Card

## Goal

Continue HCM-8 Stage-B thin-adapter work by moving CLI args/source resolution
out of `compiler_stageb.hako`:

```text
compiler_stageb.hako inline StageBArgsBox
  -> lang.compiler.entry.stageb_args_box
```

This keeps the Stage-B entry file focused on orchestration and leaves source
selection policy in one named box.

## Design

Create `stageb_args_box.hako` with:

```text
StageBArgsBox.resolve_src(args)
StageBArgsBox.resolve_stage3(args)
```

The moved box keeps existing behavior:

```text
--source
--source-file + HAKO_SOURCE_FILE_CONTENT
HAKO_SRC
default "return 0"
--stage3
```

It also keeps the existing `HAKO_STAGEB_TRACE` diagnostics and imports
`stageb_trace_box.hako` as the trace helper.

## Boundary

- BoxShape only.
- No parser invocation changes.
- No body extraction or same-source defs split in this card.
- No CoreMethodContract, `.inc`, or runtime lowering changes.
- Do not change the resolution precedence for source inputs.

## Implementation

- Added `lang/src/compiler/entry/stageb_args_box.hako`.
- Moved `StageBArgsBox.resolve_src(...)` and
  `StageBArgsBox.resolve_stage3(...)` out of `compiler_stageb.hako`.
- Added `using lang.compiler.entry.stageb_args_box as StageBArgsBox` to the
  Stage-B entry adapter.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
