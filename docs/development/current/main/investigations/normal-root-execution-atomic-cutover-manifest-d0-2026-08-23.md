# Normal root execution atomic cutover manifest D0

Status: accepted design — C0 closed; R0 selected
Date: 2026-08-24
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

### C0 premise correction before policy implementation

The long read-only premise audit found one pre-policy omission in the
unlanded C0 implementation. This correction narrows the existing C0; it does
not add another issuer or another execution row.

- the same `ParserNormalSourcePlanSurfaceIssuerV1` must preserve a
  policy-total Box row for ordinary/non-static and static Box declarations;
  `Unsupported(Box)` may remain a policy outcome, but may not be the parser's
  lossy source representation;
- Box declaration mode/name plus the exact direct-callable relations and
  observed member coverage remain one same-invocation row. Names, ordinals,
  and paths are coverage/diagnostic data, never downstream pairing keys;
- a non-static `Main` remains a complete observed Main surface. Compiler
  policy, not the parser surface issuer or Builder, returns
  `MainMustBeStatic`;
- source-backed `SourceAuthorityUnavailable`, `Incomplete`, and
  `IntegrityInvalid` are typed terminal failures. Only an explicit outer
  `CompatibilityAbsence` route may enter compatibility extraction;
- normal/default, canonical, Raw, retained, and test exits each consume the
  root relation and Script-A sibling through a named affine terminal. `_`,
  `..`, and implicit product drop are not terminals;
- the parser-to-MIR source-plan boundary exposes an opaque owner and one
  one-shot scoped visitor. It does not re-export parser surface rows or expose
  a general `into_parts()` tuple.

If policy parity requires a second AST scan or a second Box/source issuer
after this correction, C0 returns to `NoSafeSlice`.

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

At C0 selection, the parent D0's earlier `open=6, parked=8` classification was
too narrow. Rows 7–9 and 11–13 still received or dropped an unconsumed parser
relation, or reclassified from `PreparedNormalSourcePlanInputV1`, so they were
C0 blockers.

| # | Entry | Pre-C0 state | C0 terminal |
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

The pre-C0 classification was:

```text
Exhausted(14)
CutoverBlockerOpen = 12
ParkedSealed = 2
Unclassified = 0
```

The C0 acceptance condition was `CutoverBlockerOpen = 0`. Rows 11–13 became
closed only after their source-backed input no longer owned
`PreparedNormalSourcePlanInputV1` and could not classify from AST.

## Pre-C0 owner/caller census

The already-landed source-surface owner was retained in place:

```text
ParserNormalSourcePlanSurfaceIssuerV1::issue_once
  definition = 1
  production caller = 1
```

The pre-C0 open exits were:

| Row | Pre-C0 owner / symbol | Pre-C0 caller evidence | Defect |
| ---: | --- | --- | --- |
| 1 | `ParsedProgramWithCallableParameterSourceV1::new` | direct 1 | total execution issuer absent |
| 2 | `ParserCallableSourceDispositionV1::into_normal_callable_program` | direct production callsites 2 | `normal_source_plan_surface: _` |
| 3 | `prepare_source_plan_request` | production 1, test 7 | enters whole-AST inventory |
| 4 | `prepare_raw_vm_handoff` | production 1, test 4 | drops Script-A sibling then `into_ast()` |
| 5 | `into_retained_source` | production 0, test 1 | `..` drops the surface |
| 6 | parser test helpers | 15 callsites in 9 files | no named terminal |

The pre-C0 source-backed normal/default lifecycle also performed three raw-root
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
| lifecycle facade | `PreparedNormalDefaultProgramRootV1::consume_source_backed_root_once` | `normal_default_program_root.rs` | `normal_default_root_catalog_lifecycle_tests.rs` |

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

The pre-C0 frontdoor baseline was 21 passing and 8 known
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
| `frontend_normal_source_plan_surface_i0_a_guard.sh` | retired into the reusable total-route guard; compiler caller is now exactly one named consumer |
| `frontend_main_app_entry_transport_i0_guard.sh` | retired with old narrow transport |
| `parser_normal_root_preservation_a_i0_guard.sh` | retired; total preservation checks moved into the reusable total-route guard |
| `frontend_main_app_entry_i0_guard.sh` | retired with the narrow Main/App authority |
| `script_direct_static_canonical_parser_source_handoff_guard.sh` | retired; root/front-door invariants moved into the reusable total-route guard and composite invariants remain in the composite-source guard |
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

This selection evidence closed the design question and selected
`NORMAL-ROOT-EXECUTION-ATOMIC-CUTOVER-C0`. The implementation remained one
unlanded semantic working set until every route, terminal, test, guard
migration, and old-edge zero in this card closed together.

## C0 closeout evidence — 2026-08-24

The manifest boundary is exhausted:

```text
Exhausted(14)
CutoverBlockerOpen = 0
ParkedSealed = 2
Unclassified = 0
```

Rows 1–9 and 11–13 now terminate through the one total relation and their
named affine consumers. Rows 10 and 14 remain the two declared boundary-outside
owners: the typed AST-only fixture and Raw runtime after authorized extraction.
Neither can re-enter the C0 source-authority boundary. The production chain is:

```text
Parser SourceSurface
  -> ParserNormalRootExecutionIssuerV1
  -> exact transform preservation or one route-specific typed terminal
  -> parser-bound policy / NormalRootExecutionConsumerV1
  -> first Builder effect or authorized Raw extraction
```

The old narrow Main/App authority, generic parser handoff, source-backed AST
reclassification, silent sibling drops, and the five superseded guards are at
caller zero. Canonical-to-Raw fallback/retry and a second source-plan loan
remain zero. The reusable route guard pins the issuer/consumer census, old-edge
retirement, focused evidence, and the `< 760` source limit; the largest touched
Rust source is `src/parser/mod.rs` at 757 lines.

Acceptance results:

```text
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo test --profile quick --lib normal_callable_semantic_package \
  --message-format short
  -> 39 passed; 0 failed; 7303 filtered out

direct focused filters:
  atomic_root_cutover_tests                 5 passed
  normal_root_execution                    19 passed
  normal_root_execution_preservation        7 passed
  normal_source_plan_surface                7 passed
  parser_bound_policy                       9 passed
  normal_callable_program_source           36 passed
  normal_callable_transform                 7 passed
  normal_file_vm_frontdoor                 33 passed
  retained                                 27 passed
  canonical_script_source_admission         4 passed
  static_box_source                         6 passed
  script_source_rows                        3 passed
  script_source_authority                   3 passed

normal_root_execution_reference_route_guard.sh          PASS
normal_callable_source_carrier_cutover_guard.sh          PASS
frontend_normal_source_plan_seed_retention_i0_guard.sh   PASS
mir_root_app_mode_failfast_guard.sh                      PASS
current_state_pointer_guard.sh                           PASS
git diff --check                                         PASS
```

The bounded warning cleanup reduced the same C0 build from 1088 to 1063
warnings and left zero new warning signatures against the immediately
preceding stored C0 fingerprint. Remaining warnings are not acceptance
evidence: the derived `root_execution_mode` dead transport is explicitly owned
by R0, and broader pre-existing warning debt stays outside this atomic commit.

C0 is closed. The next selected row is
`NORMAL-ROOT-EXECUTION-BOOL-RETIREMENT-R0`; it may remove only derived bool/mode
transport and stale compatibility comments/classifier presentation, and must
not issue source meaning or reopen AST classification.

## R0 warning cleanup evidence — 2026-08-24

The first behavior-neutral R0 warning slice is landed as three separate commits:

```text
cdcde58d12  refactor: retire unused root mode transport
d1895b6ded  refactor: remove unused parser source accessors
a0699f977e  refactor: align semantic package warning visibility
```

The slice removed only caller-zero mode/accessor transport, stale parser source
accessors, and package-internal visibility mismatches. It did not remove typed
reject payloads or introduce a new source authority. The final focused package
test remains green:

```text
cargo test --profile quick --lib normal_callable_semantic_package
  -> 39 passed; 0 failed; 7303 filtered out
```

The same test fingerprint moved from 1063 to 1044 warnings. The remaining
warnings are pre-existing broad debt (including intentionally retained typed
reject payloads); they are not claimed as cleared by this R0 slice.

The next bounded R0 slice is landed as `17f3757c36`:

```text
17f3757c36  refactor: retire stale root warning transport
```

It removes the caller-zero semantic lineage accessor, the duplicate mode field
from the consumed source receipt, the unconstructed deferred root-lowering
variant, and two unused production imports. The prepared receipt still owns
the test-terminal mode observation, which is derived from the preserved parser
root relation; no source authority or production route was added.

Acceptance for this slice:

```text
normal_root_execution                 -> 19 passed; 0 failed
normal_callable_semantic_package      -> 39 passed; 0 failed
normal_default_root_catalog_lifecycle -> current 6/10; parent 6/10
program_root_work_plan                -> current 2/9; parent 2/9
```

The two partial focused suites reproduce their existing baseline failures
(`mir/script-neutral-window/work-plan-edge` and
`mir/instance-constructor-source/cohort-missing`); they are classified as
known baseline debt, not current-change failures. The same test fingerprint
moved from 1044 to 1040 warnings, with zero warning records in all four
touched files. The five reusable guards and `git diff --check` are green.

The following import-only R0 slice is landed as `b96eeb93ad`:

```text
b96eeb93ad  refactor: prune raw root physical imports
```

