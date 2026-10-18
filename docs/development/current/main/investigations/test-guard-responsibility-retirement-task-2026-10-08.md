Task: TEST-GUARD-RESPONSIBILITY-RETIREMENT-D0
Status: selected TR-1..TR-6 batches verified; wider inventory unselected
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

The user requested another bounded cleanup on 2026-10-19. TR-1..TR-4 below
are already landed; do not redo them or treat their old consultation stop as
current. The active compiler Count WIP remains unfinished and must be preserved.
The no-growth rule applies now. At the next safe handoff, select TR-5 in
CURRENT_STATE before further compiler construction; TR-6 follows. This is a
user-requested maintenance interruption, not a dependency on MirBuilder convergence.
Do not start a new compiler series first. Implementation of these new tasks is
not claimed by this documentation update.

The quoted approximately 9,900 test locations / 2,990 guard files / 300,000 lines
are external snapshot observations, not a current executable inventory or timing
baseline. Record current scoped evidence when this task is selected. Separate
Rust build/link time, test execution, shell checks and repeated gate invocations.
The existing 511-test receipt runs in 0.26s; this does not prove either build-time
causality or speedup from removing tests. No repository-wide ledger is required.

## Initial selected Decision (TR-1; now verified)

TR-1 is selected after explicit goal resume. Source authority is the SAME
`generic_i64/typed_object.rs` compiled twice through identical inherited helpers.
The direct module remains owner; delete only the nested two-line registration.
Existing immutable known-red inventory lists both old names as a historical
full-suite receipt; do not silently rewrite its hashes/results from a focused run.
The current compiler WIP already differs from that historical full inventory.
Unfinished return: `MIRBUILDER-GATE1-LOOP-AFTER-TEXT-READ-S0`, source-to-final
Loop graph and both-final acceptance in `mirbuilder-gate1-static-text-input-s0-2026-10-17.md`.
No compiler code or test semantics change in TR-1. Reuse source-equal935D as
baseline (34/34 PASS,0.02s), then one rebuilt family run and exact name delta.

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
are zero, the old script is deleted, and scoped script count actually falls.
Report LOC separately, including any small shared-dispatch overhead; do not add
wrappers or weaken checks to meet a numeric deletion target.
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

Historical status before TR-1 execution: it was the first requested maintenance
task after consultation. The current selection is described at the top and below.
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

## TR-1 closeout / TR-2 selection

TR-1 removed only the two-line nested registration; direct three tests and all
unique obligations remain. Source-equal935D baseline:34PASS/0.02s. Root936A
`cargo quick-test-fast --no-run --message-format=json`, jobs4:61.828s PASS;
8MiB stack, one thread, family31PASS/0.02s. Exact full `--list` difference is
only the three nested duplicate names (9267→9264); no added names. No speedup
claim. Worker independently confirmed module/helper equivalence and no live
filter callers; historical full-suite red snapshot is preserved, not rebased.
Evidence936A/935D remains under /tmp/mirbuilder-required-forward-resume until
this maintenance closes; commands/results above remain in Git after cleanup.

TR-2 selected: reuse existing `tests::final_source` in loop_break_source_tests
and physical_header_tests; delete their two byte-identical16-line helpers and
unused parser imports. New tests/helpers/files:0. Default parser config,
macro-disable scope and SourceBacked refusal are identical; resolver sessions
93/194 remain local. Existing tests:: group is the shared helper's other caller.
936A baseline:loop9PASS, header5PASS; shared-caller group20PASS/1 existing red
(to be compared by exact failure before closeout). Unfinished Loop compiler WIP
stays retained in its original card; no production connection/completion claim.

TR-2 closeout: two duplicate parser helpers removed; test Rust net-34 lines,
no new helpers/files/test cases.936B build61.539s PASS. Same 8MiB/one-thread
filters:loop9PASS/header5PASS/shared20PASS+1 unchanged baseline failure,
`top_level_and_dynamic_candidate_share_one_complete_package_batch`, same
`ordinary-new/source-sum/caller-contract`. Full test-name inventory unchanged.
This refactor does not fix that compiler-source contract red or claim suite green.
Worker confirmed exact env/config/order equivalence. Evidence936A/B retained
until maintenance closure; no new baseline, checker or safety waiver.

