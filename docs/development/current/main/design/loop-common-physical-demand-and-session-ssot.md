---
Status: SSOT
Date: 2026-08-08
Decision: accepted after external review — `LOOP-COMMON-PHYSICAL-DEMAND-AND-SESSION0-D0-r1`
Activation: `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, callable static-prefix
P0, bounded `LOOP-PHYSICAL-PREPARE-P0`, common-boundary design stop,
caller-zero `LOOP-PRELUDE-ARGUMENT-RECEIPT-P0`, passive operation/effect S0,
Callable/G0 adapters, and cross-profile parity are closed. Decision B now
separates full-demand preflight from leaf emission; the Builder-free
`LOOP-RECIPE-OPERATION-PHYSICAL-DEMAND-P0` and the behavior-neutral
physicalizer module split, physical block receipt, private ConstI64
leaf-emitter canary, bounded ReadBinding I0, callable full physical P0, and
G0 exact-ingress I0 are closed. Top-down review revised the next boundary:
before a G0 fresh-session canary, one private Builder-free segment/resume
layout must be mechanically derived from Recipe/JoinSig. Operation production
activation remains 0. The bounded After-closure canary is green: the real
Prelude receipt feeds the complete seven-operation Callable dispatch, fixed
CFG edges, and canonical CFG/identity sealing. The Tail handoff now reads the
exact binding through canonical identity, validates the existing trivial ABI,
and claims Completion/return coverage once. The sealed After receipt also
moves a non-Clone callable profile-close receipt proving exact
`7 = Pure4 + Read2 + Write1` coverage, the Bool condition, owner, terminal
block, and After predecessor. Finish must consume that receipt through a
non-no-op `finish_profile_close` closure. DraftSeal, production selection,
retry/fallback, and legacy retirement remain closed. The bounded
`CALLABLE-LOOP-DRAFT-SEAL-P0` canary now consumes the profile-close receipt
through the existing typed finish terminal, then uses DraftSeal
prepare/commit to produce one unpublished `CompletedFunctionDraftV1`; no
collector or module publication is performed. The production-edge census and
Admission D0 are closed as `NoSafeSlice`. The source/facts bridge D0 is
accepted without a new semantic Bridge owner: the existing resolver ledger
plus neutral SyntaxFacts/SourceMap are the target production boundary. The
source/facts issuer S0 and bounded logical issuer D0/S0 are now closed with
bounded negatives, exact parity, and caller-zero/current receipt audit. The
profile Recipe shape is production-owned while the old shape helper remains a
test-only parity wrapper. `CALLABLE-LOOP-PRODUCTION-PREPARED-INGRESS-D0` is
accepted and its S1/S2 caller-zero products are closed.
`LOOP-CALLER-ZERO-PARITY-G0-D0` is also accepted. Its exact resolver-issued
G0 source/input/entry capability is carried by a thin compiler-side composite
ingress; neutral S4 remains the sole Recipe/effect/After owner. I0 is closed
as Builder-free exact ingress plus fifteen-row `prepare_all`. The accepted I1
design now requires `LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1`, segment-aware
block cutover R2, and neutral recursive After R3 before G0 physical I1-R0.
R1 is now closed as a Builder-free derived layout; R2 is the current bounded
physical cutover.
No named production caller switch or G0 physical implementation is open.
Scope: common Loop physical demand, fresh unpublished function session, failure discard, completion/DraftSeal handoff
Related:
  - docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/reference/mir/loop-recipe-contract.md
  - docs/reference/mir/generic-loop-stage-matrix.md
  - src/mir/builder/resolved_lowering/README.md
  - docs/development/current/main/investigations/loop-physical-prepare-design-correction-r0-task-2026-08-07.md
---

# Loop Common Physical Demand and Session SSOT

## Decision

Close the post-Recipe boundary before physical implementation begins.

```text
resolver / source map
  -> VerifiedLoopRecipeCoSealV1
  -> VerifiedLoopOperationEffectProductV1
  -> VerifiedLoopOperationPhysicalDemandV1
  -> one thin prepared execution product
       PreparedCallableLoopPhysicalizationV1
       OR PreparedGenericG0LoopPhysicalizationV1
  -> one fresh unpublished function session
       completion moves here exactly once
  -> outer callable lowerer + one common recursive Loop physicalizer
  -> open After continuation
  -> existing completion / DraftSeal
  -> one unpublished function draft
```

The canonical full-operation input is the private, move-only, AST-free, and
physical-ID-free `VerifiedLoopOperationPhysicalDemandV1`. It bundles the
Core-bearing operation/effect product with one common continuation capability;
the two are never passed as independent physicalizer arguments. Each thin
prepared product is move-only and physical-ID-free. Its source-backed input
must be issued by one existing-owner ingress receipt that pairs the exact
`ResolvedFunctionLoweringInputV1` with its resolver ledger view and, where the
profile requires it, the exact callable index/header. No prepare/physicalizer
policy may inspect or rematch its AST. The current
`NormalCallableSemanticLoanPortV1` is only the raw-lowering host and does not
yet issue this receipt; that source-loan expansion is the remaining D0 gate,
not a reason to add a second resolver or to remove `cfg(test)` from prepared
types prematurely.
A profile prepare consumes one inner demand plus already sealed boundary
capabilities and fixes their exact compatibility before the first Builder
effect. Only the common inner demand is consumed by the recursive Loop
physicalizer. Neither product is a new Recipe, selector, SSA, CFG, PHI,
transaction, Return writer, or publication owner.

Nested control can split one logical Recipe block into multiple physical
segments. Generic G0 is the counterexample: root block B1 contains a carrier
read, a nested Loop item, then the root update. Therefore logical-block-to-one-
physical-block mapping is not a sufficient execution contract. A private,
move-only `PreparedLoopPhysicalLayoutV1` target is mechanically derived from
the complete Recipe/JoinSig and exact operation coverage before Builder
effect. It owns only ordered segment placement and transfer compatibility:

```text
Recipe item -> exact segment
segment -> ordered operation rows + one verified transfer
nested After -> exact parent resume segment
```

Recipe/JoinSig remain the sole logical authority. The layout may not infer
control meaning, reorder by item key, accept a profile name, or survive as a
second Recipe. Unsupported structural items reject with typed `NoSafeSlice`
before Builder mutation. Canonical CFG remains the sole physical block/edge/
terminator owner after the layout is admitted.

The existing `VerifiedLoopPhysicalDemandV1` is a closed topology-only P0
compatibility transport. It feeds only the historical caller-zero topology/
After probe, carries no complete operation/effect ledger, and cannot be
extended, renamed, or reused as the canonical operation input. The module-
split row moves the flat file into one directory facade, deletes the old flat
module, and quarantines that entry behind the topology-only test facade. Two
module entries or two topology authorities are forbidden.

`Admission` remains the semantic family-selection term. `Prepared...` means
only that already verified capabilities have been related into one executable
request. The prepared product owns exactly one new relational fact: its Loop,
Prelude/input, Tail, return ABI, Completion, owner, and frame may execute
together once. It does not own or copy the component semantic truths.

The existing owners remain authoritative:

| Concern | Sole owner |
| --- | --- |
| source membership, owner, frame, Scope/Region | resolver ledger and source map |
| logical operations, keys, recursive nesting | `LoopRecipeV1` |
| logical ports, edges, carrier obligations | `LoopJoinSigV1` |
| source/effect/input relations | `VerifiedLoopRecipeCoSealV1` |
| `BindingRef -> ValueId`, lexical SSA | `CanonicalSsaFunctionSessionV2.identity` |
| physical blocks, predecessors, sealing | `CanonicalCfgSessionV1` |
| provisional and patched PHI lifecycle | the function session's one `PhiTxn` |
| source completion evidence | `VerifiedFunctionCompletionV1`; moved exactly once from the prepared request into the function session |
| mutable physical completion consumption | fresh `CanonicalSsaFunctionSessionV2.completion` / `ResolvedFunctionCompletionConsumptionV1` |
| common function-local finish terminal | `CanonicalSsaFunctionSessionV2::finish_for_draft_seal` target API |
| captured caller restore, unpublished discard, prepared close | `CanonicalFunctionLoweringSessionV1` |
| detached DraftSeal prepare and rejected-owner retention | `OpenFunctionDraftSealV1` |
| sole function commit terminal | `PreparedFunctionDraftSealV1::commit` through prepared session close |
| draft collection and module publication | `ModuleDraftCollectorV1` plus the existing module transaction / `ModuleBuilderInvocationSessionV1` |

## Why this boundary exists

Recipe completion is not physical completion. A verified Recipe proves the
logical program, but it does not prove:

- which already-resolved callable prelude supplies an external value;
- that every Recipe input can be materialized in the preheader;
- that the function return has an exact supported ABI;
- that the terminal source binding is the value returned;
- that one fresh session can finish CFG, SSA, PHI, completion, and DraftSeal;
- that late failure leaves the live caller unchanged.

These obligations must be sealed once before mutation. A physicalizer must not
rediscover them from AST, source names, route labels, or existing MIR.

### Repository audit receipt

| Observed code authority | Confirmed boundary |
| --- | --- |
| `CanonicalSsaFunctionSessionV2::new` consumes `VerifiedFunctionCompletionV1` into `ResolvedFunctionCompletionConsumptionV1` | Completion cannot remain owned by a prepared sibling after session open |
| `CanonicalDirectAccumSsaLowererV1::lower` finishes semantics/If/identity/Phi/binding/completion but omits `cfg.finish` | prose ordering alone is insufficient; the V2 finish terminal is required |
| `ReadyFunctionDraftSealV1::new` currently accepts only ready completion + current block | current Ready type does not prove common CFG/SSA/PHI closure by construction |
| `ResolvedFunctionLoweringInputV1` is an existing exact read-only source/function/forest/header view | prepared outer product may retain it; common Loop demand must not receive it |
| `NormalCallableSemanticLoanPortV1` currently forwards a raw body after consuming a loan, while `VerifiedNormalCallableSemanticLoanV1::into_parts()` retains only lineage + request-local lowering state | a source-loan expansion receipt must be issued before I0; AST re-walk, name lookup, and synthetic catalog/header pairing are `NoSafeSlice` |
| `CompilationContext::callable_declaration_catalog()` is installed before normal lowering | the catalog is an existing borrowed authority, not an automatic source/forest pairing; owner/frame/scope/index/header identity must be checked once |
| `loop_physical_prepare.rs::VerifiedCallableFunctionLoweringInputV1` is `cfg(test)` and static-header-profile-specific | it remains a canary witness; removing `cfg(test)` is not a production ingress design, and normal callables must not be forced through that header profile |
| `VerifiedCallableSingleLoopSourceMapV1` carries source roles, BindingRefs, loop context, and resolved exit evidence only | current co-seal cannot issue ABI or Completion authority |
| `PhiTxn::abort_on_err` sees only still-pending provisional PHIs | whole-session discard, not PHI rollback, owns atomicity |

### Current co-seal stop correction

`RECIPE-COSEAL-I0-R0` has no authority to issue an exact return ABI or a new
`VerifiedFunctionCompletionV1`; its current source map contains source-role and
resolved-exit evidence only. The current row therefore publishes these
disjoint caller-zero products:

```text
VerifiedLoopRecipeCoSealV1
  Core / Recipe / JoinSig
  operation-source and input-source relations
  semantic context
  VerifiedLoopContinuationContractV1

