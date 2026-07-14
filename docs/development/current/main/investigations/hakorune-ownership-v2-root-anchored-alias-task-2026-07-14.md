---
Status: Active taskboard; accepted for tasking and parked
Date: 2026-07-14
Decision: Root-Anchored ScopedBoxAlias V2 direction
Current activation: 0
Current blocker remains: SSA-I1 trivial-profile/atomic-cutover design stop
First O2 row: O2-P0a machine-readable initializer-shape census
Related:
  - hakorune-ownership-v2-scoped-mutable-alias-consultation-2026-07-14.md
  - hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md
  - mirbuilder-ssa-i1-trivial-profile-atomic-cutover-design-stop-2026-07-14.md
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - ../DOCS_LAYOUT.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md
---

# Hakorune Ownership V2 Root-Anchored Alias Taskboard

## Authority and reading order

```text
current executable frontier:
  SSA-I1 trivial-profile/atomic-cutover design stop

parked Ownership V2 execution owner:
  this taskboard

historical question packet:
  hakorune-ownership-v2-scoped-mutable-alias-consultation-2026-07-14.md

future long-lived language/design SSOT:
  design/ownership-v2-root-anchored-alias-ssot.md
  create only at O2-D0 after machine evidence
```

The consultation is evidence, not an implementation checklist. This taskboard
is the only O2 task-order owner until O2-D0 creates the long-lived SSOT.
`CURRENT_STATE.toml`, `CURRENT_TASK.md`, `10-Now.md`, and the active SSA-I1
card remain unchanged while this roadmap is parked.

## Objective

Keep Hakorune's lightweight local spelling while making hidden ownership cost
impossible on the V2 route.

```hako
local a = new Node() // one owner
local b = a          // free scoped alias; a remains usable

b.value = 1
print(a.value)

local c = take a     // explicit owner transfer after b's last use
local s = share c    // explicit Unique -> Shared conversion
local t = clone s    // explicit independent shared owner
```

```text
Scoped alias:
  owner-token delta = 0
  RC/opcode/ValueId delta = 0
  independent lifetime = no

take:
  one owner token forwarded
  runtime ownership opcode = 0

share:
  one Unique token consumed
  one Shared token produced
  representation conversion, not duplication

clone:
  one Shared owner added
  CopyOwned
```

This roadmap is parked beside D-prime. It does not replace the current
SSA-I1 design stop or authorize production behavior changes.

## Ergonomic cost law

Scope escape and owner multiplication are different facts. A Unique owner may
cross a function, object, closure, or task boundary by move without RC.

| Source intent | Owner-count delta | Required ownership action |
| --- | ---: | --- |
| exact noescape call | 0 | scoped loan, ownership opcode 0 |
| owned Return | 0 | token forward, ownership opcode 0 |
| owning field store | 0 | token forward, ownership opcode 0 |
| move capture / `spawn move` | 0 | token forward, ownership opcode 0 |
| arena-bound reference | 0 | region-bound loan, per-object RC 0 |
| explicit `share` after all loans end | 0 | Unique-to-Shared representation conversion; paid shared lane |
| explicit `clone` of a Shared root | +1 | independent owner plus `CopyOwned` |
| foreign or unknown destination | unknown | typed preflight stop |

> Escape alone never selects RC. Explicit `share` selects the paid Shared
> representation without increasing the owner count; `clone` adds an
> independent owner and is the `+1` operation.

No `EscapeToRc`, hidden promotion, or unknown-call fallback exists on the V2
route.

## Decision accepted for tasking

| Boundary | Decision |
| --- | --- |
| alias | `RootAnchoredScopedBoxAliasV1` |
| mutation | same-task sequential whole-Box mutation allowed |
| lifetime | L1, path-sensitive local last-use |
| first projection profile | P0, exact whole-root only |
| control | C0, no alias reassignment or alias PHI |
| fini | F0, live alias forbids fini |
| independent lifetime | explicit `share` / `clone` |
| manual lifetime | explicit unsafe raw lane only |

The direction is accepted with five required corrections to the Pro answer.

### 1. Alias identity is separate from alias flow

The identity and flow products are separate resolver/source capabilities:

```text
AliasIdV1
  -> exact stable AliasAnchorV1
  -> exact creation/use sites

VerifiedScopedAliasFlowV1
  -> source-CFG live-in/live-out
  -> exact source-CFG program points and successors
  -> exact root-effect and boundary-use dispositions
```

