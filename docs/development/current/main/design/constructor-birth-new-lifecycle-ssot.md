---
Status: SSOT
Date: 2026-09-05
Scope: source-level object construction lifecycle: `new`, `birth`, field initializers, explicit reuse methods, factories, and `fini`.
Related:
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/design/box-member-field-method-surface-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/design/mimalloc-object-lifecycle-queue-ssot.md
  - docs/development/current/main/design/ownership-home-model-ssot.md
  - docs/development/current/main/investigations/hakorune-home-ownership-task-2026-08-04.md
---

# Constructor Birth / New Lifecycle SSOT

Decision: accepted.

This document owns construction ordering and the direct-`birth` ban. The Home
document owns Home tokens and destinations. The bounded failed-construction
decision below supplies `OWN-HOME-BIRTH-D0` without changing successful order;
source/exit products and runtime adoption remain unimplemented.

Hakorune keeps construction small and explicit:

```text
birth:
  constructor hook
  direct receiver `birth(...)` call forbidden
  fires only through new

new:
  canonical construction surface
  positional args now
  explicit per-construction field overrides now
  named args later

reuse:
  explicit lifecycle methods such as reset / reactivate / configure / clear / attach

field initializer:
  per-instance
  runs before birth

fini:
  object usable-lifetime exit / cleanup
```

## Canonical construction

Canonical source:

```hako
local page = new HakoAllocPageModel(PageId(0), Bytes(32), 2, 2)
```

The construction order is:

```text
allocate object identity
run declaration-site field initializers
run matching birth(args...)
publish the object as usable
```

`birth` is special because it initializes a fresh identity. It is not hidden
magic: it is a declared hook with normal parameters and body rules. The
special rule is only its call permission.

## Direct `birth` calls are forbidden

Forbidden source:

```hako
page.birth(PageId(0), Bytes(32), 2, 2)
```

Reason:

```text
Direct birth calls would let user code reinitialize an existing object identity.
That makes lifecycle state ambiguous and weakens verifier / allocator reasoning.
```

Parser diagnostics should point users at the canonical surface:

```text
direct receiver `birth(...)` calls are forbidden; `birth` is a constructor hook fired only by `new`; use `new HakoAllocPageModel(...)` for construction
```

Existing internal or legacy `birth` routes are compatibility residue unless a
specific row marks them as part of the canonical language. They must not be used
as permission to add source-level `page.birth(...)`.

## Field initializers

Stored field initializers are per-construction values.

```hako
box Counter {
    count: i64 = 0

    birth(start: i64) {
        me.count = start
    }
}
```

For each `new Counter(...)`, field initializers run before `birth(...)`.

Rules:

```text
field initializers:
  create the initial per-instance state
  do not replace birth parameters
  must not be shared mutable state between instances

birth:
  may override initialized fields
  owns fresh-object initialization only
  is not a reuse/reset surface
```

## New-box field initializer block

Decision: accepted for explicit field entries.

This is not a line-count reduction feature. It is a boundary/contract feature:

```text
value:
  group report construction into one initialization boundary
  make duplicate fields fail-fast
  make unknown user-defined box fields fail-fast
  make record-local carrier -> ordinary box crossing visible
  keep runtime/backend semantics unchanged

non-goal:
  reduce source line count by itself
```

The canonical object field-copy surface is:

```hako
local result = new Report {
    accepted: fields.accepted
    reason: fields.reason
}

return result
```

For successful execution, the assignment order resembles:

```hako
local result = new Report()
result.accepted = fields.accepted
result.reason = fields.reason
return result
```

This is not a failure-preserving rewrite: the `new` expression does not publish
its result before overrides succeed. An override Fault still cleans an
incomplete construction, without the outer `fini` hook.

Rules:

```text
new Box { field: expr }:
  constructs an ordinary box identity
  then assigns the listed fields in source order
  does not create a record value
  does not call a named-argument constructor
  does not open reflection or backend-specific lowering

duplicate field:
  fail-fast

unknown field on a user-defined box:
  fail-fast

unmentioned fields:
  keep declaration-site defaults / birth behavior
```

