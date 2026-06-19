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
CREAT-SUBSET-PILOT-SELECTION-001
```

Purpose:

```text
Inventory and select a small real creat subset pilot now that the synthetic
crate handoff and OrderedMapBox detour are closed.
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
```

Acceptance for the current slice:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
# plus the focused inventory/probe commands selected by
# CREAT-SUBSET-PILOT-SELECTION-001
```

## Task Order

1. Inventory candidate creat files/modules before adding schema nodes.
2. Count unsupported RustSubset families by reason/code.
3. Select a 2-3 module pilot slice with minimal new semantics.
4. Do not add creat-specific schema or converter branches in the selection row.
5. Update `CURRENT_STATE.toml` when the selection row closes.

Recommended next row:

```text
CREAT-SUBSET-PILOT-SELECTION-001
```

## Pointers

- Current state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Latest phase card: read `latest_card_path` in `CURRENT_STATE.toml`
- Current docs policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Restart mirror: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Thin dashboard: `docs/development/current/main/10-Now.md`