`AliasIdV1` never owns liveness. `VerifiedScopedAliasFlowV1` is the only
pre-Builder loan certificate and contains no MIR identity.

It creates none of:

```text
Binding SSA definition
ValueId
Owned/Borrowed MIR kind
CopyOwned / DestroyOwned
MIR instruction
```

Each alias use resolves to its ultimate anchor and asks the one function-owned
Binding SSA for that anchor's reaching value. The alias product stores no
ValueId and is not a second reaching-value map.

### 2. Passive anchor vocabulary is wider than first activation

Closed passive anchor vocabulary:

```text
OwnedLocalRoot
OwnedTakeParameterRoot
BorrowedParameterRoot
BorrowedReceiverRoot
SameTaskSharedRoot
```

This vocabulary is not the `ALIAS-I0` activation allowlist. The first
executable row accepts only:

```text
OwnedLocalRoot
  + exact UniqueConcreteBox
  + ordinary non-resource Box
  + same-task/non-sync representation
  + no fini/tombstone/weak/stable-handle/dyn/Any capability
```

`OwnedTakeParameterRoot` waits for the take-parameter ABI row.
`BorrowedParameterRoot` and `BorrowedReceiverRoot` wait for exact ordinary-Box
borrow/call ABI rows. `SameTaskSharedRoot` waits for `SHARE-LOCAL-I0` and a
separate alias activation row. Resource and SharedSync roots require the later
B-prime lifecycle-lease profile; root names never imply those capabilities.

First-profile rejects:

```text
Outbox/result slot
Upvar/capture
field/index place
temporary
Any/dyn/unknown root
```

Alias chains flatten to one ultimate root. Cycles, forward roots, foreign
roots, and roots outside the task/source lifetime are rejected.

### 3. Share conversion is not CopyOwned

```text
move:
  one token -> one token; opcode 0

ShareOwnedLocal / ShareOwnedSync:
  Unique consumed; one Shared token produced; not publication

CopyOwned:
  existing Shared token remains; one new Shared token produced

PublishOwnedSync:
  existing SharedSync token published; not conversion
```

`CopyOwned` on Unique, hidden Unique promotion, and implicit Local-to-Sync
promotion are forbidden.

### 4. B-prime is version-scoped

B-prime remains authority for:

```text
SharedV1 compatibility
explicit shared/resource Boxes
tombstone/fini transaction
weak generation and upgrade
identity/root split
plugin/host lifecycle
legacy caller-zero retirement
```

V2 supersedes these B-prime source defaults:

```text
normal Box is implicitly independently shareable
ordinary local/field alias is shared-strong
hidden adaptive promotion is default source behavior
different BindingRef implies an owned copy
```

The B-prime SSOT is reconciled only at O2-D0 after the census.

### 5. Dynamic Any and Outbox are later profiles

Runtime type must never choose whether `local b = a` is copy, move, or alias.
The first executable alias profile requires an exact static Box anchor.
`Any`, dyn, Outbox, String, Null/Void, erased values, and plugins are separate
ABI/compatibility rows.

## Candidate boundary disposition vocabulary

These vocabularies become authoritative only when O2-D0 closes. Alias boundary
permission and an owner-token destination are different facts and must never be
collapsed into one enum.

```rust
enum AliasBoundaryDispositionV2 {
    NoEscape,
    LoanWithin(LoanRegionIdV2),
    RejectEscape(AliasEscapeKindV2),
}

enum OwnerDestinationDispositionV2 {
    LocalUse,
    MoveToOwner(OwnershipDestinationV2),
    ArenaBound(AllocationRegionId),
    ConvertUniqueToShared(SharedDomainV2),
    CloneSharedRoot(SharedDomainV2),
    ForeignOrUnknown,
}
```

| Disposition | Closed meaning |
| --- | --- |
| `NoEscape` / `LoanWithin` | alias stays non-owning; ownership action 0 |
| `RejectEscape` | typed alias-boundary error before Builder effects |
| `LocalUse` | root remains in its ownership domain |
| `MoveToOwner` | after all loans end, explicit take/consuming ABI forwards the token; RC 0 |
| `ArenaBound` | region owns lifetime; per-object RC 0 |
| `ConvertUniqueToShared` | explicit share converts one Unique token to one Shared token; delta 0 |
| `CloneSharedRoot` | explicit clone adds one Shared owner; delta +1 |
| `ForeignOrUnknown` | typed stop before Builder effects |

