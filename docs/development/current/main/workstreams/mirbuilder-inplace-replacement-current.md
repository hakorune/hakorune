---
Status: Design stop — MIR-PHYSICAL-TYPE-INPUT-D0
Date: 2026-08-25
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

Current capsule:

```text
  current decision  = MIR-PHYSICAL-TYPE-INPUT-D0
  implementation    = JoinIR remapper isolation I0 landed at 0048c0176a
  mode              = design_stop
  production stop   = active lifecycle inventory is the sole production collector; old merge/remap is reference-only
  exit              = accept D0 authority/matrix; then one exact-i64 implementation cell
  fallback / retry  = 0
```

The post-root task-order audit is closed; historical queues cannot select a row; only the six-line brief and ordered frontier below may do so.

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

The completed Dynamic AOT activation is historical evidence only. Do not
restore its task order or any other closed chronology here.

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

Normal-root C0/I0 is closed; `NORMAL-ROOT-WORK-PLAN-MODE-AUTHORITY-CUTOVER-I0`
landed as `c152f9f883`. Public whole-file AST Compatibility remains
`ParkedSealed`. The Call retirement design remains the long-term target, while
the active Decision is now `MIR-PHYSICAL-TYPE-INPUT-D1-P0`.

```text
canonical core
  Call { dst, callee: Callee, args, effects }

JSON-v0 compatibility ingress
  owner-private JsonV0CallInput -> resolve exactly once -> Callee -> Call

forbidden
  core LegacyCall; Option<Callee>; func field; sentinel/default Callee;
  optimizer target inference; backend by-name fallback or retry
```

`JsonV0CallInput` is evidence, not target authority, and never crosses into
canonical MIR. Typed producers and each compatibility owner resolve an exact
`Callee`; the thin `MirInstruction::call(dst, callee, args, effects)` physical
constructor classifies nothing. `func`, `ValueId::INVALID`, target-name Const,
printer/JSON text, optimizer scans, and runtime/backend lookup are non-authority.

The finite D0 boundary covers canonical Call production through all non-test
constructors/reconstructors and interpreter/printer/JSON/backend terminals.
HEAD has 20 direct `MirInstruction::Call` literals plus the canonical helper (`MirInstruction::call`): 21 writer definitions; helper callers are grouped by owner
family and never double-counted. The selected boundary has zero
missing-callee publication edges: historical Program JSON-v0 literals were removed
before HEAD and MIR JSON-v0 resolves/rejects before publication. Tests/comments and
unrelated MIR are excluded; typed stale `func` and `Method(None)` remain blockers.

JSON-v0 input outcomes are total: valid explicit callee wins and legacy fields
are decoration; malformed explicit callee rejects without fallback; legacy
name or unique exact function-local `func -> Const(String)` may resolve once;
undefined, non-String, duplicate, foreign, or missing target rejects before
Call publication. No reject may retry another target source.

Before core field deletion, one `Callee` operand projection must own this law:
Method.receiver, Value(value), Closure.captures and Closure.me_capture are
ValueId operands; Global/Extern/Constructor add none; ordered Call args are
always operands. Escape policy may consume that projection but remains a
separate semantic policy.

## Closed bounded cell: MIR-CALL-JSONV0-QUALIFIED-PRODUCER-I0-R0

The two exact `static_methods` producers now use the thin canonical constructor
with `Callee::Global(qualified)`. Their target Const, `fun_val`, and
missing-callee edges are zero; instance `me_v`, argument order, dst, and READ
effect are unchanged. Program JSON-v0 literal issuers are 3 -> 1 and the
dynamic MIR JSON-v0 edge remains open.

Evidence: focused static/instance tests 2/2, unchanged late-resolver tests 7/7,
Call corridor and pointer guards green, touched-file rustfmt green, source 446,
constructor owner 385, guard 105. The 433 warnings remain the parked baseline
(422 dead_code + 11 private_interfaces); no new warning class was introduced.
The shared MirBuilder guard retains its known parent Loop-owner red and is not
an R1 waiver or failure.

