# Normal root execution atomic cutover manifest D0

Status: accepted design — T0 closed; atomic C0 selected
Date: 2026-08-23
Decision: NORMAL-ROOT-EXECUTION-ATOMIC-CUTOVER-MANIFEST-D0
Parent: NORMAL-ROOT-EXECUTION-REFERENCE-ROUTE-CLOSURE-D0

## Six-line brief

Decision:
  Accept one bounded series: behavior-neutral `T0`, one atomic semantic `C0`,
  then bool/comment cleanup `R0`. The pre-audit C0 manifest is superseded.
Source authority + canonical issuer:
  Existing `ParserNormalSourcePlanSurfaceIssuerV1::issue_once`, followed once
  by `ParserNormalRootExecutionIssuerV1::issue_once`; no second surface issuer.
Non-authority:
  narrow Main0/Script-A products, AST inventory/raw expansion, names,
  ordinals, pointers, Builder state, work-plan bool, Recipe, and MIR.
Fail-fast boundary:
  One typed route consumer must consume the total relation before AST
  extraction, source-plan policy, retained/test exit, or the first Builder effect.
Smallest next slice:
  `NORMAL-ROOT-EXECUTION-ATOMIC-CUTOVER-C0`: replace all twelve in-bound
  routes and old authorities in one semantic commit.
Non-claims:
  No semantic Rust in T0; no language-cohort, Recipe/Join/MIR, compatibility,
  fallback, performance, or physical-shelf expansion in C0.

## Decision reconciliation

The older review is directionally correct, but its first parser slice is now
landed. Current `main` already has:

- one `ParserNormalSourcePlanSurfaceIssuerV1::issue_once` production call;
- complete top-level and nested static-member rows;
- `CompleteEmpty` distinct from missing/incomplete coverage;
- full static-member ownership in the source-plan surface instead of the
  narrow static-Main seal.

The remaining production debt is still real:

```text
Parser SourceSurface
  -> silent drop or AST re-scan
  -> source-plan family / App-Script bool
  -> another raw-root scan
  -> Builder effect
```

The accepted finish line is:

```text
Parser SourceSurface
  -> one total RootExecution relation
  -> exact transform preservation or one route-specific terminal
  -> one Compiler SourcePlan
  -> one admitted RootExecutionInput
  -> Builder effect
```

`SourceSurface`, `SourcePlan`, and `RootExecutionInput` are the three stable
terms. A parser source surface owns observed facts; compiler policy owns the
Script/Main0/CallableModule decision; Builder only consumes admitted execution
input. No later stage may reissue an earlier meaning.

## Census boundary — corrected after independent audit

This census covers the following exact boundary:

```text
ParsedProgramWithCallableParameterSourceV1::new
  -> every product move/destructure and profile split
  -> normal/default exact transform and first typed Builder consumer
  -> canonical source-plan policy and its three typed terminals
  -> Raw-only discard plus the first authorized extraction
  -> compatibility absence/extraction terminal
  -> retained/test terminal
```

Includes:

- every owner that can drop, reclassify, consume, or bypass the unconsumed
  source surface, total-root relation, or Script-A sibling;
- the first normal/default consumer before declaration/catalog/module effects;
- `SealedNormalScriptSourceV1`, `SealedNormalMainSourceV1`, and
  `SealedNormalCallableModuleSourceV1` until their old AST-owning input is
  replaced;
- compatibility until its source-authority absence is consumed explicitly.

Excludes:

- the separately typed AST-only fixture owner;
- Raw runtime/compiler after one authorized extraction;
- Recipe/module/physical work after it receives a consumed typed terminal.

The parent D0's earlier `open=6, parked=8` classification was too narrow.
Rows 7–9 and 11–13 still receive or drop an unconsumed parser relation, or
reclassify from `PreparedNormalSourcePlanInputV1`, so they are C0 blockers.

