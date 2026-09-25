# MIR-CALL-R7-CALLER-ZERO-D1 — caller-zero census reopen

Status: open__design_stop__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D0 (closed NoSafeSlice 2026-09-25,
        reopen trigger fired by S4+S5)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Reopen trigger (fired)

D0 recorded: "when BOTH quarantined ingress minters are retired the
reader census flips — rerun this D0". Both are now retired:

- S4 `1bf8dc40c7`: `emit_value_unified` mints canonical
  `Call{Callee::Value}`; no `NYASH_MIR_UNIFIED_CALL=0` path reaches a
  legacy Value mint.
- S5: `mir_json_v0/module.rs` boxcall arm mints canonical
  `Call{Callee::Method}` via `mir::ssot::method_call`; zero
  `LegacyCallV0` text remains in the v0 parser.

Residual production mint (unchanged, in-scope for classification not
deletion): `mir::array_element_write::project_module_to_legacy_calls`
— the documented ExplicitCompatibility projection owner for the
`llvmlite-compat` lane (sole caller seam
`runner/modes/common_util/array_write_backend.rs`). It rewrites
`ArrayElementWrite` -> `LegacyCallV0{Method}` on a cloned module for a
backend that has not learned the V1 op. This is a projection owner,
not an ingress minter; its fate is decided by the llvmlite lane's own
typed-consumer install, not by this census.

## Task (design_stop)

Re-run the D0 census with the minter set now empty on ingress:

1. `LegacyCallV0` production writers/reissuers:
   - Writers: none (S4+S5).
   - Reissuers that *reshape* existing legacy rows:
     `callsite_canonicalize` (LegacyCallV0{Method} arm — proven dead on
     v0 input, but is it dead on every scheduled caller?),
     `edge_rematerialization`, `joinir_id_remapper*`, `builder_emit`,
     `array_element_write` (the projection owner above).
   - Classify each: still-live reissuer vs dead-on-all-inputs.
2. Readers that require `LegacyCallV0`/`func`/optional callee to keep
   working — the ~10 owner families named in D0 plus any new ones.
3. Select exactly one bounded deletion set with caller-zero evidence
   (e.g., the dead canonicalize Method arm, or a `func`-slot
   retirement for a callee-kind whose readers all stop), or record
   `NoSafeSlice` with the next observable trigger.

Known baseline caveat: the `func` slot is still load-bearing for
`callee:None` render/validation and the v0 receiverless-compat emit
(`method_none_keeps_legacy_receiver_func_until_r6`); removing it is a
contract change, not cleanup. Deletion candidates must exclude
contracts still consumed by the v0 wire or the llvmlite projection.

## Six-line Decision (to fill)

```text
Decision:
Source authority + canonical issuer:
Non-authority:
Fail-fast boundary:
Smallest next slice:
Non-claims:
```

## Exit

- [ ] Census covers the exact D0 boundary: minters -> reissuers ->
      production readers -> compat/contract consumers; includes/excludes
      stated.
- [ ] One bounded deletion set with caller-zero evidence selected and
      named as the next execution row — or `NoSafeSlice` + trigger.
- [ ] Guard/pointer/workstream synced.
