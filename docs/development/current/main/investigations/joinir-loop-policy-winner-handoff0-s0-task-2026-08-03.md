# JOINIR Loop Policy Winner Handoff S0

Status: Policy-only caller-zero implementation slice.

Task: `JOINIR-LOOP-POLICY-WINNER-HANDOFF0-S0`

## Scope

Upgrade the pure frozen-row evaluator's qualified result with one opaque,
non-`Clone` `VerifiedLoopPolicyWinnerV1` capability:

```text
first Candidate row
  -> raw_cursor + private seal
  -> consuming winner capability
```

The cursor is migration provenance used later to pair independently-owned
structural/source facts. It is not a route/family semantic dispatch key.

The policy layer must not acquire Recipe, AST, `CanonicalLoopFacts`, Builder,
StructuralFacts, Retry, PHI, or physicalization imports. Production consumers
remain zero in this slice.

## API boundary

`LoopQualifiedV1` remains non-`Clone` and exposes only a consuming accessor:

```rust
into_parts(self) -> (LoopRouteCandidateFactsV1, VerifiedLoopPolicyWinnerV1)
```

`VerifiedLoopPolicyWinnerV1` exposes only a consuming cursor extraction for the
future selected-demand issuer. No constructor is public; only the pure policy
evaluator can issue it. `Blocked`, `Exhausted`, and `GenericDebt` results never
contain a winner capability.

## Acceptance gates

1. Candidate at raw cursor `N` yields winner cursor `N`; declined rows before it
   do not create a suffix or second winner.
2. Exhausted, policy-blocked, and Generic-debt evaluations yield no winner.
3. `Clone`/`Copy` is absent from both capability types.
4. No caller consumes the winner yet; no route ID or cursor match/index dispatch
   exists outside policy tests.
5. `loop_route_policy` tests, M3 parity, pointer guard, shared logical-demand
   guard, diff check, and release build remain green.
6. All touched Rust files remain below 800 lines.

## Stop conditions

Stop and return to `JOINIR-LOOP-SELECTED-RECIPE-DEMAND0-D0` if the winner needs
to interpret route/family, carry AST/structural facts, resolve source paths, or
solve Generic post-effect debt. Those are separate owners and gates.