| # | Entry | Current state | C0 terminal |
| ---: | --- | --- | --- |
| 1 | parser product `new` | open | one aggregate total issuer |
| 2 | source-backed `into_normal_callable_program` | open | exact preserved relation |
| 3 | canonical `prepare_source_plan_request` | open | parser-bound policy terminal |
| 4 | source-backed `prepare_raw_vm_handoff` | open | atomic Raw discard/extraction |
| 5 | `into_retained_source` | open | required retained field |
| 6 | direct parser-product test helpers | open | named test terminal |
| 7 | compatibility normal/default | open | named absence closure |
| 8 | compatibility Raw | open | compatibility-only extraction |
| 9 | compatibility canonical | open | `CompatibilitySourceUnavailable` |
| 10 | AST-only source-plan fixture | parked | unchanged fixture owner |
| 11 | sealed Script terminal | open | consumed bound Script owner |
| 12 | sealed Main0 terminal | open | consumed bound Main owner |
| 13 | sealed CallableModule terminal | open | consumed bound module owner |
| 14 | Raw runtime after authorized extraction | parked | unchanged execution owner |

Current classification is:

```text
Exhausted(14)
CutoverBlockerOpen = 12
ParkedSealed = 2
Unclassified = 0
```

C0 closes only at `CutoverBlockerOpen = 0`. Rows 11–13 become parkable only
after their source-backed input no longer owns `PreparedNormalSourcePlanInputV1`
and cannot classify from AST.

## Current owner/caller census

The landed source-surface owner is retained in place:

```text
ParserNormalSourcePlanSurfaceIssuerV1::issue_once
  definition = 1
  production caller = 1
```

Current open exits are:

| Row | Current owner / symbol | Current caller evidence | Defect |
| ---: | --- | --- | --- |
| 1 | `ParsedProgramWithCallableParameterSourceV1::new` | direct 1 | total execution issuer absent |
| 2 | `ParserCallableSourceDispositionV1::into_normal_callable_program` | direct production callsites 2 | `normal_source_plan_surface: _` |
| 3 | `prepare_source_plan_request` | production 1, test 7 | enters whole-AST inventory |
| 4 | `prepare_raw_vm_handoff` | production 1, test 4 | drops Script-A sibling then `into_ast()` |
| 5 | `into_retained_source` | production 0, test 1 | `..` drops the surface |
| 6 | parser test helpers | 15 callsites in 9 files | no named terminal |

The source-backed normal/default lifecycle also performs three raw-root
classifications before its semantic package is complete:

```text
normal_default_root_catalog_lifecycle.rs        = 2
callable_declaration_catalog/source_backed.rs   = 1
source-backed pointer pairing                   = 1
```

The first typed consumer must run before
`PreparedNormalProgramDeclarationFactsV1::collect`, catalog installation,
module preparation, target capability installation, or any Builder mutation.

## Canonical owner manifest

### Parser source and transport

| Responsibility | Sole owner / symbol | Production file | Required test owner |
| --- | --- | --- | --- |
| observed source surface | existing `ParserNormalSourcePlanSurfaceIssuerV1::issue_once` | `src/parser/callable_parameter_source/normal_source_plan_surface.rs` | `normal_source_plan_surface_tests.rs` |
| non-empty rows invariant | `NonEmptyParserNormalSourcePlanRowsV1` | same file | `normal_root_execution/tests.rs` |
| total App/ProgramRuntime relation | `ParserNormalRootExecutionIssuerV1::issue_once` | `src/parser/callable_parameter_source/normal_root_execution/issuer.rs` | `normal_root_execution/tests.rs` |
| total model/owner | `ParserNormalRootExecutionSourceDispositionV1` | `src/parser/callable_parameter_source/normal_root_execution/model.rs` | same |
| compatibility absence closure | `ParserNormalRootExecutionCompatibilityClosureV1::consume_once` | `src/parser/callable_parameter_source/normal_root_execution/compatibility.rs` | same |
| product attachment | required `normal_root_execution` field | `src/parser/callable_parameter_source/product.rs` | parser product tests |

`ParserNormalRootExecutionSourceDispositionV1::Ready` owns the existing
`ParserBackedNormalSourcePlanBoundV1`; the parser product does not retain a
parallel `normal_source_plan_surface` or old `normal_root_source` field.
`ParserNormalSourcePlanSurfaceV1` remains the one surface vocabulary. C0 must
not introduce `ParserBackedNormalSourcePlanSurfaceV1` as a parallel type.

