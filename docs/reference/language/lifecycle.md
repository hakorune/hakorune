# Box Lifecycle and Finalization (SSOT)

Status: SSOT (language-level), with implementation status notes.

Decision: B′ eager-fini tombstone semantics accepted on 2026-07-14; sparse
source ownership boundary accepted on 2026-07-15.

Implementation status: transitional. Current `InstanceBox`, global
finalization tracking, plugin Drop-fini routes, and generation-0 host/weak
tables do not yet implement the full decision. Unsupported B′ profiles must
remain unclaimed/fail-fast until their taskboard rows close.

Design note:
- Source-level owner forwarding, scoped aliases, anchored views, and explicit
  Shared entry are owned by `docs/reference/language/ownership.md` (SSOT).
- For normative exit-order, canonical `cleanup`, Compat2025 DropScope
  aliases (`fini {}` / `local ... fini {}`), postfix protected-region/cleanup
  routing, and
  ownership-transfer terminology, see
  `docs/reference/language/scope-exit-semantics.md` (SSOT).
- This file remains authoritative for object states (Alive/Dead/Freed), weak refs, and memory policy.
- Runtime/Ownership SSA implementation order is owned by
  `docs/development/current/main/design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md`.

This document defines the Hakorune/Nyash object lifecycle model: logical
finalization (`fini()`), strong/weak object residency, and what is (and is not)
guaranteed across backends. It does not decide whether a source binding is an
owner, scoped alias, or anchored view.

Construction SSOT:
- Source-level construction order and the `birth` direct-call policy are fixed
  in `docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md`.
- Short rule: `birth` is a constructor hook. It is fired only by `new`.
  Direct receiver calls such as `obj.birth(...)` are forbidden.

## Terms

- **Binding**: a local variable slot (created by `local`) that points to a value.
- **Box value**: an object reference (user-defined / builtin / plugin).
- **Strong owner token**: an independently consumable owner that contributes
  to keeping the object alive.
- **Scoped alias / anchored view**: a non-owning source capability governed by
  `ownership.md`; it does not add a strong token.
- **Weak reference**: a non-owning reference; it does not keep the object alive and may become dead.
- **Finalization (`fini`)**: a logical end-of-life hook. It is not “physical deallocation”.
- **Structural drop**: runtime payload/field-token teardown needed for memory
  safety. It is not user `fini()`.

## Construction lifecycle (`new` / field initializers / `birth`)

Canonical construction surface:

```nyash
local obj = new SomeBox(arg0, arg1)
```

Explicit construction-site field initializers are also canonical:

```nyash
local report = new Report {
    accepted: fields.accepted
    reason: fields.reason
}
```

This form is meant to make initialization a single checked boundary, not to
reduce line count. Duplicate fields and unknown fields on a known user-defined
box fail-fast. For repeated report construction, keep call sites small with a
same-owner helper such as `makeReport(fields)` and use this initializer block
inside that helper when it improves clarity.

Construction order:

```text
allocate object identity
run declaration-site field initializers
run matching birth(args...)
run construction-site field initializer block assignments
publish the object as usable
```

Rules:

- `new Box(args...)` is the canonical source surface for constructing a box.
- `new Box { field: expr }` and `new Box(args...) { field: expr }` are
  explicit field-initializer sugar for ordinary post-construction field
  assignments.
- `birth(...)` is a constructor hook, not an ordinary public method.
- Direct receiver `birth(...)` calls such as `obj.birth(...)` are forbidden.
- Stored field initializers are per-instance values and run before `birth`.
- Reuse/reset/reactivation must use explicit ordinary methods such as
  `reset`, `reactivate`, `configure`, `clear`, or `attach`.
- `birth` must not be reused as a reset or reactivation surface.
- Named constructor arguments are reserved for a later row; current canonical
  construction uses positional arguments.
- Shorthand field copy and wildcard copy are reserved for later rows; current
  construction-site field initializers must use `field: expr`.
- `with` copy-update is record-only. Ordinary boxes are identity/resource
  boundaries, so they must not be implicitly shallow-copied by `with`.

Example:

```nyash
local page = new HakoAllocPageModel(PageId(0), Bytes(32), 2, 2)
local report = new Report { accepted: 1, reason: 0 }
page.reactivate()
page.resetForReuse(Bytes(64), 4)
```

Non-canonical and rejected:

```nyash
page.birth(PageId(0), Bytes(32), 2, 2)
```

Rationale:

```text
Allowing direct birth calls would let user code reinitialize an existing object
identity. That makes lifecycle state ambiguous and weakens verifier reasoning.
```

