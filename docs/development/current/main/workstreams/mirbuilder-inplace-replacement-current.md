---
Status: Active workstream
Date: 2026-07-31
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

現在のMirBuilderを稼働させたまま、競合する責務ownerを一つずつ交換する。
第二MirBuilder、production consumer 0のroute拡張、Legacy fallbackは作らない。
cell数、pack数、LOCは観測値であり、完成条件ではない。

## Current state

```text
Parent:        RAW-ENTRY-MATERIALIZATION-CONTRACT0-D0
Latest landed: NORMAL-INSTANCE-CONSTRUCTOR-CALLABLE-IDENTITY0-I0-R0
Result:        selected Script direct Box raw compatibility is retired
Latest landed:  `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-DISPOSITION0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS24-D0` — closed
Latest landed:  `NORMAL-SCRIPT-PRINT-DIRECT-OWNER0-I0-R0`
Latest design:  `NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-I0-R0`
Latest design:  `NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS25-D0` — closed
Latest design:  `NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS26-D0` — closed
Latest design:  `NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS27-D0` — closed
Latest design:  `NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS28-D0` — closed
Latest design:  `NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS29-D0` — closed
Latest design:  `NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS30-D0` — closed
Latest design:  `NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-D0` — closed
Latest landed:  `NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-I0-R0`
Latest census:  `MIRBUILDER-LIVE-EDGE-CENSUS31-D0` — closed, NoSafeSlice
Latest design: `RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-D0` — closed
Latest landed: `RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS33-D0` — closed
Latest landed: `RAW-ROOT-LEGACY-BRANDED-TERMINAL-RESIDUE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS34-D0` — closed
Latest design: `RAW-STATIC-MAIN-COMPAT-BATCH-DISPOSITION0-D0` — RETAIN-FENCED
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS35-D0` — closed
Latest landed: `JOINMODULE-PHI-OBSERVER-RETIRE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS36-D0` — closed
Latest landed: `JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS37-D0` — closed
Latest landed: `LLVM-JOINMODULE-EXPERIMENT-ROUTE-RETIRE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS38-D0` — closed
Latest landed: `JOINIR-FRONTEND-FUNC-META-RETIRE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS39-D0` — closed
Latest landed: `JOINMODULE-AST-FRONTEND-LEGACY-RETIRE0-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS40-D0` — closed
Latest landed: `JOINMODULE-VM-LOWERONLY-OBSERVATION0-REOWN-RET0`
Latest census: `MIRBUILDER-LIVE-EDGE-CENSUS41-D0` — closed
Latest landed: `MIR-CFG-JUMP-ARGS-LAYOUT-REHOME0-I0-R0`
Latest landed: `JOINMODULE-BRIDGE-DEAD-API-RETIRE0-RET0`
Latest census: `MIRBUILDER-JOINMODULE-CLEANUP-BATCH-CENSUS42-D0` — closed
Latest landed: `JOINMODULE-FORMER-LOWERONLY-TARGET-LOWERERS-RETIRE0-RET0`
Latest landed: `JOINIR-CALLER-ZERO-EXPR-SCOPE-LOWERING-ISLAND-RETIRE0-RET0`
Latest landed: `JOINIR-IF-SELECT-ALTERNATE-LANE-RETIRE0-RET0`
Latest conformance: `MIRBUILDER-R4-FINAL-CONFORMANCE0-C0` — Incomplete
Latest design: `RAW-RECURSIVE-LOCATED-PORT-MIGRATION0-D0` — shrinking carrier accepted
Latest landed: `RAW-LOCATED-STRUCTURED-CONTROL-BODIES0-I0-R0`
Latest design: `RAW-LOCATED-CONTROL-BODY-PARTITION1-D0` — one-row deletion rejected
Latest design: `RAW-LOCATED-RESIDUAL-CONTROL-PARTITION2-D0` — exact residual split
Latest landed: `RAW-LOCATED-DIAGNOSTIC-CONTROL-TERMINALS0-I0-R0`
Latest landed: `RAW-LOCATED-BRANCHING-EXPR-CHILDREN0-I0-R0`
Latest landed: `RAW-LOCATED-LOOP-CHILD-ENTRY0-I0-R0`
Latest landed: `RAW-LOCATED-TRYCATCH-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-PROGRAM-BODY-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-SCALAR-VARIABLE-WRITES0-I0-R0`
Latest landed: `RAW-LOCATED-SCALAR-LOCAL-COMPOUND-WRITE0-I0-R0`
Latest landed: `RAW-LOCATED-PRINT-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-ORDINARY-FIELD-ASSIGNMENT-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-ORDINARY-INDEX-ASSIGNMENT-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-ORDINARY-FIELD-COMPOUND-ASSIGNMENT-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-ORDINARY-INDEX-COMPOUND-ASSIGNMENT-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-NONMATCH-VALUE-RETURN-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-VOID-RETURN-SOURCE-HANDOFF0-I0-R0`
Latest landed: `RAW-LOCATED-NONHOOK-LOCAL-SOURCE-HANDOFF0-I0-R0`
Latest design: `RAW-LOCATED-SCALAR-BINDING-REMAINDER10-D0` — closed
Latest landed: `RAW-LOCATED-SPECIAL-LOCAL-HOOK-SOURCE-HANDOFF0-I0-R0`
Latest design: `RAW-LOCATED-SCALAR-BINDING-DIAGNOSTIC-RESIDUE0-D0` — closed
Latest landed: `RAW-LOCATED-SCALAR-BINDING-DIAGNOSTIC-PORTAL-RETIRE0-I0-R0-RET0`
Latest landed: `RAW-LAMBDA-CHILD-OWNER-SOURCE-ADMISSION0-I0-R0`
Latest design: `RAW-LAMBDA-CHILD-OWNER-SOURCE-LINEAGE1-D0` — closed, NoSafeSlice
Latest design: `RAW-INVOCATION-SEMANTIC-OWNER-CARRIER0-D0` — closed, NoSafeSlice
Latest design: `RAW-SCRIPT-ROOT-EXACT-PROGRAM-SOURCE0-D0` — closed, T1
Latest landed: `RAW-SCRIPT-ROOT-EXACT-PROGRAM-SOURCE0-I0-R0` — `066e33319d`
Latest design: `RAW-SCRIPT-ROOT-SEMANTIC-OWNER0-D0` — closed, NoSafeSlice
Latest design: `RAW-SCRIPT-ROOT-SEMANTIC-SURFACE0-D0` — closed, NoSafeSlice
Latest design: `RAW-SCRIPT-ROOT-SEMANTIC-ADMISSION0-D0` — closed, Accept-corrected
Latest landed: `RAW-SCRIPT-PROGRAM-ITEM-ADMISSION-SSOT0-I0-R0` — `507851393c`
Latest landed: `NORMAL-DEFAULT-PROGRAM-CATALOG-SEAL-HANDOFF0-I0-R0` — `ffda60241b`
Latest landed: `SEMANTIC-OWNER-SOURCE-KIND0-S0` — `9b94cc69c4`
Latest landed: `RAW-SCRIPT-RUNTIME-DEMAND-ADMISSION0-I0-R0` — `24930b1547`
Latest design: `RAW-SCRIPT-SEMANTIC-SOURCE0-I0-R0` — closed, NoSafeSlice
Current design stop: `RAW-SCRIPT-SEMANTIC-OWNER-CORE0-D0`
History:       Git history and the short landed tail below
```

## Latest closeout

`RAW-SCRIPT-ROOT-EXACT-PROGRAM-SOURCE0-I0-R0` — T1 atomic source-spine repair

```text
Change:
  selected Script root now starts at ProgramBodyRoot; each runtime row retains
  its original Program ordinal through filtering, and existing source-preparation
  sites receive ProgramBody(original_ordinal). Compact runtime index remains
  sequencing/suffix bookkeeping only.

Contract:
  PreparedNormalDefaultProgramRootV1 remains the sole AST owner. Root clone
  count, grammar, diagnostics, candidate isolation, and one-execution policy
  are unchanged. No FunctionOwnerIdV1, forest, projection, resolver, Lambda
  publication, or second source identity is introduced.

Evidence:
  Focused Program-root, runtime-work, source-transport, general-module parity,
  cargo check, pointer guard, and active cut0 guard are green. Touched source
  and check files remain below 800 lines.

Next:
  RAW-SCRIPT-ROOT-SEMANTIC-SURFACE0-D0 is the sole design stop. This closeout
  does not activate Script semantic ownership or Lambda lineage.
```

## Latest design closeout

`RAW-SCRIPT-ROOT-SEMANTIC-OWNER0-D0` — closed, NoSafeSlice

```text
Decision:
  Program-owned VerifiedScriptSemanticSourceV1 is the eventual target, but no
  safe I0/R0 exists until Script source-kind, semantic admission, and failure
  precedence are designed as one boundary.

Evidence:
  FunctionSyntaxViewV1, owner_resolver, function-root verification, and
  VerifiedSourceProjectionV1 are Function/Lambda-rooted. Selected Script also
  admits surfaces that the semantic shadow vocabulary marks ExplicitUnsupported;
  direct connection would narrow grammar or move diagnostics earlier.

Done:
  The source-spine row is landed. This D0 issues no semantic owner and opens no
  implementation row. Existing Script grammar, diagnostics, Lambda publication,
  candidate isolation, and one-execution policy remain unchanged.

Stop:
  Program clone/reparse, synthetic FunctionDeclaration, generic Function view
  widening, partial forest/projection pairing, a second resolver/registry,
  unsupported-node opacity, grammar/diagnostic narrowing, Lambda activation,
  fallback, and retry remain forbidden.
```

## Latest design closeout

`RAW-SCRIPT-ROOT-SEMANTIC-SURFACE0-D0` — closed, NoSafeSlice

```text
Decision:
  NoSafeSlice. No direct I0/R0 is opened from the current Script surface.

Evidence:
  The selected Script route spans 57 AST variants, while the existing
  Function/Lambda resolver vocabulary covers only a subset. A kind-only matrix
  is insufficient: Break/Continue and similar nodes change disposition by
  enclosing root/body context. The missing typed OpaqueBoundary contract means
  skipping unsupported nodes would make forest/projection coverage partial;
  resolving them now would narrow grammar or move diagnostics earlier.

Done:
  Worker audits agree on the next prerequisite: a context-sensitive Script
  admission matrix, typed opaque/source boundary, one-traversal contract,
  Lambda inventory-only boundary, and duplicate/unresolved/unsupported
  diagnostic precedence. The landed ProgramBodyRoot/original-ordinal source
  spine remains valid and unchanged.

Stop:
  Do not add a semantic owner, production caller, resolver pass, new guard,
  synthetic FunctionDeclaration, partial forest, second traversal, fallback,
  or retry from this closeout.
```

## Latest design closeout

`RAW-SCRIPT-ROOT-SEMANTIC-ADMISSION0-D0`

Consultation:
`docs/development/current/main/investigations/raw-script-root-semantic-admission0-question-2026-07-31.md`

```text
Decision:
  Accept-corrected. One pre-RootLower admission choice is valid, but the
  proposed 18-file semantic-owner I0 is NoSafeSlice. Split the work by live
  production authority.

Contract:
  owned Program -> one source-only admission -> exactly one pre-lowering
  authority: SemanticEligible or ExistingRootLowerAuthority -> RootLower once.
  ExistingRootLowerAuthority owns no forest/projection and is not a fallback.

Language correction:
  Hakorune has no source try statement and rejects source throw. Canonical
  target syntax is a protected expression/block/member body followed by one
  postfix catch and optional cleanup. ASTNode::TryCatch/finally_body are legacy
  physical carrier names, not language-semantic authority. Multiple-catch,
  first-catch-only, catch-binder, and cleanup env behavior remain compatibility
  evidence; RecoverableFailure activation stays in the language lane.

Stop:
  Do not mix source-kind, resolver generalization, forest/projection, catalog
  movement, cleanup policy, and port cutover into one commit. No synthetic
  Function, partial forest, second resolver, source try/throw permission,
  fallback, retry, or new per-row guard.
```

## Latest execution closeout

`RAW-SCRIPT-PROGRAM-ITEM-ADMISSION-SSOT0-I0-R0` — T1 atomic BoxShape row

```text
Landed:
  507851393c refactor(mir): unify Script program-item admission

Change:
  program_root_work_plan::classify_statement now consumes one exhaustive
  NormalScriptProgramItemAdmissionV1 for each selected Script Program item.
  Box and non-Box disposition are issued from the renamed
  normal_script_program_item_admission module; the former runtime -> non-Box
  classifier chain and old non-Box module are retired.

Contract:
  Grammar, diagnostics, execution order, result/publication policy, original
  Program ordinal, raw/reference behavior, and Hakorune's postfix catch/cleanup
  compatibility carrier behavior are unchanged. Builder access, resolver,
  owner ID, forest, projection, semantic terminal, fallback, and retry remain
  zero.

Evidence:
  cargo check --lib: green
  focused admission/runtime/work-plan tests: green (2 + 7 + 8)
  cut0 shared guard: green
  touched source/check files: below 800 lines
  new source/test/check files: zero

Stop preserved:
  No source try/throw activation, no promotion of ASTNode::TryCatch into
  language authority, no nested semantic traversal, no new resolver/owner,
  no fallback/retry, and no new per-row guard.
```

## Current design consultation and selected execution

`RAW-SCRIPT-SEMANTIC-COMPLETE-CLOSURE0-D0` is closed Accept-corrected. The
external answer confirms the one-pre-RootLower admission principle, but the
worker review rejects the proposed monolithic semantic-owner I0 as NoSafeSlice.
The compact question packet remains the consultation record:

```text
docs/development/current/main/investigations/
  raw-script-semantic-complete-closure0-question-2026-07-31.md
```

It explicitly keeps Hakorune source `try`/`throw` rejected, treats postfix
`catch`/`cleanup` as the canonical target, and treats `ASTNode::TryCatch` as an
internal compatibility carrier. No semantic-owner implementation is authorized
by this closeout; the first executable row is the narrower transport cutover below.

## RAW-SCRIPT-ROOT-PROFILE-TRANSPORT0-I0-R0 — T1 execution brief

Change:
  At `MirBuilder::lower_program_root_after_catalog_install_v1`, replace the
  generic Script-root transport's path-derived body-kind inference with an
  explicit Program-root profile. Keep `ProgramBodyRoot` and original Program
  ordinals; delete only the selected `site == ProgramBodyRoot` kind inference.

Contract:
  One existing selected production caller, one source profile, and one
  `RawInvocationSourceContextV1` handoff. No `FunctionOwnerIdV1`, resolver,
  forest, projection, semantic admission terminal, Lambda publication,
  RecoverableFailure, or catch/cleanup language change. Hakorune has no source
  `try`/`throw`; postfix `catch`/`cleanup` and `ASTNode::TryCatch` remain the
  existing compatibility boundary.

Done:
  Existing raw source-transport tests prove Script roots carry
  `SourceBodyKindV1::Program` explicitly, non-Script roots retain their current
  contracts, and Program ordinal/source-site parity is unchanged. Run the
  existing focused transport tests, `cargo check --lib`, the current pointer
  guard, and the shared cut0 guard. Keep touched source/check files below 800
  lines; add no new guard/test file.

Stop:
  Return to `RAW-SCRIPT-SEMANTIC-COMPLETE-CLOSURE0-D0` if the row needs a
  semantic resolver, a new owner/forest/projection, a second source traversal,
  a new catch/cleanup meaning, a raw retry/fallback, or any Program-to-Function
  coercion. Do not widen this row to catalog movement or Script semantic
  admission.

## Latest execution closeout

`RAW-SCRIPT-ROOT-PROFILE-TRANSPORT0-I0-R0` — `19d68ca708`

```text
Change:
  LocatedRawNodeV1 now carries SourceBodyKindV1 explicitly. Script roots issue
  Program, ordinary roots issue Function, and body-statement transport retains
  the selected profile. The old site-path root-kind inference is deleted.

Contract:
  ProgramBodyRoot, original Program ordinals, source-site shape, root lowering,
  and compatibility transport are unchanged. No resolver, owner ID,
  forest/projection, semantic terminal, Lambda publication, RecoverableFailure,
  or postfix catch/cleanup meaning was added.

Evidence:
  Focused raw source-transport tests: 15 passed. `cargo check --lib`, pointer
  guard, shared cut0 guard, `git diff --check`, and the <800-line source limit
  are green. No new test/check file was added.

Next:
`NORMAL-DEFAULT-PROGRAM-CATALOG-SEAL-HANDOFF0-D0` is closed Accept/T1 after
worker review. The bounded execution brief is below.

## NORMAL-DEFAULT-PROGRAM-CATALOG-SEAL-HANDOFF0-I0-R0 — T1 execution brief

Change:
  Move the existing `VerifiedSameModuleCallableDeclarationCatalogV1::seal_root`
  call and `CatalogSeal` error mapping from
  `MirBuilder::lower_normal_default_program_root_catalog_v1` into
  `ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle`,
  after `PrepareModule` and before `CatalogInstall`. Delete the selected old
  Builder-side method/call; keep the existing root-lowering kernel.

Contract:
  One source-only catalog seal and one install remain. Duplicate-owner,
  method-shape, and parameter-cardinality diagnostics retain their current
  stage/text/order. No source identity, resolver, semantic owner,
  forest/projection, catch/cleanup meaning, or publication policy changes.
  Hakorune source `try`/`throw` remain rejected; postfix `catch`/`cleanup` and
  `ASTNode::TryCatch` remain compatibility evidence only.

Done:
  Lifecycle tests prove CatalogSeal still follows PrepareModule and precedes
  CatalogInstall/RootLower, while success and fresh-request reuse remain green.
  Run focused lifecycle/catalog tests, `cargo check --lib`, pointer guard, and
  the shared cut0 guard. Keep all touched source/check files below 800 lines;
  add no new test/check file.

Stop:
  Return to `NORMAL-DEFAULT-PROGRAM-CATALOG-SEAL-HANDOFF0-D0` if the move needs
  semantic resolver/owner work, changes clone or diagnostic precedence, adds a
  second seal/install, widens grammar, touches postfix catch/cleanup semantics,
  or introduces fallback/retry.
```

## NORMAL-DEFAULT-PROGRAM-CATALOG-SEAL-HANDOFF0-I0-R0 — T1 closeout

```text
Change:
  CatalogSeal and CatalogInstall now run in the selected normal root lifecycle
  after PrepareModule and before RootLower. The old Builder-side catalog method
  and selected call edge were deleted; the post-install Program root kernel
  remains the lowering owner.

Contract:
  One source-only seal, one install, and one RootLower remain. Catalog error
  stage/text/order, source retention, candidate isolation, publication,
  grammar, semantic resolver, and postfix catch/cleanup behavior are unchanged.
  Hakorune source try/throw remain rejected; ASTNode::TryCatch remains an
  internal/legacy carrier and is not a source-syntax admission.

Evidence:
  Lifecycle tests 3/3 and callable-catalog tests 16/16 passed. `cargo check
  --lib`, pointer guard, shared cut0 guard, `git diff --check`, and all touched
  source/check line limits are green. No new test/check file was added.

Next:
  `RAW-SEMANTIC-OWNER-SOURCE-PROFILE0-D0` is closed as NoStandaloneRow. The
  source/profile spine already has no safe independent consumer. The sole next
  design stop is `RAW-SCRIPT-LAMBDA-SEMANTIC-CONSUMER0-D0`, which must close a
  bounded Script/Lambda semantic consumer before any implementation.
```

Corrected forward queue:

```text
1. NORMAL-DEFAULT-PROGRAM-CATALOG-SEAL-HANDOFF0-I0-R0 (closed at ffda60241b)
2. RAW-SEMANTIC-OWNER-SOURCE-PROFILE0-D0 (closed NoStandaloneRow)
3. RAW-SCRIPT-LAMBDA-SEMANTIC-CONSUMER0-D0 (closed NoSafeSlice: parent Script role missing)
4. RAW-SCRIPT-ROOT-ROLE0-D0 (closed Accept(B): Program-specific semantic root)
5. RAW-SCRIPT-PROGRAM-SEMANTIC-ROOT0-D0 (closed Accept-corrected: Program-specific root)
6. RAW-SCRIPT-PROGRAM-ROOT-OWNER-LAMBDA-HANDOFF0-I0-R0 (closed NoSafeSlice: no exact BindingRef-to-ValueId authority)
7. RAW-SCRIPT-LAMBDA-CAPTURE-BINDING-BRIDGE0-D0 (closed Accept(A-prime))
8. RAW-LAMBDA-CLOSURE-EMISSION-TERMINAL0-S0 (closed at e508f27224)
9. RAW-SCRIPT-DIRECT-EXPR-EXACT-PROGRAM-SOURCE0-I0-R0 (current source repair)
10. RAW-SCRIPT-PROGRAM-SEMANTIC-PRODUCER0-D0 (next design stop)
11. RAW-SCRIPT-LAMBDA-CAPTURE-BINDING-BRIDGE0-I0-R0 (blocked on producer D0)

Rule:
  Do not open an execution row while the Script semantic role is unresolved.
  Once selected, one row must name the production caller, the source/semantic
  owner, the exact old edge removed, and the terminal/failure authority. No
  route retries another route. S0 is allowed only when it is the first half of
  an immediately paired behavior-neutral refactor series.
```

## RAW-SEMANTIC-OWNER-SOURCE-PROFILE0-D0 — closed, NoStandaloneRow

```text
Decision:
  NoStandaloneRow. The source/profile transport is already present, but a
  source-profile-only implementation would be proof-only: no production
  consumer yet co-owns a Script forest/projection or removes an old edge.

Evidence:
  ProgramBodyRoot, explicit Program body-kind transport, original Program
  ordinals, unified Script program-item admission, and CatalogSeal-before-
  CatalogInstall are already landed. Adding another passive profile carrier
  would create a disconnected authority and violate the production-caller
  rule.

Must settle in the next design gate:
  the smallest Script semantic admission that has a real consumer; the
  Complete-versus-ExistingRootLower terminal; exact nested Lambda lineage;
  typed child/opaque demand; one traversal coverage; and the exact selected
  raw Lambda edge removed in the same I0/R0.

Hakorune syntax correction:
  source `try` and `throw` are rejected. The supported protected-region form is
  a body/expression/member followed by postfix `catch` (and optional cleanup).
  `ASTNode::TryCatch` is an internal/legacy carrier only; do not treat it as a
  source `try` grammar or invent first-catch semantics.

Hard stop:
  no standalone source-profile row, no synthetic Function, no partial forest,
  no second resolver, no source try/throw activation, and no fallback/retry.

## RAW-SCRIPT-LAMBDA-SEMANTIC-CONSUMER0-D0 — closed, NoSafeSlice

```text
Decision:
  NoSafeSlice. The two-terminal principle is valid, but the first proposed
  Lambda consumer still lacks a valid parent Script semantic owner.

Evidence:
  FunctionSemanticResolverSessionV1 issues FunctionOwnerIdV1 before traversal;
  VerifiedSemanticOwnerForestV1 and its normalized graph have no Script
  source-kind; VerifiedSourceProjectionV1's verified root contract is
  FunctionDeclaration-oriented; and the physical Lambda branch still enters
  PreparedRawLambdaLexicalCaptureLifecycleV1, whose observation is based on
  raw variable_map/ValueId and has no parent Script BindingRef/provenance.

Candidate semantic terminal:
  SemanticEligible
    = owned Program + Script root owner + one forest + one projection
      + exact Script admission coverage
  ExistingRootLowerAuthority
    = owned Program + typed deferral; forest/projection absent; existing
      RootLower runs exactly once

Required before a Lambda consumer:
  decide what a top-level Script is in the final pipeline, then define its
  source-kind/root contract, owner/forest/projection identity, admission
  terminal, and exact parent-to-Lambda lineage. A Lambda-only cutover would
  fabricate a Function root, produce a partial forest, or rerun raw
  compatibility.

Whole-Program defer for this row:
  postfix catch/cleanup carrier, QMark, Match/EnumMatch, BlockExpr
  non-local exits, Call/Object, This-family, and unproven context/control
  surfaces. Hakorune source try/throw remain rejected. ASTNode::TryCatch is
  internal/legacy carrier evidence only; do not invent first-catch semantics
  or catch-binder activation.

Required single traversal:
  one Script admission traversal emits lexical facts, Lambda topology,
  child-demand/opaque coverage, and the terminal choice. No raw Lambda
  observer pass may be run for an eligible Lambda.

Next design stop:
  RAW-SCRIPT-ROOT-ROLE0-D0

No executable row opens from this closeout. The following Lambda edges remain
explicitly fenced until the Script role and lineage are decided:
  PreparedRawLambdaLexicalCaptureLifecycleV1
  RawLambdaLexicalObservationV1::observe
  RawUnlocatedPortalV1::ControlBody
  raw Lambda dispatch dropping parent Script source context

Done criteria for the next role D0:
  choose one semantic unit for Script; state whether it is an implicit main,
  a Program-specific root, or an explicit R4-retained compatibility surface;
  define source identity, owner/forest/projection authority, diagnostics,
  and the first production edge that can be removed. No implementation row is
  valid until this choice is recorded.

Hard stops:
  synthetic FunctionDeclaration, FunctionSyntaxView Program branch, raw and
  semantic Lambda double observation, Complete-to-Deferred fallback, Lambda
  ABI/publication changes, or any selected old Lambda edge left in place.

## RAW-SCRIPT-ROOT-ROLE0-D0 — closed, Accept(B)

```text
Decision:
  B — Program-specific semantic root.

A. Implicit-main: rejected.
  The language SSOT says Main.main is an ordinary callable and Script is an
  evaluation context with a separate ScriptLastExpressionOrUnit contract.

B. Program-specific root: accepted.
  Script is not a function. Introduce a first-class Program-root semantic
  owner and adapt forest/projection/lowering around that source kind.

C. Explicit R4 retention: temporary contingency only.
  It cannot satisfy final-pipeline completion, which requires selected old
  production owners and normal/default compatibility reachability to be zero.

Current evidence:
  the existing semantic owner machinery is Function/Lambda-rooted, while the
  production Script route is Program-rooted. ProgramBodyRoot, original
  ordinals, program-item admission, and CatalogSeal are transport/admission
  pieces, not a Script semantic owner.

Normative contract:
  owned Program; source-kind Script/ProgramEvaluation; ProgramBodyRoot and
  original Program ordinals; no receiver, parameters, or parent edge; final
  source expression -> Value, final statement/empty -> Unit; Script-root
  explicit return remains rejected until a later Script-control row.

Next design stop:
  RAW-SCRIPT-PROGRAM-SEMANTIC-ROOT0-D0. No owner ID, forest, projection,
  Lambda consumer, or Script result activation is implemented by this closeout.

Hakorune syntax:
  source try/throw remain rejected. Canonical protected syntax is postfix
  catch/cleanup; ASTNode::TryCatch is an internal/legacy carrier only.
```

## RAW-SCRIPT-PROGRAM-SEMANTIC-ROOT0-D0 — closed, Accept-corrected

```text
Decision:
  Accept-corrected. Script is a Program-specific semantic root, not an
  implicit Main/function. The first consumer is the bounded Script/Lambda
  handoff below; no whole-Program semantic cutover is claimed.

Ceremony:
  T2 design gate closed; implementation is one bounded I0/R0 row.

Exact first consumer:
  RAW-SCRIPT-PROGRAM-ROOT-OWNER-LAMBDA-HANDOFF0-I0-R0

Input/product:
  owned Program + NormalScriptProgramItemAdmissionV1
  -> one source-kind-aware Script semantic root product
  -> one exact nested-Lambda lineage receipt
  -> existing raw Lambda closure/capture publication, once

Root contract:
  source kind = Script / ProgramEvaluation
  source root = ProgramBodyRoot
  statement sites = ProgramBody(original ordinal)
  receiver = none
  parameters = none
  parent edge = none
  result = ScriptLastExpressionOrUnit
  explicit Script-root return = deferred/rejected by existing authority

Eligible closure (and no more):
  Program sequence, top-level Function/Box callable boundary, Local/Literal/
  Variable facts, and an exact nested Lambda definition site/parent scope.
  Assignment, Print, Me, Unary, Binary, postfix catch/cleanup, QMark,
  Match/EnumMatch, BlockExpr exits, Call/Object, This-family, and unproven
  control/context surfaces remain ExistingRootLowerAuthority. Hakorune source
  try/throw remain rejected; ASTNode::TryCatch is an internal/legacy carrier.

Semantic terminals:
  SemanticEligible = owned Program + Script root + one forest + one projection
    + exact admission coverage. ExistingRootLowerAuthority = owned Program
    + typed deferral, with forest/projection absent. The terminal is selected
    once before RootLower. Complete construction failure is a contract error,
    never a downgrade to the deferred terminal.

Single traversal:
  one Script admission traversal emits lexical facts, Lambda topology,
  child-demand/opaque coverage, and terminal choice. No second resolver,
  raw Lambda observer, or post-failure route is allowed.

Atomic selected old-edge deletion:
  for SemanticEligible Lambda only, remove
  PreparedRawLambdaLexicalCaptureLifecycleV1::prepare(params, body),
  RawLambdaLexicalObservationV1::observe, RawUnlocatedPortalV1::ControlBody,
  and the raw Lambda dispatch parent-source-context drop in the same I0/R0.
  Deferred whole-Program and explicit raw/reference routes remain separate.

Hakorune boundary:
  source try/throw = rejected; postfix catch/cleanup is canonical; the
  ASTNode::TryCatch carrier is internal/legacy and does not establish
  first-catch, catch-binder, or cleanup environment semantics here.

Hard stops:
  synthetic Main/function, FunctionSyntaxView Program branch, partial forest,
  partial projection, second resolver/observer, Complete-to-Deferred downgrade,
  Lambda ABI/publication change, source grammar widening, AST clone/reparse,
  semantic rejection followed by raw retry, or an I0/R0 that leaves the
  selected eligible Lambda edge in place.