It removes six compiler-reported unused-import groups from the Raw root
physical drain/finalization/root-batch modules. The remaining warnings in
those modules are private-interface and dead-code observations and remain a
separate bounded family. `raw_root_environment_install` is green with 6
passed tests, all five reusable guards remain green, and the test fingerprint
moved from 1040 to 1034 warnings.

The next import-only R0 slice is landed as `4ddea83a17`:

```text
4ddea83a17  refactor: prune script physical exit imports
```

It retires the caller-zero `CompletedScriptBodyCompletionV1` and detached
direct-static helper reexports from the Script physical-exit module. The
direct-static kernel and its local tests remain intact; `script_physical_exit`
passes all 9 focused tests, all five reusable guards remain green, and the
test fingerprint moved from 1034 to 1032 warnings with zero warning records in
the touched module.

The following import-only R0 slice is landed as `2a95dd9463`:

```text
2a95dd9463  refactor: prune calls module imports
```

It removes three caller-zero import groups from `calls/mod.rs` while retaining
the one `emit_standard_value_terminal_raw_v1` interface that still has a
builder caller. The focused `call_argument_descent` suite passes 5 tests, all
five reusable guards remain green, and the test fingerprint moved from 1032
to 1029 warnings with zero warning records in `calls/mod.rs`. The broad
`--lib calls` suite remains baseline-red in both current and parent builds;
its route/emitter/constructor failures are outside this import-only slice.

The latest bounded import-only R0 slice is landed as `3f6b42e7d7`:

```text
3f6b42e7d7  refactor: prune normal transaction exports
```

It retires eight caller-zero normal-module transaction reexports (including
helper-prefix, callable-main, main-transaction, and script-transaction
products) without changing the child-module owners. The focused transaction
tests pass 4 tests, all five reusable guards remain green, and the test
fingerprint moved from 1023 to 1022 warnings with zero warning records in the
touched module.

The following import-only R0 slice is landed as `9f082bc47c`:

```text
9f082bc47c  refactor: prune collector exports
```

It removes four caller-zero collector drain/batch export groups while keeping
the collector-owned prepared products and commit-facing receipt. The focused
`module_draft_collector` suite passes 32 tests, all five reusable guards remain
green, and the test fingerprint moved from 1022 to 1018 warnings. The only
remaining warning in that module is the separate unused lifecycle method
`seal_after_exact_signature_preflight`.

The next import-only R0 slice is landed as `d336750521`:

```text
d336750521  refactor: prune direct static exports
```

It retires three caller-zero direct-static issue/recipe/proof reexport groups
while leaving their child-module authorities and focused tests untouched. The
`normal_script_direct_static_join_handoff` suite passes 10 tests, all five
reusable guards remain green, and the test fingerprint moved from 1018 to 1015
warnings. The two remaining warnings in that module are separate unused
accessors (`source_owner`/`result_site`/`parent_relations` and `row`), not
imports.

The next import-only R0 slice is landed as `8e904d8cb8`:

```text
8e904d8cb8  refactor: prune direct static ledger exports
```

It retires the two caller-zero direct-static claim-ledger reexports from
`normal_script_semantic_lowering_state.rs`; the child ledger remains the sole
owner and its internal module-path uses are unchanged. The focused
`direct_static_claim_ledger` suite passes 7 tests, all five reusable guards
remain green, and the test fingerprint moved from 1015 to 1014 warnings.
The touched file has no remaining unused-import warning records; its separate
dead-code rows are outside this import-only slice.

The following import-only R0 slice is landed as `3a2422a8c9`:

```text
3a2422a8c9  refactor: prune callable prefix import
```

It removes the caller-zero `NormalCallableHandoffStageV1` import from
`normal_module_transaction/callable_draft_prefix.rs`; the helper-draft
transaction and its child authorities are unchanged. The focused normal-module
transaction suite passes 20 tests, all five reusable guards remain green, and
the test fingerprint moved from 1014 to 1013 warnings.

The following caller-zero import slice is landed as `47f2e77cec`:

```text
47f2e77cec  refactor: prune semantic invocation imports
```

It removes only unused test/legacy imports from
`normal_script_semantic_source_tests.rs`,
`module_lowering_invocation_legacy_term.rs`, and the invocation test support
module. The focused `module_lowering_invocation::tests` suite passes 7 tests;
all five reusable guards remain green, and the test fingerprint moved from
1013 to 1009 warnings. A broad exploratory
`normal_script_semantic_source` filter remains baseline-red (17/67 passed)
because its existing fixture parser rejects `grouped_assignment` at
`normal_script_binding_rebind_tests.rs:102`; that failure is outside this
import-only diff and is not counted as current-change evidence.

The next builder-barrel import slice is landed as `053e23aa8e`:

```text
053e23aa8e  refactor: prune builder barrel imports
```

It removes 13 compiler-reported caller-zero reexports from `builder.rs` while
leaving the remaining cross-module interfaces intact. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 1009 to 996 warnings.

The following resolved-lowering import slice is landed as `61565ffc7c`:

```text
61565ffc7c  refactor: prune resolved lowering exports
```

It retires six caller-zero export groups from
`resolved_lowering/mod.rs`; the remaining lowering owners and selected routes
are unchanged. The focused `normal_root_execution` suite passes 19 tests, all
five reusable guards remain green, and the test fingerprint moved from 996 to
991 warnings.

The following if-recipe contract import slice is landed as `d41c782782`:

```text
d41c782782  refactor: prune if recipe exports
```

It retires five caller-zero reexport groups from
`if_recipe_contract/mod.rs`; recipe verification, normalization, and physical
input owners remain unchanged. The focused `if_recipe_contract` suite passes
10 tests, all five reusable guards remain green, and the test fingerprint
moved from 991 to 986 warnings.

The following loop-route policy import slice is landed as `54cc08c097`:

```text
54cc08c097  refactor: prune loop route exports
```

It retires six caller-zero export groups from `loop_route_policy/mod.rs`; the
policy evaluator and route owners are unchanged. The focused loop-route suite
ran 80 tests with 79 passing; its one failure is the existing
`policy_evidence.rs:75` Candidate assertion outside this import-only diff.
All five reusable guards remain green, and the test fingerprint moved from 986
to 980 warnings.

The following statement-export import slice is landed as `8f450bbfb9`:

```text
8f450bbfb9  refactor: prune statement exports
```

It retires three caller-zero export groups from `builder/stmts/mod.rs`; the
statement descent and completion owners remain unchanged. The focused stmts
suite ran 98 tests with 97 passing; its one failure is the existing
`variable_assignment_completion.rs:119` `No current basic block` rejection
outside this import-only diff. All five reusable guards remain green, and the
test fingerprint moved from 980 to 977 warnings.

The following structural-facts import slice is landed as `23b9b3877d`:

```text
23b9b3877d  refactor: prune structural facts exports
```

It retires three caller-zero export groups from
`loop_structural_facts/mod.rs`; structural observation and source projection
owners remain unchanged. The focused loop-structural-facts suite passes 27
tests, all five reusable guards remain green, and the test fingerprint moved
from 977 to 974 warnings.

The following loop-recipe contract import slice is landed as `8fdf548783`:

```text
8fdf548783  refactor: prune loop recipe exports
```

It retires four caller-zero export groups from `loop_recipe_contract/mod.rs`;
recipe and source-bound owners remain unchanged. The focused loop-recipe suite
ran 156 tests with 155 passing; its one failure is the existing
`source_bound_core_tests.rs:181` typed-reject assertion outside this
import-only diff. All five reusable guards remain green, and the test
fingerprint moved from 974 to 970 warnings.

The following common-v2 session import slice is landed as `8e391725cd`:

```text
8e391725cd  refactor: prune common v2 session exports
```

It retires five caller-zero export groups from
`resolved_lowering/common_v2_session/mod.rs`; common-v2 issuers and selected
physical session owners remain unchanged. The focused `normal_root_execution`
suite passes 19 tests, all five reusable guards remain green, and the test
fingerprint moved from 970 to 965 warnings.

The following control-flow/resolved-lowering import slice is landed as
`1a2db35a2d`:

```text
1a2db35a2d  refactor: prune control flow imports
```

It removes 11 compiler-reported caller-zero import records across nine
control-flow and resolved-lowering support modules; no source or physical
authority changes. The focused `normal_root_execution` suite passes 19 tests,
all five reusable guards remain green, and the test fingerprint moved from 965
to 954 warnings.

The following completion-test import slice is landed as `8e7edb391e`:

```text
8e7edb391e  refactor: prune completion test imports
```

It removes caller-zero imports from the completion-consumption, draft-seal,
and shared completion-test support modules; the completion authorities and
tests are unchanged. The focused completion filter passes 21 tests, all five
reusable guards remain green, and the test fingerprint moved from 954 to 940
warnings. An initial compile exposed three imports that were actually used;
they were restored before the green rerun and are not part of the final diff.

The following callable-contract import slice is landed as `57bdbf4971`:

```text
57bdbf4971  refactor: prune callable contract imports
```

It removes four caller-zero import groups across the callable parameter,
result-representation, and semantic-batch facades; contract owners and proof
issuers are unchanged. The focused `normal_root_execution` suite passes 19
tests, all five reusable guards remain green, and the test fingerprint moved
from 940 to 936 warnings.

The following builder import-shim slice is landed as `1c47cc8f92`:

```text
1c47cc8f92  refactor: prune builder import shims
```

It removes six caller-zero imports or internal reexports from the canonical
completion, invocation collection, module shell, cataloged admission, raw
ledger, and recursive child-lowering support modules; source, recipe, and
physical authorities are unchanged. The focused `normal_root_execution`
suite passes 19 tests, all five reusable guards remain green, and the test
fingerprint moved from 936 to 930 warnings.

