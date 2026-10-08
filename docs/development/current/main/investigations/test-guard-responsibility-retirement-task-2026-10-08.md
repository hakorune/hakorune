Task: TEST-GUARD-RESPONSIBILITY-RETIREMENT-D0
Status: accepted follow-up queue; implementation is not selected
Date: 2026-10-08
Scope: one live daily-gate family, then separately selected test/temporary-guard families
Exception: explicit user-requested tooling task, separate from the active compiler semantic slice
ParentCurrentCard: docs/development/current/main/investigations/repo-hygiene-audit-punchlist-2026-09-15.md
Related:
- docs/development/RULES.md
- docs/tools/check-scripts-index.md (Guard Growth Rule)
- docs/development/current/main/design/mir-root-facade-contract-ssot.md
- docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md

# Test and guard responsibility retirement

The user requested task organization after observing accumulated tests and row
guards. Start with one live family and close an actual consolidation/retirement.
The active MirBuilder return-outcome slice keeps its required acceptance. Select
this tooling work at a subsequent slice boundary; it is not a new mandatory gate
for MIRBUILDER-FINAL-PIPELINE-v1. CURRENT_STATE selection stays unchanged.

The quoted approximately 9,900 test locations / 2,990 guard files / 300,000 lines
are external snapshot observations, not a current executable inventory or timing
baseline. Record current scoped evidence when this task is selected. Separate
Rust build/link time, test execution, shell checks and repeated gate invocations.
The existing 511-test receipt runs in 0.26s; this does not prove either build-time
causality or speedup from removing tests. No repository-wide ledger is required.

## First family: MIR root facade and import hygiene

Decision: borrow the existing MIR root facade contract and consolidate its two
public checks into one existing family owner, preserving each caller's coverage
and rejection behavior. Do not classify their different checks as duplicates
merely because they share a contract.

| Item | Original authority / caller / candidate |
|---|---|
| Contract | `design/mir-root-facade-contract-ssot.md` owns both guards and review commands |
| Retained owner candidate | `tools/checks/mir_root_facade_guard.sh` (236 lines at `cde5f57591`) |
| Retirement candidate | `tools/checks/mir_root_import_hygiene_guard.sh` (55 lines at `cde5f57591`) |
| Daily callers | `tools/checks/lib/dev_gate_quick_steps.sh` has separate facade/import steps |
| Import-only caller | `tools/checks/guard_rows.toml`, row `mir-root-import-hygiene`, profiles `pilot` / `quick-static` |
| CI consumer | `min-gate.yml` calls the registry inventory check; this is not evidence that CI executes the import guard |
| Public navigation | check-scripts index and facade contract; historical compatibility names have separate tombstone rules |

Retain export-allowlist and module-manifest acceptance plus all four import
rejection obligations: wildcard imports in `src tests`; semantic metadata root
paths in `src/mir src/runner`; detection-helper root callers in `src/mir`; internal
detection re-export bridges in `src/mir/mod.rs`. Do not broaden the import-only
manifest caller to full facade coverage without fixing that caller contract.

| Slice | Responsibility | Required evidence |
|---|---|---|
| TG-0 | Measure the two original checks through their actual entrypoints | Same SHA/profile/toolchain; build versus execution time; explicit checks, repetitions and caller set |
| TG-1 | Share implementation in the existing family owner | Original healthy tree and each distinct rejection retain coverage, exit status and first-failure diagnosis |
| TG-2 | Switch callers and retire the exclusive old edge | Quick and manifest callers switched; old executable caller-zero; exact delete set inside this family; old script physically absent |

TG-1 may use bounded roles in the existing owner if import-only callers require
their narrower coverage. Equivalent behavior is currently unproven. Fix the
source contract and acceptance before implementation; successful new tests and
caller-zero are later cutover/deletion evidence, not construction prerequisites.
Keep TG-1 behavior-preserving and separate from semantic compiler changes.

TG-2's candidate code delete set is only the old import script. Update its live
contract/review/index references and registry/quick callers in the same series.
Keep the frozen compatibility block byte-stable. At physical deletion, register
the retired name in `tools/checks/manifests/guard_navigation_tombstones.toml` with
its owner, successor and reopen trigger. Classify historical references separately
from executable caller-zero under that existing navigation-tombstone contract.
Retain shared allowlists, module manifests, runner helpers and other families.

Close the family only after the required checks pass, old executable callers
are zero, the old script is deleted, and scoped script count/LOC actually falls.
Record before/after costs without promising a speedup or lower step count before
measurement. Use the existing row runner and gates; a new deletion-proof guard
is not part of this task.

## Following families: select one from evidence

- Temporary row guards: sample the quoted source-row and physical-negative
  guards, attribute their live callers and preserved obligations, then replace
  completed-row/pointer or exact-spelling pins with an existing reusable owner.
  Historical selection requirements are not current semantic acceptance.
- Rust tests: choose one owner family, map tests to distinct language/ownership/
  cleanup/backend/rejection rules, reuse its fixtures and parameterize equivalent
  cases. Delete only superseded cases with the same required coverage retained.
  Fewer `#[test]` names alone is not a compile-time improvement receipt.

These are follow-up candidates, not an approved bulk delete set. Keep durable
positive and negative contracts, independent final-MIR checks and known-red
visibility. Record the selected family's owner, caller switch, exact retirement
set and acceptance before changing it. Existing LEGACY-TESTS-RETIRE-R0 remains
landed; parked guard-family/quick-latency lanes retain their own selection rules.

## Documentation closeout

This registration owns this card and its one hygiene-queue link only. Pointer
guard and diff/link checks validate registration. Runtime measurement, guard
consolidation, test deletion and performance improvement remain future work.
Registration checks passed: current-state pointer guard, `git diff --check`,
required metadata/parent link/card cap and protected seven-file hash comparison.
Read-only operation_design review corroborated callers, preserved obligations,
scope and the compatibility/tombstone boundary.
