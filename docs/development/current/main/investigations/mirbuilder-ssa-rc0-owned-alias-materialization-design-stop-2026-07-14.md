---
Status: Resolved — A′ accepted; implementation remains inactive
Date: 2026-07-14
Decision: A′ — explicit Ownership SSA pair (`CopyOwned` / `DestroyOwned`)
Resolved blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-RC0-OWNED-ALIAS-MATERIALIZATION-DESIGN-STOP-001
Related:
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# SSA-RC0 Owned-Alias Materialization Design Stop

## Why implementation stops here

SSA-M0 proved that one function-owned Binding SSA can materialize real MIR
PHIs. The next row, SSA-RC0, must seal assignment and BlockExpr scope-escape
ownership before that SSA becomes a production authority.

The read-only audit found one missing primitive at that boundary:

```text
x = y

y:
  still-live borrowed BindingRef value

x:
  needs its own owned alias before x's previous value can be released
```

The same operation is required when a BlockExpr returns an outer binding. A
scope-local tail instead transfers the local's existing ownership, while an
already-owned temporary is forwarded. These cases cannot be decided from raw
`ValueId` equality.

The repository does not yet have one backend-independent MIR meaning for that
owned-alias acquire:

```text
VM Copy:
  clones VMValue; BoxRef cloning acquires another Arc owner

Wasm Copy:
  local.get + local.set only

ReleaseStrong:
  exists, but Wasm currently treats it as a no-op

kernel ABI:
  nyrt_handle_retain_h exists, but no general MIR instruction owns its use
```

Therefore `Copy = retain` is not an established cross-backend contract. RC0
cannot safely choose an executable action until this boundary is decided.

## Source authority

```text
VerifiedResolvedFunctionV1:
  exact BindingRef / ScopeId / RegionId / storage classification

BindingSsaBuilderV1:
  current reaching ValueId for a local BindingRef

resolved_lowering/ownership:
  pure assignment and scope-escape ownership transition plan

MIR ownership instruction:
  backend-independent acquire/release execution meaning

backend lowering:
  materializes that fixed MIR meaning; does not rediscover ownership policy
```

## Non-authority

```text
raw ValueId equality
AST pointer / Span / name
alias-root scan over completed MIR
post-MIR optional RC insertion
KeepAlive
JoinIR carrier/ownership rows
legacy value maps
backend-specific Copy behavior
unpublished-draft error cleanup
```

In particular, a second `BindingRef -> ValueId` map or a post-hoc ownership
event ledger must not be introduced. Binding SSA remains the sole reaching
value authority.

## Accepted decision: A′

Add an explicit MIR ownership pair whose names describe MIR meaning rather
than one backend's RC implementation:

```rust
MirInstruction::CopyOwned {
    dst: ValueId,
    src: ValueId,
}

MirInstruction::DestroyOwned {
    value: ValueId,
}
```

Semantic law:

```text
Copy:
  representation / SSA materialization only; no ownership effect

CopyOwned:
  do not consume src
  create a fresh dst for the same language-level value/object identity
  dst is an independent ownership token that may be consumed exactly once
  physical handle equality is not part of the contract

DestroyOwned:
  consume exactly the named Owned value
  never scan or destroy another alias of the same object

ReleaseStrong:
  legacy lifecycle vocabulary only
  forbidden on the canonical ownership route
  retire only after repository-wide exact caller zero
```

`AcquireStrong` is rejected as the final spelling because it resembles memory
ordering, while `RetainStrong` exposes one backend implementation strategy.

## Local verification correction

The consultation correctly identified that the published/reference
`ReleaseStrong` contract was not a suitable canonical Ownership SSA consume
operation, but one implementation observation was stale for this worktree.

Current local Rust MIR interpreter code already takes only the named register;
it no longer pointer-scans and deletes every register containing the same
`Arc`. Current instruction comments and RC insertion helpers also describe
exact named SSA values. Therefore deleting a VM alias sweep is **not** a future
task.

The separate `DestroyOwned` instruction is still required because:

```text
ReleaseStrong is vector-valued legacy lifecycle vocabulary
the tracked MIR reference contains stale alias-group permission that must be
repaired without changing the already exact-slot local VM behavior
Wasm lowers ReleaseStrong as a no-op
there is no None/Borrowed/Owned verifier or consuming-use law
PHI/Return ownership forwarding and edge-argument exclusion are not defined
```

No existing `ReleaseStrong` meaning is changed during migration.

