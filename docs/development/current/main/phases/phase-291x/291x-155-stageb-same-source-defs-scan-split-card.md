---
Status: Landed
Date: 2026-04-24
Scope: Split Stage-B same-source defs scanning out of the Stage-B entry adapter.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-154-stageb-main-detection-helper-split-card.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_same_source_defs_box.hako
  - lang/src/compiler/entry/stageb_main_detection_box.hako
  - lang/src/compiler/entry/stageb/stageb_json_builder_box.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-155 Stage-B Same-Source Defs Scan Split Card

## Goal

Continue HCM-8 Stage-B thin-adapter work by moving same-source helper method
scanning out of `compiler_stageb.hako`:

```text
compiler_stageb.hako inline same-source defs scanner
  -> lang.compiler.entry.stageb_same_source_defs_box
```

This keeps `StageBDriverBox.compile(...)` adapter-shaped: parse the entry body,
ask a named box for defs metadata, then inject the returned JSON fragment.

## Design

Create `stageb_same_source_defs_box.hako` with:

```text
StageBSameSourceDefsBox.build_fragment(source)
```

The new box owns the current compatibility scanner for same-source boxes. It
continues to use `MainDetectionHelper` for brace/pattern walking and
`ParserBox.parse_block2(...)` for method body JSON. Defs JSON emission is
delegated to `StageBJsonBuilderBox` instead of keeping another params/defs JSON
builder inside `compiler_stageb.hako`.

## Boundary

- BoxShape only.
- No parser invocation changes.
- No JSON fragment injection split in this card.
- No replacement with `FuncScannerBox.scan_all_boxes(...)` in this card; that
  route has different comment/string/Rune behavior and needs a separate parity
  card before promotion.
- No CoreMethodContract, `.inc`, or runtime lowering changes.
- Preserve `Main` box skip behavior and implicit `me` params.

## Implementation

- Added `lang/src/compiler/entry/stageb_same_source_defs_box.hako`.
- Moved same-source box/method scanning helpers out of
  `compiler_stageb.hako`.
- Kept `compiler_stageb.hako` responsible only for requesting the defs
  fragment and injecting it into the Program JSON.
- Reused `StageBJsonBuilderBox.build_defs_json(...)` for defs JSON emission
  instead of keeping another params/defs JSON builder in the Stage-B adapter.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh`
- `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_multi_method_shape_canary_vm.sh`
- `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh`
- `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Validation Notes

- Additional PASS:
  `HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh`
- Additional PASS: `bash tools/checks/dev_gate.sh quick`
