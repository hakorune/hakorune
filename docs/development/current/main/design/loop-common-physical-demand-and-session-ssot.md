---
Status: SSOT
Date: 2026-08-18
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

- **Current decision:** S6C TextEq is the existing source
  `Equal(Text,Text) -> Bool` operation and is non-faulting. The strict physical
  choice is **Direct-or-RejectBeforeEffect** through one private
  `CommonV2S6CPortableTextEqBoolCapabilityV1`; `CheckedCallOut`, raw handle
  comparison, `StringBox::equals`, and `nyash.string.eq_hh` are not its
  authority. The common Loop pipeline still has one semantic program, one
  JoinSig-bound layout, one canonical session, and one publication owner.
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
  residences must be co-sealed before V10 materialization. The common-V2
  Substring target/admission I0 is now effect-free and caller-zero. The
  existing V9 issuer I0 validates an already-produced runtime wire, adopts its
  End lease, and lends Text only through a callback-scoped view. A
  phase-boundary audit now classifies that object as a caller-zero runtime
  contract canary, not a production MIR physicalizer: it receives a concrete
  `DynamicV2CallOutV1` during compiler execution and emits neither
  `CheckedCallOut` nor a V9 `ValueId`. The runtime-only StableText wire issuer
  and the source-bound ExactText occurrence view remain valid subordinate
  substrate, but neither can bridge compile-time MIR values to concrete
  runtime pairs. The source/Facts/Recipe/Join chain is retained; only the
  compiler/runtime physical boundary is reset. The separate compiler-side V9
  MIR I0 now emits one canonical CheckedCallOut/NormalResult/Fault/End
  lifecycle through the existing CFG/SSA writers without accepting that wire;
  its outer unpublished-function rollback remains the only failure owner.
- **Current stop:** the Direct capability is selected, but the
  `PinnedTextBackendFrameBorrowV1` content projection is still the next
  bounded I0. The final route is backend-private frame-row `ptr/len` loaded
  once in the preheader; a Rust `with_text(&str)` lend is only a safety
  canary. No Bool V10 or content comparison effect is issued yet.
- **Closed substrate:** source/Facts/Recipe/Join co-seal, V9 producer and
  canonical End lifecycle, index-only TextRef entry bridge, one-shot
  V9+ExactText residence scope, and the caller-zero concrete StringBox
  Residence canary are complete. The row-11 mutable-reachability census is
  already an explicit acceptance input and reusable guard; no duplicate
  census task is needed.
- **Runtime ownership decision:** validate all lanes once at entry, pin
  atomically, lend content only through a scoped residence owner, and hold no
  registry lock, LeaseSet, allocation, callback, or finish in the loop body.
  The accepted concrete StringBox baseline is zero-copy and adds no root
  `Arc` or snapshot. `Arc` is an ownership tool after lock release, not a
  concurrency policy; an immutable shared-backing task opens only if a
  sanctioned mutable/unsafe provider path is ever admitted.
- **Next ordered task:** implement the private backend content projection on
  the existing frame contract. It must not leak a raw handle, slot/generation,
  compiler pointer, or semantic receipt; absent a safe frame-row projection,
  remain `NoSafeSlice`.
- **Production stop line:** no Bool V10, If/Return CFG, publication,
  production selector, performance promotion, fallback/retry, or `eq_hh`
  retirement is open from this capsule.

### Final shape task ladder

| order | bounded task | exit condition |
| --- | --- | --- |
| 0 | `COMMON-V2-S6C-PORTABLE-TEXTEQ-CONTENT-VIEW-D0` | Accepted: use the existing pinned C frame and `PinnedTextBackendFrameBorrowV1`; backend-private preheader `ptr/len` is the only hot route, while `with_text` is canary-only. |
| 1 | `COMMON-V2-S6C-PORTABLE-TEXTEQ-CONTENT-VIEW-I0` | One private frame-row projection and preheader load; no raw escape, lock/alloc/callback/finish in loop, or second owner. |
| 2 | `COMMON-V2-S6C-PORTABLE-TEXTEQ-V10-I0` | One Direct leaf consumes the existing scope and asks the canonical session for Bool V10; no If/Return yet. |
| 3 | `COMMON-V2-S6C-INNER-CFG-D0/I0` | Existing If/Return/FunctionExit JoinSig receipts consume V10; one unpublished transaction remains the rollback boundary. |
| 4 | `COMMON-V2-S6C-CORRECTNESS-CANARY-R0` | Positive/negative Unicode, alias, stale/foreign, lifecycle, and late-discard evidence is green. |
| 5 | `COMMON-V2-S6C-PRODUCTION-EDGE-D0/I0` | Named caller switch, same-commit old-edge retirement, and zero fallback/retry are observed. |
| 6 | `S6C-PINNED-TEXT-PERFORMANCE-PROMOTION-R0` | IR/assembly structural zero-boundary, then exact/meso/whole-call C comparison; benchmark cannot waive structure. |
| 7 | `EQ-HH-RETIREMENT-R0` | Generic C/Python caller census reaches zero independently; only then remove the legacy export. |

The runtime immutable-backing alternative is a conditional gate, not a
parallel route: if the mutable-reachability guard discovers a sanctioned
write/retention path, insert `TEXT-FORMAL-IMMUTABLE-BACKING-D0` before order
1 and choose a residence-owned immutable backing. Until then, keep the
zero-copy pinned concrete payload; do not migrate `StringBox` to `Arc<str>`
speculatively.

### COMMON-V2-S6C-PORTABLE-TEXTEQ-CONTENT-VIEW-D0 (2026-08-18; accepted)

Decision: classify the strict TextEq physical capability as
`Direct-or-RejectBeforeEffect` and use the existing pinned C residence frame
as the sole runtime-byte backing. The backend may project each validated,
occurrence-ordered frame row to `ptr + byte_len` once in the preheader and
keep those values as private SSA inputs to the later UTF-8 leaf. A
Rust-side `with_text(&str)` callback is permitted only as a caller-zero
same-value/lifetime canary; it is not the compiler or hot-loop route.

Source authority + canonical issuer: resolver
`Equal(Text,Text) -> Bool`, S6C Facts/Recipe/co-seal, and the existing TextEq
occurrence relation own meaning and root order. `TextFormalCallResidenceV1`
owns slot/generation validation, pin, and finish. The existing
`PinnedTextBackendFrameBorrowV1` owns the mechanical frame projection, and
`CanonicalSsaFunctionSessionV2` alone may issue Bool V10 later.

Non-authority: `with_text_formal_identity` lookup/read-lock, raw pointers in
MIR/compiler/JSON, handle/slot/generation/ValueId reinterpretation, Arc or
snapshot backing, alias deduplication or `noalias`, `PinnedTextOp` transport
alone, C shim/status rows, `StringBox::equals`, and `nyash.string.eq_hh`.

Fail-fast boundary: before any effect, validate the exact source/cohort,
owner/session/segment, ordered root indices, duplicate occurrence policy,
live generation and non-retiring concrete Text payload, UTF-8 validity, frame
limits, target pointer layout, plan/frame stamps, and lifetime/finish order.
Any foreign/stale/pending/reordered/unsupported/escaped projection rejects
before publication; late failure discards the unpublished function. The hot
loop contains zero host-table lock, LeaseSet, allocation, callback, retain,
generation check, or Residence finish.

Smallest next slice: `COMMON-V2-S6C-PORTABLE-TEXTEQ-CONTENT-VIEW-I0` adds one
private backend-only frame-row projection/preheader contract by reusing the
existing frame and plan owners. It adds no semantic receipt, no new runtime
wire, and no Bool/CFG effect. If the frame cannot prove the row/stamp/lifetime
relation, retain `NoSafeSlice`.

Non-claims: no V10 Bool, If/Return CFG, publication, production switch,
performance promotion, fallback/retry, `Arc<str>` migration, or `eq_hh`
retirement. The immutable-backing alternative opens only if the mutable
reachability guard ever discovers a sanctioned write/retention path.


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

Worker premise gate: Fast path is disallowed because no common-V2 caller yet
co-seals source CallSlot V9 with a checked EndAuthorized residence and its
finish/rollback owner; workers must decide whether an existing issuer is
reusable or a new BoxShape is required.

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

### COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-D0 consultation closure (2026-08-18)

Decision: accept `MaterializationRelationMissing` as the sole remaining
boundary. The source/Recipe relation is closed, but no common-V2 canonical V9
issuer exists. The checked `hako.text.scan.substring.v1` export is the only
candidate transport; it is not source authority and cannot be borrowed from
the selected-Dynamic corridor as-is.

Source authority + canonical issuer: the retained S6C resolver
`StringSubstring/2` contract and Recipe `CallSlot(item 6, B1, V0, [V6,V8] ->
V9:Text)` are the source facts. The next issuer must co-seal that contract
with the existing V6/V7/V8 callback receipt and the same Body segment; the
canonical session remains the sole physical `ValueId`/type issuer.

Non-authority: `nyash.string.eq_hh`, generic `substring_hii`, raw handles or
`ValueId` bits, `publish_end_authorized_text(String)`, Dynamic I6/I7 site
plans, `TextFormalCallResidenceV1` as a V9 result owner, and any fallback or
retry. The private V9 receipt may only lend an opaque callback-scoped result;
it may not mint TextEq/Bool meaning or consume runtime End by itself.

Fail-fast boundary: before a Substring effect, reject source role/op/arity/
placement/result drift, wrong receiver or `[V6,V8]` arguments, foreign
owner/session/body segment/stamp, non-Text or non-`EndAuthorized` provider
facts, zero/unknown/stale lease, duplicate receipt, and unsupported wire. A
fault emits no V9; late callback failure discards the unpublished function;
the runtime lease owner and the outer MIR transaction remain separate.

Smallest next slice: `COMMON-V2-TEXTEQ-SUBSTRING-V9-TARGET-I0` (BoxShape).
Transport the already-issued resolver Substring target contract into the
pre-session envelope and issue one Builder-free, physical-ID-free checked
TextScan target plan. It must stop before `CheckedCallOut`, V9 `ValueId`,
lease consumption, TextEq V10, Bool, source-level branch/Return CFG,
publication, fallback, retry, or production selection.

Acceptance for the next row: exact source/Recipe/target/provider parity and
same-cohort ownership are observable; wrong role/arity/receiver/args/result,
foreign source, and wrong provider lane reject before any Builder effect.
Only after this target plan is landed may the actual callback-scoped V9
materializer and its local checked-callout corridor be opened.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-TARGET-I0 (2026-08-18; fast)

Change: add one Builder-free, physical-ID-free `Prepared` target plan for the
source-backed S6C `StringSubstring/2` row and the checked TextScan Substring
export facts; transport the existing resolver call contract into the retained
common-V2 envelope without issuing a second semantic source product.

Contract: source/Recipe `item 6 / B1 / V0 / [V6,V8] -> V9:Text`, target
`StringBoxTextV1`/`TextToCaller`/`PureRead`, and checked provider
`HostHandle + ImmediateI64×2 -> HostHandle + EndAuthorized` must match one
owner/cohort. No MIR ID, CallOut site, lease token, or runtime text lend is
owned here.

Done: the positive exact-parity target-plan test and the foreign-owner
pre-effect rejection are green. The retained S6C source/Recipe ingress already
rejects wrong role/arity/receiver/args/result, while the provider projection
is checked as an exact static fact set. The quick focused gates, formatter,
authority guard, and module/reference receipt are green.

Stop: do not emit `CheckedCallOut`, V9 `ValueId`, End/lease consumption,
TextEq V10, Bool, source-level branch/Return CFG, publication, fallback,
retry, or production selection. Any missing source contract or provider fact
returns the row to `COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-D0`.

### S6C V6/V7/V8 operand issuer I0 closeout (2026-08-18)

