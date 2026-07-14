---
Status: Accepted for parked tasking; production activation 0
Date: 2026-07-15
Decision: Anchored View Return ABI, modified-and-adopted
Parent taskboard: hakorune-ownership-v2-root-anchored-alias-task-2026-07-14.md
Current blocker remains: SSA-I1 trivial-profile/atomic-cutover design stop
First evidence prerequisite: O2-P0b1 exact final-callee/current-signature census
Related:
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - hakorune-ownership-v2-root-anchored-alias-task-2026-07-14.md
  - hakorune-ownership-v2-scoped-mutable-alias-consultation-2026-07-14.md
  - mirbuilder-ssa-i1-trivial-profile-atomic-cutover-design-stop-2026-07-14.md
---

# Hakorune Ownership V2 Anchored View Return ABI Task

## Authority and status

This document owns the parked projection/call-result-view subtask only. It does
not replace the parent Ownership V2 task order, the current SSA-I1 card, or a
future reference-language specification.

```text
current executable frontier:
  SSA-I1 trivial-profile/atomic-cutover design stop

evidence/selection owner:
  parent O2-P0b1 -> O2-D0

parked view-return task owner:
  this document after O2-D0 selects the branch

production View activation:
  0
```

Source spelling remains provisional until `PROJ-D0`. No SharedV1 or MoveV2
route may infer a View from a method name, runtime tag, pointer identity, or
observed reference count.

## Decision summary

Call-site spelling stays lightweight:

```hako
local child = node.get()
local token = stream.next()
```

The verified callable signature determines the result capability:

```hako
get(): view Node {
    return me.child
}

next(): Token {
    return new Token()
}
```

```text
unannotated/ordinary result capability:
  Owned by default in the MoveV2 edition

explicit view result:
  non-owning, anchored, task-local, noescape

explicit shared result:
  independent Shared owner, paid shared lane

runtime/method-name inference:
  0
```

`Owned` is the declared default capability, not a claim that every result needs
an ownership token. Integer, Bool, record, and other trivial representations
normalize to a verified Trivial result after type/representation resolution.

SharedV1 source behavior does not change when passive schema lands. Edition
normalization must be explicit, whole-unit, and must never retry another edition
after a canonical failure.

## Required corrections to the consultation answer

### 1. Return type transport already exists

Current `ASTNode::FunctionDeclaration` already has:

```rust
return_type_name: Option<String>
```

Instance/static parsers already preserve the optional return type. `PROJ-S0`
therefore adds a separate ownership/anchor syntax carrier; it does not invent
return typing and does not encode ownership grammar inside `return_type_name`.

Candidate passive shape:

```rust
pub struct ReturnOwnershipSyntaxV1 {
    pub capability: DeclaredReturnCapabilityV1,
    pub anchor: Option<FormalViewAnchorSyntaxV1>,
    pub domain: Option<ViewDomainSyntaxV1>,
}
```

The existing type string and the new ownership syntax are orthogonal axes.

### 2. Declared capability and verified representation are separate

```rust
pub enum DeclaredReturnCapabilityV1 {
    Owned,
    View(ViewReturnContractSyntaxV1),
    Shared,
}
```

```rust
pub enum VerifiedReturnValueClassV1 {
    Trivial,
    Owned,
    AnchoredView(VerifiedViewReturnContractV1),
    SharedOwned(SharedReturnKindV1),
}
```

`Trivial` is never a source ownership spelling. Type/representation authority
derives it after resolving the declared result type.

### 3. Shared result and view of Shared storage are different

```text
shared Service result:
  caller receives an independent Shared owner

view Service anchored to a Shared field:
  caller receives no owner; RC 0
```

The first View profile has no acquisition capability. A later separately
verified row may allow explicit `clone` from a View anchored to exact Shared
storage. Plain View never silently promotes Unique storage or creates a Shared
owner.

### 4. V1 anchor/domain vocabulary stays closed

First sealed vocabulary:

```rust
pub enum FormalViewAnchorV1 {
    Receiver,
    Parameter(u32),
}

pub enum ViewDomainV1 {
    WholeObject,
}
```

`Static`, exact field paths, named container domains, dynamic indices,
temporary receivers, and ViewPhi are later versioned rows. Unsupported states
are not represented as optional V1 fields or permissive booleans.

