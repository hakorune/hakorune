Status: landed fast caller-zero physical contract; no Compare caller connected
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
Source authority + canonical issuer: the Loop operand issuer validates the full receipt and creates a neutral request; CanonicalSsaFunctionSessionV2 revalidates its own CFG and owns the physical-definition scan, then the Loop issuer co-seals both.
Non-authority: raw ValueId, ledger physical_block alone, names, schedule ordinal, compute_def_blocks, compute_dominators, type_ctx alone, and final verifier diagnostics.
Fail-fast boundary: before any Compare destination preparation, ledger reservation, writer call, or MIR mutation.
Smallest next slice: issue a private same-block Integer operand witness for the existing full Published receipt, rejecting unavailable, foreign, parameter, duplicate, cross-block, and non-Integer definitions; pending/poisoned ledger states belong to the next card.
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
authority. The Loop-side issuer first obtains the full Published receipt and
checks its key/owner/class/claimed block. It then creates only a neutral
request containing `owner + target block + physical ValueId`. The canonical SSA
session reissues its own session-owned target witness and performs the actual
MIR definition scan. The Loop issuer co-seals that witness with the original
receipt and never returns a raw `ValueId` as a standalone product.

## Allowed files

```text
src/mir/builder/resolved_lowering/canonical_ssa/mod.rs
src/mir/builder/resolved_lowering/canonical_ssa/session.rs
src/mir/builder/resolved_lowering/canonical_ssa/session/same_block_operand.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_ledger.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_i64_operands.rs
```

`canonical_ssa/session.rs` receives only the child-module declaration and
session facade method; its implementation body stays below the 760-line
split trigger. Focused tests may be added beside the canonical SSA child and
the Loop physicalizer operand child. Do not edit `operation_dispatcher.rs`,
the destination ledger state, `builder_emit.rs`, or any Compare caller yet.

## Required state and errors

The input must be a full `LoopOperationValueReceiptV1`, not `get() -> ValueId`.
The canonical session must reject before effects when:

```text
session owner is unavailable or mismatched
receipt owner is foreign
receipt claimed block differs from the open target
receipt is absent or otherwise not Published; Reserved/Poisoned are not yet
ledger states and belong to the next Ledger P0
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

## Finite state table

| State | Sole authority | MIR / ledger effect | Terminal |
| --- | --- | ---: | --- |
| `ReceiptUnavailable` | Loop ledger | none | typed reject; no fallback |
| `ReceiptReady` | Loop ledger | none | neutral request only |
| `ReceiptOwnerMismatch` | Loop operand issuer | none | typed reject |
| `ReceiptClassMismatch` | Loop operand issuer | none | typed reject |
| `ReceiptTargetMismatch` | Loop operand issuer | none | typed reject |
| `DefinitionMissing` | canonical SSA session | none | typed reject |
| `DefinitionDuplicate` | canonical SSA session | none | typed reject |
| `ParameterNotAdmitted` | canonical SSA session | none | typed reject |
| `CrossBlockDefinition` | canonical SSA session | none | typed reject |
| `TypeUnavailable` / `TypeUnknown` / `TypeMismatch` | canonical SSA session | none | typed reject |
| `Ready` | Loop operand issuer after SSA co-seal | none | private operand witness only |

`Reserved` and `Poisoned` are intentionally absent from this table because the
current ledger cannot represent them yet; adding those states is the next
card, not hidden state merging in this P0.

## Acceptance

- one private neutral request constructor and one private same-block Integer
  witness constructor path exist;
- only a full Published receipt can enter the issuer;
- owner, target, value, unique definition, and type are co-sealed;
- positive fixture proves an existing Header-local definition is accepted;
- negative fixtures cover missing, foreign, parameter, duplicate, cross-block,
  and non-Integer definitions; pending/poisoned ledger states are explicitly
  deferred to the next Ledger P0 because the current ledger cannot represent
  them;
- no general dominance API, CFG epoch, inherited-value admission, or raw
  `ValueId` escape is added;
- no Compare append, destination, result reservation, caller connection, or
  production claim is introduced;
- focused tests, `cargo check --lib`, source-size, pointer, and diff guards
  are green.

## Evidence

```text
same_block_operand focused tests: 3 passed
compare_i64_operands focused test: 1 passed
canonical_cfg regression suite: 32 passed
loop_recipe_physicalizer regression suite: 29 passed
cargo check --lib: passed (baseline warnings only)
canonical_ssa/session.rs: 735 lines
same_block_operand.rs: 338 lines
compare_i64_operands.rs: 157 lines
canonical Compare caller connection: 0
legacy Compare caller removal: 0 (CONNECT0 remains closed)
```

The P0 is deliberately caller-zero. It proves the physical operand contract
without opening result reservation, strict append, or production selection.
The next bounded row is the explicit result-ledger reservation/commit state.

## NoSafeSlice

Return to the strictness D0 without implementation if the existing Loop ledger
cannot provide a full Published receipt, if the canonical SSA session cannot
locate one unique physical definition without reconstructing from names or
ordinals, if the current canary requires a cross-block/parameter/inherited
operand, if the neutral request becomes a semantic receipt, or if the witness
would need to own a raw mutable MIR reference.