An alias rejection never synthesizes a move, share, or clone plan. The
programmer must end the loan and perform an explicit operation on the ultimate
root. Canonical failure never retries a SharedV1 route.

## Mutation, liveness, and CFG law

Allowed:

```text
field read/write through root or alias
exact lifecycle-neutral noescape mutating method with no callback/reentry,
  suspension, consume, fini, share, or rehome effect
stable dominating alias used in If/Loop
branch/iteration-local alias that ends locally
```

Forbidden while any alias is live:

```text
root take/move/rebind/destruction
root or alias fini
share/rehome/relocation
task transfer or suspension
```

The alias binding is immutable; heap mutation is not alias reassignment.
Root and aliases are always `may-alias`; no `noalias`, `readonly`, independent
field cache, or mutation reordering may be derived from this capability.

L1 is path-sensitive:

```text
take/rebind/fini/share/destroy at S:
  live alias set on every incoming source-CFG path at S == empty
```

Failure, cleanup, cancellation, and nonlocal-exit edges participate. Builder
does not rediscover or shorten loans after effects begin.

V1 rejects alias reassignment, predecessor-selected aliases, alias creation on
one predecessor followed by a merge use, loop-header alias PHIs, and
loop-carried alias redefinition. A stable borrowed parameter used inside a
loop is allowed because it is not selected or redefined at the header.

## Future LoanPhi boundary (not V1)

C0 rejects every join that would select a non-owning loan. This is a first
profile boundary, not a permanent language prohibition. A reserved future
`VerifiedLoanPhiV2` may own exact predecessor-to-anchor and common-loan-extent
proofs, but no MIR identity. A separate `MaterializedBorrowedPhiV2` may later
select a borrowed pointer with ownership-operation delta zero. An Owned `Phi`
is never a LoanPhi; the first possible future row is read-only statement If,
with every mutable/Loop/projection/escape form kept separate.

## Preliminary corpus evidence

A bounded read-only probe covered 3,294 tracked `.hako`/`.nyash` files and
31,486 initialized locals. Its mixed-parser snapshot found 530 direct
field/index projections (1.68%) and 18,353 MethodCall initializers (about 58%).
A narrower AST-exact heuristic found at most about 660 whole-root syntax
candidates and about 12.7k MethodCall results. The figures use different
filters and are deliberately non-authoritative.

Whole-root candidates still include primitives, shadows, and loop counters.
The provisional corpus also had only two explicit `.fini()` calls and 37
`fini {}` surfaces, but that does not prove F0 soundness. The durable
interpretation is only:

```text
P0 whole-root alias: low-risk but likely narrow
direct projection: smaller than call-result pressure
call-result ABI: dominant unresolved ergonomics boundary
method name: never ownership evidence
```

O2-P0a, O2-P0r, O2-P0b1, and O2-P0c make this evidence durable before O2-D0.
These handwritten numbers disappear as authority once generated ledgers with
fixture provenance, input hashes, parser profiles, and explicit stop reasons
land.

## Dependency DAG

