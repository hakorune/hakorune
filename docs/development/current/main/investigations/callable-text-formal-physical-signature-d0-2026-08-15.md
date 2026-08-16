---
Status: Residence D0 BoxShape accepted; next caller-zero Residence I0
Date: 2026-08-16
Work mode: fast
Classification: T2 BoxShape accepted; T2 caller-zero BoxCount next
Parent: LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
---

# CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0

The runtime can atomically pin and retire a correct ExactText pair set. This
Decision now fixes the compiler-owned callable signature as two explicit
scalar lanes and keeps the later root residence, slice/cursor, backend
projection, and route policy outside that signature.

## Six-line brief

```text
Decision: accept one leading scalar u64 InstanceReceiver lane exactly for an InstanceBoxMethod, then map each logical ExactText formal/BindingRef to two contiguous physical u64 lanes [slot,generation] and every ordinary explicit formal to one lane; source logical /N, receiver-lane count, expanded physical-formal-lane count, and total physical-callable-lane count remain separate authorities, and the 16-byte aggregate ABI is rejected.
Source authority + canonical issuer: same-brand selected/batch callable identity, declaration mode, exact source Receiver binding when present, and the complete explicit callable-parameter contract cohort are consumed by one package-owned VerifiedCallablePhysicalSignatureCohortV1 issuer; it owns the total receiver-plus-explicit lane map and never consumes Completion.
Non-authority: /N suffix, params/function-params length difference, MirType::String, FunctionSignature alone, raw Vec<ValueId>, runtime validator argument order, TextFormalBorrowV1, Completion/header rows, AST names, Recipe keys, Dynamic leases, fallback, and retry.
Fail-fast boundary: reject missing/duplicate/foreign receiver, receiver lane on a static callable, missing instance receiver lane, missing/duplicate logical ordinal, lane gap/overlap/swap/out-of-range, foreign brand/target, logical/receiver/physical count conflation, detached pair lanes, legacy one-to-one skeleton/call projection, or any need to infer receiver or generation from lengths/raw slots.
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

pub(crate) enum PhysicalCallableLaneRoleV1 {
    InstanceReceiver,
    OrdinaryScalar,
    ExactTextSlot,
    ExactTextGeneration,
}
```

Each callable row must close:

```text
selected callable identity / catalog brand
declaration mode + optional exact Receiver binding
source_logical_arity = explicit source formal count (/N)
receiver_lane_count = 1 iff InstanceBoxMethod, otherwise 0
physical_formal_lane_count = sum(explicit-formal lane widths)
physical_callable_lane_count
complete logical formal ordinal set
complete/disjoint physical lane index set

InstanceBoxMethod receiver -> [InstanceReceiver]
ordinary scalar ordinal -> [OrdinaryScalar]
ExactText ordinal       -> [ExactTextSlot, ExactTextGeneration]

physical_callable_lane_count
  = receiver_lane_count + physical_formal_lane_count
```

Lane order is deterministic:

```text
optional InstanceReceiver first
then logical explicit-formal ordinal order
  ordinary -> one lane
  ExactText -> slot immediately followed by generation
```

The receiver is a distinct source binding from
`SourceBindingSiteV1::Receiver`; it is not an ExactText formal, does not
consume an explicit formal ordinal, and cannot be reconstructed from a
parameter-count difference. Static callables have no receiver row or lane.

The four-axis count is also the module-wide ABI census boundary. A later
projection may expose callee parameter `ValueId`s or caller argument
`ValueId`s, but those are function-local carriers: callee lane `ValueId`s are
pairwise distinct, while caller occurrence rows preserve order and may repeat
one `ValueId` for aliasing occurrences (`f(text,text)` or receiver/argument
alias). Caller and callee `ValueId` numbers are never compared as identity.

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
  ├─ mapping-aware skeleton
  │    -> PreparedCallablePhysicalParameterListV1
  │         ordered lane role + callee parameter ValueId
  ├─ post-install exact call edge
  │    -> PreparedCallablePhysicalArgumentListV1
  │         same lane order + caller argument ValueId
  └─ Canonical callee adoption
       -> validates the prepared callee list against the same signature row
