# MirBuilder authority cleanup — confirmed tasks and swe-2 handoff

Status: Design/task handoff; implementation pending, not a replacement active lane.
Date: 2026-10-06
Scope: OBJECT-FIELD-READ-S0 closeout, field census contract correction, bounded owner cleanup.
Related: ../CURRENT_STATE.toml; ../../../RULES.md;
  mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md;
  mirbuilder-field-array-get-i64-result-d0-2026-10-06.md.

## Selection and evidence

The user requested confirmation, task recording and handoff to swe-2.
CURRENT_STATE continues selecting MIRBUILDER-GATE1-OBJECT-FIELD-READ-S0,
work_mode=fast. This document queues bounded follow-up work; it does not
activate parked lanes or claim Gate 1 / the whole goal complete.
Two read-only workers audited the source; the primary rechecked the findings.

Audit snapshot: /tmp/hako-field-array-get-i64-S0-checkout,
HEAD a46206eaa7f0403520a2f0d0616325d96b34281c plus S0 WIP.
At recheck this checkout is detached HEAD, not an attached branch;
intended publication branch is codex/birth-definition-publication.
Shared main checkout is behind and contains independent mixed staged/unstaged
changes. Do not reset, stash, copy the entire tree, or blindly fast-forward it.

Confirmed source findings at that snapshot:

| Finding | Source owner / consequence |
| --- | --- |
| Field census skips noninteger/weak/failed source rows | array_i64_fields.rs:281–321 contradicts its ambiguity-rejection contract; mixed same-name declarations can return true |
| Static targets/results built twice | qualified_static_call_claim.rs:60–67 and builder/normal_script_direct_static_lookup.rs:51–65 verify the same declaration catalog |
| Construction owner exceeds limit | instance_construction.rs is 950 lines in HEAD; ordinary_new_coseal_issue.rs is 796 in HEAD, 867 in S0 WIP |
| Repeated class child seal/register | ordinary_new_coseal_issue_source.rs local-new/result-new paths independently call owned_field_children_of before registering the same class |
| Repeated destruction predicate | ordinary_new_root_instance_call.rs preflight explicitly mirrors ordinary_new_local_commit/local_entry.rs::end_available |

These establish contract/maintenance issues, not an observed runtime miscompile
or a measured performance regression. Preserve independent MIR/wire verification.
Nested source child memoization and generated per-class C teardown are existing
good owners; reuse them rather than introduce another pipeline or generic cache.

## Execution order and scope

1. Resume S0 validation without treating the reported probes as complete evidence.
2. Before publishing S0, resolve source-size violations in separate BoxShape
   commits. Preserve the semantic WIP and isolate it from behavior-preserving
   splits; do not publish an intermediate selected source file above 800 lines.
3. Close and publish S0 using its existing active-card contract.
4. Select FIELD-CENSUS-CONTRACT-S0 in CURRENT_STATE/active card, then implement
   the correction below. This is a separate acceptance-boundary change.
5. Select STATIC-CATALOG-SHARE-D0 before sharing catalog issuance; settle its
   lifetime/error-order mapping first, then execute the bounded I0.
6. CHILD-INVENTORY-SHARE and TEARDOWN-CAPABILITY-SHARE are separate follow-up
   BoxShape slices. Neither is a prerequisite for S0 or Gate 1 acceptance.

Do not pause for per-slice user approval. If a follow-up mapping is not closed,
record the specific Decision and solve it under RULES; do not infer new authority.
No global solver redesign or unmeasured performance work is selected here.

## OBJECT-FIELD-READ-S0 handoff

Implemented WIP: borrowed me object-field local read selects verified walk;
Alias/null Eq and Ne classify as Bool; published JSON and C V2/V4 accept and
emit both predicates. me.nope remains rejected. If/loop, stored-child method
calls, read-on-read chains, arbitrary Bool result ABI and Gates 2–4 remain out.

Actual status has eight modified files: five implementation files from the
reported handoff plus three test files. Five new test functions exist but are
unexecuted in the handed-off evidence. No final S0 commit is recorded.

Reported receipts, not independently rerun in this documentation task:
- Release build was before the final changes; it is not current-WIP build proof.
- /tmp/ofr_s0.hako and /tmp/ofr_s0ne.hako reportedly produce lifecycle-v4-measure
  result=ok, EXE output and exit 0. Their return 0 tests compare/read execution,
  not a Bool return ABI. Retain that scope and record binary/source freshness.
- me.nope reportedly rejects instruction-unsupported.

