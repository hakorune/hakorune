# MIRBUILDER-SEARCH-LOOP-EARLY-RETURN-HOME-S0

Status: closed bounded execution slice
Date: 2026-10-11
Scope: source-backed Home rows for both returns of `SizeClassBox.size_to_bin`
Related: `mirbuilder-search-loop-early-return-home-d0-2026-10-11.md`

Implement the accepted D0. Reuse the resolver's exact source sites,
existing Static I64 source loans, and closed transitive Home-neutral proof.
Issue the existing verified Static I64 V2 semantic product once using the
base Completion before its Home scan. Home borrows only source identity from
the product; later Static V2 lowering consumes that same product. Keep its
existing Home-neutral and tagged-entry corroborations. Reject any exit set
other than exactly the early and tail return, with no implicit exit. Do not
use physical LoopCond success or a generic CFG walk as source authority.

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

## Current WIP (not closeout)

The selected V2 product is now issued before the Home walk and lent to Home
as source-only identity. The original SizeClassBox physical probe moved from
`IncompleteOrdinaryNewCoverage` to
`[freeze:contract][callable-loop/static-packet-source-missing]`
(`/tmp/hako-search-loop-home-s0-selected-20261011.log`). This is only an
outer first-stop observation: a separate per-exit Home assertion found the
early return ready but the tail return `PrefixNotCovered(Body(2))`. The early
row covered the entry and header calls, but the body predicate's
`me.bin_size(bin)` call was missing from Home observations
(`/tmp/hako-search-loop-home-s0-exits-diag-20261011.log`).

The selected Home walker now uses the existing direct-value Static call
issuer on that exact product-proved body predicate before forking the If.
Focused positive `size_to_bin_search_loop_keeps_distinct_early_and_tail_home_calls`
passes: both exits are ready; early coverage is entry/header/body and tail
coverage adds the tail call. The same filter includes the parameterized
negative for a changed comparison or I64 step; both cases reject the V2
product and leave Home unready (2/2, `/tmp/hako-search-loop-home-s0-posneg-20261011.log`).
The one-off physical probe was removed after recording its result.

The `bin_size_` regression filter ran 11 tests: 10 green, one
`real_bin_size_nested_mul_proves_only_subtract_initializer_home` red at
`ordinary-new/static-scalar/incoming-identity`. The immediately preceding
test binary gives the same error for that exact test
(`/tmp/hako-search-loop-home-s0-bin-size-regression-20261011.log`,
`/tmp/hako-search-loop-home-s0-bin-size-prior-binary-20261011.log`), so this is
baseline debt, not a new failure. The original app-based Static Loop tests
also retain independently confirmed baseline `source-actual-drift` and are
not acceptance. After formatting, `cargo check --profile quick -p nyash-rust
--lib` passed (`/tmp/hako-search-loop-home-s0-final-check-20261011.log`).

The remaining source/control refusals passed in the expanded focused family
(`2/2`, `/tmp/hako-search-loop-home-s0-final-family-20261011.log`). The
original physical first stop is **not** an independent next frontier: S0
preissue moved the selected entry, body and tagged loans into its product,
while the existing packet and body-scalar issuers still enumerated only the
old maps. Read-only D0 audit identified both consumers. The bounded repair
lends those same original loans from the preissued product to the existing
issuers; it does not clone affine loans or introduce another packet authority.
The packet and body-scalar issuers keep their own corroboration and downstream
physical checks. S0 remains open until focused tests and the unchanged source
physical probe verify this transport, followed by the selected guard and
source/docs diff audit.

The unchanged source physical probe now moves past packet issuance and stops
at `[freeze:contract][ordinary-new/local-commit/root-exit-source-missing]`
(`/tmp/hako-search-loop-home-s0-transport-physical-20261011.log`, quick test
binary built from this checkout; original `size_class_box.hako` plus a trivial
Main). The disposable probe was removed. The next stop is in the Normal exit
projection, which requires a retained terminal relation for every ready
exit; this is a separate result-source contract to audit after S0 closeout.
The final focused test now pins the retained packet's original `Rc` identity
and the body-scalar source alongside the two distinct ready Home exits; the
negative matrix remains fail-closed (2/2,
`/tmp/hako-search-loop-home-s0-final-transport-family-20261011.log`). Existing
unsupported-loop and all-exit regression tests passed 1/1 each on that same
binary. `cargo check --profile quick -p nyash-rust --lib` passed; pointer
guard and `git diff --check` passed. The earlier `bin_size_` red is reproduced
on the pre-change test binary and remains classified baseline debt. S0 closes
with a physical first-stop advance, not a whole-source or EXE pass.