Stop lines:

```text
no wildcard copy (`fields.*`)
no shorthand copy (`fields.accepted`) until BOX-INIT-002
no named constructor arguments
no constructor overload
no record materialization
no backend route or `.inc` owner-name matcher
```

Ordering with constructor lifecycle:

```text
allocate object identity
run declaration-site field initializers
run matching birth(args...)
run new-box field initializer block assignments
publish the object as usable expression result
```

This keeps declaration-site defaults / `birth` as the constructor lifecycle
owner, while `new Box { field: expr }` remains an explicit post-construction
field override at the construction site.

For call-site line-count reduction, prefer the existing RecordFields helper
pattern:

```hako
local fields = ReportFields {
    accepted: accepted,
    reason: reason
}

return me.makeReport(fields)
```

`makeReport(fields)` may use `new Report { field: fields.field }` internally,
but the primary source-size win comes from centralizing repeated copy logic in
one same-owner helper. The initializer block exists to make that helper body
more contract-like, not to replace helper scalarization.

## Failed construction: minimal integration contract

Decision: accepted design direction on 2026-09-05 after user consultation;
implementation permission remains with CURRENT_STATE and the selected slice.
This is the existing `OWN-HOME-BIRTH-D0` contract, not a new task family.

Source authority + canonical issuer: resolved source sites, binding/place
identities and declaration-owned Home demands feed the existing Home/exit
design owners (their construction issuer is not yet implemented). The ordinary-new owner co-seals that exit
relation with its existing exact constructor key; it does not invent cleanup.
Non-authority: Unit Completion, an E0 empty list, i64 storage, runtime handles,
reference counts, `DestroyOwned` alone, Pair's name, or a successful EXE.

Keep three responsibilities, with no Birth-specific general cleanup stack:

| Owner | Responsibility |
| --- | --- |
| common source exit / Home Flow | per-cutpoint local/temporary/lexical obligations, pending Fault, caller continuation |
| construction lifecycle | unpublished receiver, initialized owning fields and committed native payload, success publication |
| physical backend/runtime | consume those decisions; release storage without moving other handles; final reporting only after cleanup |

Each Home token/native owned resource has one cleanup responsibility. Borrowed
handles have none; `share` creates another Home. Native acquisition owns its
resource immediately, including allocation failure while wrapping/registering
it; only a successful destination commit transfers responsibility. This is
not a new source registration API or a second runtime registry. Unknown native
release contracts must not be admitted as compiler-managed construction.

| Cutpoint | Owned state and failure action |
| --- | --- |
| argument preparation / allocation not successful | preserve uncommitted caller Homes; clean acquired temporaries; no nonexistent outer release |
| acquisition succeeded, destination commit pending | acquiring frame/operation retains responsibility, including wrapping failure |
| initializer / Birth / override executing | track only successful field commits; clean frame obligations, initialized fields in reverse declaration order, native payload, then outer storage |
| replacement committed, old release Faults | do not roll back the install; new field remains owned and is cleaned once; old release is not retried |
| normal publication | transfer the first Home once to the result destination; disarm incomplete-construction cleanup |
| Fault propagation | common caller exit handles its surviving obligations; outermost entry reports after cleanup; no Normal join |

The source/exit verifier also owns constructing-receiver non-escape: reject
storage, return, capture or opaque forwarding of `me` before publication unless
an exact existing non-escape contract proves the use. A physical reclaim API
cannot establish this property. Include alias-mediated escapes in the same check.

Use static cleanup edges and, where control flow requires them, initialization
flags keyed to resolved places. No runtime AST/name scan, heap cleanup list,
new Call carrier, fake empty-obligation proof, or per-instruction receipt chain.
The first Fault remains primary; later cleanup Faults are suppressed and
remaining cleanup is attempted best effort. This is resource cleanup, not
rollback of I/O. The incomplete outer `fini` never runs; complete child release
uses the existing last-Home rule. Missing plans reject before artifact, while
admitted runtime contract failures follow the cleanup path, not bare trap.
Host OOM abort/process kill is not a cleanup-complete language Fault witness.