First production representation profile:

```text
StorageClass::BoxRef:
  CopyOwned / DestroyOwned allowed after exact proof

InlineI64 / InlineBool / InlineF64:
  ValueId reuse; ownership instruction count = 0

BorrowedText / Array / Future / WeakRef / Void / Opaque / Unknown:
  typed capability rejection before Builder effects
```

The existing canonical grammar contains untyped parameter/value origins, so
the first production owner profile must not infer `BoxRef` from runtime data.
It either excludes those origins or waits for an independently sealed storage
witness. Disconnected hand-built MIR fixtures may exercise `BoxRef` before a
source producer exists.

## Alternatives

### B — make ordinary Copy an ownership acquire

This is smaller in instruction count, but it changes the meaning of an
existing widely consumed MIR operation. Every optimizer, value-origin query,
VM fast alias, LLVM path, Wasm path, and future backend would need to prove
that copying an owning reference creates an independently releasable owner.

Recommendation: reject unless a repository-wide audit proves that this is
already the intended `Copy` contract and all backends can adopt it atomically.

### C — avoid acquire by scanning aliases or counting BindingRefs

This would make raw `ValueId` aliasing or a second value map an ownership
authority. It also fails for aliases whose runtime handle must be retained.

Recommendation: reject.

### D — park production Binding SSA ownership

Keep SSA-RC0 and SSA-I1 inactive and continue no farther on the production
canonical route. This is safe, but it parks If cutover and therefore Loop.

Recommendation: use only if no backend-independent acquire instruction is
accepted.

## Pure SSA-RC0 contract after the decision

The ownership box belongs at:

```text
src/mir/builder/resolved_lowering/ownership/
  README.md
  value.rs
  assignment.rs
  scope_exit.rs
  error.rs
  tests.rs
```

It remains independent of `MirBuilder` and `MirInstruction`. Its closed input
vocabulary should distinguish provenance rather than infer it from values:

```rust
enum LoweredValueOwnershipV1 {
    Trivial {
        value: ValueId,
    },
    Owned {
        value: OwnedValueIdV1,
    },
    BorrowedStrong {
        binding: LocalBindingSubjectV1,
        value: ValueId,
    },
}

enum LocalBindingClassV1 {
    Receiver,
    Parameter,
    Local,
    Outbox,
}
```

Assignment laws:

```text
x = x:
  exact same BindingRef provenance; CopyOwned 0, DestroyOwned 0

x = owned temporary:
  transfer next ownership, then destroy previous x

x = borrowed y:
  CopyOwned first, then DestroyOwned previous x

x = trivial y:
  reuse y's ValueId; ownership instruction count 0

raw ValueId equality without same BindingRef provenance:
  never self-assignment authority
```

Scope-escape laws:

```text
scope-local tail binding:
  transfer its current owned value; exclude it from scope destroy

outer borrowed tail binding:
  CopyOwned once; close the inner scope normally

owned temporary / already transferred nested tail:
  forward ownership

other closing-scope bindings:
  read their current Binding SSA values and DestroyOwned exactly once

destroy order:
  reverse source declaration order
```

Error law:

```text
successful scope close:
  may materialize the sealed runtime ownership plan

unpublished draft discard:
  restores compiler state and discards code;
  it exposes no API that emits runtime release actions
```

Upvar/cell, field, index, and general place storage are outside this local
ownership vocabulary and fail preflight until their owners are designed.

## Final implementation order