```text
parked evidence branch:
  O2-P0a
    -> {O2-P0r, O2-P0b1}
  {O2-P0r, O2-P0b1}
    -> O2-P0c
    -> O2-D0

active D-prime branch:
  current SSA-I1 consultation
    -> SSA-I0-PROFILE
    -> SSA-I1-T

first unique substrate:
  {SSA-I1-T, O2-D0}
    -> O2-A0
    -> O2-L0
    -> O2-M0
    -> O2-DIAG-S0
    -> O2-MIR-U0
    -> UBOX-P0
    -> UBOX-M0
    -> UBOX-I0

diagnostic monitor branch, never a release-safety dependency:
  {O2-A0, O2-L0, O2-M0}
    -> LOANTRACE-S0
  {LOANTRACE-S0, UBOX-M0}
    -> LOANTRACE-I0

first alias:
  {UBOX-I0, O2-A0, O2-L0, O2-DIAG-S0}
    -> ALIAS-I0
  {ALIAS-I0, UCTRL-I0}
    -> ALIAS-C-IF0
  {ALIAS-I0, UCTRL-L0}
    -> ALIAS-C-LOOP0
  {ALIAS-I0, ABI-B0, UCALL-B0}
    -> ALIAS-CALL0

unique control:
  UBOX-I0 -> UCTRL-B0 -> UCTRL-I0
  {UBOX-I0, D′ Loop-I2′} -> UCTRL-L0

call and zero-RC owner movement:
  {UBOX-I0, ABI-B0} -> UCALL-B0
  {UBOX-I0, ABI-T0, ABI-C0} -> MOVE-CALL0
  {UBOX-I0, ABI-R0, UCLEAN-E0} -> MOVE-RET0
  {UBOX-I0, ABI-F0, UCLEAN-E0} -> MOVE-FIELD0
  closure/capture authority -> MOVE-CLOSURE0
  Send/task publication authority -> MOVE-TASK0
  explicit Outbox ABI -> MOVE-OUTBOX0

post-ABI evidence audit:
  {ABI-R0, selected view/return ABI rows} -> O2-P0b2

future read-only LoanPhi branch, not an Arena blocker:
  {ALIAS-C-IF0, UCTRL-I0}
    -> LOAN-PHI-P0
    -> LOAN-PHI-D0
    -> LOAN-PHI-S0
    -> LOAN-PHI-M0
    -> LOAN-PHI-I0

required unique control/call/move/cleanup rows
  -> ARENA-D0 -> ARENA-S0 -> ARENA-I0

{UBOX-I0, SHARE-D0}
  -> SHARE-LOCAL-S0 -> LOCAL-CELL-S0 -> SHARE-LOCAL-I0

{UBOX-I0, SHARE-D0, SYNC-D0}
  -> SHARE-SYNC-S0 -> SYNC-CELL-S0 -> SHARE-SYNC-I0
  -> THREAD-PUBLISH-I0

SHARE-LOCAL-I0
  -> RESOURCE-D0 -> RESOURCE-UNIQUE-I0 -> RESOURCE-SHARED-I0

{SHARE-LOCAL-I0, WEAK-D0, STABLE-CELL-S0}
  -> WEAK-I0

DYN-P0 -> DYN-D0 -> DYN-I0
ANY-P0 -> ANY-COMPAT-I0

SV1-P0 -> SV1-ABI0 -> SV1-COMPAT-I0
  -> SV1-MIGRATE-n -> SV1-R1 -> SV1-R2
```

Dynamic/Any and SharedV1 compatibility do not block UBOX or Arena. They block
only the applicable ABI/default-route retirement claims.

## Claim-unit task order

### O2-P0a — durable initializer-shape census

Create one reusable artifact and thin guard:

```text
tools/checks/fixtures/ownership_v2_initializer_shape_census_v1.json
tools/checks/lib/ownership_v2_initializer_shape_census.py
tools/checks/ownership_v2_contract.sh
```

Every tracked `.hako`/`.nyash` path records parser profile, source hash,
parse outcome, timeout/reject reason, and every Local declaration exactly
once with its initializer status/category:

```text
whole root / alias-chain candidate
field projection / index projection
new or constructor
literal or trivial
method call / function or static call
BlockExpr or control expression
compound expression
missing initializer
ambiguous or unparsed
```

Call rows record syntactically exact spelling, receiver/callee surface family,
and top call families. They do not assign return ownership. Method names alone
are never ownership authority.

Acceptance:

```text
tracked path coverage = 100%
every Local declaration exactly once
all fallback rows explicit
category totals reconcile
regeneration deterministic
production behavior delta = 0
```

May claim only reproducible syntax/use pressure. It may not claim that a
whole-root spelling is a Box alias, a getter returns a view, or a destination
requires RC.

### O2-P0r — exact root-alias eligibility census

Consume resolved owner, `BindingRef`, representation, dominance, and exact-use
products for every whole-root syntax candidate. Each row records:

```text
exact owner / SourceBindingSiteV1 / BindingRefV1
static representation: Box | Trivial | AnyOrUnknownStop
ultimate candidate root
dominance and source-CFG use disposition
reassignment / take / fini / share sites
Return / store / capture / suspension sites
If / Loop / join disposition
exact known noescape ABI evidence, if any
P0Eligible | exact rejection reason
```

Name, Span, pointer identity, and runtime tag inference are zero. This row may
claim only exact eligible/reject classification for the measured corpus; it
does not activate alias semantics.

