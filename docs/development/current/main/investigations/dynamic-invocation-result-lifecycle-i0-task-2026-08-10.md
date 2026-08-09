# DYNAMIC-INVOCATION-RESULT-LIFECYCLE-I0

Status: closed caller-zero logical I0; production consumer 0
Date: 2026-08-10
Depends on: `DYNAMIC-CARRIER-LEXICAL-DISPOSITION-D0` accepted

## Goal

Consume the existing complete Dynamic semantic program and retain one private
complete lifecycle catalog for its two verified Dynamic invocation results:

```text
I6 Normal -> V10 -> exact Loop-body local ch
I7 Normal -> V11 -> exact inner-condition temporary
I6/I7 Fault -> static authorization retained, runtime carrier instance 0
```

## Structure

Add a private child under:

```text
src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/
  invocation_carrier_lifecycle.rs
```

The semantic-program issuer derives the rows internally from retained Recipe,
source claims, call relations, and invocation envelopes. It accepts no
caller-supplied owner, item/value key, source site, scope, destination, or
carrier category.

The catalog and owned rows are non-`Clone`, have private fields, expose only
borrow-scoped row views, and cannot be separated from the retained semantic
program. Those views retain exact call/result source, local declaration and
read source, temporary boundary source, and Recipe relations; downstream
owners never reconstruct source identity from Recipe keys.

Rename the misleading compatibility vocabulary in the same bounded module:

```text
DynamicInvocationResultHomeV1
  -> DynamicInvocationResultLifecycleV1

result_home()
  -> result_lifecycle()
```

This is an ownership-neutral naming correction; it must not add a third
result contract or retain the old name as a second authority.

## Acceptance

- exact two-row Recipe-order golden: I6/V10 local, I7/V11 temporary;
- every row borrows `EndExactlyOnceUnlessForwarded` from the canonical
  language-wide Dynamic envelope; the profile does not redefine it;
- I7 exact argument is V10 and its envelope supplies
  `BorrowedNoEscapeForInvocation`; I7 does not move/end V10's obligation;
- missing/duplicate/foreign/wrong-result/wrong-destination rows reject before
  Builder effect;
- static authorization rows always exist; I6/I7 Fault instantiates no runtime
  carrier and exact Normal publication instantiates one;
- no Home root/state, cleanup plan, physical end, Fault execution, Completion,
  CFG/MIR, retry, or fallback;
- `cargo test -q --lib dynamic_full_body_recipe` and focused contract tests;
- owner README, `docs/reference/language/dynamic-invocation.md`, task receipt,
  and guards update in the same commit;
- all touched source files remain below 800 lines.

## Nonclaims

```text
V9/V17 DynamicAdd lifecycle
complete callable carrier flow
Return forwarding
CFG-complete end coverage
Home classification / Home Flow
physical cleanup / production activation
```

## Closeout

Landed implementation:

```text
dynamic_invocation_contract:
  ResultHome vocabulary retired in favor of ResultLifecycle

semantic_program/invocation_carrier_lifecycle.rs:
  exact two-row private catalog
  OnNormalResultPublication authorization
  canonical-envelope EndExactlyOnceUnlessForwarded
  exact source-backed LocalBinding(ch) / FullExpressionTemporary(I9)
```

Focused tests prove the exact source-backed golden, I7 argument/borrow
relation, duplicate producer rejection, wrong result rejection, wrong
temporary boundary rejection, missing-envelope rejection, non-copy owned row
surface, and no Home/physical escape. The next row is the separate
`DYNAMIC-OPERATOR-CARRIER-LIFECYCLE-D0` consultation; this I0 does not infer a
contract for V9/V17.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib dynamic_full_body_recipe
  21 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib dynamic_invocation_contract
  5 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib typed_schema_v2
  green
RUSTFLAGS='-Awarnings' cargo test -q --lib join_sig
  green
RUSTFLAGS='-Awarnings' cargo check -q
  green
```