## Accepted design: MIR-CALL-JSONV0-LEGACY-TARGET-CATALOG-D1

Decision: direct MIR-v0 compatibility is an owner-private input boundary.
Source authority + issuer: raw function draft -> immutable function-local
`ValueId -> one direct Const(String)` catalog -> exact `Callee::Global` ->
`MirInstruction::call`. No module membership, arity synthesis, Extern
classification, optimizer scan, or backend lookup is allowed.
State/failure: explicit callee wins decoration; malformed explicit, malformed
name, name+func conflict, missing/duplicate/non-String/foreign/INVALID func all
typed-reject; direct name/func text is otherwise preserved exactly.
Fail-fast: catalog and target resolve before `call`/`mir_call` block publication;
no `Call(callee=None)` and no retry. Nested `mir_call` owns target/args/effects;
outer `dst` remains the destination override.
Compatibility boundary: direct MIR-v0 plus selected Rust VM/JSON/LLVM remain in
this lane; Program-v0 is R3. PyVM (`daily_route=0`, diagnostic-only),
reference-vm, and Python/llvmlite are boundary-outside `ParkedSealed` owners.
Census boundary: MIR-v0 raw function draft -> `call`/`mir_call` builder ->
block publication; `boxcall`, `externcall`, historical terminals excluded.
Topology-I0 closed: root 165, child 599, call 198; facade/API, block publication,
ownership witness, and post-loop canonicalizer order are unchanged. MIR-v0 tests
13/13, rustfmt/diff/pointer/Call guards are green; 433 warnings remain baseline.
R2-I0 closed: owner-private input state and function-local direct Const(String)
catalog now resolve before publication; MIR-v0 tests 26/26 and the shared guard
are green. The broader emit census remains red only at the recorded cohort-missing
baseline; it is outside this Call ingress slice.

## Closed bounded cell: MIR-CALL-JSONV0-PROGRAM-CATALOG-I0

`ProgramCallTargetCatalog` is built once from local defs before main/defs lowering.
Generic `ExprV0::Call` resolves target before argument lowering and emits only the
thin canonical constructor; target Const and Program `callee=None` issuance are 0.
Unique short-name/arity, qualified, Extern, unknown Global, ambiguity, duplicate,
and empty-name cases are covered by 9/9 focused tests; the bridge suite is 22/22.
The shared Call corridor, pointer, diff, and rustfmt guards are green; 433 warnings
remain the known baseline. Late `func_map`/`maybe_resolve_calls` and Program-site
canonicalizer retirement remain R3b, not an R3a claim.

## Closed bounded cell: MIR-CALL-JSONV0-PROGRAM-LATE-RETIRE-I0

Program `func_map`/`maybe_resolve_calls` are retired; the Program bridge refuses
legacy Const-to-target issuance while other sites retain their policy. Evidence:
callsite 14/14, bridge 23/23, corridor/pointer/diff/rustfmt green; program owner
121 lines, 433 warnings baseline. R4 operand/remap/ownership/escape SSOT follows.

## Ordered MIR Call retirement series

1. R1: qualified Program JSON-v0 producers (closed).
2. D1: direct catalog, policy, state matrix, and compatibility boundary
   accepted (closed).
3. Topology-I0: move the 755-line loader loop behind a thin facade; behavior
   and existing canonicalizer order stay unchanged (closed).
4. R2-I0: `call.rs` parses total state, resolves once, and publishes no
   `Call(callee=None)` or partial block (closed).
5. R3a: pre-core Program JSON-v0 catalog/resolution (closed); R3b deletes target
   Const authority, `maybe_resolve_calls`, and all remaining Program late edges
   (closed).
6. R4a: exhaustive `Callee` operand rewrite projection into SimplifyCFG
   Call-use rewrite (closed); R4b `used_values` immutable projection and
   methods.rs delegation (closed); R4c `value_consumer` Call membership,
   R4d escape, R4e ownership, R4f CallLike, and Query T0 are closed; JoinIR
   remap census is closed: lifecycle collection remains active, while disconnected merge remap APIs have no non-test entry and are queued for isolation.
