# FunctionCall weak TokenCursor Grammar Parity I0

Status: landed
Parent: `function-call-weak-paren-raw-reject-retirement-d0.md`
Row: `FUNCTION-CALL-WEAK-TOKENCURSOR-GRAMMAR-PARITY-I0`
Classification: BoxCount

## Execution brief

Decision: Project the existing weak grammar rows into TokenCursor without
changing classic parsing or the raw Builder defense.
Source authority + canonical issuer: Registry/EBNF own `weak unary_no_group`
and reject `weak (`; TokenCursor emits the same AST or stable rejection.
Non-authority: Token kind alone, raw name checks, environment flags, MIR,
tests, selfhost strings, and runtime weak representation.
Fail-fast boundary: TokenCursor consumes `WEAK` once, rejects immediate LPAREN
before parsing its contents, otherwise recursively parses one unary operand.
Smallest next slice: Add one bounded `WEAK` arm, focused Canonical/Compat tests,
and a reusable parity guard; leave strict mode and raw `WeakReject` untouched.
Non-claims: No strict-12.7 cleanup, raw retirement, generic weak callable,
weak field/runtime behavior, other parser migration, or backend work.

## Acceptance

- Direct TokenCursor parsing emits one `UnaryOp(Weak)` for `weak value`.
- Immediate `weak (` rejects with `parser/weak_paren_call_rejected` while the
  cursor still points at `(`, proving the operand was not entered.
- Classic grammar-profile witnesses remain unchanged.
- The strict tokenizer downgrade and raw `WeakReject` remain present for their
  separately ordered retirement rows.

## Landed receipt

TokenCursor now consumes `WEAK` exactly once, emits the existing Weak unary AST,
and rejects immediate parentheses while still positioned at `(`. Two focused
cursor tests, the classic two-profile grammar test, and the reusable parity
guard are green. No strict-tokenizer or MIRBuilder branch changed.