TR-3 selected next: same facade owner receives existing four import checks;
default preserves export→module→import order, registry retains import-only scope.
Before change, facade exits1 at existing non-allowlisted ArrayReadSiteId (.039s);
import hygiene PASS(.096s). Preserve this known red; do not expand the allowlist.
For independent equivalence evidence use disposable tree fixtures, including a
healthy facade and each original failure, without committing tests/guards.
Update facade contract, live cleanup command/index/registry/quick callers and
existing navigation tombstone; this expands docs only where owned facts change.

## TR-3 closeout

Old import script physically deleted; same facade owner now holds all checks.
Quick calls one combined entry (two before); registry retains its row ID/profiles
with `--imports-only`. No new executable/test/support file. Shell scripts2→1;
combined owner shell LOC291→294 (+3 for shared mode dispatch), quick step file
net-1 line. This reduces entries/duplicate invocations, not measured Rust build
cost. No LOC or speedup improvement claim for this guard slice.

Disposable fixtures: exact import stdout/stderr/exit match old owner for healthy
and each of four refusals; combined default accepts healthy and rejects distinct
export/module mutations. Real import-only and actual registry quick-static row
PASS. Real default retains original ArrayReadSiteId allowlist failure unchanged.
`guard-surface-inventory --check` PASS after staging deletion (its git-files view
cannot inspect a removed unstaged file). Registry L0 vs961d7dd0ef PASS:0 new
unmapped/0 mapping loss/0 dangling navigation; frozen index compatibility block
unchanged, old name retained only as historical navigation/tombstone.
Full inventory guard has44 errors for unmapped hako_alloc closeout wrappers;
exact same44 errors reproduced from archived961d7dd0ef. No rebaseline/waiver.
Pointer, bash syntax and diff checks PASS. Disposable scratch trees removed;
936A/B evidence kept until TR-4 decision and maintenance closeout.

## TR-4 selected retirement and batch closeout

Read-only review found no completed-row guard with live gate callers in the
current source family. Do not delete enduring gate checks to satisfy that shape.
Select instead the completed manual split entry
`raw_invocation_source_transport_classifier_split_guard.sh`: owner card landed,
no executable callers, index-only public entry. It pins an obsolete parent `.rs`
path (current owner is a directory) and duplicates existing classifier execution.
Source classification remains owned by the same Rust classifier; retained
existing classifier6 and transport13 tests PASS on source-equal936B,8MiB/one
thread. Remove only this47-line shell entry and index row;
keep all Rust tests, classifier and directory owner. No successor shell wrapper.
General size hygiene stays in RULES. Historical landed card stays historical.
Unlike TR-3, this name is absent from the frozen compatibility index. Existing
navigation validation rejects a tombstone for such a name, so no tombstone or
new compatibility entry is added. Retained Rust tests are the semantic successor.

Selected batch TR-1..TR-4 is complete after physical deletion and navigation
checks; this is not a full repository cleanup or whole-goal completion. Commits
6449fadb6f (duplicate registration) and961d7dd0ef (fixture consolidation) retain
Rust evidence. TR-3 removal b446ed7a3a and integration db797b1d04 are one series;
b446 alone is not a usable closeout revision (caller update followed immediately).
Return CURRENT_STATE to the retained Loop/Text row after this commit. Full goal
remains active and all unfinished source-to-final gates remain owed.936A/B and
935D pins stay for compiler baseline reuse; disposable guard scratch was removed.

## Next requested batch: TR-5 / TR-6

Owner: MIR test/guard maintenance. Use this existing card; do not create another
inventory, checker, proof app or permanent reporting script. Switch the current
row/card here when execution begins and retain the unfinished FIELD-ARRAY-READ-S0
handoff in the app-bundle card. Return to that exact unfinished row after cleanup.