7. R5 D0 is accepted; R5a is closed at `e36f86e869`, R5b-B0 Rust VM
   `None -> func` is closed at `95427f2cd6`/`67dd7e400a`, and R5c printer-only
   is closed at `09f0e51143`; JSON egress D0, typed decoration I0, and profile D1 are closed; profile threading I0/backend_shape/native remain separate and PyVM/reference/Python are `ParkedSealed`.
8. R6 D0 is accepted; D1-D0 closed the negative V0 edge at `f3aa0c4721`;
   D1-D1 closed V1 shape rejects at `640ac083a7`; D1-D2 records the relation shape, D1-D3 the raw/plan census, D1-D4/D1-D5/D1-D6/D1-D7 the issuer/lifecycle/relation/seed design, and D1-D8 design accepted the exact helper shelf/path-observer boundary at `1434663966`;
   the behavior-neutral shelf I0 landed at `b61f6895d2`; D1 and D2a design-only decisions are accepted at `625491fb25` and `18f05950b7`; D2b package co-seal and D2c Raw consumer selection are accepted, and the bounded Raw implementation landed at `7c976ca8b9`.
9. R7: structural guards, README/reference sync, and census closeout.

## D2c atomic Raw ordinary-`New` cutover receipt (7c976ca8b9)

Decision:
  The selected direct-body Raw cohort uses one source-backed claim from the
  installed normal-callable package. Non-selected New families and Plan remain
  outside this row.

Authority chain:
  final parser ordinary-box coverage + resolver owner/site Allocation facts
  -> package co-seal of exact `New` class/arity and optional `Class.birth/N`
  -> one installed affine claim ledger
  -> Raw direct-local admission -> existing NewBox physical owner.

Non-authority and retired edge:
  The claimed path does not consult Builder headers, by-name birth lookup,
  post-lowering target inference, or a claimless NewBox fallback. Compatibility
  rows retain their explicitly parked header route and cannot consume a source
  claim.

Evidence:
  `RUSTFLAGS=-Awarnings cargo check -q` and `cargo check --tests -q` pass;
  normal-callable semantic package tests pass 19/19 and the new affine/source
  claim tests pass 4/4; the shared
  `mir_call_canonical_corridor_guard.sh` passes; source owners remain below
  the 800-line hard stop (`recursive_child_lowering.rs` 731,
  `install.rs` 738). The broad `mir::builder::normal_callable` suite is
  43/49; its six known baseline failures remain the existing
  `script-neutral-window`/`DynamicCarrierMismatch` contracts and are not
  current-change failures.

  The selected row now has one package completion check for unconsumed claims;
  source-backed integration probing reached claim consumption before the
  unrelated existing canonical cleanup failure. No new production fallback,
  retry, or non-selected backend edge was added.

Non-claims:
  Plan-owned/control-flow New, generated constructor bodies, Core13/
  IntegerBox/record/builtin/JSON/op=newbox, Method(None), JoinIR/native,
  PyVM/reference/Python/native_driver, full D2 outside this cohort, and R6
  field deletion remain NoSafeSlice/ParkedSealed as already recorded.

## P2 implementation receipt: MIR-METADATA-CONSUMER-MANIFEST-I0 (5fb0277d91)

Decision:
  Add one observation-only, machine-readable inventory for all 127 stored
  `FunctionMetadata` fields. Each row records the metadata class, producer
  owner/count, production consumer owner/count, selected backend role and
  owner/count, Rust reference/non-selected observations, JSON egress, exact
  observed revision, caller state, and `retire_when`.

Source authority + canonical issuer:
  The stored-field declaration and each existing owner file are the observed
  source. `mir_metadata_consumer_manifest.py` is only the validator; it does
  not issue semantic facts, routes, or backend capability.

