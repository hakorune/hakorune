---
Status: accepted caller-zero mechanical observer / production-retirement target
Date: 2026-08-03
Decision: JOINIR-LOOP-CFG-JOINSIG-PHI0-D0-S4
Related:
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/binding-ssa-first-control-lowering-ssot.md
  - src/mir/loop_recipe_contract/join_sig.rs
---

# M6-B Loop PHI Materializer Design Stop

## Decision

M6-B adds one builder-side, caller-zero mechanical observer for the bounded
Accum JoinSig. It does not add a PHI/SSA lifecycle authority and does not
connect the production Loop route. Its explicit physical map/receipt is
parity evidence only; it is not the production physicalizer.

```text
VerifiedLoopJoinSigV1
  + VerifiedLoopLogicalToPhysicalMapV1
  -> exact PHI preflight
  -> PhiTxn::define_provisional_phi
  -> PhiTxn::patch_phi_inputs
  -> PhiTxn::commit
  -> non-Clone materialization receipt
```

The map is sealed before mutation and owns explicit loop-port blocks,
logical-value-to-`ValueId` rows, PHI destinations, expected predecessor rows,
and value classes. The materializer consumes only the verified JoinSig and this
map. It does not infer missing physical facts.

## Authority boundaries

| Owner | Owns | Must not own |
| --- | --- | --- |
| `VerifiedLoopJoinSigV1` | logical edges, visible carrier payloads, roles | physical IDs or MIR |
| sealed physical map | explicit block/value/dst/predecessor correspondence | AST, route, facts, repair |
| `LoopPhiMaterializerV1` | caller-zero exact PHI rows/receipt observer | production PHI/SSA authority, selection, CFG construction, `variable_map`, final values |
| `phi_lifecycle` / `PhiTxn` | MIR PHI insert, patch, rollback | Loop meaning or route choice |
| Binding SSA | binding exposure/reaching definitions | JoinSig carrier discovery |

The materializer must not call `define_phi_final*`, batch publication,
`ssa::phi_input_materializer`, `for_pred`, `update_cfg`, or a route-local PHI
writer. `define_phi_final*` and batch publication may rematerialize inputs;
M6-B uses the narrow provisional/patch transaction only.

## Initial scope

- one Accum root with predicate header and `Always` nested child;
- all PHI sites are explicit in the sealed map;
- no production caller, `Option`, retry, fallback, or Generic/D2 input;
- no `BindingSsaBuilder::read/seal` invocation: Binding SSA remains the
  exposure owner and the materializer only verifies mechanical MIR facts;
- no CFG recomputation or AST/source reread.

Nested predicate loops, wider branch closure, final-value publication, and
candidate physicalization are later M6/M7/M5 tasks.

## Required gates

1. Missing/duplicate map entries, class mismatch, and missing/phantom/duplicate
   predecessor rows reject before any Builder mutation.
2. Incoming values must be defined, reachable, and dominating their predecessor;
   the sealed predecessor witness is not recomputed.
3. A failure after the first provisional PHI leaves no empty PHI or type fact;
   `PhiTxn::abort_on_err` is the only cleanup path.
4. Success returns exact sorted PHI rows, leaves `variable_map` and Binding SSA
   untouched, and can be repeated on a fresh candidate.
5. Static guards keep production callers at zero, forbid Generic/D2/route/AST/
   CorePlan/Retry/legacy-repair imports, and allow only `phi_lifecycle` PHI calls.

M6-B is complete only when these gates are green and the receipt is non-Clone.
The later Accum vertical pilot must use canonical CFG plus one
function-owned `BindingSsaBuilderV1` and shared `PhiTxn`; it must not call this
observer. M10a remains the first production bridge, and M6-B stays caller-zero
or is retired before that bridge.
