---
Status: Parked dependency branch — taskized through final adaptive form
Date: 2026-07-14
Decision: B′ — eager-fini tombstone plus derived adaptive ownership
Current activation: 0
Does not replace current blocker: SSA-RC-L0
Related:
  - ../design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md
  - ../design/arc-retirement-and-ownership-substrate-ssot.md
  - ../design/object-handle-box-identity-contract-ssot.md
  - ../design/box-object-model-replacement-map-ssot.md
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
---

# B′ Tombstone / ObjectCell / Adaptive Ownership Taskboard

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
  Unique / LocalRc / SharedRc / Immortal representation choice
```

This is a later runtime/substrate branch. It does not authorize edits to the
active behavior-neutral SSA-RC-L0 row and does not block passive Ownership SSA
vocabulary/verifier work.

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
{BCELL-F0, BCELL-W0} -> BCELL-V0 -> BCELL-ABI0 -> BCELL-P1 -> BCELL-I0

{BCELL-I0, SSA-RC-A0, SSA-RC-V0, SSA-RC0, SSA-I1-O1}
  -> BCELL-SSA-I0

BCELL-I0 -> BFIN-S0
{BFIN-S0, BCELL-SSA-I0} -> BFIN-I0
{BCELL-I0, BCELL-W0, weak-token decision} -> BWEAK-I0
{BFIN-I0, BWEAK-I0} -> BFIN-R0

{BFIN-I0, plugin structural-drop ABI} -> BPLUGIN-I0
BPLUGIN-I0 + exact legacy caller zero -> BPLUGIN-R0

BCELL-I0 -> BCELL-FAM-Pn -> BCELL-FAM-In -> BCELL-FAM-Rn
BFIN-R0 -> BHOST-I0 / BPTR-I0 / BBACKEND-P0
BBACKEND-P0 -> BBACKEND-I0

{all supported BCELL-FAM-Rn, BHOST-I0, BPTR-I0, BBACKEND-I0, BPLUGIN-R0}
  -> BCELL-GLOBAL-R0
  -> BPRIME-CLOSE0

{BCELL-SSA-I0, BFIN-R0, production perf evidence}
  -> BADAPT-U0 / BADAPT-L0
{BADAPT-L0, thread publication capability} -> BADAPT-S0
BADAPT-U0 / BADAPT-L0 / BADAPT-S0 -> BADAPT-P0
```

The first Arc-based Rust interpreter handler remains a temporary semantic
oracle. It is not ObjectCell parity or Arc retirement proof.

## Task order

### BFIN-D0 — lifecycle constitution and taskization — closed by this card

Close the durable policy only:

```text
B′ core accepted
explicit fini != ownership destruction
ordinary strong fields are shared and do not implicitly fini children
ObjectIdentity != owner/root token
correctness-first physical strategy = SharedRc (atomic)
adaptive modes are later derived plans
production code/behavior delta = 0
current blocker remains SSA-RC-L0
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

Do not implement `Unique`, `LocalRc`, or dynamic promotion in this row.
Do not place a second count beneath a production Arc. Test storage must be
non-moving or accessed only through opaque handles.

Required fixtures:

```text
one strong / no weak
two strong aliases; destroy one leaves the other usable
last strong removes payload once
weak tombstone survives strong zero
last weak reclaims cell
stale generation reject
slot reuse changes generation
generation overflow retires slot
duplicate strong/weak destroy reject
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

### BCELL-P1 — first identity-bearing family selection

Select exactly one family from the P0 ledger. Required profile:

```text
non-plugin
no unknown FFI ownership
no cross-thread sharing
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

After BCELL-SSA-I0, for one admitted family with a user hook, activate BFIN-S0
and connect the language-level explicit fini route to BCELL-F0:

```text
BoxCallable/route truth selects the exact hook
cleanup may call explicit fini before local token destruction
fini consumes no receiver token
ordinary child field release calls no child fini
Dead aliases preserve identity-only observation
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

### BFIN-R0 — first-family legacy lifecycle retirement

For the cut-over family, prove exact caller zero and remove/isolate:

```text
global FINALIZED_BOXES entries
InstanceBox finalized/in_finalization flags
BoxFinalizer implicit-fini ownership
legacy weak registry rows
generation-0 identity claims
hidden Arc pointer cache roots
```

