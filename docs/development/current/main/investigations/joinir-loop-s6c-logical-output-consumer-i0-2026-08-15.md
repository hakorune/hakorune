---
Status: accepted; implementation row
Date: 2026-08-15
Decision: add one caller-zero typed logical consumer over the sealed S6C output
Scope: M8 LoopV0 forward ScanWithInit logical output only
---

# JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-LOGICAL-OUTPUT-CONSUMER-I0

## Six-line brief

```text
Decision: expose one typed caller-zero consumer result over the existing output façade.
Source authority + canonical issuer: VerifiedS6CScanWithInitLogicalOutputV1 and its try_with_output HRTB seam.
Non-authority: raw Recipe/JoinSig, rows().calls() as an independent source, MIR/JoinModule/IDs, names, selectors, Tail, fallback.
Fail-fast boundary: role/row/source/transfer parity is rejected before the callback; consumer terminal drift returns a named reject.
Smallest next slice: implement a separate <650-line consumer owner returning Result<S6CLogicalConsumerResultV1, S6CLogicalConsumerRejectV1>.
Non-claims: no new semantic facts, Recipe keys, Join closure, physical layout, Artifact, production caller, retry, or legacy retirement.
```

## Contract

The only input is `VerifiedS6CScanWithInitLogicalOutputV1`. The implementation
must enter through its private `try_with_output` façade and use the R0 canonical
role-wise pairs:

```text
Length    = fixed logical CallSlot + retained source contract
Substring = fixed logical CallSlot + retained source contract
```

The consumer result is fixed as:

```rust
Result<S6CLogicalConsumerResultV1, S6CLogicalConsumerRejectV1>
```

`S6CLogicalConsumerResultV1::Consumed` is the only typed terminal observation
of the existing sealed output. It carries no key space, source claim, Recipe,
JoinSig, MIR value, physical ID, selector, or Artifact. The reject enum names
only bounded output/consumer drift and never means “try another route”.

The consumer owner must be a new small module; do not append to the 753-line
`s6c_scan_with_init_joinir_output_rows.rs` file. Keep the new owner below 650
lines and hard-stop before 760.

## Required checks

```text
product input = exactly one combined output product
try_with_output HRTB = only borrow boundary
domains = loops 1, blocks 3, bindings 1, inputs 3, values 15,
          items 15, carriers 1, exits 1
calls = exactly Length and Substring in fixed role view
call parity = receiver/ordered args/result/placement/owner/frame exact
transfer = branch 1, Return summary 1, Backedge 1, After L0/B0/I64
Tail return -1 = absent from loop consumer
```

The `Consumed` terminal is issued only after these checks. I0 must use a
fallible private HRTB façade (`try_with_output` or equivalent) so malformed
internal view construction returns the named reject instead of panicking; the
consumer must not re-read AST/source/MIR or re-pair Facts, Recipe, and Join.

## Negative matrix

```text
missing/duplicate item or domain
Length/Substring swap
receiver, ordered-argument, result, placement, owner/frame drift
foreign source contract or transfer
condition/Return/summary/Backedge/After drift
Tail import
raw Recipe/JoinSig/JoinModule/MIR/ValueId/JoinValueSpace/name/selector
Option, (), generic unconstrained result, fallback, retry, production caller
```

Focused tests must prove the positive accepted result and named rejects for
the above drift families. Existing S6C, pointer, and Loop pre-cutover guards
are reused; no new top-level guard is needed.

## Stop conditions

Return to design stop if the consumer needs to own or mint a key, source
contract, Join transfer, MIR/JoinModule value, Artifact, or a new semantic
meaning. Such a change is a separate BoxCount row, not an expansion of I0.

## Implementation receipt (2026-08-15)

I0 now exposes one caller-zero consumer in a separate 122-line owner. It uses
the fallible `try_with_output` HRTB façade, checks the fixed domain/call/transfer
shape, and returns only `S6CLogicalConsumerResultV1::Consumed` or the named
output reject. No new key, source claim, Recipe/Join/MIR value, fallback, or
production caller was added. Focused S6C tests are 8/8, `cargo check --lib`,
format, diff, pointer, and Loop pre-cutover guards are green. The full
`loop_recipe_contract` suite is 132 passed / 1 known baseline failure at
`source_bound_core_tests::source_bound_core_rejects_derived_carrier_and_duplicate_effect_mismatch`.