Non-authority:
  `FunctionMetadata` as a container, JSON emitter, comments, token presence,
  backend reader names, tests/fixtures, ModuleMetadata, and non-selected
  PyVM/reference/Python surfaces are not semantic or selected-backend authority.

Fail-fast boundary:
  Source field count and manifest rows must be one-to-one; duplicate/missing
  rows, owner/count mismatches, anchor drift, role mixing, and producer-only
  rows without a retention/caller-zero condition reject before success.

Evidence:
  The manifest and validator report 127/127 rows, 323 producer owner-files,
  590 production consumer owner-files, and 83 selected-backend owner-files at
  observed revision `beb82a6756c6c0855dc76096be23b9eafe3c5ae5`. The reusable
  guard is green for positive coverage and missing/duplicate-row rejection;
  the metadata catalog guard and current-state pointer guard are green. The
  existing `src/mir/function/metadata.rs` 804-line boundary is read-only.

Smallest next slice:
  Close out this observation row, then audit `MIR-PHYSICAL-TYPE-INPUT-D0`
  before selecting any physical/backend implementation.

Non-claims:
  No metadata promotion, field deletion, backend activation, ModuleMetadata
  census, physical-type input, seed retirement, or warning cleanup.

## Accepted bounded cell: A-prime exact-i64 storage-policy I0 (not full physical-input D0)

Decision:
  Select only the A-prime exact-`i64` family as the first physical-input
  pilot. This bounded storage-policy row is accepted, and I0 (`f94bb7f2a7`)
  adds the named plain policy and co-seals it without activating a physical backend input; the full four-authority physical-input D0 remains a design stop.

Census boundary:
  Selected `ParserScanLoopBox.skip_while/4` final-source exact-`: i64` rows
  (`pos`, `end`, exact-i64 result) -> package parameter/result facts ->
  `DynamicAPrimeI64SourceRelationViewV1` plus the complete Dynamic Recipe and
  physical input -> `VerifiedAPrimeI64PhysicalDemandV1` -> session
  `DynamicV2PhysicalRepresentationV1::ImmediateI64` ->
  `APrimeI64PhysicalReceiptV1` / `DynamicV2AotCallMetadataProjectionV1` ->
  selected C validation in `hako_llvmc_ffi_checked_callout_lowering.inc`.
  Exact-i64 formals, call arguments/results, induction/return transport, and
  the same-session receipt are included. Dynamic/opaque handles, ExactText,
  generic `value_types`, boxed sums, raw/fastmem layout, other widths, other
  backends, route/cutover/perf are excluded.

Source authority + canonical issuer:
  Exact source/package i64 contracts and the final A-prime source/Recipe
  co-seal own semantic facts. A future physical-input co-sealer may aggregate
  four same-cohort rows but may not issue new meaning. Usable representation
  and ABI transport owners are the existing `ImmediateI64` capability and
  A-prime receipt/call-metadata co-seal. The new plain policy owner is
  `APrimeI64CallableStorageLayoutV1::NonAddressableSsaI64`; its issuer is the
  selected emitter close, after the existing demand, session, ledger, formal,
  and receipt rows have been proven same-brand.

Missing authority:
  I0 issues the dedicated row only at selected-emitter close. `MirType::Integer`
  and `StorageClass::InlineI64` remain logical inventories, not layout/ABI
  truth; raw-layout/fastmem vocabulary, Generic-G0 callable carriers, callout
  wire, JSON strings, ValueId, receipt lane alone, C validation, and other
  backends are non-authority.

Finite state / fail-fast:

| State | Outcome | Fallback |
| --- | --- | --- |
| `OutsideCandidate` | existing owner continues; no physical issue | none |
| `SourceExactButIncomplete` | missing representation/ABI/layout -> `NoSafeSlice` | none |
| `CompleteSameBrandNonAddressable` | I0 may attach the plain policy to the existing projection | none |
| `MissingOrConflict` | typed reject before JSON/backend | none |
| `AddressableOrExpanded` | outside D0; typed reject without layout synthesis | none |
| `ForeignStaleDuplicate` | typed reject; no repair or re-pair | none |
| `UnsupportedBackend` / `Consumed` | typed reject before effect or second take | none |

