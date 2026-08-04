---
Status: Design accepted; D2 implementation/parity green; reference closeout pending
Date: 2026-08-04
Parent: joinir-if-recipe-nested-one-level-d0-d2-execution-task-2026-08-04.md
Decision: D0 and D1 are green; choose one nested proof adapter consumed by the
  existing canonical lowerer before opening parity/abort D2
---

# Nested If production consumer — design stop

## Current evidence

The depth-one nested profile is a valid Builder-free semantic product, but it
is intentionally disconnected:

```text
AnalyzerV1
  -> existing TrivialIfRecipeFactsDraftV1::nested_candidate()
  -> VerifiedTrivialCanonicalOwnerV1 nested sidecar
  -> nested mapper / verifier / JoinSig composer (tests only)
```

The existing production lowerer still sees `recipe_facts = None` for this
shape and uses its canonical recursive If materialization path. That path can
already produce two PHIs using the existing `CanonicalCfgSessionV1`, Binding
SSA, and `PhiTxn`, but it does not consume `NestedIfRecipeArtifactV1` or
`VerifiedNestedIfJoinSigV1`.

Therefore the current nested runtime tests are physical-oracle evidence only;
they are not Recipe-to-MIR parity evidence. D2 must not claim otherwise.

## Accepted decision

Adopt one production boundary for the nested artifact:

```text
NestedIfPhysicalAdapterV1 (one producer/adapter caller)
  -> existing CanonicalTrivialSsaLowererV1 / If materialization owner
  -> existing CanonicalCfgSessionV1 + Binding SSA + PhiTxn
```

The adapter packages the two node sites, shared binding, and composition
witness as a one-shot proof receipt. It may not emit blocks, allocate physical
IDs, or write PHI inputs. The existing lowerer remains the only physicalizer;
its existing recursive `CanonicalCfgSessionV1`/Binding SSA/`PhiTxn` path emits
the nodes after the proof is admitted. A new nested physicalizer, transaction,
retry route, or detached Builder is forbidden.

The D0 sidecar is intentional: the analyzer already owns one same-pass facts
draft and the nested product is an additional sealed slot, not a second whole
owner or a second source scan. Do not split `VerifiedTrivialCanonicalOwnerV1`
into two competing owner products merely to make the type graph look
independent.

## Candidate designs

### A — nested proof admission into the existing canonical lowerer (accepted)

Add a nested admission disposition that carries exactly two verified node
sites. `CanonicalTrivialSsaLowererV1::lower_if` consumes the outer proof and
the recursive child proof in source order. The existing
`lower_if_materialization_core` emits both CFG nodes and the adapter checks the
sealed composition and physical receipt after each node. The proof is the
route/shape authority; `if_control` and the canonical session remain the only
physical layout/value authorities.

Benefits:

* one production physicalizer and one CFG/SSA/PHI owner;
* existing candidate session and draft-seal abort are reused unchanged;
* portable nested artifact remains the pre-effect route proof;
* no fake second `IfPhysicalDemandV1` is manufactured from a child node.

Required proof:

* each node is consumed exactly once;
* inner merge is the outer `then` predecessor/value;
* no raw AST/name lookup is added after admission;
* old one-If admission remains immutable and still rejects nested input.

### B — keep the nested artifact as a parity oracle only

This is safe for D0 but does not advance the production semantic boundary. It
may be used as a temporary diagnostic comparison, never as a completion claim.
If selected, the task must be renamed to a physical-oracle census and the
Recipe-to-MIR D2 row remains open.

### Rejected

* a second nested physicalizer or a second `CanonicalSsaFunctionSessionV2`;
* an independent Builder/candidate transaction for each nested node;
* widening the fixed one-If schema with `Child` fields;
* route retry when nested admission or physicalization fails;
* deriving nested ownership from runtime/physical PHI shape.

## D2 fixture correction

The D0 fixture uses `local x = 0`, so it deterministically takes the inner
then branch and produces only `1`. It cannot prove results `1/2/3` by itself.

After design acceptance, D2 must use three same-topology fixtures with
constant conditions (or a separately authorized parameter profile):

```text
inner then  -> 1
inner else  -> 2
outer else  -> 3
```

The constant-fixture option preserves the D0 profile boundary and avoids
silently widening the admitted parameter vocabulary. Each fixture must still
have one outer/inner explicit-else If, one shared i64 binding, no effects, and
one continuation read.

## D2 proof after acceptance

Only after A is accepted may the execution row open:

1. Recipe/JoinSig admission reaches the sole canonical lowerer through the
   nested proof adapter.
2. Three outcomes match interpreter output and the sealed two-node JoinSig.
3. MIR has exactly two relevant PHIs; the inner merge value is the outer-then
   predecessor/value input.
4. Existing `lower_resolved_trivial_function_draft_with_seal_failure_for_test`
   injects failure after both PHIs, without a new fault API.
5. Candidate fingerprint, live module/function state, and entry state remain
   unchanged; the same compiler successfully compiles a fresh request.

## Stop conditions

Return to design if the adapter needs a second physicalizer, a new CFG/SSA/PHI
owner, a new transaction/fault seam, depth greater than one, calls/effects/
returns/multiple bindings, parameterized conditions without a new profile, raw
lookup, retry/fallback, or any touched source/test file over 800 lines.

This design stop does not change grammar, JSON v0, ownership/Home, or the
normative reference pages. The reference closeout remains parked until the
production consumer and D2 gates are green.
