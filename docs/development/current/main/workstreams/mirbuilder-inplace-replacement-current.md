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
  RAW-SCRIPT-NEXT-NAMED-FAMILY0-D0

latest structural finding:
  Function/Lambda production now validates its complete recursive shadow tree
  before canonical owner issue. Script Lambda can use the same construction
  rule and ordered BindingRef receipt without moving closure publication.

accepted series:
  Function/Lambda construction tree -> Script Lambda observer retirement

latest production closeout:
  RAW-SCRIPT-LAMBDA-CHILD-OWNER-LINEAGE0-I0-R0

next decision:
  fresh named-family census; do not pre-authorize Control, Call/Object,
  Allocation, Weak, or Box from the Lambda closeout.
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

### Recovered WIP order

```text
closed — SEMANTIC-OWNER-RECURSIVE-SHADOW-TREE0-S0
  Existing Function/Lambda production seals one construction-local ordered
  BindingRef capture receipt against canonical upvar observations. No Script
  route, capture ABI materialization, or closure publication changed.

closed — SEMANTIC-OWNER-RECURSIVE-CONSTRUCTION-TREE0-S1
  Existing Function/Lambda recursive forest now completes shadow validation
  before canonical ID issue, so a rejected nested Lambda leaves the session
  ready for a fresh owner at ordinal zero.

closed — RAW-SCRIPT-LAMBDA-CHILD-OWNER-LINEAGE0-I0-R0
  Script Lambda now seals one child owner and ordered BindingRef receipt in
  the shared forest. Selected lowering materializes that receipt into the
  existing closure-emission owner; Deferred/raw/reference retain the old
  observer. Focused Script parity and Lambda lifecycle tests are green.

after that — fresh named-family census
  Select exactly one remaining Deferred responsibility family. Control,
  Call/Object, allocation, Weak, Lambda, and Box are not a pre-authorized
  batch. `SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001` remains the monotonic
  fixture-identity ratchet through R4.

already closed — do not reopen as WIP
  QMark propagation, root Match control, StaticConst completion, and fully
  explicit Record schema admission. Their detailed evidence is Git history;
  only a regression can reopen their authority.
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

closed
  SEMANTIC-OWNER-RECURSIVE-SHADOW-TREE0-S0

  Change:
    Existing Function/Lambda owner resolution builds one construction-local
    recursive shadow tree and records first-demand capture events before IDs.

  Contract:
    Function/Lambda behavior and canonical graph remain unchanged. No Script
    consumer, capture ABI materialization, or closure publication changes.

  Done:
    Existing Function/Lambda consumes the new tree once; ordered first-demand
    BindingRef rows are construction-local until canonicalization and are
    verified against each child upvar relation and first observation.

  Evidence:
    release check plus the dedicated order fixture and all owner-forest tests
    are green; no Script route, capture ABI materialization, or closure
    publication changed.

current design gate
  RAW-SCRIPT-LAMBDA-CHILD-OWNER-LINEAGE0-T2-D0

  closed Accept-corrected. A direct cutover was unsafe while recursive forest
  construction issued IDs before child validation. The live S1 below removes
  that ordering fault; the real lexical positive fixture and exact old edge are
  now fixed for one atomic I0.

closed
  SEMANTIC-OWNER-RECURSIVE-CONSTRUCTION-TREE0-S1
  -> Function/Lambda production first builds and validates the full recursive
     shadow tree, then issues IDs and canonicalizes it; nested failure consumes
     no session owner ID. Existing owner-forest tests remain green.

current
  RAW-SCRIPT-LAMBDA-CHILD-OWNER-LINEAGE0-I0-R0
  -> admit only `local outer = 7; local f = fn() { outer }` and its no-capture
     companion through one Script child forest/ordered BindingRef receipt;
     selected lowering deletes its raw name-observer edge while existing closure
     publication remains the sole NewClosure/body-ID owner.

scheduled design gates after fresh census
  1. Control / Mutation / JoinIR / Exit, then Call/Object, allocation,
     Weak, Lambda, and Box
     -> each is a separate capability-family D0 chosen only from a fresh
        named production edge census; no AST-bucket batch is pre-authorized.
  2. `SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001`
     -> fixture-identity Complete set may only grow; R4 must retire, reown,
        or explicitly retain every remaining Deferred family.

closed
  SEMANTIC-SCRIPT-RECURSIVE-FOREST-ORDERED-CAPTURE0-D0
  -> Accept-corrected. Ordered BindingRef receipt is the only capture-order
     authority; its set must equal child upvars. A live Function/Lambda S0
     precedes Script T2; no forest iteration or raw name observer is capture ABI.

  RAW-SCRIPT-LAMBDA-CHILD-OWNER-LINEAGE0-D0
  -> closed Accept-corrected. Direct I0 was NoSafe until ordered receipts and
     pre-issue recursive construction landed; the narrow lexical T2 I0 now has
     one real fixture and retains the existing closure-publication owner.

  RAW-SCRIPT-NEXT-CAPABILITY-FAMILY5-D0
  -> Lambda selected for child-owner lineage D0. Box runtime crosses nested
     callable/constructor/metadata/runtime owners; other narrow capability
     families are already closed or belong to Call/Object. No I0 opens.

  RAW-SCRIPT-NEXT-CONTROL-FAMILY4-D0
  -> NoSafeSlice. ContextScope is already an exact diagnostic boundary;
     TryCatch and Throw are source-reserved outcome/control families; Arrow
     has no named MIR lowering owner. No I0 opens.

  RAW-SCRIPT-MATCH-ROOT-CONTROL-RECEIPT0-I0-R0
  -> Root lexical-core Match is Complete with co-sealed Scrutinee/Arm/Else
     coverage; the dispatcher now enforces exact structured-demand consumption.
     Existing owner keeps CFG/branch/PHI/result/type authority. Two focused
     tests cover selected/legacy parity and nested-Match Deferred behavior.

  RAW-SCRIPT-MATCH-CONTROL-MERGE-RECEIPT0-D0
  -> Accept-corrected. Root Match can seal Scrutinee, all Arm, and Else source
     coverage while the existing owner exclusively keeps CFG/branch/PHI/result
     authority. The first I0 is root-only; generic/nested Match is not enabled.

  RAW-SCRIPT-NEXT-COMPOSITIONAL-FAMILY3-D0
  -> Bounded static census selects MatchExpr only for CONTROL/MERGE D0:
     dispatcher already prepares MatchScrutinee, every MatchArm, and MatchElse
     for one existing owner. RecordUpdate remains shape/state-dependent;
     Index remains Builder static-data route-dependent; Call/Object remains
     header/effect/preflight-dependent. All three stay Deferred; no I0 opens.

  RAW-SCRIPT-ENUM-MATCH-SEALED-ROUTE0-D0
  -> NoSafeSlice. Existing lowering descends only EnumMatchScrutinee, but
     Program enum declarations still terminate at the selected unsupported
     diagnostic owner, while prelude enum inventory is outside prepared Program
     declaration facts. Mirroring mutable enum route preflight would create a
     second authority. A later enum family must first establish one inventory
     owner and an EnumDeclaration completion policy; no I0 is opened.

  RAW-SCRIPT-NEXT-COMPOSITIONAL-FAMILY2-D0
  -> Fresh static census rejects reopening GroupedAssignment, Loop/JoinIR,
     FieldAccess, and broad Call/Object. It selects EnumMatch because existing
     lowering has one exact scrutinee descent while arm syntax is route
     observation; the required next proof is metadata/preflight and diagnostic
     ownership, not a second resolver.

  RAW-SCRIPT-QMARK-PROPAGATION-RECEIPT0-I0-R0
  -> Root `QMarkPropagate(existing-safe operand)` now co-seals its exact
     QMarkOperand receipt with the Script source and reaches the existing
     control/result owner once. MIR/verification parity, RootLower diagnostic
     parity, fresh reuse, source projection, and the shared guard are green;
     safe QMark no longer reaches Deferred -> bare script_root(). Next blocker:
     fresh bounded responsibility-family census.

  RAW-SCRIPT-QMARK-CONTROL-RESULT0-D0
  -> Accept-B. Common resolved exits are statement-only and must not be
     generalized for QMark. Instead, a Script-only co-sealed propagation receipt
     proves an exact QMark expression targets the current Script owner while the
     existing QMark owner retains CFG, physical Return, runtime calls, and result
     policy. Real root `(await 42)?` MIR verification is green.

  RAW-SCRIPT-GROUPED-BINDING-REBIND-DESCENT0-D0
  -> NoSafeSlice. GroupedAssignmentExpr has an exact RHS source receipt and
     the shadow can identify its synthetic BindingRebind target, but the legacy
     raw route also requires `GroupedAssignmentTarget` source preparation and
     currently fails at `raw-invocation/expr-child-missing` before the existing
     assignment owner can establish parity. Widening the selected ledger hook
     alone would therefore be a new behavior, not a safe handoff.

  RAW-SCRIPT-BLOCKEXPR-PURE-DESCENT0-I0-R0
  -> ScriptLexicalCore now admits pure BlockExpr only through the shared shadow
     traversal. Its existing raw owner receives exact prelude/tail sources,
     lowers the prelude eagerly in source order then the tail once, and retains
     its existing escaping-exit preflight. Variable/Local and escaping-exit
     diagnostic parity are green; no new source authority exists.

  RAW-SCRIPT-LOOP-JOINIR-SEMANTIC-ADMISSION0-D0
  -> NoSafeSlice. `PreparedLocatedRawLoopChildEntryV1` seals exact condition
     and body receipts but deliberately drops them before the sole JoinIR
     planner receives raw AST. A Complete Script Loop would therefore create
     unused semantic/control authority; no I0 is opened.

  RAW-SCRIPT-FIELD-ACCESS-SEMANTIC-ADMISSION0-D0
  -> NoSafeSlice. The only existing `Receiver` source path is not a
    receipt-consuming FieldAccess contract: the owner selects existing-record,
    record-construction, record-literal/update, or dynamic property-call versus
    FieldGet routes from Builder type/origin state. Broad Script FieldAccess
    would bypass or discard sealed facts and can shift diagnostics. A future
    record-only field-read family needs its own source/result receipt boundary.

  RAW-SCRIPT-RECORD-SCHEMA-ADMISSION0-I0-R0
  -> one declaration-facts collection lends a positive-only schema view before
     the same product installs once in RootLower. Record declarations transfer
     while retaining their existing runtime owner; fully explicit known
     non-generic literals use sealed exact field receipts. Defaults and invalid
     forms stay Deferred. Focused record/schema/reuse parity is green.

  RAW-SCRIPT-RECORD-RESULT-TYPE0-I0-R0
  -> `publish_record_local_fields` now publishes successful `RecordValuePublish`
     as `Void`, matching the interpreter. The minimal legacy record Program
     finalizes and supplies the prerequisite parity fixture; schema/default,
     Script routing, and record publication remain unchanged.

  RAW-SCRIPT-RECORD-SCHEMA-ADMISSION0-D0
  -> Accepts a source-only seam: `PreparedNormalProgramDeclarationFactsV1`
     already derives record fields/defaults from Program without Builder access.
     Collect it once after CatalogSeal, expose only immutable schema demand,
     and move the same prepared product to RootLower for install. Future
     Complete closure is known non-generic RecordLiteral with every field
     explicit; all residual forms retain existing diagnostics.

  RAW-SCRIPT-RECORD-LITERAL-COMPOSITIONAL-CONTRACT-DESCENT0-D0
  -> NoSafeSlice. `RecordFieldValue(n)` receipts cover explicit fields, but
     the existing Record owner subsequently lowers omitted declaration defaults
     through the same port. Schema/default demand is unavailable before
     ScriptSemanticSeal, so a Map-style cutover would assign false provenance
     or exhaust receipts. Dynamic Deferred would be fallback. The prerequisite
     is immutable schema admission; RecordUpdate remains out of scope.

  RAW-SCRIPT-POST-MAP-LITERAL-CAPABILITY-CENSUS0-D0
  -> CheckExpr is already Complete: shared profile admission, exact
     `CheckItem(n)` receipts, the existing eager Select owner, fixture ratchet,
     and its old Deferred edge are all closed. RecordLiteral is the sole next
     candidate, requiring a contract/default-field D0 before any I0.

  RAW-SCRIPT-MAP-LITERAL-COMPOSITIONAL-MUTATION-DESCENT0-I0-R0
  -> selected Script Map values now receive exact `MapEntryValue(n)` source
     receipts through the structured child port. The existing Map owner remains
     the sole `MapBox` allocation, key emission, `MapBox.set` mutation, and
     type owner; unsupported values remain Deferred. The selected MapLiteral
     `Deferred -> bare script_root()` edge is zero.

  RAW-SCRIPT-POST-WEAK-REFERENCE-CAPABILITY-CENSUS0-D0
  -> Accepts MapLiteral only. Its semantic traversal already exists; exact
     `MapEntryValue(n)` receipts let the existing Map owner retain the
     allocation/mutation boundary without activating general MethodCall.

  RAW-SCRIPT-WEAK-REFERENCE-COMPOSITIONAL-DESCENT0-I0-R0
  -> selected Script Weak Unary now enters the existing unary child-source
     handoff and existing WeakRef emission owner. WeakRef type publication and
     pure-mode behavior remain there; an unsupported operand stays Deferred.
     The selected Weak `Deferred -> bare script_root()` edge is zero.

  RAW-SCRIPT-POST-BLOCKEXPR-CLOSURE-CAPABILITY-CENSUS1-D0
  -> Accepts Weak Unary only. The existing UnaryOperand receipt and WeakRef
     emission owner provide a complete source/operation boundary. ScopeBox and
     Using were already closed; broad BlockExpr remains NoSafeSlice.

  RAW-SCRIPT-POST-ARRAY-LITERAL-CAPABILITY-CENSUS0-D0
  -> BlockExpr has exact source receipts and a shared lexical traversal, but
     its proposed outer-Variable closure cannot preserve production parity:
     legacy lowering already rejects the shape at
     `[freeze:contract][script-lexical/variable-site]`. No partial
     BlockExpr activation lands; Local/Call/Weak/exit remain Deferred.

  RAW-SCRIPT-ARRAY-LITERAL-COMPOSITIONAL-ALLOCATION-DESCENT0-I0-R0
  -> selected Script ArrayLiteral is now a complete compositional allocation
     closure. The raw expression dispatcher creates exact `ArrayElement(n)`
     source receipts and the structured child port consumes each once; the
     existing array owner remains the only allocation, type, and publication
     owner. Map and Record remain Deferred. The selected ArrayLiteral
     `Deferred -> bare script_root()` edge is zero.

  RAW-SCRIPT-POST-BINDING-REBIND-CAPABILITY-CENSUS0-D0
  -> Accepts ArrayLiteral only. Its semantic traversal already exists; the
     live missing edge was exact ArrayElement source handoff into the existing
     raw array owner. Broad BlockExpr is not selected: nested Local changes
     existing legacy failure behavior. QMark, Loop, Map, Record, Lambda, and
     Box remain separate families.

  RAW-SCRIPT-ROOT-BINDING-REBIND-ADMISSION0-I0-R0
  -> only prior-Local Variable-target Assignment/CompoundAssignment receives
     a typed BindingRebind demand. The shared forest supplies the exact target
     BindingRef, and the existing raw lower remains the only operational
     owner; its returned ValueId updates the Script ledger only on success.
     Field/Index, grouped/nested assignment, and upvar stay Deferred. The
     selected Variable-target `Deferred -> bare script_root()` edge is zero.

  RAW-SCRIPT-POST-RETURN-CAPABILITY-CENSUS0-D0
  -> Accepts only the BindingRebind Mutation slice. QMark owns an
     expression-site conditional Return plus runtime calls and needs a
     CONTROL/RESULT D0; Loop needs a typed JoinIR route plan and stays
     Deferred. Assignment is safe only for prior-Local Variable targets:
     the shared forest already owns exact BindingRebind facts, while the
     existing raw lower retains operational completion.

  RAW-SCRIPT-ROOT-RETURN-EXIT-ADMISSION0-I0-R0
  -> only final-ordinal root `Return` receives a typed exit demand. The shared
     traversal preserves existing ReturnValue/ExplicitReturn facts and the
     existing value/void terminal owns all lowering. Non-final and nested
     Return stay Deferred, so no suffix reachability owner is introduced; the
     selected final-Return `Deferred -> bare script_root()` edge is zero.

  RAW-SCRIPT-POST-IF-CAPABILITY-CENSUS0-D0
  -> CheckExpr and its safe recursive closure are already Complete through
     the shared lexical traversal and existing source-demand owner; no new
     receipt or I0 exists. Final-root Return is the next bounded live edge.

  RAW-SCRIPT-IF-CONTROL-ADMISSION0-I0-R0
  -> exact `DirectIfStatement + ASTNode::If` work-plan receipts issue one
     typed root-control demand. The shared Script traversal resolves that
     root If and its existing child source paths; Complete retains the sole
     direct-If lowering terminal. Nested ScopeBox/TaskScope/FastMem If does
     not receive the receipt and remains Deferred. The selected old
     `If -> Deferred -> bare script_root()` edge is zero; retry/fallback is
     zero. Root-profile sequence containment now preserves the distinct
     Function/Lambda compact paths and the Script ProgramBody-rooted path.

  SEMANTIC-SOURCE-CONTAINER-PROFILE0-S0
  -> Sequence containment now derives direct body membership from
     `SemanticOwnerRootProfileV1`; ProgramBodyRoot -> ProgramBody(n) is valid
     only for Script, and Function/Lambda retain their exact roots. This fixes
     the verifier precondition only; Script If routing remains Deferred.

  RAW-SCRIPT-POST-OUTBOX-CAPABILITY-CENSUS0-D0
  -> `RAW-SCRIPT-IF-LEXICAL-STRUCTURED-CONTROL0-I0-R0` is NoSafeSlice:
     root `resolve_if` fails If-region control verification and a simple
     profile gate widens nested ScopeBox If. No I0 implementation landed.

  RAW-SCRIPT-OUTBOX-SEMANTIC-MATERIALIZATION0-I0-R0
  -> Complete Script source seals every exact Outbox BindingRef in source
     order; the raw source port consumes the existing Outbox emission receipt
     once and atomically records it in the request-local lowering ledger.
     Parser-valid one-or-more-name Outbox and ignored compatibility initializers preserve parity;
     selected Complete Outbox no longer reaches Deferred/bare script_root().

closed structural prerequisite
  RAW-SCRIPT-ROOT-NEUTRAL-LEXICAL-SHADOW-CUTOVER0-I0-R0
  -> the former Script visible-name mini-resolver, manual Local/Variable
     facts, and manual source-path reconstruction are already deleted.
     `ScriptSemanticLoweringState` is only the request-local BindingRef to
     ValueId ledger, not a second resolver.

closed
  OUTBOX-ORDERED-EMISSION-RECEIPT0-S0
  -> the existing raw Outbox owner now returns every source-ordinal local
     ValueId in one ordered receipt while its sole production caller consumes
     the unchanged final statement value; Void/local/metadata order is intact

  RAW-SCRIPT-TASK-SCOPE-LEXICAL-PREFLIGHT0-I0-R0
  -> lexical normal-completion TaskScope reaches Complete through the shared
     traversal; the existing preflight remains sole early-exit authority and
     the existing raw owner remains sole push/body/pop completion authority
  -> `TaskScopeBodyRoot` transport hands leaf nodes sibling `TaskScopeBody(n)`
     sites; selected/legacy parity, early-exit Deferred/reuse, pointer, and
     shared cutover guards green

  RAW-SCRIPT-CONTEXT-SCOPE-DIAGNOSTIC-BOUNDARY0-I0-R0
  -> `ContextScope + DirectPortAwareExpression` now seals an exact existing
     diagnostic receipt and reaches Complete without observing value or body;
     the raw context-scope dispatcher remains the sole RootLower owner
  -> nested missing names still lose to the existing context-scope diagnostic;
     selected/legacy parity, fresh reuse, pointer, and shared cutover guards green

  RAW-SCRIPT-NOWAIT-LEXICAL-ASYNC-BINDING0-I0-R0
  -> lexical-safe Nowait now uses the shared traversal; the existing async
     owner remains the sole FutureNew/type/slot/variable-map authority and the
     request-local ledger records its exact canonical binding
  -> Nowait/await selected-legacy parity, unsafe operand Deferred, transport,
     pointer, and shared cutover guards green

  RAW-SCRIPT-SCOPEBOX-LEXICAL-STRUCTURED-SCOPE0-I0-R0
  -> lexical-safe ScopeBox now uses the shared traversal and the existing raw
     ScopeBox owner; `ScopeBodyRoot` remains a region receipt while transport
     hands inner nodes the canonical sibling `ScopeBody(n)` leaf site
  -> ScopeBox/nested ScopeBox selected-legacy parity, lexical non-leak, disabled
     control Deferred, transport path, pointer, and shared cutover guards green

  RAW-SCRIPT-POST-ZERO-DEMAND-CAPABILITY-CENSUS0-D0
  -> selected ScopeBox lexical structured scope: shared traversal already owns
     exact lexical scope paths and raw ScopeBox lowering remains its terminal

  RAW-SCRIPT-THIS-DIAGNOSTIC-BOUNDARY0-I0-R0
  -> bare `This + DirectPortAwareExpression` now seals an exact typed existing
     unsupported-diagnostic boundary; the raw dispatcher remains RootLower owner
  -> selected/legacy failure and fresh-reuse parity, pointer guard, and shared
     cutover guard green; nested or statement-wrapped This remains Deferred

  RAW-SCRIPT-USING-TRANSPARENT-RUNTIME-COMPLETION0-I0-R0
  -> top-level Using now seals an exact transparent receipt and retains the
     existing Void terminal, preserving `1; using` selected/legacy parity
  -> focused demand-window and semantic-source tests, pointer guard, and
     shared cutover guard green

  RAW-SCRIPT-ROOT-NEUTRAL-LEXICAL-SHADOW-CUTOVER0-I0-R0
  -> already closed by `5b963969b4`: sparse Script input reaches the shared
     root-neutral shadow traversal and the 695-line manual lexical resolver is
     deleted; only the BindingRef-to-ValueId lowering ledger remains

  RAW-SCRIPT-BARE-ME-DIAGNOSTIC-BOUNDARY0-I0-R0
  -> bare `Me + DirectPortAwareExpression` now uses a typed receiver-absent
     diagnostic boundary; `build_me_expression` remains the only RootLower
     diagnostic owner, while recursive/statement-wrapped Me stays Deferred
  -> focused Script semantic source tests, pointer guard, and shared cutover
     guard green

  RAW-SCRIPT-FASTMEM-STRUCTURED-SCOPE0-I0-R0
  -> `FastMemRegion + DirectFastMemRegion` is Resolved only through a
     recursively lexical-safe body; existing FastMem lower remains owner
  -> focused semantic, direct-owner, transport, pointer, and shared guards green

ordered after fresh evidence only
  1. complete the current ContextScope diagnostic-boundary row
  2. census exactly one Deferred family; TaskScope and Outbox are named
     design gates, not preselected implementation work
  3. activate one responsibility capability per row, with Control / Mutation /
     JoinIR / Exit before Call/Object, allocation, Weak, Lambda, and Box
  4. keep Program work-plan ownership below the file-size boundary by extracting
     a neutral demand-window module only when the selected row needs it
  5. R4 consumes the live fence registry above; every item must retire, reown,
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