VerifiedCallablePreludeV1
VerifiedCallableTailV1
```

The existing exact ABI and Completion capabilities remain with their existing
issuers. `LOOP-PHYSICAL-PREPARE-P0` later consumes all components once and
either issues one prepared execution product or returns typed `NoSafeSlice`.
`VerifiedLoopAfterTailEnvelopeV1` is rejected: Loop continuation and callable
Tail must never be fused and then split again.

## Product boundary

### Common product

### Operation physical demand

The full-program preflight consumes this private move-only product:

```text
VerifiedLoopOperationPhysicalDemandV1 {
  context: VerifiedLoopSemanticContextV1,
  operation_effect: VerifiedLoopOperationEffectProductV1,
  continuation: VerifiedLoopContinuationContractV1,
  index: private LoopOperationPhysicalIndexV1,
}
```

The context owns only the resolver-issued semantic identity relation
(owner/origin/source-kind/loop-site/frame/Scope/Region); it is moved from the
existing source authority and is not re-derived from Recipe keys. The
operation/effect product owns the moved Core and item-keyed source/effect
ledger. The continuation owns only the logical Loop After capability. Callable
and Generic G0 adapters issue the common context and continuation by exact move
from their existing resolver/source products; they do not share source types,
compare counts, or pass two independent arguments to the physicalizer. The
index is a private key-only lookup aid and never a second semantic or physical
truth. The existing
`VerifiedLoopPhysicalBoundaryV1` remains topology-only and is invalid for the
operation program because it drops source anchors.

Decision B keeps whole-program preparation and leaf emission separate:

```text
VerifiedLoopOperationPhysicalDemandV1
  -> prepare_all
  -> PreparedLoopOperationProgramV1
       complete Recipe-derived operation schedule
       exact complete-coverage receipt

PreparedLoopOperationEmissionV1
  -> one private leaf emitter
```

The full demand exposes no first/select/filter/take-operation API. Recipe
Loop/Block/Item structure is the sole execution-order authority; an evidence
vector sorted by key is only storage order. `PreparedLoopOperationProgramV1`
retains the complete demand and common continuation. A leaf emission owns only
one already-prepared operation, source evidence, expected Loop, and expected
logical block; it never sees Recipe, profile, Tail, ABI, Completion, Return,
DraftSeal, publication, or continuation.

The first leaf canary may use a private test-only ConstI64 constructor, but it
must not obtain that row by extracting it from a seven-operation Callable or
fifteen-operation Generic G0 demand. A synthetic one-operation full Recipe is
not the first authority and may be added only as a later integration fixture.

The accepted conceptual shape has two layers:

```text
PreparedCallableLoopPhysicalizationV1
  input: exact ResolvedFunctionLoweringInputV1
  loop: VerifiedLoopOperationPhysicalDemandV1
  prelude: VerifiedCallablePreludeV1
  tail: VerifiedCallableTailV1
  return_abi: existing exact ABI capability
              (ExactTrivialReturnAbiV1 for the first profile)
  completion: VerifiedFunctionCompletionV1

PreparedGenericG0LoopPhysicalizationV1
  input: exact ResolvedFunctionLoweringInputV1
  loop: VerifiedLoopOperationPhysicalDemandV1
  tail: VerifiedGenericG0TailV1
  return_abi: existing exact ABI capability
              (ExactTrivialReturnAbiV1 for G0)
  completion: VerifiedFunctionCompletionV1
