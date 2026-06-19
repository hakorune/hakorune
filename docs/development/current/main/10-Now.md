Status: SSOT mirror
Date: 2026-06-19
Scope: one-screen current dashboard. Do not store landed history here.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Now

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`

## Active Blocker

```text
PHI-INPUT-REMAT-OPERAND-MEMO-001
```

The scan-methods focused timeout slice is closed by 296x-1304, and the touched
string-corridor regression is closed by 296x-1305. Stabilize PHI input
rematerialization identity before returning to loop resolver / app-front work.

## Next

1. Locate the PHI input rematerialization owner.
2. Add predecessor-local memoization without expanding accepted shapes.
3. Preserve fail-closed cycle behavior.
4. Run:

```bash
cargo test -q phi
cargo test -q string_corridor_sink
cargo check -q --lib
```

5. Commit the PHI memo slice separately.

## Recently Closed

- `STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001`
  - semantic string-corridor benchmark contract
  - read-only `MethodCallOperandView`
  - no benchmark/source/function-name branches

Closeout evidence:

```bash
cargo test -q operand_view
cargo test -q string_corridor_sink
cargo test -q string_kernel_plan
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Paused Lanes

- exact-AOT fastpath optimization is paused until a fresh measured owner
  appears.
- VM product-route app validation is retired; app/selfhost validation uses
  EXE/AOT unless a semantic-reference VM task explicitly opts in.
- build crate split planning is available but not the active blocker.