Acceptance requires every P0a whole-root candidate exactly once as
`P0Eligible`, an exact rejection, or `UnknownStop`. Parse/resolution failures
remain explicit rows and all totals reconcile with P0a.

### O2-P0b1 — exact final-callee/current-signature census

Consume existing resolver/final-callee and current signature evidence, not raw
call names. Publish one row per initializer call:

```text
owner + exact SourceStmtSiteV1
binding subject
exact final callee
current declared type/signature
current transport shape
explicit ownership evidence present/absent
UnknownStop reason when evidence is absent
use/boundary disposition
```

P0b1 must not invent V2 `BorrowedView | Owned | Shared` classifications. Those
are future ABI decisions. Unknown rows are evidence for O2-D0, not errors to be
filled by name heuristics.

Acceptance:

```text
every P0a call candidate exactly once as resolved evidence or UnknownStop
unknown/final-callee-missing rows explicitly stopped
parse/resolution failures remain explicit
totals reconcile with P0a
name/span/pointer/runtime-tag ownership inference = 0
Rust/Hako independent traversal parity before authority claim
production behavior delta = 0
```

### O2-P0c — ownership-destination census

Classify every measured Return, take-call, owning-field, closure-capture,
task/thread, Outbox, arena, registry, and foreign/unknown destination:

```text
exact source root and source site
destination family
owner-count delta: 0 | +1 | UnknownStop
AliasBoundaryDispositionV2 when the source subject is an alias
OwnerDestinationDispositionV2 when an ultimate-root operation is requested
current ABI evidence
requires independent root/control cell
V2 candidate | exact stop reason
```

The census must distinguish `MoveToOwner`, `ConvertUniqueToShared`, and
`CloneSharedRoot`. It may not turn a generic `escape=true` observation into an
RC policy, nor turn an alias rejection into an owner-token action.
Every destination candidate is accounted exactly once as classified or
`UnknownStop`; parse/resolution failures remain explicit and totals reconcile
with the P0a/P0r/P0b1 input domains.

### O2-D0 — constitution freeze

After O2-P0a, O2-P0r, O2-P0b1, and O2-P0c create:

```text
design/ownership-v2-root-anchored-alias-ssot.md
```

Freeze rvalue ownership, root aliasing, take/share/clone, AliasId roots,
L1/P0-or-P1/C0/F0, parameter/receiver/Return ABI, reassignment, cost contract,
B-prime supersession, SharedV1 sunset, Any/dyn/Outbox exclusions, and the
closed `AliasBoundaryDispositionV2` and `OwnerDestinationDispositionV2`
vocabularies. It freezes ABI law, not fabricated per-callee ownership rows.

If projections/view-return calls are substantial, P1 may enter the edition
roadmap while ALIAS-I0 remains P0. Reference specs stay provisional until this
row closes.

### SSA-I0-PROFILE / SSA-I1-T — active D-prime branch

Keep the current exact-trivial whole-unit decision independent:

```text
whole-unit atomic Binding SSA routing
production Ownership SSA = 0
no route retry or unit-internal mixing
```

### O2-A0 — passive alias identity

Add disconnected `AliasIdV1`, `AliasAnchorV1`,
`RootAnchoredScopedBoxAliasV1`, and exact root flattening/cycle/foreign-root
verification. No ValueId/BasicBlockId/PHI imports, Binding SSA connection, or
production behavior.

### O2-L0 — source-CFG alias flow

Seal creation/use sites, live-in/live-out alias sets, exact consume/rebind/fini/
share permissions, failure/cleanup edges, noescape/suspension rejection, and
C0 join/reassignment rejection. Every boundary use receives one closed
disposition; escape rejection never synthesizes an ownership plan. Product
contains no MIR identity.

### O2-M0 — move/definite-initialization flow

Seal explicit take, use-after-move, branch/loop balance, reinitialization,
owner replacement, and cleanup disposition. It owns availability, never PHIs
or reaching ValueIds.

### O2-DIAG-S0 — typed diagnostics and golden fixes

Land an executable typed diagnostic schema plus human and machine-readable
golden fixtures. This is not a docs-only row.

```text
RootEffectWhileAliasLive
AliasEscapeRejected
JoinRequiresLoanPhi
UnsupportedAliasAnchorProfile
UnknownNoEscapeAbi
UseAfterTake
```