The child issuer now validates the exact `ReadBinding(index) -> V6`,
`ConstI64(1) -> V7`, `Add(V6,V7) -> V8` prefix and one Body segment, then lends
the canonical values through a non-Clone callback receipt. The Substring
target-plan I0 separately carries the exact source/Recipe/provider facts.
Focused 4/4 operand tests and the target-plan positive/foreign-owner gates are
green; no Substring effect, CheckedCallOut, V9 lease/ValueId, TextEq, Bool,
CFG, publication, fallback, retry, or production path opened.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-CALLOUT-ADMISSION-D0 (2026-08-18; accepted)

Decision: accept one physical-only common-V2 admission boundary. The source
contract remains S6C `StringSubstring/2` plus Recipe `CallSlot(item 6, B1,
V0, [V6,V8] -> V9:Text)`; the target plan and V6/V7/V8 receipt remain its only
semantic inputs. `MaterializationRelationMissing` is narrowed to the missing
runtime result/lifecycle implementation, not source meaning.

Source authority + canonical issuer: the real collector brand borrowed from
`ModuleLoweringPortV1::with_invocation_brand` is carried as
`InvocationBranded<PreparedPhysicalEntrySessionInputV1>` through the common
entry/session seam. A private admission owned by
`CommonV2CanonicalSessionRefV1` co-seals that brand, the landed Substring
target, checked provider facts, one neutral single-site plan, and an opaque
`CommonV2SubstringEndObligationV1`. `PhysicalFunctionEntryCohortStampV1`
remains only owner/signature/lane evidence and is not extended or used as a
plan stamp.

Non-authority: selected-Dynamic I6/I7 pairs, `legacy_test` brands, owner or
provider IDs, `nyash.string.eq_hh`, generic `substring_hii`, raw handles/tokens/
ValueIds, and fallback/retry. The aggregate owns no runtime handle/token,
`EndAuthorizedTextV1`, semantic source meaning, or cloneable parts tuple.

Fail-fast boundary: unbranded/foreign invocation, cohort or session mismatch,
wrong source/Recipe role/item/block/result, provider ABI/wire drift,
non-`EndAuthorized` shape, duplicate site/lease/admission, or absent lifecycle
consumer rejects before the first CheckedCallOut. Late callback failure uses
the existing unpublished-function discard; no retry or same-session reuse.

Smallest next slice: `COMMON-V2-TEXTEQ-SUBSTRING-V9-CALLOUT-ADMISSION-I0`.
Implement only brand transport, a neutral single-site plan/admission, and the
one-shot lifecycle sidecar API; stop before callout effect, V9 ValueId/lease
consume, TextEq V10, Bool, Branch/Return CFG, publication, fallback, retry, or
production selection. Positive acceptance is same collector brand through
entry→session→admission; negatives are unbranded/foreign brand, duplicate
consume, missing lifecycle consumer, and late unpublished discard.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-CALLOUT-ADMISSION-I0 (2026-08-18; closeout)

Change: landed the effect-free common-V2 seam. The real collector brand now
travels through `InvocationBranded<PreparedPhysicalEntrySessionInputV1>` and
the canonical session retains it as the sole neutral callout plan stamp. The
session exposes one-shot Substring admission; the admission co-seals the
landed target plan, checked provider facts, one single-site plan, and opaque
`CommonV2SubstringEndObligationV1`. No new semantic source authority was
issued.

Evidence: `cargo check`, formatter, pointer/physical-transfer guards, the
direct admission positive test, callback-scoped lifecycle test, same-brand
session test, and foreign-brand pre-session rejection are green. The AOT
activation guard remains a pre-existing selected-package-adapter baseline
failure and is not part of this lane.

Stop: no CheckedCallOut effect, V9 ValueId/lease or text residence,
TextEq/Bool, CFG/PHI, Completion/DraftSeal, publication, fallback, retry, or
production caller. The next design stop is
`COMMON-V2-TEXTEQ-SUBSTRING-V9-RESIDENCE-D0`; `nyash.string.eq_hh` remains
transport-only and non-authoritative.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-RESIDENCE-D0 (2026-08-18; consultation closure)

Decision: keep `NoSafeSlice::MaterializationRelationMissing`. The source and
Recipe meaning is closed, but no common-V2 canonical V9 materializer owns the
checked Substring result, scoped text lend, and finish/rollback edge. Split the
boundary into a V9 issuer first and a later V9+V1 residence co-seal; do not
implement either aggregate or issue a new semantic receipt in this design row.
The top-down chain remains intact: source/resolver -> exact Facts -> S6C
Recipe -> one common physical owner -> unpublished session -> publication.
The additional `nyash.string.eq_hh` export audit changes no link in that
chain; it is a lossy raw-i64 transport and stays below the RejectBeforeEffect
boundary. No architecture-wide rewrite is required.

Source authority + canonical issuer: S6C `StringSubstring/2` and Recipe
`CallSlot(item 6, B1, V0, [V6,V8] -> V9:Text)` remain the authority. The next
private issuer must consume the existing V6/V7/V8 callback receipt, the same
Body segment brand, and checked `hako.text.scan.substring.v1` facts; the
canonical session remains the sole physical `ValueId`/type issuer.

Non-authority: `nyash.string.eq_hh`, `substring_hii`, raw handles/tokens or
`ValueId` bits, `publish_end_authorized_text(String)`, selected-Dynamic I6/I7,
`TextFormalCallResidenceV1` as a V9 owner, and fallback/retry. The future
`CommonV2TextEqResidenceScopeV1` may only co-seal already-issued V9 and ExactText
V1 capabilities; it may not mint either capability or TextEq meaning.

Fail-fast boundary: before Substring effect reject wrong role/op/arity/receiver,
item/block/result or argument drift, foreign owner/session/segment/stamp,
non-Text or non-`EndAuthorized` output, unknown/reserved/zero/stale lease,
missing/duplicate receipt, escaped raw capability, double consume, and partial
acquisition without rollback. A fault emits no V9/End; late callback failure
discards the unpublished function and never retries the session.

Smallest next slice: design-only BoxShape
`COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-D0`. Name the source-backed checked V9
materializer, callback-scoped opaque result/lend API, move-only End
handle/token adoption that validates the pair, exact formal-residence acquire,
and the single finish order `residence.finish -> End consume`; partial
acquisition must roll back in reverse order while retaining primary and
suppressed errors. Record the complete negative matrix, including wire drift,
token/handle mismatch, stale generation, double finish/consume, and late
unpublished-session failure.

Acceptance/non-claims: the Decision must identify one issuer and one lifecycle
owner plus Direct/Checked/RejectBeforeEffect classification. It must not add
code, fixtures, `Verified*`/`Prepared*` receipts, CheckedCallOut effect, V9
ValueId/lease consumption, TextEq V10, Bool, CFG/Return, publication,
production selection, fallback, retry, or legacy retirement.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-D0 (2026-08-18; accepted)

Decision: accept the checked V9 issuer boundary and open its one bounded I0.
The source-backed issuer consumes the landed Substring target/admission, the
same-cohort V6/V7/V8 operand receipt, and the same Body segment; it may issue
only a private callback-scoped V9 lifetime. The top-down authority chain needs
no rewrite.

Source authority + canonical issuer: resolver `StringSubstring/2` -> Recipe
`CallSlot(item 6, B1, V0, [V6,V8] -> V9:Text)`; the common canonical session
owns physical identity. The runtime End owner is subordinate and may adopt
only a checked `Normal + HostHandle + EndAuthorized` output whose nonzero
handle/token pair is generation-valid.

Non-authority: `nyash.string.eq_hh`, generic substring, raw handle/token/
ValueId, `publish_end_authorized_text(String)`, selected-Dynamic I6/I7,
TextFormal alone as V9, fallback, retry, and any source Trap.

Fail-fast boundary: reject source/cohort/segment/provider drift, unknown or
reserved wire, Fault/Suspended/Forwarded/ImmediateI64, zero or mismatched
handle/token, stale generation, non-Text payload, duplicate consume, escaped
borrow, or missing finish before any V9 consumer. Normal cleanup is
`residence.finish -> End consume`; partial acquisition rolls back in reverse
order and preserves primary/suppressed errors.

Smallest next slice: `COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-I0` (fast). Add the
move-only End adoption and callback-scoped Text lend/finish substrate, then
thread one caller-zero common-V2 issuer seam through the existing admission.
Do not emit CheckedCallOut, V9 `ValueId`, TextEq V10, Bool, CFG/Return,
publication, production selection, fallback, retry, or legacy retirement.

Acceptance: focused positive/negative lifecycle gates prove exact wire,
handle/token, stale/foreign, double-consume, callback-scoped lend, and reverse
rollback behavior; all source files remain below 800 lines. The next design
boundary is the later V9+V1 residence co-seal.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-ISSUER-I0 (2026-08-18; closeout)

Change: landed one caller-zero common-V2 issuer seam through the existing
admission. The runtime End owner now validates and adopts the exact
handle/token generation pair, lends Text only inside a callback, and consumes
the lease once. The move-only materialization finishes explicitly after the
callback and has a Drop rollback for panic/error paths, so a partial result
cannot escape as a raw handle or leak its End lease.

Top-down review: the architecture remains one chain, not a second physicalizer:

```text
source Binary Equal
  -> exact Facts/Recipe TextEq(V9,V1 -> V10)
  -> common-V2 target/admission + V6/V7/V8 receipt
  -> V9 issuer I0 (this row)
  -> later V9+ExactText V1 residence co-seal
  -> TextEq V10 -> If/Return -> unpublished session -> publication
```

The existing `nyash.string.eq_hh` export was re-audited and remains a
RejectBeforeEffect transport: it is hook/fallback based, returns raw `i64`,
and has lossy invalid-handle behavior. It cannot mint source meaning,
generation residence, strict fault semantics, or a common-session lease. No
architecture-wide rewrite is therefore required.

Evidence: `cargo check --profile quick`, formatter, `issuer_` focused tests
(41/41), `dynamic_v2_lease` focused tests (7/7), pointer/physical-transfer/
TextScan authority guards, and `git diff --check` are green. The warning volume
and first-link memory cost are baseline repository behavior; all Cargo runs
were serialized with `CARGO_BUILD_JOBS=4` and no concurrent terminal.

Stop/next: TextEq V10, Bool, branch/Return CFG, Completion/DraftSeal,
publication, fallback, retry, and production remain closed. The next design
stop is `COMMON-V2-TEXTEQ-SUBSTRING-V9-RESIDENCE-D0`, limited to co-sealing
the already-issued V9 lifetime with the existing ExactText V1 sidecar.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-RESIDENCE-D0 (2026-08-18; consultation closure)

Decision: retain design stop. The top-down MIRBuilder chain does not need an
architecture-wide rewrite, and V9 issuer I0 remains the sole owner of the
already-issued Substring result. A later private residence scope may borrow
that V9 lifetime, co-seal the exact ExactText V1 occurrence, and own one
runtime formal residence, but the runtime ingress from the canonical
slot/generation lane to an already-issued `TextFormalWirePairV1` is not named.
Do not derive that wire from `ValueId`, a logical ordinal, a bare sidecar row,
or the `nyash.string.eq_hh` C export.

Source authority + canonical issuer: source Binary
`Equal(Text,Text)->Bool` and its existing S6C Recipe relation
`StringSubstring/2 -> CallSlot(item 6, B1, V0, [V6,V8] -> V9:Text)` remain
authoritative. `CommonV2CanonicalSessionRefV1::with_s6c_substring_v9_issuer`
owns V9 adoption/lend/finish; canonical entry adoption owns the ExactText
slot/generation sidecar; `acquire_text_formal_residence_v1` is subordinate
runtime residence ownership only after a source-bound wire issuer is named.

Non-authority: sidecar `ValueId(slot,generation)` pairs as runtime wires,
logical ordinal alone, `PhysicalFunctionEntryCohortStampV1` as a residence
root, raw handles/tokens/pointers, selected-Dynamic I6/I7, `nyash.string.eq_hh`,
and any fallback, retry, TextEq/Bool/CFG, or production selector.

