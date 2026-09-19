---
Status: selected__design_stop__loop_route_boundary__2026-09-19
Task: MIR-CALL-STATIC-COMPATIBILITY-CATALOG-TARGET-I0
Date: 2026-09-19
Parent: mir-call-static-compatibility-catalog-target-d0-2026-09-14.md
ProductionCaller: selected normal MIR/static-receiver route only
Implementation permission: false; one finite Cataloged static-result tuple remains blocked by a named loop terminal
Classification: BoxCount; one source-backed publication consumer and one cohort-local retirement
---

# Static compatibility catalog target I0

## Six-line brief

```text
Decision: switch one admitted source row through the existing publication ingress and retire only its compatibility/static-child disposition after acceptance.
Source authority + canonical issuer: parser-branded merged source -> normal callable semantic package -> ScriptDirectStaticCallLookupIssuerV1 -> VerifiedStaticCallResultPublicationOwnerV1.
Non-authority: VM compatibility roots, AST/name/arity matching, MIR scans, line-number guesses, instance ParserBox calls, expression-If PHI, and generic fallback success.
Fail-fast boundary: missing/foreign Cataloged source context, target/result drift, unsupported argument/result shape, duplicate consume, or compatibility re-entry rejects before argument effects.
Smallest next slice: make the real merged parser invocation expose ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3 as one Cataloged/Selected row and consume it through the existing static bridge.
Non-claims: VM promotion, expression-If lowering, other parser callsites, whole-R7 retirement, Windows parity, and phase14 full-artifact success before the selected terminal is observed.
```

## Exact finite tuple

```text
caller       = ParserProgramBox.parse/2
namespace    = StaticBoxMethod
source site  = parser-issued SourceExprSiteV1 (parser_program_box.hako:102 is diagnostic only)
target       = ParserStringUtilsBox.starts_with/3
args         = String, I64, String
result       = ExactI64
required i64 = [1]
consumer     = statement IfCondition -> BinaryOp::Equal.lhs
excluded     = ParserBox instance methods, Math, compatibility roots, and all other static rows
```

The implementation must consume the source-site identity produced by the
existing callable scope. It may not manufacture a site from the Hako line,
owner name, method name, arity, or MIR. The existing
`StaticResultPublicationIngressPortV1` remains the sole ingress and
`lower_selected_static_result_publication_v1` remains the physical consumer.

## Ordered implementation tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Merged-route input | **Observed.** The real `prepare_normal_source_with_imports` route merges `parser_program_box.hako`, retains `ParserProgramBox`, `ParserStringUtilsBox`, and `starts_with`, and parses the merged source. |
| 2 | Catalog observation | The existing declaration, target, result, and publication owners expose exactly one row for the selected caller/site/target tuple. A missing row is a named rejection, not a fallback. |
| 3 | Physical consume | The static receiver route reaches `Selected`, consumes the handoff once, lowers ordered arguments, and passes the returned value to the existing statement-If condition owner. |
| 4 | Negative guards | Missing/foreign source, wrong caller namespace, target/header/result drift, wrong required ordinal, argument-count drift, duplicate consume, and compatibility re-entry reject before argument effects. |
| 5 | Cohort retirement | Remove only the selected row's compatibility/static-child disposition after `Selected` evidence. Keep Math, instance ParserBox, unselected parser rows, and generic compatibility retained. |
| 6 | Acceptance | Focused positive/negative tests and owner guards pass; selected source reaches the named static publication terminal. Phase14/16/17 source-to-exe evidence is a later closeout item unless this I0 explicitly reaches it. |

## Owner boundaries

Allowed production owners are limited to the existing chain:

```text
normal_default_root_catalog_lifecycle
  -> ScriptDirectStaticCallLookupIssuerV1
  -> VerifiedStaticCallResultPublicationOwnerV1
  -> ModuleDraftCollectorV1::install_static_result_publication_owner
  -> StaticResultPublicationIngressPortV1
  -> lower_selected_static_result_publication_v1
  -> existing statement-If / Equal consumer
```

The implementation may adjust the selected route's disposition and its
focused guards. It must not add a second catalog, resolver, publication
issuer, AST matcher, source-site remapper, compatibility retry, or PHI for
this statement call.

## Stop and rollback boundary