```

### Script semantic owner inventory (fixed before implementation)

The following inventory is the boundary for the first code row. It is not a
promise to resolve all Script syntax now; it prevents another proof-only
consumer from being added.

| Surface | First-row disposition | Semantic owner | Deferred/retained reason |
|---|---|---|---|
| Program root / original ordinals | selected | Script root product | none |
| top-level FunctionDeclaration / BoxDeclaration | transferred boundary | existing callable owner | Script resolver must not enter body |
| Literal / Variable / Local | selected | Script lexical traversal | exact source sites only |
| nested Lambda definition | selected | child owner via Script forest | exact parent owner/site/scope required |
| Assignment / Print / Me / Unary / Binary | deferred | ExistingRootLowerAuthority | no Script semantic closure yet |
| postfix catch / cleanup | deferred | existing protected-region owner | RecoverableFailure/cleanup remain separate D0s |
| QMark / Match / EnumMatch | deferred | existing control/expression owner | child-demand parity not closed |
| BlockExpr non-local exits | deferred | existing BlockExpr preflight | diagnostic precedence must not move |
| Call / Object / This-family | deferred | existing raw owner | callable/header/receiver authority not closed |
| source `try` / `throw` | rejected | language diagnostic authority | Hakorune source grammar rejects them |
| `ASTNode::TryCatch` | retained carrier | internal/legacy normalization | not a source grammar decision |

The inventory is complete for the first row only when every encountered node is
one of `SemanticEligible`, `TransferredCallableBoundary`, or
`ExistingRootLowerAuthority`; no partial forest/projection is published.
```
```

Historical located-transport queue (closed/parked context):

```text
1  RAW-LOCATED-BODY-SPINE0-I0-R0
2  RAW-LOCATED-STRUCTURED-CONTROL-BODIES0-I0-R0
3  RAW-LOCATED-RESIDUAL-CONTROL-PARTITION2-D0
4+ exact residual-control rows selected by D0
then RAW-LOCATED-CALL-OBJECT-PORTALS0-D0
then RAW-LOCATED-NESTED-ADMISSION0-I0-R0
```

## R4 fence / residual registry

This is the sole current list for R4 disposition.  A prose `fenced` or
`separate` label is not a registered fence.  R4 Complete requires every active
or unregistered row below to be closed, rehomed, or retained with its complete
activation and sunset contract.

| State | Ledger key / family | Exact surface | Activation / normal-default | Target disposition | Release row / condition |
| --- | --- | --- | --- | --- | --- |
| retain-fenced | `RAW-STATIC-MAIN-COMPAT-BATCH-SUNSET-001` | `PreparedRawStaticMainBoxCompatibilityV1` prepared raw static-Main batch: sorted helpers followed by root Main with legacy entry policy | raw dispatcher static `Main` reaches the batch only through `RawLegacyChildLoweringPortV1`; selected-normal Script uses the distinct `RawInvocationChildPortV1` root-only Main rejection, while verified App Main has its separate typed owner | RETAIN-FENCED: live arbitrary-AST raw route, no exact Program/source locator, helper-first and `LegacyEnvironment` coupling | fresh named release D0 only when one raw located-source + entry-materialization contract can atomically delete dispatcher -> static-Main, RawLegacy -> prepared batch, prepared helper -> raw static method, and prepared root -> legacy Main policy edges |
| closed | `NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003` | selected Script non-Box runtime compatibility, ending with the exact 9 unsupported kinds LoopRange, Break, Continue, ImportStatement, BuildGate, EnumDeclaration, BrandDeclaration, TypeAliasDeclaration, GlobalVar | selected normal Script only; raw/reference and nested body descent remain separate | REOWN | retired by `NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-I0-R0`: exact 9 -> direct shared guarded diagnostic; selected Script `RawCompatibility` execution = 0 |
| closed | `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001` | selected Program immediate instance constructors, plus selected Script plain-instance runtime-prefix constructors | every selected Program instance Box has one immediate demand; plain Script adds its second `InstancePrefixCompatibility` demand; non-plain Script's later raw runtime lifecycle is the separate row below | REOWN | retired by `NORMAL-INSTANCE-CONSTRUCTOR-CALLABLE-IDENTITY0-I0-R0`: one source occurrence -> unchanged physical LegacySymbol admission per existing demand |
| closed | `NORMAL-TOPLEVEL-FUNCTION-CALLABLE-COMPAT-SUNSET-003` | selected Program top-level `FunctionDeclaration` raw LegacyChild admission | selected normal only; raw/reference remains separate | REOWN | retired by `NORMAL-TOPLEVEL-FUNCTION-CALLABLE-IDENTITY0-I0-R0`: source-order receipt -> unchanged legacy physical collector admission |
| closed | `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-COMPAT-SUNSET-001` | selected Script runtime's plain non-Main static/instance Box ordinary-method direct raw admission | selected normal Script only; constructors, static Main, non-plain/nested/raw-reference Box descent excluded | REOWN | retired by `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-ADMISSION0-I0-R0`: selected direct raw method edges = 0 |
| closed | `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-COMPAT-SUNSET-002` | selected Script direct non-plain `BoxDeclaration` raw statement admission | selected normal Script only; raw/reference and non-Box Script statements remain separate | REOWN | retired by `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-DISPOSITION0-I0-R0`: static/instance selected lifecycles and exact sync rejection preserve legacy parity; direct Box -> raw statement driver = 0 |
| retain-fenced | `JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0` | two dev-gated normalized-shadow mutation executions plus the strict/dev StepTree comparison observer | explicit dev/debug only; non-strict decline continues through the already-selected native route, while strict/dev observation is nonblocking; default normal without the gate = 0 | RETAIN-FENCED | fresh named normalized-shadow release D0: verified Recipe/CorePlan loop owner, both mutation paths, strict/dev parity, and independent observer disposition |
| retain-fenced | `VM-BRIDGE-COMPAT-SUNSET-001` | `join_ir_vm_bridge_dispatch` two Exec targets (`Main.skip/1`, `FuncScannerBox.trim/1`) | explicit VM keep (`vm-reference`, keep=true, fallback=false) with `NYASH_JOINIR_VM_BRIDGE=1`; default MIR and VM fallback = 0. Release success exits, dev/trace success observes then continues, non-strict failure continues, strict failure exits | RETAIN-FENCED | fresh named VM-bridge release D0: dispatcher caller = 0 or one explicit-lane execution owner replaces all success/failure continuations |
| closed | `RAW-DRAFT-DISCONNECTED-PROOF-SUNSET-001` | `RawDraftInvocationV1`, its two cfg(test) callers, compiler `begin_raw_draft`, and dedicated guard | production caller = 0; disconnected proof owner only | RET0 | retired by `RAW-DRAFT-DISCONNECTED-PROOF-RETIRE0-RET0`: complete owner/test/compiler/guard surface = 0 |
| closed | `RAW-ROOT-STATIC-CHILD-DRAFT-COMPAT-SUNSET-001` | former `InvocationPhysicalStateV1::complete_raw_static_child` direct `LegacyChildDraftAdmissionV1` issuer shared by static helpers and callable Main | explicit raw public / VM-reference route; default normal = 0 | REOWN | retired by `RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0`: one existing locator+role admission now reaches the unchanged collector projection; direct legacy-symbol issuer = 0 |
| closed | `RAW-ROOT-LEGACY-BRANDED-TERMINAL-SUNSET-001` | former caller-zero `complete_legacy_child_branded` and `commit_legacy_pending_branded` adapters from `LegacyChildDraftAdmissionV1` to branded collector receipt | activation = 0 after `RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0`; definitions only | RET0 | retired by `RAW-ROOT-LEGACY-BRANDED-TERMINAL-RESIDUE0-RET0`; unbranded, symbol-keyed, resolved, and nested-live terminals retained |
| closed | `LLVM-JOINMODULE-EXPERIMENT-ROUTE-SUNSET-001` (promotes `R4-UNREGISTERED-LLVM-EXPERIMENT-001`) | former LLVM runner `JoinIrExperimentBox`: `Main.skip/1` MIR -> JoinModule -> MIR replacement plus original-MIR return on lowering/bridge failure | activation and complete LLVM-only owner/hook/env surface = 0 | RET0 | retired by `LLVM-JOINMODULE-EXPERIMENT-ROUTE-RETIRE0-RET0`; shared JoinModule lowering, VM bridge, normalized-shadow fence, and shared experiment flag remain |
| closed | `JOINIR-FRONTEND-FUNC-META-SUNSET-001` (promotes `R4-UNREGISTERED-FRONTEND-METADATA-001`) | former `frontend::func_meta`, public `JoinFuncMeta`/`JoinFuncMetaMap`, bridge metadata observation and `*_with_meta` APIs | metadata types, non-empty issuers, observation, and old APIs = 0 | RET0 metadata authority; conversion REOWNED into crate-bounded `module_converter` and boundary-aware bridge | retired by `JOINIR-FRONTEND-FUNC-META-RETIRE0-RET0`; converter output, aliases, normalized boundary, AST analysis, and VM bridge preserved |
| closed | `MIR-CFG-JUMP-ARGS-LAYOUT-SUNSET-001` | former `JumpArgsLayout` definition/re-export under `join_ir::lowering::inline_boundary`, formerly consumed by native BasicBlock/EdgeArgs, EdgeCFG, verifier, optimizer, JSON, bridge, and tests | neutral MIR infrastructure remains live under `mir::edge_args`; JoinModule ownership/path = 0 | REOWN | retired by `MIR-CFG-JUMP-ARGS-LAYOUT-REHOME0-I0-R0`: one neutral EdgeArgs/layout owner, all-consumer path replacement, no alias |
| closed | `R4-UNREGISTERED-CARRIER-BOUNDARY-001` — carrier-boundary audit | normalized-shadow-specific `JoinInlineBoundary` / `LoopExitBinding`; former ignored bridge conversion-boundary parameter | live boundary remains constructed/merged only by explicit normalized-shadow dev execution; conversion threading = 0 | dead conversion facade/parameter RET0; real boundary covered by `JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0` | `JOINMODULE-BRIDGE-DEAD-API-RETIRE0-RET0`: caller-zero converter pair and ignored threading = 0; normalized merge keeps `Some(&boundary)` |
| closed | `JOINMODULE-PHI-OBSERVER-SUNSET-001` (promotes `R4-UNREGISTERED-PHI-OBSERVER-001`) | former `verify_phi_reserved` global collector/report, three debug observation hooks, dedicated builder/module tests, exports, README and generated owner-inventory row | production decision consumer = 0 before deletion; complete asset now absent | RET0 | retired by `JOINMODULE-PHI-OBSERVER-RETIRE0-RET0`: complete observer/test/hook/wiring/docs surface = 0 and existing native-owner inventory regenerated |
| closed | `JOINMODULE-AST-FRONTEND-LEGACY-SUNSET-001` (promotes `R4-UNREGISTERED-AST-FRONTEND-001`) | former `AstToJoinIrLowerer`, its exclusive helper/tests, six Program-JSON fixtures, three exclusive dev flags, two lowerer-to-bridge E2E tests, and current frontend contract residue | production caller = 0 before deletion; complete frontend closure now absent | RET0 | retired by `JOINMODULE-AST-FRONTEND-LEGACY-RETIRE0-RET0`; direct VM conversion/tests, JoinModule core/lowering, normalized-shadow, native Phase40 analysis, and `JOINIR_TEST_DEBUG` remain |
| closed | `JOINMODULE-TEST-HANDLER-LANE-SUNSET-001` (promotes `R4-UNREGISTERED-TEST-HANDLER-001`) | former cfg(test)-only `block_finalizer`, `handlers/**`, `merge_variable_handler`, and `terminator_builder` legacy VM-bridge handler lane | production conversion remains solely in `joinir_block_converter/**`; deleted lane and registrations = 0 | RET0 | retired by `JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0`: 14 files / 3743 lines, four cfg(test) module declarations, obsolete README section, stale PHI seam row, and generated inventory rows deleted |
| retain-fenced | `NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001` (promotes `R4-UNREGISTERED-NESTED-BOX-RAW-BODY-001`) | recursive `RawInvocationChildPortV1` -> `lower_static_box_method` / `lower_instance_box_method`, the two live nested-method `LegacyChildDraftAdmissionV1` issuers | selected normal function body is live; nested Main stays root-only reject; raw/reference are separate | R4 BLOCKER: source occurrences exist, but neither live issuer receives a function-relative located-source receipt | `RAW-LOCATED-BODY-TRANSPORT0-D0` must select one located transport whose I0/R0 deletes both named production issuers before R4 Complete |
| closed | `RAW-LEGACY-COMPLETE-CHILD-TEST-FACADE-SUNSET-001` | former caller-zero `ModuleLoweringPortV1::complete_legacy_child`, two disconnected proof modules, and three inline facade tests | production caller = 0 before deletion; live nested issuers already use capture + `commit_legacy_pending` | RET0 | retired by `RAW-LEGACY-COMPLETE-CHILD-TEST-FACADE-RETIRE0-RET0`; live commit terminals, 2 nested issuers, reentrant proof, collector tests, and live callable-Main physical owner retained |
| active compatibility | `RAW-RECURSIVE-UNLOCATED-TRANSPORT-SUNSET-001` | selected `RawInvocationChildPortV1` only: three fixed portals remain — `ControlBody` with exact residual Lambda, `CallObject`, and `NestedBoxAdmission` | one selected state and one execution per node; RawLegacy/raw-reference remain separate; root/body/direct-Box, structured/residual controls, Match/Enum, Loop, TryCatch, and nested Program exact transport are closed | no variant/reason reassignment; Lambda is governed by the linked source-lineage fence below | close only after the Lambda fence reowns ControlBody, the CallObject row removes its portal, and final nested row deletes `NestedBoxAdmission` with both selected nested legacy-symbol issuers |
| retain-fenced | `RAW-LAMBDA-CHILD-OWNER-SOURCE-LINEAGE-SUNSET-001` | selected nested Lambda definition still crosses the located child transport into the existing raw capture/publication lifecycle without consuming its parent located-source context | selected normal function/script body only; raw/reference routes remain separate | RETAIN-FENCED: Lambda and generic-carrier D0s closed NoSafeSlice; ScriptRoot is the first missing semantic-owner producer | ScriptRoot and later exact root producers are prerequisites only. Retire only when `OwnedExprSiteV1(parent_owner, exact_site)` maps through `forest.child_at` to the exact child, its exact parent edge/scope and projected `LambdaBodyRoot` are co-sealed, one reserved `ClosureBodyId` is carried by `NewClosure` and committed exactly once after emission, and the raw Lambda dispatch source-context drop plus remaining Lambda -> ControlBody classifier edge are deleted in the same I0/R0 |
| active compatibility | `RAW-LOCATED-LOOP-ROUTE-SOURCE-HANDOFF-SUNSET-001` | `PreparedLocatedRawLoopChildEntryV1` retains exact Loop parent/condition/body-root receipts, then delegates once to the existing raw JoinIR route | selected invocation only; RawLegacy/reference unchanged; no located JoinIR-plan completion claim | retire when the current Loop route/verified plan consumes the same located product and the source-erasing terminal is zero | no additional route, retry, AST clone/reparse, or receipt reconstruction may be introduced |
| closed | `JOINMODULE-VM-LOWERONLY-OBSERVATION-SUNSET-001` | former three explicit-VM `LowerOnly` target rows, dispatcher observation branch, and `lower_only_routes`; five target names were also consumed by Loop/If/strict classification | observation route and vocabulary = 0; neutral five-name policy and two VM Exec rows remain | REOWN+RET0 | retired by `JOINMODULE-VM-LOWERONLY-OBSERVATION0-REOWN-RET0`: all five lowerers/direct evidence remain; no old target-table alias |
| closed | `JOINMODULE-FORMER-LOWERONLY-TARGET-LOWERERS-SUNSET-001` | former caller-zero Stage1UsingResolver, StageB body, and StageB FuncScanner target lowerers; exclusive builders, dispatchers, Case-A entrypoints, ValueId ranges, tests, and fixtures | production and retained explicit-VM callers = 0 before deletion | RET0 | retired by `JOINMODULE-FORMER-LOWERONLY-TARGET-LOWERERS-RETIRE0-RET0`; neutral five-name policy, skip/trim VM routes, If vocabulary, native Stage1 verifier, and selfhost mode-B lane retained |
| closed | `JOINIR-CALLER-ZERO-EXPR-SCOPE-LOWERING-ISLAND-SUNSET-001` | former condition/expr/local/method/scope/user-policy lowering island, its exclusive tests, and obsolete lifecycle guards | repository production and retained reference callers = 0 before deletion | RET0 | retired by `JOINIR-CALLER-ZERO-EXPR-SCOPE-LOWERING-ISLAND-RETIRE0-RET0`; 22 source files / 4,368 lines deleted; live `ConditionBinding` remains |
| closed | `JOINIR-IF-SELECT-ALTERNATE-LANE-SUNSET-001` | former default-reachable MIR -> JoinInst Select/IfMerge observer, opt-in alternative PHI emission, strict failure policy, and VM dev dry scan | route/classifier/alternate-PHI authority and If-specific env/test surfaces = 0 | RET0; native If/PHI is sole production owner and shared JoinInst vocabulary remains | retired by `JOINIR-IF-SELECT-ALTERNATE-LANE-RETIRE0-RET0` |
| retain-fenced | `JOINMODULE-SHARED-REFERENCE-SUBSTRATE-SUNSET-001` (promotes `R4-UNREGISTERED-JOINMODULE-REMAINDER-001`) | shared JoinModule model, converter, skip/trim lowering, and dispatch substrate required by normalized-shadow and the two VM Exec routes | no independent normal/default activation; execution is reachable only through `JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0` or `VM-BRIDGE-COMPAT-SUNSET-001` | RETAIN-FENCED shared dependency; broad RET0 is invalid while either consumer fence is live | retire when both consumer fences are closed/reowned and a fresh caller census proves model/converter/lowering/dispatch production callers = 0 |

The registry has six retain-fenced families, two active compatibility
rows, zero active retirements, zero active rehomes, twenty closed
residuals, and zero unregistered R4 families.
`LegacyChildDraftAdmissionV1` occurrence count is a separate census metric
(`16` occurrences in `4` `src/mir` files at the latest exact census). The
selected-Script residual is closed; raw static Main is an explicit retained
fence above. Nested raw body descent was
promoted from its immutable unregistered audit key to
`NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001` and is now retain-fenced; it is no
longer an unregistered family.

Every census updates this table before selecting the next row. Every unregistered
family has an immutable audit key, but is not a fence or sunset until a D0
records its exact owner/edge, activation, and retire condition. A future active
fence must then receive its stable sunset ID and either its own named release D0
or the forced `MIRBUILDER-R4-FINAL-CONFORMANCE0-C0` decision here; an
unregistered family may not become active on a generic prose promise. No second
fence ledger is permitted.

`MIRBUILDER-R4-LEGACY-CHILD-ADMISSION-DISPOSITION0-D0` is closed. The exact
30 occurrences in 6 `src/mir` files comprised 8 production-core occurrences and
22 cfg(test) proof occurrences. The only two live issuers are the static and
instance nested-method issuers in `recursive_child_lowering.rs`; both flow
through `commit_legacy_pending` as `LegacyReplaceWholePair` and are owned by
`NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001`. The caller-zero
`complete_legacy_child` facade/proof cluster is retired. The remaining exact
count is 16 occurrences in 4 files: 7 production-core and 9 reentrant live-path
proof occurrences. No live site is unregistered. A newly introduced fence remains invalid unless
its release row/condition is recorded here in the same commit.

## Disposition closeout

`MIRBUILDER-R4-FINAL-CONFORMANCE0-C0` — registry conformance closed; semantic R4 Incomplete

```text
Registry:
  retain-fenced=5, active compatibility/retirement/rehome=0,
  closed=20, unregistered=0.

Pass:
  raw static-Main, normalized-shadow, VM bridge, and their shared JoinModule
  substrate have exact activation and sunset contracts.

Blocker:
  selected-normal nested static/instance method descent still issues two
  LegacyChildDraftAdmissionV1::legacy_symbol rows without a function-relative
  located-source receipt. Therefore unverified direct lower, lower-side AST
  route redecision, and selected old production edge are not zero.

Next:
  RAW-LOCATED-BODY-TRANSPORT0-D0. After that edge is deleted, final
  conformance must still decide whether the live explicit raw static-Main
  compatibility route is inside the accepted production-family completion
  claim; registration alone is not completion evidence. R5 Ownership/View
  remains parked.
```

`JOINIR-IF-SELECT-ALTERNATE-LANE-RETIRE0-RET0` — T1 atomic RET0, closed

```text
Deleted:
  MIR-to-JoinInst If router/Select/IfMerge/PhiSpec lane, alternative PHI
  emitter, VM dry observer, by-name target policy, If-specific env surface,
  and exclusive tests.

Preserved:
  native If/PHI; JoinInst Select/IfMerge/NestedIfMerge vocabulary used by the
  normalized-shadow and VM-reference fences.

Evidence:
  cargo check --lib and --tests --features vm-reference green; deleted route
  symbols and If-specific current env surface = 0.

Next:
  MIRBUILDER-R4-FINAL-CONFORMANCE0-C0.
```

`JOINIR-CALLER-ZERO-EXPR-SCOPE-LOWERING-ISLAND-RETIRE0-RET0` — T1 atomic RET0, closed

```text
Deleted:
  22 caller-zero JoinIR expression/scope/local lowering source files
  (4,368 lines), seven exclusive historical lifecycle guards, and one stale
  expression-lowerer test command.

Preserved:
  ConditionBinding and its live trim/carrier/boundary consumers; skip/trim,
  normalized-shadow, VM bridge, JoinInst vocabulary, and native If lowering.

Evidence:
  cargo check --lib and --tests --features vm-reference = green;
  failure-outcome and native-owner inventories regenerated and self-checking;
  deleted symbols have no remaining Rust caller.

Next:
  JOINIR-IF-SELECT-ALTERNATE-LANE-RETIRE0-RET0.
```

`RAW-LEGACY-COMPLETE-CHILD-TEST-FACADE-RETIRE0-RET0` — T1 atomic RET0, closed

```text
Retired:
  caller-zero complete_legacy_child closure facade; disconnected legacy-terminal
  and callable-Main receipt bridge proof modules; three inline facade tests.

Preserved:
  commit_legacy_pending and commit_legacy_symbol_pending; two live nested-method
  issuers; reentrant capture/commit proofs; collector/ledger tests; live
  callable-Main physical terminal.

Evidence:
  LegacyChildDraftAdmissionV1 30/6 -> 16/4; complete_legacy_child definition and
  Rust callers = 0; cargo check and focused structural guards green.

R4:
  retain-fenced=4, active compatibility=0, active retirement=0,
  active rehome=0, closed=18, unregistered=1.

Next:
  MIRBUILDER-R4-FINAL-CONFORMANCE0-C0.
```

`JOINMODULE-FORMER-LOWERONLY-TARGET-LOWERERS-RETIRE0-RET0` — T1 atomic RET0, closed

```text
Retired:
  three caller-zero Stage1/mode-B target lowerers and their exclusive builders,
  dispatchers, generic Case-A route, ValueId ranges, tests, and fixtures.

Preserved:
  skip/trim lowerers and VM Exec routes; neutral five-name Loop/If/strict policy;
  native Stage1 verifier; selfhost mode-B source lane; normalized-shadow fence.

Evidence:
  old lowerer/entrypoint/range symbols and deleted paths = 0;
  cargo check --lib and retained focused suites = green.

R4:
  census42 and mandatory LegacyChild crosswalk are closed;
  retain-fenced=4, active compatibility=0, active retirement=0,
  active rehome=0, closed=17, unregistered=1.

Next:
  RAW-LEGACY-COMPLETE-CHILD-TEST-FACADE-RETIRE0-RET0.
```

`JOINMODULE-BRIDGE-DEAD-API-RETIRE0-RET0` — T1 atomic RET0, closed

```text
Retired:
  caller-zero JoinIrFunctionConverter::convert_joinir_to_mir and
  convert_function; bridge_joinir_to_mir_with_boundary; ignored boundary
  parameters in bridge/module conversion; dead cfg(test) imports.

Preserved:
  one unconditional crate-private bridge_joinir_to_mir entry;
  convert_function_with_func_names, aliases, type propagation, VM Exec routes,
  JoinModule lowering; normalized execution still constructs JoinInlineBoundary
  and passes Some(&boundary) to the real merge owner.

Evidence:
  cargo check --lib and cargo check --tests --features vm-reference = green;
  join_ir_vm_bridge 19/19 and normalized_shadow 90/90 green;
  old facade/dead converter/ignored parameter symbols = 0.

Structure:
  new source/test/check files = 0; largest touched source = 295 lines;
  grammar/result/runtime/default-normal delta = 0; fallback/retry = 0.

R4:
  carrier-boundary unregistered audit is closed: dead conversion threading = 0,
  real normalized boundary is already owned by its registered dev fence.
  retain-fenced=4, active compatibility=0, active retirement=0,
  active rehome=0, closed=16, unregistered=1.

Next:
  MIRBUILDER-JOINMODULE-CLEANUP-BATCH-CENSUS42-D0, one batch-boundary census.
```

`MIR-CFG-JUMP-ARGS-LAYOUT-REHOME0-I0-R0` — T1 atomic REOWN, closed

```text
Reowned:
  EdgeArgs + JumpArgsLayout -> neutral src/mir/edge_args.rs.
  Native CFG, EdgeCFG, verifier, optimizer, JSON, bridge, and tests now import
  mir::edge_args::JumpArgsLayout; MIR root still exports EdgeArgs only.

Retired:
  inline_boundary JumpArgsLayout definition/re-export and every old qualified
  path/alias = 0.

Preserved:
  the two layout variants and derives; BasicBlock/EdgeArgs shape; CFG/runtime/
  serialization behavior; JoinInlineBoundary/LoopExitBinding; normalized-shadow
  and VM bridge semantics.

Evidence:
  cargo check --lib and cargo check --tests --features vm-reference = green;
  BasicBlock 7/7 and simplify_cfg 13/13 green; MIR root facade/import guards
  green; rc_insertion_selfcheck compiles with rc-insertion-minimal.
  Broad edgecfg/exit_args_collector filters expose three pre-existing fixture
  failures (Ring0 not initialized / one SSA input fixture), not compile errors.

Structure:
  new neutral source = 20 lines; old owner paths = 0; no compatibility alias;
  largest touched source/check file = 796 lines; all remain below 800.

R4:
  retain-fenced=4, active compatibility=0, active retirement=0,
  active rehome=0, closed=15, unregistered=2.

Next:
  JOINMODULE-BRIDGE-DEAD-API-RETIRE0-D0, bounded read-only confirmation.
```

`MIRBUILDER-LIVE-EDGE-CENSUS41-D0` — read-only census, closed

```text
Registry:
  retain-fenced=4, active compatibility=0, active retirement=0,
  active rehome=1, closed=14, unregistered=2.

Fresh comparison:
  bridge dead API = valid T1 RET0: caller-zero cfg(test) converter pair plus
    ignored conversion-boundary threading; real normalized merge remains.
  JumpArgsLayout = selected T1 REOWN: live native CFG vocabulary is wrongly
    owned by legacy JoinModule inline_boundary.

Selected:
  MIR-CFG-JUMP-ARGS-LAYOUT-REHOME0-I0-R0.

Exact transition:
  new neutral mir/edge_args.rs owns EdgeArgs + JumpArgsLayout;
  basic_block imports both; MIR root continues to export EdgeArgs only;
  every layout consumer uses mir::edge_args::JumpArgsLayout;
  old inline_boundary definition/re-export/alias = 0.

Measured census:
  38 Rust files / 136 occurrences; 33 old-owner import or qualified-path lines;
  largest touched consumer = 796 lines and receives import-only change.

Preserve:
  JumpArgsLayout variants/derives/serialization semantics; JoinInlineBoundary;
  LoopExitBinding; normalized-shadow; VM bridge; CFG/runtime/backend behavior.

Hard stops:
  MIR-root JumpArgsLayout export; compatibility alias; boundary/LoopExitBinding
  move; behavior or serialization change; file >=800; bridge dead API mixed in.

Following:
  selected REOWN -> fresh census -> bridge dead API RET0.
```

`JOINMODULE-VM-LOWERONLY-OBSERVATION0-REOWN-RET0` — T2 atomic REOWN+RET0, closed

```text
Reowned:
  exact five-name Loop/If/strict classification into
  join_ir/lowering/loop_target_policy.rs.

Retired:
  JoinIrBridgeKind and LowerOnly vocabulary; three LowerOnly target rows;
  observe_lower_only_target; lower_only_routes.rs and its three VM observers;
  old JOINIR_TARGETS dual-policy table.

Preserved:
  two explicit VM Exec routes; all five target-specific lowerers and direct
  evidence; If target/prefix policy; JoinModule model/converter;
  normalized-shadow; ordinary VM continuation outside handled Exec success.

Evidence:
  cargo check --lib and cargo check --tests --features vm-reference = green;
  neutral policy 2/2, exact VM target tests, five-name lowering policy,
  direct VM converter 5/5, Stage1 resolver lowering 2 green / 2 manual ignored,
  StageB body and FuncScanner 3 green / 1 manual ignored each;
  old code symbols and route file = 0; generated inventories and docs guard
  are current.

Measured:
  one new 50-line source owner; new test/check files = 0;
  largest touched source/check file = 193 lines;
  grammar/result/backend/default-normal delta = 0; fallback/retry = 0.

R4:
  JOINMODULE-VM-LOWERONLY-OBSERVATION-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=14, unregistered=2.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS41-D0, read-only design stop.
```

`MIRBUILDER-LIVE-EDGE-CENSUS40-D0` — read-only census, closed

```text
Registry:
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=13, unregistered=2.

LegacyChildDraftAdmissionV1:
  30 occurrences / 6 src/mir files. The two live issuers are the registered
  nested static/instance method edges; every other occurrence is definition,
  terminal support, or cfg(test). No unregistered live issuer exists.

Candidate result:
  bridge dead API = bounded later T1 RET0; retire caller-zero converter methods
    and dead conversion-boundary threading, preserve boundary merge.
  JumpArgsLayout = bounded later T1 REOWN into neutral EdgeArgs owner; no old
    JoinIR alias, while JoinInlineBoundary remains fenced.
  LowerOnly observation = selected T2 atomic REOWN+RET0. It is the only
    candidate that deletes live explicit-route execution while preserving the
    five-name production classification.

Selected execution:
  JOINMODULE-VM-LOWERONLY-OBSERVATION0-REOWN-RET0.

Exact contract:
  add one neutral loop_target_policy with the existing five names;
  make is_loop_lowered_function delegate to that policy;
  reduce VM dispatch to the existing two Exec targets;
  delete LowerOnly enum/rows/branch/observer routes;
  preserve all five lowerers and their direct evidence.

Required current updates:
  joinir-target-lowerer-thinning-ssot, dispatch/lowering READMEs,
  environment/selfhost route descriptions, sole R4 registry.

Hard stops:
  five-name classification delta; lowerer/test deletion; VM policy in the
  neutral owner; old JOINIR_TARGETS alias; Exec behavior delta; normalized-
  shadow/model/converter edits; fallback or broad JoinModule retirement.

Following order:
  selected REOWN+RET0 -> fresh census -> bridge dead API/carrier evidence ->
  mandatory LegacyChild crosswalk -> R4 final conformance -> Ownership/View.
```

`JOINMODULE-AST-FRONTEND-LEGACY-RETIRE0-RET0` — T2 atomic RET0, closed

```text
Deleted:
  AstToJoinIrLowerer frontend (32 source files); exclusive helper/tests;
  six Program-JSON fixtures; three exclusive dev flags; two frontend-to-bridge
  E2E tests; obsolete current frontend contract and tracked references.

Preserved:
  JoinModule model/lowering; direct VM module converter and five focused tests;
  normalized-shadow; native Phase40 phi analysis; JOINIR_TEST_DEBUG.

Evidence:
  cargo check --lib and cargo check --tests --features vm-reference = green;
  VM bridge direct conversion 5/5, native Phase40 2/2,
  normalized-shadow 90/90, Stage-B body and FuncScanner = green;
  generated failure-outcome/native-owner/artifact inventories are current;
  old frontend symbols, paths, fixtures, and exclusive flags = 0.

Measured:
  63 files changed, 62 insertions, 7422 deletions before this closeout;
  new source/test/check files = 0; frontend production caller = 0 before/after;
  dead-code allowance census improves HEAD 265 -> worktree 264, while the
  pre-existing <=202 repository ratchet remains independently red.

R4:
  JOINMODULE-AST-FRONTEND-LEGACY-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=13, unregistered=2.
  LegacyChildDraftAdmissionV1 = 30 occurrences / 6 files; its exact two live
  issuers are anchored by NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001. The mandatory
  final admission crosswalk remains a C0 prerequisite.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS40-D0, read-only design stop.
```

`MIRBUILDER-LIVE-EDGE-CENSUS39-D0` — read-only census, closed

```text
Registry:
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=12, unregistered=2.

Three-family result:
  carrier/boundary = split confirmed. JumpArgsLayout is neutral CFG REOWN
    material; JoinInlineBoundary remains under the existing normalized-shadow
    fence; neither is selected in this census.
  AST frontend = production caller 0 after metadata retirement; complete
    AstToJoinIrLowerer evidence closure is bounded and selected for RET0.
  JoinModule remainder = live core/normalized-shadow/VM surface. Broad RET0 is
    rejected; LowerOnly retirement first requires neutral five-target policy
    REOWN and remains a later T2 D0.

Selected:
  JOINMODULE-AST-FRONTEND-LEGACY-RETIRE0-RET0, T2 atomic RET0.

Preserve:
  direct VM module conversion and five converter tests; native Phase40
  analysis; JoinModule model/lowering; normalized-shadow; JOINIR_TEST_DEBUG.

Next:
  execute the selected RET0, then return to a fresh live-edge census.
```

`MIRBUILDER-LIVE-EDGE-CENSUS38-D0` — read-only census, closed

```text
Registry:
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=11, unregistered=3.

Four-family result:
  frontend metadata = empty-only production observation; RET0 selected.
  carrier boundary = JumpArgsLayout neutral REOWN plus JoinInlineBoundary
    subordinate to the existing normalized-shadow fence; later D0.
  AST frontend = caller-zero but 5560-line semantic evidence closure; later
    RETAIN-FENCED D0 after evidence crosswalk.
  JoinModule remainder = live normalized-dev/VM core; broad RET0 rejected.

Selected:
  JOINIR-FRONTEND-FUNC-META-RETIRE0-RET0, T2 atomic RET0 with bounded
  converter REOWN.

Next:
  execute the selected RET0, then return to a fresh live-edge census.
```

`JOINIR-FRONTEND-FUNC-META-RETIRE0-RET0` — T2 atomic RET0, closed

```text
Deleted:
  frontend func_meta module and public types; metadata observation and
  *_with_meta APIs; two metadata-only Phase40 tests and stale status prose.

Reowned:
  unchanged Structured JoinModule conversion into crate-bounded
  module_converter; boundary-aware bridge no longer accepts an empty metadata
  map.  Existing function aliasing and normalized-shadow boundary remain.

Evidence:
  cargo check --lib and vm-reference hakorune = green;
  Phase40 analysis 6/6, VM bridge 7/7, Stage-B body/FuncScanner, and
  normalized-shadow 90/90 = green; old metadata symbols/APIs = 0; focused
  rustfmt, diff check, and pointer guard = green.

Measured:
  metadata/tests net deletion = 250 lines before neutral converter move;
  new source/test/check files = 0 (meta.rs responsibility renamed/reowned);
  largest touched source/check file = 297 lines; replacement credit = 0.

R4:
  JOINIR-FRONTEND-FUNC-META-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=12, unregistered=3.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS39-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS37-D0` — read-only census, closed

```text
Registry:
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=10, unregistered=4.

LegacyChildDraftAdmissionV1:
  30 occurrences / 6 src/mir files.  The two live issuers both map to
  NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001; all other occurrences are
  support/test-only.  No unregistered live admission site remains.

Selected:
  LLVM-JOINMODULE-EXPERIMENT-ROUTE-RETIRE0-RET0, T2 atomic RET0.
  The LLVM-only opt-in Main.skip/1 mutation is a live competing production
  authority and silently returns the original MIR on lowering/bridge failure.

Preserve:
  shared JoinModule model/lowering; VM bridge; normalized-shadow dev fence;
  NYASH_JOINIR_EXPERIMENT and its shared accessor.

Next:
  execute the selected RET0, then return to a fresh live-edge census.
```

`LLVM-JOINMODULE-EXPERIMENT-ROUTE-RETIRE0-RET0` — T2 atomic RET0, closed

```text
Deleted:
  JoinIrExperimentBox and its source file; LLVM runner hook; pipeline plan and
  report fields; LLVM-only environment accessors/key; current inventory,
  runtime-report, reference-doc, and hako_check observations.

Preserved:
  LLVM compilation/execution order outside the hook; shared JoinModule
  lowering/model; VM bridge; normalized-shadow dev route;
  NYASH_JOINIR_EXPERIMENT; historical archive records.

Evidence:
  cargo check --lib, --bin hakorune, and llvm-harness+vm-reference = green;
  LLVM pipeline inventory and runtime report smokes = green;
  shared skip_ws, VM bridge, Stage-B body, and FuncScanner focused tests =
  green; retired current symbols/env/fallback owner = 0; pointer guard and
  diff check = green.

Measured:
  retirement surface excluding current pointer/closeout = 13 files,
  5 insertions / 232 deletions including one 124-line source file;
  new source/test/check files = 0; largest touched source/check file =
  274 lines; replacement credit = 0.

R4:
  LLVM-JOINMODULE-EXPERIMENT-ROUTE-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=11, unregistered=4.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS38-D0.
```

`JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0` — T1 detached RET0, closed

```text
Deleted:
  join_ir_vm_bridge/block_finalizer.rs;
  join_ir_vm_bridge/handlers/**;
  join_ir_vm_bridge/merge_variable_handler.rs;
  join_ir_vm_bridge/terminator_builder.rs;
  four cfg(test) module registrations and the obsolete README lane section.

Synchronized:
  canonical SSA seam inventory removed the retired bridge-PHI row;
  PHI publication caller inventory removed the retired caller;
  native-owner and failure-outcome control-flow inventories were regenerated
  with their existing generators.

Preserved:
  joinir_block_converter/** as the sole production conversion owner;
  bridge integration, VM execution, JoinModule model/metadata;
  MIR, grammar, route, diagnostics, and runtime behavior.

Evidence:
  cargo check --lib = green;
  joinir_block_converter 1/1 and bridge integration 7/7 = green;
  resolved Binding SSA contract = green;
  native-owner and failure-outcome control-flow inventory checks = green;
  retired source/module/path references = 0;
  current-state pointer guard and git diff check = green.
  The broad PHI type-publication inventory reaches a pre-existing LocalSSA
  anchor failure; the same failure reproduces at pre-change HEAD 01a4c553c7.

Measured:
  retired lane = 14 source/test files / 3743 lines;
  new source/test/check files = 0;
  largest retained touched source/check file = 645 lines;
  replacement credit = 0.

R4:
  JOINMODULE-TEST-HANDLER-LANE-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=10, unregistered=5.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS37-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS36-D0` — read-only census, closed

```text
R4 registry:
  sole ledger is consistent; no duplicate or missing live fence.
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=9, unregistered=5.

LegacyChildDraftAdmissionV1:
  30 occurrences / 6 src/mir files.
  Both live issuers are nested Box body descent and map exactly to
  NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001.
  No unregistered live admission site exists.

Six-family disposition:
  LLVM experiment       = explicit env/feature live; later RETAIN-FENCED D0.
  frontend metadata     = bridge/normalized-dev live; API census required.
  carrier boundary      = normal CFG plus bridge live; partitioned REOWN.
  AST frontend          = production zero, broad fixture closure; later RET0.
  test-handler lane     = cfg(test)-only, exact RET0 selected.
  JoinModule remainder  = normalized-dev/VM/LLVM live; partition required.

Selected:
  JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0, T1 detached RET0.

Exact delete:
  join_ir_vm_bridge block_finalizer.rs;
  handlers/**;
  merge_variable_handler.rs;
  terminator_builder.rs;
  four cfg(test) module declarations and obsolete README lane section.

Preserve:
  joinir_block_converter/** production owner;
  bridge conversion, VM execution, JoinModule model and metadata;
  grammar, MIR, runtime, route, diagnostics.

Non-claims:
  no feature work; no fallback/retry; no replacement credit;
  no disposition change for the four retained fences.

Next:
  JOINMODULE-TEST-HANDLER-LANE-RETIRE0-RET0.
```

`JOINMODULE-PHI-OBSERVER-RETIRE0-RET0` — T1 detached RET0, closed

```text
Deleted:
  src/mir/join_ir/verify_phi_reserved.rs;
  three cfg(debug_assertions) observe_phi_dst hooks;
  src/mir/builder/phi_observation_tests.rs;
  join_ir/builder module wiring and JoinIR README observation row.

Regenerated:
  mirbuilder-native-owner-candidate-inventory-v0.json with the existing
  tools/rust_lifecycle generator.  Its pre-existing stale baseline was refreshed
  separately in 5bfb44e4c6; this RET0 records only the exact observer removal.

Preserved:
  next_value_id allocation; carrier/invariant PHI order and types;
  JoinIR lowering/verifier; routing, diagnostics, runtime/backend behavior.

Evidence:
  observer/hook/test symbols = 0;
  cargo check --lib = green;
  loop_header_phi_info 3/3 and phi_block_remapper 2/2 = green;
  native-owner inventory --check and current-state pointer guard = green.
  The broader merge filter has four Ring0Context-not-initialized failures;
  the same representative failure reproduces at pre-change HEAD 5bfb44e4c6,
  so it is not caused by this deletion.

Measured:
  source/test/README surface = 384 deleted lines;
  generated inventory delta = 5 additions / 13 deletions;
  new source/test/check files = 0; replacement credit = 0.

R4:
  JOINMODULE-PHI-OBSERVER-SUNSET-001 = closed.
  retain-fenced=4, active compatibility=0, active retirement=0,
  closed=9, unregistered=6.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS36-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS35-D0` — read-only census, closed

```text
Selected:
  JOINMODULE-PHI-OBSERVER-RETIRE0-RET0, T1 detached RET0.

Exact asset:
  verify_phi_reserved.rs global BTreeSet observer/report and internal tests;
  three cfg(debug_assertions) observe_phi_dst hooks;
  builder phi_observation_tests.rs;
  join_ir/builder module wiring; JoinIR README row;
  generated native-owner inventory row.

Activation:
  hooks are debug-compiled writes only.
  enable/get/analyze/disable consumers are dedicated tests.
  production semantic / routing / diagnostic reads = 0.

Atomic delete:
  complete asset above = 0;
  regenerate mirbuilder-native-owner-candidate-inventory-v0.json with the
  existing tools/rust_lifecycle generator.

Preserve:
  builder.next_value_id allocation; carrier/invariant PHI order and types;
  JoinIR lowering/verifier; runtime/backend behavior; all non-observer tests.

R4:
  R4-UNREGISTERED-PHI-OBSERVER-001 is promoted to
  JOINMODULE-PHI-OBSERVER-SUNSET-001 for exact RET0.
  retain-fenced=4, active compatibility=0, active retirement=1,
  closed=8, unregistered=6.

Next:
  JOINMODULE-PHI-OBSERVER-RETIRE0-RET0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS34-D0` /
`RAW-STATIC-MAIN-COMPAT-BATCH-DISPOSITION0-D0` — read-only disposition, closed

```text
Decision:
  RAW-STATIC-MAIN-COMPAT-BATCH-SUNSET-001 = RETAIN-FENCED.

Owner:
  PreparedRawStaticMainBoxCompatibilityV1.

Activation:
  raw expression dispatcher static Main
  -> RawLegacyChildLoweringPortV1::lower_static_main_box.
  selected normal verified App Main reachability = 0.

Why no RET0 / REOWN:
  drive_raw_legacy_* remains live; the batch owns cloned box_name+methods
  rather than an exact Program/source locator; helper-first lowering and root
  diagnostics are coupled to LegacyEnvironment entry materialization.

Release:
  one exact raw located-source + entry-materialization contract must delete in
  one named row:
    dispatcher -> static-Main terminal
    RawLegacy port -> prepared compatibility batch
    prepared helpers -> raw static-method terminal
    prepared root -> legacy-policy Main terminal.

R4:
  retain-fenced=4, active compatibility=0, closed=8, unregistered=7.
  code / behavior / grammar / route delta=0.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS35-D0.
```

`RAW-ROOT-LEGACY-BRANDED-TERMINAL-RESIDUE0-RET0` — T1 detached RET0, closed

```text
Deleted:
  ModuleLoweringPortV1::complete_legacy_child_branded
  ModuleLoweringPortV1::commit_legacy_pending_branded

Preserved:
  complete_legacy_child; commit_legacy_pending;
  commit_legacy_symbol_pending; commit_legacy_symbol_pending_branded;
  resolved/canonical terminals; both live nested-Box legacy issuers.

Measured:
  LegacyChildDraftAdmissionV1 32 -> 30 occurrences, 6 files unchanged.
  module_lowering_invocation_legacy_term.rs 144 -> 106 lines.
  largest touched source/check file = 106 lines.

Evidence:
  exact caller/symbol absence; cargo check --lib; legacy terminal 4/4;
  reentrant nested/failure/reuse 10/10; source admission 1/1; children 7/7;
  existing children guard; all touched source/check files below 800.

R4:
  RAW-ROOT-LEGACY-BRANDED-TERMINAL-SUNSET-001 = closed.
  retain-fenced=3, active compatibility=1, closed=8, unregistered=7.
  replacement credit=0; production/grammar/result/route delta=0.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS34-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS33-D0` — read-only census, closed

```text
LegacyChildDraftAdmissionV1:
  32 occurrences / 6 src/mir files.

Crosswalk:
  production vocabulary                     = 2 occurrences
  production neutral terminals              = 5 occurrences
  production live nested issuers             = 3 occurrences
    (one import + exact static/instance constructors)
  cfg(test) evidence                         = 22 occurrences

Live source:
  nested ordinary static method
  nested instance constructor / method
  -> NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001, RETAIN-FENCED.
  Exact function-relative source transport remains absent; no safe REOWN.

Selected detached residue:
  complete_legacy_child_branded              = definition only
  commit_legacy_pending_branded              = definition only
  disposition                                = RET0
  production / grammar / route delta         = 0
  replacement credit                         = 0

Excluded:
  resolved/canonical caller-zero terminals; eight raw port-dropping facades;
  phi observer; raw static Main; JoinModule families. Each remains a separate
  responsibility and requires a fresh census or named D0.

Next:
  RAW-ROOT-LEGACY-BRANDED-TERMINAL-RESIDUE0-RET0.
```

`RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0` — T2 REOWN, closed

```text
Production:
  explicit raw-root helper schedule / callable Main
  -> one existing RawSourceLocatorV1 + typed demand role
  -> unchanged LegacySymbol / symbol / arity collector projection.

Deleted:
  module_invocation_brand0 direct LegacyChildDraftAdmissionV1 issuer/import;
  legacy-admission version of the sole branded static-child terminal.

Preserved:
  lexical helper order; helpers before callable Main; request -> reserve ->
  child -> ledger complete; prefix/abort; LegacyReplaceWholePair; candidate
  discard and fresh compiler reuse. No second locator, duplicated physical
  fields, catalog widening, grammar, result, fallback, retry, or reselection.

Measured:
  LegacyChildDraftAdmissionV1 35 occurrences / 7 src/mir files
  -> 32 occurrences / 6 files.
  largest touched source/check file = recursive_child_lowering.rs, 791 lines.

Evidence:
  cargo check --lib; source-keyed admission 1/1; raw children 7/7;
  callable Main 3/3; receipt ledger 11/11; raw public ingress 6/6;
  raw physical 2/2; reentrant failure/reuse 1/1; children and public-ingress
  guards; current-state pointer guard.

R4:
  RAW-ROOT-STATIC-CHILD-DRAFT-COMPAT-SUNSET-001 = closed.
  retain-fenced=3, active compatibility=1, closed=7, unregistered=7.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS33-D0.
```

`RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-D0` — T2 design, accepted

```text
Named production caller:
  explicit raw-root helper schedule and callable-Main compatibility
  -> RawRootPhysicalStateV1
  -> InvocationPhysicalStateV1::complete_raw_static_child.

Existing source authority:
  RawSourceLocatorV1 = top-level statement + Box name + method name, with
  source-verified symbol and arity projections. OwnedRawSourceV1 directly
  indexes the Program and co-seals the declaration into
  RawRootStaticChildWorkV1 before ledger or Builder effects.

New owner:
  RawRootStaticChildDraftAdmissionV1 owns the existing locator by value plus
  exactly one demand role:
    StaticHelper { schedule_ordinal }
    CallableMain
  It creates no second locator, source identity, catalog row, or duplicated
  physical symbol/arity fields.

Terminal:
  helper and callable-Main typed consuming constructors issue the role;
  the shared physical terminal captures the unchanged body once, then consumes
  the admission into LegacySymbol(symbol), symbol, and arity exactly once.
  ModuleLoweringPortV1 applies unchanged LegacyReplaceWholePair.

Preserve:
  lexical helper schedule; helpers before callable Main; request -> reserve ->
  child -> ledger complete; prefix/abort evidence; candidate-only publication;
  last-completed whole-pair replacement; fresh compiler reuse.

Atomic delete:
  module_invocation_brand0.rs direct
  LegacyChildDraftAdmissionV1::legacy_symbol(work.symbol(), work.arity())
  and its now-unused import; legacy-admission version of the sole branded
  static-child terminal.

Non-claims:
  Main.main has separately materialized equal locators for root and callable
  demands; repository-wide unique locator issuance is not claimed.
  Raw static-Main and nested Box compatibility are separate fences.

Structure:
  new raw_root_static_child_admission.rs owns only source admission/projection;
  recursive_child_lowering.rs replaces, rather than stacks on, its sole branded
  legacy terminal and remains below 800; no new test/check file; existing
  children guard is extended.

Next:
  RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-I0-R0, one atomic T2 commit.
```

`RAW-DRAFT-DISCONNECTED-PROOF-RETIRE0-RET0` — T1 detached deletion, closed

```text
Deleted:
  raw_draft_invocation.rs                         396 lines
  raw_draft_invocation_p0.rs                       98 lines
  cut0_i0_root0_raw_source0_lower_s0_guard.py     119 lines
  builder/compiler wiring and checks-index row     15 lines

Production:
  MirCompiler::begin_raw_draft callers before deletion = 2, both cfg(test).
  production caller / behavior / grammar / route delta = 0.
  replacement credit = 0.

Preserved:
  SourceBoundRawPackageV1; raw root source/planning; raw expansion receipt
  ledgers; RawRootPhysicalStateV1; selected normal and explicit raw routes.

R4:
  RAW-DRAFT-DISCONNECTED-PROOF-SUNSET-001 = closed.
  LegacyChildDraftAdmissionV1 = 35 occurrences / 7 src/mir files.
  retain-fenced=3, active compatibility=1, closed=6, unregistered=7.

Evidence:
  exact absence/census; cargo check --lib; raw expansion receipt ledger 11/11;
  raw root physical 2/2; active public-ingress guard; pointer guard.

Next:
  RAW-ROOT-STATIC-CHILD-DRAFT-ADMISSION0-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS32-D0` — read-only census, closed

```text
Decision:
  select one detached RET0 before the next production REOWN.

Selected asset:
  RawDraftInvocationV1 / RejectedRawDraftInvocationV1.
  MirCompiler::begin_raw_draft has exactly two callers, both cfg(test) in the
  dedicated raw_draft_invocation_p0 fixture. Production caller = 0.

Atomic RET0:
  delete raw_draft_invocation.rs and raw_draft_invocation_p0.rs;
  delete builder registration/re-export and MirCompiler::begin_raw_draft;
  delete its dedicated guard and checks-index row; update stale positive guard
  assertions. Shared raw source projection and receipt ledgers remain.

Measured reduction:
  source/test Rust -494 lines before incidental wiring changes;
  LegacyChildDraftAdmissionV1 37 occurrences / 8 files
  -> 35 occurrences / 7 files.

Not replacement credit:
  no named production caller changes. This is a detached RET0 asset deletion.

Raw static Main:
  RETAIN-FENCED. Its direct RawLegacy owner edge is one, selected-normal
  reachability is zero, but arbitrary-AST RawLegacy callers prevent a safe
  reachability-zero claim. Helper-first ordering, delayed Main diagnostic, and
  LegacyEnvironment semantics also prevent a local REOWN.

Following production candidate:
  the explicit raw-root static-child issuer already receives an exact
  RawSourceLocatorV1 and may admit a bounded source-keyed REOWN after a fresh
  D0. Do not mix it into this detached deletion.

Next:
  RAW-DRAFT-DISCONNECTED-PROOF-RETIRE0-RET0.
```

`NESTED-BOX-SOURCE-OCCURRENCE0-D0` — T2 design, closed

```text
Decision:
  NoSafeSourceTransport; RETAIN-FENCED.

Reusable vocabulary:
  SourceNodeSiteV1 / SourcePathSegmentV1 can describe one function-relative
  structural path. They do not by themselves issue an occurrence.

Missing production seam:
  RawInvocationChildPortV1 retains only ModuleLoweringPortV1. Raw body,
  statement, expression, If/Loop/scope/Lambda, and Box terminals consume bare
  ASTNode values without enclosing source owner, body index, or child role.
  A nested-only wrapper therefore cannot prove one exact source occurrence.

Rejected:
  symbol/arity as source identity; Span identity; name matching; AST pre-scan
  event queue; clone/reparse; root catalog widening; a renamed
  LegacyChildDraftAdmissionV1; constructor-as-method role collapse.

Required future product:
  one function-root-relative located raw transport, issued at the existing
  recursive traversal and preserved through every child portal. Only after a
  fresh RAW-LOCATED-BODY-TRANSPORT0-D0 names a bounded production edge and its
  same-series deletion may nested StaticMethod / InstanceConstructor /
  InstanceMethod occurrences co-seal the unchanged physical symbol, arity,
  LegacyReplaceWholePair, depth-first order, and candidate-only publication.

Execution:
  S0 = 0; I0/R0 = 0. Building a caller-zero location substrate now would repeat
  the proof-only route failure.

R4:
  NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001 becomes RETAIN-FENCED.
  active compatibility=1, retain-fenced=3, closed=5, unregistered=7.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS32-D0.
```

`NESTED-BOX-RAW-BODY-DISPOSITION0-D0` — T2 design, closed

```text
Decision:
  NoSafeI0; exact prerequisite source authority selected.

Live selected-normal edge:
  active callable body
  -> RawInvocationChildPortV1
  -> nested non-Main static / instance Box
  -> two LegacyChildDraftAdmissionV1::legacy_symbol issuers.

Covered roles:
  NestedStaticMethod
  NestedInstanceConstructor
  NestedInstanceMethod.

Missing capability:
  enclosing callable identity + exact recursive source site/path + nested Box
  occurrence. Root callable catalog excludes nested body declarations, while
  physical symbol/arity alone cannot identify the source occurrence.

Preserve:
  nested Main pre-effect rejection; sync rejection; static method sort; instance
  metadata -> constructors -> methods order; receiver physical arity; depth-first
  child-before-parent collection; LegacyReplaceWholePair parity; candidate-only
  publication and failure discard.

Reject:
  type-renaming LegacyChildDraftAdmissionV1; root catalog widening; AST pre-scan,
  clone, or reparse; constructor/method authority collapse; retry/fallback.

R4:
  R4-UNREGISTERED-NESTED-BOX-RAW-BODY-001 is promoted to
  NESTED-BOX-RAW-BODY-COMPAT-SUNSET-001.
  active compatibility=2, retain-fenced=2, closed=5, unregistered=7.

Next:
  NESTED-BOX-SOURCE-OCCURRENCE0-D0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS31-D0` — read-only census, closed

```text
Decision:
  NoSafeSlice.

Raw static Main:
  RawLegacyChildLoweringPortV1
  -> PreparedRawStaticMainBoxCompatibilityV1
  -> sorted helpers first
  -> Missing / NotFunction / LegacyEnvironment Main.

Selected normal:
  RawInvocationChildPortV1
  -> raw_invocation_main_box fail-fast.
  Prepared compatibility reachability = 0.

Blocker:
  heterogeneous non-test RawLegacy facades still exist, but a complete
  static-Main production reachability map is absent. RET0 is unproven; direct
  normal-Main reuse would change source authority and helper/root ordering.

R4 registry:
  active compatibility=1, retain-fenced=2, closed=5, unregistered=8.
  LegacyChildDraftAdmissionV1 remains 37 occurrences / 8 source files and
  still requires the mandatory per-site disposition crosswalk before C0.

Next:
  NESTED-BOX-RAW-BODY-DISPOSITION0-D0, because the LegacyChild census found
  two higher-priority selected-normal live issuers in nested Box descent.

Parked:
  RAW-STATIC-MAIN-COMPAT-BATCH-DISPOSITION0-D0 remains required before R4 C0.
```

`NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-I0-R0` — T1 atomic
replacement, closed

```text
Production:
  exact 9 selected Script unsupported kinds
  -> DirectSelectedUnsupportedStatement
  -> current span
  -> existing raw expression recursion guard
  -> shared unsupported raw-AST diagnostic.

Atomic delete:
  exact 9
  -> StatementControlCompatibility / DeclarationIngressCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Parity:
  all 9 normal errors equal legacy errors; declaration-fact preparation order,
  recursion-depth precedence, candidate discard, and fresh compiler reuse are
  unchanged. Successful MIR effect, port demand, retry, fallback, and grammar
  delta are zero.

R4:
  selected Script compatibility residual = 0.
  NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003 = closed.
  active compatibility=1, retain-fenced=2, closed=5, unregistered=8.

Evidence:
  focused non-Box disposition, direct-owner, and runtime-work tests;
  shared public-ingress guard; cargo check --lib; all touched source/check
  files below 800 lines.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS31-D0.
```

`NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-D0` — T1 design,
closed

```text
Decision:
  one exact selected-Script unsupported-statement diagnostic terminal.

Named caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected surface:
  LoopRange, Break, Continue, ImportStatement, BuildGate, EnumDeclaration,
  BrandDeclaration, TypeAliasDeclaration, GlobalVar.

Current outcome:
  each kind passes the raw expression recursion guard and reaches the same
  final Unsupported AST node diagnostic; successful MIR effect = 0.

Selected owner:
  DirectSelectedUnsupportedStatement
  -> align current statement span
  -> existing with_legacy_expression_recursion_guard_v1
  -> one shared unsupported raw-AST diagnostic factory.

Atomic delete:
  exact 9 selected Script kinds
  -> StatementControlCompatibility / DeclarationIngressCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Preserve:
  Program declaration-fact preparation before runtime; recursion-depth error
  precedence; exact Debug-format diagnostic; candidate discard and compiler
  reuse. Raw/reference and nested loop/body routes are non-claims.

Forbid:
  emit_void; declaration installation in the terminal; LoopRange/exit semantic
  activation; AST rewrite; port demand; retry/fallback; wildcard source set.
```

`MIRBUILDER-LIVE-EDGE-CENSUS30-D0` — read-only census, closed

```text
Finding:
  no remaining kind has a successful direct runtime owner, but all exact nine
  selected Script roots share one effect-free final raw rejection.

Decision:
  behavior-neutral diagnostic REOWN is the next live T1. It closes the
  selected Script RawCompatibility execution surface without claiming nested
  LoopRange/Break/Continue semantics or accepting declaration ingress.

R4:
  NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003 can close in the same I0/R0
  when selected Script RawCompatibility execution reaches zero.

Next:
  NORMAL-SCRIPT-UNSUPPORTED-STATEMENT-DIAGNOSTIC0-I0-R0.
```

`NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-I0-R0` — T1 atomic replacement,
closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected path:
  FastMemRegion
  -> DirectFastMemRegion
  -> lower_direct_fastmem_region_v1
  -> existing build_fastmem_region_with_port_v1.

Atomic delete:
  selected Script FastMemRegion
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Preserved:
  exact source span; contract/body/span transport; same RawInvocation port;
  register -> push -> source-order body -> pop; candidate-local region metadata;
  typed body error and outer-region restoration.

Failure:
  typed child failure pops the inner region, rejects and discards the candidate,
  and the same compiler accepts a fresh FastMem request. Panic unwind cleanup
  and candidate-internal metadata rollback remain non-claims.

Evidence:
  disposition tests; full normal/legacy MIR and verification parity; FastMem
  metadata/MemOp tests; typed cleanup test; direct-owner 6/6; runtime mapping;
  shared guard; cargo check --lib; diff check.

Residual:
  10 -> 9 exactly.

Structure:
  runtime work = 799; direct owner = 649; disposition = 237;
  FastMem region tests = 179; shared guard = 799.
  New source/test/check files = 0.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS30-D0.
```

`NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-D0` — T1 design, closed

```text
Decision:
  one direct selected Script FastMemRegion admission.

Named caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected existing owner:
  build_fastmem_region_with_port_v1.

Atomic delete:
  selected Script FastMemRegion
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Contract:
  exact FastMemRegion source; statement span aligned once; contract, body,
  source span, and the same RawInvocation port passed once to the existing
  register -> push -> body -> pop lifecycle owner.

Failure:
  register fails before push; typed body failure still pops the inner region
  and candidate isolation prevents metadata or Builder publication. Panic
  unwind cleanup and rollback inside the discarded candidate are non-claims.

Evidence:
  full normal/legacy MIR and metadata parity; same-port body order; typed
  body failure cleanup; candidate discard and fresh compiler reuse.

Exclude:
  fastmem lifecycle rewrite; metadata duplication; fresh child port; nested
  body reclassification; retry/fallback; new source/failure/compat owner.
```

`MIRBUILDER-LIVE-EDGE-CENSUS29-D0` — read-only census, closed

```text
Selected:
  FastMemRegion is the sole safe live T1 edge. Its existing port-aware owner
  already owns register -> push -> same-port body -> pop and metadata.

NoSafeSlice:
  LoopRange / Break / Continue require loop/exit/CFG authority.
  ImportStatement / BuildGate / EnumDeclaration / BrandDeclaration /
  TypeAliasDeclaration / GlobalVar have no equivalent direct runtime owner.

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 remains a separate 37-occurrence / 8-file
  census metric; it is not a fence count.

Next:
  NORMAL-SCRIPT-FASTMEM-REGION-DESCENT0-I0-R0.
```

`NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-I0-R0` — T1 atomic replacement,
closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected path:
  If
  -> DirectIfStatement
  -> lower_direct_if_statement_v1
  -> drive_raw_if_statement_with_port_v1
  -> existing IfForm
  -> complete_if_statement_v1.

Atomic delete:
  selected Script If
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Preserved:
  exact statement span; same RawInvocation port; condition then optional-else
  demand order; unknown-span Program branch shells; CFG/PHI/JoinIR behavior;
  success-only Void completion; branch termination and suffix stop.

Failure:
  condition/then/else failures reject the candidate without another route;
  live Builder remains unpublished and the same compiler accepts a fresh If.

Evidence:
  direct-owner 5/5; disposition 2/2; If-descent 7/7; runtime-work 6/6;
  If-parity focused tests; shared guard; cargo check --lib; diff check.

Residual:
  11 -> 10 exactly.

Structure:
  runtime work = 794; direct owner = 585; disposition = 225;
  shared guard = 799. New source/test/check files = 0.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS29-D0.
```

`NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-D0` — T1 design, closed

```text
Decision:
  Candidate A — one direct selected Script statement-If admission.

Named caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Selected existing owner:
  drive_raw_if_statement_with_port_v1
  -> existing IfForm
  -> complete_if_statement_v1.

Atomic delete:
  selected Script If
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Contract:
  exact If source; statement span aligned once; condition then branch and
  optional else branch demanded through the same RawInvocation port; existing
  unknown-span Program branch shells and Void completion preserved.

Evidence:
  no-else / else / nested-body full MIR parity; condition, then, and else
  failure ordering; branch termination and suffix stop; candidate discard and
  compiler reuse.

Exclude:
  FastMemRegion region lifecycle; LoopRange/Break/Continue loop-exit authority;
  Import/BuildGate/declaration ingress; new CFG or completion semantics.

Structure:
  normal_script_runtime_work.rs and the shared guard begin at 799 lines.
  The atomic row must include only meaning-neutral local compaction sufficient
  to keep both files below 800; no new source/check file.

Next:
  NORMAL-SCRIPT-IF-STATEMENT-DESCENT0-I0-R0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS28-D0` — read-only census, closed

```text
Exact residual:
  StatementControl = If / LoopRange / Break / Continue / FastMemRegion.
  DeclarationIngress = Import / BuildGate / Enum / Brand / TypeAlias / Global.

Bounded owners:
  If -> existing raw statement-If descent and completion.
  FastMemRegion -> existing region owner, but it requires an independent
  register/push/body/pop and metadata parity row.

No direct equivalent:
  LoopRange / Break / Continue currently terminate in raw unsupported or
  loop-frame-specific authority.
  All six ingress kinds currently terminate in raw unsupported; Enum/Brand
  declaration facts do not make runtime completion Void. Direct no-op would
  change behavior.

Selection:
  If alone, T1. No multi-kind bulk cutover.

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 = 37 occurrences / 8 src/mir files, separately.
```

`NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-I0-R0` — T1 atomic
replacement, closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Atomic delete:
  Assignment / CompoundAssignment / Loop / Nowait / TaskScope / ContextScope /
  TryCatch / Throw / Local / ScopeBox / Outbox / Program / UsingStatement
  -> RawCompatibility
  -> drive_legacy_statement_v1
  -> build_statement_with_port_v1 fallthrough
  = 0.

Selected path:
  DirectPortAwareExpression
  -> drive_legacy_expression_v1 with the same RawInvocation port
  -> the unchanged raw-expression statement-surface terminal.

Parity:
  all 13 roots compare exact normal/legacy success or diagnostic; successful
  rows compare full MirPrinter output and verification_result. Existing Return
  suffix termination and failure/reuse evidence remain green.

Residual:
  24 -> 11 exactly.
  StatementControl = If / LoopRange / Break / Continue / FastMemRegion.
  DeclarationIngress = ImportStatement / BuildGate / EnumDeclaration /
  BrandDeclaration / TypeAliasDeclaration / GlobalVar.

Non-delta:
  grammar/result/verification/publication/raw-reference/fallback/retry = 0.

Evidence:
  shared cutover guard; direct-owner 4/4; disposition 2/2; runtime-work 4/4;
  statement-surface/task-scope focused tests; cargo check --lib; diff check.

Structure:
  direct owner = 467 lines; disposition = 223; shared guard = 799.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS28-D0.
```

`NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-D0` — T1 design, closed

```text
Decision:
  Candidate A — delete one shared statement-to-expression adapter for the
  complete StatementSurfaceFallthrough0 set.

Exact set:
  Assignment / CompoundAssignment / Loop / Nowait / TaskScope / ContextScope /
  TryCatch / Throw / Local / ScopeBox / Outbox / Program / UsingStatement.

Structural proof:
  RawInvocationChildPortV1::lower_statement
  -> build_statement_with_port_v1
  -> none of {If, StaticConstTable, FastMemRegion}
  -> drive_legacy_expression_v1 with the same port
  -> raw expression statement_surface exact owner.

New path:
  DirectPortAwareExpression
  -> drive_legacy_expression_v1 with the same RawInvocation port
  -> the same raw expression statement_surface exact owner.

This is one adapter responsibility:
  inner Assignment/place, Loop/CFG, async, scope/body, exception, binding,
  ContextScope diagnostic, Program body, and Using Void owners are not grouped,
  copied, or changed. They remain the terminal authorities in both paths.

Atomic delete:
  the 13 selected roots -> RawCompatibility -> drive_legacy_statement_v1 = 0.
  Residual 24 -> 11.

Exact residual:
  StatementControl = If / LoopRange / Break / Continue / FastMemRegion.
  DeclarationIngress = Import / BuildGate / Enum / Brand / TypeAlias / Global.

Forbid:
  If/FastMem special-arm bypass; a blanket AST match; a second dispatcher or
  port; terminal-specific semantic edits; source allowlists; fallback/retry;
  raw/reference widening; new source/check file; any file reaching 800.

Evidence:
  exhaustive classifier set; old dispatcher three-special-arm guard; parity
  matrix across all 13 terminal families; ContextScope exact diagnostic;
  child/terminal failure and fresh reuse; body/suffix/termination/source-order;
  same-port nested call/Box/Loop behavior.

Next:
  NORMAL-SCRIPT-STATEMENT-SURFACE-FALLTHROUGH0-I0-R0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS27-D0` — read-only census, closed

```text
Selected Script residual:
  StatementControlCompatibility = 16
  DeclarationIngressCompatibility = 8
  total = 24.

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 = 37 occurrences / 8 src/mir files, separately.

Selection:
  the exact 13-kind StatementSurfaceFallthrough0 structural set. Single-kind
  Using or Nowait rows are rejected as unnecessarily fine-grained because the
  outer adapter relation is identical and terminal owners remain independent.
```

`NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-I0-R0` — T1 atomic
replacement, closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

New selected path:
  StaticConstTable
  -> DirectStaticConstRuntimeCompletion
  -> normal_script_direct_statement_owner
  -> exact source check
  -> statement span
  -> emit_void.

Atomic delete:
  selected Script StaticConstTable
  -> DeclarationIngressCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Authority:
  PreparedNormalProgramStaticTableMetadataV1 remains the sole metadata owner;
  its prepare/commit still precedes work-plan/runtime exactly once. The direct
  runtime helper has no metadata read, reconstruction, validation, or commit.

Evidence:
  disposition and runtime partitions are exhaustive; valid table compilation
  preserves full module/verification outcome and table span; metadata pair
  ordering and failed-prepare atomicity tests remain green; shared guard fixes
  pre-runtime ordering and forbids metadata access in the direct helper.

Result:
  residual 25 -> 24; DeclarationIngress 9 -> 8; StatementControl remains 16;
  new source/metadata owner, port, route, grammar, publication, fallback,
  retry = 0.

Structure:
  direct owner 342 lines; disposition 223; runtime work 799; shared guard 799.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS27-D0.
```

`NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-D0` — T1 design, closed

```text
Decision:
  Candidate A — direct selected-Script runtime completion for StaticConstTable.

Existing authority:
  PreparedNormalProgramDeclarationFactsV1 and
  PreparedNormalProgramStaticTableMetadataV1 complete source-order metadata
  preparation and atomic commit exactly once before Program work-plan/runtime.

Old runtime path:
  StaticConstTable
  -> DeclarationIngressCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  -> statement span
  -> emit_void.

New runtime path:
  DirectStaticConstRuntimeCompletion
  -> existing normal_script_direct_statement_owner sibling
  -> exact StaticConstTable check
  -> statement span
  -> existing emit_void.

Atomic delete:
  selected Script StaticConstTable -> drive_legacy_statement_v1 = 0.
  Residual 25 -> 24; DeclarationIngress 9 -> 8; StatementControl stays 16.

Forbid:
  metadata read/rebuild/revalidation/recommit in runtime; DirectPortAwareExpression
  misuse; child port; AST clone; Enum/Brand/Using or another ingress kind;
  result/grammar/publication change; retry/fallback; new source/check file.

Evidence:
  exact disposition partition; metadata prepare/commit precedes runtime; table
  span owns one Void completion; multiple tables preserve metadata/runtime
  source order and scalar tail; invalid metadata fails before runtime and fresh
  request reuses compiler; shared guard and all files below 800.

Next:
  NORMAL-SCRIPT-STATIC-CONST-RUNTIME-COMPLETION0-I0-R0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS26-D0` — read-only census, closed

```text
Selected Script residual:
  StatementControlCompatibility = 16
  DeclarationIngressCompatibility = 9
  total = 25

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 = 37 occurrences / 8 src/mir files, separately.

Decision:
  StaticConstTable is the only next zero-child metadata/runtime-completion
  responsibility. Remaining control and ingress kinds retain distinct CFG,
  binding, scope, exception, import, or declaration authorities.
```

`NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-I0-R0` — T1 atomic replacement, closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement.

Change:
  ASTNode::Return is classified as DirectPortAwareExpression and reaches the
  existing raw expression statement-surface Return owner through the same
  RawInvocation port.

Atomic delete:
  selected Script Return
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Preserved:
  root span; return; Void completion; arbitrary value-child descent; cleanup;
  Match-return; defer; emitted Return; block termination and suffix stop;
  diagnostics; candidate discard; fresh compiler reuse.

Evidence:
  direct root parity covers void and FunctionCall-bearing value Return;
  a Return followed by an undefined-variable Print proves suffix suppression;
  failing value lookup followed by fresh success proves reuse; existing raw
  Return descent suites, shared guard, cargo check, and pointer guard are green.

Result:
  residual 26 -> 25; StatementControl 17 -> 16; DeclarationIngress remains 9;
  new owner/product/route/grammar/result/fallback/retry = 0.

Structure:
  direct owner 323 lines; disposition 210; runtime work 789; shared guard 799.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS26-D0.
```

`NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-D0` — T1 design, closed

```text
Decision:
  Candidate A — route every ASTNode::Return through the existing
  DirectPortAwareExpression terminal.

Old selected path:
  Return
  -> StatementControlCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  -> build_statement_with_port_v1
  -> raw expression statement-surface Return owner.

New selected path:
  Return
  -> DirectPortAwareExpression
  -> drive_legacy_expression_v1 with the same RawInvocation port
  -> the same raw expression statement-surface Return owner.

Preserved owners:
  return; uses build_void_return_statement;
  return value uses drive_value_return_statement_v1, including cleanup
  preflight, Match-return probe, one arbitrary value-child descent, defer, and
  emit_return_from_value. Block termination and suffix stopping remain in the
  unchanged block driver.

Atomic delete:
  selected Script Return -> drive_legacy_statement_v1 = 0.
  Residual 26 -> 25; StatementControl 17 -> 16; DeclarationIngress stays 9.

Forbid:
  a Return operand allowlist; a second port; custom Return semantics; new
  owner/product/route/failure; fallback/retry; StaticConstTable or another
  statement responsibility in this row.

Evidence:
  void/value/arbitrary-child full MIR and verification parity; exact diagnostic
  parity; Return span; Match-return and termination/suffix behavior; late
  failure then fresh compiler reuse; shared guard and all files below 800.

Next:
  NORMAL-SCRIPT-RETURN-DIRECT-OWNER0-I0-R0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS25-D0` — read-only census, closed

```text
Selected Script residual:
  StatementControlCompatibility = 17 exact kinds
  DeclarationIngressCompatibility = 9 exact kinds
  total = 26

Retired category:
  CallObjectHeaderCompatibility live source occurrence = 0.

R4 registry:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 remains 37 occurrences / 8 src/mir files and is
  an independent observation metric, not a fence count.

Safe independent candidates:
  Return and StaticConstTable. Return is selected because it reaches an
  existing exact owner through the already selected expression terminal;
  StaticConstTable retains a separate metadata-before-runtime completion
  contract.
```

`NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-I0-R0` — T1 atomic
replacement, closed

```text
Named production caller:
  NormalScriptRuntimeBlockPortV1::lower_statement

Selected responsibility:
  QMarkPropagate / MatchExpr / EnumMatchExpr / ArrayLiteral / MapLiteral /
  RecordLiteral / RecordUpdate / Lambda / BlockExpr / Arrow /
  GroupedAssignmentExpr / MethodCall / FieldAccess / Index / New / This /
  FromCall / ThisField / MeField / FunctionCall / Call.

New path:
  DirectPortAwareExpression
  -> existing normal_script_direct_statement_owner
  -> drive_legacy_expression_v1 with the same RawInvocation port.

Atomic delete:
  CallObjectHeaderCompatibility = 0;
  the 21 roots -> RawCompatibility -> drive_legacy_statement_v1 = 0.

Parity:
  representative call/object/allocation/control/nested-function roots compare
  full MirPrinter + verification on success and exact diagnostics on failure;
  existing root-span, nested FunctionCall, late-failure, and compiler-reuse
  evidence remains green.

Result:
  selected Script compatibility residual 47 -> 26 exact kinds;
  compatibility terminal count remains one; new owner/route/grammar/result/
  publication/fallback/retry = 0.

Structure:
  direct owner 250 lines; disposition 198; runtime work 789; shared guard 799.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS25-D0.
```

`JOINMODULE-REFERENCE-ASSET-DISPOSITION0-D0` — T2 disposition, closed

```text
normal/default JoinModule execution = 0

REOWN (separate CorePlan/MIR rows):
  CFG/boundary carriers (`JumpArgsLayout`, `JoinInlineBoundary`, carrier and
  loop-scope facts); finalization type helpers; shared operator/error policy.

RETIRE (after each named closure):
  normalized-shadow emission/observation, direct JoinIR runner, and cfg(test)
  VM-bridge handlers. They are not final planners or acceptance truth.

RETAIN-FENCED:
  JoinIR model/lowering/JSON/format only while required by the explicit
  `NYASH_JOINIR_VM_BRIDGE` VM route or LLVM experiment gates. Neither is a
  normal/default route; their sunset is decided before R4.

DERIVEDSHADOW:
  all 48 manifests are `mainline_selected=0`. Retire the direct stale
  `condition_fn_injection` bundle now; reown/refresh bounded-finalize,
  function-region-stack, and aggregate evidence that named its deleted edge;
  retain the remaining 45 caller-zero reference families. Raw root condition
  drafts are a separate live owner and are excluded.
```

## Current design decision

`JOINMODULE-PHI-RETURN-STRATEGY-REOWN0-D0` — T2, closed

```text
Decision: Candidate A — Builder-finalization return-type strategy rehome.

The sole live consumer is finalize_module.  The existing strategy moves as one
Builder sibling; JoinIR's TypeHintPolicy and GenericTypeResolver are deleted
with their exports, not re-exported.  This corrects the stale P3-D-only card:
the actual policy is one ordered finalization owner.

Observed order (must not move):
  Direct value type -> primary name hint -> P3-D known definition
  -> P4 PhiTypeResolver -> P3-C uniform-PHI fallback.

No route, grammar, result, publication, fallback/retry, Ownership/View, or
feature delta.  VM/LLVM, normalized-shadow, Loop/CorePlan, JumpArgsLayout,
JoinInlineBoundary, and all remaining R3 fences stay separate.
```

## Closed design decision

`JOINMODULE-NORMALIZED-SHADOW-RETIRE0-D0` — RETAIN-FENCED

```text
Default normal compilation has normalized-shadow execution = 0, but explicit
JoinIR dev/debug reaches two direct execution sites and a body observer:

  1. cf_loop_joinir_impl -> try_normalized_shadow
  2. drive_legacy_block_v1 -> NormalizedShadowSuffixRouterBox
  3. strict/dev StepTree observer (diagnostic only)

Fence: JOINMODULE-NORMALIZED-SHADOW-DEV-FENCE0
  - selected normal authority = 0
  - compatibility expansion = 0
  - new fallback/retry approval = 0
  - JoinInlineBoundary and JumpArgsLayout ownership move = 0

Sunset: remove both direct execution edges only after one verified
Recipe/CorePlan owner covers their loop shapes, strict/dev parity is green,
and the observer contract is independently disposed. The explicit VM/Stage1/
StageB reference consumers are handled only by the next reference-sunset D0.
```

## Census13 disposition

`MIRBUILDER-LIVE-EDGE-CENSUS13-D0` — closed, NoSafeLiveI0

```text
Selected normal/default is one candidate-session -> collector -> finalization
route. It has no RawLegacyChild port, raw driver, build_module edge, or safe
competing live authority left to switch atomically.

Separate D0 boundaries, not executable I0s:
  raw/static-Main callable compatibility (env policy can create Main.main/N)
  header-sensitive Global Call result policy
  selected-invocation Loop/CorePlan
  If/JoinIR control

RETAIN-FENCED:
  explicit VM bridge/LowerOnly, normalized-shadow dev route, LLVM experiment,
  frontend metadata, JumpArgsLayout/JoinInlineBoundary carriers.

R3 selection:
  one cfg(test)-only return-collector asset may retire. This is the first
  detached retirement after the preceding live I0/R0; it earns no replacement
  credit. After it closes, Census14 is mandatory before another selection.
```

## Latest closeout

`JOINMODULE-RETURN-COLLECTOR-TEST-ASSET-RET0` — T1 detached R3 retirement

```text
Change:
  `join_ir/lowering/return_collector.rs`, its cfg(test) module declaration, and
  five stale current control-flow inventory rows = 0.

Contract:
  The asset had no external Rust consumer beyond that cfg(test) declaration;
  return semantics, normal/default, VM bridge, LLVM, routes, and fences stayed
  unchanged.

Done:
  Source/current-inventory references = 0; lib/vm-reference and reusable
  lane/pointer guards = green.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS14-D0`.
```

## Census14 disposition

`MIRBUILDER-LIVE-EDGE-CENSUS14-D0` — closed

```text
Selected normal/default remains one Program-only candidate-session -> collector
-> finalization route; no raw legacy port, build_module edge, fallback, or safe
competing live owner is reachable. Therefore NoSafeLiveI0.

R3 has no second detached asset: inline_boundary_builder has Boundary-carrier
test consumers; the other inspected JoinIR surfaces have live/fenced consumers.
Therefore NoSafeDetachedR3.

The only selected next boundary is raw/static Main compatibility. Its module
ingress snapshot can materialize Main.main/N and alter runner entry selection;
it is a policy D0, not an atomic cleanup.
```

## Current design stop

`RAW-STATIC-MAIN-CALLABLE-COMPATIBILITY0-D0` — T2, closed as RETAIN-FENCED

```text
`prepare_module` snapshots `NYASH_BUILD_STATIC_MAIN_ENTRY` once into the sole
`CallableMainCompatibilityPolicyV1`; body lowering never rereads it. Required
materializes `Main.main/N` through the existing port, while Omitted does not.

This is not caller-zero or a decls-only cleanup: selected normal reaches it,
`Main.main/0` changes normal runner entry selection, and `N > 0` retains
explicit-entry semantics. The raw ledger is a separate reference witness, not
the selected normal receipt.

Do not rehome yet. Moving only the conditional would preserve every authority
and make a wrapper, not an in-place replacement.

Sunset requires one explicit entry-materialization request/result contract
consumed by normal, raw/reference, and runner entry selection; only then may a
later row retire the snapshot adapter, compilation-context policy field, direct
lower-side read, and raw ledger/physical disposition together.
```

## Current design stop

`RAW-ENTRY-MATERIALIZATION-CONTRACT0-D0` — T2 policy boundary

```text
Decision: Candidate C — source-owned materialization facts; route-specific
normal/raw receipts; runner-specific selection stays with its existing adapter.

Shared vocabulary:
  `CallableMainMaterializationPolicyV1` plus an exact issued target
  (symbol/arity only). It owns no AST, source identity, brand, collector, or
  runner choice.

Route receipts:
  normal: ingress snapshot -> Program expansion source receipt -> collector
  completion; raw/reference: existing explicit selection -> raw source receipt
  -> raw physical/ledger completion. Do not combine their brands, drains, or
  runner routes.

Runner boundary:
  receipt proves what functions materialized; it does not choose execution.
  Preserve each current selector, including `NYASH_ENTRY`, MIR/PyVM/mock
  `Main.main/0` preference, native LLVM `Main.main/1`, and raw exact `main/0`.

Compatibility:
  normal Script + Required remains Omitted; raw Script + Required remains its
  existing source rejection. `Main.main/0` is a preference candidate; `N > 0`
  remains explicit invocation, not default entry.

Hard stop: no global runner selector, entry-name inference, AST/config clone,
env reread, second route/collector, public result/JSON change, retry/fallback,
Ownership/View, or feature activation.
```

## Latest closeout

`ENTRY-MATERIALIZATION-RECEIPT0-S0` — T2 prerequisite, closed

```text
Change:
  `CallableMainMaterializationPolicyV1`, symbol/arity target vocabulary, and
  separate normal/raw source receipts are source-only products.

Contract:
  No receipt stores AST/config/brand or selects a runner entry. Builder,
  collector, publication, raw ledger, and runners remain unchanged.

Done:
  normal Script+Required -> Omitted; raw Script+Required has no receipt and
  retains the existing binding rejection; App keeps exact `Main.main/N` facts.

Next:
  `ENTRY-MATERIALIZATION-NORMAL-CONSUMPTION0-I0-R0`.
```

## Latest closeout

`ENTRY-MATERIALIZATION-NORMAL-CONSUMPTION0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Named caller:
  NormalDefaultPublishedPipelineV1::compile

Change:
  snapshot the materialization policy once at normal ingress; thread the sealed
  normal receipt through the existing one-session lifecycle; delete the selected
  normal lower-side environment snapshot/materialization decision.

Keep:
  raw/reference source receipts and physical ledger, all runner selectors,
  result/publication policy, and the existing candidate reuse contract.

Evidence:
  Required/Omitted x Script/App x Main.main/0/nonzero; exact symbol/arity;
  helper -> callable -> root order; failure leaves the live Builder reusable.

Kept:
  global compatibility-field deletion, raw/reference consumption, runner-policy
  changes, a second route/collector, AST/config duplication, reread/retry,
  Ownership/View, or feature work.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS15-D0` — read-only, closed

```text
Inventory:
  selected normal, raw/reference, and runner materialization consumers.

Result:
  selected normal has no safe immediate I0/R0; raw/reference receipt has
  production consumer=0; runner selectors remain independent fences.

Selected D0:
  `NORMAL-RUNTIME-INPUT-SNAPSHOT0-D0`.
```

## Current design decision

`NORMAL-RUNTIME-INPUT-SNAPSHOT0-D0` — T2, closed

```text
Decision:
  Candidate N — infallible normal-only ingress receipt.

Normal preserves its current permissive contract: only untrimmed case-insensitive
1/true/on enables the entry safepoint; absent, empty, malformed, wrong-typed,
or empty script-argument JSON means no pushed arguments and no diagnostic.
NYASH takes precedence over HAKO even when its value is malformed.  Raw's strict
snapshot remains separate because it rejects malformed input and carries distinct
provenance.
```

## Latest closeout

`NORMAL-RUNTIME-INPUT-SNAPSHOT0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  NormalDefaultPublishedPipelineV1 captures NormalRuntimeInputSnapshotV1 once;
  the existing candidate lifecycle consumes it for entry safepoint and Main
  wrapper arguments.  Selected lower-side reads = 0.

Result:
  NYASH/HAKO precedence, permissive malformed values, App/Script behavior,
  request-versus-compile timing, failure/reuse, raw static-Main compatibility,
  normal/vm-reference checks, and the reusable ingress guard are green.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS16-D0`.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS16-D0` — closed

```text
Result:
  Selected normal has no safe immediate I0/R0.  Its Program root still sends
  Box methods, instance constructors, and top-level functions through
  LegacyChildDraftAdmissionV1; raw receipts, runtime inputs, runners, and
  explicit reference lanes remain fenced.  The
  JoinModule generic Case-A, VM bridge, LowerOnly, and LLVM surfaces also have
  live consumers, so Census16 records them RETAIN-FENCED rather than inventing
  an R3 retirement.

Next:
  `NORMAL-CALLABLE-DRAFT-IDENTITY-AND-ADMISSION0-D0`.
```

## Latest design decision

`NORMAL-CALLABLE-DRAFT-IDENTITY-AND-ADMISSION0-D0` — T2, closed

```text
Inventory:
  The live child port carries catalog-addressable static/instance Box methods,
  uncatalogued instance constructors, and uncatalogued top-level functions.

Decision:
  Candidate C is accepted: first replace catalog-addressable Box methods only.
  Their existing CanonicalSameModuleCallableKeyV1 becomes a source witness and
  seals one physical symbol/arity relation; instance physical arity includes
  exactly one receiver.  The existing one body snapshot, parent restoration,
  LegacySymbol key, and LegacyReplaceWholePair collector policy remain exact.

Reject:
  ResolvedChildDraftAdmissionV1 is not a substitute: it requires an
  invocation-local FunctionOwnerIdV1, a canonical no-body session, and reject-
  duplicate collector policy.  Do not fabricate that owner or change drain
  policy in this cell.

Fence:
  Constructors and top-level functions stay on the existing normal
  compatibility edge.  Their sunset is
  `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001`: each must obtain an
  exact source identity before its branch can move; neither is completion debt
  hidden behind the cataloged-method product.
```

## Latest closeout

`NORMAL-CATALOGED-BOX-METHOD-DRAFT-ADMISSION0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  Main helpers, non-Main static methods, and ordinary instance methods carry
  their existing CanonicalSameModuleCallableKeyV1 through the root port into
  NormalCatalogedBoxMethodDraftAdmissionV1.  The receipt derives the physical
  symbol and arity, including exactly one instance receiver.

Delete:
  Those selected child paths construct LegacyChildDraftAdmissionV1 = 0.  The
  receipt maps once to the unchanged LegacySymbol + LegacyReplaceWholePair
  collector boundary; collector-key replacement remains a later named cell.

Evidence:
  source/physical receipt tests, general-module normal-vs-legacy MIR parity
  including static and instance methods, candidate failure/reuse, raw legacy
  terminal tests, lib/vm-reference checks, and current guards are green.

Residual:
  constructors, top-level functions, optional callable Main, and Script-runtime
  Box descent remain on explicit normal compatibility edges pending independent
  source/port ownership.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS17-D0` — read-only, closed

```text
Result:
  `NormalEntryMaterializationSourceReceiptV1::App` already carries the exact
  `Main.main/N` source target and the installed callable catalog supplies its
  exact static row.  Required callable Main is therefore the one safe selected
  normal I0/R0.  Collector/drain stays RETAIN-FENCED: it still consumes only
  LegacySymbol + LegacyReplaceWholePair and has no old selected-normal edge to
  delete.

Fence:
  Constructors and top-level functions lack source callable owners.  Script
  runtime Box descent is shared raw-port work without normal admission facts.
  Keep them separate; do not fold them into callable Main.  Record
  `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001` for constructors and
  top-level functions, and `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-COMPAT-
  SUNSET-001` for Script runtime Box descent.

Next:
  `NORMAL-CALLABLE-MAIN-MATERIALIZATION-ADMISSION0-I0-R0`.
```

## Latest closeout

`NORMAL-CALLABLE-MAIN-MATERIALIZATION-ADMISSION0-I0-R0` — T1 atomic selected-normal cutover, closed

```text
Change:
  Required App callable `Main.main/N` now proves its receipt target against the
  installed static catalog row, seals NormalCatalogedBoxMethodDraftAdmissionV1,
  and uses the cataloged static port.  Its selected materialization ->
  LegacyChildDraftAdmissionV1 edge = 0; raw policy materialization is unchanged.

Done:
  exact target/catalog fixture, missing-row fail-fast, Required/Omitted normal
  integration, raw static-Main compatibility, candidate/reuse, lib/vm-reference,
  and reusable lane guards are green; `decls.rs` is 449 lines.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS18-D0`.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS18-D0` — read-only, closed: NoSafeLiveI0

```text
Result:
  Script Program runtime re-enters raw Box descent, while constructors and
  top-level functions still have no callable source identity.  Thus no normal
  edge can move without a new source/port decision.  Collector/drain and all
  explicit VM/LLVM/raw fences remain retained.

R3:
  Legacy AST->JoinModule frontend and the cfg(test) legacy handler lane are
  disposition D0 candidates only.  Neither retires until its owned test and
  reference contract is independently resolved.  R4 still decides delete vs
  explicit fenced-reference disposition for the complete old-IR scope.

Next:
  `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-ADMISSION0-D0`.
```

## Latest design decision

`NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-ADMISSION0-D0` — T2, closed: Candidate A

```text
Change:
  Select one Program work-plan-owned, source-order Script runtime receipt and
  a narrow selected-normal Box-callable adapter.  It classifies each runtime
  Box statement exactly once before Builder effects; generic raw ports and the
  raw expression dispatcher remain non-normal authorities.

Contract:
  Preserve Program immediate/runtime order, one body descent, source identity,
  collector mapping, runner selection, and candidate failure/reuse.  Ordinary
  non-Main static and instance methods use their installed exact catalog rows;
  constructors, top-level functions, static Main, nested/raw-reference Box
  descent stay with their separately registered compatibility residuals.

Stop:
  Return if runtime source order cannot coexist with the existing block driver,
  an instance method could enter the collector twice, a generic raw port gains
  normal authority, or a constructor/top-level identity, collector policy,
  second session, fallback, or retry is required.
```

## Latest closeout

`NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-ADMISSION0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  Selected-normal Script runtime uses one source-order admission receipt.
  Ordinary non-Main static methods enter the installed cataloged static port;
  ordinary instance methods retain their runtime declaration prefix but do not
  re-admit already cataloged methods.  Raw/reference uses a neutral statement
  carrier and never constructs the normal admission receipt.

Deleted:
  selected Script runtime direct raw admission for catalog-addressable ordinary
  non-Main static/instance methods = 0.

Done:
  mixed static/instance Script normal-vs-legacy MIR parity, no duplicate
  callable functions, late method failure/fresh reuse, neutral raw carrier,
  lib/vm-reference builds, focused lifecycle/port suites, and shared guards
  are green.  `NORMAL-SCRIPT-RUNTIME-BOX-CALLABLE-COMPAT-SUNSET-001` is closed.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS19-D0`.
```

## Latest closeout

`MIRBUILDER-LIVE-EDGE-CENSUS19-D0` — read-only, closed: NoSafeLiveI0

```text
Result:
  The remaining selected-normal LegacyChild admissions are top-level
  FunctionDeclaration and instance constructors.  The existing callable
  catalog owns only static/instance Box methods, so neither edge can move as a
  T1/I0 replacement.  Raw static-Main remains explicit raw compatibility.

Registry correction:
  Script I0 retired only plain direct Box ordinary-method admission.  Non-plain
  Script Boxes still select raw compatibility and are registered as
  `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-COMPAT-SUNSET-002`; no surface is hidden
  by the closed plain-Box sunset.

Next:
  `NORMAL-TOPLEVEL-FUNCTION-CALLABLE-IDENTITY0-D0`.
```

## Latest design decision

`NORMAL-TOPLEVEL-FUNCTION-CALLABLE-IDENTITY0-D0` — T2, closed: Candidate A

```text
Decision:
  one selected-normal Program-work-plan receipt per top-level
  `FunctionDeclaration`, carrying a source-order occurrence key
  `{ statement_index, declared_name, declared_arity }` and a separately sealed
  physical `{ symbol = name/arity, arity }` admission.

Collector:
  source occurrence identity is distinct from physical collector identity.
  Preserve `LegacySymbol + LegacyReplaceWholePair`, including legacy
  source-order last-wins when two occurrences project to one `name/arity`.
  Body capture, header lookup, parent restoration, result policy, and candidate
  rejection remain existing owners.

Reject:
  widening `VerifiedSameModuleCallableDeclarationCatalogV1` or its Box-method
  namespace; synthetic Box owners; caller-zero normal_source_plan reuse;
  CanonicalRejectDuplicate; a detached S0; raw/reference receipt issuance;
  source reread/reparse/second root scan; fallback or retry.

I0 contract:
  selected normal only replaces
  `PreparedProgramRootTopLevelFunctionWorkV1::lower_with_port_v1`
  -> raw static method -> `LegacyChildDraftAdmissionV1` with one dedicated
  selected top-level capture port.  Constructors, Script non-plain Boxes,
  static Main, and raw/reference retain their registered routes.
```

## RAW-SCRIPT-DIRECT-EXPR-EXACT-PROGRAM-SOURCE0-I0-R0 — closed

```text
Change:
  Every selected DirectPortAwareExpression now installs the already-sealed
  ProgramBody(original ordinal) statement source before one raw descent.
  Local, Variable, Lambda, and the previously located structured roots use the
  same source rule.

Delete:
  selected DirectPortAwareExpression -> descent with active ProgramBodyRoot
  instead of its exact statement site = 0.

Preserve:
  Program classification, runtime order, MIR/result policy, candidate
  isolation, raw/reference routes, Lambda ABI, and fallback/retry = 0.
```

## RAW-SCRIPT-PROGRAM-SEMANTIC-PRODUCER0-D0 — closed, Accept-corrected

```text
Decision:
  A Script semantic body is the ordered, original-Program-ordinal demand
  window sealed from the existing Program work plan. It is not the whole
  Program, a compressed runtime vector, or a synthetic function. Callable Box
  and Function subtrees are transferred/opaque boundaries where the work plan
  assigns another owner.

Architecture:
  Keep FunctionSyntaxViewV1 public behavior Function/Lambda-only. Add a private
  neutral syntax core and sibling Script view. Brand verified and normalized
  owner identity with DeclaredFunction/Script/Lambda. Complete co-owns Program,
  Script root profile, forest, Program projection, coverage, and ordered
  capture receipts. Deferred owns no partial forest/projection and executes
  ExistingRootLower exactly once; it is a pre-lowering terminal choice, never
  retry/fallback.

Execution series:
  1. SEMANTIC-OWNER-SOURCE-KIND0-S0
     behavior-neutral source-kind identity; the only caller-zero prerequisite.
  2. RAW-SCRIPT-RUNTIME-DEMAND-ADMISSION0-I0-R0
     selected lifecycle consumes the one existing work plan and seals every
     original-ordinal runtime demand as Resolve / TransferredOpaque / Deferred.
  3. RAW-SCRIPT-SEMANTIC-SOURCE0-I0-R0
     first bounded Complete closure co-seals Script forest/projection and
     deletes the bare Script-root authority for that closure.
  4. RAW-SCRIPT-BINDING-MATERIALIZATION0-I0-R0
     exact BindingRef ledger and first-demand capture receipts replace eligible
     name-based Variable/Lambda materialization. Nested-Lambda forwarding stays
     deferred until its own production replacement row.

Compatibility ratchet:
  sunset_id = SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001
  baseline  = Complete 0 by construction at the first admission I0/R0;
              ExistingRootLower owns every selected Script request
  metric    = Complete request count over one fixed fixture corpus plus one
              deterministic real-.hako corpus
  law       = Complete count never decreases; Deferred reason vocabulary never
              grows without a new D0; each expansion row deletes one named
              Deferred reason in the same commit
  first_real_milestone = first repository .hako file in the lexical-only
                         Complete closure, fixed by path in the admission row
  later_milestones = control composition; call/object demand; Box runtime
                     demand; postfix catch/cleanup; nested-Lambda forwarding
  retire_when = every selected normal/default Script demand is Complete or has
                an explicitly retained R4 terminal, and the compatibility
                caller is zero

Forbid:
  caller-zero producer; synthetic FunctionDeclaration; Program widening of
  Function public products; partial forest/projection; source/name-derived
  BindingRefV1; capture order from forest.upvars(); second resolver/observer;
  semantic failure downgrade; Complete-to-Deferred downgrade; fallback/retry;
  unbounded parallel corpus/allocator benchmark.

After Accept:
  connect one invocation-local BindingRefV1-to-ValueId ledger; publish eligible
  Local rows by exact declaration site; read selected Variable/Lambda captures
  only from that ledger; remove the selected raw Lambda observer/name
  materialization edge in the same I0/R0.
```

## RAW-SCRIPT-RUNTIME-DEMAND-ADMISSION0-I0-R0 — closed

```text
Change:
  The selected normal lifecycle prepares the one Program work plan after
  CatalogSeal and before CatalogInstall. The prepared original-ordinal demand
  sequence is passed into Program lowering by value. Raw/reference callers
  retain their existing lower-side preparation.

Delete:
  selected Program RootLower -> prepare/classify Program work plan = 0.

Ratchet:
  SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001 is active.
  Complete baseline = 0; Deferred baseline = all selected Script requests.
  RAW-SCRIPT-SEMANTIC-SOURCE0-I0-R0 must make the first monotonic increase and
  fix one real repository .hako path without a broad parallel census.

Preserve:
  Catalog/diagnostic order, Program clone count, runtime order, MIR/result,
  candidate isolation, raw/reference routes, and fallback/retry = 0.
```

## RAW-SCRIPT-SEMANTIC-SOURCE0-I0-R0 — closed, NoSafeSlice

```text
Finding:
  The selected lifecycle now owns the original-ordinal work plan, but no
  Script Complete consumer exists. FunctionSyntaxViewV1 remains
  Function/Lambda-only; VerifiedResolvedFunctionV1, lowering-root verification,
  forest payload, and source projection still require Function/Lambda contracts.
  SemanticOwnerSourceKindV1 branding alone does not seal a Program root.
  Raw Lambda capture is still name-based and has no BindingRef-to-ValueId
  ledger or ordered capture consumer.

Decision:
  Stop implementation and open RAW-SCRIPT-SEMANTIC-OWNER-CORE0-D0. Compare a
  private generic semantic-owner core with Function/Script wrappers against a
  Script-specific product. Keep public Function views unchanged. Fix the first
  Complete lexical closure and concrete fixture path only after that owner
  contract is selected.

Hard stops:
  no partial forest/projection, synthetic FunctionDeclaration, Function-root
  widening without source-kind contract, forest.upvars() as capture ABI,
  Complete-to-Deferred downgrade, fallback/retry, or fourth edge census.
```

## RAW-SCRIPT-SEMANTIC-OWNER-CORE0-D0 — closed, Accept(A-prime)

```text
consultation_question:
  docs/development/current/main/investigations/raw-script-semantic-owner-core0-consultation-question-2026-07-31.md

Finding:
  Source-kind branding and work-plan hoisting are only prerequisites. The
  semantic forest still stores VerifiedResolvedFunctionV1, lowering roots
  require Function/Lambda body contracts, and source projection seals a
  FunctionDeclaration root. Raw Lambda capture is name-based and has no
  BindingRef-to-ValueId consumer.

Decision:
  Accept Candidate A-prime, corrected by worker review. Use one private
  root-neutral semantic-owner core with explicit DeclaredFunction/Script/Lambda
  root profiles, typed Function/Script wrappers, one shared forest, and one
  shared Program-capable projection authority. A Script-specific outer bundle
  is allowed, but a second Script forest/projection authority is forbidden.
  FunctionSyntaxViewV1 and existing public Function APIs remain unchanged.

  A direct Literal production cutover is not yet safe: the current forest,
  lowering-root verifier, and projection still have Function/Lambda-only
  contracts. The design is therefore executed as a short BoxShape refactor
  series; no grammar or Complete closure is added until that series is green.

  The runtime window, not the whole Program, is the Script body. Immediate
  callable boundaries are transferred to their existing owners; only the
  exact runtime demand window may become Script-owned semantic input.

Required first Complete closure after selection:
  zero-or-more literal-only Script runtime items plus transferred callable
  boundaries. Variable/Local may be semantic facts only if no ValueId/ABI or
  materialization claim is made; otherwise defer them. Me, Lambda, assignment,
  control, call/object, Box runtime demand, and postfix catch/cleanup remain
  deferred. Lambda requires an ordered first-demand BindingRef capture receipt;
  forest.upvars() is never an ABI order.

Fixed evidence:
  `tools/checks/fixtures/raw_vm_reference_conformance/integer_0.hako` is the
  first real source milestone, loaded once into AST and passed to a selected
  NormalCompileRequest. Raw VM conformance is not milestone evidence.

Resume conditions:
  The following task sequence is now fixed. Each refactor commit is buildable
  and behavior-neutral; the final I0/R0 is the only commit that changes the
  selected production edge. No fourth census, partial forest,
  Complete-to-Deferred downgrade, fallback, or retry.

Task sequence:

1. `SEMANTIC-OWNER-ROOT-PROFILE0-S0`
   Add a private `SemanticOwnerRootProfileV1` with exact contracts:
   `DeclaredFunction -> FunctionBody`, `Script -> ProgramBodyRoot`, and
   `Lambda -> LambdaBodyRoot`. Derive source kind from this profile rather
   than keeping an independent source-kind authority. Add counterexample
   tests for mismatched root/profile pairs. Keep public Function/Lambda APIs
   and production behavior unchanged. New module target <=250 lines; do not
   edit the 799-line Program work-plan file.

2. `SEMANTIC-OWNER-CORE0-S1A`
   Introduce a private `VerifiedResolvedOwnerCoreV1` around the current
   data/normalized/lowering/index products and make the existing Function
   wrapper delegate to it. Add a private profile-checked Script wrapper with
   no constructor or consumer yet. Preserve the existing Function API and
   avoid renaming the many direct `ResolvedFunctionDataV1` test fields.
   Buildable and behavior-neutral; no forest payload or root verifier change.

3. `SEMANTIC-OWNER-CORE0-S1B`
   Move the forest payload to a new bounded payload module with one enum
   `VerifiedSemanticOwnerProductV1::{Function, Script}`. Preserve
   `forest.owner()`/`owners()` as the existing Function-only API; add only
   internal generic accessors for later Script verification. Update forest
   helper dispatch without duplicating upvar/topology authority. Buildable and
   behavior-neutral; no Script construction, port, fixture, or grammar.

4. `SEMANTIC-OWNER-CORE0-S1C`
   Replace the Function/Lambda body-root union search with a profile-exact
   root witness (`ResolvedOwnerLoweringRootsV1`), retaining a compatibility
   alias for existing Function consumers. Reject mismatched profile/root pairs
   in focused tests. Keep ScopeKind/RegionKind execution storage unchanged.
   Do not add Script production or edit the 799-line Program work-plan file.

5. `SEMANTIC-OWNER-PROJECTION0-S2`
   Generalize the private projection seal to profile-exact roots and add a
   separate Script source view. FunctionSourceView remains Function/Lambda
   only; no Program arm is added to it. Program projection must co-seal with
   the same shared forest and must not store AST references. Keep all existing
   Function projection behavior unchanged.

6. `SEMANTIC-OWNER-FOREST-DISPATCH0-S1D`
   Complete the missing behavior-neutral generic dispatch behind the shared
   forest enum. Move internal verification, parent/topology, upvar derivation,
   normalized-graph construction, and projection-facing iteration from the
   Function-only map to a root-neutral owner-product view. Preserve
   `forest.owner()`/`forest.owners()` as Function-only compatibility APIs, and
   do not construct a Script product or connect production. Keep the work-plan
   file untouched and split helpers into bounded siblings if needed.

7. `SEMANTIC-OWNER-SCRIPT-PORT0-S3`
   **No standalone row.** A port-only change has no real Script product
   producer/consumer at the current boundary and would create a carrier-only
   API. Do not add an empty typed carrier, a ProgramBody-only validator, or an
   unused source-view loan. Fold this responsibility into the producer-backed
   Literal I0/R0 below: only a co-sealed Script product may create the typed
   immutable source-view loan, and the selected port must consume exact
   `ProgramBody(original ordinal)` sites. Raw/reference ports remain unchanged.

8. `RAW-SCRIPT-LITERAL-SEMANTIC-OWNER0-I0-R0`
   After S0/S1/S1D/S2 are green, connect the selected normal lifecycle at
   the existing production caller
   `ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle`.
   This row includes the S3 integration; there is no prior port-only commit.
   The only semantic admission seam is after CatalogSeal/work-plan prepare
   and before CatalogInstall. Complete candidates must use the shared forest
   enum/generic-core accessors; the existing Function-only `forest.owner()`
   and `forest.owners()` compatibility APIs must not be used for Script.
   Complete closure is exactly empty/literal-only Script runtime items plus
   transferred top-level FunctionDeclaration boundaries. Co-seal Program,
   Script root profile, one shared forest, one Program projection, and exact
   coverage; issue no owner for Deferred candidates. Remove only the selected
   Complete branch's `RawInvocationSourceTransportV1::script_root(())` edge.
   Deferred and raw/reference routes retain their existing script_root exactly
   once. First real fixture is
   `tools/checks/fixtures/raw_vm_reference_conformance/integer_0.hako`, read
   once and passed through a selected NormalCompileRequest; Raw VM CLI success
   is not evidence. Literal I0 must not add Variable/Local materialization,
   Lambda capture, ValueId/ABI, control, call/object, or Box runtime demand.

## RAW-SCRIPT-LITERAL-SEMANTIC-OWNER0-I0-R0 — closed

```text
Decision:
  Candidate A-prime landed for the bounded empty/literal-only Script
  Complete closure with one shared forest/projection and a typed source loan.

Production seam:
  CatalogSeal/work-plan prepare -> ScriptSemanticSeal -> CatalogInstall
  -> one selected root lowering.

Deferred:
  non-literal selected Script demands keep the existing root compatibility
  owner; no owner/forest/projection is issued for those requests.

Atomic retirement:
  Complete no longer calls bare `script_root(())`.
  Deferred and raw/reference routes retain their existing transport.

Evidence:
  integer_0.hako selected NormalCompileRequest = green
  Script product/projection and invalid-ordinal tests = green
  focused lifecycle/runtime tests = green
  pointer/CUT0 guards = green
  all touched source/check files < 800 lines.

Non-effects:
  grammar, result policy, raw/reference behavior, Lambda/Variable/Local,
  control, call/object, Box runtime demand, fallback, and retry unchanged.

Next:
  fresh live-edge census; do not preselect the next Script responsibility.
```

## MIRBUILDER-LIVE-EDGE-CENSUS42-D0 — closed

```text
Scope:
  read-only census limited to the shared Script/root descent after the
  literal Complete cutover.

Finding:
  the only selected-normal old edge is the intentional Deferred branch
  `program_root_lowering.rs` -> `RawInvocationSourceTransportV1::script_root(())`.
  Raw/reference callers and test-only ScriptRoot transports are not selected
  production edges. The compatibility sunset remains active.

Next design:
  RAW-SCRIPT-LEXICAL-BINDING0-D0
```

## RAW-SCRIPT-LEXICAL-BINDING0-I0-R0 — closed at 7bf6c9b996

```text
Decision:
  Candidate A-prime. Extend the existing Script semantic product and shared
  forest; do not create a second Script resolver, whole-program variant, or
  new forest/projection authority.

Selected Complete closure:
  source-order Script runtime window containing only:
    - Literal expressions
    - bare Variable expressions that resolve to a prior Script-root Local
    - Local with exactly one untyped declaration and one initializer expression
      whose recursive expression closure is Literal or Variable
  Empty sequence remains Complete. Transferred top-level callable boundaries
  remain outside the Script forest. Any unsafe sibling defers the whole
  request once.

Semantic product owns:
  exact ProgramBody(original ordinal) sites, Script root profile, shared forest
  and projection, Local BindingRef/name/declaration site, Variable source site
  and resolved lexical ref, initializer-to-BindingRef relation, and exact
  owner/scope/declaration/use coverage.

Semantic product does not own:
  ValueId, MirType, ABI/slot, BindingId allocation, SSA/PHI, ownership,
  capture materialization, MIR emission, or publication.

Lowering bridge:
  the selected Complete source loan creates one request-local
  `BindingRef -> ValueId` ledger. Local initializer lowering evaluates first,
  materializes once through the existing Local owner, then records the exact
  BindingRef mapping only after success. Variable lowering consumes the ledger
  by resolved BindingRef. Complete must not fall back to name-based
  `variable_map`/`build_variable_access`; Deferred/raw/reference retain those
  existing routes.

Production caller:
  ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle
  remains the only caller. Admission is selected once after work-plan prepare;
  Complete/Deferred is chosen before CatalogInstall and RootLower.

Atomic old-edge deletion:
  selected Complete Local/Variable lowering's name-based/raw Local edges are
  removed in the same I0/R0 commit. The Deferred `script_root(())` edge remains
  until every deferred reason in this sunset is retired.

Deferred and precedence:
  source-level undefined/redeclaration/unsafe-shape cases choose Deferred up
  front and let ExistingRootLower emit the existing diagnostic once. Only
  invariant/coverage/forest/projection failures are ScriptSemanticSeal errors;
  Complete never downgrades to Deferred and no retry is allowed.

Fixture and ratchet:
  existing `raw_vm_reference_conformance/local.hako` is the first declaration
  milestone. Add an inline focused `local x = 1; x` read case in the existing
  normal test module. `integer_0.hako` must remain Complete. Complete fixture
  identity is monotonic; Deferred reason vocabulary cannot grow without a new
  D0. Sunset: `SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001`.

Ceremony:
  T2; one atomic I0/R0 commit. The D0 decision is closed; no S0-only
  producer or carrier is permitted.

Hard stops:
  Variable/Local facts without exact BindingRef sites, name-based fallback in
  Complete, ValueId/ABI/SSA in the semantic product, partial forest, second
  resolver pass, mixed selected/Deferred items, explicit Return/assignment/
  control/Lambda/call/object/Box runtime demand, program_root_work_plan.rs
  edits, new per-row guard, or any touched source/check file >=800 lines.
```

Evidence:
  `normal_script_lexical_binding.rs` owns the source-only lexical admission
  and request-local ledger. `ResolvedScriptSemanticDraftV1` publishes exact
  Local declaration and Variable-use BindingRef facts into the existing shared
  forest; it does not own ValueId, ABI, SSA, or MIR. Complete lowering
  intercepts only the selected concrete invocation port, materializes Local
  through the existing Local owner, and reads Variables through the ledger.

Focused proof:
  `normal_script` tests are green, including real
  `raw_vm_reference_conformance/local.hako`, nested `local y = x`, and the
  existing failure/reuse and parity fixtures. `integer_0.hako` remains green.
  `current_state_pointer_guard`, the shared cut0 guard, and all touched source
  files remain below 800 lines.

Atomic deletion:
  selected Complete Local/Variable lowering no longer uses the raw
  `build_variable_access`/name-map route. Deferred, raw, and reference Script
  calls retain `script_root(())`; no fallback or retry was added.

Next design:
  MIRBUILDER-LIVE-EDGE-CENSUS43-D0 — read-only, narrow census of the
  remaining selected-normal live edge. Do not preselect the next Script
  semantic family.

Ratchet and sunset:
  `SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001` tracks the Deferred owner.
  The manifest must track fixture identity sets and exact Deferred reasons,
  not only a Complete percentage. Existing Complete fixtures may never regress
  to Deferred; a new reason requires a new D0. The first Complete milestone is
  `integer_0.hako` with LiteralOnlyV1.

Hard stops:
  synthetic FunctionDeclaration, a second Script forest/projection,
  FunctionSyntaxView Program widening, body-root union inference, a
  standalone Script port/carrier, work-plan reconstruction, partial forest,
  Complete-to-Deferred downgrade,
  raw Lambda capture order from `forest.upvars()`, editing
  `program_root_work_plan.rs`, or touching raw/reference `script_root` in the
  Literal row.
```

## MIRBUILDER-LIVE-EDGE-CENSUS43-D0 — closed

```text
Finding:
  The selected-normal graph has exactly one remaining old source edge:
  `program_root_lowering.rs` Deferred Script ->
  `RawInvocationSourceTransportV1::script_root(())`. Complete
  Literal/Variable/Local uses the semantic source loan. Raw/reference
  `script_root` calls are tests or explicit compatibility, not this edge.

Decision:
  No safe T1 remains without selecting a semantic family. Worker review
  compared ordinary Unary with Print and selected ordinary Unary as the
  smallest recursive expression extension; Print remains a later statement
  effect row despite its real fixture candidate.

Sunset:
  `SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001` remains active. The
  Deferred edge is removed only when its exact deferred reasons are retired
  or explicitly retained at final conformance.
```

## RAW-SCRIPT-UNARY-LEXICAL-CLOSURE0-I0-R0 — closed at 1adb617542

```text
Change:
  Extend the existing Script Complete admission with one recursive
  expression constructor: Unary(Minus|Not|BitNot, E), where E is the
  current Literal or prior-root Local-backed Variable closure. Use the
  existing port-aware unary owner and exact UnaryOperand source paths.
  Safe Unary requests stop using the Deferred `script_root(())` edge in the
  same I0/R0 commit; unsafe Unary, Weak, Print, control, calls/objects, Box,
  Lambda, and mixed unsafe trees remain Deferred.

Contract:
  Reuse the existing Script semantic source, shared forest/projection, and
  request-local BindingRef-to-ValueId ledger. Record nested Variable sites
  structurally; do not add ValueId/ABI/SSA to the semantic product, create a
  second resolver/forest, or retry through raw compatibility.

Done:
  `local x = 1; -x` and a nested Unary source-path fixture are selected
  normal parity cases; integer_0.hako and existing lexical fixtures remain
  green. The selected old Deferred edge is zero for safe Unary, the shared
  replacement guard is green, and every touched source/check file is <800
  lines.

Evidence:
  `cargo check --lib`, the focused `normal_script` suite (24/24), the
  integer_0 fixture, and general-module parity are green. The selected
  Unary path uses exact `UnaryOperand` source segments and the shared raw
  port-aware owner; the shared pointer/cut0 guards are green and touched
  source files remain below 800 lines.

Atomic deletion:
  Safe ordinary Unary no longer selects the Deferred
  `RawInvocationSourceTransportV1::script_root(())` edge. Weak and all
  residual statement/control/call/object/Box/Lambda shapes retain their
  existing Deferred/raw/reference ownership; no fallback or retry was added.

Next design:
  `MIRBUILDER-LIVE-EDGE-CENSUS44-D0` — fresh, read-only, narrow census of
  the selected-normal graph. Do not preselect the next Script family.

Stop:
  Return to design if exact nested source receipts cannot be sealed in one
  traversal, Weak/operator-box/short-circuit semantics enter the closure,
  a new materialization owner is needed, or Complete failure would need
  Deferred/fallback/retry.
```

## RAW-SCRIPT-PRINT-LEXICAL-CLOSURE0-I0-R0 — closed at c1c7852b76

```text
Decision:
  Candidate A — Print(E) over the existing Script lexical closure

Ceremony:
  T2, one atomic I0/R0 commit

Change:
  Admit DirectPrint in the same Script lexical admission when its operand is
  the existing Complete expression closure: Literal, prior-root Local-backed
  Variable, or ordinary Unary(Minus|Not|BitNot) recursively. Reuse
  lower_direct_print_v1 and the existing BindingRef-to-ValueId ledger.

Source contract:
  ProgramBody(original ordinal) -> PrintValue -> UnaryOperand...
  Variable facts are recorded at exact nested SourceExprSite paths. The Print
  ordinal is included in expression_source_indices; the work plan is not
  rebuilt or reclassified.

Atomic delete:
  Safe Print no longer selects Deferred
  RawInvocationSourceTransportV1::script_root(()). Unsafe Print operands,
  Weak/Binary/Check/Call/Object/Box/Lambda, raw, and reference routes retain
  their existing ownership. No fallback or retry.

Fixtures:
  print(1), local x = 1; print(x), and print(-x) with normal/legacy MIR,
  verification, failure/reuse, and real-file parity.

Evidence:
  `cargo check --lib`, the focused `normal_script` suite (27/27),
  `print.hako` parity, pointer/cut0 guards, and touched-file line limits
  are green. DirectPrint remains the only print lower owner; TypeOp and
  effect/publication policy were not changed.

Atomic deletion:
  Safe Print no longer selects Deferred
  `RawInvocationSourceTransportV1::script_root(())`. Raw/reference and
  unsafe Print routes retain their existing transport and no retry/fallback
  was added.

Next design:
  `MIRBUILDER-LIVE-EDGE-CENSUS45-D0` — fresh, read-only, narrow census of
  the selected-normal graph. Do not preselect the next Script family.

Hard stops:
  Print-specific semantic/failure owner, operand TypeOp/Call/Object/Weak,
  mixed selected/compatibility items, second traversal, ledger name fallback,
  program_root_work_plan.rs edits, or any touched source/check file >=800.
```

## RAW-SCRIPT-BINARY-LEXICAL-CLOSURE0-I0-R0 — closed at b562263854

```text
Decision:
  Candidate A — ordinary Binary lexical closure

Ceremony:
  T2, one atomic I0/R0 commit

Change:
  Extend the Script Complete expression closure with Binary(op, E, E) for
  ordinary operators only. E is the existing Literal, prior-root
  Local-backed Variable, ordinary Unary, or Print(E) closure. And/Or stay
  Deferred under the existing short-circuit CFG owner.

Source contract:
  ProgramBody(original ordinal) -> BinaryLeft/BinaryRight recursively,
  preserving any PrintValue/UnaryOperand prefixes. Variable facts remain
  BindingRef/source-site receipts only; Binary ordinals join
  expression_source_indices and the work plan is not rebuilt.

Lowering:
  Reuse drive_ordinary_binary_expression_v1 through the same RawInvocation
  port and ledger. No Binary-specific semantic owner, type/ABI fact, MIR
  variant, or failure owner.

Atomic delete:
  Safe ordinary Binary no longer selects Deferred
  RawInvocationSourceTransportV1::script_root(()). And/Or, unsafe children,
  calls/objects/fields/new, Weak, control, Box, Lambda, raw, and reference
  routes retain their existing ownership.

Fixtures:
  1+2, local x = 1; x+2, nested Binary, print(x+1), and And/Or Deferred
  parity/reuse cases.

Evidence:
  `cargo check --lib`, the focused `normal_script` suite (30/30),
  general-module parity, integer_0 parity, pointer/cut0 guards, and touched
  source line limits are green. Existing binary, unary, and print owners
  remain the only lowering authorities.

Atomic deletion:
  Safe ordinary Binary no longer selects Deferred
  `RawInvocationSourceTransportV1::script_root(())`. And/Or, unsafe
  children, raw/reference, and all residual control/object/Box/Lambda routes
  retain their existing ownership; no fallback or retry was added.

Next design:
  `MIRBUILDER-LIVE-EDGE-CENSUS46-D0` — fresh, read-only, narrow census of
  the selected-normal graph. Do not preselect the next Script family.

Hard stops:
  short-circuit inclusion, mixed routing, type/ABI inference, second
  resolver, name fallback, program_root_work_plan.rs edits, new Binary
  owner/variant, or any touched source/check file >=800.
```

## RAW-SCRIPT-AWAIT-LEXICAL-CLOSURE0-I0-R0 — closed at 074e944fec

```text
Decision:
  Candidate A — Await(E) over the existing Script lexical closure

Ceremony:
  T1, one atomic I0/R0 commit

Change:
  Admit Await recursively when E is Literal, prior-root Local-backed
  Variable, ordinary Unary, ordinary Binary(op != And/Or), Print(E), or
  another safe Await. Print(Await(E)) composes through the same closure.

Source contract:
  ProgramBody(original ordinal) -> AwaitOperand recursively, preserving
  BinaryLeft/Right, UnaryOperand, and PrintValue prefixes. Await ordinals
  join expression_source_indices; the work plan is not rebuilt.

Lowering:
  Reuse build_await_expression_with_port_v1 through the same RawInvocation
  port and ledger. Preserve operand -> Safepoint -> Await -> type propagation
  -> Safepoint exactly; no Future/ABI/type policy or Nowait activation.

Atomic delete:
  Safe Await no longer selects Deferred
  RawInvocationSourceTransportV1::script_root(()). Call/Object/Field/New/
  Array/Check/Weak/AndOr/Nowait/control/Box/Lambda, raw, and reference
  routes retain their existing ownership.

Fixtures:
  await 1, local f = 1; await f, await -(1+2), print(await 1), unsafe
  operand rejection/reuse, and normal/legacy MIR+verification parity.

Hard stops:
  safepoint order change, Future/ABI inference, mixed routing, child retry,
  second resolver, name fallback, new Await owner/variant, work-plan edits,
  or any touched source/check file >=800.

Evidence:
  `normal_script` 32/32, selected Await lexical MIR/verification parity,
  general-module parity, integer_0 fixture parity, `cargo check --lib`,
  current-state pointer guard, and cut0 guard are green. The three touched
  Rust files remain below 800 lines. Code landed in 074e944fec.

Atomic deletion:
  Safe Await no longer selects the Deferred Script `script_root(())` edge;
  unsafe operands and raw/reference routes retain their existing owners.

Next design:
  `MIRBUILDER-LIVE-EDGE-CENSUS47-D0` — fresh, read-only, narrow census.
  Do not preselect Check, And/Or, control, calls/objects, Box, or Lambda.

## RAW-SCRIPT-CHECK-LEXICAL-CLOSURE0-I0-R0 — closed at a78d4e968a

```text
Decision:
  Candidate A — recursive CheckExpr closure over the existing Script lexical closure

Ceremony:
  T1, one atomic I0/R0 commit

Accepted shape:
  CheckExpr(items*) where every item.expression is Literal, prior-root
  Local-backed Variable, ordinary Unary, ordinary Binary (And/Or excluded),
  Print, Await, or another safe CheckExpr.

Source contract:
  labels and name are preserved but are not admission authority. Each item
  is observed source-only through existing CheckItem paths in source order;
  the intact CheckExpr parent produces one prepared receipt per item before
  the item vector is moved. No AST clone, reconstruction, or work-plan rebuild.

Lowering:
  Reuse build_check_expression_with_port_v1. Keep eager item evaluation,
  Const 1/0 emission, child -> Select -> type commit order, and the same
  selected port. No Bool admission constraint is added.

Atomic deletion:
  Safe CheckExpr no longer selects Deferred Script `script_root(())`.
  A CheckExpr with any unsafe child remains one explicit Deferred request;
  item-level mixed routing and retry are forbidden.

Evidence required:
  recursive admission fixtures (empty, nested, Unary/Await/Binary wrappers),
  normal/legacy MIR+verification parity with a lexical Variable under CheckItem,
  source-order recording, failure/reuse, pointer/cut0 guards, and all touched
  source/check files below 800 lines.

Hard stops:
  short-circuiting, Bool type policy, item-level route mixing, new Check owner
  or failure type, selected retry, AST clone/reparse, new guard/test file, or
  any touched source/check file reaching 800 lines.

Evidence:
  `normal_script` 34/34, selected Check lexical MIR/verification parity,
  Check owner order/failure tests, `cargo check --lib`, current-state pointer
  guard, and cut0 guard are green. The touched Rust files and shared guard
  remain below 800 lines. Code landed in a78d4e968a.

Atomic deletion:
  Safe CheckExpr no longer selects the Deferred Script `script_root(())`
  edge. Unsafe items remain one Deferred request; no item-level retry or
  mixed routing was added.

Next design:
  `MIRBUILDER-LIVE-EDGE-CENSUS48-D0` — fresh, read-only, narrow census.
  Do not preselect And/Or, control, calls/objects, Box, Weak, or Lambda.

## RAW-SCRIPT-ANDOR-LEXICAL-CLOSURE0-I0-R0 — closed at 1f17bc93d1

```text
Decision:
  Candidate A — And/Or over the existing short-circuit expression owner

Ceremony:
  T1, one atomic I0/R0 commit

Accepted shape:
  BinaryOperator::And or BinaryOperator::Or whose left and right trees are
  already in the Script lexical closure: Literal, prior-root Local-backed
  Variable, ordinary Unary, ordinary Binary, Print, Await, or CheckExpr.

Owner boundary:
  Reuse drive_short_circuit_expression_v1 and the existing logical
  short-circuit CFG/PHI/result/type/diagnostic owners. Existing
  RawStructuredChildScopePortV1 consumes left then conditional-right source
  receipts; no new branch or semantic owner is introduced.

Source/semantic contract:
  Remove only the source-only lexical admission rejection for And/Or and
  recurse through BinaryLeft/BinaryRight. Runtime conditional demand remains
  the existing short-circuit contract; both lexical facts are source paths,
  not a request to evaluate RHS eagerly.

Atomic deletion:
  Safe And/Or no longer selects Deferred Script `script_root(())`. Unsafe
  descendants remain one Deferred request; item-level routing, retry, and
  fallback are forbidden.

Evidence required:
  And and Or with literal operands, Local-backed variables on both sides,
  nested Unary/Await/Check/Binary composition, RHS conditional-failure parity,
  fresh-request reuse, normal/legacy MIR+verification parity, existing
  short-circuit owner tests, pointer/cut0 guards, and touched files below 800.

Hard stops:
  eager RHS lowering, CFG/PHI/result policy edits, variable-map merge changes,
  new short-circuit owner/failure type, mixed routing, second resolver, retry,
  AST clone/reparse, new guard/test file, or any source/check file reaching 800.
```

Evidence:
  `cargo test --lib mir::builder::normal_script` = 34/34 passed, including
  nested And/Or MIR+verification parity and a missing-RHS failure followed by
  fresh-request reuse. The existing short-circuit owner test
  `and_and_or_share_the_existing_short_circuit_completion`, the general-module
  parity test, `cargo check --lib`, pointer guard, and cut0 guard are green.
  `normal_script_lexical_binding.rs` is 472 lines,
  `normal_script_semantic_source.rs` is 564 lines, and the shared guard is 794
  lines; no new source/test/check file was added.

Atomic deletion:
  And/Or trees inside the existing lexical closure no longer select Deferred
  Script `script_root(())`. Unsafe descendants still select one Deferred
  request, and the existing raw compatibility path remains the sole fallback
  boundary for those shapes. No eager RHS demand, CFG/PHI edit, or retry was
  introduced.

Next design:
  `MIRBUILDER-LIVE-EDGE-CENSUS49-D0` — fresh, read-only, narrow census.
  Do not preselect control, calls/objects, Box, Weak, or Lambda.
```
```
```

## MIRBUILDER-LIVE-EDGE-CENSUS49-D0 — closed, NoSafeSlice

```text
Decision:
  NoSafeSlice

Evidence:
  The remaining Script lexical candidates all cross an authority that is not
  part of the current source-only closure. QMark/Match/Enum/TryCatch/If/Loop
  require control, exit, or cleanup facts. Call/Field/New/Array/Map/Record
  require header, allocation, object, or metadata facts. Weak requires a
  BoxRef/type precondition; Lambda requires forest/capture/ClosureBodyId;
  Box requires catalog/lifecycle ownership. Existing `*_with_port_v1`
  functions are lowering owners, not semantic admission producers.

  The shared Deferred edge remains
  `program_root_lowering.rs` -> `RawInvocationSourceTransportV1::script_root(())`.
  It cannot be deleted without a complete producer for one selected family.

Non-effects:
  no code change, no new owner, no new resolver, no fallback/retry,
  no broad source scan, no collector-drain restart. Collector drain is already
  closed at `67488ff283`, and the existing Deferred sunset/ratchet SSOT is
  sufficient; neither is reopened as a docs-only substitute.

Next design stop:
  `RAW-SCRIPT-SEMANTIC-CLOSURE-BOUNDARY1-D0`
  Choose exactly one semantic family with a named production consumer,
  one traversal/coverage contract, existing-lowering parity, fresh-request
  reuse evidence, and the existing Deferred sunset/ratchet integration.

Hard stops:
  whole Script closure, multiple families in one row, synthetic Function,
  partial forest, second resolver, fallback/retry, semantic diagnostic
  reordering, ValueId/ABI materialization, Lambda/Control/Call/Box mixed
  activation, or a new source/check file over 800 lines.
```

## RAW-SCRIPT-SEMANTIC-CLOSURE-BOUNDARY1-D0 — closed, Candidate A-prime

```text
Decision:
  Candidate A-prime accepted

Selected family:
  StaticConstTable + DirectStaticConstRuntimeCompletion

Reason:
  StaticConst is a zero-child transfer boundary. Its metadata and runtime
  completion owners already exist, so the Script semantic product needs only
  exact ProgramBody source coverage. It does not need control, call/object,
  allocation, capture, Binding, or ABI authority.

Consultation:
  docs/development/current/main/investigations/
    raw-script-semantic-closure-boundary1-design-consultation-question-2026-08-01.md

Worker corrections:
  locate StaticConst through a dedicated zero-child source predicate;
  source/admission mismatch is a typed ScriptSemanticSeal invariant rejection;
  table type/value diagnostics remain RootLower; the existing manifest ratchet
  must be consumed by the shared guard rather than stored as inert JSON.

Next:
  RAW-SCRIPT-STATIC-CONST-SEMANTIC-CLOSURE0-I0-R0
  docs/development/current/main/investigations/
    raw-script-static-const-semantic-closure0-i0-r0-execution-task-2026-08-01.md
```

## RAW-SCRIPT-SELECTED-UNSUPPORTED-SEMANTIC-CLOSURE0-I0-R0 — closed

```text
Decision:
  Accept the existing DirectSelectedUnsupportedStatement family.

Exact family:
  LoopRange / Break / Continue / ImportStatement / BuildGate /
  EnumDeclaration / BrandDeclaration / TypeAliasDeclaration / GlobalVar
  paired with DirectSelectedUnsupportedStatement.

Replacement:
  exact family -> UnsafeRuntimeStatement -> Deferred -> script_root(())
  becomes
  exact zero-demand diagnostic receipt -> VerifiedScriptSemanticSourceV1
  -> exact ProgramBody source -> unchanged RootLower diagnostic terminal.

Required:
  one admission traversal; whole-request Complete or Deferred; exact original
  ordinal; diagnostic text/stage/order unchanged; source/admission mismatch is
  ScriptSemanticSeal invariant rejection; fallback/retry 0.

Proof:
  compact the 797-line shared guard into a table-driven fixture-ID consumer;
  do not add a guard; freeze the near-limit integration/source-transport tests;
  every source/check file remains below 800 lines.

Excluded:
  Using; semantic Control; Call/Object; Allocation; Weak; Lambda; Box; Outbox;
  child traversal; new diagnostic authority; partial request routing.

Closeout:
  exact nine-kind admission is one Complete zero-demand receipt; the same
  ProgramBody source reaches the unchanged diagnostic terminal; wrong pairing
  is a hard semantic-seal invariant; mixed residual requests remain wholly
  Deferred. Admission 12/12, semantic source 15/15, nine-kind diagnostic
  parity/reuse, source-order parity, release build, pointer guard, and shared
  cut0 guard are green. Maximum touched source/check file is 797 lines.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS51-D0 at the zero-demand batch boundary.
```

## MIRBUILDER-LIVE-EDGE-CENSUS50-D0 — closed

```text
Three bounded read-only audits compared the remaining production families.
The existing selected-unsupported diagnostic family is the smallest clean
replacement. Using was rejected for this row because it would require a new
admission/terminal no-op family. No external consultation remains open.
```

## RAW-SCRIPT-STATIC-CONST-SEMANTIC-CLOSURE0-I0-R0 — closed

```text
Ceremony:
  T2, one atomic I0/R0 commit

Atomic replacement:
  exact StaticConst/admission pair
  -> UnsafeRuntimeStatement -> Deferred -> script_root(())
  becomes
  -> typed zero-child completion receipt
  -> VerifiedScriptSemanticSourceV1
  -> script_semantic_root

Required:
  one admission traversal; exact original Program ordinal; one RootLower;
  dedicated located source context; existing metadata/runtime owners;
  invalid table diagnostics remain RootLower; whole-request Deferred for any
  unsupported sibling; no Complete-to-Deferred downgrade; fallback/retry 0.

Evidence:
  exact-pair and mismatch tests; original-ordinal coverage; parser-backed
  positive source; MIR plus static metadata parity; invalid-table precedence;
  failure discard/fresh reuse; mixed-request Deferred; consumed identity
  ratchet. No corpus or benchmark harness is run.

Hard stops:
  Control/Call/Object/Allocation/Weak/Lambda/Box; table semantics copied into
  semantic source; partial request routing; second resolver/forest; new
  source/test/check file; inert manifest key; any source/check file >= 800.

Closeout:
  the exact StaticConst/admission pair is Complete; the semantic source owns
  one typed zero-child ProgramBody receipt; the runtime terminal receives the
  same located source; source/admission mismatch is a hard invariant error;
  table diagnostics remain RootLower; mixed unsupported requests remain
  wholly Deferred; the existing identity ratchet is consumed by the shared
  guard. Focused tests, release build, pointer guard, and shared cut0 guard are
  green. No corpus or benchmark harness was run.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS50-D0
  Run one bounded production-edge census. Do not preselect the next family.
```

## SEMANTIC-OWNER-SCRIPT-PORT0-S3 — closed, NoStandaloneRow

```text
Finding:
  The current RawInvocationChildPortV1 carries raw source/path transport only.
  No co-sealed Script forest/projection product is produced yet, and the
  forest/projection consumers still expose Function-only compatibility APIs.
  A standalone typed Script loan would therefore be an unused carrier or a
  second source authority rather than a production replacement.

Decision:
  Do not land an S3-only commit. Integrate the typed Script source-view loan
  inside RAW-SCRIPT-LITERAL-SEMANTIC-OWNER0-I0-R0 after the Complete Script
  product exists. Delete the selected Complete branch's bare `script_root(())`
  edge in that same atomic commit; keep Deferred and raw/reference calls.

Non-effects:
  no standalone port API
  no ProgramBody validator without a producer
  no second forest or projection
  no behavior change

Next:
  RAW-SCRIPT-LITERAL-SEMANTIC-OWNER0-I0-R0
  Complete closure remains empty/literal-only runtime items plus transferred
  top-level FunctionDeclaration boundaries. All other Script demands stay
  Deferred with the existing compatibility owner and sunset ledger.
```

## SEMANTIC-OWNER-ROOT-PROFILE0-S0 — closed

```text
Decision:
  Behavior-neutral BoxShape refactor landed. `SemanticOwnerRootProfileV1`
  now owns the exact source/body-root contract for DeclaredFunction, Script,
  and Lambda. Existing Function/Lambda views and public source_kind accessors
  remain behavior-compatible; source kind is derived from the profile rather
  than stored as a second product authority.

Evidence:
  cargo test --lib resolved_semantics:: --quiet
    = 163 passed
  cargo check --lib --quiet
    = passed (pre-existing warnings only)
  New profile module:
    src/mir/resolved_semantics/owner_root_profile.rs
    = 78 lines

Non-effects:
  Script production consumer = 0
  forest payload change      = 0
  projection/lowering cutover= 0
  grammar / result / fallback = unchanged

Next:
  SEMANTIC-OWNER-CORE0-S1
  Keep the forest Function API stable while introducing the private generic
  owner core and profile-exact lowering root witness. Do not add Script
  production or edit program_root_work_plan.rs.
```

## SEMANTIC-OWNER-CORE0-S1A — closed

```text
Decision:
  Behavior-neutral product-core refactor landed. The resolved semantic
  payload is now wrapped by private `VerifiedResolvedOwnerCoreV1`; the
  existing `VerifiedResolvedFunctionV1` remains the public Function/Lambda
  wrapper and delegates through that core. A private Script wrapper type is
  reserved without a constructor or production consumer.

Non-effects:
  forest payload             = unchanged
  lowering-root verification = unchanged
  Script production consumer = 0
  grammar / result / fallback = unchanged

Evidence:
  cargo check --lib --quiet
    = passed (pre-existing warnings only)
  cargo test --lib resolved_semantics:: --quiet
    = 163 passed
  touched source/check files = all <800 lines

Next:
  SEMANTIC-OWNER-CORE0-S1B
  Move only the forest payload/dispatch into a bounded sibling module. Keep
  `forest.owner()` and `forest.owners()` Function-only for API compatibility;
  do not construct a Script owner or connect a port.
```

## SEMANTIC-OWNER-CORE0-S1B — closed

```text
Decision:
  The forest now stores one shared `VerifiedSemanticOwnerProductV1` payload
  enum (`Function`/`Script`) in a bounded sibling module. The current producer
  still emits only the Function variant, so behavior and production reachability
  are unchanged. Existing `forest.owner()` and `forest.owners()` remain
  Function-only compatibility accessors; `semantic_owners()` is internal for
  the later Script consumer.

Evidence:
  cargo check --lib --quiet
    = passed (pre-existing warnings only)
  cargo test --lib resolved_semantics:: --quiet
    = 163 passed
  owner_forest.rs             = 714 lines
  owner_forest_payload.rs     = 29 lines

Non-effects:
  Script owner construction   = 0
  Script port / fixture       = 0
  lowering/projection change  = 0
  grammar / result / fallback = unchanged

Next:
  SEMANTIC-OWNER-CORE0-S1C
  Make lowering-root verification consume the exact profile body root instead
  of scanning the FunctionBody/LambdaBodyRoot union. Keep the existing type
  name as a compatibility alias and add mismatch tests only.
```

## SEMANTIC-OWNER-CORE0-S1C — closed

```text
Decision:
  Lowering-root verification now consumes `SemanticOwnerRootProfileV1` for
  exact body-root matching. `ResolvedOwnerLoweringRootsV1` is the neutral
  carrier, with `ResolvedFunctionLoweringRootsV1` retained as a compatibility
  alias. FunctionBody, ProgramBodyRoot, and LambdaBodyRoot are no longer a
  union search; a profile/root mismatch is rejected before publication.

Evidence:
  cargo check --lib --quiet
    = passed (pre-existing warnings only)
  cargo test --lib resolved_semantics:: --quiet
    = 164 passed
  function_root.rs             = 128 lines
  function_root_tests.rs       = 252 lines

Non-effects:
  Script construction/consumer = 0
  projection / port / fixture = 0
  ScopeKind/RegionKind storage = unchanged
  grammar / result / fallback = unchanged

Next:
  SEMANTIC-OWNER-PROJECTION0-S2
  Generalize only the private projection seal to profile-exact Function vs
  Program roots. Keep FunctionSourceViewV1 Function/Lambda-only and do not
  connect a Script production consumer yet.
```

## SEMANTIC-OWNER-PROJECTION0-S2 — closed

```text
Decision:
  The private projection seal now derives the root contract from
  `SemanticOwnerRootProfileV1`. Function roots still require
  `FunctionDeclaration`; Script roots are structurally recognized as
  `Program` without widening `FunctionSyntaxViewV1`. A separate
  `ScriptSyntaxViewV1` sibling now carries an owned Program borrow and the
  Script profile, with no builder/work-plan dependency.

Evidence:
  cargo check --lib --quiet
    = passed (pre-existing warnings only)
  cargo test --lib source_projection --quiet
    = 13 passed
  cargo test --lib script_view --quiet
    = 1 passed
  source_projection.rs        = 528 lines
  script_view.rs              = 61 lines

Non-effects:
  Script forest construction  = 0
  Script port / production    = 0
  raw/reference behavior      = unchanged
  grammar / result / fallback = unchanged

Next:
  SEMANTIC-OWNER-FOREST-DISPATCH0-S1D
  Complete the shared forest's internal root-neutral dispatch before any
  Script producer or port loan. A carrier-only rename is forbidden; the
  selected port must consume exact ProgramBody(original ordinal) sites only
  after a real Script product can be co-sealed. Keep raw/reference ports
  intact.
```

## SEMANTIC-OWNER-FOREST-DISPATCH0-S1D — closed

```text
Decision:
  Behavior-neutral shared-dispatch refactor landed. The forest draft now
  stores the shared `VerifiedSemanticOwnerProductV1` payload boundary from
  insertion through verification; internal parent/topology, upvar,
  normalized-graph, and projection-facing paths use root-neutral product
  accessors. `insert_product` is available for the future Script producer,
  while the current production producer remains Function-only.

Compatibility:
  `forest.owner()` and `forest.owners()` remain Function-only compatibility
  APIs. Script-facing code must use `semantic_owner()`/`semantic_owners()`.
  No Script constructor, port, production caller, grammar, or fallback was
  added by this row.

Evidence:
  cargo check --lib --quiet = passed
  cargo test --lib owner_forest --quiet = 13 passed
  cargo test --lib source_projection --quiet = 13 passed
  current-state-pointer-guard.sh = passed
  owner_forest.rs = 718 lines
  owner_forest_payload.rs = 163 lines

Next:
  RAW-SCRIPT-LITERAL-SEMANTIC-OWNER0-I0-R0
  Build the real producer-backed Literal-only Script product and integrate
  the typed source-view loan in the same selected Complete cutover. Do not
  add a standalone S3 carrier or touch raw/reference `script_root` edges.
```

## RAW-SCRIPT-LAMBDA-CAPTURE-BINDING-BRIDGE0-D0 — closed, Accept(A-prime)

```text
Finding:
  The Program-specific Script root is the correct semantic unit, but the raw
  Script route has no exact BindingRefV1 -> ValueId authority. Its variable_map
  is name-based, while canonical identity/SSA owners are function/CFG products
  and cannot be partially activated for Lambda alone.

Decision:
  Use one invocation-local Script materialization ledger. Eligible Local
  completion publishes one exact semantic BindingRef-to-ValueId row; selected
  Variable reads and Lambda captures consume only that ledger. A semantic
  capture plan owns exact child owner/definition site/parent scope and capture
  rows in first-demand order. `forest.upvars()` ordering is not an ABI.

Receiver:
  Script root receiver is absent. Direct `Me` stays deferred and never enters
  the named-capture lane.

Refactor series:
  1. RAW-LAMBDA-CLOSURE-EMISSION-TERMINAL0-S0 extracts the existing
     NewClosure/body-id reserve -> emit -> commit terminal without behavior
     change.
  2. RAW-SCRIPT-LAMBDA-CAPTURE-BINDING-BRIDGE0-I0-R0 connects the ledger and
     semantic capture plan, then removes the eligible Script Lambda raw
     observer/ControlBody edge in the same commit.

Forbid:
  name-to-Value pairing as semantic authority; copying legacy BindingId into
  BindingRefV1; a second reaching-value map for selected Script reads; capture
  ordering from owner IDs/BTreeSet; whole canonical CFG/SSA activation;
  fallback/retry; ABI or ClosureBodyId publication-order changes.

Hakorune syntax:
  source try/throw remain rejected. Protected syntax is postfix catch/cleanup;
  ASTNode::TryCatch is only an internal/legacy carrier and this row adds no
  first-catch or catch-binder semantics.
```

## Current execution

`NORMAL-TOPLEVEL-FUNCTION-CALLABLE-IDENTITY0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  selected Program top-level functions now issue one source-order occurrence
  receipt and separate legacy physical admission in the existing work plan;
  the selected capture port consumes it while raw/reference retains the raw
  work item.

Delete:
  selected normal top-level FunctionDeclaration -> raw static method
  `LegacyChildDraftAdmissionV1` = 0.

Evidence:
  normal-vs-legacy general-module MIR/function-set parity, including duplicate
  `name/arity` last-wins; source-order/physical receipt tests; late body
  failure then fresh reuse; raw/reference receipt = 0; shared guard; and all
  touched source/check files below 800 lines.

Registry:
  `NORMAL-TOPLEVEL-FUNCTION-CALLABLE-COMPAT-SUNSET-003` is closed.  The older
  combined `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001` is narrowed
  to constructors only, so no active row silently retains top-level scope.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS20-D0`; do not preselect another replacement.
```

## Latest census

`MIRBUILDER-LIVE-EDGE-CENSUS23-D0` — read-only, closed: expression facade selected for D0

```text
Remaining selected compatibility:
  54 exact kinds
  = PortAwareExpression 7
  + StatementControl 17
  + DeclarationIngress 9
  + CallObjectHeader 21

Observed seven-kind edge:
  Literal / Variable / Me / Unary / Binary / Await / Check
  -> NormalScriptRuntimeStatementAdmissionV1::RawCompatibility
  -> drive_legacy_statement_v1
  -> RawInvocationChildPortV1::lower_statement
  -> build_statement_with_port_v1
  -> drive_legacy_expression_v1

Finding:
  build_statement_with_port_v1 has no kind-specific policy for these roots. It
  writes the root span once, then the raw expression dispatcher writes the same
  span and lowers through the same RawInvocation port. Descendant MethodCall,
  New, Field, Call, or control-sensitive shapes therefore do not need a new
  allowlist: they retain the exact selected port they already receive.

Selected design stop:
  NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-D0

Required D0:
  decide one direct drive_legacy_expression_v1 handoff for all seven roots;
  preserve block order/suffix/termination and every existing expression owner;
  delete only the seven-kind statement-facade edge in the later atomic I0/R0.

Not selected:
  Nowait has Future/type/binding/slot publication ordering; Local/Assignment,
  control/exit, declaration/ingress, and call/object/header remain separate.

R4 census:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  LegacyChildDraftAdmissionV1 remains 37 occurrences / 8 source files.
```

`MIRBUILDER-LIVE-EDGE-CENSUS22-D0` — read-only, closed: non-Box Script residual registered

```text
Selected production edge:
  selected normal Program -> Script runtime work
  -> NormalScriptRuntimeBlockPortV1::lower_statement
  -> RawCompatibility -> drive_legacy_statement_v1

Decision:
  Direct BoxDeclaration is retired by the preceding I0, but every direct
  non-Box Script statement still reaches this broad compatibility edge. Record
  `NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003` in the sole R4 registry
  before selecting its disposition. It is a live selected residual, not an
  unregistered R4 family and not a `LegacyChildDraftAdmissionV1` occurrence.

Next:
  `NORMAL-SCRIPT-NONBOX-STATEMENT-DISPOSITION0-D0`. It must inventory the
  exact non-Box kind families, identify port-neutral/statement/control/call
  boundaries, and select at most one source-only partition with a named old-edge
  delete. Do not open an I0 from the catch-all branch.

R4 census:
  retain-fenced=2, active compatibility=2, closed=4, unregistered=8.
  `LegacyChildDraftAdmissionV1` remains 37 occurrences / 8 source files.
```

## Current design decision

`NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-D0` — T1, closed

```text
Decision:
  Candidate A — move the complete 21-kind CallObjectHeader family to the
  existing DirectPortAwareExpression terminal as one production responsibility.

Selected kinds:
  QMarkPropagate / MatchExpr / EnumMatchExpr / ArrayLiteral / MapLiteral /
  RecordLiteral / RecordUpdate / Lambda / BlockExpr / Arrow /
  GroupedAssignmentExpr / MethodCall / FieldAccess / Index / New / This /
  FromCall / ThisField / MeField / FunctionCall / Call.

Parity basis:
  build_statement_with_port_v1 does not intercept any selected kind. It writes
  the root span and immediately calls drive_legacy_expression_v1; the raw
  expression dispatcher then writes the identical span and uses the same
  RawInvocation port. Header loans, collector visibility, allocation/type/birth
  effects, QMark/Match control, Lambda/BlockExpr lifecycle, arbitrary children,
  diagnostics, and failure remain owned by the existing expression terminals.

Atomic delete:
  all 21 selected Script roots
  -> CallObjectHeaderCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0.

Structure:
  reuse normal_script_direct_statement_owner exactly; add no product, owner,
  route terminal, test/check file, or per-row guard. Delete the retired
  CallObjectHeaderCompatibility category. Residual count becomes 26.

Evidence:
  exhaustive source partition; representative full MIR/verification/diagnostic
  parity across header/collector, allocation, control, nested lifecycle, and
  currently unsupported members; distinct root/child spans; late failure then
  fresh compiler reuse; selected statement-facade edge zero.

Not selected:
  Return is a valid later direct statement responsibility with completion and
  cleanup pairing. StaticConstTable is a valid later metadata-runtime Void
  completion. Neither enters this expression-facade row.

Forbid:
  child allowlists; new header/collector/allocation/control policy; selecting
  only successful members; raw/reference widening; retry/fallback/reselection;
  a second port; AST reparse/clone beyond current terminals; or any source/check
  file reaching 800 lines.

Next:
  NORMAL-SCRIPT-CALL-OBJECT-DIRECT-EXPRESSION0-I0-R0.
```

`NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-D0` — T1, closed

```text
Decision:
  Candidate A — retire the statement facade for all seven direct expression
  roots as one production responsibility.

Selected roots:
  Literal / Variable / Me / UnaryOp / BinaryOp / AwaitExpression / CheckExpr.
  Descendants are unrestricted and keep the exact existing RawInvocation port.

Old path:
  PortAwareExpressionCompatibility
  -> RawCompatibility
  -> drive_legacy_statement_v1
  -> build_statement_with_port_v1
  -> drive_legacy_expression_v1

New path:
  DirectPortAwareExpression
  -> selected direct-statement sibling
  -> drive_legacy_expression_v1 with the same port

Parity basis:
  build_statement_with_port_v1 has no policy for these roots. Its only extra
  operation is writing the root span immediately before the expression
  dispatcher writes the identical span. Header, collector, Box, Loop, Call,
  child failure, and nested expression routes remain on the same port.

Structure:
  normal_script_runtime_work.rs is already 798 lines. Create one small
  normal_script_direct_statement_owner sibling, move the existing DirectPrint
  terminal into it, and add the expression terminal there. The source-only
  disposition file must not gain Builder/lowering authority.

Atomic delete:
  the seven roots -> RawCompatibility -> drive_legacy_statement_v1 = 0.
  Residual kind count becomes 47; compatibility terminal count remains one.

Evidence:
  exhaustive/disjoint source partition; full MIR/verification parity across
  all seven roots and nested call/object/control-sensitive descendants; exact
  root/child span parity; undefined-variable failure then fresh reuse; no new
  RawLegacy port, build_expression facade, retry, or fallback.

Forbid:
  operand allowlists; new expression semantics; Nowait or another statement
  kind in this row; block-driver bypass; raw/reference widening; AST reparse;
  new failure/source identity; or any source/check file reaching 800 lines.

Next:
  NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-I0-R0.
```

`NORMAL-SCRIPT-NONBOX-STATEMENT-DISPOSITION0-D0` — T1 partition, closed

```text
Observed edge:
  selected normal Script
  -> NormalScriptRuntimeBlockPortV1::RawCompatibility
  -> drive_legacy_statement_v1
  -> RawInvocationChildPortV1::lower_statement

Correction:
  This is not a RawLegacy-port fallback. The selected RawInvocation child port
  already crosses the facade unchanged. The debt is the broad statement
  dispatcher and its catch-all admission.

Total direct non-Box inventory:
  55 AST kinds exactly; direct BoxDeclaration is already owned by the preceding
  I0, and top-level FunctionDeclaration is consumed by immediate work.

Selected first slice:
  Print, with its complete current expression surface. Existing
  PreparedRawPrintV1 source observation, TypeOp/general route, child descent,
  diagnostics, and output emission remain the sole semantics.

Residual partition:
  port-aware expression family                    = 7
  statement/control/state family excluding Print = 17
  declaration/ingress family                     = 9
  call/object/header-sensitive family             = 21
  total residual                                  = 54

Why no operand allowlist:
  Both old and new Print routes use the same selected invocation port. A
  Literal-only or port-neutral-only Print slice would narrow the replacement
  for testing convenience without protecting a real authority boundary.

Next:
  NORMAL-SCRIPT-PRINT-DIRECT-OWNER0-I0-R0
  -> classify the 55-kind partition once
  -> direct Print to PreparedRawPrintV1 and its existing lower terminal
  -> delete Print -> RawCompatibility -> drive_legacy_statement_v1
  -> keep all 54 residual kinds at one compatibility terminal

Forbid:
  grammar/result/publication changes; Print TypeOp re-observation; a second
  child port; raw/reference widening; selected failure -> compatibility retry;
  block-driver/suffix bypass; AST clone/reparse; new failure/source identity;
  or selecting another residual kind in the same I0.
```

## Current closeout

`NORMAL-SCRIPT-PORT-AWARE-EXPRESSION-DIRECT-OWNER0-I0-R0` — T1 atomic cutover, closed

```text
Change:
  Move the existing DirectPrint terminal into one small selected
  direct-statement sibling and add a direct expression handoff for all seven
  previously classified expression roots.

Direct owner:
  Literal / Variable / Me / Unary / Binary / Await / Check
  -> drive_legacy_expression_v1
  -> the exact same RawInvocation child port and existing expression owners

Delete:
  seven direct expression roots
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0

Parity:
  Full normal/legacy MIR, verification, diagnostic and distinct-span outcomes
  are exact across all roots. A nested FunctionCall remains on the same port.
  Undefined-variable failure discards the candidate and a fresh request reuses
  the compiler. No operand allowlist, grammar change, retry, or fallback exists.

Structure:
  normal_script_direct_statement_owner.rs = 191 lines,
  normal_script_runtime_work.rs = 790,
  shared guard = 799; every source/check file remains below 800.

Registry:
  NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003 remains active and narrows
  from 54 to 47 exact kinds. The expression family leaves the residual; the
  statement/control, declaration/ingress, and call/object/header families remain
  at one compatibility terminal. No fence or compatibility terminal was added.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS24-D0; select nothing before the fresh census.
```

`NORMAL-SCRIPT-PRINT-DIRECT-OWNER0-I0-R0` — T1 atomic cutover, closed

```text
Change:
  One exhaustive source-only disposition owner partitions all 57 AST kinds:
  direct Box and top-level Function remain owned elsewhere, Print is selected,
  and the other 54 direct non-Box kinds remain in four compatibility families.

Direct owner:
  Print -> PreparedRawPrintV1
        -> lower_prepared_raw_print_with_port_v1
        -> the same RawInvocation child port

Delete:
  selected Script Print
  -> RawCompatibility
  -> drive_legacy_statement_v1
  = 0

Parity:
  General and TypeOp Print routes keep current source observation, expression
  descent, diagnostics, MIR, verification, block order, and failure/reuse.
  No operand allowlist, grammar change, new port, retry, or fallback exists.

Structure:
  the new source file owns only the total disposition; production/test/check
  files are 798 / 201 / 799 lines at closeout and all remain below 800.

Registry:
  NORMAL-SCRIPT-NONBOX-STATEMENT-COMPAT-SUNSET-003 remains active but is
  narrowed from 55 to 54 exact kinds. No new fence or compatibility terminal
  was created.

Next:
  MIRBUILDER-LIVE-EDGE-CENSUS23-D0; do not preselect another AST responsibility.
```

`NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-DISPOSITION0-I0-R0` — T1, closed

```text
Change:
  Replace the selected Script runtime's direct BoxDeclaration
  RawCompatibility -> drive_legacy_statement_v1 branch with one total direct-Box
  partition.  The I0/R0 delete is only the direct-Box use of that broad raw
  branch; non-Box Script statements retain their current raw compatibility.

Contract:
  non-sync/non-Main static Box -> existing selected non-Main static lifecycle;
  non-sync instance Box -> existing full selected instance lifecycle, retaining
  its second Script demand through the already-issued constructor source batch
  and cataloged method admissions; sync Box -> the current fail-fast diagnostic
  at the same runtime statement point; static Main -> its separately fenced
  invocation-port compatibility terminal. No new Box semantics, identity, collector
  key/policy, source read/clone, result/publication policy, or fallback/retry.

Evidence:
  every direct Script Box is selected exactly once by the new source-only
  partition; no direct Box reaches drive_legacy_statement_v1. Normal/legacy
  parity covers generic instance callable output, Script's repeated instance
  demand, sync diagnostic/order, and failure-then-fresh-reuse.

Registry:
  `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-COMPAT-SUNSET-002` is closed. The raw
  statement compatibility now owns non-Box Script statements only; raw/reference
  and nested Box descent remain outside this row.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS22-D0`; do not preselect another I0.
```

`MIRBUILDER-LIVE-EDGE-CENSUS21-D0` — read-only, closed: non-plain Script Box D0 selected

```text
Selected production edge:
  selected normal Program -> Script runtime work
  -> NormalScriptRuntimeBlockPortV1::lower_statement
  -> RawCompatibility -> drive_legacy_statement_v1

Decision:
  `NORMAL-SCRIPT-NONPLAIN-BOX-CALLABLE-DISPOSITION0-D0`.
  `RawCompatibility` is a catch-all for non-Box Script statements too, so no
  direct I0 is safe.  D0 must decide one total, source-only partition for only
  direct non-plain BoxDeclaration shapes, with one exact new owner or a
  parity-equivalent pre-descent rejection per shape.  It must leave non-Box
  statements, raw/reference, Script order, and fallback/retry unchanged.

Not selected:
  nested Box raw body descent remains the separately unregistered R4 family:
  it crosses Main/static/instance/constructor/header/collector lifecycle and
  therefore needs NESTED-BOX-RAW-BODY-DISPOSITION0-D0 first.

R4 census:
  registry before I0 was retain-fenced=2, active compatibility=2, unregistered=8,
  closed=3. LegacyChildDraftAdmissionV1 remains a separate 37-occurrence / 8
  source-file observation, not an owner-disposition count.  The sole registry
  now names exact active owner anchors and a named-or-forced-R4 release path.
```

`MIRBUILDER-LIVE-EDGE-CENSUS20-D0` — read-only, closed: constructor D0 selected

```text
NoSafeLiveI0:
  No remaining selected-normal LegacyChild edge can be deleted without a new
  source authority.  Raw static-Main remains explicit raw compatibility; Script
  non-plain Box remains its separately registered broad disposition.

Candidate:
  `NORMAL-INSTANCE-CONSTRUCTOR-CALLABLE-IDENTITY0-D0` (T2).
  The one selected-normal residual is the instance-constructor path:
    selected Program immediate instance Box (including Script non-plain)
    + selected Script plain-instance prefix
    -> constructor batch -> raw instance child terminal -> LegacyChild.
  Existing ordinary instance methods are cataloged already and remain excluded.

Registry correction:
  `NORMAL-UNCATALOGUED-PROGRAM-CHILD-COMPAT-SUNSET-001` covers selected
  Program immediate constructors for every instance Box and the second
  selected Script plain-instance-prefix demand.  Script non-plain Boxes also
  have that immediate demand; only their later full raw runtime lifecycle is
  owned by the separately fenced non-plain Script surface.
  Selected function-body nested `BoxDeclaration` can independently reach the
  raw recursive child terminal, so it is registered as an unregistered R4
  family.  It is not folded into the Program/root constructor row.

Constructor D0 must decide:
  source identity from exact Box occurrence plus parser-owned
  `init|pack|birth/arity` key; Script's immediate-plus-runtime constructor
  demand law; physical receiver arity; and unchanged LegacySymbol +
  LegacyReplaceWholePair parity.  It must not widen the Box-method catalog,
  issue receipts for raw/reference, change collector policy, or add retry.
```

## Current closeout

`NORMAL-INSTANCE-CONSTRUCTOR-CALLABLE-IDENTITY0-I0-R0` — T2 atomic selected-normal cutover, closed

```text
Change:
  Program work-plan issues `{statement index, Box name, parser init|pack|birth/arity}`
  once per parser-normalized constructor row. Every selected Program instance
  Box consumes one source-keyed admission; selected Script plain-prefix gets
  a second physical admission from the same transported source occurrence.

Delete:
  selected constructor direct `LegacyChildDraftAdmissionV1` construction = 0.
  Raw/reference and non-plain Script raw-runtime edges do not consume a
  selected receipt and remain outside this closed row.

Evidence:
  parser-key order/non-Function skip, Script two-demand source transport,
  non-plain receipt exclusion, App and Script normal/legacy MIR parity,
  late-failure fresh reuse, shared guard, pointer guard, and lib check = green.

Next:
  `MIRBUILDER-LIVE-EDGE-CENSUS21-D0`; do not preselect another I0.
```

## Latest closeout

```text
JOINMODULE-PHI-RETURN-STRATEGY-REOWN0-I0-R0
  Builder finalization owns Direct -> primary hint -> P3-D -> P4 -> P3-C
  phi_type_inference + JoinIR TypeHintPolicy/GenericTypeResolver + exports = 0
  focused strategy + normal parity + lib/vm-reference + lifecycle/lane/pointer = green
  next = fresh census13 D0
```

## Latest closeout

```text
JOINMODULE-OWNERSHIP-ANALYSIS-RETIRE0-RET0
  join_ir/ownership tree, module export, stale private-BindingId inventory/docs = 0
  semantic rehome / normal-default / VM-LLVM bridge / feature delta               = 0
  scoped build + seam/lane/pointer guards                                          = green
  next                                                                             = fresh census D0
```

## Latest design decision

`MIRBUILDER-LIVE-EDGE-CENSUS11-D0` — closed

```text
Normal root, child descent, collector, finalization, and publication have no
safe remaining live competing edge. The 13-file join_ir/ownership analysis
asset is caller-zero outside its module export, so RET0 is selected; all other
JoinIR/bridge/EdgeArgs surfaces remain independently fenced or rehome work.
```

## Prior design decision

`NORMAL-COLLECTOR-DRAIN-LIFECYCLE0-D0` — closed

```text
Candidate C-prime: reuse normal drain semantics in a normal-owned lifecycle,
and bind it to the existing candidate-session brand. Raw and canonical drains
remain incompatible family/receipt owners, not adapters. The brand is session
correspondence only; it does not reclassify normal work as a raw route.
```

## Latest closeout

```text
NORMAL-COLLECTOR-DRAIN-LIFECYCLE0-I0-R0
  selected normal collector: existing session brand -> one normal receipt -> ordered commit
  old normal_legacy_drain module and selected caller                                  = 0
  general Program function-set/MIR/metadata parity; collision/reuse; lib/vm-ref/gates = green
  next                                                                                = census11 D0 (closed)
```

## Latest closeout

```text
JOINMODULE-VERIFY-REFERENCE-RET0
  join_ir::verify module / export / progress-select test closure / stale inventory = 0
  verify_phi_reserved / VM bridge / LowerOnly / normalized shadow / LLVM             = unchanged
  focused if-select + lib/vm-reference checks + lane guards                           = green
  next                                                                                 = fresh R3 census D0
```

## Prior selection

```text
JOINMODULE-REFERENCE-LIVE-EDGE-CENSUS2-D0
  caller-zero legacy join_ir/ownership analysis                     = retirement candidate
  AST frontend                                                       = larger test/fixture closure
  VM bridge / normalized shadow / LLVM / phi observer / carriers    = fenced or separate
  consecutive detached RET0                                          = 3; fourth RET0 prohibited
  next                                                               = live-edge census10 D0
```

## Prior selection

```text
MIRBUILDER-LIVE-EDGE-CENSUS10-D0
  Program/root final collector drain                                = only live residual
  existing raw/canonical drains                                    = incompatible ownership
  expression/statement port                                        = already selected; NoSafeLiveI0
  next                                                              = normal collector-drain lifecycle D0
```

## Prior selection

```text
JOINMODULE-REFERENCE-LIVE-EDGE-CENSUS1-D0
  selected caller-zero verifier closure; all other R3 surfaces stayed fenced,
  retained, or separate pending a fresh post-retirement census.
```

## Latest closeout

```text
JOINMODULE-JSONIR-V0-REFERENCE-RET0
  serializer / snapshots / export / env helpers / duplicate manifest evidence = 0
  Stage-B bridge tests                                                        = retained
  focused Stage-B tests + lib/vm-reference checks + lane guards              = green
  normal/default, bridge, strict, LLVM delta                                 = 0
  next                                                                        = fresh R3 census D0
```

## Latest closeout

```text
GENERIC-CASE-A-APPEND-DEFS-RET0
  generic append lowerer / selector vocabulary / ValueId range = 0
  current two-input helper / non-append generic Case-A routes    = retained
  selector and ValueId tests + lib/vm-reference checks           = green
  bridge, normal/default, strict, LLVM delta                     = 0
  next                                                           = R3 reference census D0
```

## Latest closeout

```text
JOINMODULE-LOWERONLY-STALE-DESCRIPTOR-RET0
  stale bridge target / Loop exclusion / Case-A name facade = 0
  generic ArrayAccumulation lowerer / range / shape route   = retained
  target, Case-A, Loop predicate tests + lib checks          = green
  normal/default, bridge behavior, strict, LLVM delta        = 0
  next                                                       = generic asset D0
```

## Latest closeout

```text
JOINMODULE-NORMALIZED-SHADOW-RETIRE0-D0
  default normal normalized-shadow execution                         = 0
  explicit dev/debug direct execution + observer                     = retained-fenced
  normal authority / grammar / result / publication delta            = 0
  new fallback/retry approval                                         = 0
  next                                                               = explicit-reference D0
```

## Latest closeout

```text
JOINMODULE-DIRECT-RUNNER-RETIRE0-RET0
  direct JoinIR runner / test-only callers / module export            = 0
  HMI caller inventory                                                   = 10 -> 8
  normal/default / VM bridge / LLVM experiment behavior                 = unchanged
  failure-outcome inventories / default + vm-reference builds / guards  = green
  next                                                                   = VM bridge fence D0
```

## Latest closeout

```text
JOINMODULE-VM-BRIDGE-FENCE0-D0 / DOC0
  route                           = explicit vm-reference --backend vm only
  default mir / vm-fallback       = 0 reachability
  activation                       = NYASH_JOINIR_VM_BRIDGE=1 only
  Exec / LowerOnly / nonstrict VM continuation = retained-fenced
  stale gate and stdout contract   = corrected without behavior change
  sunset                           = VM-BRIDGE-COMPAT-SUNSET-001
  next                             = strict policy D0
```

`VM-BRIDGE-COMPAT-SUNSET-001` owns only
`join_ir_vm_bridge_dispatch` from the explicit VM keep route. It retires when
that dispatcher caller reaches zero or a separately selected one-execution
owner replaces the entire explicit lane; it does not authorize normal/default
fallback, VM bridge growth, or LLVM changes.

## Latest closeout

```text
JOINMODULE-VM-BRIDGE-STRICT-POLICY0-D0 / STRICT-ALIAS0-I0-R0
  strict authority                 = HAKO_JOINIR_STRICT || NYASH_JOINIR_STRICT
  changed surface                  = explicit VM bridge Exec failure only
  global JoinIR strict helper      = unchanged
  LowerOnly / dev-trace success    = unchanged and retained-fenced
  dual-alias policy tests          = green
  next                             = LowerOnly target alignment D0
```

## Previous closeout

```text
JOINMODULE-METHOD-RETURN-HINT-REOWN0-I0-R0
  selected P3-D normal finalization observation                    = private owner
  obsolete JoinIR helper import/call/module/file                   = 0
  resolver order / type-annotation policy / grammar / publication  = unchanged
  focused policy tests / normal parity / candidate reuse / guards  = green
  fallback / retry                                                 = 0
  next                                                             = normalized-shadow D0
```

## Previous closeout

```text
DERIVED-CONDITIONFN-SHADOW-RETIRE0-RET0
  direct condition_fn generated bundle / generator / dedicated guards = 0
  bounded-finalize and function-region-stack evidence                = refreshed
  aggregate and strict-converter evidence                             = refreshed
  raw root condition draft / JoinModule / normal-default routes      = unchanged
  focused lifecycle, artifact, parity, reuse, cargo check            = green
  next                                                                = JoinModule carrier-boundary D0
```

## Census9 closeout

```text
MIRBUILDER-LIVE-EDGE-CENSUS9
  Program/root/lifecycle                  = NoSafeLiveI0
  finalization/call                       = NoSafeLiveI0
  raw/reference compatibility             = NoSafeLiveI0
  no-header Call                          = separate caller-zero D0 only
  R2                                      = closed
  next                                    = JoinModule/reference-asset disposition
```

## Latest closeout

```text
FINALIZE0-CONDITIONFN-RET0-I0-R0 / ba8c111974
  finalizer missing-symbol injection             = 0
  Call materializer name-special const-1 path    = 0
  minimal lifecycle / normal parity / reuse      = green
  RawRequiredConditionDraftV1                    = unchanged
  next                                           = R2 live-edge census
```

## Census8 closeout

```text
MIRBUILDER-LIVE-EDGE-CENSUS8
  publication/pipeline                   = NoSafeLiveI0 (sole commit terminal)
  Program root / raw compatibility        = NoSafeLiveI0
  finalization metadata projections       = already replaced
  selected bounded design                 = FINALIZE0-CONDITIONFN-RET0-D0
  JoinModule                              = remains R3-only; not reactivated
```

## Latest closeout

```text
MODULE-FINALIZATION-FUNCTION-METADATA0-I0-R0

prepared function-metadata owner                      = exactly one
selected inline type/origin-caller projection          = 0
type hints -> owner commit -> return/PHI inference     = preserved
unit / normal general parity / candidate reuse / guard = green
fallback / retry / grammar / result / publication delta = 0
new source/test/check file                             = 1 / 0 / 0
largest touched source/check file                      < 800
next                                                   = fresh live-edge census
```

## Previous closeout

```text
NORMAL-PROGRAM-DEFERRED-STATIC-CONTEXT0-I0-R0

selected direct context open/clear                                 = 0
private scoped context owner                                       = exactly one
prior None/Some restored on success, error, and unwind             = green
method order, primary error, callable capture                      = preserved
non-Main static candidate failure -> fresh corrected reuse         = green
shared guard / focused tests / cargo check                         = green
fallback / retry / grammar / collector / finalization delta        = 0
new source/test/check file                                         = 0 / 0 / 0
largest touched source/check file                                  < 800
next                                                               = fresh live-edge census
```

## Previous closeout

```text
MIRBUILDER-LIVE-EDGE-CENSUS5

safe live T1 replacement                                           = none
selected normal live boundary                                      = deferred static context
selected next stop                                                 = T2 D0
detached RET0 selected                                             = 0 (three-row horizon closed)
raw/static-Main, no-header Call, Loop/CorePlan                     = separate D0 only
JoinModule normal/default execution                                = 0; R3 disposition required
JoinModule current inventory                                       = 34,212 LOC
next                                                               = deferred-static context D0
```

## Previous closeout

```text
NORMAL-PROGRAM-STATIC-TABLE-PLAN0-I0-R0

`PreparedNormalProgramStaticTableMetadataV1`                     = exactly one
selected direct source collect/plan/two metadata writes           = 0
facts -> paired static-table metadata -> work-plan/body           = preserved
source order, diagnostics, candidate discard/reuse                = preserved
static-table unit / existing static-const tests / guard           = green
fallback / retry / grammar / result / finish / publication delta  = 0
new source/test/check file                                        = 1 / 0 / 0
largest touched source/check file                                 < 800
next                                                              = fresh live-edge census
```

## Previous closeout

```text
RAW-INDIRECT-CALL-LEGACY-FACADE-RETIRE0-RET0

`build_indirect_call_expression` facade                           = 0
ambient production `RawLegacyChildLoweringPortV1`                  = 0
raw dispatcher -> with-port indirect-Call owner                   = exactly one
raw-port Call regression / shared guard / cargo check             = green
fallback / retry / grammar / result / Call policy delta           = 0
new source/test/check file                                        = 0 / 0 / 0
largest touched source/check file                                 < 800
next                                                              = fresh live-edge census
```

## Previous closeout

```text
RAW-CHECK-LEGACY-FACADE-RETIRE0-RET0

`build_check_expression` facade                              = 0
ambient production `RawLegacyChildLoweringPortV1`             = 0
raw dispatcher -> with-port Check owner                       = exactly one
Check unit / raw-port integration / shared guard / cargo check= green
fallback / retry / grammar / result / control delta           = 0
new source/test/check file                                    = 0 / 0 / 0
largest touched source/check file                             < 800
next                                                          = fresh live-edge census
```

## Latest closeout

```text
RAW-QMARK-LEGACY-FACADE-RETIRE0-RET0

`build_qmark_propagate_expression` facade                = 0
ambient `RawLegacyChildLoweringPortV1` construction       = 0
raw dispatcher -> with-port QMark owner                  = exactly one
raw-port QMark regression / shared guard / cargo check   = green
fallback / retry / grammar / result / control delta      = 0
new source/test/check file                               = 0 / 0 / 0
largest touched source/check file                        < 800
format check                                             = unrelated pre-existing diffs
next                                                     = fresh live-edge census
```

## Latest closeout

```text
NORMAL-PROGRAM-COLLECTOR-DRAIN0-I0-R0

selected direct `collector.into_draft_functions()`                 = 0
selected direct `try_add_functions_atomic(drafts)`                 = 0
prepared final-row normal legacy drain -> atomic commit            = exactly one
legacy symbol/replacement admission and RootLower mapping          = preserved
normal general parity / collision / reuse / imports                = green
fallback / retry / grammar / result / publication delta            = 0
new source/test/check file                                         = 1 / 0 / 0
largest touched source/check file                                  < 800
next                                                               = fresh live-edge census
```

## Latest closeout

```text
CALL-GLOBAL-PRESENCE-LEGACY-FACADE-RETIRE0-RET0

Call global-presence facade/direct module observation = 0
authority-aware resolver live emitter                 = exactly one
LegacyCompatibility no-header authority               = retained
header authority / lane guards                         = green
grammar / result / dialect / fallback delta           = 0
new source/test/check file                            = 0 / 0 / 0
next                                                  = fresh live-edge census
```

## Previous closeout

```text
RAW-LEGACY-EXPRESSION-FACADE-RETIRE0-I0-R0

raw expression facade/module/input-view boundary       = 0
sole port-aware raw matcher                             = retained
raw static-Box / Lambda direct-matcher evidence        = green
raw dispatcher unit suite / shared lane guard          = green
grammar / result / route / fallback delta              = 0
new source/test/check file                             = 0 / 0 / 0
next                                                   = fresh live-edge census
```

## Previous closeout

```text
PROGRAM-ROOT-WORK-PARTITION0-I0-R0

source-only source-ordered work plan                  = exactly one
selected mixed Program statement coordinator           = 0
facts -> static-table -> immediate -> deferred -> terminal = preserved
runtime non-Function/Box retention                     = preserved
Script/App terminal and collector failure authority    = unchanged
normal general parity / candidate reuse / imports      = green
fallback / retry / grammar / result delta              = 0
new source/test/check file                             = 1 / 0 / 0
largest touched source/check file                      < 800
next                                                   = fresh live-edge census
```

## Previous closeout

```text
NORMAL-DEFAULT-PROGRAM-DECLARATION-FACTS0-I0-R0

source-only source-ordered facts product             = exactly one
selected `declaration_indexer` file/module/caller    = 0
catalog -> facts -> static-table -> body              = preserved
Brand / Enum / record defaults / Box-field-weak       = preserved
static-scalar updates retain source-order removal     = preserved
normal general parity / candidate reuse / imports     = green
fallback / retry / grammar / result delta             = 0
new source/test/check file                            = 1 / 0 / 0
largest touched source/check file                     < 800
next                                                  = fresh live-edge census
```

## Previous closeout

```text
RAW-THROW-DEBUG-TRACE-COMPAT-RETIRE0-I0-R0

statement_surface Throw -> prepare/lower                = exactly once
NYASH_BUILDER_DISABLE_THROW definition/read/docs         = 0
debug completion/enum/field/fixture/guard residue        = 0
physical Throw completion                                = sole
Throw unit / normal parity / failure-reuse / imports      = green
fallback / retry / grammar delta                          = 0
new source/test/check file                              = 0
largest touched source/check file                       < 800
```

## Forward task order

This is a dependency order, not a pre-authorized construction queue.  A row
opens only when its predecessor's evidence is green and the required fresh
census or D0 has selected it.

```text
1. ENTRY-MATERIALIZATION-RECEIPT0-S0                    (closed)
   Source-only normal/raw request, target, and receipt vocabulary.  No Builder,
   collector, ledger, runner, or old-edge effect.

2. ENTRY-MATERIALIZATION-NORMAL-CONSUMPTION0-I0-R0      (closed)
   Named caller: NormalDefaultPublishedPipelineV1.
   Consume the normal source receipt through the existing one-session lifecycle
   and delete the selected lower-side environment snapshot/materialization edge.
   Raw/reference and every runner selector retain their current authority.

3. MIRBUILDER-LIVE-EDGE-CENSUS15-D0                     (closed)
   Re-inventory selected normal, raw/reference, and runner materialization
   consumers.  It may select exactly one bounded D0, one live I0/R0, or
   NoSafeLiveI0; it may not assume a raw handoff or runner cutover in advance.

4. NORMAL-RUNTIME-INPUT-SNAPSHOT0-D0                    (closed)
   Candidate N selects one infallible normal-only ingress receipt.  It preserves
   normal's permissive malformed-value behavior; raw/reference remains separate.

5. NORMAL-RUNTIME-INPUT-SNAPSHOT0-I0-R0                 (closed)
   Named caller: NormalDefaultPublishedPipelineV1::compile.  One atomic
   normal-only receipt cutover deletes the two selected lower-side ambient reads.

6. MIRBUILDER-LIVE-EDGE-CENSUS16-D0                     (closed)
   Re-inventory the remaining selected-normal, compatibility, and fenced
   reference edges before selecting another live replacement or retirement.

7. NORMAL-CALLABLE-DRAFT-IDENTITY-AND-ADMISSION0-D0     (closed)
   Selected catalog-addressable Box-method replacement; uncatalogued children
   are explicit compatibility, not omitted coverage.

8. NORMAL-CATALOGED-BOX-METHOD-DRAFT-ADMISSION0-I0-R0   (closed)
   One atomic selected-normal static/instance Box-method source-witness cutover.

9. MIRBUILDER-LIVE-EDGE-CENSUS17-D0                     (active)
   Fresh selection after cataloged Box-method admission; no successor is
   presumed before live consumer evidence.

10. Entry-materialization residuals                     (census-selected only)
   A raw/reference receipt handoff and each runner-adapter receipt are separate
   responsibility decisions.  They must preserve their route-specific policies:
   no global selector, no `NYASH_ENTRY` reinterpretation, and no provenance
   collapse.  Their shared completion goal is the removal of the old snapshot /
   compilation-context / lower-side materialization authority, not a new route.

11. R3 reference-asset disposition                      (interleaved only by census)
   Each cycle is fresh consumer census -> one RET0, REOWN, or RETAIN-FENCED
   decision -> fresh census.  These rows earn no replacement credit.  The VM
   bridge, normalized shadow, LLVM experiment, and any live carrier remain
   named fences until their own evidence changes.

12. R4 final conformance
   Decide every live edge, compatibility sunset, and retained reference asset.
   The 34K-line JoinModule scope is decided here as either deletion or an
   explicit fenced reference asset; LOC is not a completion metric.  Complete
   requires normal/default reachability=0, acceptance truth=0, and final
   planner=0 for every retained reference family.

13. R5 features, strictly after R4 Complete
   Refresh Ownership readiness -> implement Ownership -> View D0 and I0 -> one
   later unimplemented feature semantic slice at a time.
```

## Task-selection rules

```text
Live I0/R0:
  named non-test production caller + new owner + same-series old-edge deletion
  + parity/failure/reuse evidence.  fallback/retry = 0.

Prerequisite S0:
  allowed only as the immediate, explicitly named predecessor of its I0/R0.
  A second proof-only row cannot be stacked onto it.

Compatibility owner:
  one bounded residual branch inside the selected pipeline, never a second
  route.  Creation or retention records sunset ID, exact non-growing surface,
  retirement owner, retire condition, and target row/evidence.  Any expansion
  returns to D0.

RET0:
  removes only a registered caller-zero asset, earns no replacement credit, and
  cannot revive JoinModule as a normal/default planner.

Frozen until R5:
  whole-function acceptance variants, Ownership, View, and feature work.
```

The observed shelves—header-sensitive Call, selected-invocation Loop/CorePlan,
raw/static-Main, function-state/control residuals, and raw AST/Recipe
composition—are census input, not an implementation queue.  JoinModule remains
outside R2 replacement commits but inside R4 completion: do not delete it by
name or silently inherit its final disposition.

## Previous closeout

```text
RAW-LAMBDA-LEXICAL-CAPTURE-LIFECYCLE0-I0-R0

raw dispatcher Lambda edge                  = lifecycle once
build_lambda_expression / exprs_lambda.rs   = 0
capture order                                = lexical first demand
missing capture / direct Me                  = pre-effect failure
nested Lambda / Function / Box               = pre-effect failure
closure metadata                             = reserve -> emit -> commit
raw Lambda / normal reuse / general parity   = green
fallback / retry / compatibility             = 0
new source/test/check file                   = 2 / 0 / 0
largest touched source/check file            < 800
```

## Previous closeout

```text
RAW-STATIC-MAIN-COMPAT-FACADE-RETIRE0-I0-R0

implementation commit                        = b23b654aa7
RawLegacyChildLoweringPort direct prepared handoff = exactly 1
build_static_main_box facades                = 3 -> 0
fresh RawLegacyChildLoweringPort construction = 1 -> 0
helper/root order and error mapping          = preserved
focused raw/verified Main ordering           = green
normal parity / failure / reuse              = green
release build / pointer / lane guards        = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
sunset manifest                              = active / facade edges 0
```

## Previous closeout

```text
VERIFIED-MAIN-ROOT-BODY-LOWERING-HANDOFF0-I0-R0

implementation commit                       = c0929f8171
selected root().source() lowering handoff    = 0
selected lower-side FunctionDeclaration rematch = 0
selected late missing/not-function errors    = 0
typed verified root payload handoff          = exactly 1
raw Main / callable-main / root behavior delta = 0
Main expansion/order/failure tests           = green
general module parity / failure / reuse      = green
release build / pointer / lane guards        = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## Previous closeout

```text
VERIFIED-MAIN-STATIC-CHILD-LOWERING-HANDOFF0-I0-R0

implementation commit                       = 67085bec4f
selected helper lower-side AST rematch       = 0
late static-child-source rejection           = 0
typed verified child payload handoff         = exactly 1
raw Main / root / callable-main identity delta = 0
expansion / helper order / failure tests     = green
general module parity / failure / reuse      = green
release build / current pointer / guards     = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## Latest closeout

```text
RAW-PORT-AWARE-COMPOUND-EXPR-OWNED-INPUT0-I0-R0

implementation commit                       = 1e73b9180f
immediate compound-expression deep clones   = 8 -> 0
Call / QMark / Await / Record owner APIs     = unchanged
child order / diagnostics / RootLower delta = 0
focused Call/QMark/Await/Record tests        = green
general module parity / failure / reuse      = green
release build / current pointer / lane guard = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## Latest closeout

```text
RAW-INSTANCE-METHOD-PARAM-NORMALIZATION-ONCE0-I0-R0

implementation commit                       = 71714556db
instance params normalization calls         = exactly 1
instance param-decls normalization calls    = exactly 1
duplicate capture normalization pair        = 0
normalized-input-only capture terminal      = exactly 1
static capture / route / grammar delta       = 0
focused normalization / constructor tests   = green
depth-three capture / first-failure tests    = green
release build / current pointer / lane guard = green
fallback / retry / View / Ownership          = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

R1 closeout:

```text
selected normal construction sites = exactly 4
NormalCompileRequestV1 constructors = exactly 4
selected-normal Legacy reachability = 0
candidate / finish / publication    = exactly 1
compatibility build_module edge     = exactly 1
fallback / retry / reselection      = 0
new test/check file                 = 0
```

## Evidence

```text
selected normal constructors
  -> NormalCompileRequestV1
  -> NormalDefaultPublishedPipelineV1
  -> ModuleBuilderInvocationSessionV1
  -> ExistingGeneralModuleCompatibilityV1
  -> MirBuilder::build_module(ASTNode)

selected normal constructors:
  execute_mir_mode
  execute_mir_json_minimal
  LLVM source compiler
  Wasm source compiler

explicit compatibility:
  VM keep/fallback, Stage1, REPL, Program JSON v0, selfhost macro-preexpand
explicit reference:
  VM-Hako and the three VM-reference lanes
definition-only:
  execute_mir_interpreter_mode
```

The shared source-hint wrappers are provenance-blind and remain compatibility
surfaces. NarrowV1 lacks normal imports and general module/callable coverage;
only its source-neutral lifecycle kernels are reusable.

R2b closeout:

```text
selected lifecycle caller            = 1
ExistingGeneralModuleCompatibilityV1 = 0
selected-normal build_module edge    = 0
root-level AST clone                 = 1
typed lifecycle failure evidence     = 3/3
normal parity / failure / reuse      = 4/4
explicit compatibility build_module = 2, unchanged
new source file                      = 1, 292 lines
new test/check file                  = 0
all source/check files               < 800
optional quick gate                  = pre-existing EBNF naming-charter failure
clean efe2c467c2 reproduces the same failure
```

R2d closeout:

```text
shared root classifier                 = exhaustive 57/57
Program owner                          = unchanged
selected recursive-safe kinds          = 5
registered residual kinds              = 51
broad self.build_expression fallback   = 0
selected invocation-port descent       = 1
root-specific raw compatibility edge   = 1
selected failure retry                 = 0
focused tests                          = 7/7
new source file                        = 1, 338 lines
new test/check file                    = 0
largest touched source/check file      = 796
```

R2f closeout:

```text
Await recursive closure                 = selected safe / residual unsafe
selected invocation-port descent        = 1
existing Await completion owner          = 1
selected failure retry                   = 0
selected recursive-safe kinds            = 6
registered residual kinds                = 50
focused tests                            = 8/8
new source/test/check/task file           = 0
largest touched source/check file         = 574
```

R2h closeout:

```text
Check recursive closure                  = selected safe / residual unsafe
same-port eager child order              = sealed
existing Check completion owner          = unchanged
selected failure retry                   = 0
selected recursive-safe kinds            = 7
registered residual kinds                = 49
focused Rust tests                        = 13/13
existing Check surface guard              = green
new source/test/check/task file           = 0
largest touched source/check file         = 582
```

R2j closeout:

```text
safe Print compatibility edge           = 0
selected expression kinds               = 7
selected statement-root kinds           = 1
registered residual kinds               = 48
selected / compatibility terminals      = 1 / 1
direct instruction/span parity           = green, unified on/off
focused tests                            = 6/6
fallback / retry / reselection           = 0
production Rust delta                    = +37
new source/test/check/task file           = 0
largest touched source/check file         = 593
largest relevant source/check file        = 774, unchanged
```

R2l closeout:

```text
safe Nowait compatibility edge             = 0
selected expression kinds                 = 7
selected statement-root kinds             = 2
registered residual kinds                 = 47
selected / compatibility terminals        = 1 / 1
MIR/span/Future/binding/slot parity         = green
focused Rust tests                          = 7/7
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +153
new source/test/check/task file             = 0
largest touched source/check file           = 706
largest relevant source/check file          = 774, unchanged
```

R2n closeout:

```text
safe Array compatibility edge              = 0
selected expression kinds                 = 8
selected statement-root kinds             = 2
registered residual kinds                 = 46
selected / compatibility terminals        = 1 / 1
empty/homogeneous/mixed/nested parity       = green
focused Rust tests                          = 8/8
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +206
new source/test/check/task file             = 0
largest touched source/check file           = 745
largest relevant source/check file          = 774, unchanged
```

R2p closeout:

```text
safe Map compatibility edge                = 0
selected expression kinds                 = 9
selected statement-root kinds             = 2
registered residual kinds                 = 45
selected / compatibility terminals        = 1 / 1
duplicate/nested/unified off-on parity      = green
focused Rust tests                          = 9/9
fallback / retry / reselection             = 0
Rust net delta, including focused tests     = +241
new source/test/check/task file             = 0
largest touched source/check file           = 769
largest relevant source/check file          = 774, unchanged
```

## Program-v0 closeout

```text
RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-R0

production / test files              = 317 / 447 lines
existing focused tests               = 4/4 green
selected expression / statement      = 9 / 2 unchanged
registered residual kinds            = 45 unchanged
selected / compatibility terminals   = 1 / 1 unchanged
new test-only Rust file               = 1
production behavior / grammar delta  = 0
shared guard / artifact inventory     = green
fallback / retry / reselection        = 0
```

## Latest closeout

```text
RAW-NONPROGRAM-GROUPED-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0

safe Grouped Assignment compatibility edge = 0
selected expression / statement kinds      = 10 / 2
registered residual kinds                   = 44
selected / compatibility terminals          = 1 / 1
root focused tests                           = 6/6
grouped parity / failure / reuse             = 3/3
existing assignment/raw-port evidence        = green
normal-vs-Legacy non-Program parity          = green
shared guard / artifact inventory            = green
fallback / retry / reselection               = 0
new source/test/check file                    = 0
largest touched source/check file             = 665
```

## Latest closeout

```text
RAW-NONPROGRAM-INDEX-COMPOSITIONAL-DESCENT0-I0-R0

safe Index compatibility edge             = 0
selected expression / statement kinds     = 11 / 2
registered residual kinds                  = 43
selected / compatibility terminals         = 1 / 1
static / generic descent laws               = 0/1 and 1/1 green
StaticDataLoad success-only type evidence   = green
root focused tests                          = 7/7
Array/Map Index full-effect parity          = green
normal-vs-Legacy parity/failure/reuse        = green
shared guard / artifact inventory           = green
fallback / retry / reselection              = 0
new source/test/check file                   = 0
largest touched source/check file            = 675
```

## Latest closeout

```text
RAW-NONPROGRAM-EMPTY-BLOCK-EXPR-COMPOSITIONAL-DESCENT0-I0-R0

safe empty-prelude BlockExpr edge       = 0
selected expression / statement kinds  = 12 / 2
registered residual kinds               = 42
selected / compatibility terminals      = 1 / 1
statement / tail demands                = 0 / 1
nested selected / unsafe-tail partition = green
raw-port MIR/type parity                 = green
normal-vs-Legacy parity/failure/reuse    = green
shared guard / artifact inventory        = green
fallback / retry / reselection           = 0
new source/test/check file               = 0
largest touched source/check file        = 729
```

## Latest closeout

```text
RAW-NONPROGRAM-ANNOTATION-FREE-LOCAL-ROOT-DESCENT0-I0-R0

safe annotation-free Local edge          = 0
selected expression / statement kinds    = 12 / 3
registered residual kinds                 = 41
selected / compatibility terminals        = 1 / 1
typed-array / record special-hook reach    = 0 / 0
root partition tests                       = 8/8 green
existing Local descent/raw/parity suites   = 8/8, 7/7, 6/6 green
standalone lexical-scope diagnostic parity = green
candidate discard / compiler reuse         = green
shared guard / artifact inventory          = green
fallback / retry / reselection             = 0
new source/test/check file                 = 0
largest touched source/check file          = 782
```

## Latest closeout

```text
RAW-NONPROGRAM-ROOT-PARITY-TEST-SEAM1-R0

production Rust delta                = 0
moved fixture body parity            = 6/6 exact
root partition/parity tests          = 8/8 green
normal integration/failure tests     = 8/8 green
selected expr/stmt/residual          = 12 / 3 / 41
selected/compatibility terminals     = 1 / 1
parent/child/shared guard lines       = 482 / 305 / 718
shared guard / artifact inventory    = green
fallback / retry / reselection       = 0
new test-only Rust file              = 1
new check/guard/task file            = 0
all source/check files               < 800
```

## Latest closeout

```text
RAW-NONPROGRAM-BLOCK-EXPR-COMPOSITIONAL-PRELUDE0-I0-R0

safe non-empty BlockExpr compatibility edge = 0
selected prelude responsibilities             = Expr / Print / Nowait / Local
unsafe prelude or tail                         = whole compatibility
existing raw BlockExpr semantic owner          = unchanged
standalone Local lexical-scope failure parity  = green
root partition/parity tests                    = 10/10 green
normal integration/failure tests               = 8/8 green
selected expr/stmt/residual                     = 12 / 3 / 41
selected/compatibility terminals                = 1 / 1
production/parent/parity/integration/guard LOC  = 386/512/374/623/735
shared guard / artifact inventory               = green
fallback / retry / reselection                  = 0
new source/test/check/task file                  = 0
all source/check files                           < 800
```

## Latest closeout

```text
RAW-NONPROGRAM-TASK-SCOPE-COMPOSITIONAL-DESCENT0-I0-R0

safe TaskScope compatibility edge          = 0
empty/non-empty/nested safe partition      = green
safe TaskScope in BlockExpr prelude        = green
existing early-exit / push-body-pop owner  = unchanged
child failure pop-order parity             = green
root partition/parity tests                = 12/12 green
normal integration/failure tests           = 8/8 green
selected expr/stmt/residual                 = 12 / 4 / 40
selected/compatibility terminals           = 1 / 1
production/parent/parity/integration/guard = 413/577/440/646/751 lines
shared guard / artifact inventory          = green
fallback / retry / reselection             = 0
new source/test/check/task file             = 0
all source/check files                      < 800
```

## Closed execution

`NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-I0-R0` — T2, parent
`NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-D0`.

```text
result:
  four selected normal constructors -> one opaque Program admission
  -> one session-owned Program root/catalog kernel
  selected bare-AST/non-Program/generic-root admission = 0
  fallback / retry / reselection = 0

evidence:
  root/catalog lifecycle 2/2; constructor admission 1/1; normal candidate 8/8
  generic Program parity 1/1; raw non-Program partition 12/12
  shared guards, pointer guard, diff check, release build = green

structure:
  module_lifecycle.rs 796 -> 575
  program_root_lowering.rs = 286
  shared guard = 798
  new source/test/check files = 1/0/0
  every touched source/check file < 800
```

## Closed conformance census

`MIRBUILDER-EIGHT-PACK-FINAL-CONFORMANCE0-C0` — T0 conformance census,
replacement credit 0.

```text
verdict:
  selected-normal production chain = Complete
  repository-wide final pipeline   = Residual
  replacement credit               = 0

selected-normal:
  four typed constructors
  -> one Program admission before token/session
  -> one candidate/session
  -> one root/catalog lifecycle
  -> one collector-backed callable batch
  -> one finish/readiness/external commit

ledger reconciliation:
  14 landed production rows backfilled
  SOURCE-NEUTRAL-CALL-RECEIPT = ReuseNeutral closed
  PRELOOP-STAGEB-SPECIAL-ACTIVATION = Delete closed
  test-only seam rows receive replacement credit = 0
```

Eight-pack verdict:

| Pack | Verdict | Exact residual |
| --- | --- | --- |
| `REPLACEMENT-LEDGER0` | Residual | detached Stage-B asset is deleted; active compatibility sunsets remain |
| `DESCENT-SPINE0` | Complete | fixed selected old-edge inventory is physically zero |
| `FUNCTION-STATE0` | Residual | `function_state` / PHI / `variable_map` authority remains distributed |
| `CALL-OBJECT0` | Residual | MethodCall / Call / New / Field / Index and other header-sensitive compatibility surfaces remain |
| `CONTROL0` | Residual | If / Loop / TryCatch / Throw / QMark / Match and related control authority remains |
| `FUNCTION-LIFECYCLE0` | Residual | selected-normal is complete; raw legacy direct function publication remains |
| `MODULE-LIFECYCLE0` | Residual | selected-normal is complete; two production arbitrary-AST `build_module` surfaces remain |
| `COMPILER-RESIDUE0` | Residual | MirCompiler/runtime arbitrary-AST compatibility remains; Stage-B activation is zero |

Repository-wide final-pipeline completion additionally requires a **legacy
JoinModule disposition decision**.  The decision is not a blind `JoinModule`
file-count target: it must classify remaining carrier/boundary use, state
whether each surface is retired or intentionally retained, and prove that no
retained surface is a final planner, acceptance truth, or normal/default
pipeline route. It must keep the CorePlan carrier/boundary ledger separate
from JoinModule execution, observation, JSON/format, runner, and explicit-env
bridge ledgers; each family needs a retire or named non-JoinModule replacement
decision before repository-wide completion is claimed.

## Closed execution

`PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-RET0` — T1 detached-asset
retirement, one atomic commit.

```text
Decision:
  Delete the complete caller-zero Stage-B whole-source selector -> activation
  -> carrier -> function-session -> physical-publication closure.

Atomic delete roots:
  compiler:
    legacy_source_selection
    legacy_static_import_snapshot
    legacy_whole_source_request
    legacy_module_activation/**
  mir:
    preloop_stageb_candidate_shell
    preloop_stageb_carrier/**
  builder:
    preloop_stageb_context_install
    preloop_stageb_function_activation
    calls/preloop_stageb_instance_function_session/**
    calls/preloop_located_argument_*
    calls/preloop_located_outer_completion
    calls/preloop_nested_result_*
    calls/preloop_outer_carrier_*
    all dedicated cfg(test) support for those physical owners
  wiring:
    module declarations/reexports
    Stage-B-only module-lifecycle/readiness/install helpers
    Stage-B-only compilation-context helpers
  proof:
    two Stage-B child guards and their aggregate positive assertions

Keep:
  source_call_target/**
  source_instance_result_contract/**
  callable result/catalog and generic method-call receipts
  unified emitter, recursive child ports, generic function session
  ModuleDraftCollectorV1

Measured law:
  production caller                         = 0 -> 0
  detached production-capable asset family  = 1 -> 0
  replacement cell / credit                 = 0
  replacement owner / fallback              = 0

Ledger:
  PRELOOP-STAGEB-SPECIAL-ACTIVATION
  Delete pending -> Delete closed

Evidence:
  exact special-root repository-zero census
  retained source-neutral receipt tests = green
  normal candidate/session focused tests = 8/8 green
  retained method-call focused tests = green
  cargo check --tests = green
  cargo test --lib attempted; pre-existing baseline failures remain
  (one exact edgecfg failure reproduced at clean pre-RET0 HEAD)
  existing aggregate/replacement/pointer guards
  git diff --check
  all source/check files < 800

Hard stop:
  any non-test caller outside this closure appears
  selected-normal or explicit build_module behavior needs an edit
  a retained source-neutral receipt/catalog semantic must change
  a tombstone, alias, forwarding facade, fallback, or new guard is needed
  JoinIR/runtime Stage-B would be touched
```

## Latest closeout

`PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-I0-R0`

```text
production caller moved                    = 1
typed Program admission/lifecycle          = 1
ProgramV0Compatibility origin/constructor  = 0
Program-v0 raw-binding tombstone            = 0
loader compile_legacy edge                  = 0
direct ProgramV0-to-MIR JSON bridge delta   = 0
source hint / Builder imports               = exact / empty
module, metadata, verification, diagnostics = parity green
failure / compiler reuse                    = green
fallback / retry / reselection              = 0
new source/test/check file                   = 0
largest touched source/check file            = 799
```

## REPL closeout

`REPL-TYPED-PROGRAM-INGRESS0-I0-R0`

```text
production caller moved                  = 1
typed Program constructor/caller         = 1 / 1
REPL compile_legacy edge                 = 0
ReplCompatibility Rust symbols           = 0
source hint / Builder imports             = <repl> / empty
repl/quiet/plugin/ContinueLive config     = parity green
MIR/verification/failure/reuse            = parity green
vm-reference build and REPL execution     = green
VM/session/rewrite/auto-display delta     = 0
fallback / retry / reselection            = 0
direct production build_module edges      = 2, unchanged
new source/test/check/task file            = 0
largest touched source/check file          = 799
```

## Historical TryCatch transaction closeout
```text
Decision: Candidate A
Row: RAW-TRYCATCH-FUNCTION-STATE-TRANSACTION0-I0-R0
Ceremony: T2, one atomic implementation/retirement commit
Pack: FUNCTION-STATE0 + CONTROL0

Caller:
  statement_surface ASTNode::TryCatch

New owners:
  PreparedRawTryCatchV1
    -> DisabledCompatibility(try body only)
    -> Enabled(owned try / catches / finally)
  ActiveRawTryCatchFunctionStateV1
    -> exact seven-field success-only transaction

Exact state:
  return_defer_active / slot / target / emitted
  in_cleanup_block / cleanup_allow_return / cleanup_allow_throw

Contract:
  disable/enabled route sampled pre-effect exactly once
  same child port used exactly once
  first catch only, current try/catch/finally order unchanged
  catch body clone = 0
  success restores exact seven fields
  every typed failure restores 0 and preserves current dirty state
  primary String error unchanged
  CFG / ID / type / binding rollback = 0
  fallback / retry / reselection = 0
  grammar / MIR success / result / publication delta = 0
```

Success-only restoration is intentional. Restoring on failure would be a
separate behavior change; the outer candidate session already owns live-Builder
isolation.

### Atomic delete and sunset

```text
delete:
  statement_surface -> cf_try_catch_with_port_v1(raw fields)
  old terminal definition/export
  lower-side disable-route read
  seven saved_* locals and manual restore assignments
  catch body clone
  caller-zero cf_try_catch / MirBuilder facade / fresh Legacy port facade

sunset:
  id: RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-SUNSET-001
  owner: PreparedDisabledRawTryCatchV1
  surface: NYASH_BUILDER_DISABLE_TRYCATCH=1 -> try body only
  retire_when: env definition/read/documented consumers and fixture are zero
  growth: forbidden
```

### Evidence and hard stops

Use module-local tests plus existing integration/shared guards. New test,
check, task, and per-row guard files are zero.

```text
evidence:
  enabled success restores seeded seven fields and preserves MIR
  try/catch failure leaves current inner defer state and stops later bodies
  finally failure leaves current cleanup state and stops exit
  nested success restores outer state, then caller state
  disabled route lowers try only with no transaction/block/catch/finally
  first catch executes once; later catches execute zero
  failed candidate leaves live Builder unchanged; fresh request succeeds

hard stop:
  error-path restore or Drop/RAII restore
  broad FunctionOwnedStateTransactionV1 or non-seven-field capture
  MIR/CFG/ID/type/binding rollback
  cleanup-policy sampling before finally entry
  changed debug timing, catch semantics, or primary error
  clone/reparse, second port, fallback, retry, or old wrapper
  Return/Throw/QMark/If/Loop or Ownership/View/feature work
  compatibility growth/missing sunset
  any touched source/check file >= 800
```

Closeout evidence:

```text
prepared production caller                    = exactly 1
old cf_try_catch terminals/facades             = 0
manual seven-field snapshot/restore authority  = 0
catch-body clone                               = 0
success exact restore                          = green
try/catch/finally failure-state parity         = green
disabled prepare route                         = green
candidate isolation / fresh reuse              = green
fallback / retry / reselection                 = 0
release build                                  = green
quick gate                                     = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
largest touched source/check file              = 774
```

## Latest static Box closeout

```text
Decision: non-Main static Box state authority
Row: RAW-NONMAIN-STATIC-BOX-COMPILATION-STATE-TRANSACTION0-I0-R0
Ceremony: T2, one atomic implementation/retirement commit
Pack: FUNCTION-STATE0

Named caller:
  raw_expression_dispatch
  -> BoxDeclaration
  -> is_static && name != Main && !root_is_app_mode

New owner:
  ActiveRawStaticBoxCompilationStateV1

Exact state:
  variable_map
  TypeContext snapshot
  current_slot_registry
  BoxCompilationContext option
```

The transaction begins after `register_user_box`, captures/installs the four
states in their current order, and restores them only after every sorted
method succeeds. A method failure consumes a typed rejection without restoring,
preserving the current dirty candidate state and primary String error. The
outer invocation session remains the sole unpublished-candidate discard owner.

### Atomic delete

```text
dispatcher saved_var_map / saved_type_ctx       = 0
dispatcher saved_slot_registry / saved_comp_ctx = 0
dispatcher direct BoxCompilationContext install = 0
dispatcher four manual success restores         = 0
transaction begin / complete / reject            = exactly 1
```

The existing registration point, sorted method iteration, FunctionDeclaration
filter, same child port, per-method lowering, draft behavior, and
restore-before-Void order remain unchanged.

### Evidence and hard stops

Use module-local transaction/route tests plus existing candidate reuse and
static-Box parity evidence. New test/check/task files and per-row guards are
zero.

```text
evidence:
  seeded four-state success restores exactly before Void
  method N failure stops N+1 and leaves the current inner state
  zero methods begin/restore once and emit Void
  nested success restores outer transaction then caller
  Main / App-mode / instance / Program-root static routes do not participate
  late failure publishes no candidate and a fresh compiler request succeeds

hard stop:
  restore-on-error or Drop/RAII restore
  whole FunctionLoweringState / whole MirBuilder capture
  reuse of per-function FunctionOwnedStateTransactionV1
  register_user_box movement or rollback
  method order/filter/port/draft/publication change
  Main, App-mode, instance Box, or Program-root static integration
  Match clone cleanup in the same commit
  fallback, retry, grammar, View/Ownership, or feature work
  new per-row guard/test file or any source/check file >= 800
```

Match owned-input clone retirement remains a fresh-census candidate; it is not
bundled with this state-authority replacement.

Closeout evidence:

```text
named production branch                      = exactly 1
four-state begin / complete / reject          = exactly 1
dispatcher saved_* / direct context authority = 0
success exact restore before Void             = green
method-N failure / later-method stop          = green
failure non-restore / primary String parity   = green
static + instance invocation collection       = green
general-module MIR/result parity              = green
outer candidate discard / compiler reuse      = green
fallback / retry / reselection                = 0
release build                                 = green
quick gate                                    = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
shared spine guard                            = binary-only green
  full mode has pre-existing selected_static_sites drift
largest touched source/check file             = 781
```

## Latest deferred static Box closeout

```text
PROGRAM-ROOT-DEFERRED-STATIC-BOX-LIFECYCLE0-I0-R0
Pack: FUNCTION-LIFECYCLE0
Ceremony: T1
```

```text
direct Program-root context and method-lifecycle authority = 0
new consuming owner production calls                       = exactly 1
sorted demand / success clear / method-N dirty failure      = green
general Program parity / candidate reuse                    = green
fallback / retry / reselection                             = 0
new source/test/check file                                  = 0
largest touched source/check file                           = 784
release build                                               = green
quick gate                                                  = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

The owner preserves the existing success-only lifecycle: method failure leaves
the dirty candidate context, skips later methods, and retains the primary
String. Outer candidate discard remains the sole isolation owner.

## Latest static method-batch closeout

```text
NONMAIN-STATIC-BOX-METHOD-BATCH-SSOT0-I0-R0
Pack: FUNCTION-LIFECYCLE0
Ceremony: T0

prepared batch production issuers                   = exactly 2
caller-local non-Main static method dispatch copies = 0
sorted/non-Function/main-name/symbol/arity contract  = green
Program success-clear / method-N dirty failure       = green
raw four-state success / failure                     = green
general Program parity / candidate reuse             = green
fallback / retry / reselection                       = 0
new source file                                      = 1, 89 lines
new test/check file                                  = 0
largest touched source/check file                    = 776
release build                                        = green
quick gate                                           = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Current design stop

`MIRBUILDER-POST-CALL-NAME-CLASSIFICATION-LIVE-EDGE-CENSUS0-D0`.

```text
Read only:
  recount the production graph after call-name policy cutover
  select at most one behavior-neutral responsibility with a named caller
  require same-commit deletion of its competing old edge

Do not:
  infer the next row from deferred Main-helper or record-helper candidates
  add caller-zero routes, compatibility growth, View, Ownership, or features
```

## Latest call-name policy closeout

```text
CALL-NAME-CLASSIFICATION-SSOT0-I0-R0 / T1 / CALL-OBJECT0

decision owner / production consumers        = 1 / 3
Raw admission / Callee class facts           = independent, one total match
old predicate definitions / call edges       = 0 / 0
resolution priority and cross-surface matrix = exact
stable guard transports / Hako parity        = green
fallback / retry / compatibility growth      = 0
route/MIR/result/View/Ownership delta         = 0
new source / test / check files               = 1 / 0 / 0
new policy / largest touched source-check     = 98 / 460 lines
largest relevant source-check                 = 799, unchanged
release build                                 = green
quick gate                                    = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Latest call policy closeout

```text
CALL-BOX-KIND-POLICY-SSOT0-I0-R0
Pack: CALL-OBJECT0
Ceremony: T1

decision owner                              = 1
production consumers                       = 6
resolver-extended / general contexts        = 2 / 4
old classifier definitions and calls        = 0
analyzer compatibility growth               = forbidden
policy/resolver/call-route/parity/reuse      = green
fallback / retry / reselection               = 0
route/MIR/result/View/Ownership delta         = 0
new source file                              = 1, 114 lines
new test/check file                          = 0
largest source/check file                    = 799
release build                                = green
quick gate                                   = unrelated pre-existing
  docs/reference/language/EBNF.md naming-token failure
```

## Latest closeout

`INSTANCE-BOX-DECLARATION-LIFECYCLE-SSOT0-I0-R0` / T1 /
`FUNCTION-LIFECYCLE0` is closed.

```text
authority:
  one PreparedInstanceBoxDeclarationLifecycleV1
  one effectful common prefix
  distinct consuming root/raw method terminals

production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

deleted from both callers:
  register_user_box_declared_fields
  build_box_declaration
  constructor-batch issue/lower
  instance-method-batch issue/lower

preserved:
  field -> metadata -> every constructor -> every method
  metadata/constructor/method first-error dirty prefix
  root exact catalog key and missing-row diagnostic
  raw lookup-free method demand and trailing Void placement
  compatibility owner/sunset = 0
  fallback/retry/reselection = 0
  grammar/result/publication/View/Ownership delta = 0

evidence:
  lifecycle capture tests 14/14
  depth-three constructor/method capture, general Program parity, reuse = green
  route inventory/root/binary guards and release build = green
  quick gate = unrelated pre-existing EBNF compatibility-alias failure

structure:
  new source file = 1, 98 lines
  new test/check file = 0
  largest source/check file = 799
```

`INSTANCE-BOX-METHOD-BATCH-SSOT0-I0-R0` / T1 /
`FUNCTION-LIFECYCLE0` is closed.

```text
authority:
  PreparedInstanceBoxMethodBatchV1 prepares each lexically sorted
  non-static instance FunctionDeclaration once

production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

durable terminals:
  root -> exact catalog key -> lower_root_instance_method
  raw  -> no catalog lookup -> lower_instance_box_method

deleted:
  both caller-local sorted_method_entries loops
  duplicated filtering, symbol construction, payload cloning, dispatch

preserved:
  build_box_declaration -> constructor batch -> method batch
  lexical order, static/non-Function skip, first-error prefix
  root missing-key diagnostic and raw lookup-free behavior
  grammar/result/publication/View/Ownership delta = 0
  fallback/retry/reselection = 0

evidence:
  exact canonical namespace/symbol handoff, skip matrix, prefix failure
  nested constructor/method order, general Program parity, compiler reuse
  route inventory/root/binary guards and release build = green
  quick gate = unrelated pre-existing EBNF compatibility-alias failure

structure:
  new source file = 1
  new test/check file = 0
  largest source/check file = 799
```

`NORMAL-DEFAULT-VERIFIED-MAIN-LOWERING-HANDOFF0-I0-R0` / T1 /
`MODULE-LIFECYCLE0` is closed.

```text
authority:
  selected Program App terminal consumes VerifiedMainExpansionV1 directly
  for exact Main root source, sorted static children, and callable-main symbol

deleted from selected Program:
  raw Main method-map accumulator and methods.clone()
  second App/Main re-selection and impossible Script fallback
  build_static_main_box_with_port_v1 compatibility-facade edge
  helper re-sort/filter/symbol re-projection

preserved:
  RootExpansion validation precedence
  helper order, first failure, stop-before-later-helper/Main body
  callable-main Omitted/Required policy and Main body exactly once
  explicit raw Main compatibility via one shared body/state kernel
  Main args/state restoration, general MIR/result/publication behavior
  fallback / retry / reselection = 0

evidence:
  verified helper order and helper-N failure stop        = green
  Required selected/compat helper+Main order parity      = green
  general Program MIR/result parity                      = green
  late failure candidate isolation/compiler reuse        = green
  shared root guard / binary-only lane guard              = green
  release build                                           = green
  quick gate                                              = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source/test/check file                              = 0
  largest touched source/check file                       = 799
```

## Previous closeout

`NORMAL-DEFAULT-ROOT-EXPANSION-ROUTE-HANDOFF0-I0-R0` / T1 /
`MODULE-LIFECYCLE0` is closed.

```text
authority:
  VerifiedRawRootExpansionV1 is issued once before prepare_module and borrowed
  through the selected normal Program kernel; is_app_mode is consumed once.

deleted:
  declaration_indexer::has_main_static definition and sole caller
  root_is_app_mode.unwrap_or_else ambient fallback
  second source AST Script/App classification

preserved:
  invalid/duplicate Main RootExpansion precedence
  CatalogSeal/CatalogInstall/RootLower/Finalize ordering
  root_is_app_mode publication for the existing raw observer
  Main/helper/body, catalog/index/static-data, collector/publication behavior
  explicit compatibility, fallback/retry/reselection = 0

evidence:
  Script/App disposition handoff fixture             = green
  invalid Main precedence                            = green
  general Program MIR/result parity                  = green
  late failure candidate isolation/compiler reuse    = green
  shared root guard / binary-only lane guard          = green
  release build                                      = green
  quick gate                                         = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source/test/check file                         = 0
  largest touched source/check file                  = 792
```

## Latest closeout

`INSTANCE-BOX-CONSTRUCTOR-BATCH-SSOT0-I0-R0` / T0 /
`FUNCTION-LIFECYCLE0` is closed.

```text
new owner:
  PreparedInstanceBoxConstructorBatchV1

named production issuers:
  Program-root instance Box = 1
  raw instance Box          = 1

deleted:
  both caller-local constructor sort/projection/symbol/clone/dispatch loops
  sorted_constructor_entries helper and its caller-zero test

preserved:
  field registration and build_box_declaration ordering
  ordinary instance-method routes
  Main/static behavior
  stop-on-constructor-N failure and partial candidate state
  grammar/result/publication behavior
  fallback/retry/reselection = 0

evidence:
  lexical order/non-Function skip/first-failure stop = green
  nested constructor and depth-three capture paths   = green
  general Program parity and compiler reuse          = green
  focused shared guard and binary-only lane guard    = green
  full legacy module-draft/headerport guard           = pre-existing stale
    port_aware_function_draft.rs path failure before selected assertions
  release build                                      = green
  quick gate                                         = unrelated pre-existing
    docs/reference/language/EBNF.md naming-token failure
  new source file                                    = 1, 91 lines
  new test/check file                                = 0
  largest touched source/check file                  = 799
```

Lambda capture collector SSOT is a pre-designed candidate only. Main helper
batching and Match hygiene also remain unselected. Feature additions remain
parked until a fresh live-edge census selects one bounded replacement.
Breaking series selected by `MIRBUILDER-PUBLIC-ROOT-API0-D0`:
```text
1 MIRBUILDER-ROOT-TEST-EVIDENCE0-R0 closed (direct callers 15 -> 5)
2 HOST-PROVIDER-CFGTEST-AST-JSON-COMPAT0-RET0 closed (5 -> 4)
3 MIRBUILDER-RAW-OWNER-TEST-EVIDENCE0-R0 closed (4 -> 1)
4 MIRBUILDER-MINIMAL-LIFECYCLE-SMOKE0-R0 closed (1 -> 0)
5 MIRBUILDER-PUBLIC-ROOT-API0-RET0 closed (definition/wrappers -> 0)
```
External consumers are unknown; migration is `MirCompiler::compile*` for Program.

Compatibility sunset:
```text
sunset_id: RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001
state: closed
owner / residual surface / root-specific raw edge / execution callers: 0
retired by: RAW-NONPROGRAM-ROOT-COMPAT-RET0-R0

sunset_id: STAGE1-DIRECT-POST-MACRO-NONPROGRAM-COMPAT-SUNSET-001
state: closed
owner and NonProgram Legacy edge: 0
retired by: STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0
```

Closed sunset:

```text
NORMAL-DEFAULT-GENERAL-MODULE-COMPAT-SUNSET-001
  owner ExistingGeneralModuleCompatibilityV1 = 0
  selected-normal build_module surface       = 0
  global build_module definition/callers     = non-claim
```

Compatibility sunsets:

```text
CALL-BOX-KIND-ANALYZER-COMPAT-SUNSET-001
  state: active
  owner: CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler
  surface: BreakFinderBox / PhiInjectorBox / LoopSSA
  growth: forbidden
  retire_when: analyzer production routes are zero, or one-profile
    classification parity is proven and all callers migrate atomically

RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-SUNSET-001
  state: closed
  owner: deleted
  definition/read/docs/fixture/route shell = 0
  retired by: RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-RETIRE0-I0-R0

RAW-THROW-DEBUG-TRACE-COMPAT-SUNSET-001
  state: closed
  owner: deleted
  definition/read/docs/fixture/route shell = 0
  retired by: RAW-THROW-DEBUG-TRACE-COMPAT-RETIRE0-I0-R0

MIRCOMPILER-ARBITRARY-AST-COMPAT-SUNSET-001
  state: closed
  production build_module edge: 0
  retired by: MIRCOMPILER-PUBLIC-PROGRAM-ADMISSION0-I0-R0

RUNTIME-MIRBUILDER-AST-JSON-COMPAT-SUNSET-001
  state: closed
  measured build_module edge: src/runtime/mirbuilder_emit.rs = 0
  env.mirbuilder.emit contract: Program(JSON v0) only
  AST JSON rejection: before Builder, no retry
```

## Queue

```text
R0  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-D0 closed
R1  NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0 closed
R2a NORMAL-DEFAULT-ROOT-CATALOG-PREFLIGHT0-D0 closed
R2b NORMAL-DEFAULT-ROOT-CATALOG-LIFECYCLE0-I0-R0 closed
R2c NORMAL-DEFAULT-NONPROGRAM-ROOT-DESCENT0-D0 closed
R2d RAW-NONPROGRAM-PORT-NEUTRAL-EXPR-DESCENT0-I0-R0 closed
R2e RAW-NONPROGRAM-NEXT-COMPOSITIONAL-EXPR0-D0 closed
R2f RAW-NONPROGRAM-AWAIT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2g RAW-NONPROGRAM-NEXT-COMPOSITIONAL-EXPR1-D0 closed
R2h RAW-NONPROGRAM-CHECK-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2i RAW-NONPROGRAM-NEXT-RESPONSIBILITY0-D0 closed
R2j RAW-NONPROGRAM-PRINT-ROOT-DESCENT0-I0-R0 closed
R2k RAW-NONPROGRAM-NEXT-RESPONSIBILITY1-D0 closed
R2l RAW-NONPROGRAM-NOWAIT-ROOT-DESCENT0-I0-R0 closed
R2m RAW-NONPROGRAM-NEXT-RESPONSIBILITY2-D0 closed
R2n RAW-NONPROGRAM-ARRAY-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2o RAW-NONPROGRAM-NEXT-RESPONSIBILITY3-D0 closed
R2p RAW-NONPROGRAM-MAP-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2q RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-D0 closed
R2r RAW-NONPROGRAM-ROOT-PARTITION-TEST-SEAM0-R0 closed
R2s RAW-NONPROGRAM-NEXT-RESPONSIBILITY4-D0 closed
R2t RAW-NONPROGRAM-GROUPED-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2u RAW-NONPROGRAM-NEXT-RESPONSIBILITY5-D0 closed
R2v RAW-NONPROGRAM-INDEX-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2w RAW-NONPROGRAM-NEXT-RESPONSIBILITY6-D0 closed
R2x RAW-NONPROGRAM-EMPTY-BLOCK-EXPR-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2y RAW-NONPROGRAM-NEXT-RESPONSIBILITY7-D0 closed
R2z RAW-NONPROGRAM-ANNOTATION-FREE-LOCAL-ROOT-DESCENT0-I0-R0 closed
R2aa RAW-NONPROGRAM-NEXT-RESPONSIBILITY8-D0 closed
R2ab RAW-NONPROGRAM-ROOT-PARITY-TEST-SEAM1-R0 closed
R2ac RAW-NONPROGRAM-NEXT-RESPONSIBILITY9-D0 closed
R2ad RAW-NONPROGRAM-BLOCK-EXPR-COMPOSITIONAL-PRELUDE0-I0-R0 closed
R2ae RAW-NONPROGRAM-NEXT-RESPONSIBILITY10-D0 closed
R2af RAW-NONPROGRAM-TASK-SCOPE-COMPOSITIONAL-DESCENT0-I0-R0 closed
R2ag RAW-NONPROGRAM-NEXT-RESPONSIBILITY11-D0 closed
R2ah NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-D0 closed
R2ai NORMAL-DEFAULT-PROGRAM-ROOT-ADMISSION0-I0-R0 closed
R2aj RAW-SCRIPT-LEXICAL-BINDING0-I0-R0 closed at 7bf6c9b996
R2ak MIRBUILDER-LIVE-EDGE-CENSUS43-D0 closed
R2al RAW-SCRIPT-UNARY-LEXICAL-CLOSURE0-D0 accepted design stop
R2am RAW-SCRIPT-UNARY-LEXICAL-CLOSURE0-I0-R0 closed at 1adb617542
R2an MIRBUILDER-LIVE-EDGE-CENSUS44-D0 closed
R2ao RAW-SCRIPT-PRINT-LEXICAL-CLOSURE0-D0 accepted design stop
R2ap RAW-SCRIPT-PRINT-LEXICAL-CLOSURE0-I0-R0 closed at c1c7852b76
R2aq MIRBUILDER-LIVE-EDGE-CENSUS45-D0 closed
R2ar RAW-SCRIPT-BINARY-LEXICAL-CLOSURE0-D0 accepted design stop
R2as RAW-SCRIPT-BINARY-LEXICAL-CLOSURE0-I0-R0 closed at b562263854
R2at MIRBUILDER-LIVE-EDGE-CENSUS46-D0 closed
R2au RAW-SCRIPT-AWAIT-LEXICAL-CLOSURE0-D0 accepted design stop
R2av RAW-SCRIPT-AWAIT-LEXICAL-CLOSURE0-I0-R0 closed at 074e944fec
R2aw MIRBUILDER-LIVE-EDGE-CENSUS47-D0 closed
R2ax RAW-SCRIPT-CHECK-LEXICAL-CLOSURE0-D0 accepted design stop
R2ay RAW-SCRIPT-CHECK-LEXICAL-CLOSURE0-I0-R0 closed at a78d4e968a
R2az MIRBUILDER-LIVE-EDGE-CENSUS48-D0 closed
R2ba RAW-SCRIPT-ANDOR-LEXICAL-CLOSURE0-D0 accepted design stop
R2bb RAW-SCRIPT-ANDOR-LEXICAL-CLOSURE0-I0-R0 closed at 1f17bc93d1
R2bc MIRBUILDER-LIVE-EDGE-CENSUS49-D0 closed: NoSafeSlice
R2bd RAW-SCRIPT-SEMANTIC-CLOSURE-BOUNDARY1-D0 closed: Candidate A-prime
R2be RAW-SCRIPT-STATIC-CONST-SEMANTIC-CLOSURE0-I0-R0 closed
R2bf MIRBUILDER-LIVE-EDGE-CENSUS50-D0 closed: selected existing diagnostic family
R2bg RAW-SCRIPT-SELECTED-UNSUPPORTED-SEMANTIC-CLOSURE0-I0-R0 closed
R2bh MIRBUILDER-LIVE-EDGE-CENSUS51-D0 current bounded batch census
R3  MIRBUILDER-EIGHT-PACK-FINAL-CONFORMANCE0-C0 closed: Residual
R4  PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-D0 closed
R5  PRELOOP-STAGEB-SPECIAL-ACTIVATION-RETIRE0-RET0 closed
R6  PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-D0 closed
R7  PROGRAM-JSON-V0-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R8  REPL-TYPED-PROGRAM-INGRESS0-D0 closed
R9  REPL-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R10 POST-MACRO-PROGRAM-ADMISSION0-D0 closed
R11 STAGE1-DIRECT-POST-MACRO-PROGRAM-INGRESS0-I0-R0 closed
R12 MIR-INTERPRETER-POST-MACRO-PROGRAM-INGRESS0-D0 closed: NoProductionCaller
R13 MIR-INTERPRETER-DETACHED-ASSET-RETIRE0-RET0 closed
R14 BENCH-DETACHED-ASSET-RETIRE0-RET0 closed
R15 INTERPRETER-LEGACY-FEATURE-CLOSURE0-D0 closed: Retire
R16 INTERPRETER-LEGACY-FEATURE-RETIRE0-RET0 closed
R17 MIR-CONTROL-FLOW-DETACHED-HELPERS0-RET0 closed
R18 RAW-NONPROGRAM-VARIABLE-ASSIGNMENT-COMPOSITIONAL-DESCENT0-D0 closed
R19 RAW-NONPROGRAM-VARIABLE-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R20 RAW-NONPROGRAM-VARIABLE-COMPOUND-ASSIGNMENT-COMPOSITIONAL-DESCENT0-D0 closed
R21 RAW-NONPROGRAM-VARIABLE-COMPOUND-ASSIGNMENT-COMPOSITIONAL-DESCENT0-I0-R0 closed
R22 RAW-NONPROGRAM-SAFE-RETURN-ROOT-DESCENT0-D0 closed: Accept
R23 RAW-NONPROGRAM-SAFE-RETURN-ROOT-DESCENT0-I0-R0 closed
R24 RAW-NONPROGRAM-PLAIN-SCOPEBOX-COMPOSITIONAL-DESCENT0-D0 closed: Accept
R25 RAW-NONPROGRAM-PLAIN-SCOPEBOX-COMPOSITIONAL-DESCENT0-I0-R0 closed
R26 RAW-NONPROGRAM-SAFE-THROW-ROOT-DESCENT0-D0 closed: NoProductionConstructor
R27 RAW-NONPROGRAM-ROOT-INGRESS-POLICY0-D0 closed: IndependentSunsets
R28 RUNTIME-MIRBUILDER-AST-JSON-COMPAT-RETIRE0-I0-R0 closed
R29 POST-MACRO-ROOT-CONTRACT0-D0 closed: WholeFileProgram
R30 STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0 closed
R31 SELFHOST-MACRO-PREEXPAND-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R32 VM-HAKO-POST-MACRO-TYPED-PROGRAM-INGRESS0-I0-R0 closed
R33 VM fallback closed; R34 VM keep closed; R35 helper DEL0 closed
R36 public contract D0 closed; R37 public Program admission closed
R38 public root D0 closed; R39 root test evidence closed
R40 host cfgtest AST JSON closed
R41 RAW-TRYCATCH-FUNCTION-STATE-TRANSACTION0-I0-R0 closed
R42 MIRBUILDER-POST-TRYCATCH-LIVE-EDGE-CENSUS0-D0 closed: static Box selected
R43 RAW-NONMAIN-STATIC-BOX-COMPILATION-STATE-TRANSACTION0-I0-R0 closed
R44 MIRBUILDER-POST-STATIC-BOX-LIVE-EDGE-CENSUS0-D0 closed: deferred Program static Box selected
R45 PROGRAM-ROOT-DEFERRED-STATIC-BOX-LIFECYCLE0-I0-R0 closed
R46 MIRBUILDER-POST-DEFERRED-STATIC-BOX-LIVE-EDGE-CENSUS0-D0 closed: method-batch SSOT selected
R47 NONMAIN-STATIC-BOX-METHOD-BATCH-SSOT0-I0-R0 closed
R48 MIRBUILDER-POST-STATIC-METHOD-BATCH-LIVE-EDGE-CENSUS0-D0 closed: constructor batch selected
R49 INSTANCE-BOX-CONSTRUCTOR-BATCH-SSOT0-I0-R0 closed
R50 MIRBUILDER-POST-CONSTRUCTOR-BATCH-LIVE-EDGE-CENSUS0-D0 closed: root expansion handoff selected
R51 NORMAL-DEFAULT-ROOT-EXPANSION-ROUTE-HANDOFF0-I0-R0 closed
R52 MIRBUILDER-POST-ROOT-EXPANSION-HANDOFF-LIVE-EDGE-CENSUS0-D0 closed: verified Main handoff selected
R53 NORMAL-DEFAULT-VERIFIED-MAIN-LOWERING-HANDOFF0-I0-R0 closed
R54 MIRBUILDER-POST-VERIFIED-MAIN-HANDOFF-LIVE-EDGE-CENSUS0-D0 closed
R55 INSTANCE-BOX-METHOD-BATCH-SSOT0-I0-R0 closed
R56 MIRBUILDER-POST-INSTANCE-METHOD-BATCH-LIVE-EDGE-CENSUS0-D0 closed
R57 INSTANCE-BOX-DECLARATION-LIFECYCLE-SSOT0-I0-R0 closed
R58 MIRBUILDER-POST-INSTANCE-BOX-LIFECYCLE-LIVE-EDGE-CENSUS0-D0 closed
R59 CALL-BOX-KIND-POLICY-SSOT0-I0-R0 closed
R60 MIRBUILDER-POST-CALL-BOX-KIND-POLICY-LIVE-EDGE-CENSUS0-D0 closed
R61 CALL-NAME-CLASSIFICATION-SSOT0-I0-R0 closed
R62 MIRBUILDER-POST-CALL-NAME-CLASSIFICATION-LIVE-EDGE-CENSUS0-D0 closed: Match owned-input selected
R63 RAW-MATCH-OWNED-INPUT-SINGLE-USE0-I0-R0 closed
R64 MIRBUILDER-POST-MATCH-OWNED-INPUT-LIVE-EDGE-CENSUS0-D0 closed: record-helper body selected
R65 RECORD-HELPER-BODY-INVOCATION0-I0-R0 closed
R66 MIRBUILDER-POST-RECORD-HELPER-BODY-LIVE-EDGE-CENSUS0-D0 closed: instance normalization selected
R81 RAW-NONMAIN-STATIC-BOX-LIFECYCLE-HANDOFF0-I0-R0 closed: raw dispatcher lifecycle deleted
R82 post-raw-static-Box census closed: no bounded edge; Lambda design selected
R83 RAW-LAMBDA-CAPTURE-OBSERVATION0-D0 closed: NoSafeI0
R84 RAW-LAMBDA-LEXICAL-BOUNDARY-MATRIX0-D0 closed
R85 RAW-LAMBDA-LEXICAL-CAPTURE-LIFECYCLE0-I0-R0 closed: old authority deleted
R86 post-Lambda census closed: its NoSafeSlice verdict was corrected by the
    later multi-owner task census
R87 RAW-TRYCATCH-DISABLE-ROUTE-COMPAT-RETIRE0-I0-R0 closed: disable route and
    sunset retired; fresh live-edge census is current
R88 RAW-THROW-DEBUG-TRACE-COMPAT-RETIRE0-I0-R0 closed: debug-trace route and
    sunset retired; fresh live-edge census is current
R89 MIRBUILDER-LIVE-EDGE-CENSUS0 closed: no safe immediate I0/R0; Program
    declaration facts selected for T2 D0, while JoinModule remains final-C0
    family-disposition work
R90 NORMAL-DEFAULT-PROGRAM-DECLARATION-FACTS0-D0 closed: total source-ordered
    facts product accepted; atomic indexer replacement is next
R91 NORMAL-DEFAULT-PROGRAM-DECLARATION-FACTS0-I0-R0 closed: source-only facts
    product replaces the selected raw indexer edge; fresh live-edge census is current
R92 MIRBUILDER-LIVE-EDGE-CENSUS0 closed: raw non-Program is NoSafeI0, immediate
    compatibility retirement has no safe I0, and Program-root work partition is the
    sole selected T2 design stop; JoinModule remains final-C0 disposition work
R93 PROGRAM-ROOT-WORK-PARTITION0-D0 closed: one total source-only partition of the
    mixed Program-root coordinator is accepted; atomic I0/R0 is next
R94 PROGRAM-ROOT-WORK-PARTITION0-I0-R0 closed: one source-only work plan replaces the
    mixed coordinator while preserving source order, runtime retention, and terminals;
    fresh live-edge census is current
R95 RAW-LEGACY-EXPRESSION-FACADE-RETIRE0-I0-R0 closed: caller-zero raw expression
    facade and expression input view are deleted; fresh live-edge census is current
R96 CALL-GLOBAL-PRESENCE-LEGACY-FACADE-RETIRE0-RET0 closed: caller-zero direct
    module-presence facade is deleted; authority-aware resolver is sole entry
R97 MIRBUILDER-LIVE-EDGE-CENSUS0 closed: no safe live T0 replacement remains;
    normal collector drain selected as the sole T2 design stop

after every bounded retirement:
  fresh-census then select one named production edge or detached Delete asset

after final-pipeline Complete only:
F0  refresh missing-feature / Ownership / View readiness inventory
F1  resume the existing Ownership taskboard from its read-only readiness gate
F2  Unique Box / ScopedAlias -> callable ABI -> Anchored View
F3  select one later unimplemented feature from the language status index
```

The old M2c-to-M8 complete-program queue is superseded. Passive assets are
reconsidered only when the selected live edge names an exact consumer.
Source-level Ownership/View and other new language semantics do not enter the
MirBuilder replacement train. Analysis-only views used to observe existing
control flow are not source-language View activation.

R23 removed the root-only safe Return compatibility edge without body widening.
## Closed tail

```text
MODULE-SOURCE0-S0 / e6baf9b4
  exact Main0 + plain instance Boxes + callable catalog co-seal

INSTANCE-INTEGER-RETURN0-S0 / 34ea62cfea
  every instance method -> exact integer-literal Return plan

MAIN0-BRIDGE0-S0 / 7aed7848e6
  retained instance owner + existing Main0 semantic receipts

INSTANCE-CUMULATIVE0-S0 / 7e3144da62
  one source-owning cumulative set; exact ordered key coverage

INSTANCE-I64-PARAMETER-RETURN0-S0 / bdd0812c26
  total two-family classifier; exact Receiver + Parameter(0) + Return use
  evidence 76/76; production +464, test +62, check +36; max file 791

INSTANCE-INTEGER-LOCAL-RETURN0-S0 / adbb737f8a
  third cumulative variant; exact Receiver + Local(0) + Integer initializer
  + terminal Local read; evidence 74/74; production +391, test +62, check +8;
  one new source file, no new test/check file, max source/check 799

NORMAL-SOURCE-PLAN0-PROOF-COMPACTION / 8859caecba
  behavior/grammar delta 0; tests 701 lines, callable guard 755 lines
```

Detailed landed diffs and older cell measurements belong to git history and
the linked task map. They are not copied into this rolling card.

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

新しい発見はこの8 packのいずれかへ入れる。新packは増やさない。

## Parked

```text
source-level Ownership/View and unimplemented language features until the
repository-wide final pipeline is Complete
.hako selfhost MirBuilder/parser migration
unselected cleanliness work
new language semantics
default Raw/Canonical cutover before M7
```

新しいper-row shell guardは作らない。通常gateと詳しいassertionはactive
source/testおよび既存shared guardが所有する。
