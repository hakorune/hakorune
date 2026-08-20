---
Status: Closed fast row
Date: 2026-08-20
Decision: MIRBUILDER-CLEANUP-T2-S1-ANALYZER-MODE-I0
Parent: docs/development/current/main/investigations/mirbuilder-cleanup-retirement0-d0-task-map-2026-08-04.md
ProductionCaller: src/mir/compiler/capability.rs
ReplacementCell: behavior-neutral refactor
---

# MIRBUILDER-CLEANUP-T2-S1-ANALYZER-MODE-I0

## Six-line brief

Decision: Replace the four trivial-canonical policy wrappers with one neutral
mode vocabulary and one analyzer entry without changing any accepted source,
route, Recipe, SSA/PHI, or publication behavior.

Source authority + canonical issuer: the existing trivial analyzer remains the
sole policy consumer; `TrivialCanonicalAnalysisModeV1` only selects the already
closed ordinary/main and closed/finite-direct-call quadrants.

Non-authority: AST/name lookup, new semantic receipts, capability inference,
MIR/SSA shape, test-only facades, and any route-specific policy reclassification.

Fail-fast boundary: every old quadrant must map exactly once to the same
`DirectCallPolicyV1` and `RootProfilePolicyV1`; unknown mode, role drift, or
missing caller migration is a compile/guard failure, never a default quadrant.

Smallest next slice: add the neutral mode module, migrate capability and tests
to the single entry, remove the four wrapper symbols, update the profile guard,
and run the focused analyzer suite plus `cargo check --lib`.

Non-claims: no source-shape expansion, backend change, optimizer change,
Recipe/Join change, production route switch, JSON-v0 change, or performance claim.

## Acceptance

```text
four old entry definitions/callers                 = 0
one mode entry definition                           = 1
ordinary closed quadrant                             = unchanged
ordinary finite-direct-call quadrant                = unchanged
normal-main closed quadrant                         = unchanged
normal-main finite-direct-call quadrant             = unchanged
role/policy mapping                                 = exhaustive
route/Recipe/SSA/PHI diff                            = 0
analyzer source files >= 800 lines                  = 0
focused tests + analyzer-mode guard + cargo check --lib = green
```

The old entry names remain historical evidence only; no compatibility alias is
permitted after the migration. The analyzer mode is an internal policy value,
not a new language or MIR authority.

The existing V2 profile receipt remains a known baseline-red historical guard
(stale seam/manifest/caller census); this row uses the dedicated analyzer-mode
guard and does not alter that unrelated receipt.

## Closeout evidence

- `cargo test -q --manifest-path Cargo.toml --lib mir::resolved_value_profile`:
  46 passed, 0 failed.
- `CARGO_BUILD_JOBS=4 cargo check -q --lib`: passed. The existing warning
  inventory remains baseline noise; no new warning policy is claimed here.
- `tools/checks/trivial_canonical_analyzer_mode_guard.sh`: passed.
- `tools/checks/lib/resolved_callable_p0c_f.py .`: passed.
- `tools/checks/current_state_pointer_guard.sh` and `git diff --check`: passed.
- Changed Rust/check files remain below the 800-line hard stop; the analyzer
  and focused test owner remain below the 760-line split trigger.
- `normal_source_plan0_guard.py` and `resolved_binding_ssa_i1_t.py` still show
  their pre-existing module/manifest baseline reds; this row does not rewrite
  those unrelated receipts. The stale V2 profile guard remains historical and
  unchanged as recorded above.

## Closeout boundary

This closes one behavior-neutral analyzer cleanup row only. It does not claim
canonical Script transport, production cutover, raw retirement, or a
performance result. The next row must be selected explicitly after the parked
canonical-transport NoSafeSlice is revisited or another bounded cleanup row is
accepted.
