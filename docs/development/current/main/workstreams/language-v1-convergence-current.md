---
Status: Parked taskboard; no current execution authority
Date: 2026-10-12
Scope: Hakorune language v1 semantic convergence before selfhost migration.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md
  - docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md
  - docs/reference/language/EBNF.md
  - docs/development/current/main/investigations/docs-spec-adapter-artifact-hygiene-task-map-2026-10-08.md
  - docs/reference/language/grammar-contract.md#separator-list-and-precedence-convergence-queue
  - docs/reference/language/types.md
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/design/object-handle-box-identity-contract-ssot.md
---

# Language v1 Convergence Current Taskboard

## Parking notice

`MIRBUILDER-INPLACE-REPLACEMENT0` currently owns execution order.

Language semantics and this taskboard remain valid, but no row below may
preempt the current production responsibility replacement. Resume only when
`CURRENT_STATE.toml` explicitly selects this workstream again.

## Language review decision (2026-10-12)

Decision: accepted — preserve the current language contracts, converge the
existing targets, and make delivered support demonstrable. This review accepts
no new syntax or runtime semantic change. Implementation remains parked; the
user authorized judgment, documentation and task registration during the paused
MirBuilder goal. These follow-ups do not enlarge that goal's mandatory scope.

| Feedback | Judgment and existing owner |
| --- | --- |
| Remove null or Option | Keep `Option` for absence, `Result` for recoverable failure, and Unit for successful no-result. Reaffirm Failure/Outcome's existing Canonical-null retirement direction. Immediate deletion is not accepted: current `null`/`void` aliasing stays valid until that row decides and implements the explicit profile/source migration. `Some(null)` rejection alone does not disprove the model. |
| Dynamic values versus `x: T` | Keep dynamic values plus boundary contracts enforced by runtime checks or verified proof. A static checker is not required for that model. Publish actual annotation-site support; `Integer` and active exact-numeric `i64` annotations are not interchangeable availability claims. |
| Strict comparison versus stringify `+` | Different operations may have different contracts. Do not change coercion from this criticism alone. Resolve the concrete operand-order discrepancy in the existing Coercion/Equality row below; Canonical `String + String` remains its pending candidate. |
| Loops, accumulation, recursion and enum match fail | Treat each as a route/profile implementation or conformance gap until reproduced against its exact normative source shape. Do not weaken the language to fit a missing lowering. Distinguish known-enum shorthand, qualified constructors and qualified match patterns. |
| Rename static `me` | Retain the existing lexical `StaticCurrentOwner` meaning; no instance receiver is fabricated. Explain it with instance/static examples in the user guide; no new keyword or source migration. |
| Freeze a working v1 subset | Accept a verified delivery profile, separate from the full normative language and full MirBuilder completion. An unpassed case, including Result/match, is a backlog item, never a supported example. Existing accepted features are implemented before adding unrelated language features. |

Source contracts remain in [types](../../../../reference/language/types.md),
[Option](../../../../reference/language/option.md) and
[call evaluation](../../../../reference/language/function-call-evaluation.md).
Null and coercion changes still require reference-first Decisions and the
existing meaning-specific migration gates; this review does not activate them.

Evidence boundary: fable's reported tests used a 2026-10-04 binary against a
2026-10-12 checkout. They motivate reproduction, not a claim that the current
compiler supports only print/return. Its counts and app failures were not
independently remeasured here. The manual `fast-smoke.yml` uses MIR JSON ->
`ny-llvmc`, including an explicit compatibility replay case; that does not prove
the selected direct `--emit-exe` route. No compiler or application was run for
this documentation decision.