```

The exact Rust field split may remain private, but the following contract is
fixed.

The common prepare consumes the Loop co-seal once and moves it wholly into one
`VerifiedLoopOperationPhysicalDemandV1`. Callable Prelude/Tail remain separate
inputs; they are not
split back out of the co-seal. The prepare never borrows, clones, or re-catalogs
the co-seal. The inner receives, without duplication:

- verified Recipe/Core and JoinSig;
- the moved resolver-issued semantic context (including Scope/Region);
- semantic owner/origin/source-kind, loop source and execution frame;
- Scope/Region relation;
- exact operation-source and input-source relations;
- `VerifiedLoopContinuationContractV1`, which owns only the logical Loop After
  port/capability.

`LoopOperationPhysicalIndexV1` is a private, key-only search index over
existing logical keys:

- Recipe binding/value/item/block keys and their placement roles;
- Recipe input value -> logical preheader port;
- Recipe item -> owning Loop/Block + exact input/output value keys;
- JoinSig port/edge obligations -> logical placement roles.

`BindingRef`, source site, source/effect relation, semantic identity, Recipe,
and JoinSig truth remain solely in the moved co-seal owner held by the inner
demand. The private index cannot be independently constructed, published, or
verified; it refers to those logical keys and may be rebuilt only inside the
same prepare operation. The physicalizer consumes the demand as one product,
not Recipe plus a second public topology truth.

The physicalizer boundary is move-only. Prepare must issue a private consuming
operation for `VerifiedLoopOperationPhysicalDemandV1`; borrowing, cloning, a
second co-seal, or MIR reconstruction is invalid. This prevents logical demand
from being silently reused after it crosses into physical lowering.

The callable prepared product relates the non-Loop obligations:

- exact prelude caller/site/target/result contract when a prelude result is
  required;
- destination source `BindingRef` for that result;
- exact terminal return statement and value sites;
- exact terminal source `BindingRef`;
- exact supported return ABI capability;
- the matching `VerifiedFunctionCompletionV1`.

The prelude contract contains no `ValueId`. Its current source shape also does
not prove argument bindings: arity is not an argument materialization receipt.
The selected prerequisite is one move-only, AST-free
`VerifiedCallablePreludeArgumentListV1`. Each row carries an exact ordinal,
`SourceExprSiteV1`, resolver-issued `BindingRefV1`, and exact `i64` ABI. The
issuer reads `VerifiedResolvedFunctionV1.variable_ref(site)` and admits only
`ResolvedLexicalRefV1::Local` owned by the caller. Upvar, literal, nested
expression, unknown site, foreign binding, and unsupported ABI are typed
`NoSafeSlice`. No new resolver or semantic owner is introduced.

The outer lowerer consumes this list once, reads each BindingRef through the
canonical session identity, and materializes the external Prelude result. The
Loop input initializer is a separate exact source-site obligation: it is
resolved through the existing source view, emitted as the entry value, and
published under the co-sealed Loop input BindingRef. The Prelude result local
and Loop input binding must never be conflated. Only then does the adapter
issue one private `ReadyLoopEntryV1`. AST reread, name lookup, and arity-only
reconstruction are forbidden. It then opens the exact function session, moves
Completion into `CanonicalSsaFunctionSessionV2::new` exactly once, and retains
Prelude/Tail/ABI evidence only. The future full operation physicalizer consumes
the `VerifiedLoopOperationPhysicalDemandV1` plus that entry receipt and never
observes the callable boundary.

The Generic G0 prepared product wraps one instance of the same complete
operation-demand type but retains its existing `L0.After/b1` boundary
capability. It neither
reuses the callable prefix `value` Tail nor creates a G0 physicalizer.

The Generic G0 window lease is the source authority for its Scope/Region pair.
The lease therefore retains the pair alongside its existing owner/source/frame
brand, and the G0 product moves that context into the common demand. If a
profile cannot provide this exact pair, the demand is a typed `NoSafeSlice`; no
synthetic region or route-local context is permitted.

The G0 adapter must consume the existing S4 product into a common co-seal view
by a disjoint move of its already verified Core/relations/After evidence. If
that view cannot be issued without copying source truth or re-verifying the
Recipe, the G0 adapter is `NoSafeSlice` and parity remains parked.

### These are prepared execution products, not callable megaboxes

No profile prepared product becomes a universal callable semantic owner. It
implements no new Call, ABI, Loop, Return, or publication algorithm. It owns
only the relational compatibility proof that already sealed capabilities
belong to the same callable execution:

```text
source/target identity  -> existing resolver/catalog authority
argument/result ABI     -> existing verified ABI capabilities
Loop meaning            -> Recipe/JoinSig/co-seal
terminal disposition    -> existing completion capability
physical commit         -> existing DraftSeal owner

profile prepared product
  -> one exact owner/site/BindingRef compatibility proof
  -> one fixed Prelude -> Loop -> Tail -> Completion order
```

Prelude/Input, Loop, Tail/Return, and Completion stay typed sub-capabilities.
The two-layer product prevents the Loop physicalizer from observing the
callable boundary at all; only the outer callable lowerer sees both siblings.
They are not flattened into an opaque `CallablePlan` payload. The envelope
moves or borrows sealed evidence and cannot copy facts into a second catalog.
A non-Loop callable remains outside this Loop-specific prepared product, so
this D0 does not pre-empt the final general callable design.

### Completion and ABI are separate

`VerifiedFunctionCompletionV1` is necessary but insufficient. It seals exit
cardinality, terminal statement kind, target function, cleanup, and declared
result contract. It does not by itself carry the return value `BindingRef`,
the return expression site, or a concrete physical ABI. An unannotated
explicit return can therefore pass completion verification without being safe
for this physical row.

Each prepared profile with a value return requires both:

```text
exact terminal value site + BindingRef + exact return ABI
AND
matching VerifiedFunctionCompletionV1
```

For the first row the supported ABI is the already verified exact trivial
`i64` capability. Unannotated, dynamic, unknown, or inferred-by-name return
types reject before Builder effects. Later ABI profiles require separate
verified capabilities, not widening inside the physicalizer.

### Loop After and callable Tail are separate

The selected callable profile returns the prefix `value` binding:

```hako
local value = helper.to_i64(n)
local i = 0
loop i < 1 { i = i + 1 }
return value
```

Its terminal operand is not a Loop carrier After value. Generic G0 currently
returns `L0.After/b1`; that is a different profile adapter.

```text
logical Loop After capability != callable terminal Tail capability
```

The callable prepared product keeps both fields distinct. A profile adapter may prove
the same binding supplies both, but no consumer may infer that equality.
Generic G0's `VerifiedGenericAfterEffectG0` remains its boundary input and is
adapted beside the same common inner demand; it is neither the common Loop
authority nor the callable Tail authority.

## Forbidden contents

The inner demand and co-seal must be AST-free. The prepared profile product may
retain only the exact existing `ResolvedFunctionLoweringInputV1` source view;
it must not add independent raw source fields. Across these products the
following are forbidden:

```text
raw AST / StmtRef / ExprRef fields outside ResolvedFunctionLoweringInputV1
source or callable name selectors
path-suffix or ordinal rematching
legacy route ID or scheduler cursor
ValueId / BasicBlockId / PHI destination
MirBuilder or CanonicalSsaFunctionSessionV2
PhiTxn or rollback journal
ResolvedFunctionCompletionConsumptionV1
retry / fallback / reselection
commit or publication capability
```

The old DirectAccum-only `VerifiedLoopPhysicalInputV1` contains Recipe and
JoinSig only. It is a pilot input and must not be renamed or reused as the
final common demand; it lacks the co-sealed source/effect, continuation,
private-index, ABI, Tail, and prepared execution contract.

No session brand is added merely to pair either demand with a session. The
pre-effect issuer verifies semantic owner/frame/scope contracts; the consumer
then checks them against the freshly opened existing session. If the existing
session cannot expose the required prepare facts, the result is
`SessionPreparationUnavailable`, not a second session identity.

## Exact consumption

The common prepare consumes one `VerifiedLoopRecipeCoSealV1` plus the complete
operation/effect product and either issues one non-Clone
`VerifiedLoopOperationPhysicalDemandV1` or returns a typed rejection retaining
the sole unconsumed owner. A thin callable or G0 prepare then consumes exactly
one full demand plus the profile's disjoint boundary capabilities and issues
one prepared product. Neither step re-runs Recipe verification, mints keys, or
consults the legacy scheduler.

The outer profile entry consumes one prepared product to open the exact fresh
function session. `VerifiedFunctionCompletionV1` moves exactly once into
`CanonicalSsaFunctionSessionV2::new`; it cannot remain in the prepared product
or a sibling boundary. The outer lowerer retains only Prelude/Tail/ABI evidence,
transfers the full operation demand exactly once to the future full
physicalizer, and later claims the exact return operand through
`session.completion`. Lowering by `&demand`,
cloning a split/prepared product, recreating one from MIR, or trying a second
route is forbidden.

Logical keys map to physical owners as follows:

| Logical evidence | Physical interpretation |
| --- | --- |
| `LoopBindingKey` + source `BindingRef` | canonical identity/BindingSSA |
| Recipe input + preheader relation | outer prelude/input materialization |
| `LoopItemKey` + owning block + value keys | common recursive physicalizer |
| JoinSig port/edge role | canonical CFG allocation and sealing |
| carrier obligation | canonical identity plus the one PHI transaction |
| Loop After capability | open allocation result first; sealed `ReadyLoopAfterContinuationV1` before any Tail read |
| terminal Tail capability | outer callable lowerer and completion consumer |

The topology physicalizer initially returns an open After/continuation receipt.
That receipt is not readable by Tail. A callable profile must first consume it,
issue the verified CFG edges, seal CFG and identity for every loop block, and
mint one session-local `ReadyLoopAfterContinuationV1`. Only that sealed receipt
may be passed to the outer Tail handoff. The physicalizer must not write
`Return`, take the function, publish a draft, or close the module.

### Session-local entry receipt

Opening a function session does not prove that Prelude/parameter/input values
have been installed. The outer profile lowerer must materialize every required
entry binding first and issue one private, session-local `ReadyLoopEntryV1`.
The future full operation physicalizer requires:

```text
PreparedLoopOperationProgramV1
+ ReadyLoopEntryV1
+ borrowed canonical CFG / Binding SSA / PhiTxn services
```

`ReadyLoopEntryV1` owns no source or callable semantics. It proves the
temporal fact that the exact logical input keys required by the demand, and
their resolver-issued BindingRef-to-entry materialization, are already
installed in this function session. It is non-Clone, cannot cross a session,
and is consumed once by the physicalizer. A receipt containing only arity or a
source-site label is insufficient.

The argument list is a Prelude product, not a Loop demand field. It is
consumed before `ReadyLoopEntryV1` is issued and is never passed to the common
physicalizer. This preserves the single common physical algebra while keeping
call argument source proof at the callable boundary.

## Fresh session and atomic failure law

Neither demand owns freshness or rollback. Existing transactions do.

```text
A. semantic prepare failure
  -> no Builder/session effect

