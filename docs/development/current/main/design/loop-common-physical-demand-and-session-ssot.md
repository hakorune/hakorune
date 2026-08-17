---
Status: SSOT
Date: 2026-08-16
Decision: accepted after external and independent worker review — `LOOP-COMMON-PHYSICAL-DEMAND-AND-SESSION0-D0-r2`
Activation: `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, callable static-prefix
P0, bounded `LOOP-PHYSICAL-PREPARE-P0`, common-boundary design stop,
caller-zero `LOOP-PRELUDE-ARGUMENT-RECEIPT-P0`, passive operation/effect S0,
Callable/G0 adapters, and cross-profile parity are closed. Decision B now
separates full-demand preflight from leaf emission; the Builder-free
`LOOP-RECIPE-OPERATION-PHYSICAL-DEMAND-P0` and the behavior-neutral
physicalizer module split, physical block receipt, private ConstI64
leaf-emitter canary, bounded ReadBinding I0, callable full physical P0, and
G0 exact-ingress I0 are closed. Top-down review revised the next boundary:
the private Builder-free segment/resume layout and bounded G0 fresh-session
canary are closed. A later audit found that the landed layout still derives
logical transfers from Recipe instead of consuming JoinSig authority; the
post-M9 pre-cutover R0 rows below own that correction. Operation production
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
as Builder-free exact ingress plus fifteen-row `prepare_all`. R1 is closed as
a Builder-free derived layout, R2 as a Callable adapter, and R3-I0 as the
selected Callable exact-segment/neutral-After canary. Per-transfer Predicate
value receipts, the profile-neutral `DerivedCarrierEntry` operation, and the
bounded G0 I1 canary are also closed. A later top-down audit found that current
caller-zero products can still be re-paired and current Layout code still
derives logical transfers without JoinSig authority. The accepted target is
unchanged, but semantic-program and transfer-authority R0 rows must close
after M8/M9 and before production selection. No named production caller
switch is open.
The 2026-08-15 S6C audit adds no second physicalizer: it names one missing
common-V2 pre-session contract that must close exact callable ABI and the
complete V2 envelope before any TextEq leaf or canonical session is admitted.
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

## Current Capsule

- **Current decision:** every admitted Loop profile reaches one complete
  semantic program, JoinSig-bound layout, and canonical SSA session; V1 and V2
  are exact projections of that one responsibility graph.
- **Current implementation status:** S6C source/site, ExactText formal,
  result/header, installed child, package physical-signature map, and
  caller-zero residence/backend transport substrate are closed. The common
  V2 operation/control/coverage issuers and installed Port HRTB are landed
  as caller-zero source products. Existing resolver Loop membership can issue
  the outer-If residual, and the installed S6C child can lend the actual
  Completion without cloning. The resolver-owned BlockExpr expectation is
  now batch-owned and reaches the selected/package HRTB as a borrow. The
  callback-scoped common admission is landed; the detached physical skeleton,
  slot-only ExactText adoption canary, and consuming physical-entry/session
  seam are also landed. The session-stamp retention I0 now moves the existing
  mechanical cohort witness exactly once into the canonical session and lends
  only a scoped borrow. The V2-native physical-ID-free layout/placement
  BoxShape and its caller-zero transport I0 are now landed. The Length-result
  canary I0, direct Length Call/result I0, and exclusive session-scoped Length
  receipt lifetime I0 are also landed. The source-only initial-index seed
  relation transport, its one-entry Const/exact-declaration materializer I0,
  and the receipt-owned Bool/Compare materializer I0 are now landed; the
  all-family source-parent/co-seal R0, Generic G0 source-parent BoxShape, and
  same-cohort source-view BoxShape are accepted; the Generic source-parent I0
  replaces the test-only ingress with one production issuer.  The resolver
  body-shape product is now transported from the same source-unit resolution
  into the root lowering input and Generic source parent with owner/body-root
  checks.  The private Generic no-external-effect receipt and same-cohort
  result-ABI transport I0 and direct canonical Completion transport I0 are now
  landed before demand/product consumption.  Generic storage/lane and
  physical-effect source projections are landed caller-zero products; they
  cannot open an EffectMask or any physical/session effect.
  A-prime lifecycle activation remains parked until its boundary owns
  `PreparedFunctionExitSetV1`.  The selected Dynamic physical-input
  authority is landed; the post-Dynamic unification rows below remain a
  design-stop closeout until their direct negative fixtures and old-edge
  caller census are recorded. The If continuation source/Recipe/Join binding,
  common aggregation, canonical-session consumer transport, Return-read
  receipt, and one-shot shared-segment scope I0 are landed. Branch emission
  remains closed because outer-loop Bool V5 cannot feed the inner TextEq If
  at V10. The premise-reset audit corrected TextEq from a source StringEquals
  call/Trap boundary to a portable non-faulting operation whose V9/V1
  residences must be co-sealed before V10 materialization.
- **Next ordered task:**
  `LOOP-PHYSICAL-S6C-TEXTEQ-OPERAND-ISSUER-D0` is the next design stop. It
  must name the S6C-only canonical issuer for the Body V6 read, V7 constant,
  and V8 add before a Substring V9 result or any V10 capability is opened. No
  Dynamic callout or C ABI is implied.
- **Production stop line:** no leaf emission or session admission may infer
  ABI, control, transfer, or source identity from Recipe/MIR, coerce V2 to V1,
  or select a second physicalizer.
- **Retirement finish line:** all admitted profiles use one common physical
  owner and old topology, route-local schedulers, direct transfer inference,
  retry, and fallback have zero callers.


## Historical boundary

This file remains the current-owner authority and the stable path used by
CURRENT_STATE.toml, design/INDEX.md, and existing investigations. The former
append-only D0/I0/canary prose is now indexed in the [historical ledger](archive/loop-common-physical-demand-and-session-history-2026-08-18.md).
It is historical evidence only; it cannot select a task, mint a receipt, or
change the blocker.

| Kept live here | Archived from the live surface |
| --- | --- |
| Current Capsule, durable physical/session contract, stop lines, and the active 2026-08-18 chain | 2026-08-07–08-17 landed/closed detail, superseded design rows, and pre-Return-read canary prose |
| Current blocker and current_execution_row token | Exact historical prose, recoverable from the linked ledger and Git history |

The linked ledger is intentionally compact. Do not restore the old append-only
body; add only a compact current row and put exact evidence in code/tests,
reference pages, or the archive ledger.
## Decision

Close the post-Recipe boundary before physical implementation begins.

```text
resolver / source map
  -> versioned VerifiedLoopSemanticProgramV1 | V2
  -> versioned complete operation/source/control demand
  -> PreparedLoopOperationProgramV1 | target V2
  -> one thin prepared execution product
       PreparedCallableLoopPhysicalizationV1
       OR PreparedGenericG0LoopPhysicalizationV1
       OR scoped target PreparedLoopV2PreSessionEnvelopeV1<'loan>
  -> target LoopV2CanonicalSessionAdmissionV1
       fan-in of the neutral envelope, separately admitted route policy,
       and callable signature/residence demands
  -> one fresh unpublished function session
       completion moves here exactly once
  -> outer callable lowerer + one common recursive Loop physicalizer
  -> open After continuation
  -> existing completion / DraftSeal
  -> one unpublished function draft
