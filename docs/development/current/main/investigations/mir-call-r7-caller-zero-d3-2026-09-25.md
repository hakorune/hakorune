# MIR-CALL-R7-CALLER-ZERO-D3 — canonicalize Closure arm deletion selection

Status: closed__accepted__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D2 (accepted 2026-09-25;
        S7 landed at ec9a78254a)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Evidence (from D1 census + owner spot-check)

- The `callsite_canonicalize` `LegacyCallV0{Closure}` arm rewrites a
  receiverless closure-constructor legacy row into `NewClosure`.
- No production ingress mints `LegacyCallV0{Closure}`: v0 `"call"` /
  `"mir_call"` rows stop pre-construction
  (`[freeze:contract][mir-json-v0/legacy-call-stopped]`); lambdas mint
  `NewClosure` directly; the llvmlite projection only emits Method
  rows.
- `classify_closure_call_shape` stays live — `allowlists.rs:21` still
  uses it for the named-stop support surface. Only the pass's arm +
  its imports are deleted.
- `ncl0_rewrites_call_closure_to_newclosure` pins the rewrite itself;
  it flips to a no-laundering pin (residual Closure row untouched).
  `ncl0_does_not_rewrite_*` / `ncl2_*` / `ncl1_*` tests stay green.

## Six-line Decision (accepted 2026-09-25)

```text
Decision: delete the callsite_canonicalize LegacyCallV0{Closure}
          rewrite arm and its classify_closure_call_shape/
          ClosureCallShape imports — input-zero on every schedule
          site.
Source authority + canonical issuer: NewClosure rows are issued by
          the builder's lambda lowering directly; closure ctor shape
          classification stays with ssot::closure_call and the
          allowlist reader.
Non-authority: the canonicalize Closure arm — a repair path reshaping
          a retired carrier variant.
Fail-fast boundary: a hypothetical residual LegacyCallV0{Closure}
          row falls to `.. => 0` and is still rejected by the
          interpreter (`Closure creation in VM`) and
          backend_capability named-stops; flipped test pins the
          no-laundering behavior.
Smallest next slice: MIR-CALL-R7-CANONICALIZE-LEGACY-CLOSURE-ARM-DELETE-S8
          — pass.rs arm + imports + ncl0 pin flip.
Non-claims: does not remove NewClosure externalization (NCL-1), the
          Global no-op arm, the LegacyCallV0 type, or
          ssot::closure_call.
```

## Exit

- [x] Bounded deletion set selected with caller-zero evidence —
      `MIR-CALL-R7-CANONICALIZE-LEGACY-CLOSURE-ARM-DELETE-S8`.
- [x] Guard/pointer/workstream synced.
