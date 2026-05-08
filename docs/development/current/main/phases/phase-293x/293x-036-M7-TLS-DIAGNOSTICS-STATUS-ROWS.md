---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-036-M7-TLS-DIAGNOSTICS-STATUS-ROWS
Scope: M7 hako.tls diagnostics status helpers
---

# 293x-036 M7 TLS Diagnostics Status Rows

## Decision

`hako.tls` gains two narrow diagnostics status helpers:

```text
TlsCoreBox.last_error_is_ok_i64()
TlsCoreBox.last_error_code_i64()
```

These helpers are derived from the existing diagnostics TLS seam:

```text
hako_last_error -> nyash.box.from_i8_string
```

This is not a generic thread-local slot API.

## Responsibility

- `lang/src/runtime/substrate/tls/` owns TLS capability vocabulary.
- `TlsCoreBox` owns the `.hako` diagnostics facade.
- VM-hako subset owns no-arg shape acceptance for these helper-shaped rows.
- Native keep remains the diagnostics TLS implementation below
  `hako_last_error`.

## Live Mapping

`last_error_code_i64()` maps diagnostics text to a narrow code:

| Text | Code |
| --- | --- |
| `OK` | `0` |
| `OOM` | `1` |
| `VALIDATION` | `2` |
| `UNSUPPORTED` | `3` |
| `NOT_FOUND` | `4` |
| other | `-1` |

VM-hako subset currently returns the deterministic default:

```text
last_error_text_h()      -> "OK"
last_error_is_ok_i64()   -> 1
last_error_code_i64()    -> 0
```

## Non-Goals

- No raw numeric TLS slot API.
- No generic thread/task-local slot API.
- No cache-slot primitive.
- No `TlsCell<T>` lowering.
- No allocator cache policy.
- No final platform TLS fallback.

## Acceptance

- `TlsCoreBox.last_error_text_h()` is strict no-arg in subset validation.
- `TlsCoreBox.last_error_is_ok_i64()` is accepted with no arguments.
- `TlsCoreBox.last_error_code_i64()` is accepted with no arguments.
- accidental arguments are rejected by the subset checker.
- reference docs state that generic TLS slots and cache slots remain future
  splits.

## Gates

```bash
bash tools/checks/k2_wide_tls_first_row_guard.sh
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
