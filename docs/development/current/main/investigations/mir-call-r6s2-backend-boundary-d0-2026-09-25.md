# MIR-CALL-R6S2-BACKEND-BOUNDARY-D0 — backend boundary selection

Status: selected__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R6S1-GLOBAL-PRODUCER-COHORT-S1 (landed 2026-09-25)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             section "R6/R7 migration program" row R6-S2.

## Task

Select one bounded backend boundary for R6-S2, or record `NoSafeSlice`
with a reopen trigger. Queue contract:

```text
R6-S2  one backend boundary:
       typed consume or UnsupportedBeforeArtifact before codegen;
       no JSON, name, registry, args[0], fallback, or retry.
```

## Decision inputs

R6-S1 landed the only viable producer cohort (`Callee::Global`); every
other cohort was declined in the D0. The backend side still has legacy
`LegacyCallV0` consumers: the mir_interpreter dispatch, WASM codegen
preflight stops (already `Stop`ed per landed rows), plus analysis-side
readers (`global_call_route_plan`, `string_corridor*`,
`value_representation_fact`, `ordered_map_origin_plan`,
`same_module_body_shape`, `hotcore_method_summary`, pass-layer
canonicalizers) and `verification/` consumers.

Candidate boundaries to audit (pick exactly one, or NoSafeSlice):

- `MirInterpreter` call dispatch: does `execute_instruction` consume
  typed `MirCall` for all admitted callees and reject `LegacyCallV0`
  with `UnsupportedBeforeArtifact` before dispatch? Which callee kinds
  still ride the legacy arm, and is the reject path one named terminal?
- AOT/EXE (`aot`/`wasm` codegen): are all LegacyCallV0 readers already
  stopped, or is there a residual typed consume to land?
- Analysis readers (`global_call_route_plan` family,
  `value_representation_fact`, passes): readers of emitted modules —
  classify which are producer-adjacent (must move to typed `MirCall`)
  vs quarantined ingress readers (R6-S3).

Constraints (from the migration program): no `CallV2`, no new receipt,
no fallback/retry, typed consume or `UnsupportedBeforeArtifact` only,
one boundary per row, finite caller list before implementation.

## Exit

- [ ] One accepted bounded boundary with source authority, canonical
  issuer, fail-fast boundary, finite callers, and verification named —
  or `NoSafeSlice` with reopen trigger.
- [ ] Reader classification: producer-adjacent vs quarantined-ingress
  for every `LegacyCallV0` match site listed above.
- [ ] Named next execution row, or an explicit pause.