Fail-fast boundary: before any CheckedCallOut or residence pin, reject source/
Recipe role or item drift; wrong V9/V1 relation; foreign owner, invocation,
session, entry, Body segment, physical block, brand, or stamp; missing,
duplicate, non-adjacent, stale, zero, or carrier-mismatched ExactText lane;
absent or mismatched runtime wire ingress; unsupported StableText-only
residence for the actual StringBox payload; duplicate scope/finish, escaped
borrow, partial rollback, or late unpublished-session failure. Existing I0
must finish V9 exactly once; a future scope must finish runtime residence
before that V9 End lease and preserve primary/suppressed errors.

Smallest next slice: `COMMON-V2-TEXTEQ-SUBSTRING-V9-RESIDENCE-OCCURRENCE-D0`
(design-only). Name one source-bound, callback-scoped occurrence view that
connects the existing S6C TextEq needle binding to the canonical ExactText
sidecar and identifies the one legitimate issuer of an already-published
`TextFormalWirePairV1`. Classify Direct/Checked/RejectBeforeEffect for the
current StringBox/StableText mismatch. This row may only co-seal existing V9,
V1, and session/segment brands; it may not issue a semantic receipt or open a
physical effect.

Acceptance/non-claims: the decision must identify one occurrence authority,
one runtime wire issuer, and one rollback owner; prove same owner/session/
entry/body-segment and exact needle relation; and enumerate foreign/stale/
duplicate/escape negatives. It must not emit CheckedCallOut, V9 `ValueId`,
TextEq V10, Bool, Branch/Return CFG, publication, production, fallback, retry,
or legacy retirement. Until this ingress exists, the correct state is
`NoSafeSlice::ExactTextResidenceOccurrenceIssuerUnsealed`, not a speculative
residence implementation.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-RESIDENCE-OCCURRENCE-D0 (2026-08-18; accepted and closed)

Decision: accept one BoxShape-only source-bound occurrence view without
changing the MIRBuilder architecture. The source mapping is already closed;
the missing relation is physical occurrence transport. The common-V2 side may
issue only a private callback-scoped mechanical view, while
`runtime::text_formal_abi` remains the only owner allowed to validate and issue
an already-published `{slot,generation}` wire. Neither side may reconstruct a
runtime wire from a MIR `ValueId` pair or from a logical ordinal.

Source authority + canonical issuer: existing S6C typed input and Recipe
facts prove `Needle(Text)` as the TextEq right operand `V1`; canonical physical
signature/entry adoption proves the same `BindingRef` and adjacent
`ExactTextSlot + ExactTextGeneration` lanes. The future occurrence view
co-seals only that binding relation with the same owner, entry, session,
physical block, and invocation/body-segment stamp. The runtime pair issuer is
the host-handle generation table through a narrow `text_formal_abi` API; the
existing `TextFormalCallResidenceV1` remains its StableText-only pin/root
consumer.

Non-authority: `PhysicalTextEntryLaneSidecarV1` as a live pin, `ValueId`
slot/generation, ordinal-only lookup, `capture_text_formal_pair(raw_handle)`
as a re-capture path, `PinnedTextBackendFrameContractV1` as runtime state,
`hako_text_formal_validate_v1` or C frame entry as source issuer,
`nyash.string.eq_hh`, selected-Dynamic I6/I7, StringBox-to-StableText fallback,
and retry/fallback.

Fail-fast boundary: reject before residence pin or CheckedCallOut on any
Needle/subject swap, TextEq RHS drift, missing/duplicate/non-adjacent lane,
binding/ordinal/carrier mismatch, foreign owner/entry/session/segment/stamp,
zero/stale/mismatched published wire, absent runtime ingress, and an actual
`StableBox(StringBox)` payload when the selected residence owner is
StableText-only. No TextEq V10, Bool, CFG, publication, or production path is
opened by this row.

Smallest next slice: the private HRTB callback at the existing common-V2/session
boundary is now landed. It co-seals the already-issued S6C Needle/TextEq
relation with the matching canonical ExactText sidecar row. The view exposes
only mechanical binding/ordinal/carrier/owner/entry proof; it exposes neither
ValueIds nor raw slot/generation pairs. If the S6C StringBox shape must be
admitted, open a separate representation design; do not widen
`TextFormalCallResidenceV1`.

Acceptance/non-claims: positive evidence proves
`Needle BindingRef == TextEq RHS == ExactText sidecar binding` and same-cohort
lane/session/entry parity, ordinal 1, U64 carrier, duplicate one-shot rejection,
and zero instruction growth. Existing sidecar adoption guards retain the
missing/duplicate/carrier rejection boundary. This I0 does not add runtime
pinning, CheckedCallOut, V9 `ValueId`, TextEq/Bool/CFG, publication,
production, fallback, retry, or legacy retirement. The runtime wire issuer
remains the separate `TEXT-FORMAL-WIRE-INGRESS-I0` owner.

Evidence: `s6c_occurrence_view_co_seals_needle_with_exact_text_sidecar` (1/1),
the enclosing common-V2 S6C operand module (5/5), quick `cargo check`, format,
diff, pointer, physical-transfer, and Text-scan admission guards are green.
The broader physical-entry module is 12/13: its pre-existing direct-Length
duplicate assertion is a known baseline red because the compatibility wrapper
allocates a segment before checking its one-shot flag; this I0 does not touch
that path.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-RESIDENCE-MATERIALIZATION-D0 (2026-08-18; design stop)

Decision: keep the occurrence view and runtime StableText wire as separate
owners. A top-down redesign is not indicated: the source Binary Equal /
portable `LoopOperationV2::TextEq` chain remains authoritative, and
`nyash.string.eq_hh` remains only a subordinate transport candidate.

Source authority + canonical issuer: existing S6C `StringSubstring/2` and
TextEq source/Recipe facts; the common session owns V9 End materialization,
while `runtime::text_formal_abi` owns published StableText wire validation.
Non-authority: MIR `ValueId` pairs, logical ordinals, raw `eq_hh` i64 results,
fallback dispatch, and StringBox-to-StableText coercion.

Fail-fast boundary: no residence or CheckedCallOut effect until an existing
V9 materialization, exact sidecar occurrence, same session/segment, and
already-issued StableText wire/residence can be co-sealed; StringBox, stale,
zero, foreign, or missing bridge rejects before effect.

Smallest next slice: the residence audit is now complete and leaves the
runtime-value ingress as the next design-only boundary. No new semantic
receipt, runtime pin, TextEq/Bool/CFG, publication, fallback, retry, or
production switch is opened by this D0.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-EXACTTEXT-LANE-BORROW-INGRESS-D0 (2026-08-18; design stop)

Decision: keep `NoSafeSlice::ExactTextLaneBorrowIngressUnsealed` and define one
physical-only lane adapter before any residence or TextEq effect. This is a
BoxShape transport boundary, not a new source shape or semantic receipt; no
architecture-wide rewrite is required.

Source authority + canonical issuer: S6C `StringSubstring/2` and the existing
TextEq Facts/Recipe relation remain the source authority. The
`S6CTextEqOccurrencePhysicalViewV1` and
`PhysicalTextEntryLaneSidecarV1` prove the exact owner/binding/ordinal,
adjacent slot+generation lanes, and `U64BitsOnI64` carrier, but never mint a
runtime value. `CanonicalSsaFunctionSessionV2` remains the sole MIR `ValueId`
issuer. The runtime `text_formal_abi`/host-handle owner is the sole issuer of a
generation-branded `TextFormalBorrowV1`; the next bounded design names one
private Rust-only batch adapter equivalent to
`issue_text_formal_borrow_from_published_wire_v1(slot, generation)` and feeds
the existing atomic residence/lease owner. A fused C lane-entry is explicitly
not selected: if C is needed, it delegates to the Rust owner and remains only
the existing frame transport projection.
The existing `PinnedTextBackendFrameContractV1` and proof-only
`PinnedTextResidenceExitObligationV1` remain the frame/exit evidence owners;
the lane adapter may borrow those facts but may not reissue them.

Non-authority: sidecar `ValueId` numbers or ordinals as slot/generation data,
MIR metadata or frame-row counts, raw handles/tokens, `DynamicV2CallOutV1`,
`hako_text_formal_validate_v1` status-only validation, `nyash.string.eq_hh`,
C status/frame rows as source issuers, StringBox-to-StableText coercion, and
`TextFormalCallResidenceV1` reinterpreted as source meaning.

Fail-fast boundary: before the first `CheckedCallOut` effect, reject missing or
duplicate ExactText lanes, non-adjacent slot/generation pairs, carrier/owner/
session/entry/segment/occurrence drift, and any attempt to recapture from a
`ValueId`. At runtime, reject zero/out-of-range/missing/stale/foreign/
retiring/non-Text pairs, overflow, or unsupported representation before any
pin or frame publication. A failure leaves no partial pin, token, root row, or
V9 result.

Lifetime boundary: ExactText residence is invocation-scoped and acquired once
at entry; its root `ptr/byte_len` rows are loaded in the preheader and reused
by the loop. V9 `End` is occurrence-scoped and is consumed immediately after
that occurrence's sole TextEq consumer. The rejected design is the
per-iteration `pair -> LeaseSet -> lock -> callback -> finish` loop. Normal
function exits finish the invocation residence before `Return`; recoverable
unwind remains closed until a matching cleanup proof exists.

Smallest next slice: design only the private Rust batch adapter, its connection
to `acquire_text_formal_residence_v1` and
`TextFormalResidenceFrameHeaderV1`, the all-pairs atomic acquire/rollback
contract, and the connection to the existing V9 `NormalResult`/`End`/`Fault`
lifecycle. No wire construction from compiler metadata is permitted. If this
source-bound lane ingress cannot be named precisely, retain the current
`NoSafeSlice`.

Acceptance when selected: one named lane adapter and one runtime issuer;
positive live Text pair; zero/missing/stale/foreign/generation-mismatch,
retiring/non-Text, overflow, and non-adjacent-lane negatives; no partial pin,
frame, or root publication; one invocation-scoped residence finish; one V9 End
consume per normal occurrence; and explicit caller-zero proof before any
production edge. This remains a design-only acceptance list and authorizes no
new semantic `Verified*`/`Prepared*` receipt.

#### COMMON-V2-TEXTEQ-SUBSTRING-V9-EXACTTEXT-LANE-BORROW-INGRESS-D0 closeout (2026-08-18; accepted)

The authority review is closed. The selected implementation is one private
Rust-only batch adapter from published `{slot,generation}` entry lanes into
the existing `TextFormalBorrowV1`/Residence owner. A fused C lane-entry is
rejected; any C projection delegates to the Rust owner and remains frame
transport only. ExactText residence is acquired once per invocation, while
V9 End remains one consume per normal occurrence. The lane adapter may borrow
the existing backend-frame and exit-obligation facts but may not reissue them.

Next selected slice: `COMMON-V2-TEXTEQ-SUBSTRING-V9-EXACTTEXT-LANE-BORROW-INGRESS-I0`.
Its acceptance is limited to the runtime issuer, all-pairs atomic Residence
connection/rollback, focused positive and negative lane tests, and a reusable
guard against raw-pair/ValueId recapture. No MIR effect, TextEq V10, CFG,
publication, production caller, fallback, retry, or `eq_hh` retirement opens.

Non-claims: no residence I0, new semantic receipt, CheckedCallOut emission,
V9 `ValueId`, TextEq/Bool/CFG, publication, fallback, retry, production,
performance promotion, StringBox policy change, or `eq_hh` retirement.

### TEXT-FORMAL-EXACT-STRINGBOX-RESIDENCE-D0 (2026-08-18; audited, production-parked)

Decision: keep the production route `StableText`-only, but allow one bounded
caller-zero runtime canary for the exact built-in `StringBox` before the first
production edge. This is a runtime prerequisite, not a TextEq effect or a
production admission. It must reuse the existing registry-held payload; the
Residence token owns call pins and root descriptors, not a second `Arc` owner.