Every error carries an exact owner, primary source site, stable error code, and
remediation category. Variant-specific payload is closed as follows:

| Variant family | Additional payload |
| --- | --- |
| alias flow | AliasId, ultimate root, creation/conflict sites, next-use frontier |
| move/availability | root subject/definition epoch and consume/later-use sites; no AliasId required |
| profile/ABI | candidate site and exact rejection reason; AliasId only if already sealed |

Names and spans are presentation metadata. Diagnostics never suggest hidden
retain/promotion/fallback, and mention `unsafe raw` only in an explicit
unsafe/expert context.

### ABI-B0/T0/R0/F0/C0 and production movement rows

Land exact borrowed parameter/receiver, take parameter, owned Return, owning
field, and call transport one row at a time. Their production consumers remain
separate:

```text
UCALL-B0:
  non-escaping borrowed call

MOVE-CALL0:
  exact take-parameter token transfer

MOVE-RET0:
  exact owned Return token transfer

MOVE-FIELD0:
  exact owning field token transfer
```

Closure, task/thread, Outbox, collections, callback, Any, dyn, plugin, and
unknown FFI remain separately gated. Do not duplicate MOVE-CALL/RET authority
under legacy `UCALL-T0` or `UCALL-R0` names.

### O2-P0b2 — post-ABI call-result coverage audit

After ABI-R0 and the selected view/return ABI rows land, re-run exact
final-callee coverage and classify only sealed evidence as:

```text
BorrowedView | Owned | Shared | Trivial | UnknownStop
```

P0b2 proves corpus coverage/parity for implemented ABI rows. It is not an
O2-D0 prerequisite and never assigns ownership to an unannotated/unknown
callee.

### O2-MIR-U0 / UBOX-P0/M0/I0

Seal a `UniqueConcreteBox { type_id, layout_id, drop_glue_id }` sidecar.

```text
move/Phi/Return = token forwarding
DestroyOwned Unique = structural drop + free
CopyOwned Unique = reject
ObjectCell/RC/generation/tombstone = 0
```

First production family is straight-line, one concrete ordinary Box with
trivial fields, one owner move, exact close, and trivial function result.
It claims exactly-once drop/free only; no alias, control, call, Arena, shared,
resource, weak, dyn, stack-promotion, or general Box claim.

### LOANTRACE-S0 / LOANTRACE-I0 — debug/test observer

`VerifiedScopedAliasFlowV1` remains the sole release lifetime authority.
`DebugLoanTraceV1` is an optional default-OFF observer, never a shadow RC.

S0 lands a disconnected normalized event schema and replay checker:

```text
RootDefinition
AliasBegin / AliasUse / AliasEnd
RootEffectAttempt { TakeOrMove, Rebind, Destroy, Fini, Share, Rehome }
FrameExit
```

The key is `(FunctionOwnerId, root subject, root-definition epoch)`, not an
object pointer, name, Span, or runtime identity. I0 connects one selected
VM/reference backend and proves normalized ON/OFF result parity.

LoanTrace never increments a strong count, retains an object, delays free,
revives payload, observes allocation reclamation, chooses a route, changes
cleanup, or publishes lifetime state. Allocation/drop reclamation, if a future
debug oracle needs it, belongs to a separate observer with its own allocation
identity. Trace success is diagnostic evidence only and cannot replace the
pre-Builder loan verifier or production MIR verification.

### ALIAS-I0 / ALIAS-C-IF0 / ALIAS-C-LOOP0

ALIAS-I0 activates one straight-line exact `OwnedLocalRoot` / ordinary
non-resource `UniqueConcreteBox` alias family: chain flattening, direct
root/alias field read-write, and last-use before take/close. It contains no
call, parameter/receiver root, shared root, resource, callback, or control-flow
claim. Required zeros:

```text
alias Binding SSA definitions
alias ValueIds
alias CopyOwned/DestroyOwned
hidden retain/promotion
noalias attributes from alias
```

IF0 and LOOP0 separately add stable dominating and locally ending aliases.
Alias PHIs and loop-carried alias redefinition remain zero.

`ALIAS-CALL0` separately adds an exact lifecycle-neutral borrowed call. Its ABI
must prove noescape, nocallback/reentry, nosuspend, noconsume, nofini, noshare,
and norehome. A generic `noescape` bit is insufficient.