## 0) Two-layer model (resource vs memory)

Nyash separates two concerns:

- **Resource lifecycle (deterministic)**: `fini()` defines *logical* end-of-life and must be safe and explicit.
- **Heap memory reclamation (non-deterministic)**: physical memory is reclaimed by the runtime implementation (typically reference counting). Timing is not part of the language semantics.

This split lets Nyash keep “箱理論” simple:
- Programs must use `fini()` (or sugar that guarantees it) to deterministically release external resources (fd/socket/native handles).
- Programs must not rely on GC timing for correctness.

### No manual physical free on normal Box

The normal source-level Box API does not expose raw `free`, physical
`reclaim`, or the runtime's selected RC strategy.

```text
deterministic resource shutdown:
  explicit fini() / cleanup

end one ownership lifetime:
  scope exit, transfer, or verified DestroyOwned materialization

physical backing-allocation/control-cell memory reclamation:
  runtime/backend strategy; not a source timing guarantee
```

`fini()` does guarantee logical payload teardown and publication of payload
absence. It does not guarantee when the backing allocation or control-cell
memory is returned to an allocator.

An optimizer may prove a Box statically unique and materialize its terminal
ownership consume as immediate structural drop/reclamation. That does not add
a source `unique` type or a second reclaim operation. A future exclusive
source capability or raw-memory API requires a separate language Decision and
pointer/provenance model; it is outside the normal Box lifecycle contract.

## 1) Scope model (locals)

- `local` is block-scoped: the binding exists from its declaration to the end of the lexical block (`{ ... }`).
- Leaving a block ends its bindings immediately (including inner `{}` blocks).
- Ending an owning binding consumes/forwards its token according to the sealed
  exit plan. Ending a scoped alias/view adds no ownership destroy.
- Consuming the last owner may or may not immediately return backing memory;
  that depends on the verified representation and remaining strong/weak roots.

This is the “variable lifetime” rule. Object lifetime is defined below.

## 2) Object lifetime (strong / weak)

### Strong ownership

- A strong reference keeps the object alive.
- When the last strong reference to an object disappears, the object becomes eligible for physical destruction by the runtime.
  - In typical implementations this is immediate (reference-counted drop) for acyclic graphs, but the language does not require immediacy.
- Last-strong structural drop never calls user-defined `fini()`.
- Shared-lane assignment, parameter/result transport, and Shared owning fields
  may preserve Box identity while carrying independent owner tokens. Ordinary
  local aliases/parameters are not independent strong owners.
- An existing owner forwarded into an owning destination uses `move source`.
  If the source must remain usable with an independent lifetime, the boundary
  uses `share source`. Ordinary assignment does not silently add an owner.
- One object identity may have multiple independently consumable strong
  ownership tokens. Destroying each distinct token once is legal; consuming
  the same token twice is a verifier/checked-carrier error, not an idempotent
  object operation.

### Weak references

Weak references exist to avoid cycles and to represent back-pointers safely.

Language-level guidance:
- Ordinary results and owning destinations carry/forward owners.
- Ordinary local aliases and parameters do not add owners.
- Independent lifetime enters the Shared lane through `share`.
- Back-pointers / caches / parent links that would create cycles should be weak.

Required property:
- A weak reference never keeps the object alive.

Observable operations (surface-level; exact API depends on the box type):
- “Is alive?” check.
- Weak-to-strong conversion (may fail): `weak_to_strong()`.

## 3) Finalization (`fini`) — what it means

`fini()` is a **logical** termination hook:
- After `fini()` has executed successfully for an object, the object must be treated as unusable (use-after-fini is an error).
- `fini()` must be **idempotent** (calling it multiple times is allowed and must not double-free resources).
  - This supports “external force fini” and best-effort cleanup paths safely.
  - A later call after Dead is a no-op. Recursive `fini()` from the winning
    finalizer transaction is a fail-fast reentrancy error; it is not a second
    completed call. Concurrent callers do not execute a second hook and wait
    for the terminal transaction result.
- Calling `fini()` does not consume the caller's strong ownership token.
  Teardown may still destroy ownership tokens stored in the object's fields.
- Source/runtime `fini()` must enter the object lifecycle transaction. It must
  not dispatch the user hook directly.

### Fail-fast after `fini`

After an object is finalized, operations must fail fast (use-after-fini).
Permitted exceptions (optional, per type) are strictly observational operations such as identity / debug string.

### Object states (Alive / Finalizing / Dead / Freed)