Source authority + canonical issuer: the existing ExactText formal occurrence
and S6C source/Facts/Recipe relation own Text meaning. The host table alone
validates the published `{slot,generation}`, retirement state, and exact
concrete payload, then issues the physical root residence. A runtime payload
class never becomes a new source meaning.

Memory proof: a call pin prevents removal, free-list reuse, and generation
replacement of the registry payload. For `StableBox`, the table's existing
`Arc<dyn NyashBox>` keeps the heap allocation alive; moving the `Arc` value does
not move the object. The exact built-in `StringBox` has no interior mutation
surface through a shared reference, so its `String` buffer remains stable.
The issuer must use concrete downcast, not spoofable `type_name()` plus
`as_str_fast()`. Neither a pin for an arbitrary payload nor a concrete type
without retirement protection is sufficient alone.

Non-authority: a cloned `Arc`, raw slot/generation, a pin for an unclassified
payload, StringBox name checks, StableText coercion or snapshot copies,
`eq_hh`, C frame rows, MIR `ValueId`, and benchmark success.

Fail-fast boundary: one entry write-lock transaction validates every pair,
checks the exact concrete payload, byte length, retirement state, and all pin
counts, then publishes one token and occurrence-ordered root set. Any error
leaves pins/tokens/roots at zero. Finish consumes the token exactly once; no
fallback, retry, or partial publication is allowed.

Smallest next slice: execute the three runtime rows in order — ABI/frame
limits, lease/root admission split, then the exact-StringBox BoxCount — only
when `CURRENT_STATE.toml` selects the canary. The canary has no source/Facts/
Recipe consumer and does not unblock TextEq V10, inner CFG, publication, or
production cutover by itself.

Non-claims: no StringBox production admission, ABI/raw-pointer publication,
compiler lifecycle, direct leaf, speed result, extra `Arc` root, or `Arc<str>`
migration.

Mutable-reachability census (2026-08-18, workspace scope): the only direct
`as_any_mut` call sites are the boxed-array text mutator and the borrowed
handle-box decoder; neither can reach the registry-held `Arc<dyn NyashBox>`
StringBox allocation. There is no `Arc::get_mut`/`Arc::make_mut`, host-handle
mutable borrow, or writable Arc/raw-pointer projection for StringBox. The
nowait/future paths transfer handles or `Send + Sync` shared objects, not a
mutable registry borrow. C/extern hooks in this repository receive handles or
read-only callbacks; an external unsafe provider that can retain a writable
pointer remains an explicit contract violation and keeps the canary at
`NoSafeSlice` until classified. This census is reusable release evidence, not
an informational grep.

### COMMON-V2-S6C-V9-CALLOUT-MIR-D0 (2026-08-18; design stop)

Decision: retain the source/Facts/Recipe/Join architecture and reset only the
compiler/runtime physical boundary. The concrete-wire V9 issuer remains a
runtime-lifecycle canary; it is not the successor of the common-V2 compiler
admission.

Source authority + canonical issuer: resolver `StringSubstring/2` and Recipe
`CallSlot(item 6, B1, V0, [V6,V8] -> V9:Text)`, plus the existing target,
provider, admission, operand, invocation, and Body-segment proofs. The
canonical CFG/SSA session is the only issuer of `CheckedCallOut`, its
Normal/Fault landings, `CheckedCallOutNormalResult` V9, and
`CheckedCallOutEnd`/`CheckedCallOutFault`.

Non-authority: concrete `DynamicV2CallOutV1`, `EndAuthorizedTextV1`,
`TextFormalWirePairV1`, LeaseSet, a sidecar-`ValueId`-to-`u64` conversion,
`nyash.string.eq_hh`, raw handle equality, and
`with_s6c_substring_v9_issuer` as production compiler evidence.

Fail-fast boundary: before the first MIR mutation, reject source/site/provider
ABI, invocation brand, owner/session/segment, operand/result, or
End-authorized-shape drift. Compile time never receives the runtime wire.
At runtime, Normal defines V9 and later consumes End exactly once after the
callback-scoped V9 consumer; Fault defines no V9 and terminates without End.
Any compiler failure after mutation discards the whole unpublished function.

Smallest next slice: remain design-only and close one callback-scoped
materializer shape for `CheckedCallOut -> NormalResult(V9) -> consumer -> End`
and the separate Fault terminal, including all exit/lifecycle cutpoints. No
code opens until this D0 is accepted.

Non-claims: no implementation, StringBox runtime-pair issuer, residence pin,
TextEq V10/Bool, inner If/Return CFG, Completion/DraftSeal, publication,
production switch, fallback, retry, or legacy retirement.

#### D0 consultation closure (2026-08-18; accepted)

The worker audits close the authority question without changing the source
architecture. The canonical materializer is one private, callback-scoped
consumer of the existing admission: it co-seals the admitted site plan,
source result key, owner/session/Body-segment brand, Normal/Fault landing
blocks, and every source-authorized normal exit cutpoint before the first MIR
mutation. It does not expose a decomposable site-plan/End tuple or accept a
runtime wire.

The only physical writers remain the existing canonical owners:

```text
CanonicalCfgSessionV1
  -> CheckedCallOut and Fault terminal
CanonicalSsaFunctionSessionV2
  -> NormalResult(V9) and End at each verified normal cutpoint
outer unpublished function transaction
  -> rollback for every compiler failure
```

Normal is exactly `CheckedCallOut -> NormalResult(V9) -> source-proven
consumer -> End`; Fault is a successorless `CheckedCallOutFault` with no V9
and no End. End coverage is issued from existing source Join/Completion and
cleanup facts, not inferred from MIR adjacency. The runtime-wire issuer stays
a caller-zero lifecycle canary. A materializer failure rejects before its
first mutation when possible; any late failure discards the whole unpublished
function and never retries the same session.

Accepted smallest next slice: `COMMON-V2-S6C-STRUCTURE-R0`, a behavior-neutral
split that keeps one `VerifiedS6CPrephysicalIngressV2` owner and one
`CommonV2CanonicalSessionRefV1` owner while moving validation/segment helpers
behind private child modules. No new semantic receipt is required.

Non-claims: this closure does not open V9 I0 implementation, TextEq V10,
StringBox residence, inner CFG, Completion/publication, production switch,
fallback, retry, or compatibility retirement.

#### Layer review

| Layer | Verdict | Boundary |
| --- | --- | --- |
| source -> Facts -> Recipe -> Join/Completion | keep; thin | Binary Text equality remains `LoopOperationV2::TextEq`, not a hidden third call |
| prephysical target/admission/operand proofs | keep; thin in responsibility | compile-time only; no runtime wire, handle, or token |
| canonical CFG/SSA writers | reuse | sole MIR mutation and physical `ValueId` authority |
| current V9 concrete-wire issuer | reclassify | caller-zero runtime-contract canary only |
| ExactText wire/LeaseSet/residence | defer | runtime ABI/backend substrate, never compiler semantic authority |
| pinned Text direct lowering | open | performance candidate only after correctness cutover |

The responsibility graph is therefore still one chain:

```text
source Binary Equal / StringSubstring
  -> exact Facts + Recipe CallSlot/TextEq
  -> common target/admission/operand proofs
  -> canonical CheckedCallOut
       Normal -> V9 SSA -> TextEq consumer -> End
       Fault  -> terminal, no V9 and no End
  -> V10 Bool -> inner If/Return
  -> Completion / DraftSeal / atomic publication
```

The correction removes a phase leak; it does not add a second source meaning
or a second physicalizer.

#### Final-state design brief

Decision: land one portable correctness route and its production cutover
first. Only afterward may one source-proven pinned-Text projection replace the
physical work for the exact S6C cohort; it adds no source meaning and keeps no
runtime fallback.

Source authority + canonical issuer: resolver source membership, complete
Facts/Recipe/Join relations, and the canonical session remain the semantic and
CFG/SSA owners. The host registry alone owns physical payload residence; the
LLVM leaf owns only backend-private loads and comparisons.

Non-authority: `nyash.string.eq_hh`, a cloned `Arc`, raw slot/generation or
pointer values, MIR adjacency, V9/V10 ordinals, C frame layout, and benchmark
success.

Fail-fast boundary: compile-time cohort/capability validation and runtime
pair/root validation both finish before their first effect. Later compiler
failure discards the unpublished function; no retry, fallback, partial pin,
or partial publication is allowed.

Smallest next slice: `COMMON-V2-S6C-TEXTEQ-TEXTREF-ENTRY-BRIDGE-D0` remains
the current design boundary for the portable correctness path. Independently, rows 9–11
may be selected as a caller-zero runtime canary prerequisite; rows 12–17 stay
parked until correctness and the first production cutover.

Non-claims: no TextEq V10, inner CFG, Completion/publication, production
switch, StringBox production admission, direct kernel, C-speed result, or
legacy retirement is open now.

#### Final convergence task graph

```text
portable correctness and first production edge
  1 -> 2 -> 3 -> 4 -> 4v9 -> 4b -> 4a -> 5 -> 6 -> 7 -> 8

runtime root foundation and direct physical projection
  8 -> 9 -> 10 -> 11 -> 12 -> 13 -> 14 -> 15 -> 16

independent compatibility retirement
  generic C/Python caller cutover -> 17
```

| # | Row | Kind | Exact exit |
| ---: | --- | --- | --- |
| 1 | `COMMON-V2-S6C-V9-CALLOUT-MIR-D0` | design stop | Accept the callback-scoped `CheckedCallOut -> NormalResult(V9) -> consumer -> End` plus separate terminal Fault design. |
| 2 | `COMMON-V2-S6C-STRUCTURE-R0` | BoxShape | Split the 786-line S6C ingress and 752-line common session before adding orchestration; semantic ownership remains singular. |
| 3 | `COMMON-V2-S6C-V9-CALLOUT-MIR-I0` | BoxShape | Emit canonical Normal/Fault landings, V9, End, and Fault; the concrete-wire canary stays test-only and caller-zero. |
| 4 | `COMMON-V2-S6C-V9-EXACTTEXT-COSEAL-D0/I0` | BoxShape | Co-seal V9 with the adopted ExactText lanes in one session/segment without constructing a runtime pair in the compiler. |
| 4v9 | `COMMON-V2-S6C-TEXTEQ-V9-RUNTIME-PRODUCER-D0/I0` | BoxShape | Use one private provider-return Rust bridge: static producer plan -> move-only runtime result -> opaque scope input; one `EndAuthorizedTextV1` adopter, atomic normal/fault wire, and no post-hoc pairing. |
| 4b | `COMMON-V2-S6C-TEXTEQ-TEXTREF-ENTRY-BRIDGE-D0/I0` | BoxShape | First accept the index-only plan binding source identity, explicit root/lane indices, and cohort stamp; then consume it once to build the published pair vector for the existing Residence owner, with no ordinal rematching, partial pin, or MIR Residence import. |
| 4a | `COMMON-V2-S6C-TEXTEQ-TEXTREF-SCOPE-D0/I0` | BoxShape | After the entry bridge is proven, consume the existing V9/ExactText co-seal through one private opaque scope, with one consumer, one ExactText finish, and canonical V9 End order; no V10 effect. |
| 5 | `COMMON-V2-S6C-PORTABLE-TEXTEQ-V10-D0/I0` | one BoxCount or `NoSafeSlice` | Select one strict non-fallback physical capability for the existing portable TextEq and issue Bool V10. |
| 6 | `COMMON-V2-S6C-INNER-CFG-D0/I0` | BoxShape | Consume V10 with existing Return-read, shared-segment, and FunctionExit proofs to write the inner If/Return CFG. |
| 7 | `COMMON-V2-S6C-CORRECTNESS-CANARY-R0` | evidence | Close positive/negative behavior, exact lifecycle census, and late unpublished-function discard. |
| 8 | `COMMON-V2-S6C-PRODUCTION-EDGE-D0/I0` | production replacement | Connect Completion/DraftSeal/publication, switch the real caller, and retire the old selected edge after caller-zero proof. |
| 9 | `TEXT-FORMAL-RESIDENCE-ABI-LIMIT-GUARD-R0` | BoxShape | Enforce the runtime-owned root/frame maxima in Rust and C entry before allocation or pin; exact-limit positive and over-limit mutation-free negatives are required. |
| 10 | `TEXT-FORMAL-LEASE-ROOT-ADMISSION-SPLIT-R0` | BoxShape | Remove the `stable_text_only` boolean: lease-only acquisition creates no root vector, while root-bearing Residence uses one strict classifier under the same write-lock transaction. |
| 11 | `TEXT-FORMAL-EXACT-STRINGBOX-RESIDENCE-D0/I0` | one BoxCount | Add exactly the built-in `StringBox` by concrete downcast. Keep the registry payload pinned, clone no root `Arc`, copy no bytes, and reject spoofed/string-like boxes before mutation. |
| 12 | `S6C-PINNED-SCALAR-SLICE-CORRIDOR-D0/I0` | BoxShape | Derive non-materialization from the complete Recipe cohort and sole-use relation, never from MIR adjacency; replaced operations must not also dispatch. |
| 13 | `PINNED-TEXT-INVOCATION-LIFECYCLE-D0/I0` | BoxShape | Acquire once at entry, load ptr/len in the preheader, finish before every normal Return, and keep recoverable unwind closed without cleanup proof. |
| 14 | `LLVM-PINNED-TEXT-LEAF-D0/I0` | BoxShape | Lower only `ByteLen`, `Utf8WidthAt`, and scalar-slice equality with width 1..4, exact byte reads, alignment 1, no overread, and no root-to-root `noalias`. |
| 15 | `PINNED-TEXT-PERFORMANCE-PROMOTION-R0` | evidence | Pass the structural zero-boundary before exact, meso, and whole-call C comparisons; benchmark wins cannot waive structure or correctness. |
| 16 | `S6C-PINNED-TEXT-PRODUCTION-SWITCH-I0` | production replacement | Select Direct before effects for the proven cohort, remove its materialized Substring/TextEq physical edge, and retain the portable route only for distinct admitted shapes, never as retry. |
| 17 | `EQ-HH-RETIREMENT-R0` | compatibility retirement | Cut over generic C/Python callers, prove external caller zero, then remove the declaration and `nyash.string.eq_hh` export. |