`CompleteRows` is strengthened to own the private non-empty wrapper. The
unreachable `CompatibilityOutside` surface variant retires; compatibility is
an outer route with a named absence closure, never an empty/default surface.

The old narrow authority family retires in the same C0:

```text
main_app_entry.rs
normal_root_source.rs
normal_root_preservation.rs
their authority guards and narrow transport tests
```

Any small Main0 projection retained by policy is issued from the total
surface and is explicitly non-authoritative for App/ProgramRuntime.

### Exact transform and normal/default consumption

| Responsibility | Sole owner / symbol | Production file | Required test owner |
| --- | --- | --- | --- |
| exact preservation | `ParserNormalRootExecutionPreservationIssuerV1::seal_after_transform` | `src/parser/normal_callable_program_source/normal_root_execution_preservation.rs` | `normal_root_execution_preservation_tests.rs` |
| final required owner | `VerifiedFinalCallableProgramSourceV1::normal_root_execution` | `src/parser/normal_callable_program_source/model.rs` | same |
| pre-effect consumer | `NormalRootExecutionConsumerV1::consume_once` | `src/mir/builder/normal_root_execution/consumer.rs` | `normal_root_execution/tests.rs` |
| admitted aggregate | `PreparedNormalRootExecutionConsumptionV1` | `src/mir/builder/normal_root_execution/model.rs` | same |
| consumed callable source | `ConsumedNormalRootCallableSourceV1` | same | same |
| lifecycle facade | `PreparedNormalDefaultProgramRootV1::consume_source_backed_root_once` | `normal_default_root_catalog_lifecycle.rs` | new consumer tests |

The consumer uses one scoped syntax loan only to project already-paired App
root/static-child or ProgramRuntime body syntax. It does not call
`VerifiedRawRootExpansionV1::from_program`, compare AST pointers, or classify
by name/ordinal. `normal_callable_semantic_package` accepts the consumed source
in production. A `cfg(test)` convenience wrapper may consume through this same
owner; it may not bypass it.

C0 may derive an `AdmittedNormalRootExecutionModeV1` projection for existing
work-plan plumbing. That enum/bool is downstream transport, not authority.
Removing `MirBuilder::root_is_app_mode` and the bool-shaped work-plan API is
the immediately reserved R0, not a reason to add another C0 classifier.

### Canonical source-plan policy and terminals

| Responsibility | Sole owner / symbol | Production file | Required test owner |
| --- | --- | --- | --- |
| one-shot canonical bind | `ParserNormalRootSourcePlanConsumerV1::consume_once` | `src/parser/callable_parameter_source/normal_source_plan_consumer.rs` | compiler parser-bound tests |
| bound source owner | `SourcePlanBoundNormalCallableSourceV1` | same | same |
| pure policy | `NormalSourcePlanClassifierV1::seal_parser_bound` | `src/mir/compiler/normal_source_plan/parser_bound_policy.rs` | `parser_bound_policy_tests.rs` |
| existing result family | `SealedNormalSourcePlanV1` | `normal_source_plan/product.rs` | existing + parser-bound tests |

The existing `SealedNormalScriptSourceV1`, `SealedNormalMainSourceV1`, and
`SealedNormalCallableModuleSourceV1` remain the sole result family, but their
source-backed owner changes from `PreparedNormalSourcePlanInputV1` to the
consumed parser-bound owner. Do not add a parallel `ParserBackedSealedNormal*`
family. Their terminal consumers may extract syntax after admission, but may
not rescan App/ProgramRuntime or source-plan family.

`NormalSourceSurfaceInventoryV1::collect` and
`PreparedNormalSourcePlanInputV1::new` remain only for AST-only fixtures.

### Reference route, Raw, and retained/test closure

