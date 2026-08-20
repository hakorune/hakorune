---
Status: Active fast row
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
focused tests + profile guard + cargo check --lib   = green
```

The old entry names remain historical evidence only; no compatibility alias is
permitted after the migration. The analyzer mode is an internal policy value,
not a new language or MIR authority.
