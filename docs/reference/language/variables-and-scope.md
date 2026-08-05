# Variables and Scope (Local/Block Semantics)

Status: Stable lexical rules; Home ownership projection is staged/provisional.

This document defines the variable model used by Hakorune/Nyash and clarifies how locals interact with blocks, memory, and references across VMs (Rust VM, Hakorune VM, LLVM harness).

For source ownership and aliasing, see
`docs/reference/language/ownership.md`. For lifecycle/finalization, see
`docs/reference/language/lifecycle.md`.

## Local Variables

- Syntax: `local name` / `local name = expr`
- Scope: Block‑scoped. The variable is visible from its declaration to the end of the lexical block.
- Redeclaration: Writing `local name = ...` inside a nested block creates a new shadowing binding. Same-scope redeclaration (`local name` twice in one lexical scope) is a compile-time error. Writing `name = ...` without `local` updates the nearest existing binding in an enclosing scope.
- Mutability: Locals are mutable unless a narrower verified capability says
  otherwise. HomeV1 must separately decide whether reassignment of a
  handle-bearing local is handle rebinding, Home replacement, or rejected;
  runtime value kind may not decide it.
- Lifetime: The variable binding ends at block end (`}`). A non-owning alias may
  end earlier at its last use. Ownership and object finalization are defined
  separately in `ownership.md` and `lifecycle.md`.
- Concurrency: `local` is per routine/task activation and is thread-irrelevant. Concurrency-specific state crosses tasks only through explicit boundaries such as `Future<T>`, `Channel<T>`, `sync box`, `context`, or runtime/internal worker-local substrate (SSOT: `docs/reference/concurrency/boundary-model.md`).

Notes:
- `local` is part of the current surface.
- `local x` is treated as `local x = null` (SSOT: `docs/reference/language/types.md`).

## Assignment Resolution (Enclosing Scope Update)

Assignment to an identifier resolves as follows:

1) If a `local` declaration with the same name exists in the current block, update that binding.
2) Otherwise, search outward through enclosing blocks and update the first found binding.
3) If no binding exists in any enclosing scope, it is an error (undeclared variable). Declare it with `local`.

This matches intuitive block‑scoped semantics (Lua‑like), and differs from Python where inner blocks do not create a new scope (function scope), and assignment would create a local unless `nonlocal`/`global` is used.

## Reference Semantics (Home / Handle / Weak)

The accepted Home direction distinguishes lexical binding, Home slot/token,
and non-owning handle.

- An owning rvalue such as `new Box()` creates one Home token and installs it
  in the receiving Home slot.
- `local b = a`, when `a` is an eligible whole-root binding, creates a
  non-owning mutable handle. It does not add or transfer a Home and performs
  no owner bookkeeping.
- The Home binding and handle may both read and mutate the same Box
  sequentially.
- A Home transfer/rebind/release cannot leave a dependent handle with a later
  reachable use. `release(root)` explicitly consumes one verified whole-root
  Home and invalidates its dependent handles without silently re-rooting them.
  C′ fixes that only terminal Home release may run the non-callable Box
  `fini {}` hook; exact Shared representation and the proof/rejection of
  remaining live handles are still Home D0 work.
- A destination with a sealed Home demand transfers one available Home;
  ordinary use remains a handle.
- Independent lifetime enters the Shared lane through explicit `share`.
- `weak x` creates a generation-aware non-owner governed by `lifecycle.md`; it
  is not an ordinary handle or Shared owner.

This ownership behavior is staged and does not claim that every current
SharedV1 production route has already changed.

Example (nested block retains object via outer local):

```
local a = new Box()
{
  local b = a
  b.touch()
  a.inspect()
}
// b was a non-owning handle. Its scope/last-use adds no owner drop.
// a retains the Home.
```

## Shadowing vs. Updating

- Shadowing: `local x = ...` inside a block hides an outer `x` for the remainder of the inner block. The outer `x` remains unchanged.
- Updating: `x = ...` without `local` updates the nearest enclosing `x` binding.

Prefer clarity: avoid accidental shadowing. If you intentionally shadow, consider naming or comments to clarify intent.

## Const/Immutability (Future)

- A separate keyword (e.g., `const`) may introduce an immutable local. Its
  ownership capability remains determined independently by `ownership.md`.

## Cross‑VM Consistency

The lexical scope, shadowing, and enclosing-assignment rules are enforced
consistently across:
- Rust VM (MIR interpreter): scope updates propagate to enclosing locals.
- Hakorune VM/runner: same resolution rules.
- LLVM harness/EXE: parity tests validate identical exit codes/behavior.

The Home/handle projection remains staged with production activation 0;
this paragraph does not claim backend parity for it.

See also: quick/integration smokes `scope_assign_vm.sh`, `vm_llvm_scope_assign.sh`.
