# P381FI Stage0 Cleanup Remaining Inventory

Date: 2026-05-06
Scope: inventory the remaining work after P381FY so the lane can be read as "what must still land before Stage0 / Program(JSON v0) feels clean clean".

## Read

After P381FD through P381GH, the lane is no longer blocked by:

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
- PatternUtil local-value probe proof/return contract split between the body
  recognizer and top-level classifier
- BoxTypeInspector describe proof/return contract split between the body
  recognizer and top-level classifier
- uncounted T6 smoke/archive bucket reachability; P381GC locks the five target
  bucket counts and proves broad directory deletion is not allowed
- smoke inventory summary class-column drift; P381GD makes summary orphan counts
  read the TSV `class` column
- unclassified T6 delete candidates; P381GE fixes the first zero-ref v2 archive
  candidate wave
- first zero-ref v2 archive deletion; P381GF deletes exactly the 45 P381GE
  candidates
- legacy root-smoke lifecycle classification; P381GG classifies the four
  zero-ref legacy root scripts as delete candidates
- legacy root-smoke zero-ref deletion; P381GH deletes exactly the four P381GG
  candidates

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

### 1. T6 referenced smoke holds closeout

SSOT:

- `docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md`

- `docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md`

- `docs/development/current/main/phases/phase-29cv/P381GC-SMOKE-ARCHIVE-INVENTORY-LOCK.md`

- `docs/development/current/main/phases/phase-29cv/P381GD-SMOKE-INVENTORY-REPORT-CLASS-COLUMN.md`

- `docs/development/current/main/phases/phase-29cv/P381GE-SMOKE-ARCHIVE-FIRST-CANDIDATE-LIST.md`

- `docs/development/current/main/phases/phase-29cv/P381GF-SMOKE-ARCHIVE-FIRST-DELETE-WAVE.md`

- `docs/development/current/main/phases/phase-29cv/P381GG-LEGACY-ROOT-SMOKE-LIFECYCLE.md`

- `docs/development/current/main/phases/phase-29cv/P381GH-LEGACY-ROOT-SMOKE-DELETE.md`

The large smoke/dev surface should not be reduced by feel. P381GC fixed the
bucket inventory and showed that each target directory is mixed. P381GD fixed
the report summary column drift. P381GE fixed the first candidate list to 45
zero-ref v2 archive scripts, and P381GF deletes only that wave. The remaining
legacy root-smoke zero-ref group is classified by P381GG and deleted by P381GH.
The remaining smoke surface needs hold closeout, not directory deletion.

## Optional Polish

These do not block the structural "Stage0 is behaving correctly" reading:

1. additional `.inc` helper dedup once the remaining owner blockers are gone
2. follow-up doc compaction once the last T5/T6 slices land

## Size Reading

If the question is "how much is left?", the best current answer is:

```text
must-fix:
  1 slice

optional polish:
  2 smaller follow-ups
```

That is close enough to call the lane late-stage, but not close enough to say
"only bookkeeping remains".

## Ordered Next Checklist

1. close out remaining referenced smoke holds

## Concrete Near-Term Order

`P381FN-CONCRETE-BLOCKER-ORDER.md` is the near-term ordering SSOT.

Post-P381GH status: wrapper/enrichment cleanup is complete on the public
BuildBox Program(JSON v0) path, parser Program(JSON) is diagnostics-only, the
remaining T5 owner/body cleanup is closed, and T6 smoke/archive inventory is
locked as a mixed protected/referenced surface with corrected report summary
counts plus a landed first zero-ref v2 delete wave and legacy root-smoke
zero-ref deletion. The remaining concrete cleanup order is:

1. T6 referenced smoke holds closeout

## Result

The lane is now small enough to reason about directly:

- **not** a broad compiler cleanup campaign
- **not** zero-work bookkeeping
- **yes** to a short, explicit finish list