The following reference-lane feature import slice is landed as `868dafe161`:

```text
868dafe161  refactor: gate reference imports by feature
```

It gates four imports that are only used by the `vm-reference` execution or
test branches; the default reference front doors and the feature-enabled
owners are unchanged. The focused `normal_root_execution` suite passes 19
tests, all five reusable guards remain green, and the test fingerprint moved
from 930 to 926 warnings.

The following JoinIR reexport slice is landed as `ee668f0f95`:

```text
ee668f0f95  refactor: prune unused joinir reexports
```

It removes two caller-zero JoinIR reexports from the merge and live ordered
terminality facades; the coordinator and logical-product owners remain in
place. The focused `normal_root_execution` suite passes 19 tests, all five
reusable guards remain green, and the test fingerprint moved from 926 to 924
warnings.

The following resolved-lowering import slice is landed as `bfba3ab775`:

```text
bfba3ab775  refactor: prune resolved lowering imports
```

It removes five compiler-reported caller-zero imports from the generic
carrier bridge test, loop operation physicalizer, read-emitter test, and
nested-predicate adapter test; lowering behavior and test fixtures are
unchanged. The focused `normal_root_execution` suite passes 19 tests, all
five reusable guards remain green, and the test fingerprint moved from 924
to 919 warnings.

The following loop-physicalizer facade import slice is landed as
`cbf4bec31f`:

```text
cbf4bec31f  refactor: tighten loop physicalizer imports
```

It removes two caller-zero facade imports and changes one test-support
reexport to a private parent import so child test modules retain their
existing names without widening the facade. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 919 to 916 warnings.

The following compiler import-shim slice is landed as `d40c601363`:

```text
d40c601363  refactor: prune compiler import shims
```

It removes six caller-zero imports or bounded internal reexports from the
DirectAccum, Dynamic full-body co-seal, and Generic G0 projection facades;
source/Recipe issuers and physical owners are unchanged. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 916 to 910 warnings.

The following root-compiler import slice is landed as `737db8c777`:

```text
737db8c777  refactor: prune root compiler imports
```

It removes six caller-zero imports or bounded reexports from the Generic G0
cohort, normal source plan, Raw root, canonical drain fixture, postprocess
fixture, and root manifest package modules; ownership and compatibility
boundaries are unchanged. The focused `normal_root_execution` suite passes
19 tests, all five reusable guards remain green, and the test fingerprint
moved from 910 to 904 warnings.

The following contract-facade import slice is landed as `d96e52761e`:

```text
d96e52761e  refactor: prune contract facade imports
```

It removes twelve caller-zero imports or bounded reexports across Dynamic,
If, JoinSig/S6C, JoinIR lowering, resolved-value, source-call, and Raw
builder facades; all source and Recipe authorities remain unchanged. The
focused `normal_root_execution` suite passes 19 tests, all five reusable
guards remain green, and the test fingerprint moved from 904 to 892 warnings.

The following MIR-root facade import slice is landed as `38b3f1641d`:

```text
38b3f1641d  refactor: prune mir root warning exports
```

It removes caller-zero root exports, gates VM-reference-only report/profile
exports behind `vm-reference`, and leaves the allowlisted core MIR facade
unchanged. The focused `normal_root_execution` suite passes 19 tests, all
five reusable guards plus MIR root facade/import-hygiene guards remain green,
and the test fingerprint moved from 892 to 888 warnings with
`unused_imports=0`.

The following behavior-neutral callable-test mutability slice is landed as
`ebc9fd6350`:

```text
ebc9fd6350  refactor: remove needless callable test mutability
```

It removes six unnecessary `mut` bindings from `Complete(forests)` in five
normal-callable test modules. The source, Recipe, and physical owners are
unchanged. The focused `normal_root_execution` suite passes 19 tests, all
five reusable guards remain green, and the test fingerprint moved from 888
to 882 warnings (`unused_imports=0`, `unused_mut=13`).

The following behavior-neutral mutability cleanup slice is landed as
`a8eff51aa6`:

```text
a8eff51aa6  refactor: remove needless mutability bindings
```

It removes the remaining thirteen unnecessary `mut` bindings across raw-root,
builder, lowering, compiler, and resolved-value test/implementation helpers.
No source, Recipe, physical, or visibility authority changes. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 882 to 869 warnings
(`unused_imports=0`, `unused_mut=0`).

The following behavior-neutral unused-binding cleanup slice is landed as
`fd838c4745`:

```text
fd838c4745  refactor: silence unused variable bindings
```

It preserves the existing admission, reject, and lowering calls while marking
fourteen intentionally unused local/field bindings explicitly. No source,
Recipe, physical, or visibility authority changes. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 869 to 855 warnings
(`unused_imports=0`, `unused_mut=0`, `unused_variables=0`).

The final non-structural lint slice in this pass is landed as `716ec2a2fb`:

```text
716ec2a2fb  test: remove unreachable destination arm
```

The destination enum currently has one authoritative variant, so the test's
unreachable fallback arm was removed without changing the issued destination
or any production route. The focused `normal_root_execution` suite passes 19
tests, all five reusable guards remain green, and the test fingerprint moved
from 855 to 854 warnings (`unused_imports=0`, `unused_mut=0`,
`unused_variables=0`, `unreachable_patterns=0`).

The following parser visibility cleanup slice is landed as `f038089c28`:

```text
f038089c28  refactor: narrow parser source visibility
```

The parser source-session fields and postpass entrypoints are consumed only by
the parser module tree, so their crate-wide visibility was narrowed to private
without changing source sealing or compatibility entrypoints. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 854 to 847 warnings
(`private_interfaces=114`; non-structural lint categories remain zero).

The following disconnected invocation-owner visibility slice is landed as
`eea766b5c1`:

```text
eea766b5c1  refactor: narrow invocation physical parts visibility
```

`InvocationPhysicalStateV1::into_parts` is consumed only inside the Builder
module tree, so its return types no longer cross the wider MIR boundary. The
focused `normal_root_execution` suite passes 19 tests, all five reusable guards
remain green, and the test fingerprint moved from 847 to 845 warnings
(`private_interfaces=112`; non-structural lint categories remain zero).

The following common-V2 session boundary cleanup slice is landed as
`3b448fecf7`:

```text
3b448fecf7  refactor: narrow common v2 session visibility
```

All four segment/continuation allocation helpers are called only within the
`resolved_lowering` owner tree, matching the visibility of their receipts and
callback scopes. The focused `normal_root_execution` suite passes 19 tests, all
five reusable guards remain green, and the test fingerprint moved from 845 to
838 warnings (`private_interfaces=106`, `private_bounds=6`; non-structural
lint categories remain zero).

The following callable-main helper visibility slice is landed as
`9248c82faa`:

```text
9248c82faa  refactor: narrow callable main helper visibility
```

The source/physical getters and evidence decomposition are consumed only by
the Builder transaction tree; the rejection enum remains at its compiler
boundary. The focused `normal_root_execution` suite passes 19 tests, all five
reusable guards remain green, and the test fingerprint moved from 838 to 834
warnings (`private_interfaces=102`, `private_bounds=6`; non-structural lint
categories remain zero).

The following source-bound compiler visibility slice is landed as
`fa4b65ea09`:

```text
fa4b65ea09  refactor: narrow source bound compiler visibility
```

Manifest projection and source-binding rejection accessors are consumed only
inside the compiler module tree, so their visibility now matches the private
manifest/error products. The focused `normal_root_execution` suite passes 19
tests, all five reusable guards remain green, and the test fingerprint moved
from 834 to 831 warnings (`private_interfaces=99`, `private_bounds=6`; all
non-structural lint categories remain zero).

The following dynamic source-target visibility slice is landed as
`e9c7386848`:

```text
e9c7386848  refactor: narrow dynamic source target visibility
```

The dynamic-member issue/reject products and their three source-call entry
points are consumed only inside `crate::mir`; their visibility and re-export
boundary now match that owner without changing source relations or catalog
behavior. The focused `normal_root_execution` suite passes 19 tests, all five
reusable guards remain green, and the test fingerprint moved from 831 to 827
warnings (`private_interfaces=95`, `private_bounds=6`; non-structural lint
categories remain zero).

The following receiver-accessor visibility slice is landed as `db0b90ce08`:

```text
db0b90ce08  refactor: narrow receiver accessor visibility
```

The source-call receiver projection and semantic owner-profile accessor are
consumed only inside `crate::mir`, so their method boundaries now match the
existing receiver-policy products. The focused `normal_root_execution` suite
passes 19 tests, all five reusable guards remain green, and the test
fingerprint moved from 827 to 824 warnings (`private_interfaces=92`,
`private_bounds=6`; non-structural lint categories remain zero). The remaining
owner-profile enum-field warning is intentionally left for a separate census,
because narrowing that enum would widen the affected field/accessor surface.

The following qualified-receiver boundary slice is landed as `9e4710a08a`:

```text
9e4710a08a  refactor: narrow qualified receiver boundaries
```

The canonical function-view receiver accessor and complete-inventory lexical
sealer are consumed only inside `crate::mir`; their method visibility now
matches the existing receiver-policy and shadow products. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 824 to 822 warnings
(`private_interfaces=91`, `private_bounds=5`; non-structural lint categories
remain zero).

The following Builder callback/test-surface visibility slice is landed as
`1918b24106`:

```text
1918b24106  refactor: narrow builder callback visibility
```