```

The target canonical full-operation input is a private, move-only, AST-free,
and physical-ID-free semantic-program receipt which feeds
`VerifiedLoopOperationPhysicalDemandV1`. It co-seals the Core-bearing
operation/effect product with one common continuation capability issued by
that Core's own JoinSig; the two are never independently re-paired at the
physical boundary. Each thin
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


### Common V2 pre-session (compact contract)

The common-V2 boundary is one non-splittable installed-package/Port loan. Its
private parent lends same-brand identity, one Completion owner, callable
physical-signature state, retained S6C Facts/Recipe/Join input, and neutral
operation/control/coverage projections only within one callback.

The operation adapter covers every admitted Operation placement. The JoinSig
control adapter covers If, Exit, and transfers. A passive union receipt proves
disjoint complete coverage; the S6C adapter may then assert its profile-specific
13 + 1 + 1 placement fact. The parent and its views are move-only and
callback-scoped, and no view may be stored or recombined.

This is a source-level pre-session product, not a MIR/JSON carrier or a
physicalizer. It cannot issue ValueId, residence, CFG/PHI, lifecycle, Text
route, fallback, retry, production, or publication effects. V1/V2 coercion,
Recipe/MIR rescans, Dynamic-cursor reuse, and a second S6C physicalizer remain
forbidden. The detailed 2026-08-15/16 transport and issuer receipts are
historical; the current contract is the compact boundary above.
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

The current caller-zero compatibility shape is:

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
function session. The installed `VerifiedFunctionCompletionV1` remains owned
by its cohort; the session consumes one owned
`ResolvedFunctionCompletionConsumptionV1` issued from that scoped borrow. The
semantic Completion is not cloned or moved into a sibling boundary. The outer
lowerer retains only Prelude/Tail/ABI evidence, transfers the full operation
demand exactly once to the future full physicalizer, and later claims the
exact return operand through `session.completion`. Lowering by `&demand`,
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
Recipe/Prepared issuance and the logical Recipe/JoinSig/After issuer rows are
historical closeout evidence; the live next stop is selected only by
CURRENT_STATE.toml and the active execution sections below. A by-name adapter,
fixture copying, selector, retry, fallback, Generic G0 substitution, or legacy
deletion is not authorized by this census.

### Production admission contract (design-only)

The future production chain is fixed, but not implemented:

```text
NormalCallableSemanticLoanPortV1
  -> production source/facts bridge
  -> PreparedCallableLoopPhysicalizationV1
  -> fresh CanonicalFunctionLoweringSessionV1
  -> CanonicalSsaFunctionSessionV2
       (one physical Completion consumer issued from one scoped semantic borrow)
  -> Prelude / common Loop / After / Tail
  -> finish_for_draft_seal
  -> DraftSeal prepare/commit