I0 implementation receipt:
  The plain `APrimeI64CallableStorageLayoutV1` enum, child co-sealer, and
  non-optional projection field are landed. The co-sealer runs at
  `finish_unpublished_draft` after receipt issue and before projection, borrows
  existing same-brand rows, and performs no AST/Recipe/MIR scan or backend
  activation. Parent emitter source is 751 lines (<760).

I0 implementation contract:
  `src/mir/policies/a_prime_i64_callable_storage_layout.rs` owns the singleton;
  `selected_dynamic_physical_emitter/a_prime_callable_storage_layout.rs` owns
  the typed co-seal; `call_metadata.rs` stores only the issued policy. The
  existing guard was extended rather than creating a second activation guard.

Evidence and boundary:
  The exact projection positive, selected-emitter positive, and malformed-
  receipt negative tests pass; the physical-input authority and pointer guards
  pass; `cargo check --lib` passes. The quick profile reports 441 known
  baseline warnings, and whole-workspace fmt remains a baseline failure; no
  new warning class or backend edge was introduced. I0 does not claim the full
  four-authority backend input because `pos`/`end` still lack independent
  `DynamicV2PhysicalRepresentationV1` ledger rows.
  Preserve existing i64/object parity for `pos`/`end` and inner/outer returns
  through one input; reject bare `MirType::Integer`, `StorageClass::InlineI64`,
  JSON `"i64"`, receipt-only lanes, and missing/conflicting/foreign rows; then
  require one selected-backend consumer and caller-zero for the old independent
  return-type/lane/parameter reconstructions.

## Accepted design: MIR-PHYSICAL-TYPE-INPUT-D1

Decision:
  `selected_dynamic_physical_capability` is the sole issuer of an exact
  two-row `pos/end` representation pair. Existing source/package contracts
  already co-seal ordinal 1/2, BindingRef, Recipe value, and exact I64 class.
Source authority + issuer:
  `DirectExactI64` demand + `DynamicAPrimeI64SourceRelationViewV1` -> capability
  pair -> same-session formal adoption -> existing ledger/projection carrier.
Non-authority:
  MIR type/storage, ABI lane/receipt alone, formal index alone, generated-value
  ledger, synthetic producer, JSON/C text, and backend reconstruction.
Fail-fast:
  before Builder open, then before body emission; missing/partial/conflict,
  foreign/stale/duplicate/second-consume, or ABI/representation mismatch rejects
  and discards unpublished state. fallback/retry/re-inference = 0.
Implementation receipt:
  `5bde4c6f92` issues/moves the exact pair through formal adoption, stores the
  opaque carrier, and extends the guard; check/focused test (`1/1`)/diff green;
  JSON/C/backend activation remains closed.
Non-claims:
  returns/generated values/handles, other widths/cohorts/backends, ABI/layout,
  Call R6, and old reconstruction retirement.

Census boundary: exact source `pos/end : i64` -> package contract -> A-prime
relation -> capability pair -> formal header -> same-brand ledger/projection;
includes only those two formals, excludes generated values/returns/handles and
all JSON/C/backend terminals.

Finite states: `OutsideCandidate`, `ExactPairReady`, `OpenedSameSession`,
`Projected`, `MissingOrPartial`, `Conflict`, `ForeignStaleDuplicate`,
`ProducedLedgerSubstitution`, and `Unsupported`. Only the first four proceed;
all negative states reject before effect, with no repair or re-pair.

P0 receipt: `41f82d4be4` split emitter 587/185; D1 pair child 118, capability
parent 652 (<760); no new edge/warning class.

## Ordered follow-ups (design/retirement, not selected)

