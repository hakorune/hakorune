# P381FN Concrete Blocker Order

Date: 2026-05-06 (refreshed post-P381GI)
Scope: lock the near-term order after wrapper/owner cleanup, T5 body contract cleanup, and T6 smoke closeout.

## Read

Closed work is recorded in the individual P381 cards. The current compressed
reading is:

- public BuildBox / BuildProgramFragmentBox wrappers and source-owner seams are
  collapsed or closed
- parser Program(JSON) is diagnostics-only at the Stage0 boundary
- generic string-or-void, PatternUtil, and BoxTypeInspector contract ownership
  is shared at the body/helper seam
- T6 smoke/archive cleanup is closed for this lane; future smoke deletion needs
  an owner-specific card

## Current Blocker Order

```text
BuildBox.emit_program_json_v0/2
  -> BuildBox._parse_program_json/2
     status: diagnostics_only parser proof, not a live Stage0 lowering blocker

  -> BuildProgramFragmentBox._inject_defs_json/2
     status: DirectAbi

  -> BuildProgramFragmentBox._inject_enum_decls_json/2
     status: DirectAbi
```

## Decision

The enrichment-side blockers are resolved, and the parser proof boundary is
closed as diagnostics-only. The next cleanup slice should not invent another
enrichment wrapper or promote parser-private ownership.

Current preferred order:

1. targeted helper dedup only when a local owner seam is clear

This keeps the lane on wrapper/owner cleanup and avoids promoting
parser-private semantics into Stage0 without an explicit parser-owner card.

## Result (post-P381GI)

`CURRENT_TASK.md` and the phase inventory should read the lane as:

- must-fix wrapper/owner/body cleanup: complete
- T6 smoke/archive cleanup: complete for this lane
- parser-private contract discussion: no longer blocked by enrichment-side wrappers
- current cleanup blocker: targeted helper dedup if local seam is clear

The next slice is targeted helper dedup if a local seam is clear, not wrapper
structure, not parser-private ownership, and not another Stage0 body-shape
expansion. If a real Stage0 expressivity blocker appears, switch lanes to a
fixture-backed BoxCount card.