The selected-Dynamic test helpers and Generic G0 preflight are file-local,
the invocation callback stays inside the Builder tree, and the explicit-extern
port now shares that same Builder boundary. No lowering or publication route
changed. The focused `normal_root_execution` suite passes 19 tests, all five
reusable guards remain green, and the test fingerprint moved from 822 to 815
warnings (`private_interfaces=89`, `private_bounds=0`; non-structural lint
categories remain zero).

The following Raw source-binding visibility slice is landed as `0f011cbf52`:

```text
0f011cbf52  refactor: narrow raw source binding visibility
```

The Raw binding error, identity issuer entry, and reject accessor are consumed
only inside `crate::mir::compiler`; their boundaries now match the compiler
source-bound products without changing the Raw ingress surface. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 815 to 813 warnings
(`private_interfaces=87`, `private_bounds=0`; non-structural lint categories
remain zero).

The following Script-recipe rejection accessor slice is landed as
`473625d903`:

```text
473625d903  refactor: narrow script recipe rejection accessors
```

The Script recipe reject accessors expose only the existing `crate::mir`
projection error and remain inside the compiler-owned source-plan route. No
Recipe classification or physical entry behavior changed. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 813 to 811 warnings
(`private_interfaces=85`, `private_bounds=0`; non-structural lint categories
remain zero).

The following resolved-region field visibility slice is landed as
`f779341360`:

```text
f779341360  refactor: narrow resolved region field visibility
```

The owner-core if/loop region indexes are consumed only inside
`crate::mir::resolved_semantics`, so their field boundaries now match the
existing region-index owners without changing verification or lookup behavior.
The focused `normal_root_execution` suite passes 19 tests, all five reusable
guards remain green, and the test fingerprint moved from 811 to 809 warnings
(`private_interfaces=83`, `private_bounds=0`; non-structural lint categories
remain zero).

The following Raw-drain owner visibility slice is landed as `2fdbf4b508`:

```text
2fdbf4b508  refactor: align raw drain owner visibility
```

The prepared Raw drain aggregate and its nested Script/App owners now share
the compiler-owned visibility boundary, and `prepare_drain` exposes the same
boundary to its compiler consumers. No drain, rejection, or publication
behavior changed. The focused `normal_root_execution` suite passes 19 tests,
all five reusable guards remain green, and the test fingerprint moved from 809
to 807 warnings (`private_interfaces=81`, `private_bounds=0`; non-structural
lint categories remain zero).

The following Raw callable environment-handoff visibility slice is landed as
`40da90c2f1`:

```text
40da90c2f1  refactor: narrow raw callable environment handoff
```

The consuming `into_environment_parts` handoff is used only by the compiler's
declaration-access route, so its method boundary now matches the existing
compiler-private parts product. No callable-main or declaration-access
behavior changed. The focused `normal_root_execution` suite passes 19 tests,
all five reusable guards remain green, and the test fingerprint moved from 807
to 806 warnings (`private_interfaces=80`, `private_bounds=0`; non-structural
lint categories remain zero).

The following Script physical-entry rejection visibility slice is landed as
`26a6bacdb1`:

```text
26a6bacdb1  refactor: narrow script entry rejection access
```

The Script physical-entry rejection cause accessor is consumed only by the
compiler source-plan/dispatch route, so its boundary now matches that
compiler-owned handoff. No Script physical-entry or rejection behavior
changed. The focused `normal_root_execution` suite passes 19 tests, all five
reusable guards remain green, and the test fingerprint moved from 806 to 805
warnings (`private_interfaces=79`, `private_bounds=0`; non-structural lint
categories remain zero).

The following Raw-root physical parts visibility slice is landed as
`ec2801c134`:

```text
ec2801c134  refactor: narrow raw root physical parts
```

The Raw-root post-body physical `into_parts` handoff is consumed only by the
same module's root-batch terminal, so its boundary now matches the
module-private ledger product. No root-batch or physical-drain behavior
changed. The focused `normal_root_execution` suite passes 19 tests, all five
reusable guards remain green, and the test fingerprint moved from 805 to 804
warnings (`private_interfaces=78`, `private_bounds=0`; non-structural lint
categories remain zero).

The following Raw body-parts handoff visibility slice is landed as
`95758c9050`:

```text
95758c9050  refactor: narrow raw body parts handoff
```

The completed Raw root-body `into_parts` handoff is consumed by the parent
environment-install module and its own batch-input adapter, so its boundary
now matches the existing environment-install owner. No body completion or
root-batch behavior changed. The focused `normal_root_execution` suite passes
19 tests, all five reusable guards remain green, and the test fingerprint
moved from 804 to 803 warnings (`private_interfaces=77`, `private_bounds=0`;
non-structural lint categories remain zero).

The following canonical-publication handoff visibility slice is landed as
`bb65fd5f53`:

```text
bb65fd5f53  refactor: narrow canonical publication handoff
```

The canonical publication preparation method is consumed only within the
compiler dispatch route, so its boundary now matches the prepared publication
owner. No target, membership, pairing, or commit behavior changed. The
focused `normal_root_execution` suite passes 19 tests, all five reusable
guards remain green, and the test fingerprint moved from 803 to 802 warnings
(`private_interfaces=76`, `private_bounds=0`; non-structural lint categories
remain zero).

The following Main source-view visibility slice is landed as `978c9ce0c1`:

```text
978c9ce0c1  refactor: narrow main source view access
```

The verified Main source unit's exact-function view accessor is consumed only
inside the compiler-owned normal source-plan tree, so its boundary now matches
the existing `pub(super)` view. No source verification or lookup behavior
changed. The focused `normal_root_execution` suite passes 19 tests, all five
reusable guards remain green, and the test fingerprint moved from 802 to 801
warnings (`private_interfaces=75`, `private_bounds=0`; non-structural lint
categories remain zero).

The following Raw-ledger error boundary slice is landed as `f135329711`:

```text
f135329711  refactor: align raw ledger error visibility
```

The Raw expansion role and receipt-ledger error are shared by the compiler
facing Raw physical rejection products, so both now use the existing
`crate::mir` boundary. No reservation, receipt, abort, or root-batch behavior
changed. The focused `normal_root_execution` suite passes 19 tests, all five
reusable guards remain green, and the test fingerprint moved from 801 to 792
warnings (`private_interfaces=66`, `private_bounds=0`; non-structural lint
categories remain zero).

The following normal transaction/collector error-boundary slice is landed as
`bea17bd394`:

```text
bea17bd394  refactor: align normal transaction error visibility
```

The shared draft key, collector admission/brand errors, normal transaction
schema, source draft, typed-definition, and physical-thunk products now match
the existing `crate::mir` rejection boundary. No schema validation,
transaction, or physical-thunk behavior changed. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 792 to 778 warnings
(`private_interfaces=52`, `private_bounds=0`; non-structural lint categories
remain zero).

The following Common V2 segment-handoff visibility slice is landed as
`e9afd400a3`:

```text
e9afd400a3  refactor: narrow common v2 segment handoffs
```

The Common V2 session callbacks that consume segment brands, prepared segment
receipts, and shared segment scopes now use the existing
`crate::mir::builder::resolved_lowering` boundary. No lowering, receipt
consumption, or physical emission behavior changed. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 778 to 766 warnings
(`private_interfaces=40`, `private_bounds=0`; non-structural lint categories
remain zero).

The following capability-witness visibility slice is landed as `bf3d80cf4c`:

```text
bf3d80cf4c  refactor: align capability witness visibility
```

The recursive capability-install receipt and acyclic capability-absence
witness already cross the compiler-facing `crate::mir` aggregate, so their
struct boundaries now match that owner. No capability selection or install
behavior changed. The focused `normal_root_execution` suite passes 19 tests,
all five reusable guards remain green, and the test fingerprint moved from 766
to 764 warnings (`private_interfaces=38`, `private_bounds=0`; non-structural
lint categories remain zero).

The following Raw-drain error-boundary visibility slice is landed as
`41ed4112d6`:

```text
41ed4112d6  refactor: align raw drain error visibility
```

The Raw manifest and collector drain errors are carried by the existing
compiler-facing `RawPhysicalDrainErrorV1`, so their enum boundaries now match
that `crate::mir` owner. No drain validation, rejection, or publication
behavior changed. The focused `normal_root_execution` suite passes 19 tests,
all five reusable guards remain green, and the test fingerprint moved from 764
to 762 warnings (`private_interfaces=36`, `private_bounds=0`; non-structural
lint categories remain zero).

The following loop-physicalizer visibility slice is landed as `2fbddaafd6`:

```text
2fbddaafd6  refactor: align loop physicalizer visibility
```

The loop topology/segment reject products and operation-target receipt now
share the existing `crate::mir::builder::resolved_lowering` boundary used by
the allocator and service façade. No topology, block allocation, or operation
target behavior changed. The focused `normal_root_execution` suite passes 19
tests, all five reusable guards remain green, and the test fingerprint moved
from 762 to 758 warnings (`private_interfaces=32`, `private_bounds=0`;
non-structural lint categories remain zero).

The following canonical drain-error visibility slice is landed as
`21c34c6d34`:

```text
21c34c6d34  refactor: align canonical drain error visibility
```

The canonical collector and manifest errors are carried by the existing
compiler-facing drain rejection products, so their enum boundaries now match
the `crate::mir` owner. No canonical drain validation, rejection, or
publication behavior changed. The focused `normal_root_execution` suite
passes 19 tests, all five reusable guards remain green, and the test
fingerprint moved from 758 to 756 warnings (`private_interfaces=30`,
`private_bounds=0`; non-structural lint categories remain zero).

The following resolved-lowering receipt visibility slice is landed as
`18c04de50b`:

```text
18c04de50b  refactor: align resolved lowering receipt visibility
```

