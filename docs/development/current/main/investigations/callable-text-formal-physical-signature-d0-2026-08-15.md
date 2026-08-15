---
Status: accepted BoxShape; active caller-zero implementation brief
Date: 2026-08-16
Work mode: fast
Classification: T2 BoxShape accepted; next T2 BoxCount is caller-zero
Parent: LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
---

# CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0

The runtime can atomically pin and retire a correct ExactText pair set. This
Decision now fixes the compiler-owned callable signature as two explicit
scalar lanes and keeps the later root residence, slice/cursor, backend
projection, and route policy outside that signature.

## Six-line brief

```text
Decision: accept logical ExactText as one formal/BindingRef mapped to two contiguous physical u64 lanes [slot,generation], while every ordinary scalar maps to one lane; logical /N and physical_formal_lane_count remain separate authorities, and the 16-byte aggregate ABI is rejected.
Source authority + canonical issuer: same-brand selected/batch callable identity plus the complete callable-parameter contract cohort are consumed by one new package-owned VerifiedCallablePhysicalSignatureCohortV1 issuer; it owns the total ordinal-to-lane map and never consumes Completion.
Non-authority: /N suffix, MirType::String, FunctionSignature alone, raw Vec<ValueId>, runtime validator argument order, TextFormalBorrowV1, Completion/header rows, AST names, Recipe keys, Dynamic leases, fallback, and retry.
Fail-fast boundary: reject missing/duplicate logical ordinal, lane gap/overlap/swap/out-of-range, foreign brand/target, logical/physical count conflation, detached pair lanes, legacy one-to-one skeleton/call projection, or any need to infer generation from a raw slot.
Smallest next slice: CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-I0 issues and transports the complete caller-zero package mapping, including one combined Installed S6C loan; skeleton/call-edge/session consumers remain later rows.
Non-claims: no call-site actualization, C/LLVM activation, session ValueId, entry acquire/root projection, Completion epilogue, Text slice/cursor, Trap lowering, TextEq route, production caller, main integration, fallback, or retry.
```

## Target product

```rust
#[derive(Debug)]
pub(crate) struct VerifiedCallablePhysicalSignatureCohortV1 {
    // non-Clone; package-owned; fields private
    // complete same-brand callable rows
}

pub(crate) enum PhysicalFormalLaneRoleV1 {
    OrdinaryScalar,
    ExactTextSlot,
    ExactTextGeneration,
}
```

Each callable row must close:

```text
selected callable identity / catalog brand
logical_arity
physical_formal_lane_count
complete logical formal ordinal set
complete/disjoint physical lane index set

ordinary scalar ordinal -> [OrdinaryScalar]
ExactText ordinal       -> [ExactTextSlot, ExactTextGeneration]
```

Lane order is deterministic:

```text
logical formal ordinal order
  ordinary -> one lane
  ExactText -> slot immediately followed by generation
```

The product contains no `ValueId`, `BasicBlockId`, runtime token, source call
site, Completion, root residence, slice, pointer, length, or route policy.

## Boundary after the signature

The two-lane wire is the stable callable boundary, not the function-internal
Text representation. Later owners must preserve this one-way split:

```text
ExactText logical formal
  -> [slot, generation] physical signature
  -> atomic callee-entry lease-set
  -> non-splittable TextFormalCallResidenceSetV1
       lease-set token + PinnedTextRootResidenceV1[]
  -> session-branded TextSliceRefV1 / backend-local TextPlan
  -> scoped backend ptr/len projection only
```

`PinnedTextRootResidenceV1` identifies one immutable valid-UTF-8 root while
the enclosing residence set owns its lifetime. `TextSliceRefV1` is only a
bounded range over such a root with a UTF-8/code-point boundary receipt.
`TextPlan` remains the existing transient non-Box carrier. Raw `ptr,len` is a
backend projection; it is never the lifetime owner, callable ABI, BindingRef,
or independently storable common product.

Production entry consumes the already-published two lanes directly. It must
not call the probe issuer that reconstructs a generation from a raw handle.
The landed `TextFormalBorrowV1` remains validator/test evidence, not the
production call actualizer.

## Required owner fan-out

One package-owned row must be borrowed, never reconstructed, by three later
mechanical consumers:

```text
physical-signature row
  ├─ mapping-aware callable skeleton/publication
  ├─ post-install exact call-edge argument expansion
  └─ Canonical callee composite-formal adoption
```

