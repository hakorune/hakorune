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

The loop-owner decision is closed by the forest/exit D0: retain the typed
`GenericLoopV1NotSelected` terminal. Any future promotion requires a separate
design card that extends one existing Facts/Recipe/JoinSig/physical authority
as a co-sealed source product. No new parser Recipe issuer, AST/MIR rescan,
compatibility/VM retry, static catalog row, production switch, or retirement
is authorized by this card.

## Existing loop-owner audit decision — 2026-09-19

The read-only owner audit resolves the choice for this static I0. The parser
outer loop has `cont_prog == 1`, state assignments, early `break` exits, and
two nested loops in the same callable body. The route registry therefore
fronts the existing `LoopBreakRecipe` family before source-aware
`GenericLoopV1`; `pred_generic_loop_v1` deliberately excludes a loop when
`loop_break` facts are present. The `LoopBreakRecipe` handler still consumes a
legacy `LoopRouteContext` and lowers through `MirBuilder`; it has no
source-lineage co-seal or callable source handoff that this card can reuse.

The source-backed GenericLoop adapter is also explicit: its
`GenericLoopV1SourceLoweringContextV1` has no legacy route capability and
returns `UnsupportedFirstCohort` for nested lowering. The available
`loop_cond_break_continue` projection is `#![cfg(test)]` and only accepts a
single statement body containing an `if` with `break`/`continue`; it is not a
production consumer for this parser body.

**Decision for this I0: retain the named typed terminal and do not widen the
static tuple.** There is no existing source-aware Facts/Recipe/physical owner
that can consume this shape without a new semantic loop admission contract.
That is a `NoSafeSlice` design result for the parser dependency, not permission
to add a parser-specific issuer, re-enter the legacy route, or skip to the
static catalog. The selected compatibility edge remains retained.

If parser-loop promotion is later selected, it requires a separate bounded
design card with one existing authority extended together: source loop/exit
Facts, a LoopBreak-compatible Recipe/JoinSig co-seal, and the physical adapter
plus positive/negative ownership guards. Only after that card names an
accepted source terminal may this static I0 resume at Cataloged observation.

The source-admission D0 and its parser-loop forest/exit co-seal successor are
closed at `NoSafeSlice`. This static tuple therefore remains parked before
Cataloged/Selected; no parser-loop promotion or compatibility re-entry is
authorized. The independent source-hint fixture correction I0 is also closed;
the pointer now returns to this static design stop and its result does not
promote the tuple.

## Premise-reset audit — 2026-09-19

The repeated `NoSafeSlice` results are now treated as a premise boundary, not
as a request for another census or a parser-specific patch.

```text
semantic unit:
  the complete ParserProgramBox.parse/2 outer loop, including its nested
  static-semicolon and semicolon-scan loops, state updates, break exits, and
  return transfers; starts_with/3 is only the downstream witness.
exact membership:
  parser_program_box.hako:81 (cont_prog loop), :102 (starts_with site),
  :131 (static_semis loop), :182 (semicolon scan), plus their state writes and
  early exits.
classifier arms:
  LoopBreak direct-three-statement recognition; specialized break/continue and
  return routes; GenericLoopV1's raw [GenericLoopV1] selection; and the
  non-nested CallableSingleLoop cohort.
transferred or opaque subtree:
  nested loop bodies and exit transfers are present in resolver forest/exit
  data, but the callable source Facts and physical adapter carry only the
  direct condition/body relation. Dropping those relations would lose the
  source owner at the publication boundary.
type requirements:
  resolver forest/exits and portable Recipe fields exist, but no callable
  source issuer currently co-seals the parser forest plus resolved exits into
  a Recipe/JoinSig product consumed by the physical adapter.
counterexample:
  admitting only the line-102 starts_with call leaves the line-131 and
  line-182 nested loops and their early exits without an owned terminal.
```

**Premise decision.** Extending the existing GenericLoopV1 or LoopBreak owner
inside this static I0 is not a safe slice: it would require a new semantic
loop-admission contract and a new co-sealed source product. Retain the typed
`GenericLoopV1NotSelected` terminal and the compatibility edge. Do not add a
parser issuer, AST/MIR rescan, compatibility/VM retry, catalog row, production
switch, or retirement under this card.

**Frontier pause.** There is no executable successor for this tuple until
another inventoried family satisfies its own exclusive-owner and delete-set
boundary, or a separately accepted parser-loop promotion design names the
existing authority to extend. This card may resume at Cataloged observation
only after that decision; the current pointer remains a design stop.

## Frontier inventory check — 2026-09-19

The existing queue was checked once after the premise reset, so the pause is
not based on an unexamined historical mirror:

| Existing row | Current evidence | Scheduling result |
| --- | --- | --- |
| `MIRBUILDER-FINAL-ACCEPTANCE-SCOPE-R0` | Its fixed 11-entry acceptance handoff is already closed with recorded results and owner terminals. | No executable acceptance row remains here. |
| `MIR-CALL-COMPATIBILITY-RETIRE-R7-M7S-OWNER-MATRIX-D0` | The finite physical-options matrix is closed as `NoSafeSlice__M7SNoRemainingUnsharedDeleteSet`. | No exclusive delete-set row may be invented. |
| `MIR-CALL-LOOP-TERMINAL-RETURN-I0` | The bounded completion slice is landed; its route evidence reaches the next named terminal. | No unfinished return-completion work is selected. |
| `MIR-CALL-R7-STRINGBOX-LOWER-STRUCTURAL-MEMBERSHIP-I0` | The Hako owner is implemented, but its dynamic owner evidence still stops at the same static-call terminal. | It cannot bypass this static design stop. |
| OwnedText T3 | `ParkedSealed__NoSelectedOwnedTextCaller` remains the recorded disposition. | It is not reopened by this pause. |

This leaves the parser-loop promotion decision as the only open successor
inside the selected static dependency. It must be a separate design card that
extends one existing semantic authority and names its source/exit co-seal,
physical consumer, negative ownership matrix, and eventual old-edge delete
set before any implementation permission or catalog observation is opened.
