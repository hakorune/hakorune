---
Status: Active workstream
Date: 2026-08-01
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
North star:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
Task map:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
---

# MirBuilder In-Place Replacement Workstream

## Goal

最終production authorityを次の一本へ収束させる。

```text
Resolve -> Observe -> Facts -> Recipe -> Verify
        -> Lower -> Seal -> Collect -> Atomic Publish
```

現在のMirBuilderを稼働させたまま、実在するproduction responsibilityを
一つずつ交換する。第二MirBuilder、production consumer 0のroute拡張、
Legacy fallback/retry、完成Program形ごとのvariant列挙は作らない。

## Current state

```text
active lane:
  MirBuilder in-place replacement

current design stop:
  RAW-SCRIPT-ROOT-NEUTRAL-SHADOW-TRAVERSAL0-D0

latest structural finding:
  normal_script_lexical_binding::admit_expression_v1 is becoming a second
  lexical resolver beside resolved_semantics/shadow/**.

next authority:
  external design consultation only; no further AST-family cutover is selected

latest production closeout:
  RAW-SCRIPT-SELECTED-UNSUPPORTED-SEMANTIC-CLOSURE0-I0-R0 / 106e5cbe5b

latest docs boundary:
  root-neutral shadow traversal consultation / 3aad4871ea
```

`CURRENT_STATE.toml` is the pointer SSOT. Git history owns detailed landed
diffs and proof transcripts; this card keeps only the live boundary, active
fences, compact queue, and short landed tail.

## RAW-SCRIPT-ROOT-NEUTRAL-SHADOW-TRAVERSAL0-D0

Consultation:
`docs/development/current/main/investigations/raw-script-root-neutral-shadow-traversal0-design-consultation-question-2026-08-01.md`

```text
Change:
  design one private root-neutral semantic traversal over the Script runtime
  demand window and original ProgramBody ordinals. The first production
  cutover must delete admit_expression_v1 and the manual Script Local/Variable
  fact construction in the same implementation series.

Contract:
  Function/Lambda public views remain narrow. Function/Lambda and Script share
  one semantic-owner core, one forest, and one projection authority. A request
  selects Complete or Deferred before lowering and executes RootLower exactly
  once. Existing diagnostic order, source identity, candidate isolation, and
  raw/reference behavior remain unchanged.

Done:
  the accepted answer identifies the neutral demand-window input, exact
  source/transfer coverage, issuer stage, root-profile canonicalization,
  failure mapping, first atomic production slice, and old edges deleted.
  Existing Complete fixture identities remain a mechanically anchored subset.

Stop:
  no synthetic FunctionDeclaration; no FunctionSourceViewV1 or
  FunctionSyntaxViewV1 Program widening; no second resolver/forest/projection;
  no partial forest; no AST-family enumeration; no Complete-to-Deferred
  downgrade; no fallback/retry; no Program clone/reparse; no caller-zero owner;
  no new per-row guard; every touched source/check file stays below 800 lines.
```

While this stop is open, do not select `UsingStatement` or another isolated
AST constructor. Using is locally movable but would formalize a generic
`emit_void` compatibility no-op while the competing resolver remains.

## Production invariants

```text
named production caller required       = yes
same-commit selected old-edge deletion = yes
route selection per request            = exactly 1
RootLower execution per request        = exactly 1
canonical rejection -> retry/fallback  = 0
partial product publication            = 0
source AST clone/reparse                = 0
new whole-function accepted variants   = 0
new per-row guard                       = 0
source/check file line limit            < 800
```

One explicit compatibility owner may exist inside the selected production
pipeline only with a stable sunset ID, exact owned surface, no retry, and a
named release condition. Each replacement row shrinks that surface; it may not
grow or silently absorb a new family.

## R4 active fence / residual registry

This is the sole live R4 disposition list. Closed residual history is in Git.
R4 Complete requires every row below to be retired, reowned, or explicitly
retained by final conformance.

| State | Stable ID | Exact live surface | Release condition |
|---|---|---|---|
| retain-fenced | `RAW-STATIC-MAIN-COMPAT-BATCH-SUNSET-001` | arbitrary-AST raw static-Main helper-first batch through `RawLegacyChildLoweringPortV1` | one located-source and entry-materialization owner atomically deletes dispatcher, helper, and legacy Main-policy edges |
| retain-fenced | `JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0` | two dev-gated normalized-shadow mutations plus comparison observer | verified Recipe/CorePlan loop owner replaces both mutations and observer disposition is explicit |
| retain-fenced | `VM-BRIDGE-COMPAT-SUNSET-001` | explicit VM keep skip/trim bridge only | caller zero or one explicit-lane owner replaces every success/failure continuation |
| retain-fenced | `NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001` | two live nested static/instance method `LegacyChildDraftAdmissionV1` issuers | one function-relative located-source contract deletes both issuers |
| active compatibility | `RAW-RECURSIVE-UNLOCATED-TRANSPORT-SUNSET-001` | selected ControlBody Lambda, CallObject, NestedBoxAdmission portals | Lambda lineage, CallObject, and nested admission rows delete all three portals |
| retain-fenced | `RAW-LAMBDA-CHILD-OWNER-SOURCE-LINEAGE-SUNSET-001` | selected nested Lambda still enters raw capture/publication without semantic child-owner source | exact `forest.child_at`, parent edge/scope, projected LambdaBodyRoot, and single ClosureBodyId publication replace the edge atomically |
| active compatibility | `RAW-LOCATED-LOOP-ROUTE-SOURCE-HANDOFF-SUNSET-001` | located Loop product delegates to existing raw JoinIR route | verified Loop plan consumes the same located product and source-erasing terminal becomes zero |
| retain-fenced | `JOINMODULE-SHARED-REFERENCE-SUBSTRATE-SUNSET-001` | JoinModule model/converter/lowering shared only by normalized-shadow and VM bridge fences | both consumer fences close and fresh census proves all production callers zero |