Rows 1–8 remain the production correctness path. For the current caller-zero
TextRef canary, rows 9–11 may open as a bounded runtime prerequisite before
row 8; this does not select a production caller, publish a function, or relax
the first production cutover. Rows 12–16 remain parked until correctness and
the first production cutover. Each I0 carries focused positive/negative tests,
the owning README/reference update, and one reusable guard where the invariant
cannot be covered by a stable test.

The row-11 acceptance matrix must include StableText preservation, exact
StringBox success, same-root aliases, nested residences, drop-to-Pending,
allocation churn while pinned, stale generation, and a box spoofing the
`StringBox` name. Success has zero new `Arc` clones, zero snapshots, zero body
locks, and one exactly-once finish owner.

Before row 11 may leave D0, one reusable mutable-reachability census must
classify every `as_any_mut` caller, `Arc` uniqueness/recovery path, sanctioned
extern/C provider, nowait/task handoff, and writable raw-pointer projection by
whether it can reach the same registry-held concrete `StringBox` allocation
while pinned. Any unclassified or sanctioned reachable path is `NoSafeSlice`;
copy-distinct objects and explicit unsafe-provider contract violations are
recorded separately and do not become backing-stability authority.
This census is the proof obligation for the theorem that no in-scope path can
reach `&mut StringBox` for that allocation while its residence is live.
The stable executable receipt is
`tools/checks/stringbox_mutable_reachability_census_guard.sh`; it must remain
green whenever this theorem is used as a fast-route prerequisite.

The row-15 structural gate requires zero host-table locks, allocations,
deallocations, callbacks, external/indirect calls, handle publication,
generation checks, LeaseSet operations, and Residence enter/finish inside the
hot loop. Root ptr/len loads occur in the preheader. Initial promotion targets,
on the same target/optimization level with warmup and at least 30 samples, are
ASCII exact-kernel p50 at most 1.10x C, mixed UTF-8 at most 1.15x, 4 KiB-or-
larger meso at most 1.15x, long whole-call at most 1.20x, and p95 at most 1.30x.

#### COMMON-V2-S6C-STRUCTURE-R0 closeout (2026-08-18; accepted)

This behavior-neutral split is complete. `s6c_prephysical_ingress.rs` is now
672 lines and keeps the sole `VerifiedS6CPrephysicalIngressV2` owner; its
source-anchor verifier is a private `s6c_prephysical_ingress_validation.rs`
child. `common_v2_session.rs` is now 392 lines and keeps the sole
`CommonV2CanonicalSessionRefV1` owner; Length and segment/target projections
extend that same type from private child modules. No semantic receipt, Recipe
key, physical ID, MIR effect, runtime wire, or second session was added.

Evidence: `cargo fmt --all`, `tools/checks/common_v2_s6c_structure_guard.sh`,
`CARGO_BUILD_JOBS=4 cargo check --profile quick`,
the exact `prephysical_ingress_seals_exact_source_and_transfer_census` test,
and the exact `shared_segment_scope_threads_length_into_condition_bool` test
are green. The initial short filter that matched zero tests is not counted as
evidence; the full test paths were resolved with `-- --list` and rerun exactly.

Accepted smallest next slice: `COMMON-V2-S6C-V9-CALLOUT-MIR-I0`. It may now
bind the existing canonical CFG/SSA writers through the accepted callback
boundary, while TextEq V10, residence, inner CFG, publication, production,
fallback, retry, and compatibility retirement remain closed.

The independent structural audit remains a separate pre-production backlog:

- `GENERIC-SESSION-SEALED-CONSUME-R0` narrows session-preflight decomposition
  to the canonical opener and replaces duplicate-consume panic with typed
  rejection.
- `CALLABLE-DEMAND-OPAQUE-CONSUME-R0` closes the theoretical owned callback
  tuple escape without changing the semantic program.
- `COMMON-DISPATCHER-ENTRY-RETIREMENT-R0` takes the final target-explicit,
  block-receipt, and segment-dispatch caller census during production cutover.
- `TRANSITION-DEAD-CODE-ALLOW-R0` shrinks transition-only `allow(dead_code)`
  after caller-zero retirement.
- `HOST-HANDLE-GENERATION-LIFECYCLE-R0` migrates new lifecycle ABIs away from
  raw-handle-only release and retires the legacy surface only after its own
  compatibility caller census. It is not a prerequisite for the already
  generation-branded ExactText lane.

#### COMMON-V2-S6C-V9-CALLOUT-MIR-I0 closeout (2026-08-18; accepted)

The canonical compiler-side V9 lifecycle is now implemented as a private
callback-scoped materializer on the existing `CommonV2CanonicalSessionRefV1`.
The source-backed Substring target, checked single-site plan, V6/V8 operand
receipt, Body segment, invocation brand, and physical-entry stamp are
validated before the first callout mutation. The admission's new private
`consume_for_canonical_materializer` moves the site plan with its target and
End obligation; no decomposable site-plan/End tuple can be re-paired.

The materializer reuses the sole canonical writers in this exact order:

```text
CheckedCallOut(source, V0, [V6,V8], normal, fault)
  -> CheckedCallOutFault(fault)             # terminal, no V9/End
  -> CheckedCallOutNormalResult(normal,V9)
  -> callback-scoped V9 consumer
  -> CheckedCallOutEnd(normal, lease_slot=0)
```

The concrete `DynamicV2CallOutV1`/EndAuthorizedText issuer remains a runtime
canary and is not called by the compiler materializer. Late callback or End
failure is rejected through the existing outer unpublished-function
transaction; the session's one-shot state forbids retry. The source result is
the existing CallSlot item 6 / Body block 1 / V9 key 9; TextEq V10 and all
later consumers remain closed.

Evidence: `cargo fmt --all`,
`tools/checks/common_v2_s6c_structure_guard.sh`,
`CARGO_BUILD_JOBS=4 cargo check --profile quick`, and the exact
`s6c_substring_callout_materializer_emits_normal_fault_and_end_once` plus
`s6c_substring_callout_materializer_late_callback_discards_unpublished_function`
tests are green. The positive test observes NormalResult before End and
verifies exactly one CheckedCallOut, one terminal Fault, and one End across the
unpublished function's blocks; the negative test verifies a late callback
rejection leaves no current function/block for publication. Warning output
remains baseline-only; no `--nocapture`, release profile, runtime wire, TextEq,
residence, inner CFG, publication, fallback, retry, production switch, or
`eq_hh` retirement was opened.

#### COMMON-V2-S6C-V9-EXACTTEXT-COSEAL-D0/I0 closeout (2026-08-18; accepted)

The existing source-bound `S6CTextEqOccurrencePhysicalViewV1` is now the sole
ExactText occurrence bridge for the compiler-side V9 lifecycle. The canonical
materializer consumes it in the same `CommonV2CanonicalSessionRefV1` and
`PreparedSegmentBlockReceiptV1` scope as V9. It validates the source-left/V9
key, TextEq result/If-condition relation, owner, logical Body key, and
physical Body block before the first CheckedCallOut mutation.

The callback receives one opaque
`CommonV2SubstringCallOutExactTextCoSealRefV1` containing the NormalResult and
the occurrence/sidecar proof. It exposes neither slot/generation `ValueId`s
nor a runtime wire, and it cannot be split into a free V9 result and a foreign
ExactText row. The existing occurrence one-shot state and the V9 one-shot state
share the outer unpublished-function rollback boundary; callback or End
failure discards the whole draft and never retries.

The lifecycle remains:

```text
CheckedCallOut
  -> terminal Fault (no V9/sidecar consumer)
  -> NormalResult(V9)
  -> callback-scoped V9 + ExactText occurrence co-seal
  -> End
```

Evidence: `cargo fmt --all -- --check`,
`tools/checks/common_v2_s6c_structure_guard.sh`,
`tools/checks/current_state_pointer_guard.sh`,
`CARGO_BUILD_JOBS=4 cargo check --profile quick`, the two exact
`s6c_substring_callout_materializer_*` tests, and the exact
`s6c_occurrence_view_co_seals_needle_with_exact_text_sidecar` test are green.
TextEq V10, residence, inner CFG/Return, Completion/publication, production,
fallback, retry, direct kernel, and `eq_hh` retirement remain closed.

#### COMMON-V2-S6C-PORTABLE-TEXTEQ-V10-D0 (2026-08-18; design stop)

Decision: name exactly one strict backend-neutral capability,
`CommonV2S6CPortableTextEqBoolCapabilityV1`, but keep its MIR effect closed
until its TextRef residence issuer is available. The source meaning is already
closed as `LoopOperationV2::TextEq(V9,V1 -> V10)`, with `NonFaulting` execution.
The current physical vocabulary has no strict TextEq leaf or backend-neutral
content-equality capability: integer `MirInstruction::Compare` cannot compare
Text contents, and `Call`/method dispatch would introduce a second semantic
authority.

Source authority + canonical issuer: the existing resolver
`Equal(Text,Text) -> Bool`, S6C Facts/Recipe, and the callback-scoped V9 plus
ExactText occurrence co-seal remain authoritative. The named capability is a
physical-only contract, not a new source or Recipe product. It consumes one
opaque TextRef for the canonical V9 NormalResult and one opaque TextRef for the
ExactText entry occurrence; it compares exact Unicode scalar sequences, so
same-content/different-handle is true and handle identity is never observed. A
future issuer must consume that co-seal and publish one canonical
`MirType::Bool` through `CanonicalSsaFunctionSessionV2` only after the
TextRef residence producer is named.

Non-authority: raw handle `Compare`, `StringBox::equals`,
`nyash.string.eq_hh`, runtime wire/status, MIR adjacency, V5 outer-loop Bool,
and any fallback/retry. `eq_hh` remains generic C/Python compatibility
substrate and cannot be promoted to the S6C correctness path.

