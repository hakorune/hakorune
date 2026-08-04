---
Status: Superseded historical taskboard — no task-selection authority
Date: 2026-07-14
Decision: B′ superseded by C′ terminal Home finalization on 2026-08-05
Current activation: 0
Does not replace current blocker: CURRENT_STATE.toml D-prime next-row selection
Related:
  - ../../../../reference/language/ownership.md
  - ../design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md
  - ../design/arc-retirement-and-ownership-substrate-ssot.md
  - ../design/object-handle-box-identity-contract-ssot.md
  - ../design/box-object-model-replacement-map-ssot.md
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
---

# B′ Tombstone / ObjectCell / Adaptive Ownership Taskboard

> Historical notice: do not resume rows from this board. The accepted owner is
> `hakorune-home-ownership-task-2026-08-04.md`, headed by
> `OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0` and the C′ lifecycle SSOT. The
> detailed B′ rows remain provenance and migration-census evidence only.

Scope correction (2026-07-15): this board implements the explicit Shared,
resource, weak, and ObjectCell lanes selected by `ownership.md`. It no longer
owns a “normal Box is implicitly shareable” source default. Unique/scoped
alias source semantics and their activation order are owned by the sparse
ownership reference/taskboard.

## Objective

Move from fragmented Arc/finalization/handle truths to one family-gated object
substrate while preserving the already accepted Binding SSA and Ownership SSA
boundaries.

```text
language:
  explicit fini creates Dead; ownership end does not

MIR:
  Binding SSA = reaching ValueId
  Ownership SSA = strong token forwarding/consume

runtime:
  ObjectCell = lifecycle, payload, generation, physical counts, leases

optimizer:
  StaticUnique / LocalRc / SharedRc representation choice
```

This is a later runtime/substrate branch. It does not authorize switching the
current D-prime lane and does not block passive Ownership SSA vocabulary or
verifier work selected by that lane.

## Verified starting evidence

The consultation was checked against local source and existing SSOTs.

| Surface | Current truth | B′ gap |
| --- | --- | --- |
| `InstanceBox` | multiple `Arc<Mutex<...>>`, `finalized` bool, separate field stores | no single lifecycle/payload owner |
| global finalizer | raw box-id `FINALIZED_BOXES` plus `BoxFinalizer::Drop` | second Dead/fini authority |
| plugin v2 | `PluginHandleInner::Drop` and `finalize_now` call the same plugin fini | last strong currently invokes user fini |
| generic plugin | Rust `Drop` directly invokes plugin fini | same semantic collision |
| VM Box carrier | `Arc<dyn NyashBox>` / `Weak<dyn NyashBox>` | Arc still owns storage, atomic RC, identity projection, dispatch |
| host handles | reusable generation-0 slots containing Arc or text | stale handle/identity/root separation not closed |
| weak handles | separate weak registries and encodings | no co-sealed generation/state/count owner |
| pointer cache | raw typed pointer kept alive by hidden Arc | no typed ObjectCell lease/reclamation proof |
| clone/share | family-specific and partly shared/TODO behavior | `CopyOwned`, value clone, and identity share not yet separated everywhere |
| source lifecycle docs | scope exit previously implied automatic object fini | conflicts with accepted last-strong structural drop law |

These are inventory inputs. None is accepted as a permanent parallel
authority.

## Dependency DAG

The active compiler path remains independent until a real BoxRef family is
selected:

```text
SSA-RC-L0 -> SSA-RC-L1 -> SSA-RC-P0 -> SSA-RC-A0 -> SSA-RC-A1a
SSA-RC-A1a -> SSA-RC-V0 -> SSA-RC-A1b -> SSA-RC-A1c
SSA-RC-A1c -> SSA-RC-RET-P0 -> SSA-RC0 -> SSA-I1
{SSA-I1, exact BoxRef producer} -> SSA-I1-O1
```

B′ runtime branch:

```text
BFIN-D0
  -> BRC-DOC-R0
  -> BFIN-P0
  -> BFIN-D1
  -> BCELL-S0
  -> BCELL-R0

BCELL-R0 -> BCELL-F0
BCELL-R0 -> BCELL-W0
{BCELL-F0, BCELL-W0} -> BCELL-V0 -> BCELL-ABI0 -> BCELL-P1
{BCELL-P1, sparse SHARE-I0} -> BCELL-I0

{BCELL-I0, SSA-RC-A0, SSA-RC-V0, SSA-RC0, SSA-I1-O1}
  -> BCELL-SSA-I0

BCELL-I0 -> BFIN-S0
{BFIN-S0, selected fini-capable BCELL-FAM-In, BCELL-SSA-I0} -> BFIN-I0
{selected weak-capable BCELL-FAM-In, BCELL-W0, weak-token decision}
  -> BWEAK-I0
BFIN-I0 -> BFIN-R0
BWEAK-I0 -> BWEAK-R0

{BFIN-I0, plugin structural-drop ABI} -> BPLUGIN-I0
BPLUGIN-I0 + exact legacy caller zero -> BPLUGIN-R0

BCELL-I0 -> BCELL-FAM-Pn -> BCELL-FAM-In -> BCELL-FAM-Rn
{selected BCELL-FAM-Rn, applicable BFIN-R0/BWEAK-R0}
  -> BHOST-I0 / BPTR-I0 / BBACKEND-P0
BBACKEND-P0 -> BBACKEND-I0

{all supported BCELL-FAM-Rn, BHOST-I0, BPTR-I0, BBACKEND-I0, BPLUGIN-R0}
  -> BCELL-GLOBAL-R0
  -> BPRIME-CLOSE0

{BCELL-SSA-I0, BFIN-R0, production perf evidence}
  -> BADAPT-D1
BADAPT-D1 -> BADAPT-U0-P / BADAPT-L0-P
BADAPT-U0-P -> BADAPT-U0-I -> BADAPT-U0-VR
BADAPT-L0-P -> BADAPT-L0-I -> BADAPT-L0-VR
{BADAPT-L0-VR, thread publication capability} -> BADAPT-S0-P
BADAPT-S0-P -> BADAPT-S0-I -> BADAPT-S0-VR
{BADAPT-U0-VR, BADAPT-L0-VR, thread move capability}
  -> BADAPT-MOVE0-P -> BADAPT-MOVE0-I -> BADAPT-MOVE0-VR
{BADAPT-U0-VR, BADAPT-L0-VR, BADAPT-S0-VR, BADAPT-MOVE0-VR}
  -> BADAPT-P0

{BADAPT-P0, concrete first-independent-owner performance evidence}
  -> optional BADAPT-UP0 -> optional BADAPT-UP0-VR

{BADAPT-U0-VR, exact closed call-edge consumer}
  -> optional BADAPT-U0-ABI
```

`sparse SHARE-I0` is the parent sparse-ownership taskboard's verified explicit
Shared boundary. B′ inventory/schema work may remain disconnected before it,
but no ObjectCell production carrier activates without it. The first Arc-based
Rust interpreter handler remains a temporary semantic oracle; it is not
ObjectCell parity or Arc retirement proof.

## Task order

### BFIN-D0 — lifecycle constitution and taskization — closed by this card

Close the durable policy only:

```text
B′ core accepted
explicit fini != ownership destruction
explicit Shared-lane fields carry Shared owners; owning Unique fields forward
one owner; neither implicitly fini children
ObjectIdentity != owner/root token
correctness-first physical strategy = SharedRc (atomic)
adaptive modes are later derived plans
normal Box source/API has no manual physical-free operation
source-level unique/reclaim and raw unsafe memory are separate future Decisions
production code/behavior delta = 0
current blocker remains the CURRENT_STATE.toml D-prime next-row selection
```

Docs-only closeout is allowed for this durable policy decision. The next B′
row must produce a machine-readable inventory artifact; it may not be a second
free-form consultation.

### BRC-DOC-R0 — legacy RC-authority docs reconciliation — closed

Reconcile the old Phase-29y post-CFG insertion proposal with A′/B′:

```text
canonical strong event placement = Verified Ownership SSA
backend/runtime = verified event materialization only
post-CFG rc_insertion = legacy/optional compatibility
weak event placement = later co-sealed weak-token contract
```

The old pass remains classified until exact caller zero; this docs row does
not delete or change it. Production behavior delta is zero.

### BFIN-P0 — exact current-authority and hidden-root inventory

Create one reusable machine ledger and guard. Classify every current producer,
consumer, state bit, root, and finalizer:

```text
InstanceBox finalized / in_finalization / field stores
FINALIZED_BOXES / BoxFinalizer
PluginHandleInner Drop / finalize_now
GenericPluginBox Drop
FiniOwner uses and DropBox execution plans
VMValue BoxRef / WeakBox clone/drop/take
host handle roots and slot reuse
weak handle and WeakRef registries
TLS/raw pointer caches and the Arc root that pins each pointer
clone_box / share_box / clone_arc
scope cleanup and constructor partial-failure paths
backend retain/release/fini/no-op surfaces
legacy post-CFG rc_insertion pass and Phase-29y ownership-placement claims
```

Each row records:

```text
semantic role
physical lifetime owner
logical fini owner
structural drop owner
identity owner
hidden root yes/no
thread visibility
target disposition
first-family allowed/rejected
```

Acceptance:

```text
unclassified rows = 0
implicit user-fini-from-drop rows are named
parallel Dead-state authorities are named
hidden Arc roots are named
generic BoxRef first-family eligibility = rejected
production behavior delta = 0
```

### BFIN-D1 — implementation-boundary design stop

Use the P0 evidence to close only decisions needed by the passive substrate:

```text
LifecycleState / PayloadState / residency vocabulary
privileged FinalizerLease and ordinary ObjectLease law
hook failure, reentrancy, concurrent-loser, and memory-order law
deterministic stored-token destruction order
plugin logical-fini vs structural-drop ABI boundary
ObjectIdentity vs strong root/token API names
opaque handle namespace and generation overflow policy
weak value copy/destroy representation
non-moving storage and reclamation/lease strategy
construction/unpublished state and partial-birth teardown
Dead tombstone CopyOwned/identity-forwarding law
implicit weak anchor and count overflow/underflow law
same object identity vs independently consumable root-token law
duplicate-consume rejection owner for MIR, VM registers, and checked host roots
```

Already fixed and not reopened:

```text
DestroyOwned never invokes user fini
last strong never invokes user fini
ordinary shared field release never implicitly calls child fini
CopyOwned is not thread publication
raw pointer is not canonical identity
Dead CopyOwned may preserve/duplicate the tombstone owner without resurrection
weak create/upgrade to Dead is rejected
normal Box source/API exposes no raw physical free or reclaim
derived StaticUnique reclaim uses terminal DestroyOwned, not a second consume op
```

The D1 result must also decide whether concurrent losers observe the winner's
hook error or a normalized terminal error, while preserving this invariant:
hook failure/panic still completes teardown, publishes Dead once, and never
retries the user hook. It must distinguish winner-recursive Finalizing from a
later idempotent call on Dead and must define callback/method-lock ordering.

If any row has more than one viable authority, keep BFIN-D1 stopped. Do not
hide the choice in an ObjectCell constructor.

### BCELL-S0 — passive typed substrate contracts

Add disconnected, private contracts and module README before implementation:

```text
BoxIdentityV1 / StrongRootTokenV1 separation
LifecycleStateV1
PayloadStateV1
RcStrategyV1
ObjectLeaseV1
FinalizerLeaseV1
ObjectCellErrorV1
weak upgrade request/result
cell reclamation witness
```

Rules:

```text
production callers = 0
Arc backing unchanged
no second production refcount
no direct pointer in public identity/API
no generic BoxRef conversion
```

### BCELL-R0 — disconnected SharedRc ObjectCell

Implement a correctness-first reference cell in isolated tests:

```text
one atomic strong-count authority
one atomic weak-count authority
payload Present/Absent slot
generation-bearing stable allocation
implicit weak-anchor convention fixed by D1
strong zero performs structural drop only
weak zero after strong zero permits cell reclamation
generation wrap retires the slot
no user hook in retain/release/drop
```

Do not implement `StaticUnique`, `LocalRc`, or dynamic promotion in this row.
Do not place a second count beneath a production Arc. Test storage must be
non-moving or accessed only through opaque handles.

Required fixtures:

```text
one strong / no weak
two independent owner tokens; destroy one leaves the other usable
last strong removes payload once
weak tombstone survives strong zero
last weak reclaims cell
stale generation reject
slot reuse changes generation
generation overflow retires slot
count underflow and invalid state transition reject
two distinct roots for one identity may each be destroyed once
```

The count core cannot distinguish “one root destroyed twice” from “two roots
of the same identity destroyed once each”; both are the same decrement trace.
Token-specific rejection belongs to the verified carrier:

```text
verified MIR:
  Ownership SSA rejects duplicate consume of one ValueId token

Rust interpreter / VM:
  taking an already-consumed owned register rejects

checked host / FFI root table:
  consuming an already-consumed root-token slot rejects

ObjectCell count core:
  rejects underflow and invalid lifecycle/count transitions only
```

### BCELL-F0 — disconnected eager-fini transaction

Implement the state transaction over the R0 cell:

```text
Alive -> Finalizing winner exactly once
new ordinary leases rejected
existing leases drain
winner gets privileged self-access lease
hook executes once
ordinary shared-field tokens release in reverse declaration order
child user fini is not implicit
stored weak tokens are destroyed without target traversal/fini
payload teardown exactly once
Dead publication after teardown
receiver/root token remains owned by caller
Dead repeat fini is idempotent
Dead identity/type/hash/debug observation uses tombstone metadata only
```

Failure injection:

```text
winner transition
lease drain
user hook
middle stored-token release
native payload teardown
Dead publication
recursive fini and concurrent loser
hook/helper callback and lock-order violation
```

Every failure preserves primary plus cleanup errors, prevents payload
resurrection, and never publishes a partially Alive cell.

Required same-identity/different-token sequence:

```text
b = CopyOwned(a)
fini(a)
fini(b)
DestroyOwned(a)
DestroyOwned(b)

user hook = 1
payload teardown = 1
root consumes = 2
cell reclamation = 1
```

The second `fini` is a Dead-state no-op. The two destroys are legal because
`a` and `b` are distinct root tokens, even though they name one identity.

### BCELL-W0 — disconnected weak/generation algebra

Close the weak-value decision from BFIN-D1 and implement it with the same cell:

```text
weak creation only from Alive strong access
weak clone/drop count discipline
upgrade atomically validates generation + Alive + strong>0 and acquires root
Finalizing/Dead/weak-only/stale/reclaimed upgrade reject
weak equality uses slot + generation
no separate weak identity table as semantic truth
```

Required weak-token boundary fixtures:

```text
same identity + two distinct weak tokens + one drop each = legal
same weak token dropped twice = weak carrier/table error
weak count core distinguishes underflow, not token identity
reclaimed/stale checked token rejects before cell dereference
```

Production WeakRef activation remains zero.

### BCELL-V0 — state-machine and concurrency verification

Add model/state transition tests plus bounded concurrent tests:

```text
no payload access without a valid lease
no new lease after Finalizing wins
no weak resurrection after Finalizing
one hook under concurrent fini
one structural drop under concurrent destroy
last strong cannot reclaim or reuse a cell under an active ObjectLease
acquire/release publication of Dead and payload absence
callback reentrancy follows the D1 rule
all terminal counter/state combinations are valid
```

This proves the disconnected SharedRc profile only; its counters are atomic.