```

Before production activation, `LOOP-SEMANTIC-PROGRAM-COSEAL-R0` replaces the
three separately supplied semantic fields with one consumed
`VerifiedLoopSemanticProgramV1`. The demand may retain a private lookup index,
but it cannot expose `first`/`select`/`filter`, split the semantic program, or
reconstruct context/continuation from matching keys. The old multi-argument
issuer and any caller that manufactures context or continuation from parts are
deleted in the same Refactor Series.

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

The canonical full operation demand accepts one recursive Loop Recipe algebra
through an exact V1 or V2 projection. V2 adds typed operation/value vocabulary;
it is not converted into V1 and does not create another physicalizer. The
algebra does not contain `DirectAccum`, `GenericG0`, `LoopTrue`, `LoopCond`, or
the 19 legacy route labels as physical variants.

```text
source profiles/adapters: many bounded rows
portable Recipe algebra:  one, with exact V1/V2 projections
prepared profiles:        bounded callable/G0 compatibility products
full operation demand:    one
common physicalizer:      one
```

If the selected callable profile cannot later enter the existing family
selection envelope exactly, production selection returns `NoCandidate` and
parks it. Shape similarity must not relabel it as LoopV0 or Generic G0, and a
20th Recipe kind or second selector is forbidden.


## Implementation ladder (compact)

The ladder is a temporal ordering over existing owners, not a semantic
classifier. Completed caller-zero prerequisites stay closed; no row below
opens a production switch by itself.

1. Source/Facts/Core/Recipe/JoinSig co-seal and family-specific prepared demand.
2. Canonical session admission, entry/signature/lane transport, and one
   callback-scoped unpublished session.
3. V2-native layout, segment/After allocation, condition-result ownership, and
   source-backed If/Return relation transport.
4. The active branch/Return consumer must finish its source-backed physical
   receipt and one-sided terminal co-seal before any branch/CFG effect.
5. TextEq ordering is Substring V9 residence plus ExactText V1 residence,
   portable non-faulting TextEq V10 materialization, then If/Return. A checked
   C transport, if selected later, is subordinate to that physicalizer.
6. Production selection, fallback/retry retirement, publication, and legacy
   deletion remain closed until the named authority and caller census are
   complete.

Historical ladder rows and closed receipts are indexed in the archive ledger.
The active row is selected only from CURRENT_STATE.toml and the live sections
below.
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

For the new pre-cutover rows, the co-seal and transfer-authority implementation
commits each update the reference with their exact caller-zero status; each
Always/If/Exit BoxCount commit updates the supported structural matrix; M10b
then updates the same reference once more to record the real production caller
and removed legacy authorities. A design-only commit does not pre-announce any
of those capabilities in `docs/reference/**`.


## Current execution boundary

Historical Callable/G0/Common-V2 canary closeouts through 2026-08-08 are
archived. The live execution chain below starts with the 2026-08-18
Return-source relation and continues through the current TextEq design stop.
Keep the exact current_execution_row and blocker tokens in this section; a
green local test never advances them.
### Generic G0 I1 caller-zero receipt (2026-08-08; Decision: accepted)

`LOOP-CALLER-ZERO-PARITY-G0-I1-R0` is closed as a profile wrapper around the
same common physical services. The exact resolver-issued G0 ingress moves once
into the full common operation program and the separate G0 Tail. The canary
opens a fresh unpublished function session, publishes the resolver-declared
receiver and two parameters through canonical identity, allocates five R1
segments plus root After, and dispatches the fifteen prepared rows exactly
once. The structural nested Loop item remains a control/layout row rather
than a fabricated operation.

The carrier row uses the profile-neutral `CarrierSeed` emitter and canonical
`read_entry_receipt`; an unsealed PHI value is typed only through the existing
`ensure_provisional_value_class` contract. Each Predicate transfer consumes
its own completed Bool receipt, so root and child conditions have distinct
physical values and source segments. The G0 `L0.After/b1` Tail read is
canonical, exact I64 Completion is claimed once, and
`finish_for_draft_seal`/DraftSeal reaches one unpublished completed draft.

The late duplicate fixture fails after earlier emission, discards the whole
unpublished session, and a fresh session reproduces the same semantic receipt.
No G0-specific CFG/SSA/PHI owner, production selector, caller switch,
retry/fallback, collector publication, backend/performance claim, M8/M9
coverage, or M10b/M11/M12 retirement is opened. The behavior-preserving
`LOOP-INPUT-SOURCE-RELATION-SET-R0` is now closed: callable consumes the common
exact-coverage initialized-local input set and Generic parameter inputs remain
separate. S6A's caller-zero Facts/producer and typed Main C/D/U/R ingress are
landed; its exact identity/source-coherence negative closeout remains current.
After M8/M9, the semantic-program and transfer-authority rows above are the
mandatory production-selection prerequisites. Current source task order is
owned by the Loop pipeline SSOT and `CURRENT_STATE.toml`.

### Return source-to-Recipe/Join binding I0 closeout (2026-08-18; Decision: accepted)

`LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-I0` is landed as
one caller-zero transport-only BoxShape. The resolver-owned S6C Exit/Tail
co-seal now retains the exact nested-If region, Return region, index
`BindingRef`, Return site, and Return value. The sole S6C Recipe producer issues
one non-Clone relation that binds those source facts to the fixed If/then
blocks, `ReadBinding` Return item/result, logical Exit key, and the JoinSig
Return/FunctionExit arm plus matching Body-to-FunctionExit summary. The
relation is lent through the existing product, logical JOINIR, and logical
output/prephysical façades; no second semantic issuer or Recipe key owner is
introduced.

Focused `s6c_scan_with_init` tests are green (9/9), including the positive
source/Recipe/Join binding and existing swapped-call, swapped-argument, and
domain-drift negatives. `current_state_pointer_guard.sh`,
`loop_physical_transfer_authority_guard.sh`, formatting, and diff checks are
green. No block/edge/terminator, Return emission, CFG/SSA/PHI, session,
production, fallback, retry, or publication effect is opened. The next design
stop is `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COMMON-AGGREGATE-D0`:
the common pre-session envelope must first be named as the sole consumer and
rollback owner before this relation can affect physical demand.

### Return source common aggregation D0 (2026-08-18; Decision: accepted)

```text
Decision:
  accept one transport-only BoxShape: the existing common pre-session issuer
  borrows and retains the already-issued Return source-to-Recipe/Join relation
  in its envelope; it does not reissue or reinterpret semantic meaning.
Source authority + canonical issuer:
  `VerifiedS6CReturnSourceRecipeBindingV1`, issued by the S6C Recipe producer,
  remains the sole source-to-key authority. `issue_s6c_common_v2_pre_session_v1`
  is the sole common borrower/aggregator for the same ingress cohort.
Non-authority:
  prephysical count/cleanup, fixed ordinals, layout, item-set equality, and
  physical/session code cannot create or re-pair this relation.
Fail-fast boundary:
  missing/foreign relation or owner mismatch rejects before returning the
  common envelope; the outer unpublished canonical session remains the only
  rollback owner for later effects.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COMMON-AGGREGATE-I0`: split
  embedded common tests to preserve the 760/800 source boundary, then retain
  the borrowed relation field/accessor and add focused transport negatives.
Non-claims:
  no new semantic receipt, Recipe key, branch/edge/Return/PHI, session
  mutation, production switch, fallback, retry, or publication.
```

The common envelope is a mechanical aggregate of existing sibling receipts,
not a new authority. The test-module split is a behavior-preserving source
shape cleanup required before adding the narrow field; it does not change the
common operation/control contract. The relation must remain callback-scoped
and non-Clone, and it must never be reconstructed from Recipe ordinals or
layout placement.

### Return source common aggregation I0 closeout (2026-08-18; Decision: accepted)

`LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COMMON-AGGREGATE-I0` is landed
as the bounded transport slice. The common pre-session envelope now borrows
the existing non-Clone `VerifiedS6CReturnSourceRecipeBindingV1` from the same
ingress cohort and exposes it without reissuing, cloning, or deriving a
Recipe/Join key from layout. Owner mismatch still fails before the envelope is
returned; the outer unpublished canonical session remains the rollback owner.

The common issuer tests are split into a sibling test module so the production
issuer stays below the 760/800-line source boundary. Focused common tests are
green (25/25) and focused S6C tests remain green (9/9); format, diff, pointer,
and transfer-authority guards are green. No Return/edge/terminator/PHI,
session mutation, production switch, fallback, retry, or publication effect
is opened. The next design stop is
`LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COMMON-CONSUMER-D0`: name the
single physical-demand consumer and its fail-fast/rollback boundary before
the retained relation can drive any physical effect.

### Return source common consumer D0 — worker premise gate (2026-08-18)

This is not Fast path: the retained semantic relation has several existing
session/preflight entry points, and the sole physical-demand consumer plus its
RejectBeforeEffect/rollback boundary is not yet named from source authority.

### Return source common consumer D0 (2026-08-18; Decision: accepted)

```text
Decision:
  accept one transport-only BoxShape: the existing consuming
  `with_common_v2_physical_entry_session` is the sole physical-demand and
  outer rollback owner; its callback-scoped `CommonV2CanonicalSessionRefV1`
  is the only consumer view for the retained relation.
Source authority + canonical issuer:
  `VerifiedS6CReturnSourceRecipeBindingV1` remains issued by the S6C Recipe
  producer; common admission aggregates it, and the canonical session consumes
  that same envelope cohort without reacquiring a sibling loan.
Non-authority:
  `CommonV2CanonicalSessionRefV1` may not reissue source meaning; Recipe
  ordinals, owner equality, Layout, segment rows, MIR IDs, or a second
  physicalizer cannot create or re-pair the Return relation.
Fail-fast boundary:
  admission/owner/stamp/entry adoption rejects before the session callback;
  callback errors call the existing outer `discard_unpublished` exactly once.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COMMON-CONSUMER-I0`: expose
  the already-issued relation through the canonical session callback and add
  owner/late-discard evidence, without emitting Return/edge/PHI.
Non-claims:
  no new semantic receipt, physical Return/terminator, CFG/SSA/PHI,
  publication, production switch, fallback, retry, or legacy retirement.
```

The Decision closes only the named-consumer question; it does not authorize a
Return emitter. The outer session seam remains the single failure terminal,
and the next I0 is a callback-scoped transport check over the existing
relation.

### Return source common consumer I0 closeout (2026-08-18; Decision: accepted)

`LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COMMON-CONSUMER-I0` is landed as
the smallest callback-scoped transport slice. The canonical session now lends
the existing `VerifiedS6CReturnSourceRecipeBindingV1` from its retained common
envelope; no relation is cloned, reissued, or rebuilt from Layout/Recipe
ordinals. `with_common_v2_physical_entry_session` remains the sole unpublished
function-session and rollback owner, and callback failure still discards the
unpublished shell once.

The focused admission/consumer path is green (common 25/25), the physical
consumer placement/late-discard pair is green (2/2), and S6C source/Recipe/Join
coverage remains green (9/9). `cargo fmt --all -- --check`, `git diff --check`,
`current_state_pointer_guard.sh`, and
`loop_physical_transfer_authority_guard.sh` are green. The next design stop
is the existing `LOOP-PHYSICAL-IF-CONTINUATION-BRANCH-EMISSION-D0`; item-to-
split and one-sided Return/FunctionExit terminator authority remain unnamed.
No Return/edge/PHI emission, publication, production switch, fallback, retry,
or legacy retirement is claimed.

### Branch emission D0 refresh — worker premise gate (2026-08-18)

This remains a design stop rather than Fast path: the new source-to-Recipe/Join
relation proves semantic Return provenance, but it still does not name the
physical split target, Return value receipt, or one-sided terminator owner.

### Branch emission D0 premise audit — circuit-breaker check (2026-08-18)

The same responsibility has now produced the placement D0, split/terminal D0,
Return source co-seal D0, and branch-emission refresh D0. Apply the required
premise audit before opening another suffix:

- **Semantic unit and window:** one source-backed If continuation from the
  resolver-owned Return/If regions through the Recipe Exit and JoinSig arm,
  ending at the first physical branch/Return mutation. The source-to-
  Recipe/Join relation and canonical-session transport now cover provenance
  and consumption, but not physical realization.
- **Classifier/partition arms:** the accepted logical shapes are the
  `Exit { exit_item, Return, FunctionExit }` arm and the one-sided
  `Fallthrough(NextItem)` arm. No layout ordinal, two-normal-arm merge, or
  inferred “else” arm may fill a missing relation; every unsupported or
  ambiguous arm remains a rejection.
- **Transferred/opaque subtrees:** `IfContinuationPhysicalTargetRefV1`, the
  layout segment/split receipt, and the borrowed common-session view are
  transport/placement evidence only. They do not hide an item-specific
  physical split, Return value, or terminal owner.
- **Structural requirement:** an emitter needs a source-issued item-to-split
  relation, a source-issued Return value receipt, and one-sided
  `FunctionExit`/terminator ownership before `emit_branch` or `emit_return`.
  `CanonicalCfgSessionV1` can write those physical objects mechanically but
  cannot issue their meaning.
- **Counterexample:** owner/stamp parity and target placement can all pass
  while the Return value or FunctionExit block is absent. Calling the CFG
  writer from that state would pair separate authorities by Layout/ordinal and
  could terminate the wrong block; therefore the current green placement and
  transport tests do not authorize physical emission.

**Audit result:** `NoSafeSlice::IfContinuationBranchEmissionAuthorityUnsealed`
remains the correct development state. The next bounded slice is still a
design-only source issuer/consumer decision; do not add a new semantic
`Verified*`/`Prepared*` receipt, branch/Return/edge/PHI effect, fallback,
retry, publication, or production switch until that issuer and fail-fast
boundary are named.

### Branch emission D0 — physical Return-read receipt audit (2026-08-18)

The fresh worker audit narrows the remaining boundary. The existing
`issue_s6c_return_source_recipe_binding_v1` is now the canonical logical issuer
for item 9 `ReadBinding`, its Recipe value, item 10 `Exit`, the then block, and
the JoinSig `Return -> FunctionExit` arm. It does not issue a physical
`ValueId`, a physical then/continuation split pair, or a terminal block.

No existing common-V2 physical receipt owner can be reused:

- `S6CPrephysicalIngressRefV2::operation_source` is a source-site anchor only;
  it is not a physical result or block receipt.
- `CanonicalSsaFunctionSessionV2::issue_physical_value_id` only allocates a
  raw value identity, and `CanonicalCfgSessionV1::emit_return` only writes a
  preselected physical terminator. Neither issues item 9/10 meaning.
- The old V1 `ReadBindingEmissionReceiptV1`/`emit_prepared_read_binding_v1`
  owner is outside common V2. A V2-to-V1 adapter would create a second
  physical authority and is rejected.

**Decision:** keep physical branch/Return emission at `NoSafeSlice` and name
the next bounded design row
`LOOP-PHYSICAL-IF-CONTINUATION-RETURN-READ-PHYSICAL-RECEIPT-D0`. That row must
identify one source-backed issuer for the item 9 binding/result, its exact
physical block, the continuation split target, and the item 10
`FunctionExit` terminal before any `ValueId`, `emit_branch`, `emit_return`,
edge, PHI, publication, fallback, retry, or production effect is allowed.

The fail-fast sentence is:

```text
source Return/If co-seal -> existing Recipe/Join binding -> one named
physical Return-read receipt issuer -> reject missing/foreign/duplicate/
ambiguous item/block/value/terminal before CFG/SSA/PHI mutation; the outer
unpublished session remains the sole rollback owner
```

### Return-read co-seal view D0 (2026-08-18; Decision: accepted)

The worker premise audit closes the design question only as a BoxShape.  The
next product is a same-cohort logical/layout/Join co-seal view, not a physical
Return-read receipt and not a new semantic `Verified*`/`Prepared*` authority.

```text
Decision:
  accept one callback-scoped `CommonV2ReturnReadCoSealRefV1` view that retains
  the existing Return source binding and copies only the already-issued
  logical item 9/result, logical then block, layout split ordinals for the
  then/body segments, the exact logical continuation item, and the existing
  Join `Return -> FunctionExit` target. It carries no physical block, ValueId,
  edge, terminator, or Completion claim.
Source authority + canonical issuer:
  `VerifiedS6CReturnSourceRecipeBindingV1` remains the sole source-to-key
  issuer. Existing `issue_s6c_common_v2_pre_session_v1` is the sole
  same-cohort co-seal issuer; it may validate the existing operation row,
  If/Exit rows, layout segments, and Join transfer, but may not invent a key.
Non-authority:
  fixed ordinals, Layout alone, `IfContinuationPhysicalTargetRefV1`, raw
  Builder/session cursors, MIR IDs, the old V1 read emitter, and a V2-to-V1
  adapter cannot issue or pair Return meaning. The view is not a physical
  receipt and cannot be cloned or retained outside its callback/session loan.
Fail-fast boundary:
  reject foreign owner, missing/duplicate/ambiguous item 9 or item 10,
  ReadBinding/Exit/value/block drift, source-site/region drift, non-Return or
  non-FunctionExit Join arm, continuation non-strictness, and layout segment /
  split mismatch before any physical mutation; the outer unpublished session
  remains the sole rollback owner.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-READ-COSEAL-VIEW-I0`: implement the
  callback-scoped co-seal view and focused positive/negative transport gates;
  consume it only through `CommonV2CanonicalSessionRefV1` and emit no physical
  effect.
Non-claims:
  no physical block/value, canonical binding read, Completion claim,
  `emit_branch`, `emit_return`, edge/PHI, publication, fallback, retry,
  production switch, or old-path retirement is opened by this BoxShape.
```

The next physical materialization design remains separate.  The co-seal view
may prove which existing logical/layout/Join rows must later agree, but it
does not authorize a `ValueId`, physical block, terminal, or CFG mutation.

### Return-read co-seal view I0 closeout (2026-08-18; landed)

`CommonV2ReturnReadCoSealRefV1` is now transported inside the existing common
V2 pre-session envelope. Its issuer validates one same-cohort operation row
for item 9, exact If/Exit placement, physical-ID-free segment coverage, and
the Join `Return -> FunctionExit` plus strict `Fallthrough(NextItem)` relation.
The view remains callback-scoped and non-Clone; no physical block, `ValueId`,
edge, terminator, Completion, publication, fallback, retry, or production
authority was added.

Focused `common_v2_issuers` tests are green (8/8), including operation and
Exit-value drift negatives. `cargo check --profile quick`, format, diff,
current-state pointer, and loop physical-transfer authority guards are green.
The next row is the separate
`LOOP-PHYSICAL-IF-CONTINUATION-RETURN-READ-PHYSICAL-RECEIPT-D0` design stop;
it must name a source-backed physical ValueId/read and FunctionExit-terminal
owner before any CFG/SSA/PHI mutation.

### Physical Return-read receipt D0 — accepted boundary (2026-08-18)

The follow-up authority audit confirms that the co-seal I0 alone cannot be
treated as a physical receipt. `identity.read_entry_receipt`, the segment
receipt, the continuation target reservation, Completion, and `emit_return`
remain separate owners when used independently. The old V1 read emitter is
outside the common-V2 cohort and is not adapted.

```text
Decision:
  accept one session-local, callback-scoped
  `CommonV2ReturnReadPhysicalReceiptV1` that consumes the existing logical
  co-seal, the same-session segment receipt, and the one-shot continuation
  target; it issues the item 9 canonical BindingRef read and records the
  exact item 10 FunctionExit Completion witness, but does not write a Return.
Source authority + canonical issuer:
  `VerifiedS6CReturnSourceRecipeBindingV1` -> existing
  `CommonV2ReturnReadCoSealRefV1` supplies source site/binding/result/exit
  meaning. The sole physical issuer is the new
  `CommonV2CanonicalSessionRefV1::with_return_read_physical_receipt` callback
  under `with_common_v2_physical_entry_session`; Completion remains the
  existing canonical owner for the source-site/target claim.
Non-authority:
  Layout/segment rows, `IfContinuationPhysicalTargetRefV1`,
  `identity.read_entry_receipt`, Completion, `CanonicalCfgSessionV1`, fixed
  ordinals, and the old V1 emitter cannot independently pair Return meaning.
Fail-fast boundary:
  reject foreign owner/stamp, missing or duplicate segment/target rows,
  source site/region/binding/result drift, target-function or explicit-site
  mismatch, wrong physical block/type, duplicate issuance, and any missing
  FunctionExit relation before the read/Completion mutation; the outer
  unpublished session remains the sole rollback owner.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-READ-PHYSICAL-RECEIPT-I0`: implement
  the callback-scoped receipt, canonical read, Completion claim/identity
  mark, and positive/negative/late-discard gates. Keep branch/Return CFG
  writing closed.
Non-claims:
  no `emit_branch`, `emit_return`, edge, PHI, CFG publication, DraftSeal
  publication, fallback, retry, production switch, or legacy retirement.
```

This accepted boundary is a new physical session receipt, not a second
semantic source/Recipe authority. Its terminal evidence is the existing
Completion claim; the mechanical FunctionExit Return writer remains a later
branch-emission design row.

### Physical Return-read receipt I0 closeout (2026-08-18; landed)

`CommonV2CanonicalSessionRefV1::with_return_read_physical_receipt` now
consumes the existing co-seal, same-session segment receipt, and one-shot
continuation target as one callback-scoped, non-Clone physical receipt. It
validates owner/stamp, unique rows, split/item coverage, continuation parity,
source binding/site/result, target function, and duplicate issuance before
effect. The canonical identity/SSA owner issues the item-9 read; the existing
canonical-session i64 type publication seam admits the source-proven class
when an unsealed PHI is still `Unknown`; Completion claims the existing FunctionExit
terminal witness and identity marks the return.

Focused `common_v2_return_read` tests are green (2/2), including late callback
failure with an empty outer Builder transaction. `cargo check --profile quick`,
format, diff, current-state pointer, and loop physical-transfer authority
guards are green. The touched Rust files remain below the 760-line design
trigger and 800-line hard boundary. This I0 emits no branch/Return, edge,
PHI/CFG publication, DraftSeal publication, fallback, retry, production
switch, or legacy retirement. The next design stop is the existing
`LOOP-PHYSICAL-IF-CONTINUATION-BRANCH-EMISSION-D0` / split-terminal authority
row.

### Branch-emission D0 refresh after Return-read I0 (2026-08-18; NoSafeSlice remains)

The read receipt is intentionally not a branch/terminal receipt. A read/Completion
co-seal now carries the source return binding, exact If/then/continuation layout,
and FunctionExit witness, but it does not carry the condition `ValueId` from the
existing `CanonicalConditionBoolResultReceiptV1`. Those two receipts must not be
joined by caller convention or by a placement block alone.

Decision:

Keep `NoSafeSlice::IfContinuationBranchEmissionAuthorityUnsealed`. The next
design slice is one callback-scoped canonical branch/terminal consumer that
co-seals the existing condition Bool receipt, the Return-read receipt, exact
item-to-split/then/continuation targets, and the FunctionExit terminal before
any CFG writer is reachable. No new semantic source/Recipe issuer is justified.

Source authority + canonical issuer:

`issue_s6c_v2_return_read_co_seal_v1` remains the source/Join authority;
`CommonV2CanonicalSessionRefV1::with_return_read_physical_receipt` remains the
canonical Return-read physical issuer; `CanonicalConditionBoolResultReceiptV1`
is the existing condition-result issuer; Completion remains the terminal/claim
owner. The missing authority is their single consumer/co-seal, not a new
caller-supplied value.

Non-authority:

`IfContinuationPhysicalTargetRefV1::continuation_physical_block`, a standalone
condition destination, `terminal_block`, caller-supplied condition `ValueId`,
and raw `CanonicalCfgSessionV1::emit_branch`/`emit_return` are placement or
mechanical evidence only.

Fail-fast boundary:

Reject foreign owner/stamp, missing/duplicate/drifted condition result, missing
or mismatched item-to-split and then/continuation rows, and any FunctionExit or
terminal mismatch before CFG mutation. A late callback failure must discard the
outer unpublished session transaction.

Smallest next slice:

Design-only audit of that existing-receipt consumer. Keep branch/Return,
edge/PHI/CFG publication, fallback, retry, production selection, and legacy
retirement closed until the co-seal is accepted.

Non-claims:

This refresh authorizes no code, new semantic receipt, `emit_branch`,
`emit_return`, edge/PHI/publication, production switch, fallback, retry, or
legacy retirement.

### Branch-emission D0 shared-segment audit (2026-08-18; NoSafeSlice remains)

The follow-up authority audit found a second concrete mismatch before any CFG
consumer can be implemented. `emit_length_call_result` currently allocates its
own segment receipt internally, while `with_return_read_physical_receipt`
accepts a segment receipt supplied by its caller. Therefore a green
`CanonicalConditionBoolResultReceiptV1` and a green
`CommonV2ReturnReadPhysicalReceiptV1` can still refer to different physical
condition blocks. Owner/stamp equality alone cannot prove same allocation.

Decision:

Keep `NoSafeSlice::IfContinuationBranchEmissionAuthorityUnsealed`. The next
design slice must name one canonical shared-segment consumer that owns the
allocation and co-consumes the condition Bool receipt, Return-read receipt,
and continuation target before `emit_branch`/`emit_return`. This is a physical
co-consumer BoxShape, not a new semantic source/Recipe issuer; changing the
existing Length API or adding a second segment allocation path is not
authorized until that boundary is accepted.

Source authority + canonical issuer:

The condition producer/branch plan and `CommonV2ReturnReadCoSealRefV1` remain
logical authorities. `CanonicalSsaFunctionSessionV2` is the sole physical
segment/value owner, and `CanonicalCfgSessionV1` is the sole mechanical CFG
writer. A future shared consumer must receive one session-owned segment receipt
and pass it through both physical products without reacquiring or reconstructing
layout meaning.

Non-authority:

Separate `allocate_v2_segment_blocks` calls, `condition_block`/`physical_block`
equality, owner/stamp equality, placement blocks, raw `ValueId`s, and individual
receipt green tests cannot establish same-session allocation or issue a branch
source/terminal relation.

Fail-fast boundary:

Before any CFG mutation, reject distinct segment allocation identity, owner or
stamp drift, condition logical-result/If-condition mismatch, condition physical
block/If physical block mismatch, missing or duplicate split rows, mismatched
then/continuation targets, and absent `FunctionExit` Completion evidence. The
outer unpublished function transaction remains the sole rollback owner.

Smallest next slice:

Design-only API census for one shared segment receipt from allocation through
Length/Bool/Return-read/branch-terminal consumption. No new receipt, branch,
Return, edge/PHI/CFG publication, fallback, retry, production switch, or
legacy retirement is authorized by this audit.

Non-claims:

The existing I0 receipts remain valid only for their individual bounded claims;
they do not prove a branch-ready physical topology or authorize a caller to
join independently allocated segment views.

### Shared-segment scope I0 — accepted BoxShape (2026-08-18)

Decision:

Accept one private, callback-scoped `CommonV2SharedSegmentScopeV1` as the
mechanical bridge for this cohort. The scope owns exactly one session-local
segment allocation brand and lends the same `PreparedSegmentBlockReceiptV1`
through Length/Bool/Return-read consumers. This is transport/lifetime
evidence only; it issues no semantic source/Recipe meaning and no CFG effect.

Source authority + canonical issuer:

The existing condition producer/branch plan and
`CommonV2ReturnReadCoSealRefV1` remain logical authorities. The canonical
session owns the one-shot allocation and brand; its outer unpublished function
transaction remains the rollback owner. The scope itself is not a new
`Verified*`/`Prepared*` semantic issuer.

Non-authority:

Owner/stamp equality, raw physical block equality, detached segment rows,
caller-held `ValueId`s, and separate direct Length allocation cannot establish
same-scope provenance. The old direct Length canary may remain only as a
caller-zero compatibility wrapper; the shared scope path is the only successor
allowed to feed the later branch consumer.

Fail-fast boundary:

Reject second scope/allocation, foreign brand/session/owner/stamp, missing or
duplicate/split-drifted rows, and any Length/Bool/Return-read transition that
does not carry the exact scope before physical branch mutation. A late callback
failure discards the outer unpublished session once.

Smallest next slice:

`LOOP-PHYSICAL-IF-CONTINUATION-SHARED-SEGMENT-SCOPE-I0`: implement the private
scope and explicit Length-from-scope consumer with positive, second-scope, and
late-discard gates. The Bool adapter retains the scope brand. The current S6C
fixture's Return-read If is the inner TextEq condition, not the outer
Length/Bool condition, so the Bool→Return-read adapter is guarded by an
expected `ConditionLogicalMismatch` negative rather than a false positive.
Keep branch/Return, edge/PHI/CFG publication, fallback, retry, production
switch, and legacy retirement closed.

### Shared-segment scope I0 closeout (2026-08-18)

Implemented one private `CommonV2SharedSegmentScopeV1` with a session-local
one-shot `Rc` brand. Length can only emit through the exact scope receipt;
Bool retains that brand; second allocation, foreign scope, and late callback
paths fail before publication. The Bool→Return-read adapter also checks the
co-sealed logical condition and rejects the fixture's outer-vs-inner mismatch
before any Return-read effect.

Evidence: common condition suite 7/7 green (including positive Length→Bool,
second-allocation, logical-mismatch, and late-discard cases); Return-read suite
2/2 green; `cargo fmt --all -- --check` green. No branch/Return CFG, edge/PHI,
publication, fallback, retry, production switch, or legacy retirement opened.

The next design decision remains branch/terminal authority; no further fast
slice is selected by this closeout.

Non-claims:

This BoxShape does not authorize `emit_branch`, `emit_return`, any new semantic
receipt, edge/PHI/publication, production selection, fallback, retry, or legacy
retirement.

### COMMON-V2-TEXTEQ-RESIDENCE-D0 (2026-08-18; design stop)

Decision: reject the old `A -> B -> C` premise. S6C TextEq is the portable,
non-faulting `LoopOperationV2::TextEq` physicalizer boundary; it is not a
source `StringEquals/1` method call. Physical ordering is Substring V9
residence plus ExactText V1 residence -> TextEq V10 -> If/Return.

Source authority + canonical issuer: resolver `Equal(Text, Text) -> Bool` ->
`VerifiedS6CTypedInputRelationV1` -> S6C Facts/Recipe ->
`TextEq(item 7, B1, V9, V1 -> V10)` -> `If(item 8, condition V10)` ->
Return-read/FunctionExit. `CanonicalSsaFunctionSessionV2` remains the sole
physical `ValueId` and type issuer.

Non-authority: the caller-zero DesignOnly `StringEquals/1` row,
`nyash.string.eq_hh`, raw MIR/handle equality, `CheckedCallOut`,
selected-Dynamic's TextEq rejection, and outer-loop Bool V5 cannot issue this
meaning.

Fail-fast boundary: before the first TextEq MIR effect, reject item/block/key
drift, missing/foreign/duplicate/stale V9 lease or V1 slot/generation,
owner/session/segment mismatch, unsupported Text representation, or a backend
without exact content equality. TextEq remains semantically `NonFaulting`;
unsupported shapes reject before effect, and late physical failure discards
the unpublished function without fallback or retry. No source Trap is minted.

Smallest next slice: design-only BoxShape
`COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-D0`. First name the canonical
source-backed V9 result/lend issuer; only after it is accepted may one
session-scoped owner co-seal V9 with the adopted ExactText slot/generation
sidecar and classify a TextEq capability as Direct, Checked, or
RejectBeforeEffect.

Non-claims: no new source/Recipe acceptance, C status/out ABI, source Trap,
Bool V10 materialization, branch/Return/CFG, selected-Dynamic parity,
production switch, fallback, retry, or legacy retirement is opened.

Premise-reset evidence:

- the typed source census admits exactly two calls, Length and Substring, and
  exactly four binaries, Less/Add/TextEqual/Add; there is no hidden third
  StringEquals call;
- AST/body classification exhaustively visits the S6C callable body; its
  transferred program-root placement does not make any If/Loop/Return/
  Assignment/Binary/MethodCall child opaque;
- Recipe V2 gives TextEq its own `Text x Text -> Bool` variant and classifies
  it `NonFaulting`; only CallSlot is `ExternallyBoundOutcome`;
- exact order is item 6 Substring -> V9, item 7 TextEq(V9,V1) -> V10, item 8
  If(V10), then Return-read/Exit. Substring residence must therefore precede
  TextEq materialization;
- V5 is the outer `CompareI64 Less` condition and cannot substitute for V10.
  For input `"abc", "b"`, wiring V5 to the inner If returns 0 immediately;
  the correct V10 returns index 1;
- the checked TextScan Substring EndAuthorizedHandle/lease is reusable only as
  V9 transport substrate. CheckedCallOut and any later checked C equality
  entry remain children of the portable physicalizer, never source authority;
- `nyash.string.eq_hh` was independently classified and recorded at
  `c951539dfc`: hook/fallback behavior, lossy invalid-handle handling, raw
  `i64`, and downstream `!= 0` truthification make it RejectBeforeEffect
  for this strict lane.

Additional export re-check (2026-08-18; closed read-only audit): the C export
`nyash_string_eq_hh_export` does exist at
`crates/nyash_kernel/src/exports/string.rs`, but it is only a generic
dispatch/fallback raw-`i64` transport. It does not issue the S6C source
cohort, `{slot,generation}` residence, strict result/fault contract, or a
common-session lease. Its lossy invalid-handle path and hook-miss values make
it `RejectBeforeEffect` for this lane. The runtime `TextFormalCallResidenceV1`
and `dynamic_v2_lease` owners are reusable substrate components only; neither
has a current common-V2 caller. Therefore this audit revalidates the
non-authority classification and does not create a StringEquals/1 task or
change the next row.

Residence issuer census: `CanonicalSsaFunctionSessionV2` currently exposes
only the physical-entry ExactText slot/generation sidecar; that sidecar
stores integer carriers and is not a live runtime pin. The existing
`TextFormalCallResidenceV1` owns formal slot/generation pins, but no current
common-session caller acquires or lends it. `dynamic_v2_lease` owns a
Substring EndAuthorized handle/token, but no common-V2 consumer carries it
from the source `CallSlot` result. The selected-Dynamic lifecycle terminal
and its checked-callout pair are a different caller-zero cohort. Therefore
there is no existing V9/V1 residence issuer to reuse, and inventing one from
raw `ValueId`, handle bits, or backend-frame metadata would violate the
authority boundary.

The primary missing-boundary class is `MaterializationRelationMissing`, not
semantic authority. The design row remains `NoSafeSlice` until the one
session-scoped residence owner, its finish/rollback edge, and its negative
matrix (foreign/stale/duplicate/missing lease or slot-generation) are named.

The landed DesignOnly `StringEquals/1` row is retained as a separate,
caller-zero method-surface capability and remains rejected before effect. It
does not authorize, block, or name the S6C binary-equality path.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-D0 (2026-08-18; design stop)

Decision: split the missing residence boundary at the first physical result.
The S6C `Substring` CallSlot is the only source-backed authority for V9; a
future common-V2 issuer must produce a checked `EndAuthorized` V9 result and
its scoped text-lend relation before the V1 formal residence can be co-sealed.
The previously proposed `CommonV2TextEqResidenceScopeV1` remains a private
aggregate design only: it may co-seal already-issued V9/V1 capabilities, but
it may not create either capability or issue TextEq meaning.

Source authority + canonical issuer: S6C source `StringSubstring` (arity 2,
Body) -> Recipe item 6/B1 `CallSlot(V0,[V6,V8] -> V9:Text)` -> one future
source-backed V9 materializer using the checked TextScan Substring contract;
the canonical session remains the only physical `ValueId`/type issuer.

Non-authority: `publish_end_authorized_text(String)` with an arbitrary fresh
String, `nyash.string.eq_hh`, raw handles or `ValueId` bits, selected-Dynamic
I6/CheckedCallOut, MIR `MirType`, and any generic fallback or retry.

Fail-fast boundary: before the first Substring effect, reject wrong role,
operation, arity, placement, item/block/result drift, foreign owner/session or
segment brand, missing/duplicate V9 receipt, non-Text result lane, invalid
out-wire status/tag, zero/unknown/stale lease, or a result without a scoped
text-lend owner. A Substring fault emits no V9 and no End. Late failure
discards the unpublished function; it never reuses the session.

Smallest next slice: design-only BoxShape
`COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-D0`. Name the source-backed V9
materializer, its scoped text-lend view, and its exact finish/rollback owner.
Only after that decision is accepted may the private V9+V1 residence scope be
implemented; TextEq V10 remains closed.

Required negative matrix: wrong S6C row or block, foreign session/segment,
duplicate or missing lease, `ImmediateI64`/`Forwarded`/`None` treated as V9,
stale token, missing or non-Text result, double consume, finish omission,
fault-path End, and partial-acquisition rollback failure. No fallback/retry or
semantic `Verified*`/`Prepared*` receipt is issued by this design row.

Consultation closure: both read-only workers agree that the aggregate is a
safe BoxShape but not implementation-ready. Existing
`TextFormalCallResidenceV1` and `EndAuthorizedTextV1` remain lifetime
substrates; neither currently has a common-V2 caller. The active pointer
therefore advances only to this V9 issuer design stop.

Issuer census addendum: the common session currently has no canonical Body
receipts for the `V6` index read or `V8` `Add(V6,1)` operand, and no
`StringSubstring/2` target plan. Those three source-backed operand/target
relations are part of the V9 issuer design; they must not be reconstructed
from raw item ordinals, `ValueId`s, or the selected-Dynamic cursor.

### LOOP-PHYSICAL-S6C-TEXTEQ-OPERAND-ISSUER-D0 (2026-08-18; design stop)

Decision: split the V9 boundary once more at its canonical integer operands.
S6C owns the exact body sequence `V6 = ReadBinding(index)`, `V7 = ConstI64(1)`,
`V8 = Add(V6,V7)`, and `V9 = StringSubstring(V0,V6,V8)`. The next physical
issuer is S6C-specific and may reuse only the existing canonical session
mechanics for physical ID allocation, type publication, entry/segment reads,
and integer instruction emission.

Source authority + canonical issuer: resolver S6C source rows and Recipe roles
`body_index_read`, `slice_one`, `slice_end_add`, and `substring_call` -> one
future callback-scoped S6C operand issuer. `CanonicalSsaFunctionSessionV2`
remains the sole ValueId/type issuer; the operand product carries no source
meaning or runtime lease.

Non-authority: selected-Dynamic V6/V8/V9 code, Dynamic formal values or value
ledger, outer Less V5, generic AST `Equal`/`CompareOp::Eq`, raw item ordinals,
caller-supplied ValueIds, and any S6C use of Dynamic I6/I7 CheckedCallOut.

Fail-fast boundary: before any body instruction, reject role/item/block/result
drift, non-`B1` placement, foreign owner/session/segment brand, missing or
duplicate body read, wrong `V7=1` literal, non-Add or wrong Add operands,
operand type drift, or an operand receipt from another cohort. Late failure
discards the unpublished function; same-session repair/retry is forbidden.

Smallest next slice: design-only BoxShape
`LOOP-PHYSICAL-S6C-TEXTEQ-OPERAND-ISSUER-D0`. Name one private callback-scoped
operand receipt that consumes the existing S6C ingress and shared Body segment,
emits only V6/V7/V8 through canonical mechanics, and lends those values to the
later source-backed V9 issuer. Do not emit Substring, TextEq, Bool, branch,
Return, CFG/PHI, or runtime lease in this row.

Positive acceptance: exact S6C source/Recipe parity, one same-session Body
segment brand, canonical `ValueId`/type publication, and callback-only receipt
borrowing. Negative acceptance: any selected-Dynamic row reuse, outer V5
substitution, missing/duplicate operand, foreign segment, wrong literal or
Add shape, and detached ValueId tuple. No new semantic `Verified*` or
`Prepared*` receipt is issued by this design row.

### LOOP-PHYSICAL-S6C-TEXTEQ-OPERAND-ISSUER-I0 (2026-08-18; fast)

The D0 is accepted for one BoxShape implementation. The owner will live in a
new child module beside `common_v2_session.rs` so the session parent remains
below the 800-line source limit. It will expose one private callback-scoped,
non-`Clone` receipt for V6/V7/V8, consume only the existing S6C ingress and
shared Body segment, and return the canonical ValueIds only through that
receipt. A narrow source-row accessor may be added to the existing physical-
ID-free operation program; it may not issue a second semantic product.

Implementation acceptance: exact roles/items/blocks/classes are checked before
the first body instruction; `identity.read_entry_receipt` issues V6, the
canonical session issues V7 and V8, and only canonical type publication is
used. Positive/negative focused tests must cover same-segment success,
duplicate/foreign scope, wrong literal/Add shape, and late unpublished discard.
The I0 ends before Substring V9, runtime lease, TextEq V10, Bool, branch,
Return, CFG/PHI, publication, fallback, retry, and production selection.