### 5. Callback/reentry is an effect boundary

The callable effect contract must include callback/reentry in addition to
capture, suspension, fini, and rehome. Unknown effects conservatively
invalidate `WholeObject` and stop before Builder effects.

### 6. View forwarding is the only safe escape exception

```text
field/global/collection store, capture, task/channel, await/yield:
  reject

return through an explicit verified View Return ABI with the same
formal anchor/domain:
  allow as view-to-view forwarding
```

View forwarding creates no independent owner and is not an escape into an
owned/shared destination.

## Semantic authority split

```text
Parser/AST:
  source type text plus return ownership/anchor syntax only

DeclaredCallableOwnershipSignatureV1:
  resolved source declaration or external manifest contract

VerifiedCallableOwnershipAbiV1:
  declaration + body provenance/effect proof, or trusted exact foreign ABI

VerifiedCallResultV1:
  formal anchor/domain substituted with the exact actual source root

VerifiedScopedLoanFlowV2:
  sole function-level Alias/View creation/use/last-use and conflict certificate

Binding SSA:
  only BindingRef -> ValueId reaching-value authority

Ownership SSA:
  Owned token create/forward/consume authority

Lower:
  materializes the verified call and consumes loan permissions

Runtime:
  View ownership work = 0
```

No product beside Binding SSA stores a second `BindingRef -> ValueId` map.
Formal signatures, verified body provenance, and actual call-site substitution
remain separate so a declaration is never mistaken for a body proof.
`VerifiedScopedLoanFlowV2` replaces/widens the parent
`VerifiedScopedAliasFlowV1`; it is not a parallel View-liveness authority.

## View return contract

Candidate verified shape:

```rust
pub struct VerifiedViewReturnContractV1 {
    pub anchor: FormalViewAnchorV1,
    pub domain: ViewDomainV1,
    pub acquisition: ViewAcquisitionV1,
}

pub enum ViewAcquisitionV1 {
    None,
}
```

The zero-RC first profile permits only `None`. Acquisition is versioned rather
than widening V1 with a partially supported flag.

### Anchor elision

```text
instance method `(): view T`:
  anchor = Receiver

free/static function with exactly one eligible borrowed input:
  anchor = that Parameter

multiple eligible inputs:
  explicit `from <parameter>` required

zero eligible inputs:
  reject in V1
```

Elision is derived from the verified callable signature, never from return
expression names or runtime values.

### Callee return provenance

Every reachable return must prove the declared formal anchor and domain.

First-profile accepted origins:

```text
receiver itself
declared parameter itself
direct field reached from the declared receiver/parameter,
  conservatively anchored to WholeObject
same-anchor verified View call forwarding
```

Rejected:

```text
callee-local owner
new temporary
foreign/global root
different formal anchor
branch returning different anchors
unknown/dynamic call result
```

Returning through a local ScopedAlias is accepted only when exact root
flattening proves the same formal anchor.

### Call-site substitution

```rust
pub enum VerifiedCallResultV1 {
    Trivial,
    Owned,
    SharedOwned,
    AnchoredView {
        callee: CallableIdV1,
        value_site: SourceExprSiteV1,
        actual_root: BindingRefV1,
        domain: ViewDomainV1,
        loan: LoanIdV1,
    },
}
```

The actual root is flattened through ScopedAlias chains before the loan is
sealed. Alias cycle, foreign owner, unknown final callee, ambiguous anchor, and
unsupported root profiles are typed errors before Builder effects.

## Mutation and invalidation law

Anchored View is mutable, non-exclusive, task-local, and non-owning.

Allowed in the first runtime profile:

```text
read through the View
non-consuming field write through the viewed object
sequential use through owner/View aliases
anchor operations after the View's last use
```

Rejected while the View is live:

```text
anchor/root take, rebind, destroy, fini, share, rehome
any anchor mutation in the WholeObject profile
unknown-effect call on the anchor or its aliases
View take/fini/share/clone
View store/return without exact View forwarding/capture/task/suspend
```

Mutation through a View does not imply exclusivity. LLVM `noalias`, Rust-style
alias-XOR-mutation assumptions, and mutation-based alias disappearance are
forbidden.

The loan begins after the returning call completes. A future `next_view()` may
mutate its receiver during the call, but its post-call anchor must still exist
and satisfy the declared contract.

