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

## Callable single-loop source ledger S1

`GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-LEDGER-S1` is closed caller-zero:
`CallableSemanticSourceLedgerView` exposes typed rows and resolver-issued Loop
membership/frame identity. No AST/ValueId copy, Loop policy, Recipe, physical,
or Builder/MIR caller was added; focused tests are green. The D0 authority is
closed; the current execution row is caller-zero `RESOLVER-SYNTAX-FACTS-S1`.
Its syntax observer must publish nine rows plus one prefix boundary before
MAP-S1; Recipe, physical, production, retry/fallback, and deletion stay closed.

## Root-neutral semantic foundation

Closed. One private traversal is the Function/Lambda and selected-Script
lexical authority. Function/Lambda use dense roots; Script uses the sparse
`ProgramBody(original ordinal)` window. The former Script visible-name map,
recursive mini-resolver, manual Local/Variable facts, and path reconstruction
are deleted. Complete issues one forest/projection only after total coverage;
Deferred issues neither and preserves RootLower diagnostics once. Git history
and the shared guard own the detailed proof.

### Recovered WIP order

```text
closed — RAW-SCRIPT-NEXT-NAMED-FAMILY0-D0 (NoSafeSlice)
  Call/Object needs header/type/origin preflight; Loop drops receipts before
  JoinIR; EnumMatch needs external inventory; GroupedAssignment needs a second
  target demand. No safe I0 exists. Next is Call/Object boundary design only.

closed — RAW-SCRIPT-CALL-OBJECT-OWNER-BOUNDARY0-D0 (NoSafeSlice)
  FunctionCall, indirect Call, MethodCall, New, Field/Index, and RecordUpdate
  each combine preflight route authority with operation lowering. No standalone
  I0 exists. Only catalog-resolved ordinary FunctionCall merits a new D0.

closed — RAW-SCRIPT-DIRECT-CALL-CATALOG-RECEIPT0-D0 (NoSafeSlice)
  A callable-index loan cannot exclude weak/extern/Brand/TypeOp/Math/FastMem
  or replace RootLower header observation without a second classifier.
  Next asks only whether the existing preflight can publish one SSOT receipt.

closed — RAW-SCRIPT-CALL-PREFLIGHT-CLASSIFIER-SSOT0-D0 (Decision B, R4 retained)
  The semantic unit is not a callable name or catalog target: one preflight
  owns special-name classification plus header/environment observation before
  ordinary call descent. Retain FunctionCall as one operation authority through
  R4; the Deferred floor is `FunctionCallPreflightAuthority`. Release requires
  either one all-route preflight recipe or a named final retained operation
  boundary. Do not run a fourth Script-call census.

closed — RAW-SCRIPT-GROUPED-BINDING-REBIND0-I0-R0 (T2)
  The earlier GroupedAssignment NoSafe premise is stale: Script root
  BindingRebind receipts and the BindingRef -> ValueId ledger now exist.
  Accept only `GroupedAssignmentExpr { lhs, rhs }` where `lhs` resolves to a
  prior Script Local and `rhs` is in the existing Complete lexical closure.
  `GroupedAssignmentTarget` is a synthetic, non-descended BindingRef receipt;
  `GroupedAssignmentValue` is the one physical child demand. Route it through
  the existing `drive_variable_assignment_v1` owner and rebind the ledger only
  after raw success. Field/index/nested targets and every Call/Object family
  remain Deferred. Eligible GroupedAssignment ->
  Deferred -> bare `script_root(())` reachability = 0. No ABI, publication,
  ValueId owner, fallback, retry, or raw/reference change.

closed — RAW-SCRIPT-ENUM-DECLARATION-COMPLETION0-I0-R0 (T2)
  EnumDeclaration is a typed Program transfer plus one existing Void
  completion; declaration facts remain the sole inventory producer/installer.

closed — RAW-SCRIPT-ENUM-INVENTORY-VIEW0-D0 (NoStandaloneRow)
  A view alone had no real Script consumer; AST-only EnumMatch proof is
  forbidden. The producer audit below corrected that premise.

closed — RAW-SCRIPT-ENUM-VARIANT-PRODUCER0-I0-R0 (T2)
  The one declaration-facts scan now proves only final non-generic
  `Type::Variant(args*)` routes with exact arity; Complete co-seals exact
  `CallArgument` receipts and invokes the existing enum emitter once.
  Ordinary/invalid/raw/reference FromCall remains Deferred; selected/legacy
  `VariantMake` parity is fixed by the shared fixture ratchet.

closed — SCRIPT-SEMANTIC-COMPLEXITY-CONSOLIDATION0-S0
  Receipt core/packs, source-vs-invariant Script outcomes, sealed root-demand
  issuance, test-family split, current pointer, and full Complete identity floor
  are now compact; admission, lowering, diagnostics, and raw/reference are unchanged.

closed — ENUM-MATCH-SOURCE-OWNER-FILE-SPLIT0-S0
  ScopeBox preparation/lowering now has its private sibling; enum-match owner
  is below the file boundary with raw behavior and diagnostics unchanged.

closed — RAW-SCRIPT-ENUM-MATCH-DIRECT-SCRUTINEE0-I0-R0 (T2)
  One shared direct enum preflight now serves raw lowering and the borrowed
  declaration-facts view. Complete co-seals EnumMatchScrutinee only; arms
  remain raw-owner observations. Selected/legacy parity is green.

closed — SCRIPT-ROOT-ADMISSION-WITNESS0-S0
  Root-demand AST shape/disposition proof now has one private witness; the
  demand window retains ordinal coverage only. Eligibility, route, diagnostic,
  raw/reference, and lowering behavior are unchanged.

closed — RAW-SCRIPT-NEXT-NAMED-FAMILY4-D0 (NoSafeSlice)
  No standalone Script family has an atomic old-edge deletion. Loop remains
  JoinIR-fenced; FunctionCall, Field/Index/New, and Box runtime retain their
  all-route R4 operation owners; Lambda capture/publication remains fenced;
  TryCatch/Throw and non-final Return retain their control/result owners.
  `SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001` remains the single Deferred
  terminal. Do not reopen these surfaces under a renamed I0.

closed — MIRBUILDER-R4-RESIDUAL-RECONCILIATION0-D0
  The generic Deferred terminal is real, but its R4 manifest named only
  FunctionCall. Loop, CallObject siblings, Box runtime, and control/result
  residuals must be mechanically enumerated before `unregistered = 0` can be
  claimed. No cleanup I0 is available at this seam.

closed — SCRIPT-DEFERRED-RESIDUAL-REGISTRY0-S0
  The existing root-admission witness co-issues a production-owned, root-only
  residual sidecar from the already-classified admission and exact AST shape.
  It is observability/R4 ownership only: semantic disposition, resolver
  traversal, raw lowering, route selection, and raw/reference are unchanged.

closed — SCRIPT-DEFERRED-RESIDUAL-MANIFEST-RATCHET0-S0
  The root-only sidecar and R4 manifest now share a table-driven guard. The
  old FunctionCall singleton assertion is gone; every registered residual has
  exact admission/shape/family, owner/release fields, and a fixture anchor.
  Nested/profile failures and Lambda capture remain deliberately outside it.

closed — MIRBUILDER-R4-RESIDUAL-CONFORMANCE1-D0
  The mechanically anchored registry is exact for unconditional root-admission
  residuals. Profile-dependent Deferred shapes are not falsely counted as
  root entries: they require their own exact D0. The Lambda fence now means
  Deferred/raw/reference Lambda capture/publication only; selected lexical
  Lambda already has its sealed child owner and ordered receipt. No generic
  cleanup I0 exists at this seam.

closed — RAW-SCRIPT-INDEX-WRITE-MUTATION-DESCENT0-I0-R0 (T2)
  Ordinary `Assignment(Index(Variable(prior Local Array), index), value)` now
  uses one construction-local Array-initialized binding proof before child
  descent, then the shared `IndexWrite` resolver fact and existing exact
  `IndexTarget` / `IndexSubscript` / `AssignmentValue` handoff. The raw index
  mutation owner remains the sole operational owner. Map/scalar/rebound locals,
  CompoundAssignment, FieldWrite, Index read, Loop, Call/Object, Lambda, and
  Box runtime remain Deferred. The selected prior-Local Array form no longer
  reaches Deferred-to-bare-`script_root(())`; its fixture identity is ratcheted.

closed — RAW-SCRIPT-NEXT-NAMED-FAMILY5-D0 (NoSafeSlice)
  Fresh production census found no standalone old-edge deletion after IndexWrite.
  Every remaining root surface belongs to Loop/JoinIR, a Call/Object operation
  owner, Lambda capture/publication, Box runtime, or an explicit R4 residual;
  none is reopened under a renamed small I0.

closed — SCRIPT-SEMANTIC-RATCHET-COVERAGE0-S0
  The Script Complete identity floor now includes every already-closed
  root-level family with a focused parity fixture. Behavior, admission,
  lowering, and Deferred reasons are unchanged; the shared manifest guard
  rejects any later fixture-identity regression.

closed — MIRBUILDER-LOOP-JOINIR-SOURCE-ERASING-TERMINAL0-D0 (R4 retain/rehome)
  `PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1` retains
  exact receipts only until it passes raw condition/body to
  `lower_loop_or_freeze_v1 -> try_cf_loop_joinir -> route_loop`. The existing
  verified generic loop plan is test-only and callable-result-specific, not a
  selected Script consumer. The terminal stays under
  `RAW-LOCATED-LOOP-ROUTE-SOURCE-HANDOFF-SUNSET-001`; release requires one
  all-route located plan from the existing planner/registry, including every
  dynamic child/claim schedule, to replace that terminal once.

closed — RAW-SCRIPT-LAMBDA-DEFERRED-CAPTURE-PUBLICATION0-D0 (R4 retain/rehome)
  Deferred has owner/forest/projection = 0 and raw/reference has no Script
  lineage, so neither has `forest.child_at` or an ordered BindingRef capture
  receipt. The live `RawLambdaLexicalObservationV1 -> variable_map snapshot ->
  PreparedRawLambdaClosureEmissionV1` lifecycle remains the named owner under
  `RAW-LAMBDA-CHILD-OWNER-SOURCE-LINEAGE-SUNSET-001` without an unlocated
  transport portal.
  Release requires one all-route lineage/capture product (or one final named
  Lambda operation boundary) that lets the existing emitter delete the raw
  observer atomically.

closed — MIRBUILDER-R4-RESIDUAL-FINAL-CONFORMANCE0-D0 (incomplete inventory)
  The root-only registry is mechanically sound but cannot claim all R4
  surfaces: raw/reference Lambda capture-publication and nested/profile
  Deferred paths are outside its root AST vocabulary. Final conformance is
  therefore not claimed.

closed — SCRIPT-NONCOMPLETE-PROFILE-RESIDUAL-LAMBDA0-S0
  The manifest/guard now fixes the raw/reference Lambda capture-publication
  operation, direct recursive compatibility, release condition, and focused
  raw dispatcher fixture. No route or semantic behavior changes.

closed — SCRIPT-NONCOMPLETE-PROFILE-RESIDUAL-CENSUS0-D0
  Nested CallObject and Lambda-leaf profile gates are reachable. They cannot
  share one R4 operation entry: CallObject and control/result keep
  shape-specific owners, while Lambda leaf composes its capture owner with a
  child operation. Final R4 conformance remains unclaimed.

closed — SCRIPT-PROFILE-RESIDUAL-OBSERVATION0-D0 (NoSafeSlice)
  Profile gates collapse to unbranded Deferred before any source product can
  survive. A receipt needs a new Deferred lifecycle product; Lambda leaf also
  loses its parent boundary and cannot infer a child operation residual.

closed — NESTED-BOX-FUNCTION-RELATIVE-SOURCE-CONTRACT0-D0 (Accept A′)
  The two nested static/instance issuer frames are self-created from a located
  outer Box context; no other production unlocated constructor exists. One
  function-relative method source product can retain the batch-owned method
  key, exact FunctionDeclaration/body, and legacy symbol/physical arity.
  It does not need a global Box/method source-path projection.

closed — NESTED-BOX-METHOD-SOURCE-EXTRACTION0-S0
  Both live nested static/instance method lowerings now delegate to one private,
  behavior-neutral owner. Existing collector admission, unlocated transport,
  headers, normalization, diagnostics, and raw/reference behavior are unchanged.

closed — NESTED-BOX-FUNCTION-RELATIVE-SOURCE-CONTRACT0-I0-R0
  The two nested-method batches now issue one owned method-key/body input;
  a located parent seals `NestedBoxMethod` at `FunctionBody`. Both legacy
  admission issuers and the `NestedBoxAdmission` portal are deleted.

closed — RAW-SCRIPT-NEXT-NAMED-FAMILY6-D0 (NoSafeSlice)
  Fresh bounded audit confirms that all remaining Script surfaces are already
  owned by Loop/JoinIR, all-route Call/Object, deferred/raw/reference Lambda
  capture-publication, Field/Index operation routes, or Box lifecycle. They
  remain under their named R4 fences; no renamed Script I0 may reopen them.

closed — MIRBUILDER-NEXT-NONSCRIPT-RESPONSIBILITY0-D0 (NoSafeSlice)
  Default normal App already owns verified Main. The remaining raw static-Main,
  Loop/JoinIR, VM bridge, JoinModule observer, and generic legacy child port
  are all-route R4 fences, not narrow production replacements.

closed — SCRIPT-R4-RATCHET-EVIDENCE-EXTRACTION0-S0
  The shared guard now delegates Script Complete/Deferred/profile/residual
  floors to one manifest-backed helper; inline evidence authority is deleted.

closed — MIRBUILDER-R4-FENCE-REGISTRY-SSOT0-S0
  All seven live R4 fences are first-class manifest rows with kind-specific
  source/fixture/guard evidence and dependency checks. The handwritten table
  is deleted; no route or runtime behavior changed.

closed — MIRBUILDER-R4-FENCE-EVIDENCE-MATURITY0-D0 (NoSafeSlice)
  None of the seven R4 fences has an all-route replacement owner: source and
  fixture anchors prove presence only, never atomic route replacement parity.

closed — MIRBUILDER-FINAL-R4-RETENTION-POLICY0-D0 (Decision)
  R4 named retains are migration inventory only. `MIRBUILDER-FINAL-PIPELINE-v1`
  requires every active fence to retire or complete an all-route rehome. A final
  named operation may remain only behind one typed all-route product, with the
  generic compatibility portal, source-erasing terminal, and old production edge at zero.

closed — MIRBUILDER-R4-REHOME-ORDER0-D0 (NoSafeSlice)
  No active fence can yet name an all-route typed product and atomic generic
  old-edge deletion. Required-order DAG: VM bridge policy; normalized-shadow
  Loop Recipe/CorePlan; located Loop plan consumer; raw static-Main source and
  entry materialization; Lambda lineage; CallObject preflight; then their
  transport/substrate-derived closeouts.

closed — MIRBUILDER-VM-BRIDGE-RETIREMENT-POLICY0-D0 (Decision)
  Retire the default-off explicit JoinIR VM bridge; do not migrate it to the
  `.hako` interpreter. Ordinary VM remains the only MIR execution owner.

closed — MIRBUILDER-VM-BRIDGE-RETIRE0-I0-R0
  The runner bridge call, target table, bridge-only environment flags, dispatch,
  execution routes, and A/B tests are deleted. Structured JoinModule-to-MIR
  conversion is rehomed to `join_ir_to_mir`, its one existing normalized consumer
  remains, and the VM-bridge plus shared-substrate R4 fences are retired.

closed — JOINIR-LOOP-RECIPE-COREPLAN0-D0 (NoSafeSlice)
  Nineteen registry routes plus two normalized-shadow shapes still use ordered
  operational decline, physical CorePlan composition, name/ValueId carrier
  recovery, a phase-only converter snapshot, and shared-driver fallback. No
  current product can prove all-route logical binding coverage and delete both
  mutation routes atomically.

closed premise sequence — JOINIR-LOOP-LOGICAL-INTERFACE0-D0 /
JOINIR-LOOP-ROOT-NEUTRAL-BINDING-SNAPSHOT0-D0
  Both stops correctly rejected name/ValueId recovery, but their atomic caller
  set over-counted a profile-blind helper as raw/reference. The exact located
  source, missing logical binding product, and shared suffix mutation findings
  remain valid; the raw/reference premise is superseded below.

closed — JOINIR-LOOP-ALL-ROUTE-PREMISE-RESET0-D0 (Decision B-prime)
  Explicit raw-vm-reference already owns one typed support profile, owned AST,
  and source-bound root. NarrowV1 rejects Loop while projecting the body recipe,
  before physical Builder open, so its Loop reachability is zero. The generic
  `RawLegacyChildLoweringPortV1` is not a compilation profile; static provenance
  found no repository production Loop caller for that port. Do not add a new
  raw/reference profile or universal semantic ingress.

closed premise sequence — normalized-shadow coverage / grammar / receipt
  Finite concrete-shape coverage was false: six grammar families include
  `(Assignment | Local)*; Break`.  The bounded audit found one PlanBox/executor
  gap, five ordinary-route overlaps, and suffix-to-direct retry.  Exact
  eligibility still used `variable_map`, dummy `ValueId`s, and operational
  JoinIR emission, so a passive receipt would merely rename the second physical
  resolver.  The detailed temporary manifest evidence is retired with the
  mutation fence; do not rebuild it or a name-set facade.

closed — JOINMODULE-NORMALIZED-SHADOW-MUTATION-RETIRE0-I0-R0
Change:
  Retire the duplicate dev-only normalized-shadow Loop mutation authority
  instead of migrating it.  Delete the direct and block-suffix entries, suffix
  port capability, Plan/Execute module, and phase-only `Normalized -> Structured`
  bridge.  Keep the ordinary recipe-first Loop route and the non-mutating
  normalized-shadow observer, including its independently tested JoinModule
  builders.

Contract:
  T2 deletion only: add no semantic ingress, route, fallback, environment
  toggle, or replacement adapter.  `joinir_dev` no longer changes Loop
  mutation ownership; raw/reference and Script admission remain unchanged.
  Shared JoinIR-to-MIR conversion, `JoinIrPhase`, `JoinInlineBoundary`, and the
  if-only comparison observer remain because they have independent consumers.
  Reuse shared guards; every touched source/check file remains below 800 lines.

Done:
  `try_normalized_shadow`, `try_lower_loop_suffix`, `NormalizationPlanBox`,
  `NormalizationExecuteBox`, the suffix retry edge, and the phase-only bridge
  have zero definitions/callers.  Ordinary
  recipe-first Loop tests remain green with `joinir_dev` OFF and ON.  The
  non-mutating shadow observer still compiles and its non-Loop evidence remains
  green.  The R4 fence row and stale suffix proof capability are removed.

Stop:
  Stop if removing either mutation entry changes the default route, requires a
  new compatibility branch, removes an independently consumed converter or
  observer, or cannot preserve ordinary Loop diagnostics/results without
  retrying the retired route.

closed — JOINIR-LOOP-MISSING-TRANSIENT-TYPE-OWNER0-D0 (NoSafeSlice)
  Call-result/`BindingRef` truths have no production loan; String -> `ValueId`
  -> `type_ctx` is wrong. The five-case failure predates this retirement.
closed — CALLABLE-RESULT-BINDING-REPRESENTATION-INGRESS0-D0 (NoSafeSlice)
  Activation has no production issuer; cataloged lowering keeps only key,
  symbol, arity, and lineage. The disconnected observer/session is not an
  acceptable second resolver.
closed — NORMAL-CALLABLE-SEMANTIC-SOURCE-LOAN0-D0 / SOURCE-INVENTORY0-S0
  Catalog owns exact top-level/three-terminal Box sites; Main stays transferred.
closed — NORMAL-CALLABLE-SEMANTIC-SOURCE-LOAN0-I0-R0
  One selected lifecycle issuer traverses the complete callable batch before
  owner issue and co-seals exact catalog key, Program site, forest, and
  projection. Complete consumes typed loans exactly once at top-level, static,
  and plain-instance terminals; missing, duplicate, or unconsumed loans reject.
  FunctionCall defers before children. Script non-plain instance Box makes the
  whole batch Deferred before resolution; App remains eligible. Callable scope
  cannot borrow the Script BindingRef-to-ValueId ledger. Raw/reference and user
  diagnostic order are unchanged; fallback/retry and partial forest are zero.
closed — NORMAL-CALLABLE-LEXICAL-BINDING-MATERIALIZATION0-I0-R0
  Root callable loans now install one scoped BindingRef-to-ValueId projection
  for formal entries, Local, Variable, and rebind facts. The old name-derived
  semantic identity recovery is gone from this closure; nested Lambda capture
  retains its existing ordered/name compatibility owner. Focused top-level /
  static / plain-instance compound-rebind and direct-capture parity, reuse,
  loan-consumption, guard, and test compilation are green.
closed — NORMAL-CALLABLE-DIRECT-LAMBDA-CAPTURE-MATERIALIZATION0-I0-R0
  Selected root callable direct Lambda children now consume the forest's
  ordered BindingRef capture receipt through the callable ledger and the
  selected name-observation edge is unreachable. The forest remains
  capture-existence/ancestry truth and the receipt remains capture-slot order
  truth; the ledger only projects already-materialized root bindings to
  ValueIds. Missing, foreign, duplicate, or unavailable receipts fail before
  closure publication. Descendant capture forms, Lambda-body lowering,
  capture ABI, ClosureBodyId/NewClosure, FunctionCall, raw/reference, and
  diagnostics remain with their existing owners.
closed — MIRBUILDER-NEXT-NAMED-FAMILY-CENSUS0-D0 (NoSafeSlice)
  Loop/JoinIR retains its located-receipt-to-raw-route operation until a
  verified Loop plan consumes that receipt. TryCatch/Throw/nonfinal Return
  retain one cleanup/result-policy family, and Field/Index/New/RecordUpdate
  retain all-route CallObject preflight owners. No standalone selected old edge
  can be deleted without creating a second planner/resolver; no I0 is opened.
already closed — do not reopen as WIP
  QMark, root Match, StaticConst, and explicit Record schema; only a regression
  can reopen their Git-history evidence.
```

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