The post-install call-edge issuer is distinct from the signature issuer. It
must co-seal the whole-source exact static target inventory, Installed Port,
caller original-formal/no-rebind proof, and the callee signature row. It may
project lanes but cannot change their meaning or order.

The future Installed Port must use one total exactly-once child loan. The
current S6C child and Main static-child loans consume the same selected key on
separate surfaces, so they cannot be composed by calling both. The current I0
adds one combined S6C arm that lends selected input, ExactText contracts,
package-owned S6C child, and the signature row in one HRTB callback. The later
exact call-edge issuer must consume/extend that same scoped arm rather than
open a second selected-key loan. Ordinary and Dynamic roles remain separate.

## Canonical callee boundary

The future Canonical consumer receives one signature row and derives both
physical parameter `ValueId`s from the already-created physical function
parameter list:

```text
one logical BindingRef
  -> slot ValueId: ordinary Text carrier
  -> generation ValueId: private sidecar only
```

Only the slot lane is published to ordinary Binding SSA. Generation is never
an independent binding and cannot be recovered from `MirType`, raw slot, or
ordinary SSA reads. A scoped composite forward view is required for nested
calls.

## Acceptance

```text
one package-owned signature cohort issuer
Completion/header dependency = 0
logical arity and physical lane count named separately
complete/disjoint ordinal and lane coverage
ExactText [slot,generation] adjacency and role tags exact
ordinary scalar behavior preserved
ValueId / Builder / MIR = 0 in the signature product
one future combined Installed Port seam named
one future mapping-aware skeleton consumer named
one future exact call-edge consumer named
one future composite Canonical adoption consumer named
root residence / slice / ptr-len dependency = 0 in signature issuer
V1/Dynamic adapter = 0
fallback/retry = 0
production caller = 0
```

## NoSafeSlice

Keep:

```text
NoSafeSlice::MissingTextFormalCallableSignatureIssuer
```

if any safe design requires Completion to issue formal lanes, `/N` or
`FunctionSignature` to infer physical count, caller-supplied batch/key/header,
separate slot/generation products, two independently consumable Installed
loans for one S6C key, raw-handle generation recapture, a V2-to-V1 adapter,
S6C-specific physicalizer, root/slice/pointer state in the signature product,
Builder/session inference, fallback, or retry.

## Active implementation brief

```text
Change:
  issue one non-Clone package-owned physical-signature cohort from the selected/batch identity and complete parameter-contract cohort; transport the same rows through install and one combined S6C Port loan; retire the independently consumable S6C-only signature gap.
Contract:
  ordinary scalar = one lane; ExactText = adjacent [slot,generation]; logical ordinal/BindingRef and physical lane indices are complete/disjoint; Completion, ValueId, residence, ptr/len, call edge, and route policy stay out.
Done:
  focused ordinary/ExactText/mixed positive rows; missing/duplicate/foreign/lane-gap-overlap-swap negatives; non-Clone/private-constructor/caller-zero guard; package README and ABI SSOT synchronized.
Stop:
  return to NoSafeSlice if the issuer needs header/Completion, raw function signature inference, detached lane products, two Port consumptions for one S6C key, or any Builder/runtime/production caller.
```

## Ordered successor families

After this accepted Decision:

```text
CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-I0
  caller-zero package mapping and transport

TEXT-FORMAL-PINNED-RESIDENCE-D0/I0
  ordered internal seams, not separate authority cards:
    post-install exact target/origin/signature call edge
      original-formal/no-rebind first
    exact StableText or canonical StringBox backing proof
    pair-based entry acquire + occurrence-ordered pinned UTF-8 root projection
      in one transaction and one compiler-runtime private residence frame
    Canonical session-private residence/root adoption
    function-local access-plan namespace and thin lifetime-closure seal
    Completion-backed capability-only DraftSeal per-exit finish coverage
    exact source literal as a distinct cutover subrow; result/Substring/
      copy/PHI/unknown still reject

LOOP-TEXT-SLICE-EXECUTION-D0/I0
  ordered internal seams:
    pinned root -> CP-correct transient slice
    generic sequential code-point cursor
    one narrow plan-keyed PinnedTextOp MIR variant with ByteLen / Utf8WidthAt /
      Utf8ScalarSliceEqWholeText leaf kinds
    MIR JSON + primary-AOT direct address/load/small-compare consumer

LOOP-TEXT-ROUTE-PERF-R0
  exact / meso / whole evidence
  static admitted route; runtime fallback/retry = 0
```

