---
Status: closed checkpoint — test-only witness green; scoped D3 design stop follows
Date: 2026-08-05
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Decision: accepted — `JOINIR-GENERIC-NESTED-CARRIER-BINDINGREF-DISJOINTNESS0-D2-B4-S2`
---

# Generic nested-carrier BindingRef disjointness — D2-B4-S2

## Execution checkpoint — 2026-08-05

The bounded `#[cfg(test)]` registry witness has been started in:

```text
src/mir/builder/control_flow/joinir/route_entry/registry/
  generic_nested_carrier_bindingref_tests.rs
```

The focused command was:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4_s2 -- --nocapture
```

Result: 3/3 tests passed. The parsed-source positive and shadowing-negative
BindingRef checks are green in Release/Strict coverage. The planner-required
row is separately recorded as `SuppressedByPlannerRequired` for V0 and remains
`UnresolvedStop`; no V0 composer is called in that mode. The raw schedule is
captured under the same mode-scoped configuration (`[V1]` for planner-required,
`[V0, V1]` for Release/Strict).

The direct-stage observation now consumes the same parsed condition/body and
canonical facts path as the witness. The legacy B4-S1 V0/V1 final-value/PHI
projection remains explicitly corroborating evidence only; it never issues the
BindingRef decision. No production selector, retry, fallback, planner policy,
Recipe/JoinSig/PHI/physicalizer caller, Builder, MIR, or runtime route changed.

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

## Evidence closeout — 2026-08-05

The focused row is now green, but this document remains a design stop rather
than an implementation promotion. Evidence kind is resolver/source identity
(`BindingRefV1` plus loop-source forest/frame) and canonical Generic facts;
runtime-result parity is explicitly later work. Touched Rust source and the
test sibling remain below the 800-line budget. The production caller/import
census remains zero.

The planner-required V0-facts absence is an existing typed contract boundary:
the extractor suppresses V0 under strict planner-required mode and the V0
composer must retain its fail-fast contract freeze when called without facts.
The test records this as typed suppression instead of treating it as a false
carrier or a fixture failure. Synthetic `both_body()`/legacy carrier tags are
not semantic authority and are not used by the evaluator.

Neighboring gates were also green under the same clean environment:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4 -- --nocapture
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_stage_observer -- --nocapture
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_both_semantic_parity_matrix_is_fresh_and_explicit -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The observed counts were 8, 9, 1, and successful pointer/diff checks.

Reference closeout for this checkpoint is recorded in the Generic post-effect
SSOT, the Generic stage-matrix reference, the Generic loop README, the
resolved-semantics README, `CURRENT_STATE.toml`, `10-Now.md`, and the MIRBuilder
workstream. The next scoped D3 design stop is
`investigations/joinir-generic-nested-carrier-d3-bindingref-design-2026-08-05.md`.
A green test does not authorize parent Generic D2 resolution, production
selection, or old-route retirement.
