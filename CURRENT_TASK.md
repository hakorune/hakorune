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
HAKORUNE-MIR-DEFS-CALL-UNIFIED-MATERIALIZATION-001
```

Purpose:

```text
Materialize the selected `hakorune_mir_defs::call_unified` single-module
RustSubset bundle after 296x-1360 selected it as the next green app-front
task.
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
HAKORUNE-MIR-CORE-RUSTSUBSET-PILOT-001 is closed by 296x-1332.
RUST-SUBSET-CRATE-WRAPPER-EXE-PURE-ROUTE-UNBLOCK-001 is closed by 296x-1333.
PURE-ROUTE-UNSUPPORTED-SHAPE-DIAGNOSTIC-001 is closed by 296x-1334.
RUST-SUBSET-CRATE-WRAPPER-EXE-SMOKE-001 is closed by 296x-1335.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001 is closed by 296x-1336.
HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001 is blocked by 296x-1337.
RUST-SUBSET-TUPLE-STRUCT-CONSTRUCTOR-SKELETON-001 is closed by 296x-1338.
RUST-SUBSET-COMPOUND-ASSIGN-SKELETON-SAFETY-001 is closed by 296x-1339.
RUST-SUBSET-SELF-QUALIFIED-CALL-SKELETON-SAFETY-001 is closed by 296x-1340.
HAKORUNE-MIR-CORE-ID-MODULES-MATERIALIZATION-001 is closed by 296x-1341.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-002 is closed by 296x-1342.
HAKORUNE-MIR-CORE-VALUE-KIND-MATERIALIZATION-001 is closed by 296x-1343.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-003 is closed by 296x-1344.
RUST-SUBSET-ENUM-VARIANT-VALUE-SKELETON-SAFETY-001 is closed by 296x-1345.
RUST-SUBSET-VEC-NEW-CALL-SKELETON-SAFETY-001 is closed by 296x-1346.
HAKORUNE-MIR-CORE-EFFECT-MATERIALIZATION-001 is closed by 296x-1347.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-004 is closed by 296x-1348.
HAKORUNE-MIR-BUILDER-BINDING-CONTEXT-MATERIALIZATION-001 is closed by 296x-1349.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-005 is closed by 296x-1350.
HAKORUNE-MIR-BUILDER-VARIABLE-CONTEXT-MATERIALIZATION-001 is closed by 296x-1351.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-006 is closed by 296x-1352.
RUST-SUBSET-ASSOCIATED-FUNCTION-CALL-SKELETON-SAFETY-001 is closed by 296x-1353.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-007 is closed by 296x-1354.
RUST-SUBSET-ASSOCIATED-CONST-VALUE-SKELETON-SAFETY-001 is closed by 296x-1355.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-008 is closed by 296x-1356.
HAKORUNE-MIR-BUILDER-CORE-CONTEXT-MATERIALIZATION-001 is closed by 296x-1357.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-009 is closed by 296x-1358.
HAKORUNE-MIR-BUILDER-CONTEXT-MATERIALIZATION-001 is closed by 296x-1359.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-010 is closed by 296x-1360.
```

Acceptance for the current slice:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Task Order

1. Read 296x-1361.
2. Materialize the selected `crate::call_unified` bundle.
3. Add the focused wrapper and smoke entries.
4. Keep generated-program execution claim at 0.

Recommended next row:

```text
HAKORUNE-MIR-DEFS-CALL-UNIFIED-MATERIALIZATION-001
```

296x-1360 selected `hakorune_mir_defs::call_unified` because its generated
skeleton already reaches MIR emit after previous skeleton-safety blockers were
closed. 296x-1361 is the implementation row.

## Pointers

- Current state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Latest phase card: read `latest_card_path` in `CURRENT_STATE.toml`
- Current docs policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Restart mirror: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Thin dashboard: `docs/development/current/main/10-Now.md`
