# JOINIR Direct Accum Recipe Producer D1

Status: Design stop; caller-zero boundary for worker review.

Task: `JOINIR-LOOP-DIRECT-ACCUM-RECIPE-PRODUCER0-D1`

Depends on: `JOINIR-LOOP-DIRECT-ACCUM-STRUCTURAL-PROJECTION0-S0`

## Purpose

Define the smallest production-independent producer that consumes one sealed
`VerifiedSelectedLoopRecipeDemandV1` and emits one portable
`LoopRecipeArtifactV1` for the Direct Accum shape.

This is a semantic producer design stop. It must not activate `route_loop`, the
legacy scheduler, a Builder, or physical PHI/SSA production.

## Existing authority chain

```text
FunctionSourceViewV1 / VerifiedResolvedFunctionV1
  -> VerifiedLoopStructuralFactsV1 (DirectAccum payload)
  -> VerifiedLoopPolicyWinnerV1
  -> VerifiedSelectedLoopRecipeDemandV1  // one-shot handoff
  -> DirectAccum Recipe producer         // this task
  -> LoopRecipeVerifierV1
  -> LoopJoinSigElaboratorV1
  -> existing canonical physical chain (later)
```

The producer consumes the selected demand exactly once. It must not re-run
policy, inspect AST, reconstruct names, rescan source, or try another route.
`VerifiedResolvedLoopSourceV1` is consumed only to create the existing
portable source claim (`bind_resolved_loop_root_v1` + recipe root claim).

## Decisions to fix before implementation

### 1. Facts payload handoff

`VerifiedLoopStructuralFactsV1` needs one consuming, typed accessor for the
Direct Accum payload (for example `into_direct_accum_v1`). The accessor must
consume the sealed facts and return the owned structural shape; it must not
expose `Clone`, AST, route IDs, or a second facts issuer. Non-Direct-Accum
payloads reject before recipe construction.

### 2. Recipe-local key mapping

`BindingRefV1` and source sites are compiler identities, not portable recipe
keys. The producer must create deterministic recipe-local keys from the fixed
Direct Accum shape order:

```text
induction binding -> accumulator binding -> condition value
  -> update reads/constant -> step reads/constant
```

The mapping must be owned by this producer, use no name lookup, and reject
duplicate/conflicting binding roles. Labels in `LoopRecipeBindingV1` are
diagnostic-only and must not be used as semantic identity. The mapping must be
documented so the Rust and `.hako` producers can normalize the same result.

### 3. Direct Accum recipe shape

The first recipe is intentionally narrow and deterministic:

- one root Loop with a `Less` predicate;
- one condition block that reads induction and emits the bound constant;
- two ordered body operations: accumulator `Add` write, induction `Add` step;
- explicit carrier rows for the induction and accumulator bindings;
- no nested Loop, If, Exit, call, record, match, or opaque operation;
- source binding is issued through the existing resolved-source adapter.

Any shape outside this exact contract rejects before verifier/JoinSig. It does
not fall back to `Generic` or the legacy scheduler.

### 4. Verifier and JoinSig boundary

The producer returns an owned artifact. `LoopRecipeVerifierV1` remains the
only structural recipe verifier; `LoopJoinSigElaboratorV1` remains the only
logical edge/dataflow elaborator. A JoinSig reject is a terminal typed error,
not `Option`, `Retry`, or route continuation.

## PHI/SSA boundary (explicit)

PHI/SSA is already SSOTed and is not redesigned here:

```text
CanonicalCfgSessionV1
  -> function-owned BindingSsaBuilderV1
  -> PhiTxn / phi_lifecycle
```

`LoopJoinSigV1` is logical dataflow only. `LoopPhiMaterializerV1` remains the
caller-zero mechanical M6-B observer until the later physicalization task. No
new PHI writer, carrier map, Binding SSA builder, block-argument MIR, or
Loop-local transaction may be added in this slice.

## Required caller-zero tests

1. Direct Accum demand is consumed once and emits a normalized artifact whose
   verifier and JoinSig both succeed.
2. The same semantic shape with different diagnostic route receipts produces
   identical semantic normalization and JoinSig.
3. Wrong binding role, duplicate binding mapping, non-Direct-Accum payload,
   and unsupported source owner reject before artifact issue.
4. Verifier or JoinSig rejection is terminal and does not expose `Option`,
   `Retry`, or a next-route callback.
5. Source claim is issued by `bind_resolved_loop_root_v1`; no hand-written path
   or AST reconstruction is accepted.
6. Production callers remain zero: no `route_loop`, legacy scheduler,
   `LoopPhysicalizerV1`, `LoopPhiMaterializerV1`, Builder, PHI, or SSA caller.

## Acceptance gates

```text
RUSTFLAGS='-Awarnings' cargo test -q 'mir::loop_structural_facts::' --lib
RUSTFLAGS='-Awarnings' cargo test -q 'mir::loop_recipe_contract::' --lib
bash tools/checks/lib/joinir_logical_demand_contract.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

All touched Rust files remain below 800 lines. This card is design-only until
the key-mapping and one-shot payload decisions are reviewed; do not wire the
producer into production while this stop is active.

## Non-goals

- no PHI/SSA redesign or second materializer;
- no route-policy or Generic-debt classification;
- no production `route_loop` cutover;
- no all-family adapter, Nested, LoopCond, or call/record/match expansion;
- no AST rewrite, source-name authority, or physical MIR IDs in the artifact.
