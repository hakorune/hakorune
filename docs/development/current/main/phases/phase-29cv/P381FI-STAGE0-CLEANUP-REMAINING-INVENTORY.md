# P381FI Stage0 Cleanup Remaining Inventory

Date: 2026-05-06
Scope: inventory the remaining work after P381FH so the lane can be read as "what must still land before Stage0 / Program(JSON v0) feels clean clean".

## Read

After P381FD through P381FX, the lane is no longer blocked by:

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

### 1. BuildBox wrapper chain owner cleanup

SSOT:

- `docs/development/current/main/phases/phase-29cv/P381BS-PARSER-PROGRAM-JSON-BODY-EMITTER-BLOCKER.md`
- `lang/src/compiler/build/build_box.hako`

The old direct parser contract is a one-argument body:

```text
BuildBox._parse_program_json/1
```

but the live owner is:

```text
BuildBox._parse_program_json(parse_src, scan_src)/2
```

with enum-inventory setup in between.

After the live Stage1 CLI probe and P381FL, the immediate same-module blockers
now start at the public helper body:

```text
BuildBox.emit_program_json_v0/2
  -> BuildBox._parse_program_json/2                      (blocker: ParserBox.parse_program2)
  -> BuildProgramFragmentBox._inject_defs_json/2         (DirectAbi)
  -> BuildProgramFragmentBox._inject_enum_decls_json/2   (DirectAbi)
```

Note: All public wrapper collapses are done through P381FO/FP/FQ/P381FR, and
the imports/enum/defs owner cleanup is done through P381FS/P381FT/P381FU/P381FV/P381FW. The
BuildProgramFragmentBox methods now follow a uniform direct pattern without
intermediate wrappers; imports and enum_decls have direct generic children. The
defs path now uses `FuncScannerBox.collect_defs_fragment_json/1`, so it no
longer consumes ArrayBox/MapBox def records on the public Build path, and its
method-body parsing uses `ParserBox.parse_program2` instead of `parse_block2`.
The old object-defs builder helpers have been removed from
`BuildProgramFragmentBox`.

`_parse_program_json/2` is now the first live parser-private stop in the public
BuildBox authority body. The old enrich-side blockers are no longer the active
near-term blocker.

### 2. Remaining dedicated body-handling cleanup under T5

SSOT:

- `docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md`

Target-shape retirement is done, but a few capsules still have body-handling or
source-owner cleanup left under `.inc` consolidation / uniform-emitter cleanup:

- parser Program(JSON)
- generic string-or-void sentinel plumbing
- PatternUtil local-value probe body handling
- BoxTypeInspector describe body handling

These are no longer shape-expansion work. They are delete-last owner cleanup.

### 3. T6 smoke/archive inventory before larger surface reduction

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
  3 slices

optional polish:
  2 smaller follow-ups
```

That is close enough to call the lane late-stage, but not close enough to say
"only bookkeeping remains".

## Ordered Next Checklist

1. choose the BuildBox wrapper-chain resolution:
   source-owner cleanup vs MIR-owned contract that removes the unsupported
   `_parse_program_json_from_scan_src/1` hop
2. retire the remaining dedicated body-handling capsules under T5 without adding
   new Stage0 semantics
3. lock the smoke/archive inventory for T6 before any broad script reduction

## Concrete Near-Term Order

`P381FN-CONCRETE-BLOCKER-ORDER.md` is the near-term ordering SSOT.

Post-P381FW status: wrapper/enrichment cleanup is complete on the public BuildBox
Program(JSON v0) path. The remaining concrete parser-owned seam is:

1. `_parse_program_json/2` (calls ParserBox.parse_program2 → diagnostics-only direct route, parser owner still unresolved)

## Result

The lane is now small enough to reason about directly:

- **not** a broad compiler cleanup campaign
- **not** zero-work bookkeeping
- **yes** to a short, explicit finish list
