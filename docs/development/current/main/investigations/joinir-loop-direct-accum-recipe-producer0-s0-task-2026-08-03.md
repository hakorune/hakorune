# JOINIR Direct Accum Recipe Producer S0

Status: Implementation task; caller-zero semantic producer.

Task: `JOINIR-LOOP-DIRECT-ACCUM-RECIPE-PRODUCER0-S0`

Depends on: `JOINIR-LOOP-DIRECT-ACCUM-RECIPE-PRODUCER0-D1`

## Scope

Implement one disconnected producer for the already-selected Direct Accum
demand. It consumes `VerifiedSelectedLoopRecipeDemandV1` once, issues one
portable `LoopRecipeArtifactV1`, verifies it, elaborates one `LoopJoinSigV1`,
and returns a sealed caller-zero product for tests.

The producer is not wired to `route_loop` or the legacy scheduler.

## Exact recipe contract

Use deterministic recipe-local identities; never sort or label by compiler
`BindingRefV1` values or source names.

```text
bindings: induction=0, accumulator=1
inputs:   v0, v1                         // pre-loop values
condition block:
  Const(bound)->v3, Read(induction)->v2, Less(v2,v3)->v4
body block:
  Read(accumulator)->v5, Const(update_delta)->v6,
  Add(v5,v6)->v7, Write(accumulator,v7),
  Read(induction)->v8, Const(step_delta)->v9,
  Add(v8,v9)->v10, Write(induction,v10)
carriers: induction(entry=v0), accumulator(entry=v1)
exits: none
```

All loop/block/item/value/binding/carrier keys are canonical contiguous
preorder keys. `LoopRecipeArtifactV1::provenance.producer_route` is diagnostic
only and does not affect the semantic recipe or JoinSig.

## Required code shape

- Add one neutral producer module under `src/mir/loop_recipe_contract/`.
- Add one consuming `into_direct_accum_v1()` facts accessor; no `Clone` escape.
- Use `bind_resolved_loop_root_v1` for source claims; no path construction.
- Call the existing `LoopRecipeVerifierV1::verify_artifact` boundary and then
  `LoopJoinSigElaboratorV1::elaborate`.
- Return typed producer/verifier/JoinSig rejection; never `Option`, `Retry`,
  route selection, fallback, or callback continuation.

## Explicit PHI/SSA boundary

No PHI/SSA code is added or changed. The later physical consumer must use:

```text
CanonicalCfgSessionV1
  -> BindingSsaBuilderV1
  -> PhiTxn / phi_lifecycle
```

No `LoopPhiMaterializerV1` production caller, Builder/CorePlan/PlanLowerer,
physical ID, block-argument MIR, or Loop-local transaction is allowed here.

## Tests and gates

1. Direct Accum demand produces a normalized artifact and successful JoinSig.
2. Different diagnostic route receipts preserve identical semantic digest.
3. Non-Direct-Accum payload, conflicting roles, unsupported source owner, and
   source-claim failure reject before artifact issue.
4. Verifier/JoinSig rejection is terminal and exposes no retry surface.
5. Production caller census remains zero.

```text
RUSTFLAGS='-Awarnings' cargo test -q 'mir::loop_structural_facts::' --lib
RUSTFLAGS='-Awarnings' cargo test -q 'mir::loop_recipe_contract::' --lib
bash tools/checks/lib/joinir_logical_demand_contract.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

All touched Rust files remain below 800 lines. A failing gate stops this
slice; do not wire production or broaden the shape.