Task routing: delivery evidence and CI follow-through belong to
[Conformance closeout](#7-langv1-conformance-closeout-001); current/target/user-doc
separation and proof retirement extend existing
[H-2/H-4](../investigations/docs-spec-adapter-artifact-hygiene-task-map-2026-10-08.md#independent-tasks-and-timing).
Selfhost source migration follows the existing
[unified resume order](../design/selfhost-parser-mirbuilder-migration-order-ssot.md#unified-resume-order).
No duplicate taskboard, per-example card, or new verification ledger is added.

## Priority Decision

Language coherence is the prerequisite. Selfhost migration is a proof consumer
of the language contract, not authority to preserve ambiguous syntax or
semantics.

```text
language_contract_priority_over_selfhost_migration = historical 1
selfhost_migration_parked = 1
mirbuilder_3456_parked = 1
one_active_language_card = 0
```

Selfhost resume follows only
`../design/selfhost-parser-mirbuilder-migration-order-ssot.md#unified-resume-order`.
This workstream supplies the pinned language conformance/rejection evidence;
closure does not automatically resume historical MapStore 3456.

## Target State

```text
one semantic kernel
one language contract
two independent parsers
one canonical normal form
zero implicit compatibility
```

## Language Constitution

Every row in this workstream must preserve these laws:

1. The same syntax carries the same guarantee at every use site.
2. Semantic meaning and storage/planning representation are separate.
3. Absence, recoverable failure, and language/runtime fault are separate.
4. Object identity and object lifetime are separate.
5. Sugar evaluates each source sub-expression exactly once and preserves
   source order.
6. Compatibility is available only through an explicit named profile.
7. Unsupported behavior fails before user-visible side effects.

## Management Contract

- This taskboard owns the complete order. Do not create a card for inventory,
  fixture refresh, rerun, or consultation notes.
- Open one numbered card only for the current macro row.
- Inside a macro row, use focused implementation commits and shared fixtures.
- A specification-changing row must record `Decision: accepted` before code.
- Rust and Hako parsers stay independently implemented. Shared generation may
  produce contracts, witnesses, fixtures, and documentation, not parser code.
- VM is the narrow semantic oracle; EXE/AOT must prove parity for each live
  semantic guarantee. Unsupported backends fail fast.
- No row advances from counts, source paths, or generated agreement alone.

## Authority Map

```text
language laws:
  docs/reference/language/semantic-contract-charter.md (accepted)

semantic kernel:
  docs/reference/language/semantic-kernel.md (accepted)

canonical grammar:
  docs/reference/language/grammar-contract.md (accepted)
  grammar registry -> generated EBNF/support views

parser implementation:
  Rust parser and Hako parser, independent

parser comparison:
  shared ParseWitness projection and golden corpus

type/failure/ownership semantics:
  topic SSOT plus semantic-kernel rules

runtime oracle:
  VM semantic-reference subset

product proof:
  EXE/AOT plus fail-fast backend preflight
```

The charter, semantic kernel, and grammar contract are current truth. Card 3478
closes the grammar registry, dual-parser recursive corpus, Canonical source
migration, and bounded differential composition.

`FUNCTION-EXIT-SEMANTICS-prime-r1` remains the accepted function/Main/Script/entry
boundary authority. Its implementation queue is closed history, not the current
frontier:

```text
normative topic =
  docs/reference/language/function-exit-and-entry-result.md

current executable row =
  read CURRENT_STATE.current_execution_row

execution card =
  docs/development/current/main/investigations/script-result-tail0-s0-execution-task-2026-07-25.md (closed history)

Current docs frontier:

```text
current row = read CURRENT_STATE.current_execution_row
current blocker = read CURRENT_STATE.current_blocker_token
current card = read CURRENT_STATE.latest_card_path
```

The active row is the bounded canonical-core row named by
`CURRENT_STATE.current_execution_row`. The current Main/source-plan series
does not map a legacy caller or reopen normal/default cutover. Read the card
named by `CURRENT_STATE.latest_card_path` before changing code or adding a
production consumer.
```

The preceding RETURN0 row closed the existing `VerifiedFunctionCompletionV1`
producer's exact zero-or-one root-terminal source contract. The accepted
DRAFT-SEAL decision absorbs MATERIALIZE0 into one prepared/infallible owner:
exit materialization, PHI closure verification, type/stale facts, signature,
metadata, projected verification, extraction, and session close. That exact
0-or-1 slice is closed.

`SCRIPT-RESULT-TAIL-prime-r1` now activates the accepted
`ScriptLastExpressionOrUnit` target without a parser grammar change. One
source-owned prelude/terminal contract replaces the old
`ScriptLastValueOrVoid` and Builder-last-`ValueId` policy. The same row also
closes the remaining Raw BODY temporal gap by preparing exit plus tracker
completion before one infallible commit. App policy, entry projection, and
public activation remain later rows.

The durable function-exit queue is:

```text
FUNCTION-EXIT-F1-RETURN0-S0
  existing completion owner + declared-result relation, exact 0/1 slice

-> FUNCTION-EXIT-F1-DRAFT-SEAL0-S0
  one prepared/infallible canonical draft-seal owner for exact 0/1 exits

-> SCRIPT-RESULT-TAIL0-S0
  source-owned prelude/terminal classifier; statement tails remain Unit
  existing LinearScalar0 only; parser/grammar behavior delta zero
  one prepared BODY exit/completion commit

-> ENTRY-RESULT-PROJECTION0-D0
-> ENTRY-RESULT-PROJECTION0-S0
  T2 design stop, then one macro row:
  CONTRACT0 -> ENTRY-SELECTION0 -> SOURCE-ENTRY0 -> PHYSICAL-THUNK0
  -> VM-REFERENCE0 -> EXE-AOT0 -> PARITY-G0
  source evaluation result and process status remain separate

-> ENTRY-RESULT-PROJECTION0-S1/S2/S3
  typed projection consume, VM-reference consume, and exact Raw compile/VM
  execution are closed

-> RAW-BODY-RETURN-COMPAT-P0
  parked historical observation only; do not materialize a new executable
  profile or a positive proof delta

-> FUNCTION-EXIT-APP-SCRIPT-PARITY0
  merge the admitted canonical witness matrix into the reusable entry-result
  proof family; Legacy difference stays explicit

-> OLD-RAW-RETIRE0-R0A (closed)
  PublishedShell/BuilderReadiness proofs migrated and old-only guards retired
  before opening a new runner consumer

-> OLD-RAW-RETIRE0-R0B (closed)
  delete the caller-zero old Raw source/finalizer/run_raw/evidence chain
  before a new runner consumer
  sunset_id = RAW-PUBLICATION-SUNSET-001

-> NORMAL-ENTRY-D0
  closed by POST-S3-CLEAN-CANARY-prime-r1:
  Canonical grammar + Raw NarrowV1 + no imports/macros/REPL
  + VM-reference + Canonical process projection

-> NORMAL-ENTRY-PROFILE0-S0 (closed)
  passive typed RawVmReferenceProductionRequestV1; default route unchanged

-> NORMAL-ENTRY-CANARY0-S0 (closed)
  one early explicit `--backend raw-vm-reference` consumer; fallback zero
  Stage-1/using/plugin/general dispatch are not entered first

-> NORMAL-ENTRY-CANARY-PARITY0/G0 (closed)
  real-binary status/diagnostic/route isolation proof

-> NORMAL-ENTRY-CUTOVER-D0 (closed)
  A-prime selects the existing explicit route as a supported opt-in
  reference/conformance lane; no safe bounded normal caller exists

-> RAW-VM-REFERENCE-SUPPORT0-S0 (closed; commit 6e1aa8f035)
  one consuming profile handoff, bounded diagnostic report, supported surface,
  proof reclassification, and NORMAL-ENTRY-CANARY-SUNSET-001 closeout

-> NORMAL-ENTRY-CUTOVER-D1 (accepted 2026-07-25)
  historical C park decision; its D0 family evidence is retained below.

-> NORMAL-FILE-VM0-FAMILY-D0 (closed: NoBoundedCallerFamily)
  six plain source-hint sites are benchmark, artifact-only, Stage-1, or legacy
  fallback routes; none matches the fixed normal VM-reference contract.

-> NORMAL-FILE-VM0-FAMILY-D0-FORGE-FRONTDOOR (accepted; active after docs closeout)
  create NormalFileVmFrontDoorV1 with no legacy caller mapping. The first forge
  row keeps production caller=0 and reuses the closed Raw VM-reference kernel.
  See [normal-file-vm0-frontdoor-forge-task-2026-07-26.md](../investigations/normal-file-vm0-frontdoor-forge-task-2026-07-26.md).

-> LANGUAGE-RESULT-EXIT-C-PRIME0-D0 (accepted target; production 0)
  Result-only `?`, no catch, standalone cleanup, one exit owner. Parked exact
  order: `P0 -> I0 (Trivial) -> R0 -> I0/U + I0/F -> HOME0-I0
  (Unique/field) -> I0/S -> HOME0-I0/S (Shared) -> DOC0`. See
  [Result/exit C′](../design/language-result-propagation-and-exit-transaction-ssot.md).

-> NORMAL-FILE-VM0-FRONTDOOR-FORGE0-S0 (closed; production caller=0)
  -> FORGE-CONTRACT0
  -> FORGE-SOURCE0
  -> FORGE-HANDOFF0
  -> FORGE-CORRESPONDENCE0
  -> FORGE-SEMANTIC0-S0 (closed: Script source-text matrix)
  -> FORGE-SEMANTIC0-S1 (closed: Function/Main capability boundary)
  -> RESULT-CARRIER-NORMAL-CAPABILITY0-D0 (closed: Q1/Q2/Q3 = A)
  -> RESULT-CARRIER-NORMAL-CAPABILITY0-S0 (closed)
  -> FORGE-SEMANTIC0-S2 (closed)
  -> FORGE-REUSE0-S0/S1 (closed)
  -> FORGE-G0 (closed)
  -> NORMAL-ENTRY-CUTOVER-D2 (accepted A: explicit new caller)
  The former SOURCE-PROFILE-D0/CORRESPONDENCE-P0/SEMANTIC-MATRIX-P0/
  CALLER-G0 names are historical planning aliases, not a second executable
  queue.

-> NORMAL-FILE-VM0-REQUEST0-S0 (closed)
  -> REPORT0-S0 (closed)
  -> PARITY0-P0a (closed)
  -> CALLER0-I0 (closed: 906592cb54; one central production selector)
  -> PARITY0-P0b (closed: real binary status/diagnostic/feature parity)
  -> G0 (closed: central selector/route guard)
  -> MIRBUILDER-CORE-COMPLETE0-P0 (closed: aggregate proof guard)
  -> NORMAL-SOURCE-PLAN0-D0 (closed: NORMAL-SOURCE-PLAN0-prime-r1)
  -> NORMAL-SOURCE-PLAN0-S0 (closed: disconnected source-family classifier)
  -> NORMAL-SOURCE-PLAN0-INPUT0-S0 / G0 (closed)
  -> NORMAL-MAIN0-SOURCE0-S0 (closed: Program-owned exact Main source)
  -> Main F1/thunk/TX0, neutral VM adapters, callable A0/R0/TX0 (closed)
  -> NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0 (closed)
  -> NORMAL-FILE-CANONICAL-CORE0-DISPATCH0-S0 (exact cell is in CURRENT_STATE)
  One new CLI-visible, default-off request is added; no existing caller is
  replaced, so no legacy sunset is issued in this series.
  The accepted classifier keeps the narrow profile frozen, separates profile
  admission, and seals exactly Script/Main0/CallableModule. The durable
  corrected task order is in
  [normal-source-plan0-design-stop-2026-07-26.md](../investigations/normal-source-plan0-design-stop-2026-07-26.md);
  the exact active slice is in the card named by
  `CURRENT_STATE.latest_card_path`.

-> NORMAL-CANONICAL-CORE0
  source-plan S0/INPUT/G0, Main F1 plus one shared canonical module
  transaction, neutral VM-reference adapters, exact-site callable source,
  singleton/acyclic helpers, sole dispatch, and a separate default-off
  canonical-core profile. Recursive SCC capability follows initial G0 and is
  not a first-production blocker.

-> MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
  requires one source classifier, Script/Main/helper results, one exact entry
  thunk, atomic publication, one shared VM/process terminal, one explicit
  canonical-core caller, and fallback zero.

-> OWNERSHIP-HOME-RESUME-D0
  Resume only after `NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0`, the finite
  function-plan families, atomic candidate publication, and current-normal
  `MirCompileResult` parity are closed. A Candidate A technical audit may run
  first, but the normal/default production edge must not switch yet.
  The successor program is deliberately design-first:
  -> OWN-HOME-CENSUS0
  -> taxonomy + composite/Trivial + representation + storage + lifecycle D0
  -> passive Home relation/ABI/boundary products
  -> straight and CFG Home Flow + diagnostics
  -> take-parameter/from-result/share grammar carriers
  -> first Unique ClosedCallable production slice
  -> ContractBoundary and Shared materialization
  -> measured C-speed gate
  -> OWNERSHIP-HOME-PRODUCT-READINESS-D0
  Exact dependencies and stop lines are owned by
  [hakorune-home-ownership-task-2026-08-04.md](../investigations/hakorune-home-ownership-task-2026-08-04.md).
  The former sparse/View boards are historical evidence and do not select a
  row.
  Selected Stage-B production ingress and loop-refresh remain parked.
  Ownership does not block the verified general Program owner or result-parity
  work, but Candidate A/default promotion does not bypass its readiness gate.

-> NORMAL-ENTRY-PRODUCT-BACKEND-D0
  -> NORMAL-DEFAULT-CALLER-CENSUS0-P0
  -> NORMAL-ENTRY-PROMOTION-D3
  No canonical-core proof automatically changes the default backend or an
  existing caller.

-> NORMAL-FILE-VM0-FRONTDOOR-HARDEN0-S0 (after first caller retirement)
  behavior-neutral BoxShape series: receipt typestate, typed parser rejection,
  and Raw request-constructor visibility tightening.

-> MIR-COMPILER-MODULE-LAYOUT0-D0/S0 (after bounded normal caller is live)
  classify production/proof/scaffold/legacy modules, then perform a separate
  buildable behavior-neutral physical-layout series.

-> NORMAL-IMPORT-BUNDLE-D0 (after no-import family)
  import-aware source authority is a separate bundle/profile family.

-> MIRBUILDER-LEGACY-FENCE0-S0
  direct build_module is retained only inside named LegacyCompatibility
  profiles; unclassified normal caller zero is required before completion.

-> ENTRY-EXIT-CODE-COMPAT-RETIRE0-S0
  sunset_id = ENTRY-EXIT-CODE-COMPAT-SUNSET-001
-> RAW-BODY-RETURN-COMPAT-RETIRE0-S0
  sunset_id = RAW-BODY-RETURN-COMPAT-SUNSET-001
```

`ENTRY-RESULT-PROJECTION0-D0` must inventory all current process-status
producers before selecting an implementation owner. At minimum this includes
the common `vm_execution.rs` adapter, standalone MIR interpreter, quiet-MIR
modulo/Float exit adapter, JoinIR bridge, HV1 inline runner, native
`ny_main`/NyRT positive-handle heuristic, LLVM harness, LLVM mock 42/0
fallback, and historical PyVM status transport. It must also inventory the
duplicated entry-selection logic in `entry_selection.rs` and
`MirInterpreter::execute_module`. It must not begin with a generic
`Box -> exit code` helper: source results are sealed first, the pure process
policy consumes that typed result, and backend adapters transport only the
normalized status.

`NORMAL-ENTRY-D0` is distinct from publication plumbing. An existing explicit
Raw ingress does not authorize changing `compile_with_source`. The design stop
must include an exact direct-caller census and select grammar, function/Main
completion, Script result, process projection, and backend capability before a
canary or cutover row can start. Grammar profile and semantic/process profile
must not be collapsed into one boolean even if a later typed envelope carries
both.

`FUNCTION-EXIT-F1-ALLPATH-D0` and `FUNCTION-EXIT-BARE-RETURN-GRAM0` are
separate capability/grammar branches. Neither may be smuggled into the current
exact slice.

## Confirmed Starting Gaps

```text
compound assignment:
  Rust parser clones the lvalue AST into target and value expression

grammar:
  EBNF, Rust parser, Hako parser, topic docs, and v1 freeze disagree on
  guard/try/peek/from status

type annotations:
  metadata-only and checked boundaries share the same : T spelling

failure vocabulary:
  null and void are runtime-equivalent while Option::None is distinct
  2026-07-10 null migration baseline = 764 .hako files under lang/src + apps

ownership:
  C′ selects terminal Home `fini {}` with direct `obj.fini()` rejected;
  exact owning-field classification and production remain pending

capabilities:
  EffectSummary is metadata-only; CapabilityPlan starts verified=false;
  unknown declared uses are currently ignored
```

These are implementation/inventory evidence, not independent semantic
authority. Rerun mutable counts when their owning row starts.

## Ordered Macro Rows

### 0. LANGV1-CONSTITUTION-001

Closed as 3457. The seven laws, normative precedence, explicit compatibility
and reference-first change protocol are owned by
[semantic-contract-charter.md](../../../../reference/language/semantic-contract-charter.md).

### 1. LANGV1-SEMANTIC-KERNEL-001

Closed through 3459 for the evaluated-Place slice. The five Outcomes, source
order, exactly-once Place evaluation, cleanup precedence and NoFallthrough
remain in [semantic-kernel.md](../../../../reference/language/semantic-kernel.md).
Later failure/Home activation is still gated by its rows below.

### 2. LANGV1-GRAMMAR-001

Closed through 3478 for the then-selected grammar corpus: one registry,
independent Rust/Hako parsers, Canonical default, explicit Compat2025 and shared
ParseWitness comparison. Historical evidence: 106/106 recursive corpus and
bounded differential composition green. This is not current EXE evidence or
activation of later C′/Home targets. The grammar contract and registry own the
current shapes; pending changes below retain their separate decisions.

Completed 3457–3478 task and verification detail is retained at immutable
commit `3272688876f2ae57e94daa108bca5486c4077e4b`, this file's same three
headings (`git show <commit>:docs/development/current/main/workstreams/language-v1-convergence-current.md`).
The durable laws above and all unfinished macro rows stay readable here.

### 3. LANGV1-TYPE-GUARANTEE-001

Status: design accepted as 3479; exact-numeric Box field first slice complete
as 3480; parameter-entry contract complete as 3482; return-exit design accepted
as 3483; return-exit implementation complete as 3484; local exact-numeric
owner/boundary consultation accepted as 3485; BindingId prerequisite complete
as 3486; exact-numeric local semantic implementation complete as 3487; the
representation/contract-refresh owner decision is accepted as 3488; the
single-owner BoxShape implementation is complete as 3489. Record, static table,
typed Array, and Weak field slices are complete through 3502; 3503 is the
active product-backend selection stop for Weak fields.

Target contract:

```text
annotation omitted -> Any
x: T -> gradual semantic contract T
representation/planner hint -> MIR facts, Plan, or Rune; never : T
```

Deliverables:

1. Close the type relation for primitives, exact integers, Box identity types,
   records, enums/generics, `Array<T>`, WeakRef, `Any`, and `void`/Unit.
2. Publish a guarantee matrix for local initialization/reassignment, parameter
   entry, return exit, Box field initialization/write, record fields,
   collection elements, Weak fields, FFI, and backend boundaries.
3. Choose one owner per check: callee entry for parameters, callee exit for
   returns, mutation boundary for fields/elements, and constructor/update
   boundary for records.
4. Elide a runtime check only from verifier-backed proof. Unknown proof retains
   the check; unsupported lowering fails before execution.
5. Inventory current metadata-only `: T` uses and migrate representation-only
   hints before enabling the contract by default.
6. Land boundaries in this order within the same macro row: locals and params,
   returns, Box fields, records, arrays/collections, FFI/backend preflight.

Current executable queue:

```text
T0 complete:
  3484 exact-numeric return-exit implementation

T1 complete:
  3485 local exact-numeric contract design stop
  inventory init/reassignment/PHI/loop/Any/proof invalidation
  BindingId/LocalSlotId + W1 + U1 + VM-only decision accepted

T2a complete:
  3486 behavior-preserving local BindingId owner completion
  LocalSlotId wrapper, one declaration API, CorePlan/snapshot parity
  LocalContractWrite / contract activation = 0

T2b complete:
  3487 LocalContractWrite exact-numeric semantic implementation
  init/reassignment + U1 + PHI/loop evidence + JSON/backend boundary
  no broad static checker; one LocalSlotContractOwner

T3 complete:
  3488 accepts semantic_refresh as the sole refresh-and-validate owner
  3489 routes verifier/JSON/VM/backend/tool boundaries through that owner
  representation-only :T consumer audit and migration queue
  remove direct storage/layout/planner authority from source annotations
  split types.md normative semantics from implementation-status ledger
  keep generated/current support status outside the normative type law

T4 complete:
  3490 exact-numeric annotation-island closeout audit
  exact-numeric annotation-island closeout
  parameter + return + Box field + local owner/exhaustiveness audit
  shared value-check semantics, distinct boundary timing

T5 complete through reference semantics:
  3493 record construction/update contract
  3495 readonly U16 static-table closeout
  3497 ArrayElementWrite convergence
  3499 typed Array exact-numeric state contract
  3501 WeakRef value-law corrective
  3502 WeakFieldContractOwner and VM reference consumer

T6 complete:
  3503 selects defer-with-failfast; no product backend or helper ABI selected
  declaration-level capability rejects read-only and dynamic-only obligations
  VM remains semantic-reference only and cannot satisfy product parity
  stop-the-line gate repair removes the obsolete syntax-3 comment token and
  joins naming_charter_guard to the Language v1 FULL sensitive-path gate

Queued residue after the 3503 decision:
  LANGV1-UNTYPED-FIELD-PHI-FAILFAST-CORRECTIVE-001
  replace debug panic/release Unknown fallback with one Result-bearing owner,
  stable type/* rejection, and an untyped-field method-return fixture
  all active carriers covered by unsupported-backend pre-effect rejection
  no new type-family activation

T5 active:
  3491 selects record construction/update and RecordValueContractOwner
  typed Array remains unselected without a source-owned element contract
  3492 parameter BindingId corrective and FULL 12/12 gate are complete
  3493 record carrier/VM/JSON/backend-gate implementation is active
  require producer + owner + runtime consumer + JSON carrier + backend gate
  do not open FFI/backend lowering or all-types activation implicitly
```

Do not create inventory-only, fixture-only, or rerun-only numbered cards for
T1-T5. Materialize only the currently selected design or implementation card.

Acceptance:

```text
annotation_semantic_contract = 1
unannotated_value_contract = Any
annotation_site_set_closed = 1
contract_check_owner_count_per_boundary = 1
metadata_hint_spelled_as_type_annotation = 0
unsupported_backend_fail_fast_before_effect = 1
normative_type_law_status_ledger_separated = 1
```

Next: `LANGV1-FAILURE-OUTCOME-001`.

Fable5 review routing (2026-07-11):

```text
already owned by Failure/Outcome:
  null / void / Option::None terminal relation
  local-without-initializer default migration
  failed WeakRef upgrade result migration
  grammar literal_null Canonical -> Compat2025 decision and source migration

new semantic row after Failure/Outcome:
  truthiness, compatibility equality, broad String concatenation, type tests

already owned by Ownership/Identity:
  ordinary strong-field cascade fini removal
  identity/lifetime split

parked compiler BoxShape, not Language-v1 blockers:
  nested-loop depth1 feature-product decomposition
  route-family graph shadow -> authority promotion
```

Parked compiler follow-up contracts:

```text
nested-loop depth1 decomposition:
  owner = plan/REGISTRY.md + coreplan skeleton/feature SSOT
  trigger = four feature-product planners already exist
  shape = one NestedLoopDepth1Skeleton + typed FeatureSet
  accepted-shape expansion = 0
  release-default change = 0

route-family convergence promotion:
  owner = route_dependency_graph + route_fixpoint facade
  prerequisite = fresh current-corpus shadow/full-refresh parity
  prerequisite = deterministic dirty worklist and zero stale metadata reads
  prerequisite = measured recompute reduction without semantic drift
  next step = one authority-switch design card, not an optimization patch
  helper-name cache / fixture cache / wall-clock termination = forbidden
```

Neither compiler follow-up may preempt the current Language v1 blocker without
an explicit lane-priority decision.

The current registry already classifies `literal_null` as Canonical in both
profiles. Moving it to Compat2025 therefore overturns an accepted 3477/3478
decision and must happen only through the Failure/Outcome decision plus source
migration; it is not a grammar typo cleanup.

### 4. LANGV1-FAILURE-OUTCOME-001

Status: C′ target accepted 2026-08-05; implementation remains parked.

Target vocabulary:

```text
Option::None = ordinary value absence
Result::Err  = recoverable failure returned as a value
Result expr? = exact unchanged Err propagation into enclosing Result
Option ?     = rejected; absence stays guard/match-visible
Fault        = violated language/runtime contract
Normal(Unit) = successful computation with no useful result
null         = Compat2025 migration surface only
catch / RecoverableFailure = rejected target
```

Deliverables:

1. Keep Unit, Option absence, Result error, NoFallthrough, and Fault distinct;
   Fault never converts implicitly to Result.
2. Seal Result `?` only for exact `Result<T,E>` -> `Result<U,E>`; reject
   conversion, custom Try, Option `?`, catch, and RecoverableFailure.
3. Census every current `?` token/AST site, migrate or reject C-style ternary
   `cond ? a : b`, and prove the postfix grammar has no unresolved collision.
4. Classify and migrate `null`, uninitialized locals, Weak upgrade, Null/Void/
   Missing boxes, and FFI observations by meaning; no global text rewrite or
   Canonical-to-Compat retry.
   Reaffirmed by the 2026-10-12 review: keep Option, Result and Unit distinct;
   preserve current null semantics until the migration lands. Resolve
   `Option<void>` / `Some(void)` payload-domain consistency in this same matrix;
   no new optional carrier or blanket deletion is implied.
5. Publish the VM/EXE value/Outcome/cleanup matrix, including first-Fault
   chronology, suppressed teardown Faults, and remaining-release witnesses.
6. Use only `P0 -> I0 -> R0 -> HOME0-I0 (Unique/field) -> HOME0-I0/S
   (Shared) -> DOC0`; each Home cell waits for its named DropPlan/finalization.
   `DOC0` updates grammar, both parsers' reference witnesses, references,
   examples, diagnostics, and backends only after the physical gates are green.

Acceptance:

```text
absence_failure_fault_relation_count = 1
fault_implicit_result_conversion = 0
unit_fault_nofallthrough_distinct = 1
canonical_null_surface = 0
compat2025_null_surface = 1
catchable_fault_set_closed = 1
recoverable_failure_owner = Result
result_qmark_error_relation = exact
option_qmark_surface = 0
recoverable_failure_outcome = 0
canonical_cleanup_surface_count = 1
ternary_qmark_unresolved_collision_count = 0
first_fault_primary_chronology = 1
remaining_teardown_after_fault = best_effort_verified
local_default_absence_owner_count = 1
weak_upgrade_absence_owner_count = 1
```

Next: `LANGV1-COERCION-EQUALITY-COMPAT-001`.

### 4A. LANGV1-COERCION-EQUALITY-COMPAT-001

Status: design decision required after Failure/Outcome.

Review evidence (2026-10-12, source inspection only): `types.md` §4.1 accepts
`String + any` but rejects other combinations; VM `eval_binop` in
`src/backend/mir_interpreter/helpers.rs` also accepts `any + String`.
The linked historical Phase-274 P3 C2 text rejects both mixed directions.
Deliverable 1 must resolve that discrepancy under the current normative owner,
mark superseded historical wording, and cover both operand orders, nullish
rejection and explicit conversion. No native behavior was measured here;
VM branches and old phase prose cannot redefine the current contract.

Purpose: remove dynamic-language compatibility residue from Canonical
truthiness, equality, concatenation, and type tests without silently changing
the already accepted runtime rules.

Decision surface:

```text
truthiness Canonical candidate set:
  Bool only
  or Bool + Integer

Compat2025 candidates:
  Float truthiness
  String / StringBox truthiness

canonical equality:
  primitive same-kind plus accepted numeric relation
  Box equality follows BoxIdentity
  absence follows the landed Failure/Outcome relation

compatibility equality candidates:
  Void == NullBox / VoidBox / MissingBox
  dropped WeakRef == Void

canonical concatenation candidate:
  String + String only

compatibility concatenation candidate:
  String + any implicit toString coercion

canonical type test:
  typed type-reference surface, exact spelling selected by consultation

compatibility type test candidate:
  x.is("TypeName") / x.as("TypeName")
```

Deliverables:

1. Define one closed profile-aware semantic matrix for truthiness, equality,
   concatenation, and type test/cast. Grammar acceptance alone is not semantic
   authority; decide one semantic compatibility registry or equivalent SSOT.
2. Select the Canonical truthiness set explicitly. Treat the current
   Bool/Integer/ExactNumeric/Float/String behavior as implementation evidence,
   not an automatic v1 decision.
3. Remove compatibility equality from the Canonical relation. Inventory
   `Void`, `NullBox`, `VoidBox`, `MissingBox`, dropped WeakRef, and pointer
   identity together so absence and identity are not mixed.
4. Decide whether Canonical concatenation is `String + String` only. Any broad
   implicit stringify rule retained for migration must require explicit
   Compat2025 and reject Void/Fault. Inventory both operand orders in VM
   `eval_binop`, `StringBox` dynamic-add dispatch, and the generic `AddBox`
   stringify fallback; none may independently define Canonical semantics.
5. Replace stringly typed `x.is("TypeName")` / `x.as("TypeName")` with a
   type-reference-based Canonical surface. Typos and unsupported backend paths
   fail before user effects; string spellings remain compatibility-only if
   retained.
6. Generate positive/negative/profile fixtures for independent parsers where
   syntax changes, plus VM and EXE semantic fixtures. Do not use environment
   flags or Canonical-to-Compat retry.
7. Inventory source migration before changing defaults. Selfhost usage is
   migration evidence, not authority to keep a rule Canonical.

Acceptance:

```text
truthiness_semantic_owner_count = 1
truthiness_canonical_value_set_closed = 1
implicit_string_truthiness_canonical = 0
compatibility_equality_in_canonical = 0
box_equality_uses_identity_relation = 1
broad_string_concat_canonical = 0
string_type_name_test_canonical = 0
semantic_compatibility_requires_explicit_profile = 1
runtime_backend_fallback = 0
```

Next: `BOX-MEMBER-FIELD-METHOD-SURFACE0-D0`.

### 4B. BOX-MEMBER-PROPERTY-RETIRE0

Status: target accepted; implementation parked behind the active MirBuilder
lane.

Canonical Box value members converge to stored/weak fields; computation is an
ordinary method. The fixed order is surface D0 -> full census D1 -> stored
initializer owner extraction -> source migration -> atomic Property retirement
and diagnostics -> reference/ownership docs closeout -> independent same-name
field/method audit. Exact contracts and stop lines are owned by
[`box-member-field-method-surface-ssot.md`](../design/box-member-field-method-surface-ssot.md).

This row must close before Home Flow production adoption so `obj.x` has one
place meaning and `obj.x()` has one call/result relation. It does not implement
Home Flow, readonly, visibility enforcement, memo/lazy, or same-name admission.

Next: `LANGV1-OWNERSHIP-IDENTITY-001`.

### 5. LANGV1-OWNERSHIP-IDENTITY-001

Status: C′ target accepted 2026-08-05; implementation/fixtures remain queued.

Decision owner:
`docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md`

Target laws:

```text
record = immutable identity-free value; with creates a new value
ordinary Box use = non-owning handle
verified Home destination = one transferred Home
share = sole owner-add; release root = sole early Home-end; drop alias = 0
weak = generation-aware non-owner
fini { } = non-callable terminal Home hook
direct obj.fini() = rejected
terminal DropPlan = hook -> reverse owning-field release -> structural drop
close()/shutdown() = ordinary optional domain methods
```

Deliverables:

1. Keep records identity/lifecycle-free; reuse the Home board for taxonomy,
   destination, ABI, Flow, replacement, birth, representation, and diagnostics.
2. Parent hook precedes reverse verified-owning-field release; child hook needs
   terminal child Home. User code cannot call child `fini` or reorder release.
3. Keep field classification in `OWN-FIELD-CONTAINER-DEST-D0`; project every
   host/weak/plugin identity through one generation-aware `BoxIdentity`.
4. Activate Unique, field, and Shared/weak cells separately; retire B′, run the
   C-speed gate, update exact references in each implementation commit, then
   execute fini/release Home reference closeout twice as specified.

Acceptance:

```text
record_box_semantic_law_count = 2
field_ownership_classification_owner_count = 1
terminal_home_finalization_owner_count = 1
direct_fini_call_surface = 0
parent_hook_before_field_release = 1
verified_owning_field_release_order = reverse_declaration
child_hook_requires_terminal_home = 1
box_identity_relation_count = 1
weak_identity_relation_count = 1
ownership_operation_relation_count = 1
explicit_home_release_spelling_count = 1
drop_alias_surface_count = 0
prose_only_escape_authority = 0
```

Next: `LANGV1-CAPABILITY-EFFECT-001`.

### 6. LANGV1-CAPABILITY-EFFECT-001

Status: queued after Ownership/Identity.

Target axes:

```text
uses X = declared authority budget
EffectSummary = observed/transitive effect set
@rune Contract(...) = verifier obligation
```

Deliverables:

1. Define a closed effect vocabulary and conservative treatment of dynamic
   dispatch, FFI, unknown callees, allocation, safepoints, and publication.
2. Verify `actual effects` are a subset of declared `uses`.
3. Verify Rune promises against actual effects independently of capability
   authorization.
4. Change CapabilityPlan/EffectPlan to `verified=true` only after both checks.
5. Reject unknown declared capability names instead of ignoring them.
6. Backend consumes verified Plan only and must not rediscover effects or
   authority from source names.

Acceptance:

```text
capability_effect_contract_axis_count = 3
actual_effect_subset_declared_uses = 1
rune_contract_verified_independently = 1
unknown_declared_capability_ignored = 0
backend_unverified_plan_consumption = 0
```

Next: `LANGV1-CONFORMANCE-CLOSEOUT-001`.

### 7. LANGV1-CONFORMANCE-CLOSEOUT-001

Status: queued last; delivery substeps registered 2026-10-12, not executed.
Evidence may be accumulated in the owning implementation slices; final closeout
waits for every required language macro row. Follow the unified resume order,
not the historical instruction to automatically resume MapStore 3456.

| Substep | Dependency / owner | Completion evidence |
| --- | --- | --- |
| C-1: pin delivered profile | Explicit selection after the required MirBuilder convergence; existing topic contracts and smoke owners | Fresh source revision and dirty diff identity if any, built binary provenance, toolchain, backend/profile, exact command, stdout/exit and first-stop. Classify supported, intentionally rejected and pending cases. Reproduce fable's failures without rewriting their source to bypass the compiler. |
| C-2: close elementary EXE coverage | C-1; existing callable/loop/value owners and reusable smoke runner | Cover box/field/method/new, if, condition loops (including method-call conditions), range/accumulation, recursion, Array read/write, and their meaningful compositions. Reuse existing fixtures, roughly 20–30 small programs if sufficient; coverage and exact expected behavior, not file count, close the task. Result/enum-match joins only after its parser/semantic/backend prerequisites pass. |
| C-3: require qualified native checks | C-2's deterministic green cases; existing `MIRBUILDER-PR-STRUCTURAL-GATES-I0` CI owner | Qualify the real LLVM smoke, then gate the advertised delivery pack on non-draft PR updates through the existing CI tiers. Prove actual EXE execution and expected stdout/exit with no fallback or skip-as-pass; record build/link separately from execution cost. Verify the named check on the integration head and branch protection separately. |
| C-4: publish usable guide | C-1–C-3; H-2 owns the documentation change | Every quick-reference example links to the verified fixture and executable command for the named revision/profile. Planned features and implementation status have separate destinations. Unverified cases stay out of the usable guide. |
| C-5: compiler-source migration handoff | Post-closeout follow-up; existing selfhost migration owner | After the applicable type/failure/Home contracts work, migrate one selected compiler family and its callers by meaning. Prove parser witness and required EXE behavior before removing the old form; record any accepted semantic delta. No blanket typed/Home/Result rewrite or compiler-gap workaround. |

C-5 implementation is not a prerequisite for this closeout. Syntax migrations
needed by a language macro row remain in that row's acceptance. Selfhost
authority switches wait for unified resume gates 1–3 and explicit CURRENT_STATE
selection; registering this follow-up supplies no earlier execution permission.

C-1/C-2 reuse the existing
[`real-apps-exe-boundary` suite](../../../../../tools/smokes/v2/suites/integration/real-apps-exe-boundary.txt),
runner and individual smoke owners for commands, expectations and route checks.
Do not expand that frozen suite's scope by silently inserting this proposed
user pack. Bind each advertised example to one existing or missing semantic
case; the source fixture and runner own expected behavior, not this table.
The small-program pack supplements real applications; it cannot replace the
unchanged mimalloc acceptance or whole MirBuilder completion.

C-3 follows the [CI queue](../../../../../.github/workflows/README.md#parked-mirbuilder-merge-readiness-hardening).
Do not promote `fast-smoke.yml` unchanged: its MIR-JSON/compatibility route is
not the direct physical EXE proof. Workflow wiring and external required-check
configuration are distinct completion evidence; neither was changed here.

Final acceptance retains all existing obligations:

1. Canonical grammar and ParseWitness conformance on both independent parsers.
2. Semantic-kernel evaluation-order and Outcome fixtures on VM and EXE.
3. Type, failure, ownership, identity and capability positive/negative packs,
   including unsupported-backend rejection before user-visible effects.
4. Canonical default with zero implicit compatibility, generated reference
   views and the verified delivery guide; unsupported target features are not
   advertised as delivered.
5. Selfhost handoff only through the
   [unified resume order](../design/selfhost-parser-mirbuilder-migration-order-ssot.md#unified-resume-order),
   including the separate mimalloc promotion gate and actual authority selection.

```text
language_v1_frozen = 1
semantic_kernel_count = 1
language_contract_count = 1
independent_parser_count = 2
canonical_normal_form_count = 1
implicit_compatibility_count = 0
vm_exe_semantic_parity = 1
selfhost_resume_prerequisites_recorded = 1
```

## Current Front

The live front is owned only by `CURRENT_STATE.toml`. This language taskboard
remains parked while MirBuilder owns the selected implementation lane. The
2026-10-12 documentation decision does not resume the paused goal or select
any implementation row.