These are two bounded implementation families after the signature row, not a
new card per type. The runtime lease I0 remains a substrate only. No family
may claim a production callable route until the common V2 envelope, admitted
route, residence, Completion epilogue, and canonical session meet at one edge.

The executable order stays intentionally small:

```text
current: CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-I0
  package lane map + combined Installed loan only

next D0: TEXT-FORMAL-PINNED-RESIDENCE-D0
  accept the phase-owned authority spine, exact backing, compiler-runtime
  frame ABI, root-occurrence mapping, lifetime closure, and capability-only
  DraftSeal seam as one BoxShape

next I0 family: TEXT-FORMAL-PINNED-RESIDENCE-I0
  exact formal call edge -> atomic residence frame -> Canonical private state
  -> lifetime closure -> per-exit finish projection; literal origin remains
  the final cutover subrow and does not block the formal-only kernel

then: LOOP-TEXT-SLICE-EXECUTION-D0/I0
  sequential CP-cursor proof -> one PinnedTextOp variant/three leaves -> final
  census -> MIR JSON -> primary-AOT direct consumer and structural negatives
```

The residence D0 does not become accepted merely because this card records a
candidate frame. It must close every named `NoSafeSlice` below before its I0;
the current signature I0 neither implements nor issues any of those products.

## Read-only fast-path audit

The 2026-08-16 code/performance audit confirms that the intended pinned-root
route can target a C-like hot kernel, but no such production route or measured
keeper exists yet. The current generic Substring/TextEq route may cross helper,
registry/lock, object/handle birth, and publication boundaries on each loop
iteration. The admitted target instead keeps all of those at zero inside the
loop and lowers only a valid-UTF-8 code-point cursor plus a direct one-to-four
byte equality leaf.

`LOOP-TEXT-SLICE-EXECUTION-I0` must therefore include one primary-AOT backend
consumer that turns the verified cursor/slice product into direct address,
load, and small-compare code. A target-neutral proof without that consumer is
not a speed keeper. `LOOP-TEXT-ROUTE-PERF-R0` must split entry/exit, equality
leaf, UTF-8 cursor, S6C kernel, and whole-call measurements; inspect generated
IR/assembly; and require per-iteration calls, locks, allocations, handle/Box
births, publication, retain/release, and environment reads to be zero. Kernel
and whole-call C ratios remain separate because entry/exit lease cost may
dominate short inputs. Route admission is target-stamped and static; missing
backing, boundary, backend, or performance proof rejects before effect with no
fallback or retry.

### Complexity decision

The later fast route uses a small typed MIR extension, not a second
physicalizer and not a large Text dialect. The bounded target is one
`PinnedTextOp` instruction family with three leaf kinds: byte length, UTF-8
width at a proven cursor, and one-scalar slice equality against a whole Text
root. Existing integer operations, comparison, branch, PHI, and block
placement remain owned by common MIR and the Canonical session. Entry acquire
and finish remain in the pinned-residence lifecycle family rather than being
re-owned by the leaf dialect.

Each leaf carries one function-local `PinnedTextAccessPlanIdV1`. The ID alone
is never authority: it indexes a function-owned plan table co-sealed by the
Canonical session from the residence root, session brand, operation kind and
operand arity, cursor/boundary proof, dominating range check, and pre-finish
use interval. The backend consumes that table entry and never reconstructs
safety from nearby comparisons, block order, variable names, or `ValueId`
shape. V1 does not publish a generic TextSlice MIR value; the verified slice
relation is consumed when the leaf site plan is issued.

The three leaf result contracts are fixed and infallible after their plan is
verified:

```text
ByteLen                         -> i64 in 0..=i64::MAX
Utf8WidthAt                     -> i64 in 1..=4
Utf8ScalarSliceEqWholeText      -> Bool under the exact Text equality law
```

The first primary-AOT consumer loads exactly the selected one-to-four bytes.
It does not use a wider unaligned load, overread, SIMD, or `memcmp` contract.
Those are later measured optimizations, not implicit rights of the leaf.

The table and constructors stay private, carry a function/invocation stamp,
and close an exact one-plan-to-one-instruction census before JSON emission.
Missing, duplicate, orphan, foreign-stamp, kind/root/operand/site mismatch, or
a decoder that tries to reissue authority from a numeric ID rejects the
unpublished function.

