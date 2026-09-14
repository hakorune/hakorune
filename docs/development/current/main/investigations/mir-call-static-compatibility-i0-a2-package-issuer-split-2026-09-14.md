---
Status: selected__fast__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-I0-A2-PACKAGE-ISSUER-SPLIT
Date: 2026-09-14
Parent: mir-call-static-compatibility-a2-body-call-constructor-d0-2026-09-14.md
NextCard: none__a2_body_call_constructor_handoff_i0
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
3. Existing focused package tests for issuance order, direct-call lifecycle,
   selected handoff, and physical header remain green. Add no new CI lane.
4. `git diff --check` and `bash tools/checks/current_state_pointer_guard.sh`
   pass, and the card records the exact test commands and warning classification.

## Explicit non-claims

The A2-D0 design remains the authority for the next semantic slice. This I0
does not admit MixedProgram, widen body shapes, alter constructor/generated
coverage, change direct-call meaning, switch production callers, remove the
generic compatibility terminal, restore fallback, fix StringBox readers, or
close Windows/R7 evidence.
