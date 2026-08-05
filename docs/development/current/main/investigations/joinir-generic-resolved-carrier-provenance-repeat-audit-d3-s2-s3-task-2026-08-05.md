Status: closed cfg(test)-only repeat audit; production remains closed
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-PROVENANCE-REPEAT-AUDIT0-D3-S2-S3`
Ceremony: T2 neutral passive repeat audit

# Change

Add one private, non-`Clone`, `cfg(test)` observer that consumes exactly one
pair of complete D3-S2-S2 provenance products from fresh resolver sessions A
and B. Compare source topology, sites, typed roles, binding relation, strict
ancestor, forest/frame coordinates, and resolver brands. The resolver remains
the sole owner/issuer; registry, facts, router, and Builder are not issuers.

# Contract

The pair is the only input: no loose forest/frame/role/AST/facts arguments.
The opaque witness retains only structural equality, distinct resolver brands,
and raw frame-coordinate equality as a non-identity observation. Equal raw
coordinates with distinct brands are valid evidence. Reused/equal brands,
source/site/topology/role/binding/ancestor/frame mismatches, mixed/foreign
brands, and missing/detached products reject before effects with no fallback
or retry. No Generic snapshot/key/seed, selector, `InvocationSeal`,
Builder/MIR/Recipe/PHI, Return/ABI/Home/debt, DirectAccum frame, or production
authority is introduced.

# Done

Fresh A+B products produce one opaque witness; the focused suite proves the
positive repeat and a typed pre-effect mismatch. The focused provenance suite
is green at 12/12, production issuer/caller/artifact remains zero, source/check
files remain below 800 lines, and pointer guard plus diff check are green. The
implementation commit updates the resolved-semantics README, this receipt,
the parent design card, `CURRENT_STATE.toml`, `10-Now.md`, and the MirBuilder
workstream together.

# Stop

Return to the parent D3-S2 design card if the row needs a second issuer, loose
components, AST rescan, pairing heuristic, raw `FrameKey` changes, or any
Generic/selection/production meaning. Later implementation cells must update
the exact reference documentation in the same commit and close the matching
reference receipt.