B. fresh session open + exact owner binding
  -> reversible caller-capture/function-state effect
  -> no MIR / ValueId / BasicBlockId emission yet

C. physical emission failure
  -> rollback only still-pending unpatched provisional PHIs
  -> retain any PHI cleanup failure in the typed failure
  -> discard the complete unpublished function session
  -> restore the captured caller once
  -> no repair, retry, fallback, or route advance

fresh request
  -> open a new candidate/session from source authority
  -> allocate new physical IDs
  -> lower independently
```

Stage A must complete before B. Exact owner/session binding in B must complete
before C. Any B/C failure discards the whole session; it does not return to A
or select another route.

The sole owners are:

- fresh module/candidate state: existing `ModuleBuilderInvocationSessionV1`
  with the canonical Fresh seed policy;
- fresh function state and caller capture:
  `CanonicalFunctionLoweringSessionV1` over the existing function-owned state
  transaction;
- PHI-local provisional abort: the session's `PhiTxn`;
- whole unpublished function discard:
  `CanonicalFunctionLoweringSessionV1::discard_unpublished` or rejected
  DraftSeal discard;
- function commit: `PreparedFunctionDraftSealV1::commit` through the prepared
  function-session close;
- module commit: the existing module transaction / `ModuleBuilderInvocationSessionV1`
  terminal after `ModuleDraftCollectorV1` admission.

There is no Loop-local Builder clone, `LoopEmissionDraft`, undo log, second
transaction, or same-session retry. A fresh-session proof compares semantic
result and live-caller fingerprints; it must not require `ValueId` or
`BasicBlockId` numbers to match across sessions.

`PhiTxn::abort_on_err` rolls back only provisional PHIs that are still pending
and unpatched. It is best-effort local cleanup and diagnostic hygiene, not the
atomicity owner. It does not repair patched PHIs, other MIR instructions, or ID
allocation. Even if PHI cleanup itself reports a suppressed failure, the
poisoned unpublished function is still removed by whole-session discard.

## One typed function-finish terminal

The new common path must not rely on every lowerer remembering this order.
`CanonicalSsaFunctionSessionV2` gains one consuming target API:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal(...)
  -> Result<ReadyFunctionDraftSealV1, CanonicalFunctionFinishErrorV1>
```

Profile-specific ledgers remain with their profile lowerer. Before entering the
common terminal, that lowerer must consume them and provide one private
`ReadyCanonicalProfileCloseV1`. This is a temporal receipt, not a semantic
owner. The common terminal then consumes every common function-local owner and
is the only issuer of `ReadyFunctionDraftSealV1` for
`CanonicalSsaFunctionSessionV2` paths.

The target order is:

```text
1. materialize verified callable prelude and Recipe inputs
2. physicalize the recursive Loop, leaving After open
3. close the fixed profile's CFG edges and seal the After continuation
4. materialize the verified Tail operand and claim completion once
5. consume profile-specific ledgers -> ReadyCanonicalProfileCloseV1
6. close semantic scopes and seal the terminal CFG
7. finish CanonicalCfgSessionV1
8. finish semantic, If-control, and identity/BindingSSA preconditions
9. commit the one PhiTxn
10. finish the remaining resolved-binding ledger and
   ResolvedFunctionCompletionConsumptionV1
11. issue ReadyFunctionDraftSealV1
12. prepare every detached DraftSeal check
13. commit DraftSeal once
```

The current production resolved DirectAccum lowerer is a parity oracle, not the
final common owner. The earlier census found a missing whole-function
`CanonicalCfgSessionV1::finish` call; the typed
`CanonicalSsaFunctionSessionV2::finish_for_draft_seal` terminal now owns that
finish for the V2 path. The omission must not be copied into a future common
path. Existing non-V2 direct construction is frozen compatibility debt: the
first R0 adds no caller there, and final retirement makes
`ReadyFunctionDraftSealV1::new` unavailable to every production lowerer. Tests
may then use only an explicit test factory. For every migrated V2 path the
invariant is:

```text
ReadyFunctionDraftSealV1 exists
  == common CFG / SSA / PHI / binding / Completion owners are closed
  && the profile-specific close receipt was consumed
```

### R0 audit lock (2026-08-07)

The repository audit fixes the migration boundary before implementation. The
canonical V2 session is constructed by exactly three profile lowerers:

```text
trivial_ssa/lowerer.rs
direct_accum_lowerer.rs
nested_predicate_lowerer.rs
```

The current production `ReadyFunctionDraftSealV1::new` census contains those
three V2 callers plus one non-V2 `CanonicalFunctionLowererV1` compatibility
caller. R0 migrates only the three V2 paths. The non-V2 caller is a named
compatibility debt and may not gain new callers; its later retirement is a
separate decision. Test-only constructors are allowed only through an explicit
test factory and are not production evidence.