All three `PinnedTextOp` leaves have fixed physical `EffectMask::READ`; the
instruction has no caller-supplied effect field. Semantic Text equality
remains governed by the language law. READ prevents treating a backing access
as an unconstrained pure scalar operation, but it is still moveable and is not
the lifetime proof. The residence lifecycle target gives entry
`WRITE + Barrier + Control` and finish `WRITE + Barrier`, then a final
function-local census rejects use before the normal entry landing, use on the
trap landing, use after finish, foreign-plan/root/lease pairing, a normal exit
without exactly one finish, or a trap/fault exit with a finish. Per-iteration
Text leaves do not carry Barrier themselves.

Generic `Load`/`Store` stays closed because it would expose an unsafe pointer
capability without the root, UTF-8-boundary, or lease-dominance proof. FastMem
`MemOp` stays allocator/layout-owned and is not reused as Text authority. A
whole-loop fused opcode is also rejected because it would hide CFG/SSA/PHI in
a profile physicalizer. The backend may reuse mechanical GEP/load techniques,
but never the FastMem region or layout receipt.

The signature row, not `/N`, `len(params)`, or `len(args)`, must drive future
function declaration, exact call expansion, and callee adoption. The current
LLVM path has more than one arity heuristic, so preserving the logical `/2`
name while creating four physical lanes requires all three consumers to borrow
the same package-owned mapping.

### Residence and exit corrections

One function authority spine does not mean one durable struct spanning every
phase. A proposed all-in-one `VerifiedPinnedTextFunctionPlanV1` that owns the
pre-session call edge, live session roots/accesses, target admission, and
Completion finish rows is rejected: it would either retain session-local
physical state outside `CanonicalSsaFunctionSessionV2` or duplicate the
Completion exit ledger.

The phase-preserving shape is:

```text
PreparedTextFormalCallActualizationV1
  pre-session; exact target/signature/origin; no physical IDs
    -> Canonical session-private PinnedTextFunctionStateV1<'session>
         one residence, occurrence-ordered roots, access-plan table
    -> VerifiedPinnedTextLifetimeClosureV1
         thin move-only function/session stamp + closed lifecycle census
    -> PreparedTextFormalExitFinishSetV1
         capability only; no copied exit rows
    -> private PreparedFunctionExitPlanV1
         owns the capability beside the existing PreparedFunctionExitSetV1
    -> same detached exit iteration emits finish immediately before Return
```

`PinnedTextFunctionStateV1` is private protocol state of the sole Canonical
session, not a second semantic or CFG owner. The lifetime-closure receipt does
not re-own roots, accesses, target policy, blocks, sites, or Completion; it
only proves that the state it closes belongs to the same function/session and
that the final census passed. Responsibility-specific scoped projections may
borrow the live state, but no caller can obtain detached residence, access,
or finish parts and re-pair them.

The final lifetime census mechanically verifies that the successful entry
landing dominates every `PinnedTextOp`, every referenced root and plan belongs
to this session, every access precedes its planned finish, and no root/frame/
pointer carrier escapes through a store, return, or foreign call. It also
seals one function-level finish capability without enumerating exits. The
existing DraftSeal exit iteration later checks that capability while emitting
one finish at every admitted normal Completion exit. A noreturn/no-unwind trap
edge has no finish, each return operand is complete before finish, and no
planned Text access follows finish. `EffectMask::READ` is optimizer metadata;
it is not a substitute for this census.

The closure receipt and finish capability store no source site, Completion
claim, exit block, return value, source order, or independent exit count. The
private DraftSeal preparation seam moves the capability into
`PreparedFunctionExitPlanV1`, where the already-existing
`PreparedFunctionExitSetV1` remains the sole per-exit ledger. Exact per-exit
coverage is established only by consuming both in that one projection loop.

The residence frame is concrete but is not part of the Hakorune callable ABI.
It is a versioned compiler-runtime private physical ABI: selected primary AOT
allocates one invocation-local stack frame, the runtime entry transaction
validates and pins every published `[slot,generation]` pair and fills the whole
frame under the same registry write lock, and only the AOT lowering context
projects raw residence-descriptor pointers and lengths from it. Common MIR/JSON
exposes root IDs, plan IDs, and ordinary verified leaf results such as
`ByteLen`; it never exposes the frame address, opaque token, raw pointer, or
raw descriptor length.

The later Residence D0 must accept or reject this deliberately narrow V1
candidate; it is not an issued authority or implementation permission for the
current signature I0:

```text
64-bit primary AOT only
frame header: size 32, align 8
              revision u32, header size u32, total frame size u32,
              root count u32, opaque lease token u64, reserved u64
root row:     size 16, align 8; ptr bits u64, byte length u64
frame size:   checked 32 + 16 * root_count
root count:   ExactText formal occurrence count from the physical signature
root order:   logical formal occurrence order
```

