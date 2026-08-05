Status: selected design/test-only next row; production remains closed
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Predecessor: joinir-generic-resolved-carrier-source-projection-d3-s2-p1-task-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-FACTS-SNAPSHOT0-D3-S2-P2`
Decision: selected; no production caller

# Change

Package the closed P1 resolver observation into one neutral, AST-free,
non-`Clone` facts observation for the natural parsed source class. This is a
test-only contract task first. It must consume the P1 handoff as one sealed
value and classify only mode-neutral carrier facts; it must not become a
Generic selector input or a second source authority.

# Contract

```text
P1 ResolvedCarrierObservationV1
  -> one neutral VerifiedGenericResolvedCarrierFactsV1 candidate
```

The candidate may retain exact source-backed role membership and a structural
carrier disposition, but it must not retain AST nodes, copied body recipes,
diagnostic names, route IDs, `ValueId`, PHI, `LoopBindingKeyV1`, policy flags,
Return/outer-PHI, Home, debt, retry, fallback, or runtime meaning. The
resolver remains the sole source-site/owner/`BindingRefV1` authority; the
Recipe producer alone may later issue `LoopBindingKeyV1`; Binding SSA alone
owns physical `BindingRefV1 -> ValueId/PHI`.

# Acceptance

- One non-`Clone` test-only candidate consumes the P1 matrix witness as a
  single input; no loose role/site/forest pairing is accepted.
- Natural parsed Both is classified with a mode-neutral structural result.
- Shadowing, foreign/mixed owner, incomplete forest, duplicate/unsupported
  role, and frame mismatch reject before Builder effects with stable typed
  reasons.
- No current Generic V0/V1 AST/name fact is promoted into source authority;
  no selector, Recipe, PHI, Builder, MIR, Return/Home/debt, retry, fallback,
  or production caller changes.
- The same implementation/test commit updates affected `docs/reference/**`
  and current support pages; the reference update is mandatory after any
  implementation/test landing, not a deferred follow-up.
- Source/check files remain below 800 lines; fresh request after each reject
  remains valid.

# Stop

Return to the D3-S2 design card if neutral facts need AST reconstruction,
name/`ValueId` inference, a second source issuer, a Generic key/selector,
policy transport, loose co-sealing, synthetic debt, Return/PHI/Home meaning,
retry, fallback, or a production caller. Keep this row test-only until P3
disjointness has a separate design acceptance.