### Ordered tasks and finish line

1. **Common exit connection:** extend existing `resolved_control_flow` / Home
   plan ownership and `ordinary_new_coseal` with exact New Fault cutpoints and
   outward continuation. Return/ImplicitVoid and declaration-only Home ABI are
   not this proof. Complete local/native ownership and field destination
   obligations through existing HOME/EXIT tasks; do not require new syntax,
   Result `?`, Shared or all-backend implementation to express this dependency.
   First close source-plan issuance/consumption with focused tests while the
   existing backend rejection stays intact; this is not runtime completion.
   Then fix the common Normal/Fault physical return contract before tasks 2–3
   consume it. Do not demand executed propagation before choosing that ABI.
2. **Construction cleanup connection:** `ordinary_new_admission` and
   `new_expression` consume the same plan through allocation, Birth and
   overrides. The selected typed-object store gains stable-identity reclaim;
   no double release or early publication. Raw/native wrappers retain ownership
   until handoff; existing raw alloc/free exports alone do not satisfy this.
   Name the admitted storage profiles and their reclaim consumers explicitly;
   do not silently drop the default profile to make a proof pass. Unsupported
   profiles reject before artifact. Bind runtime store selection to the admitted
   capability before allocation; a later env choice cannot bypass the guarantee.
3. **Birth consumer cutover:** return to input-wire/published-C steps 2–4 in
   `workstreams/type-contract-status.md`. Normal Unit and pending Fault have
   distinct internal control transport; never expose a status as a source value
   or use Dynamic's TextScan CallOut. Consume task 1's fixed physical return
   layout with the common exit consumer, not a second Birth failure owner.
4. **Execution and retirement:** existing test owners prove acquisition-to-
   commit failures, second-store mismatch, child failure, override failure,
   replacement cleanup Fault, and a prior live caller object. Assert parent
   hook zero, correct child terminal release, stable unrelated handles,
   primary Fault, no leak/double cleanup, no failed-construction result. Include
   fixed Pair EXE30/OBJ and finite selected old-edge removal, not only unit tests.

Task 1 first connects Birth-body non-escape verification directly to
`issue_instance_constructor_semantic_batch_v1`, before semantic row publication.
The exact receiver BindingRef seeds a conservative alias fixed point over sealed
local initializer/plain-rebind relations. Every receiver/alias occurrence must
be an admitted local alias edge or exact field receiver; capture, forwarding,
stored values and unclassified uses reject as non-escape unproven. Reassignment
never clears the alias set without a reaching-definition proof. Body relations
are not a complete child graph; missing relations cannot prove safety.
Acceptance uses existing package tests for own-field/direct-alias positives and direct/alias
store, capture, forwarding and branch-rebind negatives. No new receipt is issued.
This prerequisite does not prove initializer/override non-escape or cleanup.
Checkpoint evidence: package 77/77, including the fixed Pair publication and
pre-artifact rejection. Direct nested `me` is rejected by the earlier resolver;
alias capture reaches the new verifier. New receipt/guard/fixture = 0; the
unchecked Birth-row admission is closed, not a legacy-file deletion claim.
Actual Fault cutpoints require new issuance in the existing common exit/Home
owner: Allocation event order is source traversal, not runtime evaluation order.
Normal Completion/E0 remain insufficient; do not reuse TextScan's successorless
Fault terminal as caller propagation. No input-ABI reconsultation is needed.
Missing Home field/native products are explicit dependencies, not assumed empty. The
full Home program and general unsafe raw ownership stay parked; selected
construction obligations cannot be waived or reduced to a Pair-only success.
No new guard/fixture/card is planned. Split source owners before 800 lines;
do not create a general framework merely to save a few cleanup edges.

## Reuse is explicit

Object reuse must use ordinary, named lifecycle methods.

```hako
page.reactivate()
page.resetForReuse(Bytes(64), 4)
page.configure(policy)
page.clear()
page.attach(owner)
```

These methods are normal public methods. They should express lifecycle rules
with contracts and transitions when available:

```text
requires:
  pre-state and input validity

ensures:
  post-state and observer facts

transition:
  allowed state movement
```

Do not reuse `birth` for reset/reactivation. This keeps construction,
reconfiguration, and cleanup separate.

### Current allocator reuse inventory

Current `hako_alloc` reuse surface is explicit ordinary method surface:

| Method surface | Owner file | Role |
| --- | --- | --- |
| `HakoAllocPageModel.reactivate()` | `lang/src/hako_alloc/memory/page_box.hako` | Move an empty, committed page back to active reusable state. |
| `HakoAllocPageModel.reuse()` | `lang/src/hako_alloc/memory/page_box.hako` | Guarded wrapper over `canReuse()` and `reactivate()`. |
| `HakoAllocObjectLifecycle*Result.reset()` | `lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako` | Clear result capsule observer state before a new facade operation. |
| `HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage(...)` | `lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako` | Attach a newly sourced page to the object-lifecycle facade. |

These methods are normal public methods. Future `configure`, `clear`, or
`attach*` methods are allowed only as explicit lifecycle methods with their own
contracts / transitions or row guard. They must not be implemented as direct
receiver `birth(...)` calls.

Compatibility exception:

```text
lang/src/hako_alloc/memory/arc_box.hako: arc.birth(ptr)
```

This remains a legacy non-constructor host facade exception. It is not
permission to add source-level receiver `birth(...)` lifecycle reuse.

## Factories

Named construction variants belong in factory methods or factory boxes, not in
extra constructor keywords.

Example shape:

```hako
box HakoAllocPageFactory {
    makeSmall(page_id: PageId): HakoAllocPageModel {
        return new HakoAllocPageModel(page_id, Bytes(32), 2, 2)
    }
}
```

Factories may choose constructor arguments and policies. They do not weaken the
`birth` direct-call ban.

## Named arguments are later

This is readable but not part of the current MVP:

```hako
local page = new HakoAllocPageModel(
    page_id: PageId(0),
    block_size: Bytes(32),
    capacity: 2,
    reserved: 2
)
```

Named constructor arguments require a separate row because they affect parser
surface, diagnostics, argument binding, and metadata transport.

Current MVP:

```text
new Box(positional_args...)
new Box { field: expr }
new Box(positional_args...) { field: expr }
```

Later row:

```text
new Box(named_args...)
```

## `fini` boundary

`fini {}` is the optional non-callable terminal Home hook selected by C′. Like
`birth`, it is compiler-owned rather than an ordinary receiver call; unlike
`birth`, it runs only for a fully constructed object whose last Home is being
released. It is not a direct physical-free API.

Relationship:

```text
new -> field initializers -> birth -> usable methods
-> terminal Home release -> fini hook -> reverse field release -> structural drop
```

Direct `obj.fini()` and `fini()` method declarations are rejected targets.
Failed unpublished outer construction never runs the outer hook; already
complete child Homes are released and may finalize only when terminal, as
defined by `docs/reference/language/lifecycle.md`.

## Stage ownership

```text
Stage0:
  parse birth declarations
  parse new expressions
  reject or diagnose direct source birth calls
  transport constructor metadata
  no lifecycle checker

Stage1:
  constructor resolution
  field initializer ordering facts
  verifier-visible lifecycle facts
  explicit reuse method contracts
  direct-birth negative diagnostics

LLVM/EXE:
  primary acceptance for object-heavy allocator routes

VM:
  semantic reference / scalar smoke only
```

## Task rows

The active task placement is the phase-293x mimalloc taskboard.

Immediate rows:

```text
LIFECYCLE-BIRTH-001:
  document and enforce new-only birth policy

PARSER-BIRTH-001:
  add negative source fixture for page.birth(...)

PARSER-BIRTH-002:
  improve parser diagnostic with new Box(...) hint

NEW-NAMED-ARGS-001:
  parked; design named constructor args later

REUSE-LIFECYCLE-001:
  keep allocator reuse as explicit methods with contracts/transitions
```

Stop line:

```text
Do not accept source-level receiver.birth(...) as a quick fix for constructor
or lifecycle routing failures.
```