Run only one top-level Cargo at a time. The installed libtest runner's
--help explicitly accepts [FILTERS...] and matches any supplied filter;
the reported multi-filter invocation is valid. The earlier contrary handoff
statement was corrected after checking the actual executable. The separate
commands below are an optional way to record each pin, not a required restart.
Default to quick profile and at most four jobs for new focused invocations:

```bash
cd /tmp/hako-field-array-get-i64-S0-checkout
export CARGO_TARGET_DIR=/mnt/workdisk/hako-field-array-get-i64-target
export CARGO_BUILD_JOBS=4
cargo test --profile quick -p nyash-rust --lib local_field_read_claims_object_alias_on_entry_receiver
cargo test --profile quick -p nyash-rust --lib me_object_field_read_stays_fail_closed_without_declared_field
cargo test --profile quick -p nyash-rust --lib field_alias_null_equality_claims_both_predicates_and_orders
cargo test --profile quick -p nyash-rust --lib field_alias_compare_rejects_non_null_operands
cargo test --profile quick -p nyash-rust --lib me_object_field_read_publishes_object_get_and_null_compare
```

Require nonzero matched tests and classify every red. Then add the bounded
apps fixture/smoke and run the active card's focused local-field/scalar/
publication/construction regressions and guards. For EXE repro the handoff
reports NYASH_DISABLE_PLUGINS=1 and --emit-exe-nyrt pointing to
/mnt/workdisk/hako-field-array-get-i64-target/lifecycle-kernel/release.
Rebuild the matching compiler/shim when changed; do not reuse a stale probe
binary as proof of new code. Use release only when the EXE contract requires it.

Closeout: verified pins + reproducible eq/ne EXE + missing-field negative,
no unclassified reds, <=800-line selected source files, contract docs and
active-card evidence. The Gate-1 card is exactly 1000 lines at this snapshot;
compress completed history into commit references before adding receipts,
retaining live contracts and open obligations. Stage only scoped changes,
publish the intended branch after checking its current tip, and reconcile
main's overlapping changes explicitly instead of overwriting them.

## Task: FIELD-CENSUS-CONTRACT-S0

Decision: count all same-name declarations, not only successful integer matches.
Source authority + canonical issuer: existing ordinary-box source coverage and
constructor-backed field definitions; coverage_unique_i64_field is the issuer.
Non-authority: opaque formal class guesses, caller constants, MIR observations,
C defaults and same-name numeric declarations after filtering other types.
Fail-fast boundary: zero/multiple same-name declarations, weak/untyped/noninteger
sole declaration, duplicate declarations or unavailable source-definition proof
must not grant the field's integer authority. Distinguish an unrelated box with
no such field from an unreadable box whose contents cannot be established.

Target callers: formal_i64_index_field and the existing borrowed-result census
consults; fix the shared helper rather than add divergent consumer classifiers.
Replace: .ok().flatten()/continue erasure of disqualifying declaration evidence.
Acceptance: existing unique integer positive and duplicate integer negative;
add mixed integer/noninteger in both declaration orders, weak same-name collision,
missing declared type and source-unavailable cases at the actual issuer boundary.
Run the Array field/result consumers and their existing regression guards.
Update the owning README/contract with the implemented rejection rule.
Non-claims: no inferred formal class, new accepted field families, wire changes,
unannotated storage inference or runtime-miscompile claim.

## Task: CONSTRUCTION-OWNER-SPLIT-R0

Decision: preserve the existing construction issuer, extracting private children
for provider/actual admission and ordered store proof. No new authority or route.
Target caller: issue_construction_plan; also extract S0's me-field readiness
consult from ordinary_new_coseal_issue into the existing private source owner
when that is the appropriate responsibility boundary.
Replace: the oversized in-file responsibilities, not public contracts.
Acceptance: focused constructor/provider/nullable/field-call positives and
negatives retain rejection boundaries and source order; selected source files
are <=800 lines, with a useful split before 760. Isolate semantic S0 hunks.
Before deleting helpers, switch callers, verify parity and exact caller-zero.

## Task: STATIC-CATALOG-SHARE-D0 -> I0