### BCELL-ABI0 — runtime/backend adapter, production callers zero

Expose the exact substrate operations required by verified Ownership SSA:

```text
copy_owned strong root
destroy_owned strong root
explicit finalize object
borrow payload under lease
weak create/upgrade/drop
identity observation
```

Connect temporary Rust VM tests and the selected handle ABI without activating
canonical source callers. Unsupported backends fail preflight; no operation is
a no-op. `.hako` MIR interpreter parity remains the HMI branch owned by the
D′ taskboard and does not duplicate ObjectCell semantics.

Required token/identity fixtures:

```text
same identity + two CopyOwned roots + one destroy each = legal
same MIR/root token consumed twice = carrier verifier error
checked host/root token consumed twice = stale/consumed-root error
resident-cell count underflow = ObjectCell contract error
reclaimed/stale checked handle rejects before ObjectCell dereference
```

### BCELL-P1 — first identity-bearing family selection

Select exactly one family from the P0 ledger. Required profile:

```text
non-plugin
no unknown FFI ownership
no cross-thread sharing
no production user-fini hook
no production WeakRef carrier
no hidden pointer cache
closed clone/share behavior
closed field-root ownership
exact dispatch/type route
backend fail-fast available
generic BoxRef excluded
```

The already Arc-free stable-text handle family is useful substrate evidence but
does not prove identity, fini, or weak semantics. Do not reuse it as a false B′
completion claim.

### BCELL-I0 — first family atomic ObjectCell carrier cutover

For the selected family only, atomically switch:

```text
identity
strong/weak physical owner for the admitted profile
payload storage
dispatch projection
clone/share behavior
structural drop
backend capability
```

Forbidden intermediate states:

```text
Arc and ObjectCell both own production counts
old finalized flag plus ObjectCell state both decide Dead
host/weak legacy table and new cell both decide generation
fallback to Arc after ObjectCell failure
```

### BCELL-SSA-I0 — first canonical BoxRef materialization on ObjectCell

After both compiler and runtime prerequisites are green, materialize one exact
BoxRef canonical owner through ObjectCell:

```text
VerifiedOwnershipSsaV1 is required
CopyOwned / DestroyOwned use BCELL-ABI0
Phi and Return forward tokens without retain
Binding SSA remains the only reaching-value map
unsupported storage/ABI is rejected before Builder effects
```

This is the first row that may claim production Ownership SSA plus ObjectCell
for one closed source owner.

### BFIN-S0 — passive public-fini transaction route

Before source activation, add one disconnected route contract:

```text
public obj.fini()
  -> FinalizeObject transaction entry
  -> lifecycle winner/lease/teardown owner
  -> exact user-fini hook dispatch inside the transaction only
```

`UserFini` route metadata may identify the hook, but it cannot own once/order,
state transition, or payload teardown. Direct source-to-hook calls and generic
Rust `Drop` hook calls are guarded as forbidden. Production callers remain
zero.

### BFIN-I0 — explicit object-fini route activation

After BCELL-SSA-I0 and that family's ObjectCell cutover, for one admitted
family with a user hook, activate BFIN-S0 and connect the language-level
explicit fini route to BCELL-F0:

```text
BoxCallable/route truth selects the exact hook
cleanup may call explicit fini before local token destruction
fini consumes no receiver token
ordinary child field release calls no child fini
Dead Shared owners preserve identity-only observation
last strong after Dead performs no second payload teardown
```

### BWEAK-I0 — production WeakRef adoption

Only after BCELL-W0 and its MIR value-lifecycle decision are green:

```text
one WeakRef representation
one slot+generation identity relation
one weak copy/drop authority
one linearizable upgrade authority
legacy weak registries are projections or rejected
```

Do not infer this row from strong Ownership SSA.

### BFIN-R0 — selected-fini-family legacy lifecycle retirement

For the selected fini-capable family only, prove exact caller zero and
remove/isolate only the legacy authorities that the Pn inventory attributed to
that family:

```text
family-scoped FINALIZED_BOXES entries, if any
InstanceBox finalized/in_finalization flags, only for an InstanceBox row
BoxFinalizer implicit-fini ownership for the selected route
family-scoped legacy fini dispatch/projection rows
```

Repository-wide removal waits for every family; family-local caller zero is
not global retirement evidence. Weak registries, generation-0 identity claims,
and pointer caches retire in their own selected-family rows.

### BWEAK-R0 — selected weak-family legacy retirement

After BWEAK-I0, retire only the admitted weak family's old representation,
copy/drop owner, upgrade route, finalized-state projection, and registry rows.
Other weak families and generation-0 handle tables remain explicit pending
inventory until their own family/host cutover.

### BCELL-FAM-Pn / In / Rn — family-by-family rollout

Repeat one atomic family series at a time:

```text
Pn:
  inventory identity, fields, clone/share, dispatch, weak/fini, hidden roots,
  backend capability, and select one closed profile

In:
  atomically switch the selected family to ObjectCell/lease/ABI ownership

Rn:
  prove family-local Arc/finalized/legacy-handle caller zero and retire only
  that family's old authority, including applicable fini/weak rows
```

`BFIN-R0` and `BWEAK-R0` are the first selected-family templates. Later
families repeat those checks inside their own `BCELL-FAM-Rn`; no global
finalized or weak registry is deleted from one-family evidence.