Stop before argument descent if the merged source remains Compatibility,
the source context is not `Cataloged(StaticBoxMethod)`, the exact row is not
present, or the result handoff is not `ExactI64` with required ordinal `[1]`.
Such a result is an I0 rejection/NoSafeSlice and must be recorded with its
first named terminal. It does not authorize widening the tuple or reopening
VM. The selected old edge remains retained until the positive terminal and
negative matrix are both observed.

## Validation contract

Run one focused Cargo process with `--profile quick` and at most four build
jobs. Use nonzero positive and negative filters plus the existing publication,
lineage, and pointer guards. Do not run the whole library or Windows CI for
this I0. Keep changed Rust sources below 800 lines and update the affected
module README/reference only when the implementation changes the contract.

## Observation receipt — 2026-09-19

The merged-route guard passes its first boundary: `prepare_normal_source_with_imports`
and the production `materialize_normal_callable_program_with_identity_and_lineage_v1`
helper accepts the real parser program input, with more than one lineage
segment and at least three import edges. The source remains `SourceBacked` and
retains a merged lineage, so this is no longer the direct
`parser_scan_loop_box` fixture. The lifecycle consumer then stops before the
static declaration/target/result catalog at the named terminal:

```text
[freeze:contract][callable-loop/route-not-front-selected]
GenericLoopV1NotSelected
```

The focused guard is
`normal_default_root_catalog_merged_route_tests::merged_parser_program_source_stops_at_named_loop_boundary_before_static_target`.
It proves the merged source and production materialization while asserting the
first loop boundary; it does not claim `Cataloged`, `Selected`, physical
static-result consumption, or compatibility retirement. The next action is a
design audit against the existing source-aware GenericLoop Recipe authority
(`mirbuilder-callable-loop-ready-generic-loop-v1-recipe-authority-d0-2026-08-22.md`).
Do not add a catalog assertion, parser-specific fallback, AST rescan, or VM
route while that dependency is unresolved.

The earlier bounded timeout and stale quick-linker failure are tooling
observations, not semantic evidence. A stale-free `CARGO_INCREMENTAL=0`
focused run completed in 6m22s with the guard green; the test body completed
in 0.16s and emitted 547 existing warnings.

## Design stop — parser loop dependency

The selected source-backed parser invocation reaches an existing callable-loop
owner before the selected static call. This is inside the selected lifecycle
boundary, so it cannot be classified as an external CI issue or bypassed by a
compatibility retry. The loop source-facts issuer and semantic Recipe/physical
adapter are the existing authorities; this card may consume their future
named terminal but may not create a second loop or static catalog owner.

The static tuple remains open with this finite stop condition:

```text
merged parser source -> source-backed materialization
  -> callable-loop source facts
  -> route selection = GenericLoopV1NotSelected
  -> static Cataloged/Selected row not reached
```

Until the existing loop Recipe authority names an accepted consumer for this
source shape, the static I0 stays at `design_stop`, the selected old edge is
retained, and no production switch or retirement is authorized.

## Read-only loop-boundary audit receipt — 2026-09-19

The parser loop dependency is now pinned to one finite source shape. The
`ParserProgramBox.parse/2` owner has an outer `loop(cont_prog == 1)` at
`lang/src/compiler/parser/program/parser_program_box.hako:81`; the selected
`starts_with/3` site at line 102 is inside that loop's declaration branch. The
same body also contains nested loops (`static_semis == 1` and `loop(true)`) and
exit-driven control, so it is outside the accepted GenericLoopV1 first cohort.

The existing `CallableGenericLoopSourceFactsIssuerV1::issue_once`,
`verify_located_generic_loop_v1`, and GenericLoop semantic/physical adapter
remain the only authorities. Their exact route requirement is raw
`[GenericLoopV1]`; the current result is the named
`GenericLoopV1NotSelected` terminal. The existing Recipe cannot consume this
shape because nested or first-cohort-ineligible loops stop before effects.

The next design task is one decision for this exact finite shape: extend the
existing Facts/Recipe and adapter together, or retain the typed terminal.
Either outcome must keep one authority and one route. No new parser Recipe
issuer, AST/MIR rescan, compatibility/VM retry, static catalog row,
production switch, or retirement is authorized while this decision is open.
