---
Status: closed__2026-09-21__WarningGenericG0DemandTestFacade__OneFacadeDeletion
Task: MIRBUILDER-WARNING-GENERIC-G0-DEMAND-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i65-2026-09-21.md
Implementation permission: true for one cfg(test) parent re-export deletion only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I66
---

# Warning cleanup: Generic G0 demand test facade

## Six-line brief

```text
Decision: remove only the unused cfg(test) parent re-export of
  generic_operation_demand_parts_for_test; the existing tests keep importing
  the canonical generic_g0 owner directly.
Source authority + canonical issuer: generic_g0/producer_tests.rs owns the
  test helper; the I65 two-surface inventory proves the parent facade is
  caller-zero.
Non-authority: warning counts alone, cargo-fix, wildcard imports, visibility
  widening, Recipe meaning, or production demand interpretation.
Fail-fast boundary: any parent caller, compile error, new warning/failure
  name, or changed test behavior rejects the deletion.
Smallest next slice: delete exactly one cfg(test) use line from
  src/mir/loop_recipe_contract/mod.rs.
Non-claims: no Generic G0 producer change, Recipe/JoinSig change, suppression,
  dead-code cleanup, or production route change.
```

## Preconditions and acceptance

The current fixed baseline is lib **1,754** and lib-test **560**. The source
census has two direct child-owner test imports and no parent-facade caller;
the helper is test-only. Implementation is forbidden until the pointer enters
fast mode. Then remove only the parent re-export and run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo fmt --all -- --check
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Expected counts are lib **1,754** and lib-test **559**. Any extra reference,
compile failure, new warning/red name, or test behavior change rejects the
deletion and returns to the next baseline refresh. No `#[allow]`, cargo-fix,
or neighboring import cleanup is allowed.

## Execution evidence

The one authorized cfg(test) parent re-export line was removed. The two
existing callers continue to use `generic_g0` directly; no production code,
Recipe owner, or test body changed.

The fixed gates ran sequentially and exited 0:

| gate | result | evidence |
| --- | --- | --- |
| `cargo check --profile quick --lib -j4` | lib **1,754** warnings | `/tmp/hakorune-warning-generic-g0-lib-20260921.log` |
| `cargo test --profile quick --lib --no-run -j4` | lib-test **559** warnings | `/tmp/hakorune-warning-generic-g0-lib-test-20260921.log` |
| `cargo fmt --all -- --check` | PASS | local command |
| `git diff --check` | PASS | local command |
| current-state pointer guard | PASS | local command |

No new failure name, warning family, or semantic behavior appeared. The next
step is a fresh two-surface baseline refresh; no neighboring import or
suppression cleanup is included.