Required families are discovered by BFIN-P0 and include at least:

```text
InstanceBox/user boxes after one payload/field-root truth exists
builtin identity/resource boxes
host/FFI-exposed identity boxes
plugin boxes through their separate ABI gate
```

`InstanceBox` cannot cut over while declaration vectors, value maps,
`box_fields`, and `inner_content` can independently retain the same logical
field. Its family Pn must first select one payload/field-root owner without
changing accepted language behavior.

### BHOST-I0 — generation-tagged host/root convergence

Unify host roots with ObjectCell identity without changing the external opaque
`u64` ABI:

```text
host handle = root/token, not object identity
slot + generation identity validation
no slot reuse while any strong/weak/root token remains
strong/weak host operations use the same cell state
generation-0 compatibility is explicit and cannot enter the new profile
```

### BPTR-I0 — typed leased pointer projections

Replace hidden Arc-pinned raw caches in admitted families with private,
non-escapable typed projections:

```text
StrongObjectRef / PinnedObjectRef under ObjectLease
stable non-moving storage proof
callback/reentrancy-safe lease lifetime
no dereference before generation/root validation
hidden Arc root count = 0 for the admitted path
```

### BBACKEND-P0 / I0 — backend capability and parity rollout

P0 classifies every MIR/runtime consumer and forbids silent behavior:

```text
Rust semantic oracle
.hako MIR interpreter subset
nyrt / llvm_py handle lane
native LLVM consumers
Wasm
archived JIT/Cranelift surfaces
plugin/host providers
```

I0 implements one named backend family at a time with normalized traces for
copy/destroy/fini/weak/identity/reclamation. Unsupported families fail before
codegen/runtime effects. Rust-to-`.hako` interpreter ownership remains the HMI
taskboard; this row supplies the shared ObjectCell ABI, not a second VM plan.

### BCELL-GLOBAL-R0 — repository-wide legacy authority retirement

Only after all supported family/backend rows are green and exact callers are
zero, retire or demote:

```text
FINALIZED_BOXES and BoxFinalizer logical-dead ownership
per-family finalized/in_finalization flags
FiniOwner transition/once/order authority (route metadata may remain)
generation-0 host/weak identity authority
parallel weak registries
hidden Arc pointer roots
VMValue BoxRef/WeakBox Arc carrier aliases
legacy post-CFG ownership placement authority
drop-to-user-fini routes
```

Do not delete compatibility vocabulary while any explicit legacy provenance
still calls it.

### BPRIME-CLOSE0 — final constitution closeout

The B′ branch is complete only when machine guards prove:

```text
logical lifecycle authority per supported object = 1
physical strong/weak count authority per supported object = 1
object identity relation = slot + generation
user fini calls from last strong / DestroyOwned / native Drop = 0
public fini transaction bypasses = 0
unsupported backend silent fallbacks = 0
supported family legacy Arc/finalized/weak-handle authorities = 0
adaptive modes preserve atomic SharedRc lifecycle traces where enabled
strong cycle leak policy remains explicit
```

Update the current pointer only when this branch becomes active or closes; do
not make this parked roadmap the current blocker now.

### BPLUGIN-I0 / BPLUGIN-R0 — plugin lifecycle split and adoption

Plugins require a separate accepted ABI decision:

```text
logical plugin fini:
  explicit user route only

structural plugin instance destroy:
  last-strong/drop glue only
```

After the ABI and provider parity exist, atomically move plugin identity,
finalization, drop glue, weak/root visibility, and handle storage to ObjectCell.
Then retire `PluginHandleInner::Drop`/`GenericPluginBox::Drop` user-fini calls
only at exact caller zero.

Plugins are not a prerequisite for the first non-plugin family.

### BADAPT-D1 — representation-selection boundary

After production SharedRc correctness and real performance evidence, seal a
machine-readable forcing matrix. The B′ baseline is conservative static
selection, not lazy first-independent-owner fallback:

```text
no possible independent owner over the full lifetime:
  StaticUnique candidate

possible same-thread independent Shared owner / local CopyOwned:
  LocalRc from allocation

weak, host, registry, unknown FFI, or unknown publication:
  SharedRc baseline unless a narrower closed profile is separately proven

cross-thread share:
  SharedRc before destination publication

cross-thread move:
  may preserve the selected mode only after all source-thread roots close
```

`StaticUnique` and a possible future `PromotableUnique` are distinct
representations. `strong_count == 1` after prior owner duplication never reconstructs
either exclusivity proof, and no representation downgrades after publication.

### BADAPT-U0-P — disconnected StaticUnique proof

Build `VerifiedStaticUniqueObjectPlanV1` without production activation:

```text
one creation root
CopyOwned duplication = 0
weak/host/FFI/registry/closure/task/global publication = 0
unknown call or dynamic-dispatch escape = 0
hidden runtime roots = 0
all exact callees are non-retaining/non-publishing or rejected
each Owned SSA lifetime has one edge-forwarding or terminal disposition
the closed object lifetime reaches one terminal DestroyOwned in this proof owner
Return/parameter/outbox/generic-call capability transport = rejected in V1
outstanding BorrowedStrong/ObjectLease/raw projection at consume = 0
active finalizer/pin/cache at consume = 0
error/cleanup/early-exit paths consume exactly once
```

