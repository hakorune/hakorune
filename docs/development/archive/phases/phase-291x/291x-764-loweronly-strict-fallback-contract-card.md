# 291x-764 LowerOnly Strict Fallback Contract Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `join_ir_vm_bridge_dispatch/mod.rs`
- `join_ir_vm_bridge_dispatch/README.md`
- `CURRENT_STATE.toml`

## Why

`LowerOnly` rows are documented as lowering/structure observation only, with
normal VM Route A doing actual execution. The dispatch wrapper still treated
`false` from those routes as an execution failure under strict mode, which could
turn a successful structural observation into a strict bridge exit.

## Decision

Separate bridge execution from LowerOnly observation:

- `Exec`: may run JoinIR VM bridge and handle output/exit.
- `LowerOnly`: may invoke structural lowering observation, then always returns
  `false` to allow normal VM Route A, including strict mode.

## Landed

- Added a LowerOnly branch before Exec dispatch.
- LowerOnly routes are invoked only for observation and their return value is not
  interpreted as bridge failure.
- Exec dispatch now contains only Exec-capable routes.
- Documented the strict fallback contract in the bridge dispatch README.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The bridge LowerOnly strict semantics item is closed. Remaining cleanup is now:

- broad JoinIR lowering module-level `dead_code` allowance inventory
- older archive cards may still mention now-closed surfaces as history

## Proof

- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
