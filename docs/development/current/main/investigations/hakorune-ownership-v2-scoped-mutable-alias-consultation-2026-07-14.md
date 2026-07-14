---
Status: Historical consultation; modify-and-adopt decision returned
Date: 2026-07-14
Scope: Hakorune Ownership V2 source ergonomics and the zero-RC local-alias law
Decision requested: close the smallest sound scoped-mutable-alias profile
Decision returned: Root-Anchored ScopedBoxAlias direction, with taskboard corrections
Execution authority:
  - hakorune-ownership-v2-root-anchored-alias-task-2026-07-14.md
Related:
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md
  - mirbuilder-ssa-rc0-owned-alias-materialization-design-stop-2026-07-14.md
  - mirbuilder-ssa-i1-trivial-profile-atomic-cutover-design-stop-2026-07-14.md
Non-authority: this consultation does not change the current language or activate production Ownership SSA
---

# Hakorune Ownership V2: Scoped Mutable Alias Final Consultation

## Consultation closeout

The core proposal is accepted for tasking:

```text
mutable task-local whole-root alias
L1 path-sensitive local last-use
P0 first production projection profile
C0 no alias reassignment/PHI
F0 live alias forbids fini
explicit take/share/clone/raw lanes
```

The taskboard applies these required corrections:

```text
AliasId is a source capability, not a Binding SSA definition or MIR value
the anchor may be caller-owned parameter/receiver, not only a local owner
share conversion is not CopyOwned; clone is CopyOwned
B-prime implicit sharing is version-scoped to SharedV1/paid shared substrate
Outbox/Any/dyn/field/index are excluded from the first alias profile
escape is not RC; MoveToOwner remains zero-RC
first activation is ordinary non-resource Unique OwnedLocalRoot only
debug validation is observation-only LoanTrace, never shadow RC
LoanPhi is a future evidence-gated branch, not a V1 relaxation
```

A preliminary corpus census also showed that direct field/index projection is
small relative to call-result exposure. The durable evidence path is therefore
split into syntax census, exact root eligibility, current final-callee
evidence, and ownership-destination evidence before O2-D0 freezes P0/P1.
Final V2 call-result classifications are audited only after their ABI rows
exist; the evidence path must not depend on the Decision it is meant to inform.

The original decision request follows as historical consultation input. It is
not an implementation checklist; all row order, fixtures, diagnostics,
may-claims, and stop conditions are owned by the successor taskboard.

## 0. Requested decision

We want to preserve the current lightweight spelling:

```hako
local a = new Node()
local b = a

print(a.value)
print(b.value)
```

while avoiding an implicit retain/release pair for every local alias.

The proposed refinement is:

```text
local b = a:
  free task-local non-owning alias
  a remains the sole owner
  both names may access and mutate the same whole Box

take a:
  explicit ownership transfer

share a / clone s:
  explicit independent lifetime
  this is the paid RC lane

unsafe raw:
  unchecked manual-lifetime lane
```

Please determine whether this is the cleanest final source model and close its
exact V1 boundary. In particular, resolve the two questions identified by the
review:

1. Is mutation through the free alias allowed?
2. Which control-flow uses are rejected so that V1 needs no alias/borrow PHI?

This is a refinement of the accepted D-prime Binding SSA and B-prime runtime
substrate, not authorization to implement a new route.

## 1. Fixed context

The following decisions should remain fixed unless this consultation finds a
direct contradiction.

```text
Binding SSA:
  the only BindingRef -> reaching ValueId authority

Ownership SSA:
  Owned / Borrowed / None token discipline
  path-sensitive consuming-use verification

Copy:
  ownership-neutral

CopyOwned:
  create one independent owned token for the same object identity

DestroyOwned:
  consume exactly one owned token

B-prime shared/resource substrate:
  explicit fini is distinct from token destruction
  tombstone, weak generation, lifecycle lease, and RC are paid axes
  retained for explicit shared/resource/weak/plugin lanes

production status:
  Binding SSA activation = 0
  Ownership SSA activation = 0
  current SSA-I1 design stop remains active
```

The earlier ownership planner sealed this SharedV1 law:

```text
different BindingRef + borrowed strong value
  -> CopyOwned before replacing the destination
```

The proposed MoveV2 local-alias law is different:

```text
local b = a
  -> b is non-owning
  -> CopyOwned = 0
  -> b is not destroyed at scope close
```

Please state explicitly whether the old planner law becomes:

```text
SharedV1 compatibility only
```

and whether one edition-normalization layer should feed one common resolved
ownership plan, Binding SSA, Ownership SSA, MIR, and runtime.

## 2. Why neither implicit move nor implicit sharing is satisfactory