```

These projections are function-local mechanical lists, not shared `ValueId`
authority and never a replacement for the logical callable signature. Before
module publication, one ABI census requires the definition, every declaration,
and every direct call of the same symbol to carry the same physical-signature
ABI revision, receiver disposition, lane count, and lane order. Reusing an
older prototype or mixing logical-arity and physical-arity call edges is a
typed reject, never a `len(args)` repair.

Callee parameter `ValueId`s are pairwise distinct per physical lane. Caller
argument rows preserve occurrence order but may repeat one `ValueId` when
source occurrences alias, as in `f(text,text)`; receiver/argument alias is also
not `noalias` evidence. Caller and callee `ValueId` numbers belong to different
function scopes and are never compared as identity.

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
InstanceBoxMethod source Receiver
  -> leading receiver ValueId and existing receiver BindingRef

one logical BindingRef
  -> slot ValueId: ordinary Text carrier
  -> generation ValueId: private sidecar only
```

Only the slot lane is published to ordinary Binding SSA. Generation is never
an independent binding and cannot be recovered from `MirType`, raw slot, or
ordinary SSA reads. Receiver adoption remains distinct from explicit-formal
ordinal adoption. A scoped composite forward view is required for nested calls.

## Acceptance

```text
one package-owned signature cohort issuer
Completion/header dependency = 0
source logical arity, receiver lane count, physical formal lane count, and total physical callable lane count named separately
InstanceBoxMethod has one exact leading InstanceReceiver lane
StaticBoxMethod receiver lane count = 0
receiver source BindingRef/owner/mode exact; len-difference inference = 0
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
NoSafeSlice::ImplicitReceiverPhysicalLaneUnsealed
```

if any safe design requires Completion to issue formal lanes, `/N` or
`FunctionSignature` to infer physical count, caller-supplied batch/key/header,
separate slot/generation products, two independently consumable Installed
loans for one S6C key, receiver inference from parameter counts, raw-handle
generation recapture, a V2-to-V1 adapter,
S6C-specific physicalizer, root/slice/pointer state in the signature product,
Builder/session inference, fallback, or retry.

## Completed signature I0 brief

```text
Change:
  issue one non-Clone package-owned physical-signature cohort from the selected/batch identity, declaration mode/exact Receiver binding, and complete explicit parameter-contract cohort; transport the same rows through install and one combined S6C Port loan; retire the independently consumable S6C-only signature gap.
Contract:
  InstanceBoxMethod = one leading receiver lane; StaticBoxMethod = none; ordinary explicit scalar = one lane; ExactText = adjacent [slot,generation]; receiver/logical ordinal/BindingRef and physical lane indices are complete/disjoint; Completion, ValueId, residence, ptr/len, call edge, and route policy stay out.
Done:
  focused static/instance, ordinary/ExactText/mixed positive rows; missing/duplicate/foreign receiver and lane-gap/overlap/swap negatives; non-Clone/private-constructor/caller-zero guard; package README and ABI SSOT synchronized.
Stop:
  return to NoSafeSlice if the issuer needs header/Completion, receiver or physical-count inference from function/parameter lengths, detached lane products, two Port consumptions for one S6C key, or any Builder/runtime/production caller.
```

## Next implementation brief: Residence I0

```text
Change:
  add only the caller-zero StableText entry/frame transaction and its move-only residence handle for already-published [slot,generation] pairs; keep all compiler/session/loop consumers closed.
Contract:
  occurrence-ordered all-or-nothing validation, pin, stable UTF-8 root projection, private frame fill, and one opaque residence handle; no partial output, pointer escape, or raw-token exposure.
Done:
  valid one/two-formal and same-pair multiplicity positives; zero/missing/stale/non-Text/retiring/overflow, frame mismatch/non-overlap, partial-acquire, duplicate-finish, and post-retirement negatives; StableText-only README/test receipt.
Stop:
  return to NoSafeSlice::ResidenceI0ScopeWidened if StringBox, literal/call-edge origin, MIR ValueId, session CFG, PinnedTextOp, another backend, fallback, retry, or production selection is required.
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
    registry-owned StableText or dedicated CanonicalStringBox payload proof;
      no virtual Box method under the registry lock
    pair-based entry acquire + occurrence-ordered pinned UTF-8 root projection
      in one transaction and one compiler-runtime private residence frame
    Canonical session-private residence/root adoption
    narrow checked residence-enter terminator + effectful finish marker
    function-local access-plan namespace and prepared lifetime closure
    Completion-backed capability-only DraftSeal per-exit finish coverage,
      then optimizer/postprocess final lifetime verification
    exact source literal as a distinct cutover subrow; result/Substring/
      copy/PHI/unknown still reject

LOOP-TEXT-SLICE-EXECUTION-D0/I0
  ordered internal seams:
    pinned root -> CP-correct transient slice
    generic sequential code-point cursor
    one narrow plan-keyed PinnedTextOp MIR variant with ByteLen / Utf8WidthAt /
      Utf8ScalarSliceEqWholeText leaf kinds
    MIR JSON + current ny-llvmc(boundary pure-first) direct
      address/load/small-compare consumer

LOOP-TEXT-ROUTE-PERF-R0
  exact / meso / whole evidence
  static admitted route; runtime fallback/retry = 0
```

