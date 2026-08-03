---
Status: active test-only task order
Date: 2026-08-03
Decision: accepted boundary — `JOINIR-LOOP-ACCUM-SEMANTIC-PARITY-READBINDING0-M5-S3`
Scope: DirectAccum semantic parity after the structural P1b witness
Related:
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - joinir-loop-physical-edge-path-p1b-task-2026-08-03.md
---

# DirectAccum semantic parity: ReadBinding fixture

## Boundary

P1b-4a proved only that JoinSig, physical paths, predecessor roles, and
materialized header-PHI rows are independent of physical ID allocation. It did
not prove arithmetic or final-value parity: the current direct fixture feeds
carrier entry values directly into `BinaryI64`/`CompareI64` and has no
`ReadBinding` operation.

This task closes that semantic gap in a separate test-only lane. It must not
wire production `route_loop`, change Retry/selection, add a PHI/SSA authority,
or make `LoopPhiMaterializerV1` read AST/CorePlan/PlanLowerer.

## Design contract

The portable fixture must make the current binding read explicit:

```text
inputs:    i=v0, sum=v1
condition: ReadBinding(i)->v2 -> Const(3)->v3 -> Compare(v2,v3)->v4
body:      ReadBinding(sum)->v5 -> Add(v5,1)->v7 -> Write(sum)
           ReadBinding(i)->v8   -> Add(v8,1)->v10 -> Write(i)
backedge:  i=v10, sum=v7
```

The schema already owns `LoopOperationV1::ReadBinding`; no new recipe
vocabulary is needed. Re-number the fixture values and update verifier,
JoinSig, physical-map, and expected-PHI rows together. Keep the existing
`inputs`/carrier entries (`i=v0`, `sum=v1`) unless a verified counterexample
requires otherwise.

## Ordered slices

1. **Fixture and logical contract**
   - add the explicit reads;
   - assert `LoopRecipeVerifierV1` and `LoopJoinSigElaboratorV1` accept it;
   - assert the header carrier rows use the post-read/write values.

2. **Binding-aware test projection**
   - keep an operation projection separate from the M6-B physical map; do not
     stuff every intermediate value into `LoopLogicalToPhysicalMapInputV1`;
   - model `ReadBinding(result)` as an alias to the current binding source, not
     as a new MIR definition, and model `WriteBinding` as a binding-environment
     update;
   - map only the defined operation results needed by the semantic digest, not
     only edge payload values and PHI destinations;
   - keep this projection in a separate test child because the existing
     materializer test module is already 767 lines.

3. **Legacy oracle**
   - build the equivalent existing Accum source once;
   - run `RecipeComposer` -> `PlanVerifier` -> `PlanLowerer` only from the
     `#[cfg(test)]` oracle module;
   - do not expose PlanLowerer or synthetic AST helpers to production recipe
     code.

4. **Alpha semantic digest**
   Compare only canonical labels, never raw `ValueId`/`BasicBlockId` or
   allocation order. The digest must cover:

   - logical operations and dataflow, including ReadBinding resolution;
   - CFG edge roles and Standard5 path shape;
   - header PHI inputs and carrier classes;
   - final binding values after the after-PHI/forwarding phase, result/unit
     semantics, and MIR types.

5. **Isolation gates**
   - inject a late legacy-lower failure and prove the unpublished compile
     candidate is discarded;
   - prove a fresh session succeeds afterward;
   - keep production materializer callers at zero and keep Generic/Retry
     untouched.

## Acceptance and stop conditions

- DirectAccum portable and legacy digests are equal after alpha normalization.
- The structural P1b digest remains green and unchanged.
- Nested portable golden remains structural/path evidence only; it has no
  equivalent legacy semantic oracle in this task.
- `phi_lifecycle` and Binding SSA remain the sole PHI/SSA owners.
- All touched Rust files stay below 800 lines; semantic tests use a new child
  module instead of widening the existing 767-line test file.
- Any need for production AST reconstruction, route selection, Retry,
  route-specific PHI repair, or a second writer is a design stop, not a
  fallback implementation.