Identity/hash/tombstone observation, explicit fini, generation, or host
visibility may independently prevent header/cell elision even when the strong
root is statically unique. The proof must keep those observability axes
separate from owner count.

Required control fixtures:

```text
branch-generated static unique -> Owned Phi -> terminal consume
mutually-exclusive edge forwarding -> Phi -> one consume
loop-carried unique Phi
Return/outbox/generic call rejects the function-local StaticUnique witness
unknown call / field store / closure capture rejects the witness
active borrow or ObjectLease rejects terminal reclaim
error before terminal consume runs cleanup once
successful terminal consume prevents a second scope cleanup consume
constructor partial failure drops initialized fields in reverse order
```

Required normalized lifecycle traces and counters:

```text
Alive terminal Destroy:
  user hook 0 / payload drop 1 / cell reclaim 1

fini then terminal Destroy:
  user hook 1 / payload drop 1 / second payload drop 0 / cell reclaim 1

repeat fini then terminal Destroy:
  user hook 1 / root consume 1

CopyOwned/retain = 0
raw free without StaticUnique witness = 0
post-consume cleanup DestroyOwned = 0
self-strong-cycle / weak / host / hidden-root profile acceptance = 0
drop-error cleanup retry of the consumed token = 0
```

### BADAPT-U0-I — one-family StaticUnique materialization

For one closed family/backend only, materialize:

```text
terminal DestroyOwned
+ VerifiedStaticUniqueObjectPlanV1
  -> immediate structural drop/reclamation when legal
```

This does not add `ReclaimUnique`, call user `fini`, or change source/MIR
semantics. Header, heap allocation, and lifecycle-check elision require their
own observability proofs. Unsupported shapes fail before effects; a selected
StaticUnique path never retries through Arc or an unverified RC carrier.

### BADAPT-U0-VR — parity and selected-profile route retirement

Compare the StaticUnique family with the atomic SharedRc oracle using normalized
result, hook, payload-drop, field-drop, root-consume, and cell-reclaim traces.
Require exact caller zero only for the superseded RC/header operations on the
selected closed allocation profile/site before claiming the optimization.
Other allocation sites in the same Box family retain the SharedRc oracle path.
Header/cell retirement additionally requires independent proof that identity,
fini tombstone, weak, host, generation, stable-address, and lease observability
are all absent.

### Optional BADAPT-U0-ABI — interprocedural StaticUnique forwarding

This is not required for the first function-local U0 profile. Open it only
with an exact closed call-edge consumer. Co-seal branded parameter/result
capabilities and the caller continuation through the final terminal consume.
Generic/dynamic returns, unknown callees, or a caller that may `CopyOwned` force
LocalRc/SharedRc selection and reject the StaticUnique witness. The capability
is an ABI sidecar; it is not a new `MirOwnershipKindV1` value.

### BADAPT-L0-P — disconnected LocalRc proof

For one closed allocation profile, prove whole-lifetime thread confinement and
possible same-thread independent Shared ownership. Unknown external calls, host/registry
publication, or unknown thread escape select SharedRc before allocation rather
than a heuristic LocalRc path. Production remains zero.

### BADAPT-L0-I — one-profile LocalRc materialization

Use one non-atomic count authority for the selected profile only. Unsupported
or invalidated proofs select the SharedRc oracle before effects; failure after
LocalRc selection does not retry through Arc or an unaccounted owner.

### BADAPT-L0-VR — LocalRc parity and selected-profile retirement

Require normalized lifecycle/result parity with SharedRc, thread-confinement
violation rejection, error/cleanup exactness, and caller zero only for the
selected profile's superseded atomic-count route.

### BADAPT-S0-P — cross-thread share/promotion contract

Seal one share profile:

```text
cross-thread share:
  source owner remains; target owner is added
  reachable strong graph is closed or explicitly synchronized
  payload has shared/synchronized capability
```

The proof fixes stable identity, publication happens-before, and one promotion
linearization point. A bit-copy or `CopyOwned` alone is not a cross-thread
share witness.

### BADAPT-S0-I — one-profile SharedRc promotion

For one closed publication profile, materialize `LocalRc -> SharedRc` before
the destination root is visible. Promotion failure keeps the source roots and
old authority valid, leaves the destination unpublished, and never retries
through Arc. Direct PromotableUnique-to-SharedRc remains unavailable unless
the same S0-P publication witness explicitly admits it.

### BADAPT-S0-VR — promotion parity and retirement

Require bounded race/model tests, normalized SharedRc-oracle traces, partial
old/new count authority publication = 0, post-promotion downgrade = 0, and
caller zero only for the selected publication profile's superseded route.

### BADAPT-MOVE0-P — cross-thread ownership-move proof

Seal a distinct move profile:

```text
all source-thread roots close; one token transfers
outstanding leases/borrows = 0
payload has explicit move/Send capability
no thread-affine finalizer/provider/TLS residency
publication happens-before
one owner-thread/root-registry transfer point
destination receives no second independent owner
```

This proof may preserve StaticUnique/LocalRc only when their full invariants
remain valid in the destination thread. It is not inferred from a bit-copy or
from closing one visible local.

### BADAPT-MOVE0-I — one-profile ownership move