Nyash distinguishes:

- **Alive**: normal state; methods/fields are usable.
- **Finalizing**: runtime-internal transaction state. New ordinary payload
  access is rejected while the winning finalizer drains existing access.
- **Dead**: finalized by `fini()`; object identity may still exist but is not usable.
- **Freed**: strong count is zero and structural payload reclamation is
  complete. A generation-bearing weak tombstone/control cell may remain until
  the last weak token disappears.

State transitions (conceptual):

- `Alive --fini()--> Finalizing --> Dead --(last strong)--> Freed`
- `Alive --(runtime)--> Freed`

SSOT rule:
- `fini()` is the only operation that creates the **Dead** state.
- Runtime reclamation does not imply `fini()` was executed.
- `Dead` with a remaining strong token is not `Freed`, even though its payload
  is already absent.
- The control cell is reclaimable only when strong and weak counts are both
  zero; generation wrap permanently retires that slot.

### Dead: allowed vs forbidden operations

Allowed on **Dead** (minimal set):
- Debug/observation from immutable tombstone metadata: `typeName`, `id`, and a
  runtime-provided debug representation (if provided)
- Identity checks: `==` (identity only), and identity-based hashing if the type supports hashing
- Strong identity aliasing/forwarding is allowed; it does not resurrect the
  payload. A completed Dead alias can still be destroyed normally.

Forbidden on **Dead** (Fail-Fast, UseAfterFini):
- Field read/write
- Method calls
- ByRef (`RefGet/RefSet`) operations
- Conversions / truthiness (`if dead_box { ... }` is an error)
- Creating new weak references from a dead object (`weak dead` is an error)
  - Note: the surface form is `weak <expr>` (not `weak(<expr>)`).

### Finalization precedence

When explicit object finalization is requested:
1) If the object is already finalized, do nothing (idempotent).
2) Atomically enter Finalizing and reject new ordinary payload access.
3) Drain existing ordinary access; the winner receives a privileged,
   non-escapable finalizer self-access capability.
4) Run user-defined `fini()` once if present.
5) Release and clear stored owning/Shared field tokens in reverse declaration
   order. Do **not** implicitly call child user `fini()`.
6) Destroy stored weak tokens without upgrading, traversing, or finalizing
   their targets.
7) Tear down native payload/storage, publish payload absence, then publish Dead.

A parent that semantically owns a child resource calls `child.fini()`
explicitly inside its user hook. Any future exclusive-field surface requires a
separate language Decision after exclusivity and transfer can be enforced; no
`owned field` spelling is reserved here.

### Weak references are non-owning

Weak references are values (`WeakRef`) that can be stored in locals or fields:
- They are **not** part of ownership.
- Object finalization must not follow or upgrade weak references. The weak
  token itself still has a copy/drop discipline so its control-cell weak count
  can be reclaimed safely.
- Calling `fini()` “through” a weak reference is invalid (non-owning references cannot decide the target’s lifetime).

## 4) Ownership and “escaping” out of a scope

Nyash distinguishes “dropping a binding” from “finalizing an object”.

Ownership tokens keep identity/storage alive. Object finalization is an
explicit object-wide transition, not something inferred from scope end or last
ownership. Calling it does not consume the caller token. Source Loan Flow
forbids `fini()` while a scoped alias/view remains live; independent Shared
owners observe the same Dead identity after finalization.

### Owning contexts

An object may have a strong owner/root token in any of these contexts:
- A local binding (typical case).
- An owning field or a Shared field of another object.
- A module/global registry entry (e.g., `env.modules`).
- A runtime host handle / singleton registry (typical for plugins).

### Escapes (ownership transfer)

If one owner is forwarded into a longer-lived owning context before the
current scope ends, `move source` forwards that token and keeps the identity
alive without requiring RC. If the source must remain usable under an
independent lifetime, `share source` explicitly enters/acquires the Shared
lane. Ordinary assignment or escape does not infer either operation. Neither
case grants implicit authority to call user `fini()`.

Common escape paths:
- Assigning into an enclosing-scope binding (updates the owner).
- Returning one owner to the caller (historical `outbox` is a compatibility
  surface).
- Forwarding into an owning field, or storing a Shared owner into a Shared
  field.
- Publishing into global/module registries.

This rule is what keeps “scope finalization” from breaking shared references.

## 4.1) What is guaranteed to run automatically