The finish API must consume a typed terminal receipt rather than re-deriving
source facts at the end of lowering. The target shape is conceptually:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal(
    self,
    builder,
    profile_close: ReadyCanonicalProfileCloseV1,
) -> Result<ReadyFunctionDraftSealV1, CanonicalFunctionFinishErrorV1>
```

The exact Rust visibility may remain private, but the contract is fixed:

- `body`, `body_end`, `target_function`, `current_block`, source site, and
  return operand are not re-inferred from raw AST/source/MIR arguments at the
  terminal. Function/body identity and completion target are sealed when the
  V2 session opens; the profile close receipt carries the exact terminal block
  and already-claimed completion witness.
- `ReadyCanonicalProfileCloseV1` is move-only, non-cloneable, and contains only
  profile-ledger closure evidence. It is a temporal receipt, not a new
  semantic owner or a second Completion/CFG/PHI authority.
- the common terminal is the sole issuer of `ReadyFunctionDraftSealV1` for V2
  sessions. A direct V2 `ReadyFunctionDraftSealV1::new` caller count of zero
  is a guard, not a prose claim.
- a mismatch, duplicate close, missing close, or completion/body identity
  mismatch rejects before the terminal consumes the session. Any failure
  after the fresh session opens discards the whole unpublished function and
  restores the caller once; same-session repair/retry is forbidden.

The R0 acceptance pack therefore includes all of the following, with no
profile or MIR acceptance delta:

```text
DirectAccum omission: missing cfg.finish cannot issue Ready/DraftSeal
finish order: CFG/semantic/If/identity/binding/Phi/completion close once
profile receipt: missing/duplicate/foreign receipt rejects
completion identity: body/site/end/target mismatch rejects before effects
late failure: unpublished function is discarded and caller is unchanged
fresh reuse: a failed session cannot poison the next session
caller census: V2 direct Ready constructor callers = 0
non-V2 census: compatibility caller remains named and non-growing
source/README/reference/current-entry update in the same implementation commit
```

This audit lock is deliberately narrower than a universal function-finalizer
redesign. It does not migrate the non-V2 lowerer, add a semantic owner, change
accepted profiles, or open physical Loop lowering.

### Callable production-edge census (2026-08-08)

The new callable physical products remain test-only:

```text
loop_physical_prepare.rs
callable_loop_physical_canary.rs
loop_recipe_physicalizer/callable_canary.rs
```

No production caller currently supplies
`PreparedCallableLoopPhysicalizationV1 -> profile-close -> Completion ->
DraftSeal`. The nearest production host is
`NormalCallableSemanticLoanPortV1::lower_normal_top_level_function`, whose
loop child edge still enters
`RawInvocationChildPortV1::lower_loop ->
PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1 ->
lower_loop_or_freeze_v1`. Its current output is a legacy pending function
session and `LegacyReplaceWholePair`, not `CompletedFunctionDraftV1`.

Therefore `CALLABLE-LOOP-PRODUCTION-EDGE-D0` closes as `NoSafeSlice`. The
Admission D0 confirmed that `NormalCallableSemanticLoanPortV1` is only a
production host/outer orchestrator. The accepted source/facts bridge design
does not add a semantic owner: `CallableSemanticSourceLedgerView` remains the
resolver source authority, while neutral SyntaxFacts and SourceMap are split
from test fixtures and promoted in
`CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-ISSUER-S0`. That source/facts slice is
closed with bounded negatives, exact resolver parity, and caller-zero audit.
The resolver seam is
`CallableSemanticSourceLedgerView::only_loop_site()` and the observer seam is
`FunctionSourceViewV1::stmt_at(membership)`; zero/multiple sites are typed
`NoSafeSlice`. The neutral SyntaxFacts and SourceMap issuers now compile in
production scope; their bounded entry uses resolver `only_loop_site()` plus
branded `stmt_at`, and the SourceFacts -> SourceMap parity receipt preserves
resolver identity. They still have no production caller or physical consumer.
Recipe/Prepared issuance remains closed; the next stop is the bounded logical
Recipe/JoinSig/After issuer implementation. A by-name adapter, fixture copying,
selector, retry, fallback, Generic G0 substitution, or legacy deletion is not
authorized by this census.

### Production admission contract (design-only)

The future production chain is fixed, but not implemented:

```text
NormalCallableSemanticLoanPortV1
  -> production source/facts bridge
  -> PreparedCallableLoopPhysicalizationV1
  -> fresh CanonicalFunctionLoweringSessionV1
  -> CanonicalSsaFunctionSessionV2 (Completion moves once)
  -> Prelude / common Loop / After / Tail
  -> finish_for_draft_seal
  -> DraftSeal prepare/commit
```

The source-facts step must promote the existing neutral
`VerifiedSourceSyntaxFactsV1` and `VerifiedCallableSingleLoopSourceMapV1`;
it must not create a new aggregate Bridge owner. It may consume only
resolver-backed source/facts/forest/projection and callable lineage products.
It must not re-walk AST, recover names from route labels, infer Recipe keys
from MIR, or remove `cfg(test)` from a fixture issuer. The SourceMap does not
own Recipe/JoinSig, ABI, Completion, physical IDs, CFG/SSA/PHI, DraftSeal,
collector, or module publication. Until S0 is accepted, the production
ingress returns typed `NoSafeSlice` before opening a function session.

The sole unpublished-function/discard owner remains
`CanonicalFunctionLoweringSessionV1::discard_unpublished`. Adapter failure is
pre-effect rejection; every later failure discards the whole unpublished
function and restores the caller once. Phi rollback is auxiliary cleanup, and
same-session repair/retry/fallback is forbidden.

## Typed rejection boundary

Before Builder effects, reject at least:

```text
foreign owner/origin/source-kind/frame/Scope/Region
missing, duplicate, foreign, or unconsumed logical key/relation
Recipe item/block owner mismatch
JoinSig port/edge mismatch
input without an exact preheader producer
prefix target/result ABI unavailable
prefix destination BindingRef mismatch
terminal value site/BindingRef mismatch
missing or unsupported exact return ABI
completion owner/site/result-kind mismatch
Loop After confused with callable Tail
unsupported logical operation, exit, or recursive depth
second Recipe/SSA/CFG/PHI/completion owner
physical ID or Builder capability present in the demand
```

These are typed `NoSafeSlice`/contract rejections. They do not fall back to a
profile-specific physicalizer or the 19-route scheduler.

After physical effects begin, any failure is terminal for that unpublished
session. It is not reclassified as a pre-effect decline.

## One recursive algebra; 19 is coverage only

The canonical full operation demand accepts the one recursive `LoopRecipeV1`
algebra. It does not contain `DirectAccum`, `GenericG0`, `LoopTrue`, `LoopCond`,
or the 19 legacy route labels as physical variants.

```text
source profiles/adapters: many bounded rows
portable Recipe algebra:  one
prepared profiles:        bounded callable/G0 compatibility products
full operation demand:    one
common physicalizer:      one
```

If the selected callable profile cannot later enter the existing family
selection envelope exactly, production selection returns `NoCandidate` and
parks it. Shape similarity must not relabel it as LoopV0 or Generic G0, and a
20th Recipe kind or second selector is forbidden.

## Finite implementation ladder

The bounded design is closed, but physical activation is intentionally split
into three mechanical commits. This is not a new semantic ladder: each row
consumes an existing owner and has one named temporal prerequisite. Do not
skip the After closure or reopen a Tail-only route.

| Order | Row | One claim | Stop line |
| ---: | --- | --- | --- |
| 0 | `RECIPE-COSEAL-I0-R0` | caller-zero common logical co-seal plus separate Prelude/Tail source contracts | closed caller-zero implementation; no ABI/Completion issuance |
| 1 | `CANONICAL-FUNCTION-FINISH-TERMINAL-R0` | migrate existing canonical V2 paths to one `finish_for_draft_seal` issuer; freeze non-V2 direct construction as compat debt | BoxShape-only; accepted profiles and MIR unchanged |
| 2 | `LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0` | fix callable input/prelude/terminal/G0/lifetime pairings in the existing prepare design | design-only; no code, Builder, physicalizer, selector, or caller |
| 3 | `LOOP-PHYSICAL-PREPARE-P0` | caller-zero common demand plus callable prepared product; exact ABI/Completion are consumed from existing issuers | no physicalizer, Builder emission, selector, or I0 claim |
| 4 | `GENERIC-G0-PHYSICAL-PREPARE-P0` | exact-move G0 adapter issues the same inner demand plus distinct G0 Tail | `NoSafeSlice` if source truth must be copied or reverified |
| 5 | `LOOP-PRELUDE-ARGUMENT-RECEIPT-P0` | resolver-issued variable-only i64 argument rows -> one move-only Prelude product | caller-zero; no Builder physicalizer or selector |
| 6 | `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` | closed test-only inner demand + `ReadyLoopEntryV1` + borrowed V2 services -> topology/After continuation | no production caller; operation MIR remains `NoSafeSlice` |
| 7 | `LOOP-RECIPE-OPERATION-EFFECT-PLAN-D0` | one neutral `LoopItemKey` + exact source-anchor effect projection before operation emission | closed preparation; no production caller |
| 8 | `CALLABLE-LOOP-AFTER-CLOSURE-P0` | complete fixed callable operation schedule, issue CFG edges, seal CFG/identity, and mint one `ReadyLoopAfterContinuationV1` | closed caller-zero; no production selection |
| 9 | `CALLABLE-LOOP-TAIL-COMPLETION-P0` | consume sealed After, read exact Tail binding, `mark_return`, and claim completion once | closed caller-zero; no selector, retry, or fallback |
| 10 | `CALLABLE-LOOP-DRAFT-SEAL-P0` | consume profile close, call only `finish_for_draft_seal`, then DraftSeal prepare/commit | closed caller-zero; production selection and legacy deletion remain closed |
| 11 | `LOOP-CALLER-ZERO-PARITY-G0-D0` | accepted design: compiler-side exact-input composite ingress, neutral S4 owner, common physicalizer, distinct G0 After/Tail | no source reconstruction, physical emission, or production selection |
| 12 | `LOOP-CALLER-ZERO-PARITY-G0-I0-R0` | exact G0 ingress -> common fifteen-row `prepare_all` with Builder effect zero | closed 2026-08-08; no physical emission, Completion/DraftSeal, selector, retry/fallback, or legacy deletion |
| 13 | `LOOP-CALLER-ZERO-PARITY-G0-I1-D0` | top-down counterexample fixes segment/resume as a common prerequisite | accepted design; direct G0 implementation superseded |
| 14 | `LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` | Builder-free Recipe-derived segment/resume layout plus exact order/coverage | **closed 2026-08-08**; no Builder effect or new accepted structural family |
| 15 | `LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` | segment-aware block allocation/operation placement; Callable parity | **current**; BoxShape-only; delete selected logical-block-only execution lookup |
| 16 | `LOOP-COMMON-RECURSIVE-AFTER-R3` | neutral recursive edge writer and After receipt; Callable profile coverage stays outside | no G0 Tail/Completion or production caller |
| 17 | `LOOP-CALLER-ZERO-PARITY-G0-I1-R0` | exact parameters, derived carrier, all fifteen operations, distinct Tail/Completion, finish/DraftSeal | caller-zero only; no G0-specific physicalizer |
| 18 | existing M8 S6A..S6G + M9 S7A..S7G | close all-19 ingress coverage and Rust/.hako portable producer parity | does not activate the physical caller |
| 19 | `LOOP-PRODUCTION-SELECTION-D0` | decide exact family admission after all required gates | human consultation stop; `NoCandidate` is valid |
| 20 | existing `M10b-I0-R0` + R1/M11/M12/R2 | one production switch, same-commit old-edge deletion, direct Ready-constructor retirement, then manifest-led sole-authority proof | no fallback; cutover must be green before retirement |

### Closed implementation receipt: `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`

```text
Change:
  add one consuming finish_for_draft_seal target to the V2 session;
  migrate existing V2 profile finish sequences through it;
  add no non-V2 ReadyFunctionDraftSealV1::new caller