Repository-wide removal waits for every family; family-local caller zero is
not global retirement evidence.

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
  that family's old authority
```

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

### BADAPT-U0 — derived Unique plan

After production SharedRc correctness, add an escape/ownership proof:

```text
one creation root
no CopyOwned duplication
no weak/host/FFI/registry/closure publication
no unknown call escape
exact one path-sensitive terminal consume
```

Only then may RC operations, header, heap allocation, or lifecycle checks be
elided when their observability prerequisites are also absent.

### BADAPT-L0 — derived LocalRc plan

Prove thread confinement and local aliasing. Use one non-atomic count authority.
Unknown external calls and unknown thread escape select SharedRc, not a
heuristic LocalRc path.

### BADAPT-S0 — cross-thread move/share and one-way promotion

Separate:

```text
cross-thread move:
  all source-thread owners close; one token transfers

cross-thread share:
  source owner remains; target owner is added
```

Only share requires SharedRc. Close publication happens-before, graph closure,
payload synchronization capability, stable identity, and one linearization
point before enabling `LocalRc -> SharedRc`. `CopyOwned` must not silently
perform promotion.

### BADAPT-P0 — representation selection and parity

Seal one derived representation plan with normalized semantic parity across:

```text
SharedRc baseline (atomic)
Unique
LocalRc
SharedRc promotion where admitted
Immortal as a separate explicit runtime-root class
```

The plan is invalidated when ownership/escape/thread facts change. It is never
source syntax or Ownership SSA truth.

### Optional cycle diagnostics

After strong/weak production is stable, an optional leak/cycle diagnostic may
observe the object graph. A collector is not required for B′ completion and
must never call user `fini()`.

## Production activation table

| Milestone | ObjectCell production | user fini transaction | weak | adaptive mode |
| --- | ---: | ---: | ---: | --- |
| D0/BRC-DOC-R0/P0/D1/S0/R0/F0/W0/V0/ABI0/P1/BFIN-S0 | 0 | 0 | 0 | none |
| BCELL-I0 | 1 closed runtime family | family-dependent | admitted profile only | SharedRc |
| BCELL-SSA-I0 | 1 exact canonical BoxRef owner | unchanged | 0 unless separately admitted | SharedRc |
| BFIN-I0 | 1 closed fini-capable family | 1 | unchanged | SharedRc |
| BWEAK-I0 | admitted families | admitted families | 1 representation | SharedRc |
| BPLUGIN-I0 | plugin family only | plugin explicit route | admitted profile | SharedRc |
| BCELL-FAM-In / BHOST-I0 / BPTR-I0 / BBACKEND-I0 | one selected scope per row | only admitted families | only admitted families | SharedRc or one proven derived mode |
| BCELL-GLOBAL-R0 / BPRIME-CLOSE0 | all supported families | all supported fini families | all supported weak families | capability-gated |
| BADAPT-U0/L0/S0 | unchanged semantics | unchanged | unchanged | one selected derived mode |

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
| BCELL-FAM-In/Rn | only the selected family is cut over and its old authority reaches caller zero |
| BCELL-GLOBAL-R0 | supported-family legacy authorities are repository-wide caller zero |
| BPRIME-CLOSE0 | the full supported B′ profile satisfies the constitution counters |
| BADAPT-U0/L0/S0 | only the selected derived mode is proven for its closed profile |

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
```

## Stop conditions

Stop the active B′ row if it:

```text
changes the current SSA-RC-L0 behavior-neutral slice
creates a second production count beneath Arc
lets ordinary Rust Drop call user fini in an admitted family
lets parent ordinary fields implicitly fini shared children
uses one enum value to conflate lifecycle, payload, and residency axes
stores concurrent local/shared counts as parallel truths
exposes raw pointers as identity or across a lease boundary
reuses a slot before weak zero or after generation wrap
copies a weak value without a verified weak-count operation
upgrades weak through separate check/increment races
holds a cell/slab/method lock while invoking a user hook or drop callback
allows unknown FFI/thread escape into Unique or LocalRc
activates generic BoxRef, plugin, weak, and concurrency in one commit
silently retries a legacy Arc carrier
deletes legacy state before exact caller zero
```

## Immediate next action

None in this parked branch. Continue the current D′ taskboard at SSA-RC-L0.
When the B′ branch is selected, run BFIN-P0 first; do not start ObjectCell code
from this design card alone.
