---
Status: Active workstream
Date: 2026-08-02
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

`CURRENT_STATE.toml` is the pointer SSOT. Git history owns detailed landed
diffs and proof transcripts; this card keeps the live task and boundaries.

## Parked follow-ups from external architecture review

The review supplied on 2026-08-18 was based on old HEAD `8237906da0`; the
current A0 StringEquals/1 DesignOnly rejection is already landed at the live
HEAD. It does not reopen A or change the B blocker. Five bounded follow-up tasks
are recorded for later selection only:

| task | evidence now | acceptance when selected |
| --- | --- | --- |
| `LOOP-GENERIC-PREFLIGHT-CONSUME-SHAPE-D0` | `PreparedGenericG0PhysicalEmitterSessionPreflightV1` has borrowed getters plus `take_layout`/control extraction; canonical consumer is test-only and production callers are 0 | choose one opaque session consumer or typed duplicate rejection; no re-pairing and no Generic production switch |
| `LOOP-CALLABLE-OPAQUE-DEMAND-CONSUME-D0` | `PreparedCallableOperationDemandV1::consume<R>` has two test-only direct callsites; wrapper `into_parts` has no caller | census callers, then hide tuple-like escape behind one private/opaque aggregate; preserve source co-seal |
| `LOOP-COMMON-DISPATCHER-ENTRY-CENSUS-D0` | target-explicit and block-receipt dispatcher functions share leaf emitters; target path is mechanical, block path has tests | name one keeper, classify test-only callers, and retire/restrict duplicates at production cutover |
| `STATE-PARSER-INTEGRITY-I0` | shell pointer guard was green while standard `tomllib` rejected missing commas; the array is now repaired | add standard-parser validation to the reusable state guard and make parse failure blocking |
| `CURRENT-POINTER-COMPACTNESS-D0` | active lane drift was corrected; pointer is 115 lines and still contains bounded status/tail | only if selected, compact current pointers without copying history or changing semantic authority |

All four are `design_stop` follow-ups. They may not add a `Verified*`/
`Prepared*` receipt, production selector, fallback/retry, or physical effect
until the current TextEq B/C boundary is explicitly closed.

## Closed chronology (archived)

The callable source ledger, SyntaxFacts/SourceMap, root-neutral traversal,
Recipe/JoinSig co-seals, canonical finish, physical canaries, and retired raw
route experiments are closed. Their detailed Decisions, counters, and proof
transcripts live in ParentHistory/git history and the owning investigation
cards; they are not current scheduling authority.

Stable boundaries retained from that work:

```text
source/resolver -> Facts -> Recipe/JoinSig -> Verify
  -> one physical owner -> DraftSeal -> Collector -> Atomic Publish

closed routes never authorize a new production caller;
NoSafeSlice remains a development state;
legacy retirement requires caller-zero evidence.
```

The current Dynamic AOT activation is tracked only by CURRENT_STATE.toml and
the active investigation card. Do not restore closed chronology here.

## Protected-region control-state design

Decision: share policy and transient state, not physical exit writers.

```text
cleanup policy
  -> one immutable snapshot

TryCatch transient control
  -> one total typed state

sealed operation
  -> its existing physical consumer exactly once
```

TryCatch is a protected-region transaction, QMark is a conditional propagation
recipe, Throw is a terminator, raw Return owns defer completion, Match may use
CorePlan, and canonical Function Return is committed by DraftSeal. Combining
them into one physical terminal would create a second region/JoinIR planner.
The repository-wide target is therefore not one Return writer; it is one
physical consumer for each sealed operation, with no retry or fallback.

closed — RAW-CONTROLBODY-UNLOCATED-PORTAL-RETIRE0-R0 (RET0)
  The unreachable unlocated portal, its dead Lambda classifier arm, fabricated
  test construction, and false R4 transport claims are deleted. Lambda remains
  located; raw/reference capture retains its named operation; CallObject is the
  sole unlocated recursive portal. Focused transport/Loop tests and shared
  R4/current-pointer guards are green.

closed — CONTROL-RESULT-CLEANUP-POLICY-SNAPSHOT0-S0
  -> cleanup policy is captured at selected normal ingress or explicit raw
  TryCatch ingress, then owned by `PreparedRawTryCatchV1`; cleanup lowering no
  longer reads the environment. Snapshot/region tests are green.

closed — RAW-PROTECTED-REGION-TRANSIENT-STATE0-S1
  -> `ProtectedRegionTransientStateV1` now owns the complete return-defer and
  cleanup vector. Function and TryCatch transactions capture/restore that one
  value; success restoration and failure partial state remain covered by tests.

closed — RAW-RETURN-DEFER-INVARIANT0-R0
  -> active return defer is now a valid-only state with one slot/target
  destination. The old active-with-missing-destination direct Return fallback
  is a contract rejection; ordinary Return and valid defer remain unchanged.

closed — CONTROL-RESULT-SOURCE-DEMAND-CONTRACT0-D0 (NoSafeSlice)
  -> final root Return remains Complete; nonfinal Return needs a terminated
  suffix contract, Throw lacks a located child role, and TryCatch needs a
  first-catch-only protected-region contract. All retain their operation owner.

closed — SCRIPT-SEMANTIC-SOURCE-PACK-EXTRACTION1-S0
  -> `VerifiedScriptSemanticSourceV1` remains the sole live facade; stable
  source/forest/projection ownership and retained boundaries now live in two
  private packs. The facade fell from 795 to 637 lines without surface change.

closed — MIRBUILDER-CALLABLE-LAMBDA-GUARD-SCOPE0-P0
  -> the stale global text-order assertion is replaced by scoped callable-port
  and Lambda-dispatch proofs. Production and capture order are unchanged.

