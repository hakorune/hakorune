---
Status: accepted__MirBuilderFinalAcceptanceScope__2026-09-11
Date: 2026-09-11
Decision: MIRBUILDER-FINAL-ACCEPTANCE-SCOPE-D0
Parent: mirbuilder-canonical-ssa-s6c-state-boxshape-d0-2026-09-11
---

# MIRBUILDER-FINAL-ACCEPTANCE-SCOPE-D0

Execution row: `MIRBUILDER-FINAL-ACCEPTANCE-SCOPE-R0`.

## Six-line brief

```text
Decision: execute the existing fixed real-app EXE acceptance manifest once
  and record its finite evidence without widening or rewriting the scope.
Source authority + canonical issuer: the integration suite manifest, runner,
  and each named existing smoke own source/backend/expected-result metadata.
Non-authority: raw test counts, synthetic MIR tests, changed fixtures, or a
  whole-library green inference from this suite.
Fail-fast boundary: preserve each smoke's existing failure and classify it as
  current-change, known baseline, or informational; do not retry another route.
Smallest next slice: run the exact 11-entry suite and record hashes/results in
  this card, with no source or fixture edits.
Non-claims: no production switch, language-v1 proof, Loop parity, selfhost,
  WASM, unselected backend parity, or whole-MirBuilder completion.
```

## Fixed scope

The only selected command is:

```text
tools/smokes/v2/run.sh --profile integration --owner-profile integration \
  --suite real-apps-exe-boundary
```

The manifest is
`tools/smokes/v2/suites/integration/real-apps-exe-boundary.txt`. It selects the
five typed-object probes, `boxtorrent-mini`, `binary-trees`, `mimalloc-lite`,
`allocator-stress`, the explicit unsupported-boundary probe, and
`json-stream-aggregator`, for eleven finite entries. Existing runner, backend,
toolchain, and expected-result declarations remain authoritative.

## Evidence boundary

Record the repository HEAD, manifest/script/source SHA-256 values, observed
result, and each failure's existing owner. Keep known baseline failures separate
from current-change failures. Do not change an accepted source into a reject,
add a fixture, create a parallel ledger/guard, or rerun through another backend.
The result is a finite acceptance handoff to convergence, not a production
completion claim.

## R0 execution receipt

Pending the exact manifest run. No code, fixture, or backend change is selected
by this card.