The callable collector entry, canonical If-control consumption ledger,
completed draft, and draft-seal stage now match the existing compiler/Builder
owner boundaries that consume them. No callable collection, SSA control,
draft sealing, or rejection behavior changed. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 756 to 752 warnings
(`private_interfaces=26`, `private_bounds=0`; non-structural lint categories
remain zero).

The following Dynamic co-seal visibility slice is landed as `fd2789bca2`:

```text
fd2789bca2  refactor: align dynamic coseal visibility
```

The Dynamic source/Recipe reject enums now match the existing coseal envelope
owner, while test-only route/target/cleanup accessors remain module-private.
No source coverage, call relation, cleanup, or exit co-seal behavior changed.
The focused `normal_root_execution` suite passes 19 tests, all five reusable
guards remain green, and the test fingerprint moved from 752 to 747 warnings
(`private_interfaces=21`, `private_bounds=0`; non-structural lint categories
remain zero).

The following canonical-core dispatch facade visibility slice is landed as
`8b4d2cd395`:

```text
8b4d2cd395  refactor: narrow canonical core dispatch facade
```

The retained canonical dispatch owner, internal error enum, callable reject
product, and published-entry method now stay inside the `crate::mir` owner.
Runner tests consume the existing Callable stage projection rather than the
internal error payload. No family selection, rejection stage, publication, or
execution behavior changed. The focused `normal_root_execution` suite passes
19 tests, all five reusable guards remain green, and the test fingerprint
moved from 747 to 741 warnings (`private_interfaces=15`, `private_bounds=0`;
non-structural lint categories remain zero).

The following compiler/control projection visibility slice is landed as
`919936b0f1`:

```text
919936b0f1  refactor: align compiler control projection visibility
```

The Generic G0 control accessor, instance source-loan/error boundary, and If
coverage claim now match their existing compiler or resolved-control owners.
No entry-control, source-plan, or coverage behavior changed. The focused
`normal_root_execution` suite passes 19 tests, all five reusable guards remain
green, and the test fingerprint moved from 741 to 738 warnings
(`private_interfaces=12`, `private_bounds=0`; non-structural lint categories
remain zero).

The following text-residence handoff visibility slice is landed as
`ed8d783368`:

```text
ed8d783368  refactor: narrow text residence raw handoff
```

The runtime-owned `into_raw_parts` handoff is consumed only inside the
runtime residence owner, so its method now matches the `crate::runtime`
boundary. No root publication, lease, or rollback behavior changed. The
focused `normal_root_execution` suite passes 19 tests, all five reusable guards
remain green, and the test fingerprint moved from 738 to 737 warnings
(`private_interfaces=11`, `private_bounds=0`; non-structural lint categories
remain zero).

The following runner feature-ownership slice is landed as `19697fbd93`:

```text
19697fbd93  refactor: align runner feature ownership
```

VM-reference invocation/report types and front-door handoffs now compile only
inside the `vm-reference` owner, selected Dynamic bundle helpers now compile
only inside the `llvm-boundary` owner, and two unreferenced LLVM AST compile
helpers were removed. No runner selection, source reading, publication, or
execution behavior changed. The focused `normal_root_execution` suite passes
19 tests, and the test fingerprint moved from 737 to 718 warnings
(`private_interfaces=11`, `dead_code=707`; all non-structural lint categories
remain zero).

The following unused-vocabulary cleanup is landed as `969bf64e66`:

```text
969bf64e66  refactor: remove unused parser diagnostics
```

Six unreferenced Array-write diagnostic tag constants and the uncalled broad
`parse_postpass` parser entry were removed. No Array validation, parser
postpass, or source-to-Recipe behavior changed. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 718 to 711 warnings (`private_interfaces=11`, `dead_code=700`; all
non-structural lint categories remain zero).

The following script-source row accessor cleanup is landed as `57803513ea`:

```text
57803513ea  refactor: trim script source row accessors
```

Unconsumed ordinal/kind/type accessors and the unused disposition witness
projection were removed from the AST-free Script row model; the parser seal
token remains present as an explicitly unused `_seal` field. No row issuance,
source admission, or parser ownership behavior changed. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 711 to 705 warnings (`private_interfaces=11`, `dead_code=694`; all
non-structural lint categories remain zero).

The following parser composite/accessor cleanup is landed as `58d26e404e`:

```text
58d26e404e  refactor: trim parser composite accessors
```

Unused parser-cohort identity accessors were removed, and the composite
provider's retained identity/source-site evidence is now explicitly `_`-named
until a named consumer exists. No cohort disposition, source identity check,
or composite issuance behavior changed. The focused `normal_root_execution`
suite passes 19 tests, and the test fingerprint moved from 705 to 702 warnings
(`private_interfaces=11`, `dead_code=691`; all non-structural lint categories
remain zero).

The following unreferenced parser-facade cleanup is landed as `deff1fd24d`:

```text
deff1fd24d  refactor: remove unreferenced parser facades
```

Unused TextScan row/slot accessors, program-slot and gate projections, and an
unconnected composite-source loan wrapper were removed. No provider admission,
parser placement, gate selection, or composite source behavior changed. The
focused `normal_root_execution` suite passes 19 tests, and the test fingerprint
moved from 702 to 697 warnings (`private_interfaces=11`, `dead_code=686`; all
non-structural lint categories remain zero).

The following module-source-row accessor cleanup is landed as `a35c0c00a2`:

```text
a35c0c00a2  refactor: trim module source row accessors
```

Unconsumed module-row evidence accessors were removed and retained invocation,
placement, source-site, and callable-identity fields were explicitly `_`-named.
The typed source-authority disposition states remain intact. No module-row
issuance, source-seal validation, or parser ownership behavior changed. The
focused `normal_root_execution` suite passes 19 tests, and the test fingerprint
moved from 697 to 690 warnings (`private_interfaces=11`, `dead_code=679`; all
non-structural lint categories remain zero).

The following unused callable-source finalizer cleanup is landed as
`fa1bc72658`:

```text
fa1bc72658  refactor: remove unused callable source finalizer
```

The unreferenced catalog finalizer, its parser wrapper, and their dedicated
unused error arm were removed. The active normal disposition finalizer and the
typed `SelectedBuildGateUnsupported` disposition remain unchanged. No source
session admission, build-gate rejection, catalog issuance, or parser ownership
behavior changed. The focused `normal_root_execution` suite passes 19 tests,
and the test fingerprint moved from 690 to 687 warnings
(`private_interfaces=11`, `dead_code=676`; all non-structural lint categories
remain zero).

The following parser-source accessor cleanup is landed as `92f158f2ba`:

```text
92f158f2ba  refactor: trim unused parser source accessors
```

The unreferenced parser-invocation brand comparison and source-authority
readiness accessor were removed. The used witness comparison, composite
readiness check, and all typed source-authority terminal states remain intact.
No parser identity, source-row admission, disposition, or ownership behavior
changed. The focused `normal_root_execution` suite passes 19 tests, and the
test fingerprint moved from 687 to 685 warnings
(`private_interfaces=11`, `dead_code=674`; all non-structural lint categories
remain zero).

The following parser handoff accessor cleanup is landed as `50526dc168`:

```text
50526dc168  refactor: trim unused parser handoff accessors
```

Unused declaration-kind staticness, initial-source-row, resolver-invocation,
delegate-placement, and release-provenance accessors were removed. The
delegate inventory and release parser-provenance evidence remain stored under
explicit `_` fields until a named consumer exists. No parser handoff,
source-row, delegate relation, release-source, or ownership behavior changed.
The focused `normal_root_execution` suite passes 19 tests, and the test
fingerprint moved from 685 to 679 warnings
(`private_interfaces=11`, `dead_code=668`; all non-structural lint categories
remain zero).

The following static source-seal accessor cleanup is landed as `f81297108d`:

```text
f81297108d  refactor: trim static source seal accessors
```

The unreferenced static-parent seal accessors were removed, and unsupported
member sites plus the static seal's box/method evidence remain stored under
explicit `_` fields. Typed static-parent dispositions and their payloads are
unchanged. No static source issuance, member coverage, source-seal validation,
or parser ownership behavior changed. The focused `normal_root_execution`
suite passes 19 tests, and the test fingerprint moved from 679 to 676 warnings
(`private_interfaces=11`, `dead_code=665`; all non-structural lint categories
remain zero).

The following parser source-facade cleanup is landed as `df0b954931`:

```text
df0b954931  refactor: trim parser source facades
```

The delegate host's unused field view, unused build-gate path constructors and
segments accessor, and two unused prepared-source facades were removed. The
source path, delegate relations, typed source seals, and all rejection
dispositions remain intact. No delegate coverage, gate-path issuance, or
parser ownership behavior changed. The focused `normal_root_execution` suite
passes 19 tests, and the test fingerprint moved from 676 to 672 warnings
(`private_interfaces=11`, `dead_code=661`; all non-structural lint categories
remain zero).

The following parser evidence-retention cleanup is landed as `61b1ab68fc`:

```text
61b1ab68fc  refactor: retain unused parser evidence explicitly
```

Unused compatibility-row placement and pre-prune selection-receipt reads were
made explicit `_` fields while retaining their source evidence for a future
named consumer. No compatibility disposition, gate receipt, postpass coverage,
or parser ownership behavior changed. The focused `normal_root_execution`
suite passes 19 tests, and the test fingerprint moved from 672 to 670 warnings
(`private_interfaces=11`, `dead_code=659`; all non-structural lint categories
remain zero).

The following Builder receipt-accessor cleanup is landed as `3c4408c311`:

```text
3c4408c311  refactor: trim unused builder receipt accessors
```

Unused Brand projection owner, Compare proof, and publication receipt accessors
were removed; their owner/proof/receipt data remains retained for existing
validation and future named consumers. No Brand relation validation, Compare
append, publication quiescence, or Builder ownership behavior changed. The
focused `normal_root_execution` suite passes 19 tests, and the test fingerprint
moved from 670 to 666 warnings (`private_interfaces=11`, `dead_code=655`; all
non-structural lint categories remain zero).

The following Builder init-helper cleanup is landed as `9ead4d091b`:

```text
9ead4d091b  refactor: remove unused builder init helpers
```

The unreferenced instruction-block snapshot, predecessor/jump capture, and
duplicate closure-intern helper were removed. The active instruction snapshot
and module-lowering closure owner remain unchanged. No Builder control-flow,
metadata, or ownership behavior changed. The focused `normal_root_execution`
suite passes 19 tests, and the test fingerprint moved from 666 to 664 warnings
(`private_interfaces=11`, `dead_code=653`; all non-structural lint categories
remain zero).

The following callable-source accessor cleanup is landed as `d700cab8a5`:

```text
d700cab8a5  refactor: trim unused callable source accessors
```

The unused catalog-callable getter and instance-constructor ticket comparator
were removed, while the dynamic-local initializer remains retained as
`_initializer` evidence after its unused getter was removed. No callable source
validation, constructor demand issuance, or builder ownership behavior changed.
The focused `normal_root_execution` suite passes 19 tests, and the test
fingerprint moved from 664 to 661 warnings (`private_interfaces=11`,
`dead_code=650`; all non-structural lint categories remain zero).

The following Dynamic Loop source-accessor cleanup is landed as `036fde3d3d`:

```text
036fde3d3d  refactor: trim dynamic loop source accessors
```

Unused prepared-entry/source-coverage and operation/read accessors were
removed. The source coverage, read-site, operation, and binding-class evidence
remains retained explicitly in `_...` fields where no named consumer exists.
No Dynamic Loop source validation, handoff, operation relation, or lowering
behavior changed. The focused `normal_root_execution` suite passes 19 tests,
and the test fingerprint moved from 661 to 657 warnings
(`private_interfaces=11`, `dead_code=646`; all non-structural lint categories
remain zero).

The following collection-literal facade cleanup is landed as `40b9b1350d`:

```text
40b9b1350d  refactor: remove unused collection literal facades
```

The unreferenced non-port array/map convenience wrappers were removed; the
port-aware literal owners used by production and tests remain unchanged. No
array/map lowering, recursive child-port, or collection instruction behavior
changed. The focused `normal_root_execution` suite passes 19 tests, and the
test fingerprint moved from 657 to 656 warnings
(`private_interfaces=11`, `dead_code=645`; all non-structural lint categories
remain zero).

The following unused builder-wrapper retirement is landed as `9503ff23bb`:

```text
9503ff23bb  refactor: remove unused builder wrapper routes
```

The uncalled raw fastmem region wrapper and runtime instance-prefix wrapper
were removed; the port-aware fastmem and lifecycle owners remain the only
callable routes. No fastmem region, instance declaration, recursive child-port,
or lifecycle behavior changed. The focused `normal_root_execution` suite
passes 19 tests, and the test fingerprint moved from 656 to 654 warnings
(`private_interfaces=11`, `dead_code=643`; all non-structural lint categories
remain zero).

The following callable-lowering accessor cleanup is landed as `0a18f88c84`:

```text
0a18f88c84  refactor: trim unused callable lowering accessors
```

The unreferenced brand-disposition and Dynamic-origin test accessors were
removed. Brand projection ownership, dynamic-origin tracking, value
materialization, and final consumption checks remain unchanged. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 654 to 653 warnings (`private_interfaces=11`, `dead_code=642`; all
non-structural lint categories remain zero).

The following unused type-operation helper cleanup is landed as `abf2752d4a`:

```text
abf2752d4a  refactor: remove unused typeop helper
```

The private calls-module `is_typeop_method` adapter and its now-unused imports
were removed; the source type-operation policy classifier remains the active
owner. No call routing, type-operation policy, or argument handling changed.
The focused `normal_root_execution` suite passes 19 tests, and the test
fingerprint moved from 653 to 652 warnings
(`private_interfaces=11`, `dead_code=641`; all non-structural lint categories
remain zero).

The following call-session test-helper cleanup is landed as `7dcf4952a9`:

```text
7dcf4952a9  refactor: trim unused call test helpers
```

The unused outer payload-session capture probe and an unused integer fixture
were removed. The inner parent-capture assertion used by the session terminal
test remains intact, as does the production session restoration path. The
focused `normal_root_execution` suite passes 19 tests, and the test fingerprint
moved from 652 to 650 warnings (`private_interfaces=11`, `dead_code=639`; all
non-structural lint categories remain zero).

The following canonical draft-session facade cleanup is landed as `15be0caaa4`:

```text
15be0caaa4  refactor: remove unused draft session facade
```

The uncalled draft-capture method and its Builder wrapper were removed. The
active `run`, payload-bearing capture, and draft-seal session routes retain
their existing restoration and publication boundaries. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 650 to 648 warnings (`private_interfaces=11`, `dead_code=637`; all
non-structural lint categories remain zero).

The following prepared-session terminal cleanup is landed as `45f1eaa790`:

```text
45f1eaa790  refactor: trim unused prepared session commit
```

The uncalled `commit` alias was removed; `commit_projected` and the shared
restore terminal remain the sole prepared-close path. No function-session
publication, payload completion, or parent restoration behavior changed. The
focused `normal_root_execution` suite passes 19 tests, and the test fingerprint
moved from 648 to 647 warnings (`private_interfaces=11`, `dead_code=636`; all
non-structural lint categories remain zero).

The following method-call observer cleanup is landed as `89803fa919`:

```text
89803fa919  refactor: trim unused method call observer
```

The uncalled `AssociatedMethodCallArgumentsV1` observation adapter and its
unused import were removed; argument descent and terminal-port ownership remain
unchanged. The focused `normal_root_execution` suite passes 19 tests, and the
test fingerprint moved from 647 to 646 warnings
(`private_interfaces=11`, `dead_code=635`; all non-structural lint categories
remain zero).

The following global-terminal facade cleanup is landed as `4f64bcc7fe`:

```text
4f64bcc7fe  refactor: remove unused global terminal facade
```

The caller-free raw global-value terminal wrapper was removed; the prepared
lookup and receipt terminals remain the active owners. No static/global call
emission, destination allocation, or result publication behavior changed. The
focused `normal_root_execution` suite passes 19 tests, and the test fingerprint
moved from 646 to 645 warnings (`private_interfaces=11`, `dead_code=634`; all
non-structural lint categories remain zero).

The following duplicate static-terminal wrapper cleanup is landed as
`958e574f1e`:

```text
958e574f1e  refactor: remove duplicate static terminal wrapper
```

The uncalled inherent static-terminal adapter was removed; the existing
`StaticMethodCallCompletionV1` trait implementation remains the sole active
completion route. No static-call emission or terminal ownership behavior
changed. The focused `normal_root_execution` suite passes 19 tests, and the
test fingerprint moved from 645 to 644 warnings
(`private_interfaces=11`, `dead_code=633`; all non-structural lint categories
remain zero).

The following unified-emitter alias cleanup is landed as `7bf4a1a2a1`:

```text
7bf4a1a2a1  refactor: remove unused unified emitter alias
```

The caller-free lookup-less map-replay alias was removed; the shared
lookup/map-replay emitter remains the sole implementation. No call emission,
map-write replay, or result publication behavior changed. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 644 to 643 warnings (`private_interfaces=11`, `dead_code=632`; all
non-structural lint categories remain zero).

The following GenericLoop source-facts/view cleanup is landed as
`ab777f1dcf`:

```text
ab777f1dcf  refactor: trim unused generic loop view accessors
```

The receipt's unread condition/body evidence is retained explicitly, while
caller-free semantic-view accessors for loop site, generic facts, route
selection, and selected location are removed. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 643 to 640 warnings (`private_interfaces=11`, `dead_code=629`; all
non-structural lint categories remain zero). No source-facts claim, recipe
selection, or lowering behavior changed.

The follow-up test-only structural-view cleanup is landed as `c9224cbd04`:

```text
c9224cbd04  refactor: trim unused loop structural view field
```

The callback view no longer carries its unobserved selected-location field or
accessor; the source receipt and semantic view retain their typed selection
evidence. The direct `normal_root_execution` binary passes 19 tests, and the
test fingerprint moved from 640 to 638 warnings
(`private_interfaces=11`, `dead_code=627`; all non-structural lint categories
remain zero).

The following callable-module invocation-facade cleanup is landed as
`0eb8cb7389`:

```text
0eb8cb7389  refactor: trim callable module invocation facades
```

Caller-free `source()` accessors were removed from the prepared and rejected
collector invocation facades. The rejected facade still retains its verified
module source as explicit `_source` evidence, while collector/error and
publication routes remain unchanged. The focused `normal_root_execution`
suite passes 19 tests, and the test fingerprint moved from 638 to 635 warnings
(`private_interfaces=11`, `dead_code=624`; all non-structural lint categories
remain zero).

The following instance-constructor source-cohort cleanup is landed as
`94bdf18079`:

```text
94bdf18079  refactor: trim unused constructor cohort accessor
```