Language guarantee (deterministic):
- Only **explicit scope-exit constructs** guarantee cleanup execution for all exits (return/break/continue/error).
- Supported scope-exit surfaces are:
  - `cleanup { ... }` (canonical DropScope registration; parser rollout is phased)
  - `local x ... cleanup { ... }` (canonical single-binding sugar; parser rollout is phased)
- `fini { ... }` (Compat2025 legacy DropScope registration alias)
- `local x ... fini { ... }` (Compat2025 legacy single-binding sugar)
  - postfix `cleanup { ... }` (finally surface)

Recommended SSOT surface:
- Prefer `cleanup` terminology for lexical/block resource cleanup in new docs and examples.
- Treat DropScope `fini { ... }` / `local ... fini { ... }` as compatibility aliases.
- Keep object-level `fini()` separate from scope handlers; do not double-release the same resource.

Non-guarantees:
- “Leaving a block” does not by itself guarantee `fini()` execution for an object, because aliasing/escaping is allowed.
- GC must not call `fini()` as part of meaning.
- `DestroyOwned`, last-strong reclamation, and native `Drop` must not call user
  `fini()` as part of meaning.

### `cleanup` / legacy `fini` — DropScope cleanup

```nyash
{
  local f = open(path) cleanup {
    f.fini()
  }
  do_work(f)
}
```

SSOT semantics:
- `cleanup` runs exactly once on every exit path from the attached scope.
- Multiple cleanup handlers in the same scope run in LIFO order.
- `local ... cleanup` is declaration sugar and must target exactly one local binding.
- Cleanup handlers execute before that scope's locals are dropped.
- `fini` spelling is a bounded compatibility alias; new Canonical source uses
  `cleanup` and the alias does not create an independent semantic owner.

### `cleanup` (block-postfix) — pending cleanup surface

```nyash
{
  local f = open(path)
  do_work(f)
} cleanup {
  f.close()
}
```

SSOT semantics:
- The `cleanup` block runs exactly once on every exit path from the attached block.
- `cleanup` may attach to an independently selected protected region.
- A future postfix catch handles only `RecoverableFailure`; terminal Fault
  bypasses catch and still drains cleanup. Handler ordering and producer/ABI
  are pending `LANGUAGE-RECOVERABLE-FAILURE-D0`.

## 4.2) Weak references (surface model)

Weak references exist to avoid strong cycles and to model back-pointers.

SSOT operations:
- `weak <expr>` produces a `WeakRef` to the target (the target must be Alive).
  - **Syntax**: `weak <expr>` (unary operator, Phase 285W-Syntax-0)
  - **Invalid**: `weak(expr)` ❌ (compile error: "Use 'weak expr', not 'weak(expr)'")
- `weakRef.weak_to_strong()` returns the target box if it is usable, otherwise `null` (none).
  - It returns `null` if the target is **Dead** (finalized) or **Freed** (collected).
  - Note: `null` and `void` are equivalent at runtime (SSOT: `docs/reference/language/types.md`).

Upgrade is one linearizable runtime operation: it validates slot generation,
requires Alive with a positive strong count, and acquires the new strong token
before reclamation can win. Separate unguarded “check then increment” steps are
not conforming. Finalizing, Dead, weak-only, stale, and reclaimed targets fail.

WeakRef values also have an exact copy/drop discipline for the control-cell
weak count. A backend must not implement a counted WeakRef as an ordinary
bit-copy followed by multiple drops. The current first Ownership SSA profile
rejects WeakRef until its co-sealed weak-token representation is activated.

WeakRef in fields:
- Reading a field that stores a `WeakRef` yields a `WeakRef`. It does not auto-upgrade.

Recommended usage pattern:
```nyash
local x = w.weak_to_strong()
if x != null {
  ...
}
```

WeakRef equality:
- `WeakRef` carries a stable generation-aware target token (conceptually:
  `BoxIdentity(slot, generation)`).
- `w1 == w2` compares tokens. This is independent of Alive/Dead/Freed.
  - "dead==dead" is true only when both weakrefs point to the same original target token.

### Weak Field Assignment Contract (Phase 285A1)

Weak fields enforce strict type requirements at compile time:

**Allowed assignments** (3 cases):
1. **Explicit weak reference**: `me.parent = weak p`
2. **WeakRef variable**: `me.parent = other.parent` (where `other.parent` is weak field)
3. **Void**: `me.parent = Void` (clear operation; null is sugar for Void)

**Forbidden assignments** (Fail-Fast compile error):
- Direct BoxRef: `me.parent = p` where `p` is BoxRef
- Primitives: `me.parent = 42`
- Any non-WeakRef type without explicit `weak` conversion