Fail-fast boundary: until a strict TextRef residence issuer exists, reject
before any V10 MIR effect. Do not emit a guessed Bool, source Trap, runtime
pair, raw handle comparison, or second call route. Invalid UTF-8, stale or
foreign residence, unsupported representation, missing/duplicate co-seal, and
owner/session/segment drift remain `RejectBeforeEffect`; late compiler failure
remains outer unpublished-function discard with no retry.

Smallest next design slice: `COMMON-V2-S6C-TEXTEQ-TEXTREF-D0`. Name one
canonical producer for the two opaque TextRefs: V9's End-authorized result and
the entry ExactText occurrence must be co-sealed into one occurrence-ordered
residence scope without exposing raw handles, slot/generation pairs, sidecar
ValueIds, or a runtime wire to the compiler. Only after that producer is
accepted may V10 I0 issue MIR; If/Return CFG, StringBox admission,
production, and `eq_hh` retirement remain closed.

Evidence for the stop: `schema_v2.rs` classifies TextEq as `NonFaulting`,
`MirInstruction::Compare` accepts integer operands, the existing V9 +
ExactText co-seal exposes no runtime TextRef, no strict TextEq physical
instruction/issuer exists in common-V2, and the audited `eq_hh` export is
raw-`i64` hook/fallback transport. This is a representation boundary, not a
runtime failure.

#### COMMON-V2-S6C-TEXTEQ-TEXTREF-D0 (2026-08-18; design stop)

Decision: keep the named TextEq capability and open only its missing
representation boundary. `CommonV2S6CPortableTextEqBoolCapabilityV1` receives
two callback-scoped opaque TextRefs: the V9 NormalResult residence and the
ExactText entry residence. It does not create a new source meaning, Recipe
row, runtime wire, or C status contract.

Source authority + canonical issuer: the existing V9/ExactText callback
co-seal remains the source-bound relation; the future residence owner must be
the sole issuer of the two TextRefs, and the canonical session remains the
sole Bool `ValueId`/`MirType::Bool` issuer. No current code path is allowed to
manufacture either TextRef from a raw `ValueId`, slot, generation, handle, or
MIR adjacency.

Non-authority: `nyash.string.eq_hh`, raw handle equality, `StringBox::equals`,
the StableText-only wire ingress, sidecar numeric lanes, `PinnedTextRootIdV1`
before a residence contract, fallback/retry, and source-level Trap.

Fail-fast boundary: before any V10 effect, reject absent/foreign/duplicate or
stale TextRef residence, non-UTF-8 backing, mismatched V9/ExactText owner,
session, segment, body, or occurrence, and unsupported representation. A late
failure discards the unpublished function; no partial Bool or retry survives.

Residence candidate: one private
`CommonV2S6CTextEqResidenceScopeV1` owns the invocation-scoped ExactText
residence and lends the occurrence-scoped V9 result only inside the existing
callback. It consumes the existing ExactText entry-lane proof and the V9
End-authorized result obligation, but never places V9 into the formal root
array or repins/recaptures it from a raw handle. ExactText roots are admitted
once at entry with occurrence multiplicity; their `ptr/byte_len` rows are
loaded in the preheader and reused by the loop.

The legal lifecycle is split by lifetime:

```text
per V9 occurrence: TextEq leaf completes -> canonical V9 End consumes
normal function exit: ExactText residence.finish -> Return
```

The rejected design is a per-iteration residence acquire/lock/callback/finish
cycle. Any fault/unwind-capable path remains closed until the matching
invocation cleanup proof is present; there is no retry or second finish owner.
The row-11 mutable-reachability census is a mandatory acceptance input for the
future exact StringBox fast route; it is already recorded in this SSOT and must
be reused rather than duplicated.

#### COMMON-V2-S6C-TEXTEQ-V9-RUNTIME-PRODUCER-D0 (2026-08-18; design stop)

Decision: keep `NoSafeSlice::SourceBoundV9RuntimeProducerUnsealed`. The
canonical V9 `NormalResult`/`End` pair is already a compile-time physical
product, but the source-bound runtime owner that turns its normal host-handle
and lease result into `EndAuthorizedTextV1` has not been named. Do not bridge
that gap by importing `TextFormalCallResidenceV1` into MIR or by pairing two
runtime owners after the fact.

Source authority + canonical issuer: resolver `StringSubstring/2`, S6C
Facts/Recipe `CallSlot(item 6, B1, V0, [V6,V8] -> V9:Text)`, the existing
same-cohort V9/ExactText occurrence co-seal, and the checked site plan's
`EndAuthorizedHandle { lease_slot }` shape. The canonical session remains the
sole physical `ValueId`/type issuer. A future source-bound producer must be
the only issuer that binds the canonical normal result, its End obligation,
and the runtime `EndAuthorizedTextV1` owner for this exact site/occurrence.

Non-authority: `DynamicV2CallOutV1` as a compiler input, raw handle/token or
slot/generation recapture, `TextFormalCallResidenceV1` imported into MIR,
`with_text` on a runtime canary, sidecar `ValueId` reinterpretation,
`nyash.string.eq_hh`, raw handle equality, and any MIR adjacency heuristic.

Fail-fast boundary: before the first callout effect, reject wrong source/item/
block/result, missing or duplicate End obligation, ImmediateI64 or non-READ
site shape, foreign owner/session/segment/brand, lease-slot drift, unsupported
runtime representation, and absent source-bound provider. At runtime, a
normal result must validate as exact live Text plus its matching End lease in
one owner transaction; no partial `EndAuthorizedTextV1`, residence root, or
V9 capability may escape. Fault has no V9 and no End; late compiler failure
discards the unpublished function and never retries.

Smallest next slice: name the private source-bound producer contract only:
its input co-seal, normal-result/End binding, runtime owner handoff, primary /
suppressed cleanup order, and one consumer. It must explicitly state whether
the producer lives in the canonical backend/runtime boundary or a private
bridge, without adding a new source meaning or public C export.

Acceptance/non-claims: one issuer, one runtime owner, exact site/occurrence
binding, live/stale/foreign/duplicate/unsupported negatives, and no post-hoc
pairing. TextRef scope I0, TextEq V10/Bool, CFG/Return, Completion/publication,
production, direct leaf, fallback, retry, and `eq_hh` retirement remain
closed.

##### TextRef residence D0 audit decision (2026-08-18; accepted design boundary)

Decision: keep `design_stop` and name
`CommonV2S6CTextEqResidenceScopeV1` as the one-shot owner for the existing
V9/ExactText pair; do not implement it in this row.

Source authority + canonical issuer: S6C `StringSubstring/2` and TextEq
Facts/Recipe plus the existing occurrence co-seal. The private scope co-seals
the V9 End obligation with ExactText V1 residence; the canonical session alone
may later issue Bool/V10.

Non-authority: V9 `ValueId`, MIR adjacency, sidecar slot/generation, raw
handle/token, `eq_hh`, StableText wire alone, or `TextFormalCallResidenceV1`
reinterpreted as source meaning.

Fail-fast boundary: before residence/leaf effect, reject owner/session/segment/
body/occurrence drift, stale/foreign/duplicate/unsupported roots, or absent V9
and ExactText correspondence. ExactText entry acquisition and finish are
exactly once per invocation; V9 End is exactly once per normal occurrence.

Smallest next slice: specify only the private scope API, rollback owner,
opaque roots `[V9, ExactText]`, one consumer, one finish owner, and primary /
suppressed error order. Keep V10, additional CheckedCallOut/CFG, publication,
production, fallback, retry, and `eq_hh` retirement closed.

Non-claims: no V10 MIR effect, `MirInstruction::PinnedTextOp`, StringBox
runtime production admission, inner CFG/Return,
publication, production switch, fallback, retry, performance result, or
`eq_hh` retirement.

The exact concrete StringBox root admission is the only runtime prerequisite
opened before the production edge. It remains a caller-zero canary and must
reuse the registry payload under one write-lock validation/pin transaction;
there is no copied root `Arc`, byte snapshot, whole-function lock, or mutable
alias escape. The existing row-11 mutable-reachability census is a release
gate, not an informational grep.

##### TextRef residence D0 decision closure (2026-08-18; accepted)

The smallest accepted shape is one private
`CommonV2S6CTextEqResidenceScopeV1` owned by the existing common-V2 session.
It is a mechanical co-seal of already-issued products, not a new source or
Recipe authority:

```text
S6C V9/ExactText occurrence co-seal
  + issued V9 End-authorized lifetime
  + one entry ExactText Residence
  -> one move-only TextEqResidenceScope
  -> one callback-scoped [V9Ref, ExactTextRootsRef] view
```

The scope must consume, rather than reconstruct, the existing V9
NormalResult/End obligation and ExactText entry-lane proof. It may not accept
raw `ValueId`, sidecar slot/generation, handle/token, runtime wire, or MIR
adjacency. The canonical session remains the sole Bool `ValueId` issuer and
the existing runtime Residence remains the sole pin/root/finish owner.

The private API contract is deliberately one-way:

```text
admit(co-seal, issued-v9, exacttext-entry)
  -> scope.with_text_refs(|opaque_v9, opaque_exact| consumer)
  -> scope.finish_exacttext()
  -> existing canonical V9 End consume
```

`with_text_refs` is the sole consumer and cannot return either root or V9
capability. ExactText roots are invocation-scoped and occurrence-ordered;
V9 is occurrence-scoped and is never inserted into the formal root array.
The only legal normal order is:

```text
TextEq leaf completes
  -> ExactText residence.finish()
  -> canonical V9 End consume
```

Preflight rejects owner/session/segment/body/occurrence drift, absent or
duplicate co-seal, stale/foreign/retiring/non-Text roots, unsupported backing,
and any second consumer before the first effect. If the consumer fails, the
consumer error is primary and residence cleanup is suppressed evidence; if
the consumer succeeds, residence finish precedes End and its failure is
primary. An End failure after successful finish is primary. There is no
implicit Drop cleanup, retry, fallback, or second finish owner until an
unwind/noexcept proof is separately accepted; the outer unpublished function
transaction remains the compiler rollback boundary.

Acceptance: one private scope owner, one consumer, one finish owner, opaque
`[V9, ExactText]` views only, exact primary/suppressed error order, and
negative coverage for foreign/duplicate/stale/unsupported/late failure. No
V10 Bool, additional CheckedCallOut, CFG/Return, Completion/publication,
production, direct leaf, performance, or `eq_hh` retirement is opened.

The proposed `COMMON-V2-S6C-TEXTEQ-TEXTREF-SCOPE-I0` remains parked: a
source-bound V9 runtime producer must first prove the relation between the
canonical NormalResult/End obligation and the runtime `EndAuthorizedTextV1`.
Importing `TextFormalCallResidenceV1` into MIR or pairing the two existing
runtime owners after the fact would violate the authority chain. The next
design boundary is therefore
`COMMON-V2-S6C-TEXTEQ-V9-RUNTIME-PRODUCER-D0`; only after it is accepted may
the private scope API/test row open.

##### COMMON-V2-S6C-TEXTEQ-V9-RUNTIME-PRODUCER-D0 decision closure (2026-08-18; accepted)

The source-bound runtime producer is fixed as one private Rust bridge at the
exact provider-return boundary. The compiler keeps the existing canonical
`CheckedCallOut -> NormalResult(V9) -> End` lifecycle; it does not import a
runtime owner or reinterpret a `ValueId`.

The private products are deliberately three-layered:

```text
SourceBoundV9RuntimeProducerPlanV1
  -> provider call at the exact checked site
  -> SourceBoundV9RuntimeResultV1
  -> SourceBoundV9RuntimeInputRefV1<'_> (borrow only)
```