For one closed profile, atomically consume the source-thread root registration,
transfer the same owner token and mode metadata, and publish one destination
root. Failure leaves the source owner valid and destination unpublished; no
temporary second root or Arc fallback is allowed.

### BADAPT-MOVE0-VR — ownership-move parity and retirement

Require normalized lifecycle/result parity, thread-affine payload rejection,
lease/borrow race tests, partial source/destination root publication = 0, and
caller zero only for the selected move profile's superseded route.

### BADAPT-P0 — representation selection and parity

Seal one derived representation plan with normalized semantic parity across:

```text
SharedRc baseline (atomic)
StaticUnique
LocalRc
SharedRc promotion where admitted
cross-thread ownership move where admitted
```

The plan is invalidated when ownership/escape/thread facts change. It is never
source syntax or Ownership SSA truth.

### Optional BADAPT-UP0 — lazy first-independent-owner PromotableUnique

This row is parked and is not required for B′ closeout. Open it only after
BADAPT-P0 and concrete evidence that allocating LocalRc for a merely possible
independent owner is a hot cost.

It must define one promotion transaction:

```text
PromotableUnique -> LocalRc on first independent local owner
optional direct PromotableUnique -> SharedRc only with BADAPT-S0-P witness
promotion failure keeps src valid and leaves dst unpublished
old authority invalidation + new counter authority + dst publication
  have one linearization point
legacy Arc fallback = 0
```

PromotableUnique retains a promotion-capable control representation and cannot
claim StaticUnique header/cell elision. A normal `CopyOwned` may request this
transaction only through the sealed representation plan; it does not infer or
perform thread publication.

### Optional BADAPT-UP0-VR — first-independent-owner promotion closeout

Compare normalized result/lifecycle/count traces with the SharedRc oracle.
Inject failure before and after each promotion phase and require:

```text
source valid on failure
destination unpublished on failure
partial old/new count authority = 0
legacy Arc fallback = 0
post-promotion downgrade or StaticUnique reconstruction = 0
selected-profile superseded route caller zero
```

### Parked BRAW-SOURCE-D0 — source exclusive/raw memory decision

No current consumer justifies reopening the language surface. B′ therefore
keeps all of these at activation zero:

```text
unsafe block
source `unique` qualifier
source `reclaim` operation
raw Box physical-free API
```

If a concrete future consumer appears, split the work before implementation:

```text
BEXCL-SOURCE-D0:
  source-exclusive Box capability and affine ABI

BRAW-MEM-D0:
  raw pointer/provenance/allocation-token memory model
```

Keep a source-exclusive capability, physical `RcStrategy`, pointer form, and
terminal ownership consume as four different authorities. A future exclusive
Box consumes its Box owner through `DestroyOwned`; broad unsafe must not disable
Ownership SSA. A raw allocator allocation is not a Box identity/root and needs
its own provenance-bearing allocation token and release operation. It cannot
promise Box-local UB. Until those Decisions are explicitly reopened, unsafe
syntax remains rejected and no dormant AST/type semantics are added.

### Parked BIMMORTAL-D0 — explicit runtime-root residency

`Immortal` is not an `RcStrategy` and is not selected by BADAPT-P0. Open a
separate Decision only for a concrete singleton/type-descriptor/process-root
consumer. It must close process shutdown, explicit fini, weak/identity
observation, plugin unload, root provenance, and backend parity before adding
proof/materialization/retirement rows. It is not required for B′ closeout.

### Optional cycle diagnostics

After strong/weak production is stable, an optional leak/cycle diagnostic may
observe the object graph. A collector is not required for B′ completion and
must never call user `fini()`.

## Production activation table

| Milestone | ObjectCell production | user fini transaction | weak | adaptive mode |
| --- | ---: | ---: | ---: | --- |
| D0/BRC-DOC-R0/P0/D1/S0/R0/F0/W0/V0/ABI0/P1/BFIN-S0 | 0 | 0 | 0 | none |
| BCELL-I0 | 1 closed runtime family | 0 / fail-fast | 0 / fail-fast | SharedRc |
| BCELL-SSA-I0 | 1 exact canonical BoxRef owner | 0 / fail-fast | 0 / fail-fast | SharedRc |
| BFIN-I0 | 1 closed fini-capable family | 1 | 0 unless BWEAK-I0 separately admitted | SharedRc |
| BWEAK-I0 | admitted families | admitted families | 1 representation | SharedRc |
| BPLUGIN-I0 | plugin family only | plugin explicit route | admitted profile | SharedRc |
| BCELL-FAM-In / BHOST-I0 / BPTR-I0 / BBACKEND-I0 | one selected scope per row | only admitted families | only admitted families | SharedRc or one proven derived mode |
| BCELL-GLOBAL-R0 / BPRIME-CLOSE0 | all supported families | all supported fini families | all supported weak families | capability-gated |
| BADAPT-D1 / BADAPT-U0-P / BADAPT-L0-P / BADAPT-S0-P / BADAPT-MOVE0-P | unchanged semantics | unchanged | unchanged | proof/selection only; production zero |
| BADAPT-U0-I/VR / BADAPT-L0-I/VR / BADAPT-S0-I/VR / BADAPT-MOVE0-I/VR | unchanged semantics | unchanged | unchanged | one selected derived profile |
| BADAPT-P0 | unchanged semantics | unchanged | unchanged | selector over parity-proven profiles only |
| optional BADAPT-U0-ABI / BADAPT-UP0/VR / parked BRAW-SOURCE-D0 / BIMMORTAL-D0 | unchanged semantics | unchanged | unchanged | activation zero until separately admitted |

