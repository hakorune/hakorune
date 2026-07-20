---
Status: Active D-prime task
Date: 2026-07-20
Scope: retire three post-success literal type annotations already owned by CONST0
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Predecessor: docs/development/current/main/investigations/mirbuilder-exact0-const-task-2026-07-20.md
---

# LITERAL-POSTEMIT-RET0: retire duplicate literal type publication

## Decision

`LITERAL-POSTEMIT-RET0-D0` is closed under
`LocalMechanicalSelectorAuthorityV1`. A read-only worker inventory consumed
the current FACT0 writer census and selected the one guard-clean retirement
root with no new type authority.

```text
LITERAL-POSTEMIT-RET0-M0
  -> LITERAL-POSTEMIT-RET0-P0
  -> LITERAL-POSTEMIT-RET0-I0
  -> LITERAL-POSTEMIT-RET0-G0
```

There is intentionally no S0 product. Adding a new literal decision or
publication wrapper would create a second type authority. The existing
canonical Const producer is already the sole authority:

```text
successful ConstantEmissionBox emit_*
  -> PreparedCanonicalConstTypeV1 commit
  -> exact transient type fact
```

The row retires only three redundant post-success writes:

```text
builder_build::build_literal
  Integer / TypedInteger / Float / Bool / String

resolved_lowering::lower_literal
  Null / Void

ops::unary::build_unary_op
  folded negative Integer literal
```

## Authority and failure boundary

`PreparedCanonicalConstTypeV1` is the sole type-fact owner. The three caller
paths may consume the returned `ValueId`, but may not re-publish its type.

```text
emitter succeeds
  -> caller receives typed ValueId
  -> caller returns it without an additional type write

emitter fails
  -> `?` returns before caller post-write
  -> no literal type fact, metadata, origin, or retry publication
```

`LiteralValue::TypedInteger` remains in scope because it first uses the same
canonical Integer Const emission and separately records its existing
exact-numeric metadata. This row does not change that metadata.

Non-authorities:

```text
AST spelling beyond the existing literal dispatch
runtime tags and names
finalized function metadata
TypeRegistry/environment policy
origin, string companion facts, and exact-numeric metadata
operator-call / Core-13 unary routes
```

## Why the nearby candidates stay parked

```text
Compare:
  `cf_common::emit_compare_func` has a void/no-op branch, so Bool publication
  needs COMPARE-RECEIPT0 before any retirement.

FieldGet:
  typed allocation occurs before FieldGet emission and FastMem has a separate
  fallback Integer authority.

Call/operator routes:
  signature/name heuristics and several route transactions remain mixed.

direct Copy outside LocalSSA:
  `metadata::propagate` transfers type, origin, string/map/record facts, and
  configuration-linked compatibility together.
```

## Row contracts

### `LITERAL-POSTEMIT-RET0-M0` — next code-facing row

```text
code_or_artifact_delta_required = 1
production behavior delta = 0
production direct-write deletion = 0
```

Add focused timing tests that prove the canonical Const commit occurs before
each caller returns, and that the caller cannot reach a duplicate write after
an emitter error. Cover ordinary Integer, TypedInteger, Float, Bool, String,
Null, Void, and folded negative Integer. The test surface must observe
transient facts and ordinary finalization snapshots; it must not introduce
metadata as a lowering-time fallback.

### `LITERAL-POSTEMIT-RET0-P0`

Prove parity before deletion:

```text
canonical transient type equals the current caller-visible type
TypedInteger exact-numeric metadata remains present
String companion fact remains present
Null/Void remain Void
failed canonical Const reaches no caller post-publication
```

### `LITERAL-POSTEMIT-RET0-I0`

Delete exactly the three redundant direct-write sites. Do not modify
`emission::constant`, `constant_type`, the exact-numeric metadata write,
string companion publication, Null/Void representation, or non-folded unary
routes.

### `LITERAL-POSTEMIT-RET0-G0`

Extend the existing `mirbuilder-type-fact-partition` guard rather than adding
a new guard family. It must update the active writer replacement inventory,
freeze zero direct literal post-emission type writes in the three caller
locations, preserve one canonical Const decision/commit owner, and enforce the
800-line cap for touched source/check files.

## Stop conditions

Stop and open a separate design row if any caller needs to publish a type not
already committed by canonical Const; if TypedInteger metadata, String facts,
origin, finalized metadata, type hints, operator routing, grammar, runtime,
backend, or ownership must change; or if a caller needs an error fallback or
retry. Do not change Compare, FieldGet, Call, `metadata::propagate`, or
Unknown retirement in this row.

## Decision lock

> `LITERAL-POSTEMIT-RET0` retires three caller-side literal type annotations
> without creating a new fact publisher. Successful canonical Const emission
> already commits every relevant exact type; callers only forward the returned
> value and preserve their companion metadata. M0/P0 prove timing, failure,
> and representation parity before I0 deletes the duplicate writes, and G0
> extends the existing FACT0 partition guard. Compare, FieldGet, Call,
> metadata propagation, and operator routes stay separate because their
> emission-success or compatibility authority is not yet single-owner.
