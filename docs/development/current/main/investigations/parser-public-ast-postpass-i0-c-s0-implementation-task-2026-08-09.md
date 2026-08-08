---
Status: closed implementation receipt
Date: 2026-08-09
Decision: implement parser-private BuildGate inventory and decision-set issuer only
Parent: `parser-public-ast-postpass-i0-c-design-task-2026-08-09.md`
---

# PARSER-PUBLIC-AST-POSTPASS-I0-C-S0

## Scope

Implement the first bounded I0-C slice:

```text
parser-issued postpass-visible AST gate observations
  -> PreparedBuildGateDecisionSetV1
```

This row is Builder-free and does not switch prune, explain, source-path
rebase, or any public parser caller. It proves that one parser-private issuer
can align every AST `BuildGate` with parser-issued observations, evaluate each
predicate once, and preserve the exact relation needed by the later projection
row.

## Authority

The parser owns the observation inventory and invocation brand. The decision
issuer is the only I0-C predicate evaluator. AST traversal is an alignment
check against parser-issued rows, not a source/name/ordinal identity rebuild.
Top-level `SourceBuildGatePathV1` relations remain the narrower source-path
authority. Member-level gates merged during Box parsing are out of scope.

## Required product

Add a dedicated parser module (for example
`src/parser/build_cfg/decision_set.rs`) with a private, non-Clone
`PreparedBuildGateDecisionSetV1` and typed rows containing:

```text
invocation brand
parser-issued gate id
private structural coordinate/context
exact BuildPredicate and source span
selected branch: Then / Else / NoElse
reachability: Reachable / InactiveSubtree
optional top-level source-path relation
complete row coverage
```

The issuer must consume the parser-owned observation inventory and immutable
build configuration once. `eval_build_predicate` must not be called by any
consumer after the issuer returns. The product is parser postpass authority,
not a resolver `Verified*`, source seal, Recipe, or physical plan.

## Semantics and failures

All postpass-visible AST predicates are evaluated exactly once, including
inactive subtrees. Unknown features and unsupported predicate keys are
fail-fast `ParseError::BuildCfg` diagnostics. Missing, duplicate, foreign, or
mismatched observation/source rows reject the unpublished product. No
fallback, retry, reparse, AST/name rescan, or member-gate signature change is
allowed.

Explain counters are not changed in S0. The later projection row will count
reachable rows while retaining inactive rows for coverage and diagnostics.

## Focused evidence

Add Builder-free positive/negative tests for:

```text
top-level selected/else gate
nested top-level gate
statement-level gate in an AST container
no-else gate
Build / Feature / Target / Backend / not / all / any
unknown feature in active and inactive subtree
missing/duplicate/foreign/predicate-mismatched observation
complete versus incomplete coverage
```

Use the existing parser BuildGate tests as behavior evidence; do not switch
the public explain route or alter the known nested member-gate baseline red.

## Non-claims

```text
no prune projection cutover
no source-path rebase cutover
no explain projection or public explain switch
no grammar-evidence demand change
no resolver/source seal/Recipe/Builder/MIR/runtime work
no member-level gate redesign
no fallback/retry/reparse
```

## Closeout

Implementation must update the parser README, postpass SSOT, build-conditional
language reference, task map, CURRENT_STATE, focused tests, and the
consolidated guard in the same commit. New source belongs in dedicated parser
modules; do not append the decision set to `src/parser/mod.rs` or
`src/parser/source_seal.rs`, both of which are already past the 760-line split
trigger. All touched source files must remain below 800 lines.

## S0 implementation receipt (2026-08-09)

Closed with the parser-private `build_cfg::decision_set` issuer and focused
Builder-free tests. The issuer consumes parser-issued observations, aligns
them with every postpass-visible AST `BuildGate`, validates nested predicate
configuration before evaluation, evaluates each top-level predicate once, and
records selected branch plus reachable/inactive-subtree status in a
non-Clone `PreparedBuildGateDecisionSetV1`.

Evidence:

```text
cargo check -q -p nyash-rust                              # pass
cargo test -q -p nyash-rust --lib i0_c                   # 7 passed
cargo test -q -p nyash-rust --test parser_build_cfg_gate # 12 passed
```

The existing prune/source-path/explain consumers still use their pre-I0-C
paths. Projection is deliberately the next design-stop row; no public
consumer cutover or production semantic change is claimed here. The known
nested member-gate source-path baseline remains parked separately.