| Responsibility | Sole owner / symbol | Production file | Required test owner |
| --- | --- | --- | --- |
| typed route split | `PreparedNormalFileParsedRouteV1::{Raw, Canonical}` | `normal_file_vm_frontdoor/parser_source_handoff.rs` | `atomic_root_cutover_tests.rs` |
| source-backed Raw closure | `ParserNormalRootExecutionRawVmDiscardIssuerV1::issue_once` | `normal_file_vm_frontdoor/raw_source_handoff.rs` | same |
| atomic Raw product | `PreparedRawVmSourceExtractionV1` | same | same |
| compatibility extraction | `RawCompatibilitySourceExtractionIssuerV1::issue_once` | same | same |
| retained owner | required relation in `RetainedParserCallableSemanticSourceV1` | `callable_parameter_source/retained.rs` | `retained_tests.rs` |
| test terminal | `ParserNormalRootExecutionTestTerminalV1::consume_once` | `callable_parameter_source/normal_root_execution/test_terminal.rs` | parser test owners |

`PreparedRawVmSourceExtractionV1` co-seals total-root discard and the
unselected Script-A sibling closure. Those two operations cannot fail or be
dropped independently. Only this product may extract the source-backed Raw
AST once. The no-import check moves after the authorized Raw extraction or is
answered from the canonical source surface; it no longer calls a generic
pre-route `disposition.ast()`.

Compatibility uses its own extraction issuer after exact source-authority
absence. It cannot fabricate `Ready`, enter canonical policy, or reuse the
source-backed Raw issuer.

## Finite state table

| State | Owns | Next | Effect |
| --- | --- | --- | ---: |
| `SurfaceReady` | complete parser surface | total issuer once | 0 |
| `RootReady(App)` | total App relation + surface | one typed route | 0 |
| `RootReady(ProgramRuntime)` | total runtime relation + surface | one typed route | 0 |
| `SourceAuthorityUnavailable` | exact cause | typed terminal/compat closure | 0 |
| `Incomplete` | exact missing relation | typed reject | 0 |
| `IntegrityInvalid` | foreign/duplicate/contradictory relation | typed reject | 0 |
| `MovedToNormalDefault` | transform-preserved owner | first Builder consumer | 0 |
| `MovedToCanonicalSourcePlan` | source-plan-bound owner | pure policy once | 0 |
| `PreparedRawVmSourceExtraction` | root discard + Script-A closure | one AST extraction | 0 |
| `Retained` | all required parser fields | scoped loan/test terminal | 0 |
| `AdmittedRootExecutionInput` | consumed role + exact syntax projection | Builder lifecycle | 0 before lifecycle |
| `RejectedBeforeEffect` | typed reason + owned sibling closure | terminal | 0 |

No `Option`, empty/default product, wildcard state merge, public parts tuple,
second source loan, or retry transition is admitted.

## Old-edge retirement manifest

Every edge below reaches zero in the same semantic C0:

```text
frontdoor discard_root_before_a                              1 -> 0
root-discard-required handoff constructor/assert             1 -> 0
frontdoor no-import disposition.ast()                        1 -> 0
CanonicalParserSourceHandoffV1::into_parts                   2 -> 0
Raw _script_input drop                                       1 -> 0
Raw NormalParserCallableSourceHandoffV1::into_ast             1 -> 0
PreparedNormalSourcePlanInputV1::from_parser_callable_source  1 -> 0
NormalSourcePlanClassifierV1::seal production                1 -> 0
classifier -> NormalSourceSurfaceInventoryV1::collect         1 -> 0
co_seal_script_source_input Raw caller                        1 -> 0
source-backed into_source_disposition generic edge            2 -> 0
source-backed into_normal_callable_program old arm             2 -> 0
pre-terminal ParserCallableSourceDispositionV1::ast            2 -> 0
silent normal_source_plan_surface `_` / product `..` drops     3 -> 0
unconsumed relation reaching SealedNormal* terminals           3 -> 0
source-backed VerifiedRawRootExpansionV1::from_program         3 -> 0
source-backed callable-catalog pointer pairing                 1 -> 0
canonical reject -> Raw/compatibility fallback                 0 -> 0
second source-plan loan                                        0 -> 0
```

