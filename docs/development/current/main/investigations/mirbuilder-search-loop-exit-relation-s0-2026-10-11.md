# MIRBUILDER-SEARCH-LOOP-EXIT-RELATION-S0

Status: closed bounded execution slice
Date: 2026-10-11
Scope: retain the original `SizeClassBox.size_to_bin` early I64 terminal source
Related: `mirbuilder-search-loop-exit-relation-d0-2026-10-11.md`

Replace the child terminal-relation filter's dropped selected search-loop
`I64Scalar` arm with a bounded join to the already verified Static I64 V2
product and ExactI64 result claim. Keep Home as the only terminal relation
issuer; do not create a new scalar classifier or widen generic child
retention. The existing tail `Call` arm and unrelated callers remain intact.

Acceptance:

- Unchanged source: early exit has the exact retained `I64Scalar`, tail has
  its existing `Call`, both Normal projections are present, and the two
  ready Home exits remain distinct.
- Changed early value/class/owner/site, changed loop/exit set, absent or
  conflicting result claim, and unrelated child scalar returns remain
  fail-closed. Extend the existing `ordinary_new_bin_size_words_tests`
  family only where a distinct boundary is otherwise untested.
- Original source physical probe advances beyond
  `ordinary-new/local-commit/root-exit-source-missing`; record its next
  named first stop. Existing all-exit and unsupported-loop regressions pass.
- `cargo check --profile quick -p nyash-rust --lib`, focused positive/negative
  and pointer guard pass. Classify any baseline reds and audit the source
  file line cap before closeout.

Non-claims: no generic I64 return promotion, Home change, whole SizeClassBox
or mimalloc-lite EXE pass.

## Closeout evidence

The original source's selected early exit now retains the Home-issued
`I64Scalar` and has a Normal projection; the tail retains its independent
`Call` and Normal projection. The existing positive/negative family passed
2/2, including wrong local and Bool return refusals
(`/tmp/hako-search-loop-exit-s0-final-focused-20261011.log`). Sibling
`bin_size` relation tests and existing unsupported-loop/all-exit tests each
passed 1/1 on the same test binary. `cargo check --profile quick -p
nyash-rust --lib`, `git diff --check` and focused rustfmt passed.

The unchanged source physical probe moved from
`ordinary-new/local-commit/root-exit-source-missing` to
`ordinary-new/local-commit/root-exit-unconsumed`
(`/tmp/hako-search-loop-exit-s0-physical-20261011.log`). Its disposable test
was removed. The latter is a new physical consumption frontier, not a
whole-source pass. `ordinary_new_coseal_issue.rs` remains below the 800-line
hard stop (796 lines).
