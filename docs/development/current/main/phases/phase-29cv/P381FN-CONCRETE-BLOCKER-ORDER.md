# P381FN Concrete Blocker Order

Date: 2026-05-06 (refreshed post-P381FY)
Scope: lock the concrete post-wrapper/post-imports/post-enum/post-defs/post-parser-boundary blocker order after P381FK/P381FL/P381FM/P381FO/P381FP/P381FQ/P381FR/P381FS/P381FT/P381FU/P381FV/P381FW/P381FX/P381FY.

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

1. generic string-or-void sentinel plumbing
2. `PatternUtil` local-value probe body handling
3. `BoxTypeInspector` describe body handling
4. T6 smoke/archive inventory

This keeps the lane on wrapper/owner cleanup and avoids promoting
parser-private semantics into Stage0 without an explicit parser-owner card.

## Result (post-P381FY)

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
- parser-private contract discussion: no longer blocked by enrichment-side
  wrappers
- current cleanup blocker: remaining T5 owner/body handling

The next slices must address the remaining T5 body/owner seams, not wrapper
structure and not parser-private ownership.
