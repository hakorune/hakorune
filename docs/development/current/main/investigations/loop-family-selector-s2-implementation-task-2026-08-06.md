---
Status: Closed caller-zero implementation / next shallow selection gate taskized
Date: 2026-08-06
Decision: LOOP-FAMILY-SELECTOR-S2-IMPLEMENTATION
Design receipt: loop-family-selector-s2-design-task-2026-08-06.md
Authority: docs/development/current/main/design/loop-family-observation-policy-ssot.md
---

# Loop-family selector S2 implementation task

## Scope

Implement one caller-zero semantic selector for the landed
`VerifiedLoopFamilyAdmissionWindowV1`. This cell adds only the typed selector
consumer and focused evidence. It does not open Recipe demand, JoinSig,
Builder/MIR, physical lowering, production selection, or legacy retirement.

## Source authority

The only input is an assembler `Ready(window)` consumed by value exactly once.
The window already owns:

```text
resolver-issued non-Clone identity lease
fixed DirectAccum / NestedPredicate / LoopTrue / LoopCond / GenericG0 rows
co-sealed mode
complete coverage
```

The selector must not re-resolve source, inspect AST, read route IDs,
schedules, cursors, legacy policy, or reconstruct a row from names.

Only `Candidate` and `Declined` rows can enter this cell. Assembler
`Rejected`/`Unresolved` outcomes are terminal before selection and retain their
own lease/row evidence.

## Output algebra

```text
1 Candidate + 4 Declined -> Selected
2+ Candidates            -> Rejected(Overlap)
5 Declined               -> Unresolved(OutOfWindow)
NoCandidate              -> not an S2 output; M8 whole-unit proof only
```

The selected product owns the source lease, common mode/coverage, selected
family tag, and the typed candidate payload. Selector failures own the
consumed lease and all five rows; overlap retains every candidate payload.
There is no selector row-rejected arm.

## Implementation boundary

Add:

```text
src/mir/loop_route_policy/family_selector.rs
src/mir/loop_route_policy/family_selector_tests.rs
```

Keep `family_admission.rs` as the sole assembler and keep the historical
`family_selection.rs` marker test-only; do not rename or promote it.

The admission window/rows need consuming `into_parts` APIs. Row classification
must move typed candidate payloads without `Clone`, `Copy`, relookup, or
synthetic reconstruction. Use one `CanonicalLoopFamilyCandidateV1` enum with
five typed variants and one `CanonicalLoopFamilySelectionV1` outcome family.

Allowed selector dependencies:

```text
VerifiedLoopFamilyAdmissionWindowV1
VerifiedLoopFamilyAdmissionRowsV1
LoopFamilyObservationRowV1
the five typed observation enums/candidate payloads
common mode/coverage/tag primitives
std collections only
```

Forbidden:

```text
AST / resolver issuer / source lookup
loop_structural_facts producers
family_selection.rs / policy.rs / route IDs / schedules / cursors
Recipe / JoinSig / BindingKey
Builder / MIR / ValueId / BasicBlockId / PHI
retry / fallback / runtime effects
production caller
```

## Focused acceptance evidence

1. Each of the five families can be the single candidate with four declines.
2. Two candidates reject as `Overlap` and retain both candidate payloads plus
   all five rows and the lease.
3. Five declines produce `OutOfWindow` with all rows and the lease retained.
4. The consumed window is move-only and cannot be selected twice.
5. Mode/coverage and source lease are preserved in selected/failure products.
6. There is no source relookup, clone, retry, fallback, or production caller.
7. Focused tests, selector guard, current-state guard, and diff check are
   green.
8. This task's implementation commit updates the loop SSOT, reference matrix,
   README, workstream, and current mirrors in the same commit.

## Finish line

This cell closes only when the new selector is caller-zero, under 800 lines per
source/test file, has no legacy selector import, and its typed outcomes are
verified. Production selection remains a later activation requiring the existing
`GENERIC-SELECTION-OPEN-D0` candidate-envelope gate, then Recipe handoff,
physical/parity proof, legacy caller-zero census, and the M10b switch.

## Implementation receipt

Closed on 2026-08-06. `family_selector.rs` and its focused tests now provide
the five-family `Candidate|Declined` selector boundary. The shared selector
guard, current-state guard, focused tests, cargo check, and diff check are
green; all touched source/test files remain below 800 lines. The same commit
updates the loop policy SSOT, reference matrix, module README, workstream, and
current mirrors. No Recipe/JoinSig, Builder/MIR, physical, production, retry,
fallback, or legacy deletion caller was opened. The next row is the existing
`GENERIC-SELECTION-OPEN-D0` shallow design gate; no deeper D4 suffix is added.