`MIR-PHYSICAL-TYPE-INPUT-D0`: design_stop; Decision = exact source scalar,
representation, ABI class, and target-layout rows co-seal once; no width/layout inference.
Source authority + issuer = exact-i64 relation plus existing representation/ABI owners;
select one invocation-bound target-layout owner (PinnedText only if the selected Dynamic invocation is proven identical, otherwise a common target-only row).
Non-authority = `MirType::Integer`, `FunctionMetadata`/`StorageClass`, strings/defaults,
backend/TargetMachine reconstruction, JSON/C/Python lanes. Fail-fast = bind/validate
target before session effect, co-seal before publication, and reject missing/foreign/stale/conflicting/duplicate/unsupported rows with no retry.
D0 tasks = states (`Outside`,`SourceMissing`,`RepresentationMissing`,`AbiMissing`,
`TargetOwnerMissing`,`Foreign`,`Conflict`,`Unsupported`,`Ready`,`Consumed`) + four-row schema + old inference-edge delete set; I0 waits for this authority decision. Non-claims = other widths/pointers/aggregates, FunctionMetadata, backend activation, Call R6, VM/PyVM, warnings.
R3 D0 accepted boundary:
Decision: Program generic calls use one immutable catalog built from local defs;
import aliases retain their existing canonical producers; post-merge imports are
not silently scanned. Unique `(name, arity)` candidates become qualified
`Global`; ambiguous candidates reject; unknown source names remain exact `Global`;
`env.`/`nyash.` names become `Extern` with a numeric arity suffix removed.
Source authority + issuer: Program defs plus source `ExprV0::Call` name/arity
-> `ProgramCallTargetCatalog` -> generic lowerer -> `MirInstruction::call`.
Non-authority: `func_map`, `maybe_resolve_calls`, target Const, late Program
canonicalization, import merge membership, optimizer/backend/runtime lookup,
and PyVM/reference/Python (`ParkedSealed`). Fail-fast: catalog before main/defs,
target before argument effects and block publication; empty, duplicate, or
ambiguous target rejects without retry. Smallest next slice: R3a catalog + R3b
late issuer retirement (closed). Non-claims: core field
cutover, operand SSOT, selected terminal closure, and historical backend re-entry.
R4a closed (`bde2c1440b`): `Callee::rewrite_value_operands` is the exhaustive ordered projection owner; owner 2/2, SimplifyCFG 3/3, corridor/pointer/rustfmt/diff green, warning baseline 433, source/check LOC 332/724/180.
R4b closed (`8eca2dd048`): immutable `Callee::for_each_value_operand` -> `methods.rs` Call arm; hakorune-mir-defs 4/4, typed/legacy root 1/1 each, guard/pointer/rustfmt/diff green, warning baseline 433.
R4c/R4d/R4e/R4f/Query T0 are closed; matrix/guards + 433 warnings are recorded. R5c printer, JSON egress/decoration, profile D1/threading I0 are closed. JoinIR remap isolation I0 landed at 0048c0176a; physical type D0 is the design stop; native capability D0 is NoSafeSlice; backend strict-adapter I0 is closed; native D1/Method(None)/R6 remain outside.

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
  state: Parked; Compatibility origin lacks a canonical replacement owner

STAGE1-DIRECT-POST-MACRO-NONPROGRAM-COMPAT-SUNSET-001
  state: closed
  retired by: STAGE1-DIRECT-POST-MACRO-WHOLE-FILE-PROGRAM-SEAL0-I0-R0

RAW-NONPROGRAM-ROOT-COMPAT-SUNSET-001
  state: closed; owner / residual / execution callers = 0
```

## Ordered frontier

```text
Now
 MIR-PHYSICAL-TYPE-INPUT-D0
  -> design_stop: D0-A select one invocation-bound storage-layout owner; D0-B co-seal four rows; D0-C finite rejects; D0-D inventory old inference edges
Next (not selected)
  -> MIR-PHYSICAL-TYPE-INPUT-I0 after D0: one exact-i64 target row and one selected Dynamic consumer; JSON/C/backend remain ParkedSealed