### Implicit move

```hako
local a = new Node()
local b = a
print(a.value) // use-after-move
```

This makes ordinary source unnecessarily affine and creates a very large
migration burden for the mostly-untyped selfhost corpus.

### Implicit independent sharing

```hako
local b = a // hidden CopyOwned / retain
```

This preserves source ergonomics but loses a predictable zero-ownership-cost
contract. A later return, capture, or unknown call may silently change the
representation and cost of an otherwise local alias.

### Proposed middle path

```hako
local a = new Node() // Owned
local b = a          // ScopedAlias; non-owning

print(a.value)       // valid
print(b.value)       // valid

local c = take a     // explicit move; only after b's loan ends
```

For this common local case:

```text
CopyOwned = 0
DestroyOwned for b = 0
RC increment/decrement = 0
atomic = 0
control cell = 0
```

The honest boundary is that two aliases with independent dynamic lifetimes
cannot simultaneously have all three properties below without another
lifetime mechanism:

```text
independent lifetime
reliable memory safety
zero RC / zero GC / zero arena / zero owner tracking
```

Independent lifetime therefore remains explicit `share` / `clone`, arena, or
an explicit unsafe lane.

## 3. First terminology question

Please choose a precise semantic name for the value created by:

```hako
local b = a
```

The candidates are:

```text
A. independent owner token
   not free; requires CopyOwned or another acquisition

B. non-owning task-local alias/view
   one Owned token remains with a
   multiple BindingRefs may temporarily read the same ValueId

C. move
   a becomes unavailable
```

The proposal is B.

Because the proposed alias may mutate the whole Box, `read borrow` may be a
misleading Rust-derived name. Please choose among names such as:

```text
ScopedAliasV1
TaskLocalViewV1
BorrowedBoxAliasV1
```

and state whether it belongs to `MirOwnershipKindV1::Borrowed` or needs a
separate source/resolved loan kind while remaining non-owning in MIR.

## 4. Mutation law

The strongest ergonomic proposal is intentionally weaker than Rust's
alias-XOR-mutation rule:

```hako
local b = a

b.value = 1
print(a.value) // 1

a.other = 2
print(b.other) // 2
```

Candidate law:

```text
whole-Box alias:
  sequential field read/write allowed
  non-consuming methods allowed
  mutating methods allowed

while any alias is live:
  owner token move forbidden
  owner replacement/destruction forbidden
  fini forbidden
  representation rehome/share conversion forbidden
  alias escape forbidden

alias itself:
  take forbidden
  fini forbidden
  return/store/capture/task transfer forbidden
```

This can be memory-safe if every payload access goes through the whole Box,
the owner cannot disappear or rehome while a loan is active, and the alias is
task-local. It does not give a `noalias` optimization promise.

Please decide:

1. May both `a` and `b` mutate the whole Box sequentially?
2. May a mutating method receive the alias as `me`?
3. Must the optimizer treat all such aliases as mutually aliasing memory?
4. Is the alias binding itself immutable in V1?
5. Is owner reassignment forbidden until the alias's last use?

Counterexample that must have one exact answer:

```hako
local b = a
b.value = 1
a = new Node()
print(b.value)
```

Do not solve this by silently retaining the old object. Either reject the
owner reassignment while `b` is live, or name another explicit owner.

## 5. Interior and projection boundary

`whole Box alias` and `pointer into payload storage` must not be conflated.

```hako
local b = a             // whole-object alias
local child = a.child   // field-derived value
local item = a.items[i] // index-derived value
```

The second and third forms create a separate problem:

```hako
local child = parent.child
parent.child = new Node()
child.use()
```

If `child` is free and non-owning, replacing the field can destroy its owner.
Please choose the V1 rule explicitly:

```text
P0. whole-root only
    `local b = a` may be free
    field/index results require take, independent owner acquisition, or reject

P1. projection-sensitive loan
    BorrowPlaceV1 = exact BindingRef root + sealed field path
    overwriting any prefix of the path is forbidden while the loan is live
    ownership remains field-unsplit; no partial move, field PHI, or drop flags

P2. paid field acquisition
    field read creates an independent owner

P3. unchecked interior alias
    unsafe lane only
```

If `interior borrow is forbidden`, please distinguish:

```text
raw pointer/slice into inline payload:
  forbidden in safe V1

Box-valued stored field:
  choose P0, P1, or P2 explicitly
```

Array/Map/dynamic index projections must not be assumed disjoint from names,
spans, runtime tags, or encounter order.

## 6. Loan lifetime and diagnostics

The proposed alias is deliberately second-class:

```text
no source lifetime parameters
no field/global/collection storage
no return/outbox
no closure capture
no Future/channel/context snapshot
no thread/task transfer
no await/yield suspension
no unknown retaining call
```

Please select its V1 end rule:

```text
L0. lexical block end
    simplest verifier
    may reject `take a` even after b's last use

L1. local last-use end
    no lifetime annotations
    flow analysis shortens only non-escaping task-local loans
    still no borrow PHI or cross-function lifetime
```

Preliminary preference: L1 if it can remain a bounded pre-Builder loan-flow
analysis. Otherwise land L0 first and make the diagnostic suggest a smaller
block.

Required diagnostic shape:

```text
cannot consume `a`: task-local alias `b` remains live
alias created at: <exact site>
next use at:      <exact site>

help:
  - end or narrow the alias before this operation
  - use `take a` if transfer was intended
  - use `share a` / `clone` for independent lifetime
  - use an explicit unsafe raw alias only when manual lifetime is intended
```

## 7. Control-flow law

The simple spelling hides a critical PHI question:

```hako
local x = a
if cond {
    x = b
}
use(x)
```

If `x` is a non-owning alias, the join requires a non-consuming alias PHI. An
Owned PHI is not equivalent because it forwards/consumes the selected owner
token and may invalidate the source owner.

Please decide the minimum V1:

```text
C0. reject alias-binding reassignment and alias-valued joins
    branch-local aliases are allowed when they end inside the branch
    aliases created before the branch may be used in dominated branch bodies

C1. introduce non-consuming alias PHI
    broader grammar, but a new verifier/SSA vocabulary is required
```

Preliminary recommendation: C0.

Likewise, `loop-carried borrow forbidden` must not reject a stable borrowed
parameter merely used inside a loop:

```hako
inspect(xs) { // borrowed noescape parameter
    loop i in 0..n {
        xs.get(i)
    }
}
```

Please distinguish:

```text
allowed:
  dominating stable alias/borrow used through nested If/Loop

rejected in V1:
  alias selected or redefined by a PHI
  alias created on one predecessor and used after the join
  loop-carried alias reassignment
```

Also decide how exception/cleanup edges affect the loan end. Builder must not
discover this after effects begin.

## 8. Method, dynamic dispatch, plugin, and FFI boundary

A callee may mutate through a whole-Box alias only if it cannot keep or consume
that alias.

Required callee contract candidate:

```text
BorrowMutatingNoEscapeV1:
  read/write payload allowed
  return alias forbidden
  field/global/registry storage forbidden
  closure/task capture forbidden
  take/fini/share/rehome forbidden
```

Please decide:

1. Which pre-Builder product proves this method/call effect?
2. Does ordinary parameter/receiver syntax imply noescape borrow in MoveV2?
3. Are dynamic dispatch, plugins, reflection, and unknown FFI rejected unless
   an exact ownership/effect ABI is sealed?
4. May callbacks re-enter the same object while a mutable alias is live?
5. Does `shared sync` remain a separate capability and paid atomic lane?

No backend or runtime tag may guess that an unknown call is noescape.

## 9. Concurrency and suspension

The alias must be task-local unless another ownership mechanism is selected.

```hako
local b = a
co {
    local f = nowait mutate(a)
    mutate(b)
}
```

Please confirm the V1 law:

```text
alias capture / channel send / task transfer / await-live loan:
  reject before Builder effects

cross-thread unique move:
  move the sole owner only after all aliases end

cross-thread independent sharing:
  explicit shared sync / atomic ownership lane
```

Current sequential implementation of a task primitive must not weaken its
language-level concurrency boundary.

## 10. Dynamic Any and untyped source

The selfhost corpus contains many untyped locals. Availability must never
depend on the runtime value kind.

The proposed static source law is:

```text
local b = a where a is an existing place:
  always non-owning alias intent
  a remains the owner regardless of runtime payload kind

local b = take a:
  always consuming move intent

local b = clone s:
  always explicit independent-owner intent
```

For trivial representations, mapping two bindings to the same immutable SSA
value needs no lifetime action. For an ownable representation, the alias has a
loan against the exact owner token. This distinction must be sealed before
Builder effects, not inferred from `VMValue` or runtime tags.

Please decide:

1. Is untyped `Any` a paid dynamic lane or may it carry an exact owner token?
2. Are Any return/store/PHI/unknown-call escapes rejected until a typed ABI is
   known?
3. Does `Contract.ZeroOwnershipCost` reject Any/dyn/unknown ownership calls?
4. What exact resolved product prevents runtime copy-vs-move inference?
5. How are mixed trivial/owned branch results treated without a runtime
   ownership-mode branch?

