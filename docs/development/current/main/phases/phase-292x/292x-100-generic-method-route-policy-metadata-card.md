---
Status: Active
Date: 2026-04-22
Scope: next cleanup card for moving generic method route-policy decisions out of `.inc` raw MIR inspection and into MIR-owned metadata.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
---

# 292x-100: Generic Method Route Policy Metadata

## Problem

The migrated `array_rmw_window`, `array_string_len_window`, and string
direct-set source-window routes now demonstrate the intended boundary shape:
MIR owns legality, and `.inc` reads route metadata. The remaining generic
method policy layer still has C-side route ownership around method
classification, route selection, and helper-specific fallback paths.

## Decision

Move the next route-policy decision into MIR metadata only after identifying one
narrow method family. `.inc` may keep only:

- metadata reader / field validation
- selected helper emission
- skip marking
- fail-fast on malformed metadata

## Acceptance

Define the first narrow family before editing code, then pin it with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
cargo test -q <focused-route-test>
```
