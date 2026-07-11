---
Status: Landed
Date: 2026-07-05
Scope: docs-only policy update for MirBuilder hard-authority migration.
---

# MIRBUILDER-AUTHORITY-FACADE-HARD-AUTHORITY-PILOT-POLICY-001

## Decision

Use authority facades before meaning-based Rust crate splits.

```text
authority facade -> Rust oracle fixture -> .hako owner -> parity gate
```

Meaning-based crates are useful only after the authority seam is proven. Broad
crate splitting is held because it can preserve Rust-shaped module boundaries
instead of the target `.hako` authority boundaries.

## Policy

```text
do_now:
  cut one narrow authority facade
  freeze input/output DTO with fixture
  prove hand-authored .hako parity
  mark HakoAdoptedScoped

do_later:
  extract the proven facade to a meaning-based crate if it reduces dependency
  direction or test boundary cost

do_not_do_now:
  broad crate split
  crate split before DTO/parity proof
  lowering/mutation/allocation authority movement
```

## Hako Capability Boundary

```text
enough_now:
  token snapshot -> fact DTO
  facts -> small symbolic recipe / plan DTO
  pure reducer / classifier / formatter

gated:
  full AST traversal authority
  MIR mutation
  ValueId / BlockId allocation authority
  backend lowering
  normal-route execution
  broad typed-object / enum-handle / MapBox ABI use
```

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-011
```