```text
SSA-RC0-D0:
  accept CopyOwned / DestroyOwned and isolate legacy ReleaseStrong

SSA-RC-L0:
  behavior-neutral split of near-800-line backend opcode and MIR JSON seams

SSA-RC-L1:
  closure-scope Rust interpreter frames across every success/error exit

SSA-RC-P0:
  seal the exact value-origin/storage profile; Unknown/Opaque fail preflight

SSA-RC-A0:
  add passive CopyOwned/DestroyOwned vocabulary, conservative WRITE effect,
  printer/JSON/transport/verifier shape,
  production canonical callers 0

SSA-RC-V0:
  close None/Borrowed/Owned kinds, forwarding/consuming-use verification,
  and optimizer preservation; production canonical callers 0

SSA-RC-A1a/A1b/A1c:
  add Rust explicit handlers, then prove Rust forwarding and exact llvm_py +
  nyash_kernel materialization around the shared V0 artifact;
  every other consumer fails preflight; production canonical callers 0

SSA-RC-RET-P0:
  inventory/isolate legacy ReleaseStrong and unverified transform callers

SSA-RC0:
  implement disconnected pure assignment/scope-escape planner and fixtures

SSA-I1:
  atomically cut the whole current canonical owner, including If and
  BlockExpr, to one Binding SSA; ownership activation may remain zero

SSA-I1-O1:
  activate one exact BoxRef source owner only after its producer/ABI witness

SSA-R1:
  retire old canonical If value/effect authority after exact caller zero

Loop-S3' -> Loop-I1' -> Loop-I2':
  seal carrier-free Loop control, disconnected CFG transaction, then one
  closed production Loop family over the same Binding SSA

N1 -> N2 -> N3 -> N4:
  add one nesting shape per slice, then depth-independent evidence

EXIT-S0 -> EXIT-S1 -> EXIT-S2 -> EXIT-I1..I7:
  add typed exit semantics, roles, multi-port contracts, and bounded runtime
  activation one source shape at a time

F0 -> F1a/F1b/F1c/F1d -> F2:
  preflight and atomically cut over each remaining function-owner family

RET-I1/RET-I2 -> RET-R1/RET-R2 -> PUB-F0:
  isolate canonical legacy calls, delete only caller-zero mechanisms, and
  close one final verified publication barrier
```

Loop is not an independent SSA experiment. Existing If must reach SSA-I1
first, and nested If/Loop always share the same per-function Binding SSA.

## Minimum fixtures

```text
assignment:
  owned temporary transfer
  borrowed different-binding acquire before old release
  exact self-assignment no-op
  same raw ValueId but different provenance is not self-assignment

scope exit:
  ordinary local current value released once
  scope-local tail transferred and not released
  outer borrowed tail acquired once
  owned temporary tail forwarded
  same-name shadow does not affect outer binding ownership

error/storage:
  unpublished draft discard emits no runtime cleanup
  Outbox has an explicit local-binding classification
  Upvar/cell/place is rejected by the local planner
```

The existing guarded inventory remains 92 rows with 7 `rc_lifetime` rows.
Do not inflate it for this design stop. When code lands, extend the existing
private ownership helper under `resolved_binding_ssa_contract.sh`; do not grow
the 796-line public authority guard or add another public guard.

## Atomic acquire acceptance

Before SSA-RC0 may become executable, the selected acquire contract must fix:

```text
CopyOwned/DestroyOwned spelling and dst/src result law
trivial/non-owning ValueId reuse behavior
invalid representation fail-fast
VM semantics
LLVM/object semantics
Wasm support or preflight rejection
MIR JSON round-trip and printer spelling
verifier def/use and type/representation checks
Phi/Return ownership forwarding and V1 edge-argument exclusion
optimizer preservation or explicit rejection rules
legacy ReleaseStrong isolation and caller-zero retirement
```

## May claim after this design stop

```text
the final roadmap includes the explicit Ownership SSA prerequisite
the pure RC0 authority and its non-authorities are bounded
production Binding SSA and canonical ownership callers remain zero
```

## Must not claim

```text
CopyOwned/DestroyOwned is implemented
Copy has ownership-acquire semantics
SSA-RC0 is closed
canonical If uses Binding SSA
canonical Loop is active
all backends implement ownership RC
Upvar/capture/place ownership is solved
```

## Stop conditions

Stop implementation or publication if it:

```text
uses VM Copy behavior as the cross-backend ownership contract
releases x before a borrowed RHS obtains its owned alias
treats raw ValueId equality as self-assignment
scans MIR aliases to rediscover source ownership
adds a second BindingRef reaching-value map
lets the ownership planner allocate ValueId or inspect AST/source names
emits runtime cleanup while discarding an unpublished function draft
routes Upvar/cell/field/index/place through local Binding SSA ownership
activates If/Loop before acquire, RC0, CFG, SSA, and publication verification
silently no-ops an unsupported ownership instruction in a production backend
```

## Accepted response

```text
owned-alias operation:
  A′ explicit CopyOwned + DestroyOwned

first representation profile:
  BoxRef only; trivial values reuse; Unknown/Opaque reject

first backend profile:
  Rust MIR interpreter + llvm_py/handle ABI
  Wasm and every unproved consumer reject

landing order:
  D0 -> L0 -> L1 -> P0 -> A0 -> A1a -> V0 -> A1b -> A1c
     -> RET-P0 -> RC0 -> atomic I1 -> exact-BoxRef I1-O1
```
