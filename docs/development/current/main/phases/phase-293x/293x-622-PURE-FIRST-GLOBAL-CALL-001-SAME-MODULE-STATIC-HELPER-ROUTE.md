# 293x-622 PURE-FIRST-GLOBAL-CALL-001 Same-Module Static Helper Route

Status: selected current
Date: 2026-05-18

## Decision

`PURE-FIRST-GLOBAL-CALL-001` is the compiler acceptance sidecar selected by
`MIMAP-122A`.

The row adds a narrow pure-first route for ordinary same-module static helper
calls:

```hako
static box Main {
    helper(x): i64 {
        return x + 1
    }

    main(args) {
        local y = Main.helper(41)
        return y
    }
}
```

The route is accepted only through MIR metadata:

```text
functions[].metadata.global_call_routes
  -> functions[].metadata.lowering_plan
  -> pure-first route preflight
  -> ny-llvmc direct function call
```

## Scope

- Add a same-module static helper route proof in `global_call_routes`.
- Accept only direct same-module targets that:
  - exist in the module
  - match arity
  - have a supported body according to `same_module_body_shape`
  - publish a return contract (`scalar_i64`, `void_sentinel_i64_zero`, or
    `object_handle` for typed object returns)
- Add a focused proof app / guard.
- Restore the MIMAP-119A proof app helper shape so the compiler route, rather
  than source inlining, carries the acceptance.

## Stop Lines

- No allocator behavior change.
- No source-level syntax change.
- No cross-module global-call widening.
- No recursive broadening beyond the existing same-module body-shape contract.
- No backend `.inc` app/name matcher.
- No silent fallback when target, arity, body support, or return contract is
  missing.
- No `.hako` workaround to hide the compiler blocker.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GCALL.1` | Add route proof / return contracts for supported same-module static helpers. | `lowering_plan` rows are `DirectAbi` with a stable proof. | no by-name helper allowlist |
| `GCALL.2` | Add focused proof app and guard. | route preflight and EXE path pass. | no allocator behavior |
| `GCALL.3` | Restore MIMAP-119A helper shape. | MIMAP-119A guard passes with helper calls. | no source inlining workaround |
| `GCALL.4` | Update docs / current state. | pointer guard and diff check pass. | no task bundle |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/<focused-global-call-guard>.sh
bash tools/checks/run_proof_app.sh --only MIMAP-119A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
git diff --check
```
