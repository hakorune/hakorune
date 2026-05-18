# Hako Alloc Segment Map Readiness Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

Freeze the explicit-ID segment-map readiness proof family as one validation
pack before adding the next allocator behavior row.

The pack is:

```text
MIMAP-149A blocked-substrate matrix
MIMAP-151A segment-map scalar lookup boundary inventory
MIMAP-153A lookup-guarded readiness composition
ROW-VALIDATION-PROFILE-001 manifest metadata
ROW-VALIDATION-PROFILE-002 L2 split commands
```

## Validation Pack

Pack id:

```text
segment-map-readiness
```

Daily validation is L2:

```text
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-readiness --level L2
```

L2 means static checks, VM proof, MIR JSON emit/schema assertions, and
pure-first route preflight. It must not build or run the EXE.

Public no-arg row guards remain full L3 for first-pattern/backend-route/closeout
evidence. The split is operational only; it does not reduce the existing
no-arg evidence contract.

## Closeout Row

```text
MIMAP-155A segment-map readiness validation pack closeout guard
```

MIMAP-155A verifies:

- all three family rows are landed;
- proof manifest entries carry `closeout_pack = "segment-map-readiness"`;
- the family has `cmd_l2` commands for L2 manifest selection;
- check script index and guard manifest name the closeout entry;
- no app/owner-specific backend `.inc` matcher exists;
- allocator provider activation remains inactive.

## Stop Lines

MIMAP-155A must not add:

- allocator behavior;
- real segment-map execution;
- raw pointer residence or pointer-derived lookup;
- arena backing allocation;
- atomic bitmap execution;
- OSVM/page-source execution;
- worker scheduling or source-level concurrency behavior;
- provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`;
- backend helper/app/owner name matchers.

## Next Row

After closeout, the selected row is:

```text
MIMAP-156A post-segment-map-readiness-closeout row selection
```

MIMAP-156A chooses exactly one small follow-up after the validation pack is
stable. It should prefer composing accepted readiness into the modeled consume
or ledger lane while keeping cross-function `Result` direct ABI, runtime sum
materialization, raw pointer residence, and provider activation closed.
