# CURRENT_TASK

Status: SSOT pointer
Date: 2026-06-20
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read the `latest_card_path` named in `CURRENT_STATE.toml`.
3. Check the worktree:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

4. Run heavier gates only when the current code slice is ready:

```bash
tools/checks/dev_gate.sh quick
```

## Current Task

Read these fields in `docs/development/current/main/CURRENT_STATE.toml`:

- `active_lane`
- `active_phase`
- `latest_card_path`
- `current_blocker_token`

Current blocker:

```text
HAKORUNE-MIR-CORE-RUSTSUBSET-PILOT-001
```

Purpose:

```text
Materialize the selected `hakorune_mir_core` RustSubset module slice and run it
through the existing skeleton pipeline.
```

Current evidence:

```text
STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001 is closed by 296x-1305.
PHI-INPUT-REMAT-OPERAND-MEMO-001 is closed by 296x-1306.
STRING-CORRIDOR-STABLE-LENGTH-HINT-FALLBACK-RETIRE-001 is closed by 296x-1307.
RUST-SUBSET-APP-FRONT-LOOP-TRUE-BREAK-CONTINUE-SMOKE-CLOSEOUT-001 is closed by
296x-1308. `apps/rust-subset-to-hako/smoke.sh` reports `summary=ok`.
RUST-SUBSET-SYN-ADAPTER-SMOKE-ENTRY-001 is closed by 296x-1309.
RUST-SUBSET-SYN-ADAPTER-INDEX-EXPRESSION-001 is closed by 296x-1310.
RUST-SUBSET-SYN-ADAPTER-BREAK-CONTINUE-UNSUPPORTED-HANDOFF-001 is closed by
296x-1311.
RUST-SUBSET-SYN-ADAPTER-GENERIC-FUNCTION-SKELETON-001 is closed by 296x-1312.
RUST-SUBSET-SYN-ADAPTER-UNSUPPORTED-TRAIT-HANDOFF-HARDENING-001 is closed by
296x-1313.
RUST-SUBSET-SMOKE-FIXTURE-TABLE-REFACTOR-001 is closed by 296x-1314.
RUST-SUBSET-CRATE-HANDOFF-INVENTORY-001 is closed by 296x-1315.
RUST-SUBSET-MODULE-SCHEMA-VALIDATION-PARITY-001 is closed by 296x-1316.
RUST-SUBSET-PATH-NAME-NORMALIZATION-001 is closed by 296x-1317.
RUST-SUBSET-CRATE-MANIFEST-V0-001 is closed by 296x-1318.
RUST-SUBSET-SYN-ADAPTER-MULTI-MODULE-PROBE-001 is closed by 296x-1319.
RUST-SUBSET-CRATE-HANDOFF-MIR-ACCEPTANCE-001 is closed by 296x-1320.
HAKO-ORDERED-MAP-BOX-SSOT-001 is closed by 296x-1321.
HAKO-ORDERED-MAP-BOX-V0-001 is closed by 296x-1322.
MIRBUILDER-BINDING-CONTEXT-ORDERED-MAP-PROBE-001 is closed by 296x-1323.
CONSTRUCTOR-LIFECYCLE-FIELD-INIT-BIRTH-PROBE-001 is closed by 296x-1324.
FIELD-INITIALIZER-LIBRARY-ROUTE-PROBE-001 is closed by 296x-1325.
IMPORTED-FIELD-INIT-BIRTH-MERGE-FIX-001 is closed by 296x-1326.
GLOBAL-CALL-UNKNOWN-CALLEE-DIAGNOSTIC-001 is closed by 296x-1327.
CREAT-SUBSET-PILOT-SELECTION-001 is closed by 296x-1328.
HAKORUNE-BOX-CORE-RUSTSUBSET-PILOT-001 is closed by 296x-1329.
RUST-SUBSET-GENERATED-FUNCTION-MIR-ACCEPTANCE-001 is closed by 296x-1330.
RUST-SUBSET-NEXT-CRATE-PILOT-SELECTION-001 is closed by 296x-1331.
```

Acceptance for the current slice:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
# plus the focused hakorune_mir_core pilot command selected by 296x-1331.
```

## Task Order

1. Generate the `hakorune_mir_core` crate bundle with the external syn adapter.
2. Materialize the selected `crate::control_ids` and `crate::types` module
   slice as checked-in fixtures.
3. Run generated skeletons through parse / MIR emit where supported.
4. Keep Rust parser/crate graph discovery in the external adapter boundary.
5. Do not implement Rust name resolution or `use` resolution in this row.
6. Do not reopen fastpath or constructor lifecycle work without a new blocker.

Recommended next row:

```text
HAKORUNE-MIR-CORE-RUSTSUBSET-PILOT-001
```

## Pointers

- Current state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Latest phase card: read `latest_card_path` in `CURRENT_STATE.toml`
- Current docs policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Restart mirror: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Thin dashboard: `docs/development/current/main/10-Now.md`
