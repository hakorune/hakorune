Task: TEST-GUARD-RESPONSIBILITY-RETIREMENT-D0
Status: user-requested next maintenance task; execution waits for consultation to end
Date: 2026-10-19
Scope: ordered duplicate-test retirement, one fixture family, then existing guard consolidation
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
The unfinished MirBuilder Loop series keeps its WIP and acceptance. The user
requested this maintenance next on 2026-10-19: execute TR-1 first when consultation
ends, before resuming compiler construction; then one family at a time in the
order below. This is not parked or dependent on MirBuilder convergence. Do not
resume Cargo or compiler work during the consultation stop. At execution, switch
the current row/card to this task and retain the unfinished Loop handoff. This
documentation request does not itself resume the paused implementation goal.

The quoted approximately 9,900 test locations / 2,990 guard files / 300,000 lines
are external snapshot observations, not a current executable inventory or timing
baseline. Record current scoped evidence when this task is selected. Separate
Rust build/link time, test execution, shell checks and repeated gate invocations.
The existing 511-test receipt runs in 0.26s; this does not prove either build-time
causality or speedup from removing tests. No repository-wide ledger is required.

## Ordered execution and completion contract (2026-10-19)

Owner: MIR test/guard maintenance, with each existing semantic family retaining
its meaning. The supplied 9,672 tests / 40万 test lines / 3,389 guards and growth
counts are external audit snapshots, not independently recounted current totals.
Do not start a repository-wide recount or add a checker to enforce this policy.

| Order | Bounded task / exact first scope | Completion evidence |
|---|---|---|
| TR-1 | `global_call_route_plan/tests/generic_i64`: remove the nested `core.rs` registration of `typed_object.rs`; retain the direct registration and its three cases | Search live filter/caller references; preserve three unique obligations; old duplicate names absent from rebuilt test list and retained cases PASS. No new test or checker. |
| TR-2 | `normal_callable_semantic_package`: start with existing initializer-expression/source-loop fixtures touched by current WIP; consolidate equivalent cases and copied `integer`/`variable`/literal builders | One scoped before/after diff of files, LOC, test registrations and hooks; every removed obligation maps to a retained case. Use nearest existing family support, or `src/mir/test_support.rs` for truly shared AST constructors. Retain independent source/final-MIR and meaningful negative coverage. |
| TR-3 | MIR root facade/import hygiene guards, TG-0..2 below | Switch real callers to existing family owner, preserve distinct checks, delete superseded script. No replacement checker or permanent forwarding shell. |
| TR-4 | One live temporary row-guard family from current compiler work | Verify actual gate/registry/shell callers; move enduring checks to existing owner and delete completed-row/source-spelling pins. Keep other families out of scope. |

For TR-2, reported 126 files / 6.1万 lines are starting leads, not permission for
bulk deletion. Select one contract family; do not require all NCSP tests to be
mapped before the first consolidation. Inspect same-contract internal assertions,
error-substring checks and `_for_test` hooks; migrate/remove those made redundant
by source/MIR/refusal coverage. Preserve identity tests where identity is itself
an authority boundary. Do not bypass opaque production constructors to simplify
fixtures or consolidate distinct failure conditions out of existence.

Each task closes with actual removal/merging and retained coverage, not just a
list of candidates. Record build/link, execution and shell-check costs separately
when running Rust/guard checks; no speedup claim from test-name counts alone.
Use the existing card for scoped results and deletion mapping, not a new ledger,
per-slice report, guard or test. Retire unused helpers/hooks with their last caller.
Independent meaningful tests need not be deleted merely to satisfy a LOC target.

The no-growth policy applies immediately to new work, including current WIP;
it does not retroactively authorize deleting unverified WIP. Before resuming the
Loop series, review its added test cases against existing coverage and consolidate
redundancy without adding fresh refusal tests as a ritual completion condition.

## TR-3 existing plan: MIR root facade and import hygiene

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

## Remaining families: bounded selection within TR-2 / TR-4

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

## Selected-area candidate from the 2026-10-17 audit follow-up

Status: TR-1 is the first user-requested execution task after consultation;
compiler implementation remains paused and its pointer retained until that switch.
`src/mir/global_call_route_plan/tests` has54 Rust files at `93129de754`.
The reported four contract families are a review grouping, not yet a verified
coverage-equivalence partition or approved delete set. Start with an exact
registration duplicate: `generic_i64/mod.rs` and `generic_i64/core.rs` both
register the SAME278-line `generic_i64/typed_object.rs`, producing the same3
cases under `generic_i64::typed_object` and `generic_i64::core::typed_object`.
Read-only worker checked identical inherited facade/helpers, no extra local
bindings/cfg or module-path-dependent logic. V841b root `--list` confirms both.

For TR-1, reuse any compatible existing duplicate-execution receipt; otherwise
execute both original registrations from one build. Retain the direct
`generic_i64::typed_object` registration and all3 distinct cases (typed field,
object null guard, void-parameter null guard); remove only the nested registration
after checking exact filter/caller references. Rebuild once and verify retained3
PASS, duplicate names absent and family count34->31. Keep the source test file;
this deletes duplicate execution/compilation, not278 lines of independent tests.
No measured speedup, -59k/-118k target or deletion permission for other families.
Use this existing task, not a second campaign/guard or a new MirBuilder gate.

## Documentation closeout

This update owns RULES, this existing task and its current-state/workstream
scheduling references. It creates no test, guard or runtime change. Pointer
guard and diff/link checks validate registration. Runtime measurement, guard
consolidation, test deletion and performance improvement remain future work.
Historical registration checks passed: current-state pointer guard, `git diff --check`,
required metadata/parent link/card cap and protected seven-file hash comparison.
Read-only operation_design review corroborated callers, preserved obligations,
scope and the compatibility/tombstone boundary.
