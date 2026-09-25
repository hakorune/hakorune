# MIR-CALL-R7-CALLER-ZERO-D1 — caller-zero census reopen

Status: closed__accepted__2026-09-25
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

## Census result (read-only worker, spot-checked by owner)

- Sole live production mint:
  `array_element_write::project_module_to_legacy_calls` (llvmlite
  ExplicitCompatibility projection; clone exits via JSON emit, never
  re-enters a pass).
- All schedule sites of `callsite_canonicalize`
  (`compiler/mod.rs:719`, `module_postprocess.rs:148`,
  `drain_terminal.rs:215`, `mir_json_v0.rs:34`, v0 bridge) run on the
  *original* module strictly before any projection — so the pass's
  `LegacyCallV0` arms are unreachable from production input.
- `func` slot: no production minter writes `func != INVALID`; the
  non-INVALID consumers are compat-emit decoration and named-stop
  discriminators. Type-level retirement is part of eventual variant
  deletion, not this slice.
- Named-stop readers (must keep rejecting — deletion is a contract
  change, excluded): `backend_capability`, `allowlists`,
  `runner/product/llvm`, `backend/wasm`, `mir_interpreter`,
  `published_backend_view`, `builder_emit` method-none stop,
  `reject_residual_calls`.
- Dead-on-all-inputs work/reissue arms (deletion-safe, future slices):
  canonicalize Method+Closure arms, `canonicalize_legacy_array_write_calls`,
  `edge_rematerialization` arm, `joinir_id_remapper*` arms,
  `concat_corridor_apply` arms, `inline_soft_leaf`,
  `extern_call_route_plan`/`generic_method_route_plan` legacy-only
  scans, seed-plan family matches.
- Named casualties observed by census (recorded, not acted on):
  canonical `Call` rows bypass legacy-only readers
  (`extern_call_routes` never populated; `edge_rematerialization`
  Method arm; `fastmem::is_mem_call_with_arg`; `schedule/block.rs`
  `is_call_like`) — pre-existing shape gaps owned by their own lanes.
- S5 stale test fixed in-flight:
  `selfhost/json.rs::release_mode_keeps_legacy_callsite_compat` now
  asserts the canonical `Call{Method}` row.

## Six-line Decision (accepted 2026-09-25)

```text
Decision: delete the callsite_canonicalize LegacyCallV0{Method}
          rewrite arm plus its orphaned value_types/known_user_boxes
          plumbing and helpers.rs — the arm is input-zero on every
          schedule site.
Source authority + canonical issuer: canonical Callee::Method rows are
          issued by mir::ssot::method_call (boxcall_emit, v0 boxcall
          arm); user-box identity resolution stays with the route-plan
          owners, not a post-pass rewriting a retired carrier.
Non-authority: the canonicalize Method arm — a repair path reshaping
          rows no producer emits.
Fail-fast boundary: a hypothetical residual LegacyCallV0 row falls to
          `.. => 0` and is still rejected by the backend_capability /
          interpreter / selected-Dynamic named-stops; removal adds a
          "no silent laundering" pin to prove it.
Smallest next slice: MIR-CALL-R7-CANONICALIZE-LEGACY-METHOD-ARM-DELETE-S6
          — pass.rs Method arm + param threading + helpers.rs;
          Closure arm and Global no-op stay for a later slice.
Non-claims: does not remove LegacyCallV0, the `func` slot, the
          canonicalize pass, the Closure arm, or any named-stop
          reader; does not touch the llvmlite projection lane.
```

## Exit

- [x] Census covers the exact D0 boundary: minters -> reissuers ->
      production readers -> compat/contract consumers; includes/excludes
      stated.
- [x] One bounded deletion set with caller-zero evidence selected and
      named as the next execution row —
      `MIR-CALL-R7-CANONICALIZE-LEGACY-METHOD-ARM-DELETE-S6`.
- [x] Guard/pointer/workstream synced.
