---
Status: Landed
Date: 2026-04-27
Scope: Stage-B disabled FuncScan harness cleanup
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-439-stageb-output-boundary-helper-card.md
  - docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md
  - lang/src/compiler/entry/compiler_stageb.hako
---

# 291x-440: Stage-B Disabled FuncScan Harness Cleanup

## Goal

Remove the disabled `HAKO_STAGEB_FUNCSCAN_TEST` no-op harness from the Stage-B
entry.

This is BoxShape-only. It removes an entry-local detour and does not change
`BuildBox` source -> Program(JSON v0) authority.

## Inventory

The live `HAKO_STAGEB_FUNCSCAN_TEST` behavior was:

```text
HAKO_STAGEB_FUNCSCAN_TEST=1
  -> print disabled message
  -> clear depth guard
  -> return 0
```

That path did not execute FuncScanner and did not validate Program(JSON v0).
It was a silent success detour inside the Stage-B entry.

Exact live-use check found no active tool/smoke dependency. Remaining mentions
outside current docs are historical roadmap/archive references.

## Implementation

- Remove the disabled env block from `StageBDriverBox.main(...)`.
- Let `HAKO_STAGEB_FUNCSCAN_TEST=1` fall through the normal Stage-B compile
  path if it is accidentally set.
- Keep `FuncScannerBox` itself untouched; this card only removes the entry
  no-op harness.

## Verification

```bash
rg -n 'HAKO_STAGEB_FUNCSCAN_TEST' lang/src/compiler/entry tools
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS. The `rg` live-use check produced no output.