`SourceBoundV9RuntimeProducerPlanV1` is backend-private static evidence for the
same source/item/block/result, cohort/owner/session/segment brand, fixed
provider symbol/ABI/arity, `READ` effect, `EndAuthorizedHandle` shape, lease
slot, and canonical End obligation. It owns no `ValueId`, raw handle/token,
TextFormalResidence, or side table. The plan is consumed at one call site and
cannot be rebuilt from an ordinal, JSON, or MIR adjacency.

`SourceBoundV9RuntimeResultV1` is move-only and is the sole runtime owner that
may call `EndAuthorizedTextV1::adopt`. It contains the call-local opaque
NormalResult lane and the adopted End-authorized owner, but exposes neither a
handle/token tuple nor a lookup API. Its only public operations are a
callback-scoped `with_input` and the terminal `finish_at_canonical_end` /
`abort_on_terminal_failure` paths. Scope-I0 receives only
`SourceBoundV9RuntimeInputRefV1<'_>`; no runtime wire or residence owner
escapes the bridge.

The admitted provider is the fixed source-bound `hako.text.scan.substring.v1`
entry. Its Rust implementation must construct the complete normal or fault
wire atomically: a normal result is written only after the End lease has been
issued, and a fault carries no lease. A malformed/partial output from an
unadmitted provider is a terminal provider-contract violation; the bridge
must never guess a token or clean a foreign lease. This keeps cleanup safe and
avoids a second generic abort authority.

Fail-fast is split at the only two boundaries. Before provider effect, verify
source item/block/result, cohort/owner/session/segment, provider ABI/arity,
`READ`, EndAuthorized shape, lease slot, and the canonical End census. Before
publishing the result owner, require `Normal`, `HostHandle`, `EndAuthorized`,
forwarded-none, continuation-zero, reserved-zero, non-zero handle/token,
matching live generation, and exact live Text. Fault, Suspended, ImmediateI64,
Forwarded, foreign/stale token, and non-Text payload reject without V9 or
scope input.

The normal chronology is fixed:

```text
provider -> wire validation -> EndAuthorizedTextV1::adopt
  -> scope-I0 consumer -> ExactText residence.finish
  -> canonical End -> SourceBoundV9RuntimeResultV1::finish_at_canonical_end
```

Provider Fault has no V9 and no End owner. Consumer failure is primary and
terminal; cleanup failure is suppressed evidence. Finish failure never retries,
falls back, or creates a second owner. The existing unpublished-function
transaction remains the compiler rollback boundary. The old
`issue_s6c_substring_v9_from_wire_v1` path remains a caller-zero canary and is
not promoted to the source-bound issuer.

Acceptance for the next I0 is one private producer/one `adopt` caller, exact
site/occurrence binding, atomic normal/fault provider output, live/stale/
foreign/duplicate/unsupported/malformed negatives, no raw tuple or side-table
pairing, and exact primary/suppressed cleanup order. TextRef scope I0,
TextEq V10, CFG/Return, publication, production, direct leaf, fallback,
retry, C-speed, and `eq_hh` retirement remain closed.

The next selected row is
`COMMON-V2-S6C-TEXTEQ-V9-RUNTIME-PRODUCER-I0`; only after that row is green may
`COMMON-V2-S6C-TEXTEQ-TEXTREF-ENTRY-BRIDGE-D0` specify the runtime root
mapping that a later scope I0 may consume.

#### COMMON-V2-S6C-TEXTEQ-V9-RUNTIME-PRODUCER-I0 closeout (2026-08-18; accepted)

The caller-zero canary is implemented in the runtime-private
`source_bound_v9_runtime` child. One bridge validates the fixed provider-return
wire and is the only `EndAuthorizedTextV1::adopt` caller for this path. Its
move-only result lends only callback-scoped text input and owns explicit
finish/abort; no raw handle/token tuple, MIR `ValueId`, residence, side table,
fallback, or retry escapes. The old wire issuer remains a canary and no
production selector changed.

Evidence: `cargo fmt --all`; `CARGO_BUILD_JOBS=4 cargo test --profile quick
--lib source_bound_v9_runtime` (7 passed / 0 failed); the Dynamic lease suite
(7 passed / 0 failed); the exact S6C issuer suite (3 passed / 0 failed);
`CARGO_BUILD_JOBS=4 cargo check --profile quick`; the current-state, S6C
structure, and StringBox mutable-reachability guards; and `git diff --check`.
The first broad issuer filter returned zero tests and was discarded; the
issuer tests were rerun by their exact discovered names. Warnings remain
baseline-only.

Non-claims: the provider remains caller-zero; TextRef scope, TextEq V10,
CFG/Return, publication, production, direct leaf, C-speed, fallback/retry,
and `eq_hh` retirement remain closed. The next design stop is
`COMMON-V2-S6C-TEXTEQ-TEXTREF-ENTRY-BRIDGE-D0`.

#### COMMON-V2-S6C-TEXTEQ-TEXTREF-ENTRY-BRIDGE-D0 (2026-08-18; reopened design stop)

Decision: keep TextRef scope I0 parked until one runtime-private entry bridge
binds the source ExactText occurrence cohort to the published runtime pairs
and the invocation Residence root indices. Do not import Residence into MIR,
issue V10, or infer the mapping from an ordinal or `ValueId`.

Source authority + canonical issuer: the existing S6C ExactText sidecar and
V9/ExactText occurrence co-seal own source order and multiplicity; the runtime
`acquire_text_formal_residence_from_published_wires_v1` path remains the sole
pin/root/finish owner. The bridge is only a mechanical Rust handoff between
those already-issued products. Its single physical-plan issuer must consume
the `ResolvedCallablePhysicalSignatureLoanV1` together with the
`PhysicalTextEntryLaneSidecarV1`, validate every slot/generation lane index,
and emit the batch rows once; runtime values are read from those exact lanes,
never rematched by ordinal or raw pair order.

Non-authority: bare source ordinals, `ValueId` numbers, MIR adjacency, raw
handles/tokens, JSON, `ptr/len` values, `nyash.string.eq_hh`, or a later scan
that re-pairs rows by position. `PinnedTextRootViewRef` is an opaque,
callback-borrowed view; it is not a backing-stability or source-meaning issuer.

Fail-fast boundary: before any residence effect, require exact source/runtime
pair count and occurrence order, one owner/session/segment/body cohort, an
explicit published `{slot,generation}` binding for every root index, live
non-retiring concrete Text payloads, representable lengths, and no duplicate
or foreign pair. Any mismatch rejects with zero published root rows and no
partial pin. A root view or pointer/length projection may not escape its
callback; the current `with_root` byte-length-only API does not authorize a
content-comparison leaf.

Smallest next slice: define one private bridge product and one consumer that
accepts one opaque `ExactTextEntryBatch` containing the source binding,
explicit `root_index`, exact slot/generation lane indices, the physical cohort
stamp, and the already-published pair values. The batch is issued once from
the source/ExactText physical-entry contract, then consumed once by the
existing Residence owner; it must not be rebuilt from a bare ordinal or pair
position. Record primary/suppressed cleanup order and negative coverage for
missing, stale, foreign, duplicate, reordered, retiring, and overflow input.

The required counterexample is part of the D0 proof: if source roots are
`[Subject, Needle]` but two live published pairs arrive as `[Needle, Subject]`,
the old pair-only Residence adapter succeeds while root 1 silently changes
meaning. The explicit `root_index`/lane-index batch must reject this mismatch
before pinning; successful Residence acquisition alone is not evidence of a
correct source binding.

Non-claims: no TextRef scope implementation, V10 Bool, pinned-text leaf,
additional MIR/CFG/Return, publication, production switch, direct C-speed
route, fallback/retry, or `eq_hh` retirement.

#### COMMON-V2-S6C-TEXTEQ-TEXTREF-ENTRY-BRIDGE-D0 closure (2026-08-18; accepted)

The D0 is closed as a BoxShape. The bridge is an index-only private physical
plan, not a semantic receipt and not a runtime owner:

```text
S6C occurrence/co-seal + PhysicalTextEntryLaneSidecarV1
  -> CanonicalSsaFunctionSessionV2 issues one private bridge plan
  -> exact slot/generation lane indices + explicit root_index bijection
  -> callback consumer builds the already-published pair vector
  -> runtime text_formal_abi validates the pairs
  -> TextFormalCallResidenceV1 owns pin/root/finish
```

The plan carries only cohort/stamp, owner/entry/segment/body, binding and
occurrence identity, slot/generation lane indices, published-pair index,
root index, carrier, and exact counts. It carries no concrete pair values,
`ValueId` to `u64` reinterpretation, handle/token, runtime wire, or source
meaning. The source/entry issuer must validate adjacent lanes, unique rows,
explicit pair-to-root bijection, owner/session/segment/brand/carrier parity,
and root count/index coverage before any Residence effect.

The selected I0 may pass the plan-produced pair vector to the existing
`acquire_text_formal_residence_from_published_wires_v1` adapter, but the
adapter may not rematch rows or infer source meaning. Runtime rejects
zero/missing/stale/foreign/duplicate/reordered/retiring/overflow pairs in
the existing atomic transaction; partial pin/frame publication remains
forbidden. The old pair-only adapter is not sufficient without the plan,
because two live pairs in reversed order can otherwise silently swap roots.

Acceptance for the next I0 is one private plan issuer, one consumer, explicit
root-index/lane-index coverage, all negative mapping cases, zero partial
rollback, one Residence acquisition/finish per invocation, and the existing
V9 End exactly-once evidence. No TextRef scope, V10 Bool, MIR/CFG/Return,
publication, production switch, fallback/retry, performance claim, or
`eq_hh` retirement is opened.

#### COMMON-V2-S6C-TEXTEQ-TEXTREF-ENTRY-BRIDGE-I0 (2026-08-18; selected fast row)

Implement exactly one private physical-plan issuer and one consumer. The
issuer consumes the existing S6C co-seal, `ResolvedCallablePhysicalSignatureLoanV1`,
and `PhysicalTextEntryLaneSidecarV1`; it emits only index/stamp rows. The
consumer validates the plan once, reads the already-published slot and
generation values from the exact lane indices, and passes a root-index-ordered
pair vector to `acquire_text_formal_residence_from_published_wires_v1`.

The focused gate must cover positive one-root and multi-root cases plus
missing/duplicate/reordered lane rows, owner/session/entry/segment drift,
carrier mismatch, root-index gaps, zero/stale/foreign/retiring/non-Text pairs,
overflow, and mutation-free rollback. The consumer exposes no raw tuple,
`ValueId`, handle/token, or ordinal lookup; Residence remains the sole pin,
root, and finish owner. This is caller-zero physical evidence, not a
production switch.

#### COMMON-V2-S6C-TEXTEQ-TEXTREF-ENTRY-BRIDGE-I0 closeout (2026-08-18; accepted)

The private index-only plan and one-shot lane consumer are implemented in
`common_v2_s6c_textref_entry_bridge.rs` (currently 358 lines). Its occurrence
entry point first checks the existing S6C occurrence/co-seal against the
sidecar owner, entry, binding, and logical ordinal; the sidecar issuer then
validates monotonic source rows, adjacent `U64BitsOnI64` lanes, owner-branded
bindings, and explicit root/pair indices. It consumes runtime lane values
exactly once to lend a root-index-ordered pair batch. It does not import
Residence, emit MIR, or expose raw runtime identity.

Evidence: `cargo fmt --all`; `CARGO_BUILD_JOBS=4 cargo test --profile quick
--lib common_v2_s6c_textref_entry_bridge` (4 passed / 0 failed);
`CARGO_BUILD_JOBS=4 cargo check --profile quick`; the current state pointer
and StringBox mutable-reachability guards; and `git diff --check`. The focused
negatives cover non-monotonic ordinals, foreign bindings, non-adjacent lanes,
short input, and zero runtime pair values; the existing S6C occurrence suite
covers the source/co-seal itself.
Warnings remain baseline-only. The existing runtime Residence and V9 End
owners are unchanged. TextRef scope, V10 Bool, CFG/Return, publication,
production, direct leaf, C-speed, fallback/retry, and `eq_hh` retirement
remain closed; the next design stop is
`COMMON-V2-S6C-TEXTEQ-TEXTREF-SCOPE-D0`.

