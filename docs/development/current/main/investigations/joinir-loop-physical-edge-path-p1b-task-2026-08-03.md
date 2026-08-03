---
Status: active test-only task order
Date: 2026-08-03
Decision: accepted boundary — `JOINIR-LOOP-ACCUM-VERIFIED-RECIPE-CONSUMER0-P1B-FIXED-CFG-PARITY0-S2`
Scope: explicit logical-edge to Standard5 physical-path witness
Related:
  - ../investigations/joinir-loop-accum-verified-recipe-consumer-p1-design-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# Accum P1b: explicit physical edge paths

## Problem

`LoopJoinPortV1::Body` is used both as the body-entry target and as the source
of the logical backedge. Standard5 physical lowering cannot map both meanings
to one block: the backedge is `Body -> Step -> Header`, and the header PHI
predecessor is the terminal `Step`, not `Body`.

The existing M6-B map has only `(loop, port) -> BasicBlockId`; silently treating
`Body` as `Step` would corrupt the body-entry edge, while silently using
`Body` as the PHI predecessor would claim false parity. The physical path must
therefore become an explicit field of the verified physical map.

## Authority boundary

```text
VerifiedLoopJoinSigV1           semantic logical edges
        |
        v
LoopPhysicalEdgePathV1          explicit physical path capability
        |
        v
VerifiedLoopLogicalToPhysicalMapV1
        |
        v
existing LoopPhiMaterializerV1  PHI transaction only
```

`LoopPhysicalEdgePathV1` is physical mapping data, not a new semantic recipe or
CFG/SSA authority. It must not read AST, routes, facts, CorePlan, variable maps,
or PlanLowerer. `phi_lifecycle` and function-owned Binding SSA remain the only
PHI/SSA owners.

## Minimal API

Put the path type in a separate source file so the materializer stays below
800 lines:

```rust
struct LoopPhysicalEdgePathV1 {
    loop_key: LoopNodeKeyV1,
    role: LoopJoinEdgeRoleV1,
    blocks: Box<[BasicBlockId]>,
    terminal_predecessor: BasicBlockId,
}
```

The input map gains `edge_paths: Vec<LoopPhysicalEdgePathV1>`. The sealed map
stores sorted path rows keyed by `(loop_key, role)`; exit roles may fan out.
Sealing requires:

- at least two blocks;
- first/last block equals the mapped logical edge ports;
- every adjacent pair is an actual physical CFG edge in the predecessor
  witness;
- `terminal_predecessor == blocks[len - 2]`;
- `Enter`, `Backedge`, and `Continue` have exactly one path;
- `PredicateFalse` and `Break` may fan out; when the logical edge is present,
  it must have at least one matching path (a role with no logical edge has no
  path row);
- no implicit `[from, to]` fallback when a path row is absent.

Header PHI input construction consumes the explicit path's terminal
predecessor. It never calls `map.port(edge.from)` for a backedge/continue row.

## Ordered implementation slices

### P1b-0 — test extraction and path type

Move the inline materializer tests to a child test file so the production
module has headroom. Add `LoopPhysicalEdgePathV1` and the `edge_paths` map field
without production callers or route wiring. Update existing direct test maps to
provide explicit two-block paths.

### P1b-1 — DirectAccum Standard5 witness

Add a distinct `accum_direct_v1.json` golden. Do not truncate or reinterpret the
nested golden. Seal a `P/H/B/S/A` witness with:

```text
Enter          [P,H]
PredicateTrue  [H,B]
PredicateFalse [H,A]
Backedge       [B,S,H]
header preds   [P,S]
after preds    [H]
break paths    []
```

The witness rejects `[B,H]`, wrong predecessor sets, and missing after rows
before any PHI effect.

### P1b-2 — recursive nested golden witness

Consume existing `accum_nested_v1.json` without claiming PlanLowerer parity:

- root has `Continue`, not `Backedge`;
- child is `Always` with `Enter + Break`;
- child owns no carrier and has no backedge;
- child `Enter` carries inherited `(i,value0),(sum,value3)`;
- child `Break` carries `(i,value0),(sum,value6)`;
- child `After` resumes the parent-body tail where `i` becomes `value5`, then
  root `Continue` carries `(i,value5),(sum,value6)` through parent resume →
  `Step -> Header`.

The parent-resume segment is explicit. Do not model child `After` as a direct
edge to parent `Step`; that shortcut belongs to a different legacy nested
composer shape.

### P1b-3 — materializer connection

Convert the sealed witness into the verified physical map and call the existing
M6-B materializer. Assert that:

- DirectAccum header PHI receipt uses `(P, init)` and `(S, next)`;
- nested golden produces root PHI sites only (child has no carrier PHI);
- malformed path/predecessor/after rows reject before PHI insertion;
- live candidate isolation still comes from P1a/M1-B, not a second transaction.

### P1b-4 — alpha-normalized parity

Define a test-only `AlphaNormalizedDigestV1` over role labels and semantic value
labels, never raw `BasicBlockId`, `ValueId`, route names, or allocation order.
The digest includes logical paths, predecessor sets, header PHI rows, after
rows, inherited child payloads, final forwarding, types, and result semantics.

Only DirectAccum may compare against the legacy PlanLowerer/CorePlan oracle.
The nested portable golden is JoinSig/path-witness/M6-B-root-receipt evidence
only; its legacy nested composer is not an equivalent semantic oracle.

## Gates and non-claims

- all production `materialize_loop_phis` callers remain zero;
- PlanLowerer imports are confined to `#[cfg(test)]` oracle code;
- no route, AST, Retry, Generic, or `route_loop` imports in the new path module;
- `phi_lifecycle` and Binding SSA remain sole PHI/SSA owners;
- all touched Rust files remain below 800 lines;
- no M10a/M10b cutover or `.hako` physicalizer claim is made.

If a path needs route-specific repair, AST reconstruction, or a second PHI
writer, stop P1b and reopen the design rather than adding a fallback.