Contract:
  profile-specific ledgers close into ReadyCanonicalProfileCloseV1;
  common CFG / semantics / If / identity / PhiTxn / resolved binding /
  Completion close exactly once; whole unpublished session remains the
  failure atomicity owner

Done:
  DirectAccum cannot reach DraftSeal without cfg.finish;
  V2 direct Ready constructor callers are zero; the one non-V2 compatibility
  caller is named and non-growing; profile close is move-only and completion
  body/site/target metadata is not re-inferred at finish; focused
  omission/order/receipt/identity/failure-discard/fresh-reuse tests and the
  existing canonical gates are green; loop/function-exit references and the
  owning README update in the same commit

Stop:
  any accepted-profile or MIR delta, new semantic owner, non-V2 migration,
  or same-session repair/retry returns to design
```

### Closed design correction receipt: `LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0`

The existing prepare architecture is directionally accepted but has one
bounded BoxShape correction before implementation. The correction task fixes
the callable input brand, resolved Prelude target/result capability, one-shot
Tail/Completion/ABI compatibility receipt, G0 owner/ABI pairing, and the
borrowed `ResolvedFunctionLoweringInputV1` lifetime wording. It adds no code or
Builder authority.

The correction is accepted only when these facts are explicit:

```text
callable input = non-Clone brand over exact input + current header/index
Prelude        = resolved target/header/arity/result capability, not syntax shape
terminal       = one-shot Tail/Completion/ABI relation receipt
G0             = same owner/source-type/ABI/terminal relation check
lifetime       = owned AST-free demand/receipts separate from borrowed input
```

Missing/foreign header, target, arity, result ABI, owner, tail site/binding,
Completion site/value-kind, G0 source brand, duplicate receipt, or any physical
authority is a pre-effect typed `NoSafeSlice`. The detailed task and its
acceptance matrix were the correction checklist; that row is closed and the
current execution row is the caller-zero recursive physicalizer below.

The static-call fixture and Prelude argument receipt close the remaining
positive prepared-input prerequisites without opening a production caller.

### Closed implementation receipt: `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`

```text
Change:
  add one test-only caller-zero common recursive topology boundary that
  consumes the topology-only compatibility VerifiedLoopPhysicalDemandV1
  exactly once together with the
  private, single-use ReadyLoopEntryV1 receipt and opens one Loop After
  continuation without emitting operation MIR.

Contract:
  the physicalizer sees only the AST-free demand, ReadyLoopEntryV1, and
  borrowed CanonicalSsaFunctionSessionV2 services. It does not see callable
  Tail/ABI/Completion, profile names, legacy route labels, source AST/name
  lookup, or a second Recipe/CFG/SSA/PHI owner. Late failure discards the
  unpublished fresh session; retry and same-session repair are forbidden.

Done:
  the focused canary proves recursive child/root After topology, exact entry
  coverage, owner/binding checks, parent/preheader placement, and rejection
  before block allocation. The module is cfg(test), has no production caller,
  and keeps source/check files below 800 lines. Exact MIR references, the
  owning README, and the compact current-row receipt were updated together.

Stop:
  operation emission without the accepted operation physicalizer design and
  canary task, missing logical relation,
  copied/reverified source truth, a new Recipe kind, profile-specific
  physicalizer, public topology, Return/DraftSeal/publication, selector,
  fallback, retry, or legacy deletion returns to design.
```

The topology probe may allocate only the common logical child/header/body/
step/After structure and the existing session-local continuation receipt. It
does not claim that `ReadBinding`, `WriteBinding`, constants, comparisons, or
arithmetic have been physically emitted. Those operations need an AST-free,
item-keyed source/effect projection so repeated ordinals in nested loops
cannot be guessed or matched by name.

### Operation/effect design boundary

The operation/effect relation product and both profile adapters are now
closed caller-zero cells, and cross-profile parity is closed as a diagnostic
receipt. They remain one relation product, not a new operation owner:

```text
Recipe:
  sole logical owner of LoopItemKey -> LoopOperationV1 and operand values

profile source adapter:
  sole issuer of exact source anchor / BindingRef evidence

VerifiedLoopOperationEffectProductV1:
  move-only { co-sealed Core, item-keyed source evidence ledger }
  evidence row = item + exact anchor + optional Core BindingRef view
               + checked block/loop provenance
  no copied LoopOperationV1, no ordinal lookup, no second Recipe