## Callable effect contract

Candidate verified shape:

```rust
pub struct CallablePlaceEffectsV1 {
    pub invalidates: Box<[FormalViewDomainV1]>,
    pub may_fini: bool,
    pub may_rehome: bool,
    pub may_capture: bool,
    pub may_suspend: bool,
    pub may_callback_or_reenter: bool,
}
```

For a call while a View is live:

```text
callee.invalidates intersect active_view.domains = empty
may_fini/rehome/capture/suspend/callback_or_reenter = false
```

The first runtime slice does not require precise field effects: any unknown or
anchor-mutating call invalidates `WholeObject`. Exact fixed-field effects are a
later precision row.

## Interface, plugin, FFI, and dynamic boundaries

Ownership ABI is part of the callable signature and signature hash.

```text
interface/override V1:
  exact result capability, anchor, domain, and effects match

plugin:
  exact manifest vocabulary required before View activation

FFI:
  exact declared anchor/effect contract required

unknown dynamic call:
  never assumed View
```

Owned-to-View and View-to-Owned adapters are not implicit. An adapter that
would retain, copy, promote, or synthesize a hidden owner is rejected.

Dynamic Any is not part of the first runtime profile. Exact Any View support is
a later ABI row and still cannot use runtime type to choose ownership mode.

## Task dependency graph

```text
parent evidence/selection:
  O2-P0b1 -> O2-P0c -> O2-D0

passive callable contract:
  O2-D0 selects branch
    -> PROJ-D0
    -> PROJ-S0
    -> PROJ-ABI0
    -> PROJ-R0
    -> PROJ-CALL0
    -> PROJ-DIAG0

first runtime:
  {PROJ-CALL0, PROJ-DIAG0, UBOX-I0, UCALL-B0}
    -> PROJ-I0

precision/control extensions:
  PROJ-I0 -> PROJ-FIELD0 -> PROJ-EFFECT0
  PROJ-I0 -> PROJ-CFG0
  PROJ-I0 -> PROJ-TEMP0
  PROJ-FIELD0 -> PROJ-DOMAIN0

separate future decision:
  {PROJ-CFG0, LoanPhi evidence} -> VIEW-PHI-D0 -> VIEW-PHI-I0

post-ABI corpus proof:
  selected implemented ABI rows -> O2-P0b2
```

No PROJ row changes the active SSA-I1 pointer until the parent taskboard
explicitly selects and un-parks this branch.

## Claim-unit task order

### PROJ-D0 — decision lock

Freeze Owned default capability, explicit View/Shared spelling, anchor elision,
WholeObject V1, view-forwarding exception, no runtime/name inference, and the
versioned later-domain boundary. Reference docs remain unchanged before this
row.

### PROJ-S0 — passive syntax and transport

Add a return-ownership syntax carrier orthogonal to `return_type_name`. Update
all Rust/Hako parser, AST constructor, macro/JSON, delegate/property synthesis,
interface, and ProgramV0 transport surfaces from one generated producer/
consumer ledger.

Acceptance:

```text
production behavior delta = 0
legacy/default syntax round-trip unchanged
ownership syntax exact round-trip
missing/unknown syntax typed reject
all constructor/transport surfaces accounted
```

### PROJ-ABI0 — verified callable ABI

Seal declared result capability, formal anchor/domain, callable effects,
interface/override equality, signature hash, and exact external metadata
vocabulary. Lower connection remains zero.

### PROJ-R0 — return provenance verifier

Verify every reachable return against the declared anchor/domain. Reject local
owners, temporaries, different anchors, ambiguous branches, unverified inner
calls, non-local exits, and missing return coverage.

### PROJ-CALL0 — actual anchor substitution and loan

Resolve the exact final callee, substitute receiver/parameter anchors, flatten
the actual root, create one LoanId, seal last-use/conflict permissions, and
publish no MIR identity.

### PROJ-DIAG0 — typed diagnostics

Land stable machine-readable errors and human golden messages before runtime
activation:

```text
AmbiguousViewAnchor
ViewReturnProvenanceMismatch
UnknownViewCallableAbi
UnsupportedViewAnchorProfile
ViewAnchorInvalidation
ViewEscapeRejected
ViewAcquisitionForbidden
JoinRequiresViewPhi
```