closed — SCRIPT-SEMANTIC-OPERATIONAL-DEMAND-PACK-EXTRACTION1-S1
  -> Record/Enum/QMark/Match receipt sealing now has one private pack and the
  standalone EnumMatch seal module is deleted. Complete admission and lowering
  are unchanged.

closed — SCRIPT-SEMANTIC-LOWERING-PROJECTION1-S2
  -> one immutable projection now co-seals core facts and both receipt packs;
  the live facade delegates lowering-state creation and no longer reconstructs
  forest facts, source paths, or capture receipts after semantic sealing.

closed — SCRIPT-SEMANTIC-LOWERING-LOAN-CUTOVER1-I0-R0
  -> Complete now consumes its verified source once, moves the co-sealed
     projection into the request ledger, and deletes copied receipt maps and
     staged install APIs; source transport and admission are unchanged.
closed — SCRIPT-ROOT-ADMISSION-ISSUER-ONE-MATCH0-S3
  -> witness `issue -> new` is now one private semantic decision; operational
  classification and invariant re-projection remain separate owners.

closed — SCRIPT-ROOT-RESOLVED-DISPATCH-EXTRACTION0-S4
  -> resolved root-demand dispatch is private; recursive traversal remains one shared matcher.

closed — MIRBUILDER-R4-OPERATION-PARTITION-BOUNDARY0-D0 (Accept): shared occurrence identity only; residual R4 ownership stays operation-local and the shared scheduler is rejected.
closed — JOINIR-LOOP-ROUTE-SELECTION-PHYSICALIZATION-SPLIT0-D0 (NoSafeSlice): candidate selection remains post-effect fallback, composers are physical, and no logical recipe consumer exists; do not reopen a renamed Loop slice.

## Active task pointer

The sole current row is `CURRENT_STATE.toml.current_execution_row`. The active
JoinIR contract and ordered convergence map live in
`design/joinir-loop-selfhost-recipe-pipeline-ssot.md`. Closed route-local provenance
records below are evidence only and must not schedule another route.

Current decision and execution brief:

This workstream is a historical convergence ledger, not a second live
pointer. The sole current decision is read from:

```text
docs/development/current/main/CURRENT_STATE.toml
  -> current_execution_row
  -> current_blocker_token
  -> latest_card_path
```

For every active lane, resolve the card and row only from `latest_card_path`
and `current_execution_row` above. `current_blocker_token` describes the stop
condition for that selected row; it never selects a different task. This
historical ledger does not mirror those values. Older S6C/S6D/S6G briefs and
route chronology remain evidence in git history or their owning cards; they
do not schedule a new route from this workstream.

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

## R4 active fence registry

The sole R4 data authority is
`tools/checks/manifests/raw_public_cutover_caller_manifest_v1.json::r4_fences`.
It records stable ID, kind, exact surface, source/fixture/guard evidence,
release condition, and dependency targets. This workstream intentionally does
not copy those rows; source-anchor evidence does not claim runtime parity.

`NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001` is closed. Test-only
`LegacyChildDraftAdmissionV1` fixtures remain; nested-method production now
uses `PreparedNestedBoxMethodSourceV1` and direct legacy-symbol completion.
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
selected design stop
  SCRIPT-ORDINARY-DIRECT-CALL-PREFLIGHT-RECEIPT-D0
  -> decide whether the existing raw ordinary-call preflight decision can move
     into one source-bound Script admission lineage without reclassification.
     No I0 is authorized until exact target, ordered arguments, arity, exact-I64
     result/header, ordinary decision, and cohort close before Builder effect.

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

closed historical design gate
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

closed historical
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

ordered after B-prime correction
  1. M7-S2-A caller-zero LoopTrue branch-exit JoinSig closure and M7-S3 S0/S1/S2 reference closeout are closed with resolver-owned identity/frame receipts and typed caller-zero rejects
  2. S2A is closed as one parsed nested-IfThen carrier shape, `cfg(test)`-only; reference closeout is recorded. Parent D2 stays unresolved and no production issuer/adapter/selector/route switch is authorized.
  3. D1, D2-S1, D2-S2, D3-S0, D2-S3, D2-S4, D2-S5-S1, D3-S1-S1, D3-S1-S2, and D3-S2-S0 are cfg(test)-only closed; D3-S2 remains a typed-provenance handoff design stop with no production issuer/selector/route authority
  4. current chain: `CallableContract(query)` -> ordered Box/parser parity -> declared instance contract -> general body source -> selected Query body source -> FunctionOwner -> body Facts -> conformance -> declaration-first target -> source-bound CallSlot; old contract->target->body wording is historical. Then M8/M9, semantic co-seal/JoinSig transfer, control coverage, M10b, Generic R1, M11/M12.
  5. run `REPO-FINAL-CONVERGENCE-AUDIT0-G0` from the repository cleanup SSOT; do not close R4 until its pipeline/root/role/context/pointer/evidence/docs matrix is green
  6. keep every source/check file below 800 lines; no universal raw ingress, Script-only/raw-only resolver, compatibility adapter, or AST reconstruction
  7. R4 consumes the live fence registry above; every item must retire, reown, or be explicitly retained before final conformance

R4
  MIRBUILDER-R4-FINAL-CONFORMANCE0-C0 after all active rows have exact
  retire/reown/retain decisions.
After final-pipeline Complete only: refresh missing-feature/Home readiness,
resume OWNERSHIP-HOME-RESUME-D0, then select later language features.
```
## Parked
```text
source-level Home ownership and unimplemented language features until the
repository-wide final pipeline is Complete; .hako selfhost MirBuilder/parser
migration and post-Loop root/current-state/design-registry cleanup follow their
owning SSOT task orders; new language semantics and default Raw/Canonical
cutover remain parked before final conformance.
```