Decision to settle: one pre-effect source-catalog scope issues three distinct
owned projections: qualified claim index, Script lookup and result-publication
owner. Reuse existing verified imports/targets/results and declaration branding.
Target callers: QualifiedStaticCallClaimIndexV1::issue and
ScriptDirectStaticCallLookupIssuerV1::issue in the normal production lifecycle.
Replace: only the second same-source target/result construction.
Keep publication consumption, neutral-window checks, affine ownership and final
physical validation. No global cache or catalog-borrowing self-reference.
Acceptance: qualified provider/static calls and Script/App production evidence;
foreign imports, wrong target/arity/result and jointly invalid input retain the
documented pre-effect error precedence. No old-route retry. Prove the selected
duplicate calls gone before deletion; unrelated catalog callers stay intact.

## Separate bounded cleanup follow-ups

CHILD-INVENTORY-SHARE: within one branded source cohort, share class seal/register
for local-new/result-new using owned_field_children. Check exact source/object
correspondence before reuse; retain per-site errors, field order, recursive
declines and failed-seal distinction. Tests: repeated class across local/result,
nested/self fields, invalid cycle/source mismatch and Normal/Fault cleanup parity.

TEARDOWN-CAPABILITY-SHARE: share only the source-backed destruction capability
predicate (Plain / owned with sealed children / unavailable) between root-call
preflight and local commit. Keep emission progress and prefix/exit availability
with each consumer. Tests: missing child inventory rejects, owned children
release in order, not-yet-emitted local stays unavailable, selected root-call
Normal/Fault parity. No aggregate end_available unification.

Each follow-up requires its own active selection, exact replacement edge,
focused acceptance and scoped retirement evidence. Completion here remains
pending until implementation receipts are recorded by the implementing owner.

## Live validation observation (goal continuation)

The previous documentation turn made progress by recording scoped tasks.
On resumption, Cargo PID 47246 and rustc PID 58120 were confirmed live with
/proc/47246/cwd pointing to the S0 checkout. They run the reported release
multi-filter test command. Do not launch another Cargo, edit compiled source,
or restart because output has been quiet. Recheck these process handles and
recover the exit/test output before accepting the five pins. At this observation
no terminal result exists, so validation remains pending.

## S0 size prerequisite — exact read-only extraction mapping

A read-only worker checked current HEAD plus WIP while Cargo remains live.
Extracting only the new 71-line me-field trigger leaves the cohort issuer
around 805 lines after the helper call/module declaration, so that is not
sufficient. Use a private completion-walk-trigger child containing the existing
has_map predicate and the new has_me_object_field_read predicate. Keep their
calls at their original positions and in their original order; retain the
existing source field-read issuer and error propagation, with no second type
classifier. Estimated parent ~794 lines; verify rustfmt output, not estimates.
This is private responsibility movement, not a new Recipe/admission family.

For instance_construction.rs, extract only the ASTNode::New RHS provider body
(current lines 436–646, inside arm 435–647) to a private provider child.
Leave the match arm in issue_construction_plan and call the helper with ?.
Carry the same verified input, assignment/source site, definition/object views
and static-claim loan. Preserve the built-in/user-class checks, literal argument
order, child disposition and final missing-caller error in the same order.
Do not move the following FieldAccess arm or enlarge accepted provider forms.
Expected parent ~752–766 lines; target <=760 and require <=800 after rustfmt.
Only the moved inline responsibilities are the deletion set; public types,
construction identity, Fault discharge and ordered store commit remain intact.

Implementation waits for the current Cargo to end; do not alter its source
snapshot. Validate the five S0 pins first as handed off, then perform separately
staged BoxShape moves and their constructor/field-read regressions. Semantic
S0 hunks must remain separate from the behavior-preserving moves at publication.

Use a new private ordinary_new_coseal_issue_walk_triggers child, not the
already-793-line issue_source owner. Preserve has_map's MapLiteral/formal
short-circuit evaluation, initializer order and first Alias success. Use a
private instance_construction_provider child for the provider extraction;
keep the two existing instance_constructor_semantic production callers and
public issuer signature unchanged. Follow-up filters include construction_plan_
and provider_static_call_argument (nonzero matches required), plus active-card
Map/co-seal and partial-Birth lifecycle regressions for their affected owners.

## Implementation / red classification after release completion

Release test processes 47246/58120 ended. Direct execution of the fresh test
binary selected exactly five tests: four passed, publication failed because it
required the compare operand's ValueId to equal the field-read dst. The wire
shows Copy 5 -> 6 -> 1, where 1 is the exact declared object_field_get result.
The pin now follows only Copy producers with a bounded walk and requires that
exact root; unrelated producers and cycles cannot satisfy it. Rerun pending.