Each carries exact callable/call site, formal/actual anchor when available,
View creation, conflict, next-use frontier, and a context-specific remediation.
No diagnostic suggests hidden retain, promotion, or unsafe raw by default.

### PROJ-I0 — first production View

Closed grammar:

```text
instance method with explicit View result
explicit local UniqueConcreteBox receiver root
WholeObject domain
straight-line local initializer
callee return is receiver/direct receiver field
direct read/write through result
local last-use
no anchor mutation/call while live
no parameter/shared/resource/dyn/Any/temporary/control/callback
```

Required zeros:

```text
CopyOwned/DestroyOwned for View = 0
runtime ownership branch = 0
callee-name inference = 0
hidden root/Shared promotion = 0
second BindingRef -> ValueId map = 0
```

### Later precision rows

```text
PROJ-FIELD0:
  exact static field-path anchor and overwrite/take exclusion

PROJ-EFFECT0:
  exact non-invalidating call effects while View is live

PROJ-CFG0:
  dominating stable View and branch-local View in If/Loop; ViewPhi 0

PROJ-TEMP0:
  Ownership-SSA temporary anchor extension; hidden RC 0

PROJ-DOMAIN0:
  declared container/buffer domain and structural invalidation

VIEW-PHI-D0/I0:
  separate same-root/domain join decision and implementation
```

Static views, dynamic index, different-root ViewPhi, suspension, cross-task
views, and generic Any remain outside these rows.

## Required fixtures

Pass:

```text
ordinary/owned call result produces an owner or Trivial value
receiver View result emits ownership op 0
direct receiver-field return conservatively anchors WholeObject
View through ScopedAlias receiver flattens to the original root
read and sequential mutation through View
anchor mutation after View last-use
same-anchor callee-local alias forwarding
view-to-view return forwarding with the same formal anchor/domain
```

Reject:

```text
view return of callee-local owner or new temporary
declared/actual anchor mismatch
ambiguous multi-parameter elision
anchor take/rebind/destroy/fini/share/rehome while View live
anchor mutation or unknown-effect call while WholeObject View live
View field/global/collection store, capture, task, await, or ordinary Owned return
View take/fini/share/clone in the no-acquisition profile
branch-selected View without ViewPhi
unknown dynamic/interface/plugin/FFI ABI
method name used as ownership evidence
```

Diagnostics are golden-tested through both human and machine-readable output.
LoanTrace may observe creation/use/end/conflict only; it cannot retain an object
or delay reclamation.

## Implementation may claim

After `PROJ-I0` only:

```text
one explicit instance View ABI maps a call result to a WholeObject-anchored,
same-task, mutable, non-owning local View

the View performs no ownership runtime operation

the anchor cannot be invalidated before the View's local last-use
```

## Implementation must not claim

```text
all getters are Views
unannotated SharedV1 returns changed meaning
exact field or container-domain invalidation
temporary receiver extension
ViewPhi
borrowed return without explicit View ABI
View acquisition/clone
Any/dyn/plugin/FFI View support
static/cross-task/suspending View
all function families or default route cut over
```

## Stop conditions

Stop the slice if any implementation:

1. parses ownership as an opaque suffix inside `return_type_name`;
2. uses `Trivial` as a source ownership spelling;
3. treats a Shared owner result and a View of Shared storage as one mode;
4. infers View from method name, AST call shape, runtime type, pointer, or RC;
5. accepts an unknown callback/reentry/effect as non-invalidating;
6. gives View an ownership token, `CopyOwned`, or `DestroyOwned`;
7. creates a second reaching-value map outside Binding SSA;
8. allows ordinary View escape without exact view-to-view forwarding;
9. silently promotes Unique storage so a View can escape or clone;
10. introduces Static/NamedDomain/ViewPhi as optional unsupported V1 states;
11. activates production before provenance, loan, diagnostics, and backend
    fail-fast contracts are complete;
12. changes the current SSA-I1 pointer or retries a SharedV1 route.

## Next action

Do not implement PROJ code now. Keep the current SSA-I1 frontier. When the
parent Ownership V2 branch is unparked, run O2-P0b1 first; only O2-D0 may select
this subtask. The first code-facing task after `PROJ-D0` is the passive,
ledger-backed `PROJ-S0` syntax/transport slice.
