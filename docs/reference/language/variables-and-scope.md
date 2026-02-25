# Variables and Scope (Local/Block Semantics)

Status: Stable (Stage‑3 surface for `local`), default strong references.

This document defines the variable model used by Hakorune/Nyash and clarifies how locals interact with blocks, memory, and references across VMs (Rust VM, Hakorune VM, LLVM harness).

For the lifecycle/finalization SSOT, see: `docs/reference/language/lifecycle.md`.

## Local Variables

- Syntax: `local name` / `local name = expr`
- Scope: Block‑scoped. The variable is visible from its declaration to the end of the lexical block.
- Redeclaration: Writing `local name = ...` inside a nested block creates a new shadowing binding. Same-scope redeclaration (`local name` twice in one lexical scope) is a compile-time error. Writing `name = ...` without `local` updates the nearest existing binding in an enclosing scope.
- Mutability: Locals are mutable unless future keywords specify otherwise (e.g., `const`).
- Lifetime: The variable binding is dropped at block end (`}`); object lifetime/finalization is defined separately in `docs/reference/language/lifecycle.md`.
- Concurrency: `local` is per routine/task activation and is thread-irrelevant. Concurrency-specific state lives in `lock<T>` / `scoped` / `worker_local` (SSOT: `docs/reference/concurrency/lock_scoped_worker_local.md`).

Notes:
- Stage‑3 gate: Parsing `local` requires Stage‑3 to be enabled (`NYASH_PARSER_STAGE3=1` or equivalent runner profile).
- `local x` is treated as `local x = null` (SSOT: `docs/reference/language/types.md`).

## Assignment Resolution (Enclosing Scope Update)

Assignment to an identifier resolves as follows:

1) If a `local` declaration with the same name exists in the current block, update that binding.
2) Otherwise, search outward through enclosing blocks and update the first found binding.
3) If no binding exists in any enclosing scope, it is an error (undeclared variable). Declare it with `local`.

This matches intuitive block‑scoped semantics (Lua‑like), and differs from Python where inner blocks do not create a new scope (function scope), and assignment would create a local unless `nonlocal`/`global` is used.

## Reference Semantics (Strong/Weak)

- Default: Locals hold strong references to boxes/collections.
- Weak references: Use `weak(x)` (and fields that store `WeakRef`) to hold a non‑owning reference. Weak refs do not keep the object alive; they can be upgraded at use sites (see SSOT: `docs/reference/language/lifecycle.md`).
- Typical guidance:
  - Locals and return values: strong references.
  - Object fields that create cycles (child→parent): weak references.

Example (nested block retains object via outer local):

```
local a = null
{
  local b = new Box(a)
  a = b  // outer binding updated; a and b point to the same object
}
// leaving the block drops `b` (strong‑count ‑1), but `a` still keeps the object alive
```

## Shadowing vs. Updating

- Shadowing: `local x = ...` inside a block hides an outer `x` for the remainder of the inner block. The outer `x` remains unchanged.
- Updating: `x = ...` without `local` updates the nearest enclosing `x` binding.

Prefer clarity: avoid accidental shadowing. If you intentionally shadow, consider naming or comments to clarify intent.

## Const/Immutability (Future)

- A separate keyword (e.g., `const`) can introduce an immutable local. Semantics: same scoping as `local`, but re‑assignment is a compile error. This does not affect reference ownership (still strong by default).

## Cross‑VM Consistency

The above semantics are enforced consistently across:
- Rust VM (MIR interpreter): scope updates propagate to enclosing locals.
- Hakorune VM/runner: same resolution rules.
- LLVM harness/EXE: parity tests validate identical exit codes/behavior.

See also: quick/integration smokes `scope_assign_vm.sh`, `vm_llvm_scope_assign.sh`.