The sole current row is `CURRENT_STATE.toml.current_blocker_token`. The active
JoinIR contract and ordered convergence map live in
`design/joinir-loop-selfhost-recipe-pipeline-ssot.md`. Closed route-local provenance
records below are evidence only and must not schedule another route.

Current decision and execution brief:
```text
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SOURCE-LEASE-WITNESS0-D4-S4-S0
  -> cfg(test)-only bounded two-role lease. The resolver reissues the exact
     forest and per-member frames from one function product; owner-branded
     role sites yield BindingRef, site scope, and ancestry. Five focused tests
     are green. No AST/source lifetime, selector/demand/Recipe, Builder/MIR,
     retry, fallback, or production caller was added. External frame mixing is
     structurally impossible; the internal frame co-seal check is diagnostic.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE-WITNESS0-D4-S4-S1
  -> cfg(test)-only CarrierProof: same-BindingRef NestedWrite -> PostLoopRead
     relation, lease brand retained, source lifetime absent, three focused
     tests green. No full shape/candidate/selector/demand/Recipe/Builder/MIR.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE-EXTENSION0-D4-S4-S2-D1
  -> V1 immutable; V2 starts with inner-loop Condition+Step; later proofs stay separate.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE-ROLE-ISSUER0-D4-S4-S2-D0
  -> direct V2 issuance is NoSafeSlice until resolver site inventory/topology exists.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-RESOLVED-SOURCE-SITE-INVENTORY0-D4-S4-S2-S0
  -> resolver traversal records/seals branded statement/expression membership; focused inventory/generic tests green; no downstream consumer or public reference row.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE-ROLE-ISSUER0-D4-S4-S2-S1
  -> cfg(test)-only V2 Condition+Step issuer; five tests green; no downstream consumer or public reference row.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE0-D4-S4-S3-D0/S0/D1
  -> resolver/source-view owns AST-free facts; policy owns operator/type/overflow/
     monotonicity; six focused cfg(test) tests, no public row/caller.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE0-D4-S4-S3-S1-D0
  -> NoSafeSlice: typed literals and resolver parameter types were not co-sealed.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE0-D4-S4-S3-S1-S0
  -> cfg(test)-only exact source-unit receipt/map witness green; no selector/demand/Recipe/Builder/MIR.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE0-D4-S4-S3-S1-S1 -> cfg(test)-only non-Clone co-sealed receipt; six tests cover typed/untyped transport, provenance, map coverage, and source-unit/AST drop.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE0-D4-S4-S3-S1-S2-D0
  -> worker-reviewed two-stage boundary: substrate owns exact type/range/overflow; policy owns progression; Ready/Unresolved/Rejected is fixed.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE0-D4-S4-S3-S1-S2-S0 -> cfg(test)-only substrate projection from receipt plus explicit NumericTarget; six boundaries green, no policy/selector/demand/Recipe/Builder/MIR.
closed — JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE0-D4-S4-S3-S1-S2-S1-S1 -> cfg(test)-only policy witness; seven tests green; role-bearing operands and no selector/demand/Recipe/Builder/MIR.
closed — GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0 / GENERIC-G0-ADMISSION-WINDOW-D0 -> accepted mapping, closed five-row overlap window (not semantic Loop kinds), sole G0 selector boundary, common physical owner, checked legacy manifest, and atomic cutover/deletion contract.
closed — GENERIC-G0-STRUCTURE-S0A -> natural-source projector and sole AST-free structural issuer landed; exact shape/order/BindingRef/owner-source-frame/coverage positives and negatives, AST mutation zero, focused tests and shared caller-zero guard green; selection/type/policy/Recipe/Builder/MIR/production authority remain zero.
closed — GENERIC-G0-SOURCE-TYPE-S0B -> callable-header projector and sole AST-free source-type issuer landed; exact owner-branded parameter/result/literal/context inventory, explicit i64 and missing/non-i64 negatives, move-only S0A bundle, recursive line/caller-zero guard green; target/numeric/policy/Recipe/Builder/MIR/production authority remain zero.
closed — GENERIC-G0-NUMERIC-REPRESENTATION-S0C -> adapter consumes S0B once and seals a neutral exact target/range lease while retaining source + return ABI; plain contextual literals pass, typed suffixes reject, opaque/range boundaries are typed; caller-zero/recursive guard, focused tests, cargo check, and pointer guard green; policy/selection/Recipe/Builder/MIR/production remain zero.
closed — LOOP-JOINSIG-MODULE-SPLIT-R0 -> flat JoinSig module retired into thin facade + model/port/visibility/flow children; direct exit-edge owner is unique, verified wrapper construction remains private, Recipe/JoinSig tests, README/reference sync, and shared recursive guard are green; no acceptance delta or new caller.
closed — LOOP-RECIPE-PRODUCER-ID-S0 -> portable wire now carries `producer_id: LoopRecipeProducerIdV1`; old `producer_route` is rejected, three current producers/fixtures migrated, test-only legacy route parity receipt added, schema/producers remain route-free, focused 59 tests and shared guard green; selector/registry/production caller unchanged.
closed — LOOP-JOINSIG-NESTED-SHADOW-S0 -> visible payload projection now walks target-to-root ancestry, keeps the nearest carrier per Recipe-local binding, emits binding-key order, and isolates siblings; verifier owns unknown/duplicate carrier rejects; 64 focused tests, shared guard, pointer guard, and reference/README sync are green; no After, PHI, Generic, selector, producer, or production change.
closed — LOOP-RECIPE-SOURCE-BOUND-CORE-S0 -> caller-zero core now co-seals verified Recipe/JoinSig/source claim with exact BindingRef/effect relations and typed loop-carrier anchors; 73 focused contract tests, shared guard, pointer guard, and reference/README sync are green; no Generic key, Builder/MIR, physical, or production change.
closed — loop-family receipts and Generic S3/S4-D0 design are closed; S4-I0-R0 caller-zero Recipe producer landed with 42 focused Generic tests and synced Recipe/reference/README receipts.
closed — GENERIC-LEGACY-CORPUS-UNIVERSE-P0 and GENERIC-LEGACY-OBSERVATION-FRONT-G0 landed; S0-D0/I0, S1-D0/I0, and S2-D0/I0 are closed: raw structured-child failures preserve the primary error, FieldAccess and MethodCall consume their exact receipts, and immutable receipts expose the next source boundary.
closed — GENERIC-RAW-STRUCTURED-BODY-ITEM-SOURCE-CANONICALIZATION-S3-D0/I0 and carrier-representation D0 -> rootless nested body sites canonicalized; Program stays rootful; release probe reaches GenericLoop carrier boundary.
  Static-call result publication D0 and caller-zero I0/R0 are closed: source
  transport is locator-only, sealed `(Cataloged caller, SourceExprSite)` selects
  proof, and `CompletedUnifiedValueCallEmissionV1` is the sole success receipt.
  I1/D0 and source-bound handoff design are closed; the candidate remains the
  sole rollback owner. Current row:
  `GENERIC-STATIC-CALL-PUBLICATION-SOURCE-BOUND-ISSUER-S0` (candidate CatalogInstall + AST-free handoff + raw terminal consumption) is wired with focused tests green. Generic production selection remains closed
  until the named caller switch and fresh strict receipt.
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
  4. close Generic G0 source/common-Recipe/selection/physical/parity rows, then perform M10b atomic scheduler/Retry cutover, Generic dead-code R1, and M11/M12
  5. keep every source/check file below 800 lines; no universal raw ingress, Script-only/raw-only resolver, compatibility adapter, or AST reconstruction
  6. R4 consumes the live fence registry above; every item must retire, reown,
     or be explicitly retained before final conformance

R4
  MIRBUILDER-R4-FINAL-CONFORMANCE0-C0 after all active rows above have exact
  retire/reown/retain decisions

after final-pipeline Complete only
  refresh missing-feature / Home ownership readiness inventory
  resume the parked Home ownership taskboard at OWNERSHIP-HOME-RESUME-D0
  then select later unimplemented language features
```
## Fixed packs

Findings stay in the existing eight packs; do not create another pack.
## Parked
```text
source-level Home ownership and unimplemented language features until the
repository-wide final pipeline is Complete
.hako selfhost MirBuilder/parser migration
unselected cleanliness work
new language semantics
default Raw/Canonical cutover before final conformance
```