These are two bounded implementation families after the signature row, not a
new card per type. The runtime lease I0 remains a substrate only. No family
may claim a production callable route until the common V2 envelope, admitted
route, residence, Completion epilogue, and canonical session meet at one edge.

## Residence D0 acceptance (2026-08-16)

```text
Decision: accept a StableText-only, 64-bit `ny-llvmc(boundary pure-first)` Residence BoxShape: one package-owned physical-signature row drives one occurrence-ordered atomic entry transaction, one private residence frame, one session-branded root table, and one Completion-backed normal-exit finish capability; StringBox, literal origin, call edge, and production route remain separate cutover rows.
Source authority + canonical issuer: the package physical-signature cohort supplies logical formal occurrence/order; the registry owns StableText backing, generation validation, pin/retirement, and frame fill; the Canonical session owns opaque residence/root/access IDs; Completion owns normal exits; the final MIR verifier is the only post-transform lifetime seal.
Non-authority: raw `ptr,len`, raw slot/generation, `TextFormalBorrowV1`, type names or virtual `as_str_fast`, backend metadata, generic Load/Store, `ValueId` shape, source-site copies, Completion cleanup, benchmarks, environment, fallback, and retry.
Fail-fast boundary: reject non-StableText backing, frame revision/size/count/alignment/non-overlap drift, partial acquire or frame publication, root/plan/lease detachment, pointer escape, use after finish, missing/duplicate/foreign normal-exit finish, any effect after finish before Return, unwind/catch, or non-primary backend fallback.
Smallest next slice: `TEXT-FORMAL-PINNED-RESIDENCE-I0` implements only the caller-zero registry-owned entry/frame transaction and move-only residence handle for already-published pairs; it does not emit call edges, MIR ValueIds, Text leaves, session CFG, or production callers.
Non-claims: no StringBox canonical publisher, literal/call-result/Substring origin, CP cursor, `PinnedTextOp`, AOT loop consumer, common V2/session fan-in, route admission, performance keeper, fallback/retry, or main integration.
```

The accepted BoxShape closes the authority direction and fail-fast boundary,
but it does not claim that the compiler or runtime implementation exists. The
next I0 remains caller-zero and StableText-only until a later Decision admits
another backing class.

The executable order stays intentionally small:

```text
landed: CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-I0
  package lane map + combined Installed loan only

current I0: TEXT-FORMAL-PINNED-RESIDENCE-I0
  StableText-only registry entry/frame transaction + move-only residence handle
  for already-published pairs; caller-zero and no compiler/session consumer

Residence cutover seams (later, not this caller-zero I0):
  exact formal call edge -> atomic residence frame -> Canonical private state
  -> prepared closure -> per-exit finish projection -> final lifetime verifier;
  literal origin remains the final cutover subrow and does not block the
  formal-only kernel

next D0 after Residence I0: LOOP-TEXT-SLICE-EXECUTION-D0
  sequential CP-cursor proof -> one PinnedTextOp variant/three leaves -> final
  census -> MIR JSON -> current `ny-llvmc(boundary pure-first)` direct consumer
  and structural negatives; `llvm_py` remains a non-production keep lane
```

The accepted Residence D0 is deliberately narrow. Its contracts, the
package-owned S6C child, and its matching physical-signature row remain
scoped to one combined loan; the next I0 must close the named runtime/frame
tests without opening any later owner. The current signature I0 neither
implements nor issues those residence products.

## Read-only fast-path audit

The 2026-08-16 code/performance audit confirms that the intended pinned-root
route can target a C-like hot kernel, but no such production route or measured
keeper exists yet. The current generic Substring/TextEq route may cross helper,
registry/lock, object/handle birth, and publication boundaries on each loop
iteration. The admitted target instead keeps all of those at zero inside the
loop and lowers only a valid-UTF-8 code-point cursor plus a direct one-to-four
byte equality leaf.

