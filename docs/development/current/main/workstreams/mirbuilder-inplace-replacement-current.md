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

current execution:
  SEMANTIC-SHADOW-ROOT-NEUTRAL-ENTRY0-S0

latest structural finding:
  normal_script_lexical_binding::admit_expression_v1 is becoming a second
  lexical resolver beside resolved_semantics/shadow/**.

accepted series:
  dense Function/Lambda root-neutral core -> sparse selected-Script cutover

latest production closeout:
  RAW-SCRIPT-SELECTED-UNSUPPORTED-SEMANTIC-CLOSURE0-I0-R0 / 106e5cbe5b

consultation decision:
  RAW-SCRIPT-ROOT-NEUTRAL-SHADOW-TRAVERSAL0-D0 / Accept-corrected
```

`CURRENT_STATE.toml` is the pointer SSOT. Git history owns detailed landed
diffs and proof transcripts; this card keeps only the live boundary, active
fences, compact queue, and short landed tail.

## Root-neutral shadow traversal Refactor Series

Consultations:
`docs/development/current/main/investigations/raw-script-root-neutral-shadow-traversal0-design-consultation-question-2026-08-01.md`
and `docs/development/current/main/investigations/raw-script-demand-window-boundary2-design-consultation-question-2026-08-01.md`

```text
Decision:
  Accept-corrected. One private root-neutral traversal becomes the sole
  Function/Lambda and selected-Script lexical shadow authority. S0 is a live
  behavior-neutral refactor of existing explicit canonical Function/Lambda
  consumers; T2 is the selected Script production cutover.

Contract:
  FunctionSyntaxViewV1 and FunctionSourceViewV1 remain Function/Lambda-only.
  Dense roots preserve all current controls: lambda inventory/reject, ancestor
  bindings, qualified-receiver requests, and method-call observation. Sparse
  Script roots borrow Program by original ordinal, select Complete/Deferred
  once, issue owner/forest/projection only after Complete, and lower once.

Series boundary:
  S0 extracts a Dense ShadowRootTraversalInputV1 and an identity-free shadow
  draft, while final canonicalization remains (owner, origin, draft). T2 adds
  SparseScript plus typed Resolved/Transferred/Diagnostic/Transparent demand
  boundaries. StaticConst and selected unsupported are root-iterator
  boundaries; their children are never traversed. Responsibility gates run
  before child descent.

Stop:
  no synthetic FunctionDeclaration; no FunctionSourceViewV1 or
  FunctionSyntaxViewV1 Program widening; no second resolver/forest/projection;
  no partial forest; no AST-family enumeration; no Complete-to-Deferred
  downgrade; no fallback/retry; no Program clone/reparse; no caller-zero owner;
  no new per-row guard; every touched source/check file stays below 800 lines.
```

### SEMANTIC-SHADOW-ROOT-NEUTRAL-ENTRY0-S0

```text
Change:
  Replace the FunctionSyntaxViewV1-only traverse_shadow_view entry with a
  private Dense ShadowRootTraversalInputV1 and traverse_shadow_root_v1. Split
  origin-free shadow facts from final canonicalization; all existing Function
  and Lambda routes consume the same dense adapter.

Contract:
  Existing explicit canonical Function/Lambda production APIs are the real S0
  consumers. Function/Lambda graph, diagnostics, lambda topology, qualified
  receiver, and method-call observations are byte/graph equivalent. No Script
  production consumer or builder demand window is added in S0.

Done:
  FunctionSyntaxView-only direct traversal entry = 0; Function and Lambda each
  reach root-neutral traversal once; observer entries retain the same controls;
  dense resolver/forest/normalized fixtures remain equivalent.

Stop:
  stop if dense extraction needs a public view change, loses an observer
  control, changes Function/Lambda canonicalization, or cannot keep every
  source/check file under 800 lines. Do not add SparseScript in S0.
```

### RAW-SCRIPT-ROOT-NEUTRAL-LEXICAL-SHADOW-CUTOVER0-I0-R0

```text
Change:
  Seal one sparse Program demand window from the existing work-plan partition;
  borrow ScriptSyntaxViewV1 by exact ProgramBody(original ordinal); run the
  same root-neutral core under ScriptLexicalCoreV1; select Complete/Deferred
  before CatalogInstall; and delete the full manual Script resolver chain.

Window contract:
  Every original ProgramBody ordinal occurs once with a typed semantic and
  runtime disposition. Resolved entries are the existing lexical closure;
  StaticConst is a retained-runtime metadata transfer, selected unsupported is
  a retained diagnostic boundary, and top-level FunctionDeclaration is a
  no-runtime callable transfer. Using, Box, control, call/object, allocation,
  Weak, and Lambda select Deferred before child descent in this row.

Contract:
  Demand-window coverage includes every Program item, including transferred
  top-level callable and runtime-bearing Box boundaries. Complete retains the
  existing ten fixture identities; Deferred owns no ID/forest/projection and
  executes ExistingRootLower once. Shadow source errors only select Deferred;
  RootLower still owns user diagnostics and first-error order.

Done:
  semantic_closure_admission -> admit_runtime_script_lexical_v1 ->
  admit_expression_v1 -> manual facts -> ResolvedScriptSemanticDraftV1 = 0.
  The replacement uses one core, one forest, one Program projection, no retry,
  and Complete no longer reaches bare script_root(()).

Stop:
  no partial request routing, Script-only match tree/visible map, compact-index
  source recovery, fabricated Script origin, Complete-to-Deferred downgrade,
  raw/reference change, control/call/Lambda/Box activation, or source/check
  file >=800.
```

### Mandatory S0 preparation

```text
before T2:
  extract normal_script_semantic_source tests into its existing sibling module
  (794-line source has no room)
  move the two hardcoded ratchet checks into a reusable shared-guard helper
  and make all ten Complete IDs plus Deferred floors map to path + test anchor
  keep program_root_work_plan.rs unchanged at 799; the sparse window is a
  sibling product wired through its existing seam
  retain only BindingRef -> ValueId ledger in normal_script_semantic_lowering_state
  delete normal_script_lexical_binding.rs in T2
```

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
  closed Accept-corrected

R2bj RAW-SCRIPT-DEMAND-WINDOW-BOUNDARY2-D0
  closed Accept-corrected

current
  RAW-SCRIPT-FASTMEM-STRUCTURED-SCOPE0-D0
  -> existing FastMem structured-scope shadow traversal + existing FastMem lower
  -> lexical-safe whole FastMem body may Complete; unsafe body remains Deferred
  -> no FastMem contract/region/metadata diagnostic may move before RootLower

ordered after fresh evidence only
  1. top-level bare Me existing-diagnostic boundary (zero child; no receiver fact)
  2. If / assignment / Loop / Return only as separate Control, Mutation, JoinIR,
     and Exit design rows
  3. Call/Object / allocation / Lambda only as separate owner-family design rows
  4. R4 consumes the live fence registry above; every item must retire, reown,
     or be explicitly retained before final conformance

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
| `583c3dcf5a` | moved dense Function/Lambda consumers through one private root-neutral traversal input |
| `78771167ef` | separated identity-free shadow facts from final owner/origin canonicalization |
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
