# Box Lifecycle and Finalization (SSOT)

Status: SSOT (language-level), with implementation status notes.

Design note:
- For normative exit-order, DropScope (`fini {}` / `local ... fini {}`), `catch/cleanup` routing, and ownership-transfer terminology, see `docs/reference/language/scope-exit-semantics.md` (SSOT).
- This file remains authoritative for object states (Alive/Dead/Freed), weak refs, and memory policy.

This document defines the Nyash object lifecycle model: lexical scope, ownership (strong/weak), finalization (`fini()`), and what is (and is not) guaranteed across backends.

## Terms

- **Binding**: a local variable slot (created by `local`) that points to a value.
- **Box value**: an object reference (user-defined / builtin / plugin).
- **Strong reference**: an owning reference that contributes to keeping the object alive.
- **Weak reference**: a non-owning reference; it does not keep the object alive and may become dead.
- **Finalization (`fini`)**: a logical end-of-life hook. It is not “physical deallocation”.

## 0) Two-layer model (resource vs memory)

Nyash separates two concerns:

- **Resource lifecycle (deterministic)**: `fini()` defines *logical* end-of-life and must be safe and explicit.
- **Heap memory reclamation (non-deterministic)**: physical memory is reclaimed by the runtime implementation (typically reference counting). Timing is not part of the language semantics.

This split lets Nyash keep “箱理論” simple:
- Programs must use `fini()` (or sugar that guarantees it) to deterministically release external resources (fd/socket/native handles).
- Programs must not rely on GC timing for correctness.

## 1) Scope model (locals)

- `local` is block-scoped: the binding exists from its declaration to the end of the lexical block (`{ ... }`).
- Leaving a block drops its bindings immediately (including inner `{}` blocks).
- Dropping a binding reduces strong ownership held by that binding. It may or may not physically deallocate the object (depends on other strong references).

This is the “variable lifetime” rule. Object lifetime is defined below.

## 2) Object lifetime (strong / weak)

### Strong ownership

- A strong reference keeps the object alive.
- When the last strong reference to an object disappears, the object becomes eligible for physical destruction by the runtime.
  - In typical implementations this is immediate (reference-counted drop) for acyclic graphs, but the language does not require immediacy.

### Weak references

Weak references exist to avoid cycles and to represent back-pointers safely.

Language-level guidance:
- Locals and return values are typically strong.
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

### Fail-fast after `fini`

After an object is finalized, operations must fail fast (use-after-fini).
Permitted exceptions (optional, per type) are strictly observational operations such as identity / debug string.

### Object states (Alive / Dead / Freed)

Nyash distinguishes:

- **Alive**: normal state; methods/fields are usable.
- **Dead**: finalized by `fini()`; object identity may still exist but is not usable.
- **Freed**: physically destroyed by the runtime (implementation detail).

State transitions (conceptual):

- `Alive --fini()--> Dead --(runtime)--> Freed`
- `Alive --(runtime)--> Freed`

SSOT rule:
- `fini()` is the only operation that creates the **Dead** state.
- Runtime reclamation does not imply `fini()` was executed.

### Dead: allowed vs forbidden operations

Allowed on **Dead** (minimal set):
- Debug/observation: `toString`, `typeName`, `id` (if provided)
- Identity checks: `==` (identity only), and identity-based hashing if the type supports hashing

Forbidden on **Dead** (Fail-Fast, UseAfterFini):
- Field read/write
- Method calls
- ByRef (`RefGet/RefSet`) operations
- Conversions / truthiness (`if dead_box { ... }` is an error)
- Creating new weak references from a dead object (`weak dead` is an error)
  - Note: the surface form is `weak <expr>` (not `weak(<expr>)`).

### Finalization precedence

When finalization is triggered (by explicit call or by an owning context; see below):
1) If the object is already finalized, do nothing (idempotent).
2) Run user-defined `fini()` if present.
3) Run automatic cascade finalization for remaining **strong-owned fields** (weak fields are skipped).
4) Clear fields / invalidate internal state.

### Weak references are non-owning

Weak references are values (`WeakRef`) that can be stored in locals or fields:
- They are **not** part of ownership.
- Automatic cascade finalization must not follow weak references.
- Calling `fini()` “through” a weak reference is invalid (non-owning references cannot decide the target’s lifetime).

## 4) Ownership and “escaping” out of a scope

Nyash distinguishes “dropping a binding” from “finalizing an object”.

Finalization is tied to **ownership**, not merely being in scope.

### Owning contexts

An object is considered owned by one of these contexts:
- A local binding (typical case).
- A strong-owned field of another object.
- A module/global registry entry (e.g., `env.modules`).
- A runtime host handle / singleton registry (typical for plugins).

### Escapes (ownership transfer)

If a value is transferred into a longer-lived owning context before the current scope ends, then the current scope must not finalize it.

Common escape paths:
- Assigning into an enclosing-scope binding (updates the owner).
- Returning via `outbox` (ownership transfers to the caller).
- Storing into a strong-owned field of an object that outlives the scope.
- Publishing into global/module registries.

This rule is what keeps “scope finalization” from breaking shared references.

## 4.1) What is guaranteed to run automatically

Language guarantee (deterministic):
- Only **explicit scope-exit constructs** guarantee cleanup execution for all exits (return/break/continue/error).
- Supported scope-exit surfaces are:
  - `fini { ... }` (DropScope registration)
  - `local x ... fini { ... }` (single-binding sugar)
  - postfix `cleanup { ... }` (finally surface)