`LOOP-TEXT-SLICE-EXECUTION-I0` must therefore include exactly one production
backend consumer that turns the verified cursor/slice product into direct
address, load, and small-compare code. For the current lane that owner is
`ny-llvmc(boundary pure-first)`; `llvm_py` remains a compatibility/keep lane,
and the future Hako LLVM-text owner does not receive a duplicate lowering in
this row. A target-neutral proof without the selected consumer is not a speed
keeper. `LOOP-TEXT-ROUTE-PERF-R0` must split entry/exit, equality leaf, UTF-8
cursor, S6C kernel, and whole-call measurements; inspect generated IR/assembly;
and require per-iteration calls, locks, allocations, handle/Box births,
publication, retain/release, and environment reads to be zero. Kernel and
whole-call C ratios remain separate because entry/exit lease cost may dominate
short inputs. Route admission is target-stamped and static; missing backing,
boundary, backend, or performance proof rejects before effect with no fallback
or retry.

### Complexity decision

The later fast route uses a small typed MIR extension, not a second
physicalizer and not a large Text dialect. The bounded target is one
`PinnedTextOp` instruction family with three leaf kinds: byte length, UTF-8
width at a proven cursor, and one-scalar slice equality against a whole Text
root. Existing integer operations, comparison, branch, PHI, and block
placement remain owned by common MIR and the Canonical session. Entry acquire
and finish remain in the pinned-residence lifecycle family rather than being
re-owned by the leaf dialect.

That lifecycle still needs executable typed carriers; backend-only metadata
must not synthesize hidden semantic CFG. The Residence D0 target is one narrow
checked `PinnedTextResidenceEnterV1 { plan, normal, trap }` terminator and one
effectful `PinnedTextResidenceFinishV1 { residence }` marker. Enter alone owns
the normal/trap split and publishes roots only on the normal landing. Finish
identifies the exact sole residence through an opaque function/session-branded
`TextFormalResidenceIdV1`; it never exposes the runtime token or frame address.
These two protocol carriers are not additional Text computation leaves, and
their effects/edges are fixed by the issuer rather than caller supplied.

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

The first `ny-llvmc(boundary pure-first)` consumer loads exactly the selected
one-to-four bytes.
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
`WRITE + Barrier + Control` and finish `WRITE + Barrier + Panic`. Finish is an
ordinary effectful marker and therefore owns no CFG `Control`; only a future
checked-terminator form would add it. The session/DraftSeal path first prepares
the lifecycle closure, then every optimizer/postprocess transform completes,
and the final MIR verifier rechecks and seals it before JSON/backend emission.
That final census rejects use before the normal entry landing, use on the trap
landing, use after finish, foreign-plan/root/lease pairing, a normal exit
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
    -> PreparedPinnedTextLifetimeClosureV1
         same function/session/residence + prepared use/finish census
    -> PreparedTextFormalExitFinishSetV1
         same opaque residence link; no copied exit rows or raw token
    -> private PreparedFunctionExitPlanV1
         owns the capability beside the existing PreparedFunctionExitSetV1
    -> same detached exit iteration emits finish immediately before Return
    -> optimizer/postprocess final MIR verifier
    -> VerifiedPinnedTextLifetimeClosureV1
         final backend-admission seal; no roots/accesses/exit ledger re-owned
```

`PinnedTextFunctionStateV1` is private protocol state of the sole Canonical
session, not a second semantic or CFG owner. The prepared and verified
lifetime-closure receipts do not re-own roots, accesses, target policy, blocks,
sites, or Completion. The prepared receipt proves the session can be projected;
only the post-transform final MIR verifier issues the verified backend seal.
Responsibility-specific scoped projections may borrow the live state, but no
caller can obtain detached residence, access, or finish parts and re-pair them.

The final lifetime census mechanically verifies that the successful entry
landing dominates every `PinnedTextOp`, every referenced root and plan belongs
to this session, every access precedes its planned finish, and no root/frame/
pointer carrier escapes through a store, return, or foreign call. The session
seals one prepared function-level finish capability without enumerating exits.
The existing DraftSeal exit iteration checks that capability while emitting
one finish at every admitted normal Completion exit. After all transforms, the
final verifier rechecks that a noreturn/no-unwind trap edge has no finish, each
return operand precedes finish, the last executed instruction immediately
before every normal Return is exactly one matching-residence finish, no Text
access follows finish, and the exact residence link is unchanged.
`EffectMask::READ` is optimizer metadata; it is not a substitute for this
census.

The closure receipts and finish capability store no source site, Completion
claim, exit block, return value, source order, or independent exit count. The
prepared capability retains only the same function/session stamp and opaque
`TextFormalResidenceIdV1`. The private DraftSeal preparation seam moves it into
`PreparedFunctionExitPlanV1`, where the already-existing
`PreparedFunctionExitSetV1` remains the sole per-exit ledger. Exact per-exit
coverage is established only by consuming both in that one projection loop.

The residence frame is concrete but is not part of the Hakorune callable ABI.
It is a versioned compiler-runtime private physical ABI: the selected current
`ny-llvmc(boundary pure-first)` consumer allocates one invocation-local stack
frame, the runtime entry transaction
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
root row:     size 16, align 8; `const u8 *` backing pointer, byte length u64
frame size:   checked 32 + 16 * root_count
root count:   ExactText formal occurrence count from the physical signature
root order:   logical formal occurrence order
each byte length <= i64::MAX
target-owned maximum root count/frame bytes: fixed before stack allocation
```