The cohort keeps its parser invocation witness as `_invocation` evidence, but
the caller-free `invocation_witness()` accessor is removed. Constructor source
row validation and admission ownership are unchanged. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 635 to 633 warnings (`private_interfaces=11`, `dead_code=622`; all
non-structural lint categories remain zero).

The following raw brand-arity evidence cleanup is landed as `6e54cfcfeb`:

```text
6e54cfcfeb  refactor: retain brand arity source evidence
```

The arity-mismatch branch retains its exact-source relation as
`_exact_source`, while the error terminal continues to use only the arity
fact and no source fallback. The focused `normal_root_execution` suite passes
19 tests, and the test fingerprint moved from 633 to 632 warnings
(`private_interfaces=11`, `dead_code=621`; all non-structural lint categories
remain zero).

The following Raw root-exit facade cleanup is landed as `71b45d262e`:

```text
71b45d262e  refactor: trim raw root exit facade evidence
```

The AppVoid disposition retains its discarded-tail relation as
`_discarded_tail`, and the uncalled witness `brand()` adapter is removed;
brand validation and route/Return checks remain the existing owners. The
focused `normal_root_execution` suite passes 19 tests, and the test
fingerprint moved from 632 to 630 warnings
(`private_interfaces=11`, `dead_code=619`; all non-structural lint categories
remain zero).

The following Raw postprocess evidence cleanup is landed as `37714c1e34`:

```text
37714c1e34  refactor: align raw postprocess feature evidence
```

Runtime-input snapshots remain explicit `_runtime_inputs` evidence, and the
VM decode/entry-target helpers are compiled only with their existing
`vm-reference` consumer feature. Raw postprocess routing and publication
ownership are unchanged. The focused `normal_root_execution` suite passes 19
tests, and the test fingerprint moved from 630 to 627 warnings
(`private_interfaces=11`, `dead_code=616`; all non-structural lint categories
remain zero).

The following rejected Raw postprocess facade cleanup is landed as
`1bb310ebd2`:

```text
1bb310ebd2  refactor: trim rejected postprocess accessors
```

The rejected receipt keeps its typed owner, stage, error, and verification
payloads for one-shot `discard()`, while removing four caller-zero inspection
adapters. The focused `normal_root_execution` suite passes 19 tests, and the
test fingerprint moved from 627 to 626 warnings
(`private_interfaces=11`, `dead_code=615`; all non-structural lint categories
remain zero).

The following DraftSeal projection-facade cleanup is landed as `a50ca34379`:

```text
a50ca34379  refactor: trim draft seal projection facades
```

The disconnected `project_exit` helper and rejection `error/discard`
adapters are removed; the typed projection error and move-only rejection
object remain owned by the existing DraftSeal flow. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 626 to 624 warnings (`private_interfaces=11`, `dead_code=613`; all
non-structural lint categories remain zero).

The following OpenDraftSeal probe cleanup is landed as `b3c33e0a06`:

```text
b3c33e0a06  refactor: trim draft seal owner probes
```

Caller-zero `discard`, immutable builder, and ready-completion inspection
probes are removed from the open owner; the mutable test builder probe and all
prepare/commit/rejection restoration routes remain. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 624 to 623 warnings (`private_interfaces=11`, `dead_code=612`; all
non-structural lint categories remain zero).

The following detached DraftSeal probe cleanup is landed as `933aae284f`:

```text
933aae284f  refactor: trim unused draft seal probes
```

The caller-free projection type-facts wrapper, intermediate projection probe,
and draft-plan exit probe are removed. The lookup-aware type-facts route,
typed projection receipts, and owner prepare/commit flow remain unchanged.
The focused `normal_root_execution` suite passes 19 tests, and the test
fingerprint moved from 623 to 620 warnings
(`private_interfaces=11`, `dead_code=609`; all non-structural lint categories
remain zero).

The following composite-partition probe cleanup is landed as `174efbdbcd`:

```text
174efbdbcd  refactor: trim composite partition probes
```

The unused test-only disposition adapters and caller-zero invocation accessor
are removed; the parser invocation witness remains retained as `_invocation`
evidence, and composite source/row admission is unchanged. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 620 to 617 warnings
(`private_interfaces=11`, `dead_code=606`; all non-structural lint categories
remain zero).

The following static-lookup error-detail cleanup is landed as `74475c2762`:

```text
74475c2762  refactor: retain static lookup error details
```

The five tuple error payloads become named `_detail` fields, preserving their
diagnostic text while removing unread tuple-field warnings. Lookup admission,
typed rejection, and publication-owner routing remain unchanged. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 617 to 612 warnings
(`private_interfaces=11`, `dead_code=601`; all non-structural lint categories
remain zero).

The following neutral-window reject-detail cleanup is landed as `922e0e2b6d`:

```text
922e0e2b6d  refactor: retain neutral window reject details
```

The six tuple reject payloads become named underscore fields, retaining parser,
composite, transfer, constructor, catalog, and window-seal diagnostics while
removing unread tuple-field warnings. Neutral source admission and its
downstream split remain unchanged. The focused `normal_root_execution` suite
passes 19 tests, and the test fingerprint moved from 612 to 606 warnings
(`private_interfaces=11`, `dead_code=595`; all non-structural lint categories
remain zero).

The following composite disposition-detail cleanup is landed as `d3ba45f198`:

```text
d3ba45f198  refactor: retain composite reject details
```

The three parser reject payloads become named `_reason` fields, preserving
source-authority, incomplete, and integrity diagnostics while removing unread
tuple-field warnings. Composite admission and the Ready/Outside acceptance
shapes remain unchanged. The focused `normal_root_execution` suite passes 19
tests, and the test fingerprint moved from 606 to 603 warnings
(`private_interfaces=11`, `dead_code=592`; all non-structural lint categories
remain zero).

The following If-recipe reject-detail cleanup is landed as `e22d2122e2`:

```text
e22d2122e2  refactor: retain if recipe reject details
```

The five producer reject payloads and two cardinality counts become named
underscore fields, retaining mapper/join/input/correspondence errors and the
observed counts. If-recipe production and admission ownership are unchanged;
the intentionally unconstructed future correspondence variant remains. The
focused `normal_root_execution` suite passes 19 tests, and the test fingerprint
moved from 603 to 596 warnings (`private_interfaces=11`, `dead_code=585`; all
non-structural lint categories remain zero).

The following completion-consumption probe cleanup is landed as `ac7e5255d7`:

```text
ac7e5255d7  refactor: trim completion consumption probes
```

Four caller-zero helpers are removed: explicit-unit inspection, test-only unit
and implicit-void constructors, and an unused consumer-side implicit-void
probe. The semantic Completion `is_implicit_void` route and physical claim
consumption remain unchanged. The focused `normal_root_execution` suite passes
19 tests, and the test fingerprint moved from 596 to 592 warnings
(`private_interfaces=11`, `dead_code=581`; all non-structural lint categories
remain zero).

The following Raw publication probe cleanup is landed as `a8445d6357`:

```text
a8445d6357  refactor: trim raw publication probes
```

The VM-reference-only invocation-brand helper is feature-gated, the unused
rejection error adapter is removed, and publication/rejection payloads retain
their receipt and owner/error evidence under underscore fields. Raw publication
and compatibility erasure remain unchanged. The focused `normal_root_execution`
suite passes 19 tests, and the test fingerprint moved from 592 to 588 warnings
(`private_interfaces=11`, `dead_code=577`; all non-structural lint categories
remain zero).

The residual warning boundary is explicit after these behavior-neutral slices:
ten `private_interfaces` warnings belong to the
public `MirInstruction` pinned-Text/checked-callout fields, and one belongs to
the semantic owner root profile's `ReceiverPolicy` field. Clearing those
eleven requires a deliberate public MIR/semantic API authority decision, so
they remain deferred rather than being hidden with an allow or a synthetic
visibility. The remaining 577 `dead_code` warnings are existing disconnected
scaffolding and are not mass-deleted in this R0 lane.

The following Raw postprocess rejection-evidence cleanup is landed as
`6a512394c7`:

```text
6a512394c7  refactor: retain raw postprocess rejection evidence
```

The optimizer, contract-refresh, carrier-parity, and final-verification
payloads remain typed and owned by the rejection receipt, while unread tuple
fields and rejection-owner fields become named underscore evidence. Raw
postprocess stage ordering, failure mapping, and explicit discard behavior are
unchanged. The focused `normal_root_execution` suite passes 19 tests, and the
test fingerprint moved from 588 to 583 warnings (`private_interfaces=11`,
`dead_code=572`; all non-structural lint categories remain zero).

The updated residual warning boundary is explicit: the eleven
`private_interfaces` warnings remain deferred pending the deliberate public
MIR/semantic API authority decision described above, while the remaining 572
`dead_code` warnings are existing disconnected scaffolding and are not
mass-deleted in this R0 lane.

The following Raw external-commit rejection-probe cleanup is landed as
`2e38cad945`:

```text
2e38cad945  refactor: trim raw external commit rejection probe
```

The rejected owner and typed error remain retained for explicit discard, while
the unused error accessor is removed and the fields become named underscore
evidence. The two unconstructed typed failure variants remain as the bounded
future rejection vocabulary; external-commit validation and publication
handoff are unchanged. The focused `normal_root_execution` suite passes 19
tests, and the test fingerprint moved from 583 to 581 warnings
(`private_interfaces=11`, `dead_code=570`; all non-structural lint categories
remain zero).

The updated residual warning boundary is explicit: the eleven
`private_interfaces` warnings remain deferred pending the deliberate public
MIR/semantic API authority decision described above, while the remaining 570
`dead_code` warnings are existing disconnected scaffolding and are not
mass-deleted in this R0 lane.

