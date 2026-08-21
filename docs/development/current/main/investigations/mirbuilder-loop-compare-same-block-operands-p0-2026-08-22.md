Status: selected fast caller-zero physical contract; implementation not started
Task: MIR-LOOP-COMPARE-SAME-BLOCK-OPERANDS-P0
Date: 2026-08-22
Priority: next bounded physical contract
Parent: MIR-EMIT-CANONICAL-STRICTNESS-D0
PreviousCard: MIR-LOOP-COMPARE-SESSION-TARGET-P0
NextCard: MIR-LOOP-COMPARE-RESULT-LEDGER-P0
---

# Loop Compare same-block operand P0

## Six-line brief

```text
Decision: admit only Published full Loop value receipts whose unique physical Integer definition is already in the exact open target block.
Source authority + canonical issuer: CanonicalSsaFunctionSessionV2 owns the physical-definition scan; one Loop operand issuer co-seals the existing ledger receipt with that witness.
Non-authority: raw ValueId, ledger physical_block alone, names, schedule ordinal, compute_def_blocks, compute_dominators, type_ctx alone, and final verifier diagnostics.
Fail-fast boundary: before any Compare destination preparation, ledger reservation, writer call, or MIR mutation.
Smallest next slice: issue a private same-block Integer operand witness for the existing full Published receipt, rejecting missing, pending, foreign, parameter, duplicate, cross-block, and non-Integer definitions.
Non-claims: cross-block dominance, inherited/parameter operands, result reservation, strict writer, Compare caller connection, production I0/R0, and performance.
```

## Fixed boundary

The preceding P0 now proves that a target is created by the same owner-bound
canonical CFG session and remains open. This card adds only the operand half of
the accepted C-prime law:

```text
full Published LoopOperationValueReceiptV1
+ owner and claimed target block
+ unique actual MIR definition
+ actual definition block == target
+ exact Integer type
    -> private same-block Integer operand witness
```

The Loop receipt is transport evidence, not a second physical definition
authority. The canonical SSA session performs the actual MIR definition scan.
The Loop-side issuer checks receipt owner/class/block and co-seals that result
with the canonical definition witness. It must not obtain a raw `ValueId` from
the ledger and reconstruct the rest later.

## Allowed files

```text
src/mir/builder/resolved_lowering/canonical_ssa/mod.rs
src/mir/builder/resolved_lowering/canonical_ssa/session.rs
src/mir/builder/resolved_lowering/canonical_ssa/same_block_operand.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_ledger.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_i64_operands.rs
```

Focused tests may be added beside the canonical SSA child and the Loop
physicalizer operand child. Do not edit `operation_dispatcher.rs`, the
destination ledger state, `builder_emit.rs`, or any Compare caller yet.

## Required state and errors

The input must be a full `LoopOperationValueReceiptV1`, not `get() -> ValueId`.
The canonical session must reject before effects when:

```text
session owner is unavailable or mismatched
receipt owner is foreign
receipt claimed block differs from the open target
receipt is absent, Reserved, Poisoned, or otherwise not Published
ValueId has no actual definition
ValueId has multiple actual definitions
definition is a function parameter
definition block differs from the target
definition type is Unknown, Bool, or non-Integer
```

`compute_def_blocks()` and `compute_dominators()` are not acceptance APIs:
they lose instruction order or retain legacy unreachable-block semantics.
The same-block proof is supplied by the actual unique definition record and
the later strict append order, not by a block-only map.

Every rejection must leave instruction count, type context, and the existing
ledger unchanged. No fallback or retry may turn a rejection into a legacy
Compare emission.

## Acceptance

- one private same-block Integer witness constructor path exists;
- only a full Published receipt can enter the issuer;
- owner, target, value, unique definition, and type are co-sealed;
- positive fixture proves an existing Header-local definition is accepted;
- negative fixtures cover missing, pending, foreign, parameter, duplicate,
  cross-block, and non-Integer definitions;
- no general dominance API, CFG epoch, inherited-value admission, or raw
  `ValueId` escape is added;
- no Compare append, destination, result reservation, caller connection, or
  production claim is introduced;
- focused tests, `cargo check --lib`, source-size, pointer, and diff guards
  are green.

## NoSafeSlice

Return to the strictness D0 without implementation if the existing Loop ledger
cannot provide a full Published receipt, if the canonical SSA session cannot
locate one unique physical definition without reconstructing from names or
ordinals, if the current canary requires a cross-block/parameter/inherited
operand, or if the witness would need to own a raw mutable MIR reference.
