---
Status: Accepted
Date: 2026-04-11
Scope: decide whether to extract a generic MIR-wide `boundary_fact` / `lifecycle_outcome` vocabulary immediately after the semantic-refresh and generic-relation cleanup.
Related:
  - docs/development/current/main/phases/phase-166x/166x-90-semantic-refresh-and-generic-relation-ssot.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - src/mir/string_corridor.rs
  - src/mir/escape_barrier.rs
  - src/mir/sum_placement.rs
---

# 166x-92: boundary/lifecycle extraction decision

## Decision

Do not extract a generic MIR-wide `boundary_fact` / `lifecycle_outcome` seam in `phase-166x`.

Keep the current owners as they are:

- `src/mir/string_corridor.rs`
  - owns string-lane lifecycle/outcome and sink-state vocabulary
- `src/mir/escape_barrier.rs`
  - owns operand-role barrier causes for escape analysis
- `src/mir/sum_placement.rs`
  - owns enum-local objectization barrier causes for the sum proving slice

## Why

The current vocabularies do not answer the same question.

- `StringOutcomeFact` and `StringPlacementFact` answer:
  - what lifecycle/result state a string-lane value carries
  - where objectization/publication/materialization may sink or defer
- `EscapeBarrier` and `SumObjectizationBarrier` answer:
  - which operand-role or use-site blocks a local route from staying unmaterialized

If we force these into one generic seam now, we collapse two axes:

1. lifecycle/outcome state
2. barrier/cause state

That would create a wider shared vocabulary before we have a second real lifecycle consumer outside the string lane.

## Accepted Owner Split

### Keep local for now

- string lifecycle/outcome facts
- string objectize/publish/materialize sink-state facts
- escape barrier causes
- sum objectization barrier causes

### Keep generic

- semantic metadata refresh ownership
- `value_origin`
- `phi_relation`

The generic owner work from `166xB` through `166xE` remains the prerequisite structural cut.

## Reopen Conditions

Only reopen generic extraction when at least one of these becomes true:

1. a second non-string lane needs lifecycle/outcome facts, not only barrier causes
2. one downstream consumer must read the same lifecycle/outcome API across multiple domains
3. one MIR-side transform needs a shared generic sink-state vocabulary that cannot stay domain-local without duplicating behavior

If reopening happens, split the extraction by question:

- `lifecycle_outcome` or equivalent
- `boundary_barrier_kind` or equivalent

Do not reopen as one mixed vocabulary.

## Stop Line

- do not create a generic `boundary_fact` that mixes lifecycle state with barrier causes
- do not move barrier-cause vocabulary into runtime helpers or LLVM
- do not reintroduce helper-name semantic recovery into domain fact builders

## Result

- `phase-166x` is complete as a structural cleanup corridor
- next work returns to `phase163x-optimization-resume`