Global post-terminal AST use is not claimed as zero. Only named typed
terminals may lend or extract syntax, and none may reclassify root role or
source-plan family.

## Tests and acceptance

New tests use separate files; do not grow the 742-line
`source_plan_input_tests.rs`, the 743-line `main_expansion.rs`, or the 788-line
semantic-package test module.

| Surface | Positive | Negative | Test file |
| --- | --- | --- | --- |
| total issuer | empty/executable/non-Main provider/non-static Main; App main/0, main/N, helpers, siblings | duplicate Main, missing main, foreign/missing/duplicate row | `normal_root_execution/tests.rs` |
| transform | exact unchanged source | add/remove/reorder/change, foreign invocation | `normal_root_execution_preservation_tests.rs` |
| policy | Script/Main0/CallableModule parity | mixed/unsupported/arity/coverage rejects | `parser_bound_policy_tests.rs` |
| normal/default | App and ProgramRuntime consumed before effect | second consume, raw scan, pointer/name pairing | builder `normal_root_execution/tests.rs` |
| Raw/compat | source-backed Ready one discard/extraction; compatibility one extraction | wrong route, source failure extraction, retry | `atomic_root_cutover_tests.rs` |
| retained/test | all fields moved and consumed | silent drop or second loan | `retained_tests.rs` + named parser tests |

The current frontdoor baseline is 21 passing and 8 known
`AppReadyRequiresNormalRootConsumer` failures. C0 must turn those eight green;
it may not reclassify them as baseline debt or route them through AST fixture
or compatibility fallback.

## Reusable guard and old-guard migration

One reusable lane guard is added:

```text
tools/checks/normal_root_execution_reference_route_guard.sh
```

It checks:

```text
surface issuer def/caller                         = 1/1
total issuer def/caller                           = 1/1
preservation issuer def/caller                    = 1/1
normal/default consumer caller                    = 1
canonical source-plan consumer caller             = 1
seal_parser_bound production caller               = 1
source-backed Raw discard/extraction caller       = 1
compatibility extraction caller                   = 1
retained test terminal prod/test caller            = 0/1
old inventory/parser-backed input callers         = 0
all named old extraction/discard/drop edges        = 0
unconsumed relation reaching SealedNormal*         = 0
post-terminal root/source-plan reclassification   = 0
fallback/retry                                    = 0
second source-plan loan                           = 0
all touched source/test files                     < 760
```

The same C0 updates or retires old guards that currently pin superseded
authority:

| Guard | C0 action |
| --- | --- |
| `frontend_normal_source_plan_surface_i0_a_guard.sh` | evolve from compiler-caller zero to exactly one named consumer |
| `frontend_main_app_entry_transport_i0_guard.sh` | retire with old narrow transport |
| `parser_normal_root_preservation_a_i0_guard.sh` | replace with total preservation checks |
| `frontend_main_app_entry_i0_guard.sh` | retire or restrict to an explicitly narrow non-authority projection |
| `script_direct_static_canonical_parser_source_handoff_guard.sh` | remove requirements for old generic handoff/input |
| `mir_root_app_mode_failfast_guard.sh` | keep only derived-mode integrity in C0; retire in R0 |

`docs/tools/check-scripts-index.md` records the reusable guard and removals.
Adding a new guard while old guards continue to require old authorities is a
hard failure.

## Bounded execution series

### T0 — `NORMAL-ROOT-EXECUTION-PRE-CUTOVER-SPLIT-T0`

BoxShape only:

```text
src/mir/builder/main_expansion.rs
  inline #[cfg(test)] module
    -> src/mir/builder/main_expansion_tests.rs
```

Contract:

- production prefix through line 456 is byte-identical;
- normalized inner test-body SHA-256 remains
  `b7f3e1f5aa3244458af9bfe4754bede30b5faf130c742800ef38562b437ff3dd`;
- eight test names and behavior remain unchanged;
- only a `#[path = "main_expansion_tests.rs"] mod tests;` attachment changes;
- no authority, caller, visibility, AST classifier, or production behavior
  changes.

If C0 would require editing another file already at or above 760 lines, stop
and add a separate behavior-neutral split before semantic work. Do not widen
T0 opportunistically.

