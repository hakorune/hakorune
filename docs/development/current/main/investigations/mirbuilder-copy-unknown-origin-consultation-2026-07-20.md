---
Status: Open design consultation
Date: 2026-07-20
Scope: LocalSSA successful Copy, stored `MirType::Unknown`, and the adjacent receiver-origin fallback
Related:
  - docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
  - src/mir/builder/ssa/local.rs
  - src/mir/phi_core/copy_type_propagator.rs
  - src/mir/builder/type_hint_providers.rs
---

# COPY-UNKNOWN0: LocalSSA Unknown / origin policy consultation

## Decision requested

Choose one policy for a **successful LocalSSA materialization** when its source
has both a stored `MirType::Unknown` and an existing `value_origin_newbox`
owner. The choice must make the later exact-Copy FACT0 migration possible
without silently changing receiver behavior.

```text
current blocker:
  COPY-UNKNOWN0-D0

decision:
  Does stored Unknown remain an explicit suppression sentinel for the
  receiver-origin Box fallback, or is that fallback redesigned/separated?
```

Please select exactly one of A/B/C below, or propose a strictly narrower
alternative with the same authority and failure laws. State the next code-facing
row and the claims that remain forbidden.

## Why this is a real boundary

`MirBuilder::ssa::local::ensure_inner` emits or rematerializes a value into a
fresh local `loc`. Only after the instruction succeeds, one shared block:

```text
1. copies value_types[source] to value_types[loc] when present
2. copies value_origin_newbox[source] to value_origin_newbox[loc] when present
3. if kind == Recv and value_types[loc] is absent:
     writes Box(origin) as a compatibility fallback
```

The first action currently copies `MirType::Unknown` verbatim. That makes the
following current behavior observable:

```text
source:
  type = Unknown
  origin = Owner
  kind = Recv

successful Copy:
  dst type = Unknown
  dst origin = Owner
  receiver Box(Owner) fallback = suppressed
```

If COPY0 simply treated Unknown as a non-fact and omitted the destination type,
the unchanged origin block would instead produce `Box(Owner)`. That changes
receiver representation and downstream routing without any new source evidence.

## Current evidence

| Source state after a successful LocalSSA materialization | Current destination result |
| --- | --- |
| type missing, origin missing | type missing, origin missing |
| type exact `T`, origin missing | type `T` |
| type missing, origin `Owner`, non-`Recv` | type missing, origin `Owner` |
| type missing, origin `Owner`, `Recv` | type `Box(Owner)`, origin `Owner` |
| type `Unknown`, origin `Owner`, `Recv` | type `Unknown`, origin `Owner`; fallback suppressed |
| type exact `T`, origin `Owner`, `Recv` | type `T`, origin `Owner`; fallback suppressed |

The post-success block is also reached after Const, BinOp, Compare, Select, and
Copy rematerialization, plus the ordinary fallback Copy. It is therefore not
currently a physical-Copy-only publisher.

Stored Unknown is observably distinct from absence today:

- the final Copy propagator only fills an absent destination, so a stored
  Unknown blocks that step;
- the PHI publisher reports missing and Unknown inputs with distinct reasons;
- the same-root receiver proof rejects missing and Unknown type facts at
  distinct sites;
- call/await type annotation treats a present entry, including Unknown, as
  already annotated.

These observations do not prove that every existing use is desirable. They do
prove that changing the entry-presence law is semantic work, not a FACT0
mechanical cleanup.

## Fixed authorities and non-authorities

| Concern | Authority |
| --- | --- |
| source transient type entry | current `type_ctx.value_types[source]` |
| source allocation/box origin | current `type_ctx.value_origin_newbox[source]` |
| receiver-only compatibility condition | `LocalKind::Recv` and the existing destination-type absence test |
| physical success boundary | successful `ensure_inner` emission result |
| later exact type publication | existing `TypeFactDecisionV1`, only after this policy is selected |

Non-authorities:

```text
finalized MirFunction metadata
TypePropagationPipeline / CopyTypePropagator repair
method or field names
runtime object tags
source AST shape
route success or raw fallback
new ValueId -> type/origin side tables
```

## Candidate policies

### A — retain stored Unknown as a receiver-fallback suppression sentinel

```text
Unknown source entry
  -> successful local materialization copies Unknown
  -> origin fallback remains suppressed exactly as today
```

Pros:

- preserves current LocalSSA receiver behavior exactly;
- no origin/fallback redesign in this row;
- leaves COPY0 free to migrate only already-exact source types later.

Cost:

- stored Unknown remains an intentional legacy control fact, despite FACT0
  treating Unknown as a non-fact for monotone publication;
- COPY-UNKNOWN0 must record a durable exceptional policy and its eventual
  retirement owner.

### B — normalize stored Unknown to absence and redesign the fallback atomically

```text
Unknown source entry
  -> no generic destination type publication
  -> receiver fallback is changed so it does not accidentally publish Box(owner)
```

This is only admissible if the new fallback condition has an independent,
source-faithful authority. It may not infer from method names, runtime tags, or
the fact that a raw route happened to succeed.

Pros:

- aligns Copy publication with FACT0's Unknown-as-non-fact vocabulary.

Risks:

- changes compatibility behavior and requires an explicit receiver/origin
  transaction design;
- cannot be implemented as a local omission of `value_types.insert`.

### C — split receiver compatibility fallback from generic origin propagation

```text
generic Copy transfer:
  only already-exact type facts, after successful physical Copy

receiver compatibility:
  separately owns whether origin may yield Box(owner), including Unknown policy
```

Pros:

- produces a true physical-Copy boundary for later COPY0;
- keeps type fact and origin/receiver compatibility authorities separate;
- avoids treating generic origin propagation as a type publisher.

Risks:

- needs a narrowly sealed receiver-compatibility owner; it must not become a
  second general provenance/type propagation system.

## Required answer format

Please answer all of the following concretely.

1. Select A, B, C, or a strictly narrower alternative.
2. Give the exact selected authority for `Unknown + origin + Recv`.
3. State whether current behavior is preserved or intentionally changed.
4. Specify the next task order, including whether COPY0-S0 remains forbidden.
5. Give the commit-time no-mutation law for emission failure.
6. List the exact source/check fixtures needed before any I0 connection.
7. List what remains explicitly parked: `ORIGIN0`, finalization Copy repair,
   `metadata::propagate`, direct Copy emitters, string/map/record facts,
   FieldGet, Call, and source-shape widening.

## Non-negotiable stop laws

The selected answer must not require any of the following:

```text
writing an exact type from Unknown
using finalization as lowering-time authority
falling back to another route after selected Copy failure
backfilling value_origin_newbox from type evidence
method/field/HMI-name special cases
runtime-tag inference
new persistent ValueId -> type or ValueId -> owner maps
folding non-Copy rematerialization paths into COPY0 without a separate owner
changing source grammar, runtime, backend, or ownership semantics
```

Until a policy is selected, `COPY0-S0`, `COPY0-I0`, and all LocalSSA type-map
changes remain forbidden.
