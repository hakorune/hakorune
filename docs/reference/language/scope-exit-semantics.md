# Scope Exit Semantics (SSOT)

Status: Normative (2026-02).

This page defines the scope-exit lifecycle model around DropScope `fini`, `local ... fini`,
postfix `catch/cleanup`, and object-level `box.fini()`.

## 0) Scope

This SSOT fixes:

- DropScope registration and execution order
- failure routing (`catch` / `cleanup`)
- relation between scope cleanup and object finalization
- failure policy when cleanup/finalization handlers fail
- constructor (`birth`) partial-failure behavior
- precedence vs `lifecycle.md`

## 1) Core Surfaces

- `fini { ... }`: registers a scope-exit handler on the current DropScope.
- `local x = e fini { ... }`: declaration sugar that registers `fini` at the declaration point.
- `cleanup { ... }`: postfix always-run handler (finally surface) for `try`/block/member handlers.
- `catch (...) { ... }`: failure handler.
- `box.fini()`: object-level finalization hook when ownership ends.

Constraints:

- `local ... fini` requires exactly one local binding.
- `finally` is terminology only; the surface keyword is `cleanup`.
- `throw` is prohibited in surface language design (parser always rejects it).

## 2) Unified Cleanup Model

`fini` and `cleanup` are both scope-exit handlers and lower to the same finally-style execution
channel.

- Handlers run once per scope exit.
- Multiple `fini` registrations in the same DropScope run in LIFO order.
- `fini`/`cleanup` are not jump targets.

## 3) Exit Ordering

On normal exit, `return`, `break`, `continue`, or failure:

1. evaluate the scope body and route failures to nearest `catch` in lexical context
2. run scope-exit handlers (`fini` / `cleanup`) for the exiting scope
3. drop local bindings of that scope
4. apply `box.fini()` when ownership actually ends
5. propagate unhandled failure outward (fatal at top level)

Lexical nesting determines inner/outer handler order.

## 4) `local ... fini` Binding Rule

`local ... fini` binds to the declaration-time slot.

- later shadowing does not retarget an already-registered handler
- same-scope redeclaration is fail-fast, so slot identity stays unambiguous

## 5) Handler Restrictions and Failure Policy

`fini` block restrictions (parser-enforced):

- forbidden: `return`, `break`, `continue`, `throw`

`cleanup` policy:

- keep non-local exits out of cleanup; `return`/`throw` are rejected by default

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

- DropScope cleanup surfaces (`fini` / `local ... fini` / `cleanup`)
- exit ordering
- `catch` routing
- cleanup/finalization failure policy
- ownership-transfer terminology

`lifecycle.md` is authoritative for:

- object states (Alive/Dead/Freed)
- weak-reference semantics
- memory policy (GC/non-GC)

When texts conflict, use this file for scope-exit behavior and transfer terminology.
