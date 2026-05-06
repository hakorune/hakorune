# P381FN Concrete Blocker Order

Date: 2026-05-06 (refreshed post-P381GI)
Scope: lock the concrete post-wrapper/post-imports/post-enum/post-defs/post-parser-boundary/post-sentinel-plumbing/post-pattern-util/post-box-type-inspector/post-smoke-inventory/tooling/candidate/delete/lifecycle/hold-closeout blocker order after P381FK/P381FL/P381FM/P381FO/P381FP/P381FQ/P381FR/P381FS/P381FT/P381FU/P381FV/P381FW/P381FX/P381FY/P381FZ/P381GA/P381GB/P381GC/P381GD/P381GE/P381GF/P381GG/P381GH/P381GI.

## Read

The public wrapper collapses are complete through:

- `P381FK-BUILDBOX-EMIT-WRAPPER-COLLAPSE`
- `P381FL-BUILDBOX-PARSE-WRAPPER-COLLAPSE`
- `P381FM-BUILDFRAGMENT-ENRICH-WRAPPER-COLLAPSE`
- `P381FO-IMPORTS-INJECT-WRAPPER-COLLAPSE`
- `P381FP-IMPORTS-DEAD-WRAPPERS-PRUNE`
- `P381FQ-DEFS-IMPORTS-WRAPPER-COLLAPSE`
- `P381FR-BUILD-BUNDLE-PUBLIC-SEAM-CLEANUP`
- `P381FS-IMPORTS-OWNER-CLEANUP`
- `P381FT-ENUM-DECLS-OWNER-CLEANUP`
- `P381FU-DEFS-TEXT-FRAGMENT-SEAM`
- `P381FV-DEFS-METHOD-BODY-PARSE-PROGRAM-SEAM`
- `P381FW-BUILDFRAGMENT-DEAD-DEFS-BUILDERS-PRUNE`
- `P381FX-DEFS-METHOD-BODY-PROGRAM-NAME-CLEANUP`
- `P381FY-PARSER-DIAGNOSTICS-BOUNDARY-CLOSEOUT`
- `P381FZ-GENERIC-STRING-OR-VOID-SENTINEL-PLUMBING`
- `P381GA-PATTERN-UTIL-PROBE-CONTRACT-HELPER`
- `P381GB-BOX-TYPE-INSPECTOR-CONTRACT-HELPER`
- `P381GC-SMOKE-ARCHIVE-INVENTORY-LOCK`
- `P381GD-SMOKE-INVENTORY-REPORT-CLASS-COLUMN`
- `P381GE-SMOKE-ARCHIVE-FIRST-CANDIDATE-LIST`
- `P381GF-SMOKE-ARCHIVE-FIRST-DELETE-WAVE`
- `P381GG-LEGACY-ROOT-SMOKE-LIFECYCLE`
- `P381GH-LEGACY-ROOT-SMOKE-DELETE`
- `P381GI-SMOKE-REFERENCED-HOLDS-CLOSEOUT`

All intermediate wrapper methods in BuildBox and BuildProgramFragmentBox have been
collapsed. The imports injection path also no longer depends on parser-private
`ParserCommonUtilsBox.esc_json`; its collector/converter children are direct
generic string bodies. The enum_decls injection path is also DirectAbi. The defs
path now consumes a scanner-owned text fragment instead of
`FuncScannerBox.scan_all_boxes/1` object results, and its method-body parser
uses the existing `ParserBox.parse_program2` seam instead of `parse_block2`.
The old object-defs JSON builder helpers in `BuildProgramFragmentBox` are also
removed.
The method-body parser seam name now matches its Program(JSON v0) return
contract.
`ParserBox.parse_program2` is now closed as an intentional diagnostics-only
proof boundary. Live source-owner Program(JSON v0) calls route through the
Stage1 runtime helper.
Generic string-or-void sentinel const publication is now shared between same-
module prepass and emit const handling.
PatternUtil local-value probe proof/return ownership is now shared by its body
module contract helper.
BoxTypeInspector describe proof/return ownership is now shared by its body
module contract helper.
T6 smoke/archive bucket counts are fixed, and broad directory deletion is
blocked until per-script candidates are proven.
The smoke inventory report summary now reads the TSV `class` column instead of
`suite_hit_count`.
The first deletion candidate wave is fixed to 45 zero-ref v2 archive scripts.
The first deletion wave removed only those 45 v2 archive scripts.
The held legacy root-smoke zero-ref group is lifecycle-classified for deletion.
That group is now deleted; remaining smoke rows are referenced or owner-held.
Referenced smoke holds are closed out for this lane.

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

1. optional doc compaction / mirror thinning
2. targeted helper dedup only when a local owner seam is clear

This keeps the lane on wrapper/owner cleanup and avoids promoting
parser-private semantics into Stage0 without an explicit parser-owner card.

## Result (post-P381GI)

`CURRENT_TASK.md` and the phase inventory should read the lane as:

- public wrapper cleanup: complete (P381FO/FP/FQ/P381FR landed)
- imports owner cleanup: complete (P381FS landed)
- enum_decls owner cleanup: complete (P381FT landed)
- defs object-return cleanup: complete enough to remove `scan_all_boxes/1` from
  the public Build fragment path (P381FU landed)
- defs method-body seam cleanup: complete; public defs path is DirectAbi
  (P381FV landed)
- dead object-defs builder cleanup: complete (P381FW landed)
- method-body program seam naming cleanup: complete (P381FX landed)
- parser diagnostics boundary closeout: complete (P381FY landed)
- generic string-or-void sentinel plumbing cleanup: complete (P381FZ landed)
- PatternUtil probe contract helper cleanup: complete (P381GA landed)
- BoxTypeInspector describe contract helper cleanup: complete (P381GB landed)
- T6 smoke/archive bucket inventory: complete enough to ban broad directory
  deletion (P381GC landed)
- smoke inventory report class-column fix: complete (P381GD landed)
- first zero-ref v2 archive candidate list: complete (P381GE landed)
- first zero-ref v2 archive delete wave: complete (P381GF landed)
- legacy root-smoke lifecycle classification: complete (P381GG landed)
- legacy root-smoke zero-ref deletion: complete (P381GH landed)
- referenced smoke holds closeout: complete (P381GI landed)
- parser-private contract discussion: no longer blocked by enrichment-side
  wrappers
- current cleanup blocker: optional polish after T6

The next slice is optional polish, not wrapper structure, not parser-private
ownership, and not another Stage0 body-shape expansion.