Current exact registry count:

```text
retain-fenced        = 6
active compatibility = 2
active retirement    = 0
active rehome         = 0
unregistered         = 0
```

`LegacyChildDraftAdmissionV1` latest exact census: 16 occurrences in 4
`src/mir` files. Seven are production-core and nine are live-path proof
occurrences. The only live issuers are the two nested-method sites registered
under `NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001`.

## Other live compatibility contract

```text
CALL-BOX-KIND-ANALYZER-COMPAT-SUNSET-001
  state: active
  owner: CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler
  surface: BreakFinderBox / PhiInjectorBox / LoopSSA
  growth: forbidden
  retire_when: analyzer production routes are zero, or one-profile
    classification parity is proven and all callers migrate atomically
```

## Guard-required closed anchors

These compact anchors retain stable manifest/guard correspondence. They are
not a landed-history ledger.

```text
NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
  state: closed; selected-normal build_module edge = 0

MIRCOMPILER-ARBITRARY-AST-COMPAT-SUNSET-001
  state: closed; public compiler accepts whole-file Program only

RUNTIME-MIRBUILDER-AST-JSON-COMPAT-SUNSET-001
  state: closed; runtime Program(JSON v0) admission rejects before Builder

SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001
  state: active; Complete fixture set may only grow; every Deferred reason is typed

STAGE1-DIRECT-POST-MACRO-NONPROGRAM-COMPAT-SUNSET-001
  state: closed
  retired by: STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0

RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001
  state: closed; owner / residual / execution callers = 0
```

## Compact queue

```text
R2bi RAW-SCRIPT-ROOT-NEUTRAL-SHADOW-TRAVERSAL0-D0
  current external design consultation

next
  one accepted atomic production cutover selected by D0
  -> delete manual Script resolver edge(s)
  -> focused proof / build / shared guards
  -> batch-boundary live-edge census

R4
  MIRBUILDER-R4-FINAL-CONFORMANCE0-C0 after all active rows above have exact
  retire/reown/retain decisions

after final-pipeline Complete only
  refresh missing-feature / Ownership / View readiness inventory
  resume Ownership taskboard
  then select later unimplemented language features
```

## Short landed tail

| Commit | Result |
|---|---|
| `3aad4871ea` | opened the root-neutral shared-shadow traversal consultation |
| `106e5cbe5b` | moved the existing nine-kind selected diagnostic family into Complete semantic coverage without changing diagnostics |
| `67237924fb` | moved StaticConst zero-child completion into Complete while preserving metadata/runtime owners |
| `1f17bc93d1` | composed And/Or into the existing recursive Script closure |
| `a78d4e968a` | composed CheckExpr into the existing recursive Script closure |
| `074e944fec` | composed Await into the existing recursive Script closure |
| `b562263854` | composed Binary into the existing recursive Script closure |
| `c1c7852b76` | composed Print into Script lexical closure |
| `1adb617542` | composed Unary into Script lexical closure |
| `7bf6c9b996` | established the first selected Script lexical binding closure |

The repeated constructor history above is the final legacy example of the old
per-constructor cadence. Future constructors in one accepted family share one
implementation-coupled batch and one batch-boundary census.

## Fixed packs

```text
REPLACEMENT-LEDGER0  production owner / detached asset accountability
DESCENT-SPINE0       body / statement / expression / argument descent
FUNCTION-STATE0      function facts / PHI / finalization state
CALL-OBJECT0         calls / new / fields / index / collections / lambda
CONTROL0             If / Loop / Match / QMark / cleanup / async
FUNCTION-LIFECYCLE0  draft / collector / function finalize
MODULE-LIFECYCLE0    declaration / catalog / module transaction
COMPILER-RESIDUE0    compiler ingress / old selectors / proof routes
```

New findings enter one of these packs. Do not create another pack.

## Parked

```text
source-level Ownership/View and unimplemented language features until the
repository-wide final pipeline is Complete
.hako selfhost MirBuilder/parser migration
unselected cleanliness work
new language semantics
default Raw/Canonical cutover before final conformance
```

New per-row guards are forbidden. Normal gates and detailed assertions belong
to the active source/tests and existing shared guards.
