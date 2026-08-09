# DYNAMIC-INVOCATION-RESULT-LIFECYCLE-I0

Status: ready implementation row
Date: 2026-08-10
Depends on: `DYNAMIC-CARRIER-LEXICAL-DISPOSITION-D0` accepted

## Goal

Consume the existing complete Dynamic semantic program and retain one private
complete lifecycle catalog for its two verified Dynamic invocation results:

```text
I6 Normal -> V10 -> exact Loop-body local ch
I7 Normal -> V11 -> exact inner-condition temporary
I6/I7 Fault -> publication and lifecycle obligation 0
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

The catalog is non-`Clone`, has private fields, exposes only a borrow-scoped
read view, and cannot be separated from the retained semantic program.

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
- every row has `EndExactlyOnceUnlessForwarded`;
- I7 borrows V10 and does not move/end its obligation;
- missing/duplicate/foreign/wrong-result/wrong-destination rows reject before
  Builder effect;
- no result row exists on I6/I7 Fault;
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