Example that must fail or acquire explicitly:

```hako
id(x) {
    local y = x
    return y
}
```

If return is owned, a borrowed `y` cannot satisfy it silently.

## 11. Resource and fini interaction

```hako
local b = file
file.fini()
b.read()
```

There are two different representation cases:

```text
unique resource without tombstone:
  fini invalidates the payload observed by b

explicit shared resource under B-prime:
  stable lifecycle cell can expose Dead to independent owners
```

Please decide the V1 rule:

```text
F0. any live task-local alias forbids fini through both owner and alias

F1. unique alias forbids fini, but a shared-resource alias may observe Dead
    under the B-prime lifecycle lease/state contract

F2. path-sensitive alias-group typestate marks all aliases Dead
```

Preliminary recommendation: F0 for the first free-alias profile. Open F1 only
with the explicit shared-resource substrate. Do not introduce F2 in the first
slice.

Also confirm that alias presence forbids owner `take`, `DestroyOwned`, scope
destruction, field owner replacement, and any representation conversion that
could move the payload.

## 12. Unsafe self-responsibility lane

Hakorune may retain a small explicitly unsafe manual-lifetime lane:

```text
safe/default local alias:
  compiler-checked lifetime
  zero RC

explicit shared:
  RC-backed independent lifetime

unsafe raw alias:
  no owner tracking guarantee
  double free/use-after-free/stale pointer may be undefined behavior
```

Debug builds may add poison, generation side tables, allocation quarantine,
and source-site traces. They improve detection but cannot guarantee detection
after the physical allocation has been freed and reused without retaining
metadata or checking every access.

Please state:

1. Is raw alias syntax permitted only inside an explicit unsafe boundary?
2. Are debug checks best-effort diagnostics rather than release semantics?
3. Does safe `local b = a` never silently degrade to raw alias?
4. Does a zero-cost contract allow unsafe raw only under a separate explicit
   contract, or reject it entirely?

## 13. Proposed authority split

Please review this separation:

```text
resolver / verified source ownership:
  Own / ScopedAlias / Take / Share / Clone intent
  exact alias root or exact projection path
  noescape/effect ABI

pre-Builder verified loan flow:
  alias creation and last use
  mutation conflicts
  escape rejection
  take/fini/rehome/reassignment exclusion
  CFG join and suspension rejection
  no ValueId / BasicBlockId

Binding SSA:
  sole BindingRef -> ValueId authority
  owner and alias may reference the same ValueId
  no second reaching-value map

Ownership SSA:
  sole Owned token consuming-use authority
  alias BindingRef does not create or destroy a token

MIR materialization:
  move/forward = runtime ownership instruction 0
  scoped alias = CopyOwned 0 / DestroyOwned 0
  explicit shared clone = CopyOwned

B-prime runtime substrate:
  explicit shared/resource/weak/host/plugin paid lanes only
```

Please name the exact products and their ownership. In particular, explain how
one Owned token referenced by multiple temporary BindingRefs is verified
without introducing a second alias map synchronized with Binding SSA.

## 14. Cross-edition migration boundary

If SharedV1 remains temporarily supported, source editions may coexist, but
the compiler must not have two independent SSA/MIR/runtime authorities.

Candidate migration shape:

```text
SharedV1 / MoveV2 source
  -> temporary edition-specific source ownership normalizer
  -> ONE ResolvedOwnershipPlanV2
  -> ONE Binding SSA + Ownership SSA
  -> ONE MIR/runtime
```

Please decide:

1. Does the SharedV1 normalizer alone produce legacy implicit CopyOwned plans?
2. Must every CopyOwned carry provenance such as:

   ```text
   ExplicitClone
   LegacyImplicitShare
   CrossEditionBridge
   ```

3. Does MoveV2 reject every non-explicit clone provenance?
4. Is first migration direction limited to exact V1 caller -> V2 noescape
   borrow/trivial calls, with V2 -> V1 rejected initially?
5. What exact zero counters retire SharedV1 semantics from the main compiler?

Do not permit canonical failure to retry under the other ownership edition.

## 15. Historical minimum V1 candidate

The remaining sections preserve what the consultation asked reviewers to
return. They are superseded for execution by the successor taskboard.

The narrowest coherent first profile found by the review is:

```text
local b = a:
  straight-line task-local whole-Box alias
  same-thread sequential mutation allowed
  binding reassignment forbidden
  owner move/rebind/fini/share/rehome forbidden while live
  escape/take/fini/capture/suspension forbidden

borrowed parameter:
  dominating live-through use in nested If/Loop allowed
  Borrowed/Alias PHI, Return, capture, and await forbidden

field/index result:
  not a free alias in first profile

dynamic Any:
  zero-cost profile excludes unknown ownership escape

shared/resource:
  remains explicit paid B-prime lane
```

