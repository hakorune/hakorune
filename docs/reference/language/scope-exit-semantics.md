# Scope Exit Semantics (SSOT)

Status: Normative C′ target; grammar/runtime/Home production activation 0.

Decision: `LANGUAGE-RESULT-EXIT-C-PRIME0-D0` and
`OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0` accepted on 2026-08-05. Canonical v1 has
one standalone `cleanup {}` registration, typed Result-only postfix `?`, no
source catch/`RecoverableFailure`, and one terminal Home lifecycle hook.
Existing parser/registry/bridge behavior remains migration evidence until the
implementation and mandatory reference-closeout rows land.

This page defines the relation between lexical `cleanup`, the common exit
transaction, Home release, and Box-member `fini {}`.
Source ownership/alias rules are owned by `ownership.md`.
Function fallthrough, explicit-return materialization, Script results, and
entry/process-result projection are owned by
`function-exit-and-entry-result.md`.

## 0) Scope

This SSOT fixes:

- cleanup registration and execution order
- typed Result propagation through the common exit transaction
- relation between scope cleanup and object finalization
- failure policy when cleanup/finalization handlers fail
- constructor (`birth`) partial-failure behavior
- precedence vs `lifecycle.md`

This SSOT does not decide whether an exiting source construct contributes a
Value or Unit. It only orders cleanup around the already-selected Outcome.

## 1) Core Surfaces

- Standalone `cleanup { ... }`: the sole canonical lexical registration. It
  becomes active only after control reaches the statement.
- Typed Result-only postfix `?`: may create a pending Return; it is not a
  cleanup or exception handler.
- Box-member `fini { ... }`: non-callable terminal Home hook owned by the C′
  lifecycle DropPlan. It is not a scope handler.
- `close()`/`shutdown()` and similar names: ordinary optional methods for
  explicit, possibly fallible domain shutdown.

Constraints:

- `finally` is terminology only; the surface keyword is `cleanup`.
- source `try`, `throw`, `catch`, `RecoverableFailure`, local/postfix cleanup,
  and scope-position `fini` are rejected target surfaces.

Naming rule:

```text
cleanup = when lexical/block cleanup runs
fini    = terminal Home hook inside a Box
close   = ordinary explicit domain operation
```

## 2) Unified Cleanup Model

Standalone cleanup targets one `CleanupRegistrationV1` and one
`VerifiedExitTransactionV1`. It does not authorize a `TryCatch` AST/MIR
encoding, source handler tail, runtime registration list, or backend fallback.

- Handlers run once per scope exit.
- Multiple registrations in the same scope run in LIFO order.
- Cleanup handlers are not jump targets.

## 3) Exit Ordering

On normal exit, Result `?`, `return`, `break`, `continue`, or Fault:

1. evaluate the exit expression once and protect any outgoing Result/Home in a
   pending carrier
2. cross lexical scopes inner-to-outer; run each scope's cleanup LIFO
3. release non-forwarded local Homes in reverse declaration order; ordinary
   handles have no owner effect
4. for each terminal Home release, enter the C′ lifecycle DropPlan: parent Box
   hook first, then verified owning fields in reverse declaration order, then
   native structural drop
5. publish the pending Outcome once, or release an unpublished pending value
   and publish the first Fault in time

`take` and terminal return forward a Home atomically and do not finalize it in
transit. A parent field release invokes a child hook only if it is the child's
terminal Home; another Shared Home delays that hook.

Lexical nesting determines inner/outer handler order.

## 4) Cleanup Registration Rule

A standalone cleanup body captures only declaration-time resolved bindings.
Later shadowing does not retarget it, and same-scope redeclaration remains
fail-fast. No local-declaration cleanup sugar creates a second binding rule.

## 5) Handler Restrictions and Failure Policy

Cleanup handler restrictions (parser/verifier enforced):

- forbidden: `return`, `break`, `continue`, `?`, `throw`, `await`, `yield`, or
  suspension

If a compatibility path still accepts `break`/`continue` from a cleanup block,
that path is not canonical and must be narrowed by a dedicated verifier row.

If cleanup/finalization itself Faults, preserve the first Fault in time,
complete remaining release steps best effort, record later Faults as
suppressed diagnostics, then publish the primary terminal Fault.

Object finalization runs through the lifecycle transaction defined by
`lifecycle.md`; source/runtime routes must not call the user hook directly.

## 6) `birth` Partial-Failure Rules

If constructor (`birth`) fails:

1. do not run the unpublished outer Box `fini` hook
2. release only already-initialized field Homes
3. field destruction order is reverse declaration order
4. a fully constructed child may run its own hook only when its release is
   terminal
5. for legacy `from Parent.birth(...)` compatibility paths, apply the same rule
   across the full initialized field set. New delegation code should use
   explicit field composition and `delegate field exposes`.

## 7) Home Transfer Terminology

Ordinary Hakorune code does not annotate every local as owned/borrowed.

Use **ownership transfer** or **owner forwarding** as terminology.

- A destination with a sealed Home demand transfers one available Home token
  without adding an owner.
- A terminal `return` may forward one available Home token without a second
  transfer spelling.
- `share source` is not a move synonym. It adds a same-identity independent
  owner and is the only ordinary source authority for owner acquisition.
- Historical `outbox` remains a compatibility transfer surface until its
  owning task retires it.

## 8) SSOT Priority

`scope-exit-semantics.md` is authoritative for:

- standalone cleanup target and migration/sunset boundary for older spellings
- exit ordering
- typed Result pending-return and cleanup ordering
- cleanup/finalization failure policy
- Home-transfer terminology

`lifecycle.md` is authoritative for:

- object states and terminal Home finalization
- weak-reference semantics
- memory policy (GC/non-GC)

`ownership.md` is authoritative for:

- Home/handle laws and Shared entry
- destination Home demand and result Home relation
- callable Home ABI

`function-exit-and-entry-result.md` is authoritative for:

- ordinary function/method and `Main.main` fallthrough
- explicit Return Value/Unit materialization
- Script tail-expression results
- source-entry to process-status projection

When texts conflict, use this file for scope-exit behavior and transfer terminology.