**Error message example**:
```
Cannot assign Box (NodeBox) to weak field 'Tree.parent'.
Use `weak <expr>` to create weak reference: me.parent = weak value
```

**Rationale**: Explicit `weak` conversions make the semantic difference between strong and weak references visible. This prevents:
- Accidental strong references in weak fields (reference cycles)
- Confusion about object lifetime and ownership
- Silent bugs from automatic conversions

**Example**:
```nyash
box Node {
    weak parent

    set_parent(p) {
        // ❌ me.parent = p           // Compile error
        // ✅ me.parent = weak p      // Explicit weak conversion
        // ✅ me.parent = Void        // Clear operation (SSOT: Void primary)
    }

    copy_parent(other: Node) {
        // ✅ me.parent = other.parent  // WeakRef → WeakRef
    }
}
```

**Legacy syntax** (still supported, Phase 285A1.2):
- `init { weak parent }` — old syntax; superseded by direct `weak parent` declaration
- Both syntaxes behave identically and populate the same weak_fields set
- New code should use `weak field_name` directly for clarity

**Visibility blocks** (Phase 285A1.3):
- `weak` is allowed inside visibility blocks: `public { weak parent }`

**Sugar syntax** (Phase 285A1.4):
- `public weak parent` is equivalent to `public { weak parent }`
- `private weak parent` is equivalent to `private { weak parent }`

## 5) Cycles and GC (language-level policy)

### Cycles

Nyash allows object graphs; strong cycles can exist unless the program avoids them.

Policy:
- Programs should use **weak** references for back-pointers / parent links to avoid strong cycles.
- If a strong cycle exists, memory reclamation is not guaranteed (it may leak).
  This is allowed behavior. Current implementations must be treated as
  no-cycle-collector for language reasoning.

Important: weak references themselves do not require tracing GC.
- They require a runtime liveness mechanism (e.g., an `Rc/Weak`-style control block) so that “weak_to_strong” can succeed/fail safely.

### GC modes

GC is treated as an optimization/diagnostics facility, not as a semantic requirement. In practice, this means optional tracing/diagnostics and possible future cycle collection, not “basic refcount drop”.

- **GC off**: reference-counted reclamation still applies for non-cyclic ownership graphs; strong cycles may leak.
- **GC on / diagnostic mode**: the runtime may add safepoint/barrier/allocation
  diagnostics and reachability trials. Current runtime modes do not guarantee
  cycle detection or cycle reclamation.

Invariant:
- Whether GC is on or off must not change *program meaning*, except for observability related to resource/memory timing (which must not be relied upon for correctness).

### Operational profiles (non-normative)

The runtime may provide two operating profiles while keeping the same language semantics:
- **Beginner mode**: diagnostics enabled (currently `rc+cycle`, an external
  compatibility label; not a current cycle-collection guarantee).
- **Expert mode**: diagnostics/hooks disabled (design relies on weak references
  to avoid cycles).

Both profiles must preserve the same program meaning. Current implementations
may differ only in diagnostics/observability hooks; future collectors may also
differ in reclamation timing and leak tolerance.

## 6) ByRef (`RefGet/RefSet`) — borrowed slot references (non-owning)

Nyash has an internal “ByRef” concept (MIR `RefGet/RefSet`) used to access and mutate fields through a **borrowed reference to a storage slot**.

Intended use cases:
- Field get/set lowering with visibility checks (public/private) and delegation (from/override).
- Passing a “mutable reference” to runtime helpers or plugin calls without copying large values.

SSOT constraints:
- ByRef is **non-owning**: it does not keep the target alive and does not affect strong/weak counts.
- ByRef is **non-escaping**: it must not be stored in fields/arrays/maps, returned, captured by closures, or placed into global registries.
- ByRef is **scope-bound**: it is only valid within the dynamic extent where it was produced (typically a single statement or call lowering).
- Using ByRef on **Dead/Freed** targets is an error (UseAfterFini / dangling ByRef).

These constraints keep “箱理論” simple: ownership is strong/weak; ByRef is a temporary access mechanism only.

## 7) Diagnostics (non-normative)

Runtimes may provide diagnostics to help validate lifecycle rules (example: reporting remaining strong roots or non-finalized objects at process exit). These diagnostics are not part of language semantics and must be default-off.

## 8) Implementation status (non-normative)

This section documents current backend reality so we can detect drift as bugs.

### Feature Matrix (Phase 285A0 update)