## Required authority counters

Before BCELL-I0:

```text
ObjectCell production families = 0
production ObjectCell strong-count authorities = 0
production WeakRef ObjectCell carriers = 0
adaptive ownership production plans = 0
```

At first-family cutover:

```text
physical strong-count authorities per object = 1
logical lifecycle authorities per object = 1
payload ownership authorities per object = 1
object identity relations per object = 1
user fini calls from DestroyOwned/last-strong/Rust Drop = 0
legacy fallback after ObjectCell selection = 0
```

## May claim

| Milestone | Maximum claim |
| --- | --- |
| BFIN-D0 | B′ is accepted and taskized; runtime behavior is unchanged |
| BRC-DOC-R0 | canonical ownership-event authority is reconciled; legacy pass behavior is unchanged |
| BFIN-P0 | every current lifecycle/root seam is classified |
| BCELL-S0/R0/F0/W0/V0 | a disconnected atomic SharedRc model satisfies the named tests; production is zero |
| BCELL-ABI0 | exact operations and backend fail-fast exist; canonical callers are zero |
| BCELL-I0 | one named runtime family uses one ObjectCell physical owner |
| BCELL-SSA-I0 | one exact canonical BoxRef owner uses verified Ownership SSA plus ObjectCell |
| BFIN-I0 | one named family implements explicit eager fini tombstone semantics |
| BWEAK-I0 | the admitted weak profile uses one generation/count/upgrade owner |
| BFIN-R0 / BWEAK-R0 | only the selected fini/weak family's attributed legacy authorities reach caller zero |
| BCELL-FAM-In/Rn | only the selected family is cut over and its old authority reaches caller zero |
| BCELL-GLOBAL-R0 | supported-family legacy authorities are repository-wide caller zero |
| BPRIME-CLOSE0 | the full supported B′ profile satisfies the constitution counters |
| BADAPT-D1 / BADAPT-U0-P / BADAPT-L0-P / BADAPT-S0-P / BADAPT-MOVE0-P | one forcing/proof product exists; production remains zero |
| BADAPT-U0-I/VR / BADAPT-L0-I/VR / BADAPT-S0-I/VR / BADAPT-MOVE0-I/VR | only the selected derived profile is materialized and parity-proven |
| BADAPT-P0 | the selector chooses only parity-proven derived profiles and otherwise selects SharedRc before effects |
| BADAPT-U0-ABI / BADAPT-UP0/VR | only the explicitly admitted interprocedural/promotion profile is proven; neither is B′ baseline |

## Must not claim

```text
global BoxRef or Arc retirement from one-family evidence
adaptive ownership from the all-atomic baseline
WeakRef safety from strong-token verification
plugin B′ compliance before plugin ABI split
concurrent fini safety before every payload access uses a lease
direct-pointer safety from slot generation alone
cross-thread safety from atomic RC alone
cycle reclamation
all backend lifecycle parity
source-level move-only or explicit reclaim semantics
normal Box manual physical-free API
lazy first-independent-owner promotion from the StaticUnique proof
Immortal residency selected as an RcStrategy or before BIMMORTAL proof rows
```

## Stop conditions

Stop the active B′ row if it:

```text
changes the current lane selected by CURRENT_STATE.toml
creates a second production count beneath Arc
lets ordinary Rust Drop call user fini in an admitted family
lets parent ordinary fields implicitly fini shared children
uses one enum value to conflate lifecycle, payload, and residency axes
stores concurrent local/shared counts as parallel truths
conflates StaticUnique with a promotion-capable UniqueCell
reconstructs StaticUnique from strong_count == 1 after prior owner duplication
exposes raw pointers as identity or across a lease boundary
uses StaticUnique proof alone to authorize raw dereference, stable address, or lease elision
exposes raw/manual physical free through the normal Box API
adds a reclaim-specific second ownership consume authority
uses a source unique annotation as the runtime strategy proof
puts Immortal residency into RcStrategy or BADAPT-P0
asks the count core to identify same-token duplicate consume
confuses two roots of one identity with one root consumed twice
dereferences a reclaimed cell merely to diagnose duplicate release/underflow
reuses a slot before weak zero or after generation wrap
copies a weak value without a verified weak-count operation
upgrades weak through separate check/increment races
holds a cell/slab/method lock while invoking a user hook or drop callback
allows unknown FFI/thread escape into StaticUnique or LocalRc
activates generic BoxRef, plugin, weak, and concurrency in one commit
activates user fini or WeakRef in BCELL-I0 before BFIN-I0/BWEAK-I0
lets a selected promotion failure retry through Arc or an unaccounted owner
silently retries a legacy Arc carrier
deletes legacy state before exact caller zero
```

## Immediate next action

None in this parked branch. Continue the exact lane selected by
`CURRENT_STATE.toml`. When the B′ branch is selected after the parent sparse
Shared boundary, run BFIN-P0 first; do not start ObjectCell code from this
design card alone.