```

The existing `VerifiedLoopBindingEffectRelationV1` remains a separate
binding-level read/write/carrier product. The callable test producer's
item/site/operation relation is evidence for the adapter, not the common
authority. Generic G0 must retain or issue its item-keyed source evidence at
the producer boundary before structural source facts are consumed; it may
not reconstruct anchors from source preorder after Core issuance.

The operation product joins against the Core's already-sealed effect rows. If
the current Core API lacks the anchor/class view needed for that join, the
implementation may add one non-authority accessor or a consuming join helper
at the Core boundary. A second effect catalog or copied effect rows are not
allowed.

Coverage is by Recipe operation item, not by every Core effect row. Each
`LoopRecipeItemV1::Operation` has one exact source-evidence row;
`ReadBinding`/`WriteBinding` rows may additionally reference their sealed Core
effect row, while literal/compare/binary rows need no binding-effect row.
Most structural carrier rows and callable Tail/After reads remain with their
existing owners. The nested Generic G0 item 3 is the explicit exception: its
`ReadBinding` operation uses the existing child-entry
`DerivedCarrierEntry` anchor for carrier 2, and the Core effect relation must
match that anchor exactly. Item 4, C0/C1 carriers, and Generic tail reads stay
outside the operation product.

### Operation physicalizer design closeout

Decision B is accepted: full-demand preparation and one-operation emission are
different proofs. The full demand bundles the complete operation/effect product
with one neutral continuation and exposes only `prepare_all`; the private leaf
emitter consumes `PreparedLoopOperationEmissionV1` and never sees continuation
or any profile/function terminal contract.

The full semantic preflight runs before Builder mutation. After topology
allocation, `LoopPhysicalBlockReceiptV1` binds one exact logical Loop/Block to
one physical block before instruction emission. The leaf emitter may borrow
only the existing canonical CFG, BindingSSA, and PhiTxn services plus a
session-local `ReadyLoopEntryV1`. It creates no second CFG, SSA, PHI,
transaction, or retry owner. A post-emission failure poisons the unpublished
function and uses whole-session discard; local Phi rollback is diagnostic
cleanup only.

Generic item 3 remains a normal parent-body `ReadBinding`, but its source
anchor is the child-entry `DerivedCarrierEntry` for carrier 2. It is **not
admitted by ReadBinding D0**: the row is rejected as
`CarrierSeedUnavailable` and belongs to a later carrier-seed row. That later
row must assert parent-block placement and issue a child-entry carrier-seed
receipt through canonical BindingSSA; it must never relabel the operation or
infer placement from the anchor. The bounded leaf canaries are ConstI64 and
ReadBinding only; they do not constitute full Loop physicalization.

Duplicate item keys, foreign or missing anchors, wrong block/loop membership,
and repeated-ordinal ambiguity are typed `NoSafeSlice`. No operation MIR is
opened by these passive rows. Each profile product must be issued before the
P0 topology-only `into_physical_boundary` path, which intentionally drops
source anchors; P0 cannot be reused as the operation source.

`LOOP-PHYSICAL-PREPARE-P0`, the static-call fixture/profile, and
`LOOP-PRELUDE-ARGUMENT-RECEIPT-P0` are closed caller-zero prerequisites. The
cross-profile parity receipt and reviewed Decision-B closeout are closed.
Callable has seven item rows and Generic G0 has fifteen, but parity compares
neither counts nor source order. The full-demand P0, behavior-neutral module
split, canonical physical block receipt, private ConstI64 leaf-emitter canary,
bounded ReadBinding I0, and the caller-zero full callable physical canary are
closed. G0 D0/I0 are accepted/closed. R1 is now closed with Builder effect
zero; `LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` is the active next row. Segment
block cutover, recursive After, G0 physical parity,
production selection, M8/M9 coverage, and retirement remain separate gates.

### ReadBinding leaf D0 correction (2026-08-07; Decision: accepted and landed)

The broad B boundary remains accepted. Worker review closed the following
contracts, and the bounded ReadBinding I0 implementation landed with focused
tests. These constraints remain normative for the leaf:

- Project the row exactly once from a complete
  `PreparedLoopOperationProgramV1`. Its Recipe `binding`/`result`, verified
  effect row (`source_binding`, `anchor`, `role`), owner, and logical
  placement must agree. AST, name, ordinal, and ad-hoc full-demand
  re-extraction are forbidden.
- Admit only `LoopBindingEffectAnchorV1::Expr`. A `DerivedCarrierEntry`
  (including Generic G0 item 3) is rejected as typed
  `CarrierSeedUnavailable` and belongs to a later carrier-seed row.
- The raw `ValueId` from `ResolvedSsaIdentityStateV2::read_entry` must not
  become the public leaf receipt directly. A thin canonical seam must issue
  `CanonicalBindingReadReceiptV1 { owner, binding, physical_block,
  physical_value }` after canonical BindingSSA/PHI verification.
- Placement comes only from the sole `LoopPhysicalBlockReceiptV1` and the
  orchestrator's logical Loop/Block/role. `current_block` and ordinal
  inference are not authorities. All checks happen before the canonical
  read/PHI operation.
- The logical result key is alias publication only. The leaf returns one
  immutable receipt `{ owner, item, binding, result, block, value }`; the
  outer operation ledger owns publication. No second ValueId, BindingSSA map,
  PHI owner, Return, Completion, or DraftSeal is introduced.
- Identity and `PhiTxn` are borrowed through one explicit canonical read
  service bundle. The physicalizer does not become a second session or owner.
- Pre-effect rejects are typed `NoSafeSlice`. A post-read type/receipt
  mismatch is a late terminal: discard the whole unpublished function,
  retain only local Phi cleanup diagnostics, and never retry or fallback.

The required reject matrix is: operation-not-ReadBinding; missing or
mismatched source anchor/binding; Core effect/role mismatch;
`CarrierSeedUnavailable`; owner, logical, or physical placement mismatch;
missing entry binding; canonical BindingRead failure; result-type mismatch;
terminated block; and late emission failure.

This D0/I0 boundary claims no full-demand extraction API, AST reread, second
CFG/SSA/PHI/catalog owner, derived/G0 carrier bridge, other operation kinds,
return/seal/module publication, selector, retry/fallback, legacy retirement,
or performance result. The bounded implementation is landed. The current
authorized row is `CALLABLE-LOOP-AFTER-CLOSURE-P0`; Tail-only lowering is a
NoSafeSlice until its sealed After receipt exists. Each subsequent Tail and
DraftSeal slice must update reference documentation in the same commit as
code and focused tests.

#### ReadBinding source/effect mapping matrix

The following table is the complete D0 mapping. The full prepared program is
the only projection input; a one-row test fixture is allowed because the
complete program itself contains one ReadBinding row, not because a single
operation is extracted from a demand.

| Recipe operation | Evidence item | Core effect / anchor | D0 admission | Canonical read | Result publication owner |
| --- | --- | --- | --- | --- | --- |
| `ReadBinding { binding: LoopBindingKeyV1, result: LoopValueKeyV1 }` | same `LoopItemKeyV1` | `SourceRead { ordinal }`, `source_binding: BindingRefV1`, `Expr(OwnedExprSiteV1)` | admit only when all keys, owner, block, and role match | claim exact `SourceExprSiteV1` for `BindingRefV1`, then issue `CanonicalBindingReadReceiptV1` | outer operation ledger maps `LoopValueKeyV1` to the immutable leaf receipt |
| same operation | same item | `DerivedCarrierEntry` anchor | reject `CarrierSeedUnavailable`; no claim/read | none | none; later carrier-seed row owns it |
| any non-`ReadBinding` operation | same item | any effect row | reject `OperationNotReadBinding` | none | none |

The canonical receipt has exact field types and one issuer:

```text
CanonicalBindingReadReceiptV1 {
  owner: FunctionOwnerIdV1,
  binding: BindingRefV1,
  physical_block: BasicBlockId,
  physical_value: ValueId,
}
```

Only `CanonicalSsaFunctionSessionV2`'s borrowed read service may issue it.
The order is fixed: validate the prepared row and physical placement; claim
the exact `SourceExprSiteV1` with `claim_variable_use_binding`; call the
canonical `read_entry_receipt`; validate owner, block, and physical type;
then return the receipt. A raw `ValueId` from `read_entry` is never a leaf
receipt and cannot be fabricated or rewrapped by the physicalizer.

Before the loop block is sealed, canonical BindingSSA may return a provisional
PHI with `MirType::Unknown`. The verified Recipe class is the only permitted
publication evidence in that state: `Unknown -> exact class MirType` is
published once by the private operation-type owner, while a concrete conflict
or missing type rejects as `ResultTypeMismatch`. Block/identity sealing then
revalidates the now-concrete PHI inputs; this is not type inference or a
fallback route.

The leaf receipt uses distinct logical and physical names:

```text
ReadBindingEmissionReceiptV1 {
  owner: FunctionOwnerIdV1,
  item: LoopItemKeyV1,
  binding: BindingRefV1,
  result: LoopValueKeyV1,
  logical_block: LoopBlockKeyV1,
  physical_block: BasicBlockId,
  physical_value: ValueId,
}
```

`result` is an alias key only. The Recipe's `LoopValueClassV1` for `result`
and the binding/effect class are the logical type authority; the canonical
BindingSSA type fact is the physical observation and must match the class-to-
`MirType` mapping before the receipt is returned. The outer operation ledger,
not the leaf, publishes the result mapping. No second SSA/PHI/value map is
created.

#### Entry, placement, service, and failure contracts

`ReadyLoopEntryV1` is a **preheader seed receipt**, not a complete map of all
live bindings. The private ReadBinding projection carries an explicit
`entry_requirement: LoopReadEntryRequirementV1` with exactly two cases:
`PreheaderSeed` or `CanonicalLive`. The full-program orchestrator issues this
field from the existing Recipe input set and source-binding relations, then
checks `PreheaderSeed` against `ReadyLoopEntryV1`; the leaf never infers the
case. Body/step bindings use canonical SSA availability at their exact
physical block; absence from the preheader rows is not itself an error. A
required preheader seed missing from `ReadyLoopEntryV1` is the typed
pre-effect reject `EntryBindingMissing`.

The orchestrator supplies `expected_role: LoopPhysicalBlockRoleV1` together
with `LoopBlockKeyV1`; the sole placement authority is
`LoopPhysicalBlockReceiptV1`. The leaf never derives a role from
`current_block`, an ordinal, or source-anchor shape. The logical block and
expected role must resolve to the same `BasicBlockId` before claim/read.

The canonical borrowed bundle is the only physicalizer service boundary:

```text
CanonicalBindingReadServicesV1<'a> {
  builder: &'a mut MirBuilder,
  identity: &'a mut ResolvedSsaIdentityStateV2,
  phis: &'a mut PhiTxn,
}
```

It is created by the fresh canonical function session, borrowed for one
read, and never stores a second CFG/SSA/PHI owner. CFG placement is borrowed
separately from the sole `LoopPhysicalBlockReceiptV1`; simultaneous borrows
are sequenced so the receipt is fully validated before the canonical read.
There is no new type-fact owner or `TypeFactContext`: physical type
validation reads the existing `TypeContext` at
`MirBuilder::function_state.type_ctx`, and any publish/idempotence decision
uses the existing `TypeFactDecisionV1`/
`PreparedTypeFactPublicationV1` seam. The service bundle therefore borrows
only the canonical Builder/identity/Phi owners named above.

Phase ownership is fixed as follows:

| Phase | Allowed failure | Owner/action |
| --- | --- | --- |
| prepared-row/source/effect/entry/placement validation before claim | typed `NoSafeSlice` | no Builder/claim/PHI effect |
| canonical validation before the atomic claim/read service starts | typed `NoSafeSlice` | no claim/PHI effect |
| claim succeeds or canonical read starts, then any read/type/receipt error | terminal `Freeze` | whole unpublished function discard; caller restore once; PhiTxn abort is diagnostic only |
| post-read type/receipt/result mismatch or injected late failure | terminal `Freeze` | whole unpublished function discard; caller restore once; no retry/fallback |

This phase split is part of D0 acceptance and must be represented by focused
negative tests in the later implementation row.

## Implementation and documentation obligation

Every implementation row above must update its exact live references in the
same commit after code and focused tests land:

- `docs/reference/mir/loop-recipe-contract.md` for the landed co-seal/demand/
  physical boundary and sole-owner claims;
- `docs/reference/mir/generic-loop-stage-matrix.md` for caller-zero,
  canary, activation, and retirement status;
- `docs/reference/language/function-exit-and-entry-result.md` when the typed
  function-finish/Completion handoff lands;
- `src/mir/loop_recipe_contract/README.md` and the owning canonical lowering
  README (`src/mir/builder/resolved_lowering/README.md`) when their code
  contract changes;
- `docs/reference/mir/phi_policy.md` and `phi_invariants.md` only when a
  physical PHI contract actually changes;
- `CURRENT_STATE.toml` and the active rolling workstream for the next exact
  row and compact closeout;
- `docs/tools/check-scripts-index.md` only if a reusable public guard entry is
  added.

References must describe only landed behavior. This design SSOT may name the
accepted target now, but the reference pages must not claim physical,
production, backend, or retirement capability before the corresponding
implementation receipt exists.

## Current execution boundary

The architecture, `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, bounded
`RECIPE-COSEAL-I0-R0`, callable static-prefix prepare, and Prelude argument
receipt are closed under the typed-receipt and no-reinference contract above.
The topology/After canary `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` is closed.
The operation/effect plan, passive product, Callable adapter, Generic G0
15-row anchor ledger, cross-profile parity receipt, worker-reviewed
physicalizer Decision-B closeout, and Builder-free full-demand P0 are closed.
The `LOOP-RECIPE-OPERATION-EMITTER-CONST-S0` boundary is now closed as a
private prepared ConstI64 leaf canary. It proves exact physical placement,
canonical Const/type-fact emission, typed pre-emission rejects, and
whole-session discard/fresh-session repeat. Full operation emission,
operation production activation, callable physical completion, production
selection, retry/fallback retirement, and legacy deletion remain closed. The
logical callable issuer S0 is closed without a production caller. R1 is now
closed by the receipt below; the next row is the bounded R2 segment/block
cutover. No single-item extraction API may be added to the full demand.

