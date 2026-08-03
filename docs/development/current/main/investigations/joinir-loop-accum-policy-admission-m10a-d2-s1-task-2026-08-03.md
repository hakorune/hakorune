---
Status: Accepted task — consume the sealed singleton observation in policy
Date: 2026-08-03
Decision: policy owns canonical schedule construction; the caller receives a branded admission handoff
Related:
  - joinir-loop-accum-singleton-certificate-m10a-d2-s0-task-2026-08-03.md
  - joinir-loop-accum-production-issuer-m10a-d2-design-stop-2026-08-03.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
---

# DirectAccum policy admission: M10a/D2/S1

## Objective

Consume one `VerifiedDirectAccumSingletonObservationV1` and issue one typed
policy admission through the existing policy SSOT. The caller must not build
the 19-row schedule, select a cursor, or inspect route IDs.

```text
VerifiedDirectAccumSingletonObservationV1
  -> policy-owned canonical 19-row observation matrix
  -> freeze_loop_route_schedule_v1
  -> evaluate_frozen_loop_route_schedule_v1
  -> VerifiedDirectAccumRouteAdmissionV1
```

The admission handoff must retain the observation continuation so source,
facts, and frame identity are consumed exactly once by the next Recipe/profile
stage. A raw `VerifiedLoopPolicyWinnerV1` alone is insufficient: it loses the
fact that the singleton certificate authorized the matrix.

## Matrix contract

The policy owner may mark the Accum candidate qualified only after consuming
the singleton/disjointness proof. Every other row must be an explicit typed
pre-effect disposition. Generic V0/V1 and overlap cases must never be
silently converted to decline; they remain blocked/unadmitted when the
certificate is absent.

The schedule seal or digest must stay inside the admission. The physicalizer
must not receive route IDs, raw cursors, or the full migration schedule.

## Acceptance gates

- The only production policy entry consumes the sealed singleton observation.
- The existing `freeze_loop_route_schedule_v1` and
  `evaluate_frozen_loop_route_schedule_v1` remain the sole policy authority.
- Test-only winner issuers are not promoted or called by production code.
- Foreign source/frame/owner and missing certificate reject before Builder
  effects.
- DirectAccum fixture admits exactly once; simple-while, Generic, and overlap
  fixtures cannot enter this admission without the certificate.
- Policy parity against the existing legacy schedule is test evidence only.
- No `CanonicalFirstFamilyPlanV1::DirectAccum`, `route_loop`, physicalizer,
  PHI/SSA owner, Retry removal, or old-edge retirement is included in S1.
- Every touched Rust file remains below 800 lines.
