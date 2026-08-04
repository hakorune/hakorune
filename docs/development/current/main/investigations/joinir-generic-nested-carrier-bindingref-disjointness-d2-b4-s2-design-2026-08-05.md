---
Status: active design stop — implementation not started
Date: 2026-08-05
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Decision: accepted — `JOINIR-GENERIC-NESTED-CARRIER-BINDINGREF-DISJOINTNESS0-D2-B4-S2`
---

# Generic nested-carrier BindingRef disjointness — D2-B4-S2

## Boundary

D2-B4-S1 is useful corroborating evidence, but its `Vec<String>` labels and
post-effect V1 tags are not shadowing-safe semantic authority. Do not pursue a
standalone runtime-result oracle or V0/V1 winner equivalence. The next row is
one test-only disjointness witness keyed by resolver-issued `BindingRefV1`.

The source fixture must be an actual parsed outer-`j` program whose inner loop
writes `j` and whose outer loop later reads `j`. That post-loop read is folded
into this row as a resolved source site. Runtime execution is a later parity
gate, not the source authority for this design stop.

## Chosen authority

Use only products already owned by the resolver/source pipeline:

```text
actual parsed source
  -> VerifiedResolvedFunctionV1
  -> VerifiedResolvedLoopSourceForestV1 / exact loop-region sites
  -> resolver-issued assignment/read BindingRefV1s
  -> shared LivePreflightFrameV1
  -> frozen natural Both schedule [V0, V1]
  -> existing GenericLoopV1Facts.carrier_observation
```

The test-only witness may be issued only when the inner-loop assignment target
and the post-outer-loop read resolve to the same strict-ancestor binding,
under the same function/frame/source identity. B4-S1 labels, V1 carrier tags,
`LowerSome`, and the legacy V0 terminal remain corroborating observations;
names never pair binding identities.

## Product and acceptance

Issue one non-`Clone` test-only `VerifiedGenericNestedCarrierDisjointness`
row. It must record:

- exact parsed source and resolved loop-region sites;
- assignment target `BindingRefV1` and post-loop read `BindingRefV1`;
- strict-ancestor relationship and function/frame identity;
- release/strict natural `Both` frame and fresh-repeat digest;
- B4 V1 natural `LowerSome` plus candidate isolation;
- the V0 plan's absence of an equivalent outer-carrier binding.

The decisive negative is a shadowing fixture: an inner `local j` assignment
must resolve to a different `BindingRefV1` and must not issue disjointness.
Missing, foreign, ambiguous, owner/frame-mismatched, `NoRecursive`,
`Unavailable`, `Ambiguous`, planner-required `[V1]`, unstable-repeat, or
target-mismatch rows remain `UnresolvedStop`.

## Non-authority and non-claims

Do not re-read AST in policy or use names, route labels, plan digests,
`diagnostic_effective`, legacy receipts, or runtime terminal status as binding
semantics. Do not add production selector/policy arms, facts suppression,
Retry/fallback, Recipe/JoinSig/PHI/physicalizer callers, scheduler changes,
candidate publication, or Builder/MIR routes. Other Generic overlap classes,
planner-required suppression, M7-S4, M10a, and M10b remain unresolved.

## Implementation slice and closeout

Implement one `#[cfg(test)]` registry sibling and only the smallest
`pub(super)` projections from existing resolver/observer tests. Keep touched
Rust source and tests below 800 lines, and keep production caller/import
census at zero. Focused tests must cover the positive parsed source, the
shadowing negative, release/strict stability, candidate isolation, and all
typed unresolved dispositions.

Implementation is incomplete until these references are updated with exact
commands, evidence kind (BindingRef versus later runtime parity), line
budgets, and explicit non-claims:

```text
docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md
docs/reference/mir/generic-loop-stage-matrix.md
src/mir/builder/control_flow/plan/generic_loop/README.md
src/mir/resolved_semantics/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/10-Now.md
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
```

Even a green witness authorizes only a later design decision for this exact
BindingRef-proven class. The parent Generic D2 disposition and all other
overlap classes remain `UnresolvedStop`.
