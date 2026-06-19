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
STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001
```

The scan-methods focused timeout slice is closed by 296x-1304, but the touched
string-corridor module has a local regression. Fix that before opening more
loop-route, resolver, or app-front work.

Known failing command:

```bash
cargo test -q string_corridor_sink
```

Known failing tests:

```text
benchmark_len_substring_views_compiles_without_loop_string_consumers
benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route
```

## Next

1. Restore the string-corridor benchmark routes without benchmark/source-name
   branches.
2. Run:

```bash
cargo test -q string_corridor_sink
cargo test -q string_kernel_plan
cargo check -q --lib
```

3. Commit the cleanup slice separately.

## Paused Lanes

- exact-AOT fastpath optimization is paused until a fresh measured owner
  appears.
- VM product-route app validation is retired; app/selfhost validation uses
  EXE/AOT unless a semantic-reference VM task explicitly opts in.
- build crate split planning is available but not the active blocker.
