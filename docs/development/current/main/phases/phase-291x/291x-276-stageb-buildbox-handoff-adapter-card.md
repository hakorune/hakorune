---
Status: Landed
Date: 2026-04-26
Scope: Stage-B entry adapter cleanup / BuildBox authority handoff.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_compile_adapter_box.hako
  - lang/src/compiler/build/build_box.hako
---

# 291x-276: Stage-B BuildBox Handoff Adapter

## Problem

`compiler_stageb.hako` says it is a thin Stage-B adapter, but its compile path
still directly owns parser/body/defs/json-fragment calls. That keeps a second
`source -> Program(JSON v0)` authority beside `BuildBox.emit_program_json_v0`.

## Decision

Keep `compiler_stageb.hako` as the entry/driver wrapper only.

Move Stage-B source-to-Program handoff into a small adapter:

```text
compiler_stageb.hako
  -> StageBCompileAdapterBox
  -> BuildBox.emit_program_json_v0(...)
```

The adapter may own entry-local input adaptation:

- Stage-3 env flags
- checked BuildBox handoff
- legacy `local` keyword JSON cleanup

It must not own parser/body/defs/import scanning authority.

CLI bundle/require option packaging remains a separate follow-up. This card
only removes the duplicate parser/scanner authority from the Stage-B entry.

## Implementation Plan

1. Add `StageBCompileAdapterBox`.
2. Delegate `StageBDriverBox.compile(...)` to the adapter.
3. Make `StageBDriverBox.main(...)` call the same adapter path.
4. Remove direct parser/body/defs/json helper imports from
   `compiler_stageb.hako`.

## Acceptance

- `compiler_stageb.hako` no longer imports `ParserBox`,
  `StageBBodyExtractorBox`, `StageBSameSourceDefsBox`, or
  `StageBJsonBuilderBox`.
- `BuildBox.emit_program_json_v0(...)` remains the sole source-to-Program
  authority.
- Stage-B binop smoke remains green.
- Stage-B quick minimal emit remains green.

## Verification

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.

## Notes

An initial bundle-args packaging probe was not kept in this card. It made the
slice larger than the adapter handoff and hit the existing Stage-B selfhost
compile fragility before reaching bundle tag assertions. Keep that as a
separate BoxShape card if bundle CLI routing becomes the next blocker.