Please accept this, replace it with a cleaner equally small profile, or explain
why the proposal is unsound.

## 16. Historical requested fixtures

### Must pass if scoped mutable alias is accepted

```text
local a = new Box; local b = a; read through both
sequential field mutation through a and b, observed by the other alias
non-consuming noescape method call through b
stable borrowed parameter used inside nested If and Loop without alias PHI
alias ends before an explicit take of the owner
trivial local alias emits no ownership instruction
same runtime object identity observed through a and b
```

### Must reject in V1

```text
owner move while alias remains live
owner replacement while alias remains live
DestroyOwned or fini through owner while alias remains live
take/fini/share/rehome through the alias
alias return, outbox, field/global/collection storage
closure/Future/task/channel capture
alias live across await/yield
unknown dynamic/plugin/FFI call without noescape ABI
branch-selected alias requiring an alias PHI
loop-carried alias reassignment
field/index free alias when projection loans are not selected
Any ownership escape without an exact ABI
implicit CopyOwned or silent RC promotion on the MoveV2 local-alias path
```

### Diagnostics and counters

```text
creation site + conflicting operation site + next-use site
one actionable help for take, one for share/clone, one for scope narrowing
MoveV2 hidden CopyOwned = 0
local alias DestroyOwned = 0
runtime copy-vs-move inference = 0
unknown call noescape assumption = 0
Borrowed/Alias PHI in V1 = 0
silent raw fallback = 0
```

## 17. Historical requested response format

Please return a concrete final decision containing:

1. adopt/reject/modify the scoped-alias proposal;
2. final semantic name and source spelling for `local b = a`;
3. whether whole-Box mutation and mutating methods are allowed;
4. owner/alias reassignment and last-use law;
5. whole-root versus projection/field/index law;
6. exact CFG/PHI/Loop boundary;
7. method, dynamic dispatch, plugin, FFI, callback, and concurrency ABI;
8. Dynamic Any and zero-cost-contract law;
9. resource/fini interaction;
10. unsafe raw and debug-diagnostic boundary;
11. exact pre-Builder product and Binding/Ownership SSA responsibilities;
12. how the old SSA-RC0 `CopyBorrowedStrong` law is scoped or retired;
13. cross-edition bridge and SharedV1 sunset counters;
14. revised implementation order from SSA-I1-T through first real Box,
    arena, explicit shared, and resource/weak lanes;
15. required fixtures, guards, may-claims, must-not-claims, and stop conditions.

## 18. Historical requested nonclaims

```text
language syntax accepted
existing assignment semantics changed
SSA-I1 production activation
Ownership SSA production activation
Borrowed/Alias PHI support
interior/projection borrow support
resource typestate support
cross-thread alias support
Dynamic Any ownership closure
Arena implementation
SharedV1 retirement
default source route cutover
```

## 19. Historical requested stop conditions

Stop the design or implementation if it requires any of the following:

```text
free alias BindingRef receives its own DestroyOwned
one runtime type chooses whether `local b = a` copies or moves
borrow/alias PHI is required while V1 claims it is absent
branch-selected alias is accepted without an exact alias-PHI contract
unknown call, plugin, dynamic dispatch, or FFI is assumed noescape
field/index alias survives owner-slot replacement without a loan or owner token
owner move, fini, rehome, or destruction occurs while a free alias is live
one alias is sent to another task while the owner remains local
mutation through aliases is used to infer LLVM-style noalias
safe alias silently becomes raw
escape silently promotes a unique value to shared RC
Binding SSA is accompanied by a second reaching-value/alias map
SharedV1 and MoveV2 have separate MIR/runtime ownership authorities
canonical failure retries under another edition
debug sanitizer behavior is claimed as release memory safety
```

## 20. Historical preliminary recommendation

The current recommendation is:

```text
local b = a:
  zero-cost task-local whole-Box ScopedAliasV1
  sequential shared mutation allowed
  owner token remains solely with a

V1 restrictions:
  alias binding immutable
  owner rebind/move/fini/share/rehome forbidden while live
  no escape, capture, suspension, alias PHI, or loop-carried redefinition
  no field/index free alias until a separate projection decision

independent lifetime:
  explicit share / clone

manual lifetime:
  explicit unsafe raw only

runtime cost:
  local alias pays zero RC
  only explicit shared lifetime pays RC
```

This keeps the current source feel, removes the main MoveV2 migration cliff,
and preserves a machine-checkable zero-ownership-cost contract without hidden
promotion.