The entry ABI has separate occurrence-ordered read-only pair input and
caller-owned output-frame buffers plus a fixed `repr(u32)` status wire. The AOT
issuer proves exact stack extent, liveness, writable alignment, and input/output
non-overlap; the runtime checks scalar revision/size/count/cap metadata and does
not retain either pointer. All pair/backing/length/token checks complete before
the first pin or output write. Success alone commits all pins, token, header,
and root rows; any reject leaves runtime state unchanged and the output frame
invalid.

Compile-time target admission checks the selected backend, 64-bit pointer and
address-space/data-layout compatibility, ABI revision, fixed frame cap,
entry/finish symbol availability, and function-wide no-unwind before MIR
emission. C/Rust layout parity uses an actual private pointer field rather than
assuming an arbitrary pointer-to-`u64` round trip. The runtime entry separately
checks the concrete frame metadata before pin or frame publication. Other
backends are `RejectBeforeEffect` in V1; they do not lower to a generic String
helper, FastMem, or scalar retry. The raw frame is only transport; the runtime
token table and Canonical session state remain the lifetime authorities.

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

The first pinned backing domain is deliberately registry-issued:
`StableText(String)` and, when production StringBox admission opens, a dedicated
`CanonicalStringBox` payload variant published by the registry owner. Neither
`type_name() == "StringBox"`, virtual `as_any`/downcast, generic `as_str_fast`,
`StringViewBox`, nor fail-open `StringSpan` may execute under the registry lock
or issue the backing proof. Root descriptors and the lease-set token are
emitted by one pair-based entry transaction and remain non-splittable; common
MIR observes only opaque root and lease-set IDs, never an independently
storable raw `ptr,len` pair.

A StableText-only caller-zero/benchmark residence row may explicitly reject
all Box payloads and proceed without that variant. The dedicated
`CanonicalStringBox` publisher is a production StringBox cutover blocker, not
permission to revive virtual classification under the lock.

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

The production finish projection is a side-effecting `finish_or_abort` ABI:
the ABI itself is `nounwind` and valid finish returns `void`; its internal
missing/foreign/duplicate/state-mismatch terminal is fail-stop, `noreturn`, and
`nounwind`. It is never marked `readnone`,
`readonly`, `nofree`, or `speculatable`, so backing reads cannot move across a
finish that may retire storage. Ignoring a fallible finish result, retrying, or
falling through to Return after an invariant reject is forbidden. V1 admission
also requires every call/fault edge from successful entry through all normal
exits to be no-unwind; recoverable throw/catch, foreign unwind, async, and tail
lease transfer remain typed unsupported.

A synthetic shared epilogue/return PHI is not part of this route because it
would require new Canonical CFG/PHI authority and the MIRBuilder north star
explicitly forbids creating such a join merely to merge source Returns. A
future measured need may open a separate Decision; DraftSeal never infers that
join itself.

Future named stops include missing exact backing, literal origin, root/token
coupling, UTF-8 cursor proof, selected ny-llvmc direct-leaf consumption,
per-exit finish coverage, no-unwind Trap, and static target-stamped performance
admission.
Numeric C-ratio thresholds are owned by the later perf row rather than this
signature contract.

The successor family additionally stops with:

```text
NoSafeSlice::CrossPhasePinnedPlanWouldOwnSession
NoSafeSlice::PinnedResidenceEscapesCanonicalSession
NoSafeSlice::PinnedPlanDuplicatesCompletionExitLedger
NoSafeSlice::MissingCompilerRuntimeResidenceAbiOwner
NoSafeSlice::ResidenceEntryInputOutputAbiUnsealed
NoSafeSlice::FrameLayoutOrRevisionUnsealed
NoSafeSlice::PinnedTextFrameStackBoundUnsealed
NoSafeSlice::PinnedTextByteLengthOutOfI64Range
NoSafeSlice::PhysicalRootCountDerivedFromUniquePairs
NoSafeSlice::AliasedRootRowsMarkedNoAlias
NoSafeSlice::PartialFramePublishedOnAcquireReject
NoSafeSlice::PinnedRootIdDetachedFromFrameRow
NoSafeSlice::CanonicalStringBoxBackingIssuerMissing
NoSafeSlice::PinnedResidenceLifecycleCarrierUnsealed
NoSafeSlice::PinnedResidenceCapabilityMissingExactLink
NoSafeSlice::PinnedTextLifetimeClosureIncomplete
NoSafeSlice::PinnedLifetimeNotFinalVerified
NoSafeSlice::PinnedFinishMayReturnError
NoSafeSlice::PinnedResidenceFunctionMayUnwind
NoSafeSlice::MixedCallablePhysicalAbiRevision
NoSafeSlice::PrimaryAotDirectConsumerAmbiguous
NoSafeSlice::NonPrimaryBackendRequiresFallback
NoSafeSlice::PinnedTextOpScopeWidened
```

These stops refine the later `TEXT-FORMAL-PINNED-RESIDENCE` and
`LOOP-TEXT-SLICE-EXECUTION` families only. They do not widen or block the
current package-owned physical-signature mapping I0.

The current fast-path review is read-only because stable root backing,
CP-cursor lowering, and performance admission are future authorities, not
permissions to widen the active signature I0.

## Caller-zero physical-signature I0 landing (2026-08-16)

The package now issues one non-`Clone`
`VerifiedCallablePhysicalSignatureCohortV1` after the existing selected-map
and complete explicit parameter-contract co-seals. Each selected direct Box
method row retains its parser identity, owner, declaration mode, selected
role, exact receiver binding when applicable, four distinct count axes, and a
complete ordered lane list. Top-level rows remain in the complete semantic
batch but are not silently assigned a method physical signature.

The issuer verifies `SourceBindingSiteV1::Receiver` directly from the
batch-owned resolved function. Static methods reject a receiver binding;
instance methods require the owner-matching Receiver record; explicit
parameter bindings must be ordinal-complete and owner-matching. Lane roles
are emitted as `[InstanceReceiver?]` followed by ordinal explicit lanes, with
ExactText represented only as adjacent `[ExactTextSlot,
ExactTextGeneration]`. No physical `ValueId`, Completion, header, runtime
token, call site, or residence is created.

Installation moves the cohort with the non-splittable package. The installed
Port's S6C surface is now one combined HRTB loan: selected lowering input and
its contracts, the package-owned S6C child, and its matching physical
signature row are lent together and consumed exactly once. Generic selected
key/admission paths cannot take the Main/S6C row independently, so a later
call-edge issuer has one scoped seam to extend rather than two selected-key
consumptions.

Focused evidence:

```text
physical_signature_tests: 2/2
s6c_child_tests: 2/2
normal_callable_semantic_package: 29/29
cargo check --lib: green (inherited warning census only)
```

## Caller-zero Residence I0 landing (2026-08-16)

`text_formal_residence.rs` now consumes already-published physical pair lanes
through one registry-owned transaction. The registry preflights all
occurrences, validates generation and StableText backing, records one root
descriptor per occurrence, commits grouped pin multiplicity, and returns one
move-only residence owner with a private frame header. `finish(self)` is the
only release path; the scoped root view cannot outlive the residence owner.

Focused evidence:

```text
runtime::text_formal_residence: 4/4
runtime::host_handles::call_lifetime: 17/17
runtime::text_formal_call_lease: 2/2
RUSTFLAGS=-Awarnings cargo check --lib: green
```

The caller-zero substrate is StableText-only. It does not issue a source
actual-origin, expose common-MIR `ValueId`s, emit lifecycle CFG, or connect a
Text leaf/backend/production caller. The next design row is
`LOOP-TEXT-SLICE-EXECUTION-D0`; the later Residence cutover seams remain
explicitly parked until a production call edge exists.
