---
Status: Landed
Date: 2026-04-24
Scope: Restore the Stage-B direct binop smoke by tightening scanner/parser loop shapes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/compiler/entry/func_scanner_helpers.hako
  - lang/src/compiler/parser/scan/parser_number_scan_box.hako
  - lang/src/compiler/parser/scan/parser_common_utils_box.hako
  - lang/src/compiler/parser/scan/parser_string_utils_box.hako
  - tools/smokes/v2/lib/stageb_helpers.sh
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-130 Stage-B Parser Scanner PHI Guard Card

## Goal

Restore `stageb_binop_vm.sh` before resuming the Stage-B entry cleanup.

This is a BoxShape/bugfix card. It does not add a language feature, route row,
environment variable, fallback compiler authority, or CoreBox surface.

## Problem

Two tiny scanner loops used early `continue` exits in a way that the Stage-B VM
path could lose loop-carried state:

- `FuncScannerHelpersBox.find_matching_brace(...)` could stay inside string
  mode after seeing the closing quote, so `compiler_stageb.hako` scanned as
  `defs=0` and `StageBDriverBox.main/1` fell through as an unsupported extern.
- `ParserNumberScanBox.scan_int(...)` could lose digit cursor progress, so
  `return 1+2` emitted `{"type":"Int","value":1+2}` instead of a `Binary` node.

Stage-B smoke helpers also inherited `NYASH_JOINIR_DEV=1` from the generic smoke
environment, which polluted the emit-only raw JSON stream with dev JoinIR tags
and could route the Stage-B compiler entry through the wrong dev path.

## Implementation

- Rewrote `find_matching_brace(...)` as a single-exit `next_i/result` scanner.
- Rewrote `scan_int(...)` as a single-exit digit scanner.
- Normalized parser character predicates to return `1/0`, matching their
  existing `== 1` call sites.
- Fixed Stage-B emit helpers to run with `NYASH_JOINIR_DEV=0` while keeping
  `HAKO_JOINIR_STRICT=0`.

## Proof

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
```

Focused probes confirmed:

- `FuncScannerBox.scan_all_boxes(compiler_stageb.hako)` includes
  `StageBDriverBox.main`.
- `ParserNumberScanBox.scan_int("1+2", 0)` returns the cursor after `1`.
- `ParserBox.parse_program2("return 1+2")` emits a `Binary` node.

## Next

- Retry the Stage-B entry cleanup that thins `compiler_stageb.hako`.
- Keep future scanner loop repairs in single-exit PHI-safe form.
