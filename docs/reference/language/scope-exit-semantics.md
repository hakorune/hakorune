# Scope Exit Semantics (SSOT)

Status: Normative (2026-02).

This page defines the scope-exit lifecycle model around canonical `cleanup`,
legacy DropScope `fini`, postfix `catch/cleanup`, and object-level `box.fini()`.

## 0) Scope

This SSOT fixes:

- DropScope registration and execution order
- failure routing (`catch` / `cleanup`)
- relation between scope cleanup and object finalization
- failure policy when cleanup/finalization handlers fail
- constructor (`birth`) partial-failure behavior
- precedence vs `lifecycle.md`

## 1) Core Surfaces

- Standalone `cleanup { ... }`: canonical scope-exit handler spelling for the
  current DropScope. Parser support is phased; until the parser row lands, use
  the legacy alias below in live source.
- `local x = e cleanup { ... }`: canonical declaration sugar that registers
  cleanup at the declaration point. Parser support is phased.
- `fini { ... }`: legacy compatibility alias for a scope-exit cleanup handler.
- `local x = e fini { ... }`: legacy declaration sugar that registers cleanup
  at the declaration point.
- Postfix `cleanup { ... }`: always-run handler attached to a protected
  expression/block/member handler.
- `catch (...) { ... }`: failure handler.
- `box.fini()`: object-level finalization hook when ownership ends. This is not
  a scope-exit handler.

Constraints:

- `local ... cleanup` / `local ... fini` require exactly one local binding.
- `finally` is terminology only; the surface keyword is `cleanup`.
- `throw` is prohibited in surface language design (parser always rejects it).

Naming rule:

```text
cleanup = when lexical/block cleanup runs
fini()  = what an object does when finalized
```

## 2) Unified Cleanup Model

Canonical `cleanup`, legacy DropScope `fini`, and postfix `cleanup` all lower to
the same finally-style execution channel.

- Handlers run once per scope exit.
- Multiple DropScope registrations in the same scope run in LIFO order.
- Cleanup handlers are not jump targets.

## 3) Exit Ordering

On normal exit, `return`, `break`, `continue`, or failure:

1. evaluate the scope body and route failures to nearest `catch` in lexical context
2. run scope-exit handlers (`cleanup`, including legacy `fini` aliases) for the exiting scope
3. drop local bindings of that scope
4. apply `box.fini()` when ownership actually ends
5. propagate unhandled failure outward (fatal at top level)

Lexical nesting determines inner/outer handler order.

## 4) Local Cleanup Binding Rule

`local ... cleanup` and legacy `local ... fini` bind to the declaration-time
slot.

- later shadowing does not retarget an already-registered handler
- same-scope redeclaration is fail-fast, so slot identity stays unambiguous

## 5) Handler Restrictions and Failure Policy

Cleanup handler restrictions (parser/verifier enforced):

- forbidden: `return`, `break`, `continue`, `throw`

If a compatibility path still accepts `break`/`continue` from a cleanup block,
that path is not canonical and must be narrowed by a dedicated verifier row.

If cleanup/finalization itself fails:

1. complete remaining release steps in the current scope (best effort)
2. then fail-fast as fatal runtime error

## 6) `birth` Partial-Failure Rules

If constructor (`birth`) fails:

1. do not call `box.fini()`
2. destroy only already-initialized fields
3. field destruction order is reverse declaration order
4. for `from Parent.birth(...)`, apply the same rule across the full initialized field set

## 7) Ownership Terminology (No `move` Keyword)

Core language has no dedicated `move` keyword.

Use **ownership transfer** only as terminology.

- `outbox` is the explicit transfer surface in user syntax.
- Rust-style moved-state rules are not part of the current surface-language contract.

## 8) SSOT Priority

`scope-exit-semantics.md` is authoritative for:

- DropScope cleanup surfaces (`cleanup`, legacy `fini` aliases, postfix `cleanup`)
- exit ordering
- `catch` routing
- cleanup/finalization failure policy
- ownership-transfer terminology

`lifecycle.md` is authoritative for:

- object states (Alive/Dead/Freed)
- weak-reference semantics
- memory policy (GC/non-GC)

When texts conflict, use this file for scope-exit behavior and transfer terminology.
