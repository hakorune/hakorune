---
Status: closed__DecisionAccepted__R7BoundaryLlcFlags__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-BOUNDARY-LLC-FLAGS-D0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-m7s-owner-matrix-d0-2026-09-12.md
---

# R7 Boundary llc-flags policy

## Six-line brief

```text
Decision: choose one authoritative meaning for Boundary profile1 llc flags across unset, empty, and explicit values before implementation.
Source authority + canonical issuer: the existing Boundary Rust request/options admission and its invocation-owned C physical contract.
Non-authority: C ambient defaults as a second late snapshot, Generic/Static behavior, tool names, and test logs do not define Boundary semantics.
Fail-fast boundary: the selected meaning is captured or explicitly rejected before JSON parsing, child execution, lowering, or artifact publication.
Smallest next slice: compare capture-effective-default versus explicit-no-flags, fix the contract wording, and select one existing options-owner change.
Non-claims: no MIR change, compatibility-owner retirement, typed-error promotion, warning cleanup, fallback, or whole-R7 completion.
```

Census boundary: Boundary profile1 Rust `OwnedPhysicalCompileContract` -> the
existing C physical-options copy and llc command builder. Include environment
unset/empty/present, explicit contract NULL/empty/nonempty, and the current
`-O3 -mcpu=native` ambient default. Exclude Generic 0, Static 2, Harness 3,
AOT/public/link retirement, and backend execution measurement.

## Design questions

- Does unset mean capture the existing effective default at admission, or mean
  explicit no flags? Must empty remain distinguishable from unset?
- Which existing owner can represent that distinction without Rust duplicating
  C default authority or allowing a late environment reread?
- Does the chosen policy preserve the current public/compatibility profiles and
  the Rust/C one-snapshot invariant on success and failure?
- What positive/negative pre-effect test proves unset, empty, explicit flags,
  and changed ambient state without measuring performance?

Until this Decision is accepted, do not edit the options owner or add a fixture.

## Accepted Decision

Mill's independent read-only design consultation selects **A**: Boundary
profile1 materializes the existing effective `-O3 -mcpu=native` when the
contract's `llc_flags` is NULL, at the existing C options admission, while a
non-NULL empty string means explicit no flags. Rust preserves that distinction
only for `llc_flags`; it does not duplicate the C default. The C copy stores the
materialized value once, so later ambient changes cannot affect the invocation.
Static profile2 keeps its current NULL-to-no-flags behavior.

Decision acceptance: one existing options-owner change, no new receipt/layer;
Boundary unset/empty/value and post-admission environment mutation tests;
existing Generic/Static/Harness/invalid-contract tests; C build, options smoke,
route/static/pointer guards and source-size checks. The next implementation
card is `MIR-CALL-COMPATIBILITY-RETIRE-R7-BOUNDARY-LLC-FLAGS-I0`.
