---
Status: Closed test-only observer
Date: 2026-08-03
Decision: accepted — `JOINIR-LOOP-ACCUM-MIR-PHYSICAL-SNAPSHOT0-M5-P4-S0`
Scope: observe one existing legacy Accum physical result without creating a
       second producer
Related:
  - joinir-loop-accum-mir-physical-snapshot-design0-m5-p4-task-2026-08-03.md
  - joinir-loop-accum-semantic-parity-readbinding-m5-task-2026-08-03.md
  - joinir-loop-physical-edge-path-p1b-task-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# DirectAccum physical snapshot observer

## Boundary

This is an evidence-only slice. It observes the MIR produced by the existing
test-only legacy oracle after `RecipeComposer` -> `PlanVerifier` ->
`PlanLowerer`. It does not make the portable Recipe a producer and does not
wire `route_loop`.

The observer must not:

- lower a Recipe, `CorePlan`, or AST itself;
- allocate blocks, values, PHIs, or Binding SSA entries;
- call `LoopPhiMaterializerV1` as a second execution path;
- alter Retry, route selection, Generic disposition, or candidate publication;
- claim portable/full parity before a shared physicalizer exists.

## Product

Add a separate `#[cfg(test)]` child module (the existing materializer parent is
near the 800-line limit) with a comparison-only product:

```rust
struct AlphaPhysicalMirDigestV1 {
    cfg: Box<[CfgRoleRowV1]>,
    instructions: Box<[InstructionRoleRowV1]>,
    phis: Box<[PhiRoleRowV1]>,
    results: Box<[ResultRoleRowV1]>,
}
```

The exact row structs may stay private to the test child. They must carry
semantic labels only:

- CFG roles from the verified P1b paths (`preheader`, `header`, `body`,
  `step`, `after`) and terminator successor roles;
- operation kind and canonical operand/dataflow labels;
- PHI binding/class, predecessor port role, and input provenance;
- final binding/result/unit disposition and MIR type class.

Raw `ValueId`, `BasicBlockId`, allocation order, pointer addresses, and route
names must not appear in the final digest. A legacy-only role map may inspect
the existing CorePlan and the sealed JoinSig to label already-emitted blocks;
it may not emit or repair anything.

## Ordered implementation

1. Create the child module and the private alpha digest DTOs. Reuse existing
   snapshot helpers where they preserve the boundary; do not widen the
   793-line `loop_phi_materializer_tests.rs` parent.
2. Lower the existing direct Accum source through the already-committed
   test-only oracle and snapshot the resulting candidate function.
3. Assert that the observer sees the Standard5 physical topology, actual PHI
   instructions, typed arithmetic/compare rows, and final result disposition.
4. Compare the CFG/path/PHI-role portion with the green P1b structural digest.
   Keep operation/value parity explicitly labeled as legacy evidence, not
   portable consumer parity.
5. Add a static guard proving the observer is test-only and that no production
   materializer/physicalizer caller was added.

## Acceptance

- focused observer test is green;
- P1b structural/path alpha digest remains green;
- `phi_lifecycle` and Binding SSA remain the only PHI/SSA writers;
- no production `route_loop`, Retry, Generic, JoinIR fallback, or candidate
  commit behavior changes;
- observer child and any touched Rust source stay below 800 lines;
- no M10a/full portable MIR parity claim is made.

## Deferred to M10a

The second snapshot producer, full portable-vs-legacy physical parity,
late-failure candidate discard, and fresh-session reuse require the first real
shared Loop physicalizer. If implementing the observer needs a duplicate CFG or
instruction lowerer, stop this row and return to the P4 design card.

## Close evidence (2026-08-03)

- `AlphaPhysicalMirDigestV1` observes the legacy candidate's Standard5 blocks,
  terminators, arithmetic/compare operations, actual PHI instructions, final
  carriers, and unit result without retaining raw IDs.
- The structural P1b witness remains green and still shows the logical
  backedge as the explicit `Body -> Step -> Header` path.
- Focused Loop materializer tests: 16/16; `cargo check -q`, current-state
  pointer guard, and in-place replacement guard are green.
- No production caller, Retry behavior, candidate publication, or PHI/SSA
  writer changed. Full portable physical parity remains deferred to the first
  shared physicalizer.