The following Raw publication-adapter evidence cleanup is landed as
`e2a1498690`:

```text
e2a1498690  refactor: retain raw publication adapter evidence
```

The compatibility envelope still holds the complete route, witness, parity,
schedule, progress, publication, and verification projection evidence until
its sole result-erasure terminal. Its unread nested fields are named underscore
evidence rather than deleted. Raw verification projection and compatibility
conversion are unchanged. The focused `normal_root_execution` suite passes 19
tests, and the test fingerprint moved from 581 to 578 warnings
(`private_interfaces=11`, `dead_code=567`; all non-structural lint categories
remain zero).

The updated residual warning boundary is explicit: the eleven
`private_interfaces` warnings remain deferred pending the deliberate public
MIR/semantic API authority decision described above, while the remaining 567
`dead_code` warnings are existing disconnected scaffolding and are not
mass-deleted in this R0 lane.

The following Raw callable-main terminal error-evidence cleanup is landed as
`01bfb86665`:

```text
01bfb86665  refactor: retain raw callable main error evidence
```

The Request, Reservation, Child, Ledger, and Abort payloads remain typed and
are still carried through the physical rejection product; only unread tuple
fields become named underscore evidence. Callable-main reservation, abort,
ledger completion, and upper-layer error routing are unchanged. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 578 to 573 warnings (`private_interfaces=11`, `dead_code=562`; all
non-structural lint categories remain zero).

The updated residual warning boundary is explicit: the eleven
`private_interfaces` warnings remain deferred pending the deliberate public
MIR/semantic API authority decision described above, while the remaining 562
`dead_code` warnings are existing disconnected scaffolding and are not
mass-deleted in this R0 lane.

The following Raw child-terminal error-evidence cleanup is landed as
`c1f8ed429c`:

```text
c1f8ed429c  refactor: retain raw child error evidence
```

The Request, Reservation, Child, Ledger, and Abort payloads remain typed and
are still returned through the static-helper terminal; only unread tuple
fields become named underscore evidence. Child reservation, abort, ledger
completion, and coarse abort-reason mapping are unchanged. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 573 to 568 warnings (`private_interfaces=11`, `dead_code=557`; all
non-structural lint categories remain zero).

The updated residual warning boundary is explicit: the eleven
`private_interfaces` warnings remain deferred pending the deliberate public
MIR/semantic API authority decision described above, while the remaining 557
`dead_code` warnings are existing disconnected scaffolding and are not
mass-deleted in this R0 lane.

The following Raw drain rejection-probe cleanup is landed as `5a86d7f34d`:

```text
5a86d7f34d  refactor: trim raw drain rejection probes
```

The complete rejected owner split and typed drain error remain retained for
the error accessor and explicit discard. Unread owner fields become named
underscore evidence, and the decode-plan/entry-target helpers are gated to
the existing `vm-reference` lane that consumes them. Drain validation,
manifest projection, and physical handoff are unchanged. The focused
`normal_root_execution` suite passes 19 tests, and the test fingerprint moved
from 568 to 564 warnings (`private_interfaces=11`, `dead_code=553`; all
non-structural lint categories remain zero).

The updated residual warning boundary is explicit: the eleven
`private_interfaces` warnings remain deferred pending the deliberate public
MIR/semantic API authority decision described above, while the remaining 553
`dead_code` warnings are existing disconnected scaffolding and are not
mass-deleted in this R0 lane.

The following Raw finalization rejection-probe cleanup is landed as
`24ce822e19`:

```text
24ce822e19  refactor: trim raw finalization rejection probe
```

The physical rejection still retains its drained owner for the upper-layer
error route, and `error()` remains the sole evidence accessor. The unused
physical-level `discard` probe is removed; the existing unconstructed typed
failure vocabulary is preserved. Finalization validation and handoff are
unchanged. The focused `normal_root_execution` suite passes 19 tests, and the
test fingerprint moved from 564 to 562 warnings (`private_interfaces=11`,
`dead_code=551`; all non-structural lint categories remain zero).

The updated residual warning boundary is explicit: the eleven
`private_interfaces` warnings remain deferred pending the deliberate public
MIR/semantic API authority decision described above, while the remaining 551
`dead_code` warnings are existing disconnected scaffolding and are not
mass-deleted in this R0 lane.

The following Raw postprocess owner-progress probe cleanup is landed as
`1b13730447`:

```text
1b13730447  refactor: trim raw postprocess progress probe
```

The owner-side `progress()` accessor was caller-zero and is removed. The
postprocessed physical progress proof and stage evidence remain unchanged,
with no authority or handoff change. The focused `normal_root_execution`
suite passes 19 tests, and the test fingerprint moved from 562 to 561
warnings (`private_interfaces=11`, `dead_code=550`; all non-structural lint
categories remain zero).

The residual boundary remains explicit: the eleven `private_interfaces`
warnings await the deliberate public MIR/semantic API authority decision, and
the remaining 550 `dead_code` warnings are existing disconnected scaffolding
that is not mass-deleted in this R0 lane.

The following Raw root-batch rejection-probe cleanup is landed as
`7a6286064c`:

```text
7a6286064c  refactor: trim raw root batch rejection probes
```

The rejected batch owner still retains the session, physical carrier, draft,
completion, exit witness, token, and owner product for the existing discard
boundary. They are named underscore evidence because no accessor consumes them;
the root-batch validation and handoff remain unchanged.

The following Raw/source-site and canonical Script-exit probe cleanup is
landed as `a683f74723`:

```text
a683f74723  refactor: trim raw and script exit probes
```

The unused Raw physical preserving wrapper and source-site span accessor are
removed, while the span evidence, publication token, and static-child role
remain retained. Script entry error payloads are named underscore evidence,
and only caller-zero completion accessors are removed; the sole Script exit
source/physical projection remains. The focused `normal_root_execution`
suite passes 19 tests, and the test fingerprint moved from 561 to 548
warnings (`private_interfaces=11`, `dead_code=537`; all non-structural lint
categories remain zero).

The residual boundary remains explicit: the eleven `private_interfaces`
warnings await the deliberate public MIR/semantic API authority decision, and
the remaining 537 `dead_code` warnings are existing disconnected scaffolding
that is not mass-deleted in this R0 lane.

The following S6C semantic-child rejection-evidence cleanup is landed as
`e43411c0c7`:

```text
e43411c0c7  refactor: retain s6c child rejection evidence
```

The child issue keeps every typed cause and detail as named underscore
evidence; no rejection is flattened or reclassified. The unused child
identity/role accessors are removed while the retained identity/role fields
remain in the verified product. The unconstructed `ResultMismatch` vocabulary
is preserved. The focused `normal_root_execution` suite passes 19 tests, and
the test fingerprint moved from 548 to 537 warnings (`private_interfaces=11`,
`dead_code=526`; all non-structural lint categories remain zero).

The residual boundary remains explicit: the eleven `private_interfaces`
warnings await the deliberate public MIR/semantic API authority decision, and
the remaining 526 `dead_code` warnings are existing disconnected scaffolding
that is not mass-deleted in this R0 lane.

The following normal-callable semantic-package rejection-evidence cleanup is
landed as `b5e3067fdb`:

```text
b5e3067fdb  refactor: retain semantic package rejection evidence
```

The package-level issue keeps each typed source, batch, mapping, contract,
physical, S6C, and Dynamic cause as named underscore evidence. Dynamic batch
slot and issue data remain retained, and the existing resolver/physical-header
tests now match the named variants. No issue is flattened, reclassified, or
removed. The focused `normal_root_execution` suite passes 19 tests, and the
test fingerprint moved from 537 to 522 warnings (`private_interfaces=11`,
`dead_code=511`; all non-structural lint categories remain zero).

The residual boundary remains explicit: the eleven `private_interfaces`
warnings await the deliberate public MIR/semantic API authority decision, and
the remaining 511 `dead_code` warnings are existing disconnected scaffolding
that is not mass-deleted in this R0 lane.

The following callable physical-header rejection-evidence cleanup is landed
as `e4d1e3347e`:

```text
e4d1e3347e  refactor: retain physical header rejection evidence
```

The completion-seed/header boundary keeps batch slots, unsupported annotation
names, and completion causes as named underscore evidence. No source or
completion rejection is flattened, and header row issuance remains unchanged.
The focused `normal_root_execution` suite passes 19 tests, and the test
fingerprint moved from 522 to 515 warnings (`private_interfaces=11`,
`dead_code=504`; all non-structural lint categories remain zero).

The residual boundary remains explicit: the eleven `private_interfaces`
warnings await the deliberate public MIR/semantic API authority decision, and
the remaining 504 `dead_code` warnings are existing disconnected scaffolding
that is not mass-deleted in this R0 lane.

The following callable physical-signature rejection-evidence cleanup is landed
as `dc72a9495e`:

```text
dc72a9495e  refactor: trim physical signature probes
```

The verified physical-signature row retains role and receiver evidence, and
the cohort retains its brand, as named underscore fields. Caller-zero row and
cohort accessors are removed; the typed batch-loan issue remains named
underscore evidence. Signature row issuance, physical shape, and rejection
authority are unchanged. The focused `normal_root_execution` suite passes 19
tests, and the test fingerprint moved from 522 to 510 warnings
(`private_interfaces=11`, `dead_code=499`; all non-structural lint categories
remain zero).

The residual boundary remains explicit: the eleven `private_interfaces`
warnings await the deliberate public MIR/semantic API authority decision, and
the remaining 499 `dead_code` warnings are existing disconnected scaffolding
that is not mass-deleted in this R0 lane.
