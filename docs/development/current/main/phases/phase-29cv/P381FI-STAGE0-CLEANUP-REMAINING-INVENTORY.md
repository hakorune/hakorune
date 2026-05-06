# P381FI Stage0 Cleanup Remaining Inventory

Date: 2026-05-06
Scope: inventory the remaining work after P381FY so the lane can be read as "what must still land before Stage0 / Program(JSON v0) feels clean clean".

## Read

After P381FD through P381GM, the lane is no longer blocked by:

- public BuildBox / BuildProgramFragmentBox wrapper and owner cleanup
- parser Program(JSON) proof routing; `ParserBox.parse_program2` is
  diagnostics-only and source-owner calls use `nyash.stage1.emit_program_json_v0_h`
- generic string-or-void, PatternUtil, and BoxTypeInspector body/contract split
  cleanup
- T6 smoke/archive inventory, tooling, zero-ref delete waves, and referenced
  hold closeout
- MIR call extern emit row duplication; named validators now read the shared
  local rule table
- same-module MIR JSON `key_const_text` helper ownership; method views now own
  the read helper and emit code consumes it
- public Stage1 Program(JSON v0) runtime-helper lowering;
  `BuildBox.emit_program_json_v0(source, null)` now lowers through
  `nyash.stage1.emit_program_json_v0_h(source)`

The remaining work is optional polish:

- a small amount of **optional polish / shrink-last cleanup**

So the honest reading is:

```text
late cleanup phase
  != done
  != broad redesign
```

## Remaining Must-Fix Slices

None in this cleanup lane after P381GI.

## Optional Polish

SSOT:

- `docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md`
- `docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md`
- `docs/development/current/main/phases/phase-29cv/P381GI-SMOKE-REFERENCED-HOLDS-CLOSEOUT.md`

The large smoke/dev surface should not be reduced by feel. P381GC through
P381GI close the T6 zero-ref cleanup and park remaining referenced/owner-held
rows.

This does not block the structural "Stage0 is behaving correctly" reading:

1. targeted `.inc` / helper dedup only when another local owner seam is clear

## Size Reading

If the question is "how much is left?", the best current answer is:

```text
must-fix:
  0 slices

optional polish:
  1 optional follow-up class
```

That is close enough to call the lane late-stage, but not close enough to say
"only bookkeeping remains".

## Ordered Next Checklist

1. targeted helper dedup only when another local owner seam is clear

## Concrete Near-Term Order

`P381FN-CONCRETE-BLOCKER-ORDER.md` is the near-term ordering SSOT.

Post-P381GM status: wrapper/enrichment cleanup, parser diagnostics boundary,
remaining T5 owner/body cleanup, and T6 smoke/archive cleanup are closed. The
remaining concrete cleanup order is optional polish:

1. targeted helper dedup only when another local owner seam is clear

## Result

The lane is now small enough to reason about directly:

- **not** a broad compiler cleanup campaign
- **not** zero-work bookkeeping
- **yes** to a short, explicit finish list