Recommended SSOT surface:
- Prefer `fini` / `local ... fini` for lexical-scope resource cleanup.
- Use postfix `cleanup` when pairing with `catch` or wrapping block/member handlers.
- Keep object-level `fini()` separate from scope handlers; do not double-release the same resource.

Non-guarantees:
- “Leaving a block” does not by itself guarantee `fini()` execution for an object, because aliasing/escaping is allowed.
- GC must not call `fini()` as part of meaning.

### `fini` / `local ... fini` — DropScope cleanup

```nyash
{
  local f = open(path) fini {
    f.close()
  }
  do_work(f)
}
```

SSOT semantics:
- `fini` runs exactly once on every exit path from the attached scope.
- Multiple `fini` handlers in the same scope run in LIFO order.
- `local ... fini` is declaration sugar and must target exactly one local binding.
- `fini` executes before that scope's locals are dropped.

### `cleanup` (block-postfix) — finally surface

```nyash
{
  local f = open(path)
  do_work(f)
} catch (e) {
  log(e)
} cleanup {
  f.close()
}
```

SSOT semantics:
- The `cleanup` block runs exactly once on every exit path from the attached block.
- `cleanup` may appear with or without `catch`.
- With `catch`, `cleanup` runs after the matching `catch` body in the same wrapper.

## 4.2) Weak references (surface model)

Weak references exist to avoid strong cycles and to model back-pointers.

SSOT operations:
- `weak <expr>` produces a `WeakRef` to the target (the target must be Alive).
  - **Syntax**: `weak <expr>` (unary operator, Phase 285W-Syntax-0)
  - **Invalid**: `weak(expr)` ❌ (compile error: "Use 'weak expr', not 'weak(expr)'")
- `weakRef.weak_to_strong()` returns the target box if it is usable, otherwise `null` (none).
  - It returns `null` if the target is **Dead** (finalized) or **Freed** (collected).
  - Note: `null` and `void` are equivalent at runtime (SSOT: `docs/reference/language/types.md`).

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
- `WeakRef` carries a stable target token (conceptually: `WeakToken`).
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
- If a strong cycle exists, memory reclamation is not guaranteed (it may leak). This is allowed behavior in “no cycle collector” mode.

Important: weak references themselves do not require tracing GC.
- They require a runtime liveness mechanism (e.g., an `Rc/Weak`-style control block) so that “weak_to_strong” can succeed/fail safely.

### GC modes

GC is treated as an optimization/diagnostics facility, not as a semantic requirement. In practice, this means “cycle collection / tracing”, not “basic refcount drop”.

- **GC off**: reference-counted reclamation still applies for non-cyclic ownership graphs; strong cycles may leak.
- **GC on**: the runtime may additionally reclaim unreachable cycles eventually; timing is not guaranteed.

Invariant:
- Whether GC is on or off must not change *program meaning*, except for observability related to resource/memory timing (which must not be relied upon for correctness).

### Operational profiles (non-normative)

The runtime may provide two operating profiles while keeping the same language semantics:
- **Beginner mode**: cycle collector enabled (diagnostics/safety oriented).
- **Expert mode**: cycle collector disabled (design relies on weak references to avoid cycles).

Both profiles must preserve the same program meaning; only reclamation timing and leak tolerance differ.

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

- **Block-scoped locals** are the language model (`local` drops at `}`), but the *observable* effects depend on where the last strong reference is held.
- **WeakRef** (Phase 285A0+): VM backend fully supports `weak <expr>` and `weak_to_strong()`. LLVM harness also supports this surface as of Phase 285LLVM-1.4.
- **WASM backend** currently treats MIR `WeakNew/WeakLoad` as plain copies (weak behaves like strong). This does not satisfy the SSOT weak semantics yet (see also: `docs/guides/wasm-guide/planning/unsupported_features.md`).
- **Leak Report** (Phase 285): `NYASH_LEAK_LOG={1|2}` prints exit-time diagnostics showing global roots still held (modules, host_handles, plugin_boxes). See `docs/reference/environment-variables.md`.
- Conformance gaps (any backend differences from this document) must be treated as bugs and tracked explicitly; do not "paper over" differences by changing this SSOT without a decision.

See also:
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

## 10) RC responsibility split and retirement policy (normative)

This section fixes the ownership/lifecycle contract boundary to prevent drift across MIR/VM/LLVM.

### Role split (SSOT)

- MIR does not own or maintain numeric reference counts.
- MIR expresses lifecycle intent only, through instructions such as `keepalive` and `release_strong`.
- Backends lower lifecycle intent to runtime ABI calls.
- Runtime/Kernel is the only layer that performs retain/release count transitions and final drop.

Normative implications:

- Adding "refcount arithmetic" logic in MIR passes is out of contract.
- LLVM lowering must not invent count policy; it must call runtime ABI for lifecycle operations.
- VM interpreter lifecycle handling must be contract-equivalent to runtime ABI semantics.

### Current behavior contract (2026-02-13 snapshot)

- `release_strong` is a valid MIR lifecycle op used for overwrite/cleanup timing.
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
- Application authors should not count references manually; they should design ownership boundaries (strong/weak/explicit drop) only.