| Feature | VM | LLVM | WASM |
|---------|-----|------|------|
| WeakRef (`weak <expr>`, `weak_to_strong()`) | ✅ | ✅ LLVM harness (Phase 285LLVM-1.4) | ❌ unsupported |
| Leak Report (`NYASH_LEAK_LOG`) | ✅ | ⚠️ Parent process roots only (285LLVM-0) | ❌ |

**LLVM Leak Report の制限** (Phase 285LLVM-0):
- LLVM harness runnerで親プロセス（Rust VM側）のroot snapshotを報告
- 報告内容: modules, host_handles, plugin_boxes
- 子プロセス（native executable）内部の到達可能性は見えない（プロセス境界の制約）
- これは設計上の制約であり、バグではない

### Notes

- **Block-scoped locals** are the language model (`local` ends at `}`), but
  only owning bindings carry a token to destroy. Scoped aliases/views do not.
- **WeakRef** (Phase 285A0+): VM backend fully supports `weak <expr>` and `weak_to_strong()`. LLVM harness also supports this surface as of Phase 285LLVM-1.4.
- **WASM backend** currently treats MIR `WeakNew/WeakLoad` as plain copies (weak behaves like strong). This does not satisfy the SSOT weak semantics yet (see also: `docs/guides/wasm-guide/planning/unsupported_features.md`).
- **Leak Report** (Phase 285): `NYASH_LEAK_LOG={1|2}` prints exit-time diagnostics showing global roots still held (modules, host_handles, plugin_boxes). See `docs/reference/environment-variables.md`.
- Conformance gaps (any backend differences from this document) must be treated as bugs and tracked explicitly; do not "paper over" differences by changing this SSOT without a decision.

See also:
- `docs/reference/language/ownership.md` (owner/alias/View/Shared source contract)
- `docs/reference/language/variables-and-scope.md` (binding scoping and assignment resolution)
- `docs/reference/boxes-system/memory-finalization.md` (design notes; must not contradict this SSOT)

## 9) Validation recipes (non-normative)

WeakRef behavior (weak_to_strong must fail safely):
```nyash
box SomeBox { }
static box Main {
  main() {
    local x = new SomeBox()
    local w = weak x
    x = null
    local y = w.weak_to_strong()
    if y == null { print("ok: dropped") }
  }
}
```

Cycle avoidance (use weak for back-pointers):
```nyash
box Node { next_weak }
static box Main {
  main() {
    local a = new Node()
    local b = new Node()
    a.next_weak = weak b
    b.next_weak = weak a
    return 0
  }
}
```

## 10) Ownership materialization responsibility (normative)

This section fixes the ownership/lifecycle contract boundary to prevent drift across MIR/VM/LLVM.

### Role split (SSOT)

- MIR does not own or maintain numeric reference counts.
- Ownership-managed MIR expresses exact token operations through
  `CopyOwned`/`DestroyOwned`; ordinary `Copy` is ownership-neutral.
- Backends lower lifecycle intent to runtime ABI calls.
- Runtime/Kernel is the only layer that performs retain/release count transitions and final drop.

Normative implications:

- Adding "refcount arithmetic" logic in MIR passes is out of contract.
- LLVM lowering must not invent count policy; it must call runtime ABI for lifecycle operations.
- VM interpreter lifecycle handling must be contract-equivalent to runtime ABI semantics.

### Transitional behavior contract

- `ReleaseStrong`/`release_strong` is a legacy alias-group lifecycle operation,
  not the canonical per-owner counterpart of `CopyOwned`.
- Canonical ownership uses singular `DestroyOwned` for one exact owner token.
- `keepalive` is analysis/liveness intent and may be a no-op at execution backends.
- Legacy symbol `ny_release_strong` is compatibility-only; preferred ABI naming is `nyrt_handle_release_h`.

### RC retirement direction and timing

Direction:
- Long-term direction is reducing hard dependence on RC-specific surface behavior.
- This does not mean immediate removal in current selfhost/bootstrap phases.

Retirement gate (all required):

- VM and LLVM are parity-stable under the same lifecycle semantics for representative fixtures.
- Fast gate and milestone regression suites stay green without RC-only assumptions.
- Weak/strong cycle behavior and explicit drop timing are pinned by fixtures and docs.
- Decision is promoted from provisional to accepted in `20-Decisions.md` with rollback notes.

Until all gates pass:
- RC-backed lifecycle remains the production contract.
- Application authors should not count references manually. They express only
  the source boundary (`share`, weak, cleanup/fini); compiler/runtime products
  own physical token/count materialization.
