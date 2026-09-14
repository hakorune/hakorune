---
Status: closed__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-I0-A2-PACKAGE-ISSUER-SPLIT
Date: 2026-09-14
Parent: mir-call-static-compatibility-a2-body-call-constructor-d0-2026-09-14.md
NextCard: MIR-CALL-STATIC-COMPATIBILITY-A2-BODY-CALL-CONSTRUCTOR-D1
Implementation permission: true for BoxShape-only package issuer split; no semantic handoff change
---

# A2 package issuer split I0

## Six-line brief

```text
Decision: move the existing direct-call validation helper out of the over-limit package issuer without changing its inputs, outputs, order, or reject mapping.
Source authority + canonical issuer: normal_callable_semantic_package/issuer.rs remains the sole package issuer; the new sibling is a private helper owner, not a semantic issuer.
Non-authority: no new body/call/constructor receipt, target resolver, parser source, MIR, publication, fallback, or caller switch.
Fail-fast boundary: preserve every existing package issue and install terminal exactly; only module placement changes.
Smallest next slice: extract direct-call co-seal validation into one private sibling and keep the package orchestration call unchanged.
Non-claims: no MixedProgram admission, body-shape expansion, constructor/generated coverage change, compatibility-edge deletion, StringBox work, Windows proof, or R7 closure.
```

## Owner and delete set

- `src/mir/normal_callable_semantic_package/issuer.rs` remains the only
  production package issuer and caller.
- Add one private sibling under the same module family for the existing
  `validate_cataloged_source_co_seal_v1` helper and its imports only.
- Keep `AppMainDirectCallDispositionIssueV1`, package issue enums, and all
  semantic sequencing in their current owner unless the extraction requires a
  private type import; do not create a second public/canonical route.
- No production old edge is deleted. This is a BoxShape refactor, not the A2
  body/direct-call semantic handoff.

## Acceptance

1. The helper moves without changing its signature, call site, iteration order,
   error variants, or target/header checks.
2. `issuer.rs` and the new sibling remain below 760 lines; no compression is
   used to hide the boundary.
3. Focused package tests are run without a new CI lane. The two issuance-order
   tests that exercise the unchanged direct-call helper pass. The existing
   `ordinary_batch_preflight_checks_candidates_before_that_owners_completion`
   fixture remains a baseline red: it panics in the test helper's root-execution
   setup with `SourceAuthorityUnavailable(PostpassNotSourceBacked)` before the
   extracted helper is reached. The split diff contains no semantic change, so
   this red is recorded for a separate recovery row rather than attributed to
   the BoxShape move.
4. `git diff --check` and `bash tools/checks/current_state_pointer_guard.sh`
   pass, and the card records the exact test commands and warning classification.

## Explicit non-claims

The A2-D0 design remains the authority for the next semantic slice. This I0
does not admit MixedProgram, widen body shapes, alter constructor/generated
coverage, change direct-call meaning, switch production callers, remove the
generic compatibility terminal, restore fallback, fix StringBox readers, or
close Windows/R7 evidence.

## Implementation receipt — 2026-09-14

`validate_cataloged_source_co_seal_v1` moved to the private
`direct_call_co_seal.rs` sibling. `issuer.rs` is 632 lines and the sibling is
154 lines; the package issuer remains the sole caller and issuer. Exact tests
`dynamic_owner_error_precedes_ordinary_completion_error` and
`dynamic_lends_its_original_completion_without_owned_result_row` pass. The
ordinary preflight red above is retained as a classified baseline and is not a
claim of this refactor.
