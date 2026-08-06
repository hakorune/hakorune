# GENERIC-SELECTION-POLICY-HANDOFF-I0-R0

Status: taskized caller-zero implementation; production selection remains
closed.

## Objective

Implement one cfg(test)-only, move-only
`VerifiedGenericG0PolicyHandoffV1` from the natural typed G0 source projector.
The handoff must co-seal the resolver/source brand, existing typed G0 bundle,
candidate shape/body-effect/coverage proof, numeric target, and exact
post-loop return BindingRef relation in one issuer.

## Allowed slice

```text
ResolvedFunctionLoweringInputV1
  -> sole Generic G0 source projector / handoff issuer
  -> existing Generic G0 policy admission
  -> existing row normalization / Ready assembler / selector tests only
```

The first implementation may expose only a test helper and focused fixtures.
It must not add a production caller, alter the selector algebra, open demand,
Recipe/JoinSig, Builder/MIR, or delete legacy routes.

## Required product invariants

- non-`Clone` and AST/source-lifetime free after issuance;
- opaque source brand covers source unit/function, origin, source kind, root
  site, and loop frame;
- all condition/update/literal/tail roles and `PostLoopRead` return relation
  are issued internally with exact `BindingRef` provenance;
- typed G0 bundle and the old candidate-envelope proof are not separately
  re-paired by owner/site/name after issuance;
- policy context adds only mode/profile/coverage and is checked against the
  handoff brand;
- foreign identical-AST, shadowed binding, role-site mismatch, return-binding
  mismatch, incomplete coverage, and wrong target are typed rejects or
  unresolved outcomes before policy publication;
- no `ASTNode`, `FunctionSyntaxViewV1`, name lookup, retry, fallback, or
  `NoCandidate` appears in the resulting product.

## Acceptance evidence

Add focused positive/negative tests for the natural typed G0 fixture and the
counterexamples above. Keep every touched source/check file below 800 lines,
use the existing shared loop-family guard, and keep production caller census
at zero. Run the focused lease/G0 suite, `cargo check --lib --features
plugins`, current-state guard, shared MirBuilder guard, and `git diff --check`.

The same implementation commit must update the exact `docs/reference/**`
rows (without activating public language claims), Generic/loop SSOTs,
module READMEs, workstream, `CURRENT_STATE.toml`, and current mirrors. Commit
and push only the intended files; preserve unrelated worktree changes.

## Explicit non-goals

No policy/selector production promotion, Generic demand, portable Recipe,
JoinSig/Builder/MIR integration, backend parity, retry/fallback removal, or
legacy deletion. Those require later atomic rows after the handoff proof is
accepted.

