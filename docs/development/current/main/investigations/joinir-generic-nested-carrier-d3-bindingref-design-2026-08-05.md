---
Status: active design stop — scoped D3 consultation
Date: 2026-08-05
Parent: ../design/joinir-generic-post-effect-debt-classification-ssot.md
Decision: provisional — exact BindingRef-proven nested-carrier class only
---

# Generic nested-carrier scoped D3 — BindingRef class

## Boundary

The S2 witness is green, but it is one test-only identity certificate. It does
not close the parent Generic D2, prove a V0-debt-to-V1-success trace, or permit
global M4-D3 implementation. This card is the next design stop for the exact
class proven by S2: an inner write and post-loop outer read that share one
strict-ancestor binding.

## Source authority

```text
parsed source
  -> VerifiedResolvedFunctionV1
  -> VerifiedResolvedLoopSourceForestV1
  -> resolver-issued assignment/read BindingRefV1
  -> strict-ancestor + function/frame/source identity
  -> LivePreflightFrameV1 mode/raw schedule
  -> GenericLoopV1Facts.carrier_observation
  -> fresh V1 stage/candidate observation
```

The resolver/source products and canonical facts are the only semantic inputs.
The existing 443-line cfg(test) S2 sibling is the bounded witness surface; it
does not publish a production capability.

## Non-authority

Do not re-read AST in policy or pair bindings by names, route IDs, S1 PHI/final
value tags, synthetic `both_body()`, plan digests, `diagnostic_effective`,
legacy receipts/terminal status, or runtime result. These remain corroborating
or later-parity evidence only.

## Fail-fast and dispositions

Only a test-only neutral evaluator may issue a candidate when all of these are
present before Builder mutation:

```text
Release/Strict natural Both [V0, V1]
CompleteRecursiveCarrier
same strict-ancestor BindingRefV1
same function/frame/source identity
natural V1 LowerSome + GenericComposer first effect
fresh candidate repeat is stable
```

Every other row remains `UnresolvedStop`: planner-required `[V1]`/V0
suppression, shadowing, foreign/missing/ambiguous BindingRef, owner/frame
mismatch, `NoRecursive`, `Unavailable`, `Ambiguous`, target mismatch, failed
stage, or unstable repeat. Any effectful composer/verifier/lower failure is
`TerminalFreezeTarget` or `UnresolvedStop`; it never advances by retry or
fallback.

## Minimal design slice

The consultation should freeze one typed mismatch matrix over the existing
parsed positive, shadowing negative, and planner-required rows. Optional
negative fixtures (nested If/ScopeBox, duplicate/multi-write, unsupported
Index/Program/CompoundAssignment, V1-only/no-recursive, foreign owner/frame)
must use the same parser -> resolver BindingRef/forest -> canonical facts ->
mode-scoped selector chain. They cannot close parent D2 by themselves.

No production selector/policy arm, source-to-selection handoff, Recipe,
JoinSig, PHI, physicalizer, Retry/fallback removal, scheduler, Builder, MIR,
backend, or runtime route is in this slice. A production correction would first
need a separate design decision for a co-sealed resolved-carrier capability;
V0 remains authoritative for all other rows.

## Acceptance and closeout

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4_s2 -- --nocapture
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4 -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The existing S2 evidence is 3/3 focused tests, with the 443-line test sibling
and zero production callers/imports. This card closes only when the typed
mismatch boundary and its non-claims are synchronized into the parent SSOT,
stage-matrix reference, Generic README, resolved-semantics README, current
pointers, and MIRBuilder workstream. A green scoped evaluator still does not
resolve parent D2 or authorize M10a/M10b.

Implementation completion rule: if a later design decision authorizes a
production slice, the task is not complete until the corresponding
`docs/reference/**` pages and navigation/status indexes are updated in the
same closeout, with the implementation receipt, focused tests, and explicit
fail-fast boundary. Documentation is not deferred to a later cleanup task.