| Order | Bounded scope | Required actual result |
|---|---|---|
| TR-5a | `src/mir/resolved_semantics/home_prefix_source_loop_tests.rs`, its test-only re-exports and existing caller in `ordinary_new_initializer_expression_tests.rs` | Check the asserted source/Recipe obligations against the existing end-to-end Loop case. Consolidate genuinely duplicated assertions/setup; narrow unnecessary test-only re-export chains and retire a helper only when its necessary obligations are retained. Do not delete authority/identity/cleanup checks solely because they are internal. No replacement hook/test file. |
| TR-5b | Current field-array read family: `src/mir/compiler/normal_default_pipeline/published_backend_view/array_i64_field_call_tests.rs` and `src/mir/normal_callable_semantic_package/ordinary_new_borrowed_formal_result_pending_tests.rs` | Review current added cases against existing source/result/publication evidence; consolidate a proven equivalent case or fixture and remove its redundant setup/assertions/helper. Preserve independent source-authority, final-MIR and external execution boundaries, Get element/index proof and Count empty/nonempty behavior. No new test names/files/hooks. |
| TR-6 | Existing `tools/checks/k2_wide_phase296x_array_slot_nativedirect_lowering_readiness_inventory_guard.sh` and `tools/checks/k2_wide_phase296x_array_slot_nativedirect_lowering_readiness_refresh_after_constructor_guard.sh` | Check their real gate/registry/manual callers and contract content. Merge only equivalent enduring checks into an existing owner; delete an obsolete completed-row pin/entry and switch its actual callers. If not equivalent or still required, record that finding and select one adjacent obsolete entry, rather than weaken coverage or add a wrapper. |

For each task inspect only this family. Record a short removed→retained mapping
and scoped before/after diff in this card, including test code/parameter rows
rather than only `#[test]` counts. No blanket deletion target and no duplicate
negative cases added to prove the deletion. Existing focused tests and actual
entrypoint execution are sufficient evidence; build/link and execution costs
remain separate. Known failures retain their original classification.

The audit's helper counts are historical leads. A current narrow search found
no `fn integer` / `fn variable` definitions in NCSP test-named Rust files; do not
create a shared AST framework merely to satisfy that stale candidate. Prefer
actual duplicated setup in TR-5, and delete a hook/helper with its last caller.

Completion requires a real consolidation/removal and retained necessary coverage;
this task registration is not cleanup completion or a build-speed improvement.

### Follow-up on the Loop helper and milestone checks

Current source inspection: `home_prefix_source_loop_tests.rs` is included only
under `cfg(test)` in `home_prefix_source_loop_ports.rs`. It defines the assertion
helper, reached from one call site in `ordinary_new_initializer_expression_tests.rs`
through test-only re-exports in `home_prefix_source_loop.rs` and `home_new_prefix.rs`.
This is not evidence of a new production callable API. Its substantial internal
assertions and re-export chain still make it the first TR-5a cleanup target;
pre-policy creation does not exempt it from review. No deletion/equivalence is
claimed before mapping its source, Recipe and ownership obligations.

At each maintenance-batch or series closeout, use existing scoped searches/diff
and a compatible built binary's `--list` to compare test registrations, fixture
parameter additions, test-code changes and checker entries. An unchanged test
count alone is insufficient. Do not rebuild just to count, add periodic automation,
or establish a new inventory service. Record the short result in this card.

## TEST-GUARD-RESPONSIBILITY-RETIREMENT-TR5A selected Decision

Previous goal turn made documentation progress; this turn begins the requested
maintenance at a confirmed job-free handoff. Count WIP is preserved unchanged.
939E build PASS (142.845s); existing Loop source/final test1PASS and field-result
test1PASS, publication test1FAIL. Its compiler gap remains unaccepted in the
app-bundle row, not a cleanup regression. No full-suite green claim.

Source authority: existing original Loop fixture and its current source/Recipe/
Home tests. Worker found the helper's `check_after` branch always returns for a
successful walk, making the existing malformed-backedge tail unreachable after
the prior single-call consolidation. Remove that redundant boolean/branch;
perform the existing After checks, release their actual retained locals/loan,
then consume the sole remaining product for the existing mutation/refusal check.
No new assertion API, source walk, production Clone or fixture. Rc identity and
weak-lifetime checks remain because they enforce affine ownership/no-cycle laws.
Acceptance: reuse the existing exact Loop filter, preserve all test names and
compare unchanged Count publication failure; one rebuild for code verification.

TR-5a verified: existing helper loses `check_after` argument/branch/early return;
After ownership release now recovers the sole product via `Rc::try_unwrap` and
executes the previously unreachable malformed-backedge refusal. Existing original
and finishing consumers stay separate. Remove only the duplicate strict verifier
on the unoptimized, unchanged MIR; optimized output still gets its own verifier.
The two test-only re-exports remain because removing them would widen private
modules or move private-field assertions, adding structure rather than reducing it.
Test Rust net-2 lines; no new test/case/helper/file/checker.939E baseline Loop1PASS,
940A build57.706s PASS and same filter1PASS/.07s (8MiB, one thread). Exact full
`--list` unchanged9264; source-result1PASS and Count publication1FAIL unchanged,
with byte-equal `count_empty` unsupported Method Call diagnostic. This red belongs
to preserved Count WIP, not TR-5a. Pointer/diff PASS; no speedup/native/full-suite
claim. Keep939E/940A binary/source/results until Count acceptance and TR-5b.

