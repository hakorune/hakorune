# MIR-CALL-R7-CALLER-ZERO-D2 — next bounded deletion selection

Status: closed__accepted__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D1 (accepted 2026-09-25;
        S6 landed at 755c0688c7)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Task (design_stop)

D1 census produced a verified dead-on-all-inputs inventory. Select the
next bounded deletion set — one responsibility per slice, each with
its own fail-fast boundary and no-laundering pin.

### Dead-on-all-inputs candidates (D1 census, verified)

| Candidate | Owner surface | Notes |
| --- | --- | --- |
| `canonicalize_legacy_array_write_calls` | `array_element_write.rs:105-170` + call site `semantic_refresh/contracts.rs:116` | D1 ranked this equal-safety with S6; backstopped by the immediately-following `reject_residual_calls` named-stop (:493-524) which turns silent-upgrade into `[mir/array_write/residual_call]` rejection. |
| `LegacyCallV0{Closure}` arm | `callsite_canonicalize/pass.rs` | No ingress mints `LegacyCallV0{Closure}`; lambdas mint `NewClosure` directly. Needs its own no-laundering pin + closure-shape tests audit. |
| `joinir_id_remapper*` arms | `joinir_id_remapper.rs:301-313`, `joinir_id_remapper_values.rs:77-93` | Dead reissue/read arms inside JoinIR merge machinery running on canonical instructions only. |
| `edge_rematerialization` arm | `edge_rematerialization.rs:227-245` | Dead; note canonical `Call` is *not* matched (pre-existing gap, separate lane). |
| `concat_corridor_apply` arms | `concat_corridor_apply.rs:524-564` | Dead Method/Extern rewrite arms. |
| `extern_call_route_plan` / `generic_method_route_plan` legacy-only scans | route-plan family | Dead metadata producers (`extern_call_routes` never populated) — deleting them is a metadata-contract change; check consumers before selecting. |
| `inline_soft_leaf` arm | `inline_soft_leaf.rs:183-197` | Dead candidate-collection arm. |

### Explicitly excluded

- Named-stop readers (backend_capability, allowlists,
  `runner/product/llvm`, wasm, interpreter, `published_backend_view`,
  `builder_emit` method-none stop, `reject_residual_calls`) — deleting
  them converts fail-fast into silent fallthrough; they retire with
  the variant.
- The `func` slot — type-level change bundled with eventual variant
  retirement.
- The llvmlite `project_module_to_legacy_calls` projection owner and
  its emit-side consumers (`emitters/*`, `calls_compat_v0.rs`).
- `callsite_canonicalize` itself (NewClosure arms + receiver-operand
  rewrite remain live).

## Six-line Decision (accepted 2026-09-25)

```text
Decision: delete canonicalize_legacy_array_write_calls
          (array_element_write.rs:105-170) and its sole call site
          (semantic_refresh/contracts.rs:116-117) — the arm only
          matches LegacyCallV0{Method{ArrayBox}} which no production
          ingress mints; the sole surviving projection runs at emit
          time on a clone that never re-enters refresh.
Source authority + canonical issuer: ArrayElementWrite is minted by
          the typed owner (mir/builder array-write emission); array
          state contracts are issued by refresh_function_array_write_witnesses.
Non-authority: canonicalize_legacy_array_write_calls — a repair path
          upgrading a retired carrier shape.
Fail-fast boundary: removal strengthens behavior — a residual
          LegacyCallV0{ArrayBox.push/set/insert} is no longer silently
          upgraded; the existing reject_residual_calls named-stop
          ([mir/array_write/residual_call]) fires in the immediately
          following refresh call (rebuild -> reject_residual_calls).
Smallest next slice: MIR-CALL-R7-LEGACY-ARRAY-WRITE-CANON-DELETE-S7
          — function + call site + a residual-reject pin.
Non-claims: does not remove project_module_to_legacy_calls (the
          documented llvmlite projection owner), reject_residual_calls,
          or any other LegacyCallV0 reader; does not touch the
          canonicalize pass Closure arm or JoinIR reissuer arms.
```

## Exit

- [x] One bounded deletion set selected with caller-zero evidence —
      `MIR-CALL-R7-LEGACY-ARRAY-WRITE-CANON-DELETE-S7`.
- [x] Guard/pointer/workstream synced.
