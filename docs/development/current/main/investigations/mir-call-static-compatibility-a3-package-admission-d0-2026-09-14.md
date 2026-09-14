---
Status: selected__design_stop__NoSafeSlice__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-A3-PACKAGE-ADMISSION-D0
Date: 2026-09-14
Priority: admit the selected same-brand mixed source through the existing package/publication chain
Parent: mir-call-static-compatibility-catalog-target-d0-2026-09-14.md
NextCard: none__a0-3-mixed-admission-witness-must-close-first
Implementation permission: false until the admission witness and source coverage boundary are closed
---

# A3 package admission design stop

## Six-line brief

```text
Decision: keep A3 at NoSafeSlice until one parser-owned Mixed admission witness covers the complete same-brand source window; then reuse the existing normal semantic package and publication owners.
Source authority + canonical issuer: CompletedParserPostpassV1 plus A0-2 lineage and A1 parent/slot/parameter seals issue the future witness; ParserNormalSourcePlanSurfaceIssuerV1, root execution, and issue_normal_callable_semantic_package_with_brand_catalog_v1 remain the sole downstream issuers.
Non-authority: MixedProgram/Initial enum labels, compatibility origin, AST or aliases, names/arity, MIR, tests, and MirBuilder::handle_static_method_call_with_descent.
Fail-fast boundary: missing or foreign lineage/brand, unsupported top-level family, stale slot, duplicate or incomplete callable/parent/body coverage, missing target/result/publication, and compatibility downgrade reject before source-plan binding or package effects.
Smallest next slice: design the A0-3 Mixed admission witness/disposition and its exact relation coverage, then map parser_program_box.hako:102 -> ParserStringUtilsBox.starts_with/3 through the existing package and static lookup/publication issuers.
Non-claims: no implementation, Mixed production acceptance, caller switch, fallback restoration, target inference, old-edge deletion, Windows evidence, or StringBox work.
```

## Current boundary

The existing normal source-plan surface returns
`PostpassNotSourceBacked` for `ParserPostpassProgramCohortV1::MixedProgram`
(`src/parser/callable_parameter_source/normal_source_plan_surface.rs`). The
postpass envelope therefore reaches the compatibility terminal before any
semantic package is issued. This is the named blocker, not a missing Builder
call-site workaround.

The selected authority direction is the whole same-brand merged invocation:

```text
CompletedParserPostpass
  + A0-2 typed import lineage
  + A1 static parent/member/slot/parameter seals
  -> Mixed admission witness
  -> ParserNormalSourcePlanSurfaceIssuerV1
  -> root execution
  -> issue_normal_callable_semantic_package_with_brand_catalog_v1
  -> ScriptDirectStaticCallLookupIssuerV1
  -> existing publication owner
```

The catalog-only alternative is declined. It would admit a static declaration
without a source-backed body/import/target/result package and would create a
second semantic authority beside the normal source plan.

## Static evidence for the stop

`finish_total_with_policy` currently sends a mixed cohort through
`from_initial_compatibility`. That path retains ordinary source seals but
`source_backed_compatibility_rows` deliberately emits static declarations as
`AstOnlyCompatibility` rows. The static-parent issuer also accepts only the
`StaticBox` cohort, so a mixed static parent is currently outside its authority
even when its prepared source relation exists. Finally,
`ParserNormalSourcePlanSurfaceIssuerV1` returns
`PostpassNotSourceBacked` for `MixedProgram` before it can bind the ordinary
and static rows. A3 must close these three boundaries together; changing only
the final `MixedProgram` guard would create an unsealed path.

There is one further unresolved join: `MergedSourceLineageV1` carries
canonical files, edges and original line ranges, while parser callable rows
carry parser-brand declaration/member paths. No current owner issues a typed
relation between those two identities. A0-3 therefore may not match them by
box name, source line, merged-text order or AST reinspection. The relation must
be issued at the parser/source handoff (or the imported cohort must remain
rejected); until that issuer and its exact coverage are named, the witness is
not implementable safely.

## Ordered bounded design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | A0-3 Mixed admission witness | One parser-brand witness co-seals typed lineage, ordinary/static parent rows, callable/member identities, parameter catalog, final slots/root, and body/import coverage. Unsupported top-level families remain explicit rejects. |
| 2 | Admission disposition and terminals | `Mixed(witness)` is source-backed only when all relations match; missing/foreign lineage, brand drift, duplicate coverage, stale slot, and incomplete body/parent rows reject before `NormalSourcePlan` and never downgrade to compatibility. |
| 3 | Existing package handoff | Connect the admitted witness to `PreparedNormalDefaultProgramRootV1::from_callable_source`, the existing package issuer/collector, and brand-aware resolver batch. No new catalog or resolver is introduced. |
| 4 | First finite target tuple | Co-seal `parser_program_box.hako:102 -> ParserStringUtilsBox.starts_with/3` by source site, callable identity, target/header, result and publication owner through `ScriptDirectStaticCallLookupIssuerV1`. Names and arity alone are insufficient. |
| 5 | Acceptance matrix | Positive same-brand mixed tuple; negative missing/foreign lineage, foreign invocation, missing target, missing result/publication, stale slot, duplicate coverage, and unsupported top-level family. Existing typed issuer terminals remain the observed rejects. |
| 6 | Cutover decision | Only after A3 acceptance, identify and delete that parser cohort's compatibility classification/raw static-child edge. Keep generic `GenericCompatibility` retirement and unrelated callers. |

## Worker receipt and limits

The read-only A3 worker confirmed the same `NoSafeSlice`: the current
`MixedProgram` rejection occurs before semantic package issuance, and missing or
foreign lineage validation is a prerequisite for production acceptance. The
worker made no edits and ran no Cargo command.

This card is design-only. A0-2 transport is closed, but its product still has
no production consumer beyond parser attachment; that validation gap is kept
as an explicit blocker here. A1, A2, caller cutover, generic fallback
retirement, StringBox findings, Windows proof, and R7 remain outside this row.