### Future LoanPhi branch

This branch is evidence-gated and never blocks Arena or C0 production:

```text
LOAN-PHI-P0:
  branch-selected alias corpus/shape matrix

LOAN-PHI-D0:
  read-only statement If, common lexical loan extent

LOAN-PHI-S0:
  disconnected VerifiedLoanPhiV2
  ValueId / BasicBlockId / materialized Phi = 0

LOAN-PHI-M0:
  borrowed-pointer Phi adapter over Binding SSA/MIR

LOAN-PHI-I0:
  one atomic read-only If family, RC/CopyOwned/DestroyOwned = 0
```

Mutable and Loop-carried LoanPhi forms are later independent rows. Ownership
SSA V1 `BorrowedPhiForbidden` remains unchanged until this entire future slice
has its own verifier/backend support.

### P1 projection/view branch

Detailed task owner: [Anchored View Return ABI task](hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md).

Open only if O2-D0 selects it. The branch begins with an explicit Anchored
View Return ABI, WholeObject receiver anchor, and zero runtime ownership work:

```text
PROJ-D0 -> PROJ-S0 -> PROJ-ABI0 -> PROJ-R0 -> PROJ-CALL0 -> PROJ-I0
```

Exact field/container domains, temporary anchors, ViewPhi, dynamic index,
partial move, drop flags, and collection-element lifetime stay later.

### Arena, shared, and lifecycle branches

Arena follows the unique control/call baseline and precedes selfhost graph
migration. Its first profile is one lexical region, non-resource nodes,
trivial/same-arena fields, no individual fini, no escape/cross-arena/weak/
shared/host/dyn, and one bulk close on all exits.

Shared rows separately add `ShareOwnedLocal`, `ShareOwnedSync`, explicit-clone
`CopyOwned`, and `PublishOwnedSync`. SharedLocal uses a non-atomic cell;
unsupported backends reject instead of silently using atomic Arc.

B-prime then supplies paid unique/shared resource, weak/generation, and sync
lanes. DestroyOwned, last strong, and Rust Drop never invoke user fini.

### Dynamic/Any and SharedV1 retirement

Dispatch and ownership remain orthogonal. Neither dyn nor Any is a fallback
for unresolved ownership.

One edition normalizer feeds one resolved ownership product, Binding SSA,
Ownership SSA, MIR, and runtime. V2 aliases never enter the legacy
`CopyBorrowedStrong` planner. Retirement requires exact zero for SharedV1
units, implicit-share plans, legacy planner callers, bridge calls, unstamped
units, and edition retry/fallback.

## Required fixtures and diagnostics

Pass:

```text
ordinary non-resource Unique OwnedLocalRoot accepted in ALIAS-I0
root/alias same identity and sequential mutation
alias chain flattened
last use followed by take
stable and branch/iteration-local If/Loop use
exact ALIAS-CALL0 noescape/no-reentry method emits ownership op 0
MoveToOwner Return/call/field rows emit RC op 0
explicit Shared clone alone emits CopyOwned
source-CFG liveness equations match an independent exhaustive small-CFG oracle
```

Reject:

```text
owner consume/rebind/fini/share while alias live
alias reassignment/take/fini/escape/capture/suspend
unknown call/plugin/FFI/callback
branch-selected or loop-carried alias
field/index alias before P1
Outbox/Upvar/temporary/Any first-profile root
resource/SharedLocal/SharedSync root in ALIAS-I0
escape rejection synthesizing CopyOwned/share/promotion
CopyOwned on Unique
implicit Local-to-Sync promotion
V2 alias through CopyBorrowedStrong
```

Human and machine-readable diagnostics are both golden-tested. The structured
form retains every exact next-use frontier site and stable remediation kind;
the human form may render a deterministic bounded prefix plus remaining count.
Help is context-specific:

```text
live alias:
  narrow/end the alias or move the operation after its last use

single-owner transfer:
  end the alias, then use explicit take

independent lifetime:
  end the alias, then use explicit share/clone

LoanPhi:
  keep the alias branch-local or restructure before the join
```

Safe diagnostics do not routinely recommend `unsafe raw` and never rewrite
source automatically into a paid or unsafe lane.

