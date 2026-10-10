# MIRBUILDER-SEARCH-LOOP-EARLY-RETURN-HOME-S0

Status: selected bounded execution slice
Date: 2026-10-11
Scope: source-backed Home rows for both returns of `SizeClassBox.size_to_bin`
Related: `mirbuilder-search-loop-early-return-home-d0-2026-10-11.md`

Implement the accepted D0. Reuse the resolver's exact source sites,
existing Static I64 source loans, and closed transitive Home-neutral proof.
Factor the existing Static I64 V2 topology checks into one pre-Home role
issuer, then lend that exact role to the Home scanner and later Static V2
producer. Do not use physical LoopCond success or a generic CFG walk as
source authority.

At the Home Loop arm, accept only the sealed search-loop role. Observe the
header and body calls through existing claim walkers, the branch's early
`return bin` through existing terminal handling, and the proven I64 step.
Continue after the loop to the existing tail return handler. Keep per-exit
covered call sites and all-exit readiness; preserve existing Fault and
cleanup rules and refusal for unrelated loops.

Acceptance:

- Focused positive: unchanged `size_to_bin` has ready early and tail Home
  exits, each with its own expected covered call sites.
- Focused negative: reject altered binding/step/condition, extra exit or
  break/continue, missing or wrong-class Static claim, Home-changing callee,
  and mismatched owner/site. Reuse existing tests where they cover the same
  independent condition.
- Original SizeClassBox physical probe passes the current
  `IncompleteOrdinaryNewCoverage` first stop and records the next named
  frontier. A later physical failure is not S0 success for the whole source.
- Existing all-exit readiness and unrelated-loop refusal remain green.

Non-claims: no generic search-loop support, whole SizeClassBox pass,
or mimalloc-lite EXE pass. Static V2 physical publication is a separate
possible frontier.