Compile-time target admission checks primary AOT, 64-bit pointer width, ABI
revision, entry/finish symbol availability, and no-unwind before MIR emission.
The runtime entry separately checks the concrete frame pointer, byte size,
alignment, and root count before pin or frame publication. Other backends are
`RejectBeforeEffect` in V1; they do not lower to a generic String helper,
FastMem, or scalar retry. The raw frame is only transport; the runtime token
table and Canonical session state remain the lifetime authorities.

Runtime pin records may group identical pairs for checked multiplicity, but
the frame and Canonical state keep one root row per ExactText formal occurrence.
Thus `f(text, text)` owns two pins and two root rows even when both rows contain
the same pointer/length. Distinct root IDs never imply `noalias`, and the
backend must not deduplicate them or recover formal order from the grouped pin
table.

The session-private residence table co-seals an exact one-to-one mapping from
each `PinnedTextRootIdV1` to its occurrence-ordered frame row index. Access
plans reference only those branded root IDs; the backend resolves the row
through this table and cannot treat a numeric root ID as an offset or rebuild
the relation from pointer equality.

The first pinned backing domain is deliberately exact: registry-owned
`StableText(String)` and an exact downcast to the canonical immutable
`StringBox`. A `type_name() == "StringBox"` check, generic `as_str_fast`,
`StringViewBox`, or fail-open `StringSpan` cannot issue the backing proof.
Root descriptors and the lease-set token are emitted by one pair-based entry
transaction and remain non-splittable; common MIR observes only opaque root
and lease-set IDs, never an independently storable raw `ptr,len` pair.

The current `Main.main` fixture passes string literals, while the accepted
actual-origin partition admits only an original ExactText formal candidate.
Therefore the kernel may be measured through an explicitly benchmark-only
host pair lane before production cutover, but that probe cannot prove the
source call edge. Production requires a distinct exact-source-literal origin
arm, co-sealed with its call argument site, selected target, callee signature,
catalog brand, and literal lifetime. It must not be disguised as the formal
arm or reconstructed from a MIR constant.

The first implementation retains DraftSeal's existing exact exit-set model.
The residence/session owner co-closes one private
`PreparedTextFormalExitFinishSetV1` capability with the existing Completion
consumption. This thin product stores no copied source site, block, `ValueId`,
source order, or Completion claim. The private preparation seam moves it into
`PreparedFunctionExitPlanV1` beside the existing prepared exit set, and the
detached projection consumes both in the same iteration: after each return
operand is ready, it emits one finish immediately before that exit's Return.
`ReadyFunctionDraftSealV1` remains the current Completion/current-block owner;
DraftSeal does not infer that a lease is required and does not select a token
from MIR state.

A synthetic shared epilogue/return PHI is not part of this route because it
would require new Canonical CFG/PHI authority and the MIRBuilder north star
explicitly forbids creating such a join merely to merge source Returns. A
future measured need may open a separate Decision; DraftSeal never infers that
join itself.

Future named stops include missing exact backing, literal origin, root/token
coupling, UTF-8 cursor proof, primary-AOT leaf consumption, per-exit finish
coverage, no-unwind Trap, and static target-stamped performance admission.
Numeric C-ratio thresholds are owned by the later perf row rather than this
signature contract.

The successor family additionally stops with:

```text
NoSafeSlice::CrossPhasePinnedPlanWouldOwnSession
NoSafeSlice::PinnedResidenceEscapesCanonicalSession
NoSafeSlice::PinnedPlanDuplicatesCompletionExitLedger
NoSafeSlice::MissingCompilerRuntimeResidenceAbiOwner
NoSafeSlice::FrameLayoutOrRevisionUnsealed
NoSafeSlice::PhysicalRootCountDerivedFromUniquePairs
NoSafeSlice::AliasedRootRowsMarkedNoAlias
NoSafeSlice::PartialFramePublishedOnAcquireReject
NoSafeSlice::PinnedRootIdDetachedFromFrameRow
NoSafeSlice::PinnedTextLifetimeClosureIncomplete
NoSafeSlice::NonPrimaryBackendRequiresFallback
```

These stops refine the later `TEXT-FORMAL-PINNED-RESIDENCE` and
`LOOP-TEXT-SLICE-EXECUTION` families only. They do not widen or block the
current package-owned physical-signature mapping I0.

The current fast-path review is read-only because stable root backing,
CP-cursor lowering, and performance admission are future authorities, not
permissions to widen the active signature I0.