#### COMMON-V2-S6C-TEXTEQ-TEXTREF-SCOPE-I0 closeout (2026-08-18; accepted)

The runtime-private `TextEqResidenceScopeV1` is now a one-shot move-only
owner for the already-produced source-bound V9 result and one invocation
ExactText residence. It lends only a callback-scoped opaque V9 text view and
occurrence-ordered root view; it emits no MIR and performs no TextEq compare.
The callback result is recorded before cleanup, then ExactText `finish` runs
before canonical V9 End finish. Callback failure is primary, while cleanup
failures are retained as suppressed evidence. There is no implicit Drop
cleanup, retry, fallback, or second consumer.

Evidence: `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
text_eq_residence_scope` (2 passed / 0 failed); `cargo fmt --all`; and
`git diff --check`. This is caller-zero runtime evidence only. The source
co-seal remains the admission authority; the runtime scope does not recreate
owner/session/segment meaning and does not import Residence into MIR. TextEq
V10, CFG/Return, publication, production, direct leaf, C-speed, fallback,
retry, and `eq_hh` retirement remain closed; the next design row is
`COMMON-V2-S6C-PORTABLE-TEXTEQ-V10-D0`.

#### TEXT-FORMAL-RESIDENCE-ABI-LIMIT-GUARD-R0 closeout (2026-08-18; accepted)

The Residence ABI maxima are now enforced at every runtime entry before any
pair Vec materialization or host-table pin. The compile-time ABI view,
Rust-owned Residence acquisition, and C frame entry all share the same
root-count/frame-size boundary; an over-limit request is mutation-free and
cannot publish a token or root row. The guard is representation-only and does
not widen the StableText classifier or create a TextRef.

Evidence: `cargo fmt --all`,
`CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
text_formal_residence` (9 passed / 0 failed), and
`tools/checks/current_state_pointer_guard.sh` are green. The focused test
covers the exact maximum ABI size, over-limit C rejection before pinning,
stale/overlap/frame negatives, occurrence ordering, and exactly-once finish.
Warnings are baseline-only; no `--nocapture`, release profile, StringBox
admission, TextRef, TextEq V10, fallback, retry, or production switch opened.

Accepted next slice: `TEXT-FORMAL-LEASE-ROOT-ADMISSION-SPLIT-R0`. It must keep
one exact payload classifier, allocate root descriptors only for the
root-bearing Residence path, and preserve the same atomic validation/pin and
rollback boundary.

#### TEXT-FORMAL-LEASE-ROOT-ADMISSION-SPLIT-R0 closeout (2026-08-18; accepted)

The call-lifetime owner now has one shared write-lock validation/pin
transaction, without the former `stable_text_only` mode flag. Lease-only
acquisition validates the exact formal payload (including the existing
StringBox formal borrow) but creates no root vector or pointer rows. The
root-bearing Residence path allocates occurrence-ordered rows once at entry
and applies the strict `StableText` root classifier under that same
transaction. Any classifier, generation, retirement, length, or pin failure
still rejects before token publication; finish remains the single release
owner.

Evidence: `cargo fmt --all`,
`CARGO_BUILD_JOBS=4 cargo test --profile quick --lib text_formal` (19 passed /
0 failed), `CARGO_BUILD_JOBS=4 cargo check --profile quick`, and
`tools/checks/current_state_pointer_guard.sh` are green. The focused tests
cover exact StringBox lease acceptance, StableText-only Residence rejection,
duplicate/nested pins, stale generations, pending retirement, overflow, and
exactly-once finish. Warnings remain baseline-only; no concrete StringBox root
admission, TextRef, TextEq V10, fallback, retry, publication, or production
switch opened.

Accepted next slice: `TEXT-FORMAL-EXACT-STRINGBOX-RESIDENCE-D0/I0`. It may
replace only the strict root classifier with a concrete built-in downcast,
after the recorded mutable-reachability census; it must not widen the formal
borrow API or add a copied backing owner.

#### TEXT-FORMAL-EXACT-STRINGBOX-RESIDENCE-D0/I0 closeout (2026-08-18; accepted)

The runtime canary now admits the concrete built-in `StringBox` by
`as_any().downcast_ref::<StringBox>()`; `type_name()` and `as_str_fast()` are
not residence authority. The same entry transaction validates the exact
formal payload, generation, retirement state, byte length, and pin counts
before publishing roots. It keeps the registry-held payload alive, creates no
root `Arc` clone or byte snapshot, and holds no lock in the body. A dropped
StringBox remains pending until the one residence finish owner releases the
pin, so slot allocation cannot churn into the live root.

The focused runtime evidence is green: `cargo fmt --all`,
`CARGO_BUILD_JOBS=4 cargo test --profile quick --lib text_formal` (20 passed /
0 failed), and `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib stringbox`
(14 passed / 1 ignored / 0 failed). The tests cover concrete residence,
StableText preservation, aliases, stale generations, pending retirement,
frame/ABI limits, spoofed StringBox names, and slot-reuse blocking. Warnings
remain baseline-only. The reusable mutable-reachability census classifies all
workspace `as_any_mut`, Arc uniqueness/recovery, extern/C, nowait/task, and
writable raw-pointer paths; no sanctioned path reaches the registry-held
StringBox while pinned, and an unclassified external unsafe provider remains
`NoSafeSlice`.

Rows 9-11 are therefore closed as caller-zero runtime prerequisites. The
accepted next boundary is `COMMON-V2-S6C-TEXTEQ-TEXTREF-D0`: name the existing
V9/V1 source-bound TextRef residence producer and its finish order before
issuing V10. No TextRef, V10, fallback, retry, publication, production, direct
leaf, or performance claim is opened by this canary.

#### C-speed and legacy verdict

`nyash.string.eq_hh` is old for the S6C TextEq design, but it is not dead:
generic ny-llvmc and Python compatibility callers still use it. Its hook,
fallback, raw-`i64` result, and lossy invalid-handle behavior disqualify it as
the correctness authority. Keep it as a measured legacy baseline until the
later caller-zero retirement row.

The per-iteration `pair -> LeaseSet -> callback -> finish` path is rejected.
The thinnest candidate is one entry transaction plus a pin-owned registry
payload and exact concrete StringBox classification: no extra root `Arc`, no
whole-function registry guard, and no O(bytes) snapshot. Entry/finish
allocation or lock cost is measured separately and optimized only if whole-
call evidence names it as hot. The full S6C cursor cohort, not V9/V10
adjacency, authorizes direct lowering. C-like kernel speed is plausible;
whole-call parity remains a release-LTO IR/assembly and exact/meso/whole
measurement claim.

### TEXT-FORMAL-WIRE-INGRESS-I0 (2026-08-18; accepted and closed)

Decision: open one runtime-only, behavior-preserving ingress for an already
published StableText `{slot,generation}` pair. It validates the existing host
generation table and returns the existing private `TextFormalWirePairV1`; it
does not create source meaning, a MIR value, a residence pin, or a TextEq
operand. This is a BoxShape transport slice, not a production switch.

Source authority + canonical issuer: the host-handle generation table and its
StableText payload classifier are the sole runtime authorities. The issuer is
`runtime::text_formal_abi`; the future common-V2 occurrence view is its only
named consumer. The canonical physical session and ExactText sidecar remain
compile-time owners and are not consulted by this runtime helper.

Non-authority: raw handles and generation recapture, MIR `ValueId`, logical
ordinal, physical sidecar rows, `PinnedTextBackendFrameContractV1`, C
`eq_hh`/status exports, StringBox-to-StableText conversion, fallback/retry,
and `TextFormalCallResidenceV1` pin or root ownership.

Fail-fast boundary: reject zero/out-of-range or missing slots, zero or stale
generations, non-StableText payloads (including StringBox), and any attempted
raw-pair escape before a caller can acquire residence. Existing exact-text
borrow and residence APIs are unchanged; no partial resource is acquired.

Closeout evidence: the narrow private issuer and focused positive/negative
tests are landed. `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
text_formal_abi` is green (7/7), `CARGO_BUILD_JOBS=4 RUSTFLAGS=-Awarnings
cargo check --profile quick -q` is green, and formatter, diff, pointer,
physical-transfer, and TextScan authority guards are green. Cargo runs were
serialized with `CARGO_BUILD_JOBS=4` and did not use `--nocapture` or release
LTO. Do not add the common-V2 occurrence view, TextFormal residence,
CheckedCallOut, V9 `ValueId`, TextEq/Bool/CFG, publication, production,
fallback, retry, or legacy retirement.

Acceptance/non-claims: one issuer, one future consumer, no raw tuple or
semantic receipt escape, StableText positive, zero/stale/missing/foreign and
StringBox negatives, and source under 800 lines. The next design boundary is
the source-bound common-V2 occurrence co-seal; this I0 does not claim it.

### COMMON-V2-TEXTEQ-SUBSTRING-V9-EXACTTEXT-LANE-BORROW-INGRESS-I0 closeout (2026-08-18; accepted)

Decision: the selected lane-borrow row is closed as a runtime-only BoxShape.
`runtime::text_formal_abi` now issues a move-only formal from an already
published `{slot,generation}` lane without recapturing a raw handle or
generation. `text_formal_residence` immediately converts the batch to the
existing invocation-scoped Residence owner; the owner performs the one
all-pairs write-lock validation/pin/root transaction and remains the sole
finish owner. No lock, LeaseSet, allocation, callback, or residence entry is
introduced in the loop body.

Source authority + canonical issuer: S6C StringSubstring/2 and Binary
Equal(Text,Text) Facts/Recipe plus the ExactText entry sidecar authorize the
future source-bound consumer. The current adapter is deliberately runtime
private; host-handle generation, retirement, exact concrete Text
classification, and `TextFormalCallResidenceV1` are the only issuers used by
this row. It emits no semantic receipt, MIR `ValueId`, C status row, or
TextEq meaning.

Fail-fast boundary: zero, missing, stale, foreign, retiring, non-Text,
overflow, or generation-mismatched lanes reject before any residence token,
root row, or frame is published. Final all-pairs acquisition is atomic, so a
late lane failure cannot leave a partial pin. Normal finish remains explicit
and consuming; fallback, retry, and C-provider dispatch are absent.

Acceptance evidence: `cargo fmt --all`; `CARGO_BUILD_JOBS=4 cargo test
--profile quick --lib text_formal_abi` (9 passed / 0 failed);
`CARGO_BUILD_JOBS=4 cargo test --profile quick --lib text_formal_residence`
(12 passed / 0 failed); `CARGO_BUILD_JOBS=4 cargo check --profile quick`; and
the current-state pointer, physical-transfer, TextScan admission, and pinned
backend-frame transport guards are green. The focused tests cover live
StableText and concrete StringBox lanes, zero/stale generations, one
invocation Residence with ordered roots, and stale rejection before pinning.
The existing Residence suite continues to cover spoofed StringBox names,
stale pairs, aliases, frame limits, rollback, and exactly-once finish.

The row-11 mutable-reachability census is an explicit acceptance input, not
an informal assumption: every workspace `as_any_mut` caller, `Arc`
uniqueness/recovery path, extern/C provider, nowait/task sharing path, and
writable raw-pointer projection must be classified for reachability to the
same registry-held concrete `StringBox` while pinned. The repository census
finds no sanctioned path; an unclassified external unsafe provider remains
`NoSafeSlice` and cannot authorize a production fast route.
`tools/checks/stringbox_mutable_reachability_census_guard.sh` is the reusable
static receipt for the direct-caller and Arc-recovery portion of that census.

Non-claims: no source-bound TextRef producer, TextEq V10 Bool, inner CFG,
Completion/publication, production switch, direct pinned-text leaf,
performance result, fallback/retry, or `nyash.string.eq_hh` retirement.
The next design boundary is `COMMON-V2-S6C-TEXTEQ-TEXTREF-D0`.