Call lane (after D0/I0): R6 canonical operand/escape projection and lifecycle inventory are closed; JoinIR remap is test/reference-only at 0048c0176a, so do not revive caller-zero code
  -> cutover: `Call { dst, callee: Callee, args, effects }`; remove `func`, `Option<Callee>`, INVALID sentinels; decide MirCall/CallFlags, Method(None), Closure construction, and Constructor/NewBox boundary together; then typed-only terminal consumers, fallback/retry/by-name lookup = 0, parity + one guard
After MIR Call retirement
  1. MIRBUILDER-PR-STRUCTURAL-GATES-I0
  2. MIRBUILDER-R4-FINAL-CONFORMANCE0-C0
  3. root mode session-localization, stale ordinary-New comment, then mimalloc promotion gate and .hako selfhost migration
Parked
  -> SCRIPT-STATIC-PRODUCTION-CONVERGENCE-R0 until canonical consumer > 0
  -> Loop common/Generic/callable physical follow-ups until a named caller
  -> MIR-BUILDER-NORMAL-SCRIPT-MOD-SHELF-R0 first: replace the builder `#[path]` bridge with `normal_script/mod.rs`, then MIR-EMPTY-DIR-CLEANUP-R0 + MIR-COMMON-V2-SHELF-R0 (post-R7 physical shelf order)
  -> NORMAL-ROOT-OLD-GUARD-RETIRE-R0 + NORMAL-ROOT-BOOL-PROJECTION-RETIRE-R0, then MIRBUILDER-WARNING-RETIREMENT-R0 (A/B/C; baseline dead_code/allow census gets reasons); mode/projection/type-name/syntax-loan and performance/converter/llvmlite/Home/selfhost stay separate
Closed / do not reopen from a mirror
  -> normal-root T0/C0/R0 and 68/68 replacement manifest
  -> H2 selected Dynamic cutover through W6
  -> FunctionCall lexical/special rows as NoSafeSlice
  -> result-discard closure and normal-root caller-zero warning projections
  -> state TOML parser integrity, pointer compactness, and builder test-home split
```

## Historical evidence queue (non-authoritative)

```text
closed NoSafeSlice
  FUNCTION-CALL-LEXICAL-CALLEE-CLASSIFICATION-D0
  -> no shared pre-effect issuer or aggregate old-edge delete set exists.

landed BoxShape
  FUNCTION-CALL-PREFLIGHT-OWNER-TEST-SPLIT-I0
  -> production owner 790 -> 329 lines; five unchanged tests live in a 443-line
     child. Focused tests are green; reusable guard red is parent baseline debt.

accepted census
  FUNCTION-CALL-DIRECT-VS-VALUE-CALL-COMPAT-CENSUS-D0
  -> AST kind cannot distinguish identifier-value from FreeStatic; late target
     resolution has one main chain, two recovery consumers, and two tail variants.

accepted design
  FUNCTION-CALL-CALLEE-BINDING-AND-EVALUATION-ORDER-D0
  -> FunctionCall is direct FreeStatic/explicit special; Call evaluates one callee
     value first; arguments evaluate once left-to-right after target selection.

closed NoSafeSlice
  RAW-FUNCTION-CALL-PRE-EFFECT-DECISION-OWNER-D0
  -> argument lowering can mutate variable_map before current target resolution;
     moving target selection earlier changes callee choice and diagnostic order.

  SCRIPT-ORDINARY-DIRECT-CALL-PREFLIGHT-RECEIPT-D0
  -> ordinary retains only name plus AST arguments; target/recovery/header/tail
     decisions remain later and Builder-owned, so no affine Script transfer exists.

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
## Historical parked boundary
```text
source-level Home ownership and unimplemented language features until the
repository-wide final pipeline is Complete; .hako selfhost MirBuilder/parser
migration and post-Loop root/current-state/design-registry cleanup follow their
owning SSOT task orders; new language semantics and default Raw/Canonical
cutover remain parked before final conformance.
```
