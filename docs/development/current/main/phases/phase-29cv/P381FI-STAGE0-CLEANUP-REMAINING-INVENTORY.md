# P381FI Stage0 Cleanup Remaining Inventory

Date: 2026-05-06
Scope: inventory the remaining work after P381FY so the lane can be read as "what must still land before Stage0 / Program(JSON v0) feels clean clean".

## Read

After P381FD through P381FZ, the lane is no longer blocked by:

- raw BuildBox matcher growth
- parser-proof denylist cleanup
- BuildBox wrapper hops above the private parse owner
- bundle-side handoff through private BuildBox scan-src wrappers
- BuildProgramFragmentBox wrapper indirection (defs/imports/enum_decls wrappers collapsed in P381FQ)
- imports owner parser-private utility dependency (`ParserCommonUtilsBox.esc_json`)
- enum_decls owner parser-private utility dependency (`ParserStringUtilsBox.is_alpha`)
- defs enrichment object-return dependency (`FuncScannerBox.scan_all_boxes/1`)
- defs method-body `parse_block2` result stripping on the public Build path
- stale `BuildProgramFragmentBox` object-defs builder helpers
- `ParserBox.parse_program2` as a live Stage0 lowering blocker; the parser
  Program(JSON) proof is diagnostics-only and source-owner calls use
  `nyash.stage1.emit_program_json_v0_h`
- duplicated same-module void/null sentinel const publication for the generic
  string-or-void route

The remaining work is small in count but not all the same kind:

- a small number of **must-fix owner/contract slices**
- a small amount of **optional polish / shrink-last cleanup**

So the honest reading is:

```text
late cleanup phase
  != done
  != broad redesign
```

## Remaining Must-Fix Slices

### 1. Remaining dedicated body-handling cleanup under T5

SSOT:

- `docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md`

Target-shape retirement is done, but a few capsules still have body-handling or
source-owner cleanup left under `.inc` consolidation / uniform-emitter cleanup:

- PatternUtil local-value probe body handling
- BoxTypeInspector describe body handling

These are no longer shape-expansion work. They are delete-last owner cleanup.

### 2. T6 smoke/archive inventory before larger surface reduction

SSOT:

- `docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md`
- `docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md`

The large smoke/dev surface should not be reduced by feel. Before a meaningful
archive/delete wave, the lane still needs a reachability/keeper inventory that
proves which probes are active keepers versus historical evidence.

## Optional Polish

These do not block the structural "Stage0 is behaving correctly" reading:

1. additional `.inc` helper dedup once the remaining owner blockers are gone
2. follow-up doc compaction once the last T5/T6 slices land

## Size Reading

If the question is "how much is left?", the best current answer is:

```text
must-fix:
  2 slices

optional polish:
  2 smaller follow-ups
```

That is close enough to call the lane late-stage, but not close enough to say
"only bookkeeping remains".

## Ordered Next Checklist

1. retire the remaining dedicated body-handling capsules under T5 without adding
   new Stage0 semantics
2. lock the smoke/archive inventory for T6 before any broad script reduction

## Concrete Near-Term Order

`P381FN-CONCRETE-BLOCKER-ORDER.md` is the near-term ordering SSOT.

Post-P381FY status: wrapper/enrichment cleanup is complete on the public BuildBox
Program(JSON v0) path, and the parser Program(JSON) proof boundary is closed as
diagnostics-only. The remaining concrete cleanup order is:

1. PatternUtil local-value probe body handling
2. BoxTypeInspector describe body handling

## Result

The lane is now small enough to reason about directly:

- **not** a broad compiler cleanup campaign
- **not** zero-work bookkeeping
- **yes** to a short, explicit finish list