### C0 — `NORMAL-ROOT-EXECUTION-ATOMIC-CUTOVER-C0`

One semantic commit, with no landed caller-zero issuer or parallel authority:

1. aggregate the existing surface under the one total execution issuer;
2. preserve the aggregate through the exact transform;
3. route normal/default, canonical, Raw, compatibility, retained, and tests
   through their named one-shot terminals;
4. consume normal/default before every Builder effect;
5. run pure parser-bound policy and replace source-backed owners in all three
   `SealedNormal*` terminals;
6. retire all listed old edges, old narrow root authority, and conflicting
   guard requirements;
7. add focused positive/negative tests and the reusable guard in the same
   commit.

### R0 — `NORMAL-ROOT-EXECUTION-BOOL-RETIREMENT-R0`

Immediately after C0:

- replace bool-shaped work-plan input with `AdmittedNormalRootExecutionModeV1`
  or the admitted aggregate;
- remove `MirBuilder::root_is_app_mode` if no physical projection needs it;
- otherwise rename and restrict it to a derived physical mode;
- update the stale `VerifiedRawRootExpansionV1` authority comment;
- make the raw classifier explicitly compatibility/fixture-only.

R0 issues no source meaning and may not reopen AST classification.

## Parked ledger

| Finding | Why non-blocking | Reopen trigger |
| --- | --- | --- |
| typed discard reason / broader `#[must_use]` cleanup | C0 has named affine terminals | an unguarded drop survives C0 |
| AST-only fixture retirement | fixture authority is separately typed | a production caller appears |
| builder barrel / physical shelf cleanup | no root semantic authority change | C0/R0 caller-zero cleanup is complete |
| source-plan scan fusion / compile-time optimization | no performance claim in this row | measurement names this owner as hot |
| Recipe/Join/MIR cleanup | receives only consumed typed products after C0 | post-terminal reclassification appears |

These findings are recorded but do not expand T0/C0/R0.

## Stop / NoSafeSlice

Do not begin or continue C0 if any of these is true:

- exact total, preservation, Raw, compatibility, retained, or Builder owner
  differs from this manifest without a new design review;
- existing surface issuer and a second surface issuer coexist;
- a product exit retains `_`, `..`, implicit drop, or generic pre-terminal
  `ast()` / `into_ast()` access;
- compatibility needs `Ready`, `Option`, empty, or default total state;
- a `SealedNormal*` product still owns parser-backed
  `PreparedNormalSourcePlanInputV1` or reclassifies from AST;
- the normal/default consumer runs after any Builder effect;
- Raw total-root discard and Script-A sibling closure can fail independently;
- current eight red tests are bypassed rather than closed;
- a semantic intermediate commit exposes a caller-zero or second authority;
- a touched file reaches 760 lines;
- fallback/retry, new language cohort, Recipe/Join/MIR, or physical cleanup is
  required to make the cutover pass.

## Manifest acceptance evidence

- local branch/HEAD at audit: `main` / `8fa33a555229`;
- worktree at audit: clean and equal to `hakorune/main`;
- independent worker audit completed after repeated long wait windows;
- parser source-surface focused tests: 6 passed;
- narrow preservation focused tests: 6 passed;
- normal callable source suite: 35 passed;
- frontdoor suite: 21 passed, 8 known C0 failures;
- three existing focused guards: passed;
- reusable C0 guard: intentionally not created before semantic C0.

## C0 selection evidence

```text
selected main base                            3cf1188b20d6
worktree / remote parity                      clean / exact
T0 production/test files                      459 / 284 lines
T0 production-prefix and test-body hashes     exact
12 open / 2 parked census                     unchanged
old edge and guard inventory                  unchanged
new or second source issuer                   0
unclassified product exit                     0
additional pre-split required                 0
```

This manifest closes the design question and now selects
`NORMAL-ROOT-EXECUTION-ATOMIC-CUTOVER-C0`. The implementation must remain one
unlanded semantic working set until every route, terminal, test, guard
migration, and old-edge zero in this card closes together.