LoanTrace fixtures cover duplicate begin/end, use-after-end, foreign/root-epoch
mismatch, root effect while active, and frame-exit leak. Integration fixtures
also require identical normalized results and independently observed drop
counts with tracing OFF and ON; those drop counts are not LoanTrace events.

## Guard structure

Use one thin public facade with bounded helpers:

```text
tools/checks/ownership_v2_contract.sh
tools/checks/fixtures/ownership_v2_*_v1.json
tools/checks/lib/ownership_v2_*.py
```

First evidence fixtures are:

```text
ownership_v2_initializer_shape_census_v1.json
ownership_v2_root_alias_eligibility_v1.json
ownership_v2_call_result_current_evidence_v1.json
ownership_v2_destination_census_v1.json
ownership_v2_call_result_abi_coverage_v1.json
```

Future fixtures cover initializer census, SharedV1 compatibility, Dynamic Any,
and backend capabilities. Add the public facade to the checks index only when
its first reusable artifact lands. Do not create one shell guard per row.

Required counters include:

```text
plain MoveToOwner RC operations = 0
alias Binding SSA definitions / ValueIds / ownership opcodes = 0
CopyOwned on Unique = 0
CopyOwned outside explicit Shared clone/bridge authority = 0
unknown escape -> hidden share = 0
LoanTrace-held runtime roots = 0
LoanTrace reclamation decisions = 0
LoanTrace implicit retain/release = 0
debug trace default OFF = 1
trace OFF/ON normalized result parity = exact
unsupported backend silent lowering = 0
new/modified source/check file >= 800 lines = 0
```

## Claim discipline

One row closes one claim. Do not begin the next row before the current
may-claim is green. Refactor Series Mode may split passive structure from one
final activation commit, but BoxShape and BoxCount never mix.

After ALIAS-I0 only this may be claimed:

```text
one exact whole-root Box alias family is RC-free
root and alias may mutate sequentially
source-CFG liveness protects the anchor
alias is neither owner token nor MIR value
first alias activation is ordinary non-resource Unique OwnedLocalRoot only
```

Must not claim:

```text
all local aliases statically classified
field/index/call-result projection loans
borrowed Return or alias PHI
shared/resource/parameter/receiver alias roots
Any/dyn/cross-task ownership closure
all Box code has C-equivalent cost
Arc/SharedV1/B-prime source defaults retired
```

## Stop conditions

Stop a row if it:

```text
changes the current SSA-I1 blocker without authorization
freezes O2-D0 before durable census
classifies V2 return ownership in P0b1 before O2-D0/ABI rows
freezes O2-D0 without exact destination/owner-count evidence
stores ValueId/BasicBlockId/PHI in alias flow
defines ScopedAlias in Binding SSA
gives alias an owner token or DestroyOwned
uses textual last-use instead of path-sensitive source CFG
treats borrowed parameter/receiver as callee-owned
accepts Outbox/Upvar/field/index/temporary/Any in first profile
activates shared/resource/parameter/receiver roots in ALIAS-I0
assumes unknown call noescape
equates escape with shared ownership or RC acquisition
derives noalias/readonly from mutable alias
uses CopyOwned for share or publication
silently promotes Unique/SharedLocal
keeps B-prime implicit-share and V2 source law both canonical
routes V2 alias through legacy CopyBorrowedStrong
creates separate V1/V2 SSA, MIR, or runtime authorities
retries another edition after canonical failure
claims debug sanitizer as release safety
lets LoanTrace retain/delay free, select routes, or own reclamation
relaxes BorrowedPhi V1 or materializes LoanPhi under C0
duplicates UCALL-T0/R0 and MOVE-CALL/RET authority
blocks UBOX-I0 on unused parameter/Return ABI families
pins an O2 guard to the current blocker token
opens two consecutive docs-only rows without a new machine artifact
mixes Arena/shared/resource/weak/sync into first Box activation
```

## Next action

The next O2 action is **O2-P0a only**:

```text
materialize the initializer-shape census
freeze corpus/parser provenance
split calls without assigning ownership by method name
select O2-P0r as the exact BindingRef/root-eligibility follow-up
O2-P0r and O2-P0b1 are independent but land one claim at a time
select O2-P0c only after both, without inventing V2 ABI facts
production behavior delta = 0
```

The active pointer remains on the SSA-I1 consultation. O2-P0a is parked until
explicitly selected; this taskboard changes no grammar, MIR, runtime, or
default route.
