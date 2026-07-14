---
Status: Active design stop
Date: 2026-07-14
Decision: Pending — owned-alias acquire materialization
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-RC0-OWNED-ALIAS-MATERIALIZATION-DESIGN-STOP-001
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

## Recommended decision: A

Add an explicit MIR owned-alias acquire instruction. Working vocabulary:

```rust
MirInstruction::AcquireStrong {
    dst: ValueId,
    src: ValueId,
}
```

The final name may instead be `RetainStrong`, but the instruction should have
a destination. The runtime handle ABI may return a distinct handle for the
same object, so an in-place hint-only retain is not a sufficient general
contract.

Recommended semantic law:

```text
reference-like src:
  create one independently releasable owned alias in dst

immediate/non-owning src:
  copy the value into dst without an RC effect

invalid/unsupported representation:
  typed fail-fast before publication
```

This keeps ordinary `Copy` a representation/value-copy operation and keeps
ownership visible in MIR, its printer/JSON/verifier, and every backend.

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
    Owned(ValueId),
    BorrowedBinding {
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
  exact same BindingRef provenance; acquire 0, release 0

x = owned temporary:
  transfer next ownership, then release previous x

x = borrowed y:
  acquire owned alias first, then release previous x

raw ValueId equality without same BindingRef provenance:
  never self-assignment authority
```

Scope-escape laws:

```text
scope-local tail binding:
  transfer its current owned value; exclude it from scope release

outer borrowed tail binding:
  acquire one owned alias; close the inner scope normally

owned temporary / already transferred nested tail:
  forward ownership

other closing-scope bindings:
  read their current Binding SSA values and release exactly once
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
  decide explicit acquire versus Copy semantics

SSA-RC-A0:
  add passive MIR acquire vocabulary, printer/JSON/verifier contract,
  production canonical callers 0

SSA-RC-A1:
  prove VM + supported production backend materialization and explicit
  unsupported-backend fail-fast; production canonical callers 0

SSA-RC0:
  implement disconnected pure assignment/scope-escape planner and fixtures

SSA-I1:
  atomically cut the whole current canonical owner, including If and
  BlockExpr, to one Binding SSA plus the sealed ownership actions

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
instruction name and dst/src result law
immediate/non-owning value behavior
invalid representation fail-fast
VM semantics
LLVM/object semantics
Wasm support or preflight rejection
MIR JSON round-trip and printer spelling
verifier def/use and type/representation checks
optimizer preservation or explicit rejection rules
ReleaseStrong pairing and rollback behavior
```

## May claim after this design stop

```text
the final roadmap includes the missing owned-alias materialization prerequisite
the pure RC0 authority and its non-authorities are bounded
production Binding SSA and canonical acquire callers remain zero
```

## Must not claim

```text
AcquireStrong/RetainStrong is accepted or implemented
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

## Consultation response requested

Please return one decision in this form:

```text
1. owned-alias operation:
   A explicit MIR AcquireStrong/RetainStrong
   or B ordinary Copy contract
   or D park

2. instruction spelling and result:
   dst/src law, including whether a distinct runtime handle is allowed

3. non-owning values:
   typed copy/no-op semantics or reject

4. backend boundary:
   which backends implement the first slice and which fail preflight

5. landing order:
   SSA-RC-A0 -> SSA-RC-A1 -> SSA-RC0 -> atomic SSA-I1
   accepted or corrected
```

Preliminary recommendation is A: make ownership acquisition explicit in MIR,
then keep RC0 as a pure source-ownership transition planner and Binding SSA as
the only reaching-value authority.
