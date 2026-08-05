Status: closed docs/test-only source projection closeout
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Predecessor: joinir-generic-resolved-carrier-source-site-totality-census-d3-s2-p0-task-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-PROJECTION0-D3-S2-P1`
Decision: accepted as a packaging closeout; production remains closed

# Change

Reuse and close the existing resolver/projector/source-bridge evidence for the
natural parsed Generic source shape. Project only resolver-owned source forest,
owner brand, strict-ancestor relation, and exact `BindingRefV1` role rows into
one passive non-`Clone` source observation. Do not copy current Generic V0/V1
AST/name facts into neutral authority.

# Contract

```text
resolver source session
  -> existing source projector/bridge evidence
  -> one source-site totality witness
```

The witness must cover the exact parsed outer/inner sites and distinguish
foreign owner, shadowed binding, incomplete forest, duplicate role, and frame
mismatch before any Builder effect. `LoopBindingKeyV1` remains unissued here;
the Recipe producer is its sole issuer. Binding SSA remains the physical
`BindingRefV1 -> ValueId/PHI` owner. No selector, canonical plan, Return,
outer-PHI, debt, Home, Recipe, MIR, runtime, retry, or fallback meaning moves.

# Acceptance

- Existing D3-S2 resolver/bridge evidence is reused; no second resolver or
  neutral source issuer is added.
- One machine-readable witness records source site, owner brand, role,
  `BindingRefV1`, and strict ancestor for every admitted row.
- All mismatch rows reject before Builder effects with stable typed reasons.
- The same implementation/test commit updates the affected `docs/reference/**`
  and current support pages; production caller remains zero.
- Source/check files remain below 800 lines.

# Done

The existing non-`Clone` resolver provenance product and projector/source
bridge are the sole evidence owners. The machine-readable witness
`joinir-generic-resolved-carrier-source-projection-d3-s2-p1-matrix-2026-08-05.tsv`
records the admitted natural/nested-If role rows with exact source paths,
resolver owner brand, `BindingRefV1` relation, and strict-ancestor result. It
also records the foreign-owner, mixed-session, shadowing, incomplete-forest,
duplicate/unsupported-role, frame, and foreign-root typed rejects; all are
pre-effect observations. No Generic facts, selector, Recipe, PHI, Builder,
MIR, Return/Home/debt, retry, fallback, or production caller moved.

The reference surface now points to the witness and preserves its
inspection-only status. The selected next row is
`JOINIR-GENERIC-RESOLVED-CARRIER-FACTS-SNAPSHOT0-D3-S2-P2`.

# Stop

Return to the D3-S2 design card if any role is name-only, AST-only, detached
from a resolver owner, or cannot be mapped to an exact source site. Do not
invent a Generic key, seed, seal, selector input, Binding SSA bridge, Return
projection, natural debt witness, or generic/composite capability.
