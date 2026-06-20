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
HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-RESUME-001
```

Purpose:

```text
Resume the manifest-driven `hakorune_mir_builder` 7-module crate-bundle route
using the Main-owned dynamic FileBox loop and loop-carried string accumulation
shapes now proven by 296x-1380 and 296x-1383.
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
HAKORUNE-MIR-DEFS-CALL-UNIFIED-MATERIALIZATION-001 is closed by 296x-1361.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-011 is closed by 296x-1362.
RUST-SUBSET-GENERIC-IMPL-TARGET-SKELETON-SAFETY-001 is closed by 296x-1363.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-012 is closed by 296x-1364.
RUST-SUBSET-REFERENCE-TYPE-SPELLING-SKELETON-SAFETY-001 is closed by 296x-1365.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-013 is closed by 296x-1366.
HAKORUNE-MIR-BUILDER-TYPE-CONTEXT-MATERIALIZATION-001 is closed by 296x-1367.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-014 is closed by 296x-1368.
RUST-SUBSET-SELF-VALUE-SKELETON-SAFETY-001 is closed by 296x-1369.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-015 is closed by 296x-1370.
RUST-SUBSET-OPTION-CONSTRUCTOR-SKELETON-SAFETY-001 is closed by 296x-1371.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-016 is closed by 296x-1372.
HAKORUNE-MIR-BUILDER-METADATA-CONTEXT-MATERIALIZATION-001 is closed by 296x-1373.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-017 is closed by 296x-1374.
HAKORUNE-MIR-BUILDER-CRATE-ROOT-MATERIALIZATION-001 is closed by 296x-1375.
RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-018 is closed by 296x-1376.
HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-001 is blocked by 296x-1377
after the reusable FileBox helper reaches MIR emit but fails EXE pure-route
lowering. Hand-unrolled 7-module wrapper fallback remains forbidden.
CRATE-BUNDLE-FILE-ROUTE-HELPER-EXE-SHAPE-001 is closed by 296x-1379 as a
boundary decision: FileBox remains Main/input-route owned; helper-owned
FileBox is not implemented in that row.
FILEBOX-DYNAMIC-PATH-LOOP-EXE-SHAPE-001 is closed by 296x-1380; the focused
Main-owned dynamic FileBox path loop reaches MIR emit and EXE.
HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-RESUME-001 is blocked by
296x-1382 after the first wrapper implementation reaches MIR emit but fails
EXE on loop-carried string accumulation with an undefined LLVM value in
nyash.string.concat3_hhh.
STRING-CONCAT-LOOP-CARRIED-EXE-SHAPE-001 is closed by 296x-1383; deferred
concat pairs are materialized before later concat_hh / concat3 use.
```

Acceptance for the current slice:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Task Order

1. Read 296x-1382.
2. Implement the `hakorune_mir_builder` crate-bundle wrapper using the
   Main-owned dynamic FileBox input route.
3. Preserve manifest order and module/source framing.
4. Verify wrapper EXE parity and fixture-only aggregate MIR emit.
5. Keep use/name resolution and generated-program execution claim disabled.

Recommended next row:

```text
HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-RESUME-001
```

296x-1382 can resume now that 296x-1380 and 296x-1383 closed the input-route
FileBox and loop-carried string concat EXE shapes.

After 296x-1377 closes, the planned design follow-up is:

```text
RUST-LIFECYCLE-PROJECTION-SSOT-001
```

SSOT draft:

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
```

Planned card:

```text
docs/development/current/main/phases/phase-296x/296x-1381-RUST-LIFECYCLE-PROJECTION-SSOT-001.md
```

Task sequence:

```text
1. RUST-LIFECYCLE-PROJECTION-SSOT-001
2. RUST-LIFECYCLE-FACTS-VOCAB-000
3. HAKO-LIFECYCLE-PLAN-VOCAB-000
4. MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001
5. MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-ORACLE-PARITY-001
```

## Pointers

- Current state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Latest phase card: read `latest_card_path` in `CURRENT_STATE.toml`
- Current docs policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Restart mirror: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Thin dashboard: `docs/development/current/main/10-Now.md`