Size-preserving implementation: construction provider arm moved verbatim into
private instance_construction_provider; parent 743 lines. Map and me-field
walk-selection predicates moved into private ordinary_new_coseal_issue_walk_triggers;
parent 797 lines, with Map/formal short-circuit and original call order retained.
First quick compile found seven missing-import errors from this extraction:
classified this-change failures, corrected, no PASS claimed for that run.
Current rerun handle 89838 / Cargo PID91372 logs to /tmp/hako-s0-quick-focused.log.

Scope guard location pins now follow the provider child. A pre-existing pin
expected the old typed-live expression preceding nullable-formal S1; it now
pins origin -3 plus exact owned tuple and ordinary typed-live requirements.
Next actual guard stop is HEAD physical_abi.rs=816 (unchanged by S0). This is
baseline source-size debt at a required guard, not permission to mark it PASS.
A read-only worker is mapping a bounded extraction; do not lower the limit or
exclude this owner from the guard. Keep shape moves separate from S0 semantics
when staging; do not publish unverified helper migrations.

Required scope-guard baseline split mapping (worker-confirmed): physical_abi.rs
moves issue_exact_numeric_checks and issue_diagnostic_sites into private child
physical_abi_row_obligations.rs, using pub(super) helpers and parent-owned
products/fault prefix. Keep the two traversals separate, exact row coordinates,
sequential site order and checks-before-skipping non-FieldSet contracts.
Parent should be ~730 lines. Keep invocation order diagnostic-sites then exact
numeric checks unchanged. Focused existing filters:
repeated_physical_compiles_retain_function_and_diagnostic_order,
numeric_capability_rejects_drift_in_referenced_physical_layout,
diagnostic_finishing_accepts_the_same_optimized_pair_cleanup,
and exact_numeric_backend_capability::lifecycle_tests. Apply only after the
live quick compile snapshot is done; do not expand source or wire authority.

Current verification update: /tmp/hako-s0-quick-focused.log finished successfully:
40 tests passed, including the five S0 pins, constructor/provider and scalar
families. physical_abi row-obligations extraction is implemented (parent727,
child101); its expanded quick run handle93184 writes /tmp/hako-s0-quick-final.log.
C current-source temporary-shim regression passes original 3 Eq programs,
3 physical Ne variants and seven malformed-row negatives; scalar regression
passes 16 cases. The former Ne-rejection negative now rejects unsupported slt,
consistent with the deliberate S0 wire predicate extension.

Scope guard's next baseline stop: unchanged HEAD root_catalog_lifecycle_tests
is1351 lines. Worker found this is the only remaining >=800 statically named
Rust owner in the selected scope guards. Pending pure test split: move the
contiguous instance-value family (lines734–1351, all eight tests and comments)
into normal_default_root_catalog_instance_value_tests.rs; register it inside
existing lifecycle_tests via #[path] private mod instance_value_tests. Child
borrows existing parent helpers/types, no production owner or helper visibility
changes. Parent~736 / child~625. Keep both files in the guard's size inventory.
After current Cargo ends, apply and run normal_default_root_catalog_lifecycle_tests
plus current S0/ABI filters, checking nonzero matches and classifying any red.

ABI-expanded run finished43/43 PASS (/tmp/hako-s0-quick-final.log). Its
exact_numeric_backend_capability::lifecycle_tests filter matched zero because
the actual module path includes lifecycle::tests; do not count that component
as verified. Current final run uses the corrected path and broader physical
ABI tests plus the moved root test family, handle67747, log
/tmp/hako-s0-closeout-tests.log. Root harness split implemented735+628 lines,
all eight tests retained. Qualified-route scope guard now PASS (exit0,
[mirbuilder-qualified-route-scope] ok), without excluding old/new owners.
Final run is pending; no commit/publication of these shape moves yet.

## Verified closeout update (2026-10-06)

The preceding live-handle/pending entries are historical. Final quick run
`/tmp/hako-s0-closeout-tests.log` ended successfully: 67/67, including all eight
moved root instance-value cases and the corrected numeric lifecycle filter.
Scope and pointer guards pass. Four BoxShape commits are separate from S0:
`c36b9095d3`, `861fc524d7`, `5e5f80a759`, `841d66f510`.
Fresh CLI build finished quick in 3m54s; current C shim build also succeeded.
The repository eq/ne/missing-field smoke passes all three cases with the fresh
CLI and explicit current shim. Return-0 apps prove route admission/artifacts;
physical predicate results are covered by the independent C execution tests.
No remaining this-change red is known within the selected acceptance.
Next: publish the verified S0 commit, then select the required app frontier
through the current card; optional audit tasks do not become goal prerequisites.