## TEST-GUARD-RESPONSIBILITY-RETIREMENT-TR5B selected Decision

TR-5a landed27ea1a2c6a; only net-2 Rust lines (the earlier net-3 entry was an
arithmetic error). TR-5b deletes the standalone field-get compile-success test:
its source/options/Ring0/env are identical to the first `ok` publication row.
The same normal pipeline's final verification/ABI/JSON retains that obligation.
Require the existing publication result to be `Consumed(())`, not merely non-Err;
otherwise compatibility could bypass the callback. No new test/fixture/hook.
Keep compile_normal for other callers and all distinct index/veto/source/Count
cases. Worker verified equivalence and no live exact-name filters.940A baseline
standalone1PASS/.04s; Count publication red is retained and compared separately.

TR-5b verified: standalone compile-only test removed; its exact `ok` source now
has one retained compile/publication owner requiring `Consumed(())`. All helper
callers and independent numeric-index/veto/source/drift cases remain.940B build
61.995s PASS; focused numeric-index2PASS/veto1PASS/Loop1PASS/source-result1PASS.
Publication still fails only on preserved WIP `count_empty`, exact same unsupported
ArrayBox.length Method Call diagnostic; sequential preceding Get ok/oob rows
passed the stricter Consumed check. Full names9264→9263, exact sole removed name
is the standalone duplicate, no added names. Cleanup-only staged hunk excludes
all Count additions in the same file; test code net-8 lines in this slice.
No new checker/case/file/hook or performance/full-suite claim. Existing939E/940A/B
pins remain for Count regression provenance; no temporary cleanup adapter.

## TEST-GUARD-RESPONSIBILITY-RETIREMENT-TR6 selected Decision

Worker confirms both named readiness shell entries have zero executable gate,
registry and CI callers. Their document paths are obsolete after archive moves;
refresh also pins past CURRENT_STATE row367. Delete only these111 shell lines.
Keep allocator readiness inventory tool and its two live semantic-smoke callers;
it can launch release Cargo even with --mir-json, so do not run it for cleanup.
Constructor public/default/direct lane behavior remains in existing six Python
cases. System Python lacks llvmlite and .venv interpreter requires unavailable
GLIBC_2.38; system Python plus existing .venv site-packages runs all6PASS/.002s.
No package install, env-file mutation, new tests or release build.

Frozen index names stay byte-stable. Add two entries to existing tombstones:
current pointer guard succeeds historical selection navigation only; constructor
semantics stay in Python tests and optional measurement stays with allocator tool.
Do not claim pointer guard implements those meanings. No successor shell wrapper,
caller switch or new validator. Full inventory baseline has44 existing unrelated
unmapped hako_alloc closeout wrapper errors; compare exact errors, no waiver.

TR-6 closeout: two scripts physically removed (-111 shell lines), no replacement
entry/test/tool. Two existing-format tombstones (+18 metadata lines) preserve the
frozen compatibility names and explain navigation-only successor. Existing tool
and its two live direct callers retained unchanged. Constructor6PASS; pointer,
diff and navigation PASS (0 dangling). Comparative registry vs4a4575185b PASS:
0 new unmapped/0 mapping loss, eligible2746→2744. Full inventory retains exact
same44 unrelated baseline errors; no rebaseline/waiver. Entire public index is
byte-unchanged. No Cargo/release measurement was run for guard cleanup.

TR-5a/5b are27ea1a2c6a/4a4575185b; selected TR-6 batch now complete, not full
repository hygiene or MirBuilder completion. Return to preserved
MIRBUILDER-GATE1-FIELD-ARRAY-READ-S0 with unchanged Count WIP, root939E/940A/B
receipts retained for that unfinished series. Batch adds zero tests/checkers/
helpers/files; removes one duplicate test registration (9264→9263), net-10 Rust
test lines and two shell entries. No compile-speed improvement claimed.
