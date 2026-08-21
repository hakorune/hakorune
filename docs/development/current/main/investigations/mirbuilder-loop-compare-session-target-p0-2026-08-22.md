Status: selected fast caller-zero physical contract; implementation not started
Task: MIR-LOOP-COMPARE-SESSION-TARGET-P0
Date: 2026-08-22
Priority: next bounded physical contract
Parent: MIR-EMIT-CANONICAL-STRICTNESS-D0
PreviousCard: MIR-LOOP-OPERATION-EMITTER-SPLIT-S0
NextCard: MIR-LOOP-COMPARE-SAME-BLOCK-OPERANDS-P0
---

# Loop Compare same-session open target P0

## Six-line brief

```text
Decision: issue one private open-target witness from the owning canonical CFG session.
Source authority + canonical issuer: CanonicalCfgSessionV1 owns created/open/sealed state; the scoped Loop service issues the witness.
Non-authority: current_block, ensure_block_exists, raw BasicBlockId, Loop target fields, Builder presence, dominance helpers.
Fail-fast boundary: before any Compare preparation or MIR instruction append.
Smallest next slice: track session-created blocks and reject foreign, uncreated, missing, sealed, terminated, or mismatched targets.
Non-claims: operands, dominance, ledger reservation, strict writer, caller connection, production I0/R0.
```

## Readiness and fixed boundary

S0 has landed as a behavior-neutral owner split. This card is the next
caller-zero P0 and implements only the target half of the accepted C-prime
design. It must not start a Compare leaf or create a new semantic operation
receipt.

The target proof is valid only when all of these are co-owned by the same
scoped canonical service:

```text
canonical function owner
CanonicalCfgSessionV1
session-created block
current MIR block
Loop target receipt naming that exact block
open / unsealed / unterminated state
```

The function entry or preheader is not silently admitted as session-created.
The first cohort uses segment blocks allocated through the existing
`LoopPhysicalServicesV1 -> CanonicalCfgSessionV1::create_block()` route.

## Intended change

Add a narrow child owner under `canonical_cfg/` rather than growing the
already-large session parent. The child should contain the private witness
and its typed rejection vocabulary. `CanonicalCfgSessionV1` should record the
exact blocks it created; `create_block` must become mutable if required by
that ownership model. Add one private preparation method that checks:

```text
block is in this session's created set
block exists in the current function
block is absent from the session sealed map
the MIR block is unsealed
the MIR block is unterminated
the Loop target receipt names the same block and owner
```

The witness must not escape the scoped service callback and must not be
constructible from a raw block ID. Do not add a general dominance API,
future-edge plan, CFG epoch, or a second block authority.

## Allowed files

```text
src/mir/builder/resolved_lowering/canonical_cfg/session.rs
src/mir/builder/resolved_lowering/canonical_cfg/mod.rs
src/mir/builder/resolved_lowering/canonical_cfg/open_instruction_target.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/topology.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/segment_dispatcher.rs
```

Add focused tests beside the canonical CFG child or in the existing
canonical-CFG test module. Do not edit `operation_dispatcher.rs` to connect
Compare yet; that is a later card.

## Forbidden overlap

```text
no lhs/rhs definition scan
no operand or type witness
no Bool destination
no ledger Reserved/Poisoned state
no strict writer or append-core change
no general dominance or compute_dominators use
no current_block/ensure_block_exists fallback
no legacy retry/fallback
no production caller or old-edge retirement
no unrelated builder cleanup or performance work
```

## Acceptance

- one private open-target witness has one constructor path owned by the
  canonical CFG session;
- created-block membership is session-local and cannot be reconstructed from
  `BasicBlockId` alone;
- foreign owner, foreign session, uncreated, missing, sealed, terminated,
  and target-receipt mismatch cases reject before MIR mutation;
- a valid session-created open target is accepted without changing MIR;
- the witness does not expose a general block mutation or dominance claim;
- no new production Compare caller is connected and the current caller census
  remains zero outside `#[cfg(test)]`;
- focused tests, `cargo check --lib`, source-size, pointer, and diff guards are
  green.

Focused gates:

```text
RUSTFLAGS='-Awarnings' cargo test --lib canonical_cfg -- --nocapture --test-threads=1
RUSTFLAGS='-Awarnings' cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## NoSafeSlice

Return to the strictness D0 without implementation if the target cannot be
bound to the same canonical CFG session, if the scoped service permits
session re-pairing, if existing segment allocation cannot provide the created
set, or if satisfying this card requires admitting a preheader/entry or
building a general dominance proof. The next card is operands only after this
target witness is private, typed, and green.