### Callable physical-canary preparation slice (2026-08-07)

The current preparation slice is mechanically green without claiming the full
callable physicalizer. The Prepared callable product has one private test
handoff that moves `input`, complete operation demand, Prelude, Tail, terminal
compatibility, and Completion exactly once. The full operation contract also
projects every WriteBinding row with its exact Recipe item, source
binding/site, class, and logical placement.

Private leaf bridges cover `ConstI64`, `BinaryI64`, and `CompareI64` through
the existing Builder/type emitters. Their schedule-local value map is only a
temporary `LoopValueKey -> ValueId` transport; it is not a second SSA or PHI
owner. A focused test proves the Const -> Binary -> Compare chain. A bounded
row-level dispatcher and full Recipe-order Builder-free prepare now join
Read/Const/Compare/Binary/Write leaf services with an opaque typed value
ledger. The physical operation boundary now issues one exact
logical-to-physical target receipt per row, validates all target blocks before
the first leaf effect, and separates target/pre-claim physical failure from
semantic preflight. The caller-zero full physical canary is now closed:
the exact resolved-module input/ledger enters S2 once, then reaches Prelude,
topology, all five operation families, sealed After, Tail/Completion,
`finish_for_draft_seal`, and DraftSeal prepare/commit. Its late-failure test
discards the whole unpublished function and reruns the same semantic fixture
in a fresh session. Production selection, Generic G0 parity, retry/fallback
retirement, module publication, and legacy deletion stay closed.

### Callable full physical canary closeout (2026-08-08)

`CALLABLE-LOOP-PHYSICAL-CANARY-P0` is a caller-zero-only integration receipt.
The test-only source bridge borrows the exact existing resolver ledger from
`ResolvedFunctionLoweringInputV1`; it does not resolve a second owner or clone
the source AST. The complete seven-row Recipe schedule is consumed once and
the existing owners remain sole authorities for CFG/SSA/PHI, completion,
DraftSeal, and unpublished-function discard. The focused positive and late
duplicate/discard/fresh-reuse tests are green. G0 D0 is accepted; the next
authorized row is the Builder-free
`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` exact-input composite gate.

### Generic G0 exact-ingress I0 closeout (2026-08-08)

`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` now has a compiler-side `cfg(test)` ingress
at `src/mir/compiler/generic_g0_physical_prepare.rs`. It pairs the exact
resolver-issued `ResolvedFunctionLoweringInputV1` with the existing neutral
S4 product, validates source/owner/frame/forest/entry/tail relations, splits
`VerifiedGenericAfterEffectG0` once into the neutral continuation and the
distinct `VerifiedGenericG0TailCapabilityV1`, then issues the common demand
and `prepare_all` for all fifteen G0 Recipe items. The schedule is checked by
Recipe membership rather than Callable/G0 count or evidence order. Focused
positive, missing-input, foreign-input, and tail-separation tests are green;
existing demand/producer tests retain duplicate/missing-evidence coverage.
This remains Builder/MIR/physicalizer/selector/Retry/publication-free; later
negative expansion must use typed sealed-product rejection, not tampering or
reconstruction.

### Recursive segment plan R1 closeout (2026-08-08)

`LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` is closed as a Builder-free derived
product. `VerifiedLoopOperationPhysicalDemandV1::prepare_all` now traverses
the verified recursive Recipe preorder instead of flattening logical blocks.
`PreparedLoopPhysicalLayoutV1` consumes the complete prepared program and
records only mechanically derived segments, operation placement, and nested
After-to-parent-resume targets. It creates no `ValueId`, `BasicBlockId`, CFG,
SSA, PHI, function session, selector, retry, fallback, or legacy authority.

The exact fixtures are green:

```text
Callable: seven operation rows in Recipe preorder
Generic G0: [0,1,2,3,5,6,7,8,9,10,11,12,13,14,15]
Generic G0 segments: root B0, root B1-pre, child B2, child B3, root B1-resume
coverage: 16 items / 15 operations / 5 segments
```

R2 may bind these private segments to canonical physical blocks. Until its
closeout, physical block allocation/placement, recursive After emission, G0
physical parity, production selection, retry/fallback retirement, and legacy
deletion remain closed. The R2 task is
`investigations/loop-common-segment-block-cutover-r2-task-2026-08-08.md`.
