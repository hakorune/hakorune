---
Status: implementation_partial__reference_execution_pass__exe_aot_acceptance_open
Task: M10b-I0-R0-VAR
Date: 2026-09-24
Scope: preserve the accepted callable recurrence during the R0 cutover
Parent: joinir-loop-portable-recipe-cutover0-m10b-i0-r0-2026-09-24.md
NextCard: M10b-I0-R0 (resume its acceptance and selected retirement)
Related:
  - loop-production-selection-d0-2026-09-24.md
  - ../design/fixtures/generic-loop-legacy-disposition-v1.tsv
  - ../../../RULES.md
Implementation permission: bounded callable VAR connection described below;
  no fixture downgrade, sixth node-family selector arm, or other parked family.
---

# Callable variable recurrence — accepted-input preservation

## Decision

```text
Decision: select option 1, connect VariableAccumRecurrenceV1 through the existing callable source owner. Preserve the accepted input and correct its erroneous CallableSingleLoopV1 attribution. This is a production-edge addition, not behavior-neutral cleanup.
Source authority + canonical issuer: installed callable source loan -> same resolver input/ledger/exact Loop membership -> issue_variable_accum_recurrence_source_attempt_v1 -> produce_variable_accum_recurrence_recipe_v1 -> existing common physical admission and canonical physicalizer.
Non-authority: fixture path, variable spelling, diagnostic output, legacy route identity, AST equality search, a second resolver run, and wire-parity receipts.
Fail-fast boundary: overlap, foreign/missing source evidence, duplicate consumption, admission/physical failure and finish residuals are terminal; never retry the old node route or erase an error into no-match.
Smallest next slice: extend the existing callable source-scope loan and exact-site consumption contract, then wire the existing VAR product into the shared physical admission/physicalizer.
Non-claims: no implementation/test PASS from this design; no whole R0/M11/M12 completion, sixth canonical family, Compatibility parity or other parked-family activation.
```

Boundary: selected normal callable source loan -> exact Loop site -> shared
physicalization -> callable tail -> existing final validation/publication.
Includes input identity, both carried values, source-row consumption and failure
cleanup. Excludes other source shapes, new arithmetic semantics and full R7.

## Why this row is required

`apps/tests/loop_simple_while_inline_explicit_step_min.hako` has two initialized
locals, `loop(i < 4) { acc = acc + i; i = i + 1 }`, `print(acc)`, `return 0`.
Its existing acceptance is output `6`, exit `0`. Keep the source and expected
result. The current execution receipt below confirms this output through the
source-backed MIR route.

The former production-selection/R0 cards attributed this row to
`CallableSingleLoopV1` and therefore excluded VAR from the selected boundary.
That attribution is withdrawn. VAR is selected for this bounded callable
profile and is now connected through the same-source callable ingress. The
five-family node selector continues to own its existing domain. The exact VAR
candidate is checked against retained callable owners before any competing
candidate is consumed; unrelated shapes retain their established dispatch.

The bounded bridge is implemented as follows:

| Existing owner | Landed connection |
| --- | --- |
| `compiler/variable_accum_recurrence_projection.rs` | Requires both initializers to be source Integer literals; focused negative tests cover String, Float, nonliteral and missing values. |
| `normal_callable_semantic_loan_port/source_scope.rs::with_callable_source_scope` | Lends the original resolved input for the callback lifetime through the scoped child port; no second resolver or escaped reference. |
| `CallableSemanticLoweringState` | Validates and consumes exact source read/write sites once, looks up entry values by BindingRef, and records both writebacks. |
| `variable_accum_recurrence_producer.rs` | Exposes a consuming decomposition of the existing `(operations, inputs)` product. |
| `compiler/loop_node_physical_admission.rs` | Callable ingress reuses common product admission and the same owner/site/frame/context/Core; no five-family winner is forged. |
| `resolved_lowering/loop_recipe_physicalizer` | Reuses the canonical physical pipeline with BindingRef-based entry values and complete writebacks. |

Paths above are under `src/mir/` or `src/mir/builder/` as appropriate.
`normal_callable_dynamic_source` owns inventory, not an input-lending API.
The existing forest bridge deliberately avoids retaining a second resolver
ledger; follow that ownership boundary.

## Selection and lifetime contract

1. Prepare against the installed callable's original input and ledger. Resolve
   membership by its exact source site. The projector currently requires five
   root statements, locals at 0/1, the Loop at 2 and two ordered assignments
   in the Loop body. Keep those bounds and existing binding/operation checks.
   The two tail statements remain under the ordinary callable owner.
   The projector currently labels inputs I64 after checking only initializer
   presence. Close that hole before production: this bounded ingress requires
   both initializers to be source `LiteralValue::Integer`, checked by this same
   source projector before Facts issue. This proves the existing Integer value
   representation, not a new annotation. String/Float/nonliteral/unknown inputs
   cannot become I64 from MIR types or current physical values. Broader scalar
   inference is a separate task; this row needs no new type solver.
2. Establish exclusive ownership before consuming a competing product or
   performing loop-specific Builder effects. Split genuine no-match from overlap in
   `NonGenericOrOverlapping`; an overlap must never enter the VAR path or
   the raw node route. Retained LoopBreak/composite/LoopCond/LoopTrue owners
   retain their established domains. A VAR candidate plus another selected
   owner is a named terminal, not a priority tie-break.
3. Preserve Candidate/Declined/Unresolved/Rejected. Coverage must describe the
   completed observation, never be set Complete merely to obtain Candidate.
   Shape decline may continue only within the established preselection
   contract, before consumption/effects. Missing identity cannot decline.
4. Transfer the selected non-Clone product once at that site. Candidate
   consumption prevents re-entry into `lower_non_callable_loop_legacy_v1`.
   Remove the selected accepted row's old dispatch edge in this R0 series.
5. Lend input within `with_callable_source_scope`'s existing callback lifetime;
   extend the child-port/scoped callback contract as needed. Do not use the
   node router's condition/body equality scan or create a new resolved unit.
6. Supply physical inputs from the callable's BindingRef/value relations.
   Diagnostic names are not binding authority. Reuse the canonical CFG/SSA/PHI
   owner and its physical pipeline; no VAR-specific emitter/PHI algorithm.
7. Record the exact condition/body reads and assignments consumed by the
   Recipe once, in the existing callable ledger. These are source sites, not
   dynamic iteration counts. Update both induction and accumulator values
   from the verified continuation; then lower print/return exactly once.
   Never mark the whole subtree consumed without matching operation evidence.
8. Success passes the existing callable finish residual checks. Duplicate or
   missing sites/products reject. Any later failure discards the unpublished
   invocation; a fresh invocation remains usable. No partial artifact or retry.

## Ordered implementation tasks

| Order | Task | Observable completion | State |
| --- | --- | --- |
| 1 | Same-source loan and selection seam | Existing scope lends exact input; site-keyed attempt consumption; no-match/overlap separated; foreign, missing and duplicate input rejected before physical effects. | Implemented; focused owner tests pass. |
| 2 | Neutral product admission | Consuming VAR product API; callable ingress reuses common demand/layout checks; both input bindings and root continuation preserved. | Implemented; lifecycle test passes. |
| 3 | Callable physical consumption | Shared physicalizer reads existing callable values, accounts for precise source operations, publishes both writebacks and leaves tail/publication with existing owners. | Implemented; base and bounded source probes pass. |
| 4 | Production acceptance and selected-edge removal | Unchanged real fixture and finite semantic variants reach the selected executable terminal; no VAR retry into old route; retained-owner regressions and failure cleanup pass. | Partial: accepted fixture and two-value variants pass MIR/EXE; fallback count is zero; accounting rejects missing entry values and duplicate read/write consumption. The selected VAR dispatch also rejects an absent callable ledger before Builder effects or fallback. Issuer-boundary terminals now cover unlocated parent source, non-loop site, and foreign expected owner; overlap and forced producer/admission terminals are unconstructible under this profile (audit below) and remain fail-closed defensive branches. |
| 5 | Return to R0 | Record tested SHA/commands and owner mapping; recheck changed delete-set callers; finish R0-required acceptance/retirement, then residual M11/M12. | Not started. |

Tasks 1-3 are one bounded construction series; source-contract changes and
their focused tests land together. Task 4 is required before R0 closeout.
New code, after-change PASS and caller-zero are outputs, not entry conditions.
If implementation exposes an unsupported semantic contract, resolve it in this
card; do not widen source acceptance or downgrade the fixture to avoid it.

## Required acceptance

- Real source parser/resolver/callable loan -> VAR -> common admission -> MIR
  -> selected backend execution: unchanged fixture prints `6`, exits `0`,
  and passes callable finish. A synthetic AST test alone cannot close this.
- Renamed bindings and supported changed bound/step literals; zero iterations;
  verify both final induction and accumulator (an accumulator-only check is
  insufficient). Keep existing numeric semantics; select finite in-range cases.
- String/Float/nonliteral or missing initializer representation cannot issue
  I64 Facts or reach physical work; both initializers are checked.
- Missing callable ledger, missing entry value, and duplicate source read/write
  consumption reach named terminals. Overlap and producer/admission rejection
  are fail-closed defense terminals, but are not constructible from this
  accepted VAR source profile: its exact two-assignment loop body is disjoint
  from the retained break/composite/simple-while owners, and the production
  producer/admission receives the same sealed product, input, and site. Do not
  fabricate conflicting facts or add a test-only fault hook to force them.
  Keep the valid-source no-overlap route witness. Wrong owner/site/frame and
  incomplete coverage are exercised at the genuine boundaries that exist:
  the callable issuer rejects an unlocated parent source, a non-loop site,
  and a foreign expected owner before membership or effects, and the
  projection-level tests already cover foreign owner, foreign scope frame,
  a different loop site, and incomplete observation. The post-issuance
  identity/coverage rechecks remain fail-closed defensive branches; they are
  not separately constructible because the attempt derives its identity and
  coverage from the same sealed input/site. Unsupported source shape has
  preselection disposition.
- Fault after partial lowering publishes nothing; the next invocation succeeds.
  Verify exact source-row accounting rather than disabling finish assertions.
- Retained LoopCond/LoopTrue and applicable LoopBreak/composite cases retain
  their result and owner; selected VAR never reaches the raw reconstruction
  dispatch. The accepted corpus row and expected-output gate stay accepted.
- Required R0 parity/guards still gate R0. Run focused checks first, one Cargo
  process, quick profile up to four jobs; coordinate with the existing worker's
  build. Follow-up CI evidence does not authorize premature cutover closeout.

Update owning README/reference for the new callable connection in the same
implementation slice. Source files: plan separation at 760 lines, remain below
800; use sibling modules by responsibility. Do not enlarge the already long
R0 card with this implementation history.

## Evidence and handoff

### Current execution receipt (2026-09-24)

The same-source callable bridge, common admission and canonical physicalizer
are now connected. A quick-profile binary built with the existing
`vm-reference` feature ran the unchanged accepted fixture through
`--backend mir`:

```text
CARGO_BUILD_JOBS=4 cargo build --profile quick --features vm-reference --bin hakorune
NYASH_MACRO_DISABLE=1 NYASH_DISABLE_PLUGINS=1 HAKO_EMIT_EXE_CACHE=0 \
  ./target/quick/hakorune --backend mir \
  apps/tests/loop_simple_while_inline_explicit_step_min.hako
stdout: 6
exit: 0
```

Temporary source probes kept the five-root-statement profile and encoded both
final values as `acc * 10 + i`: renamed bindings with bound `3` and step `2`
printed `24` (`acc=2`, `i=4`); zero iterations with `acc=7` printed `70`
(`acc=7`, `i=0`). The original fixture still prints `6`. These `/tmp` probes
were not added to the accepted corpus.

Focused Rust results after the connection: `variable_recurrence_` 10/10,
`normal_callable_loop_source` 20/20, `raw_loop_child_entry::` 12/12, and
the selected callable lifecycle test 1/1. The selected lifecycle test has a
test-only, thread-local route witness and asserts `VAR=1 / legacy fallback=0`;
this proves the accepted row switches at the production dispatch. A second
test injects failure after physicalization and exact source-row consumption,
observes a rejected unpublished invocation, discards it, then confirms a fresh
invocation of the same accepted fixture succeeds. `variable_accum_tests::`
passes 2/2. The shared raw fallback remains for unmatched non-VAR shapes and
is not a VAR-specific edge to delete. The broader
`normal_default_root_catalog_lifecycle` run was 42 passed / 6 failed. Five
failures match the pinned 2026-09-19 baseline receipt in
`mir-call-map-lifecycle-consumer-i0-2026-09-15.md` and remain
`known baseline debt`; they are not changed by this slice:

- `actual_string_helpers_general_result_row_reaches_its_first_loop_carrier`
- `parser_scan_package_passes_callable_source_handoff_without_fallback`
- `source_backed_app_main_direct_call_consumes_affine_loan`
- `source_backed_package_failure_is_terminal_before_builder_effects`
- `source_bound_static_result_owner_reaches_the_raw_terminal`

The sixth failure,
`merged_parser_program_source_stops_at_named_publication_boundary`, observed
`[freeze:contract][callable-loop/facts-absent]` where the B3 D2 test expects a
different terminal. Record this as an `informational census` for the D2 owner;
do not weaken its assertion or count it as LoopCond physicalization evidence.
The earlier declines from temporary probes with an extra tail statement were
outside the fixed five-statement profile and are not compiler failures.

The `--backend mir` output above is source-backed VM-reference evidence. On
2026-09-25, the accepted fixture and two finite semantic variants also passed
the selected published EXE route:

```text
PATH=/tmp/hako-llvm18-tools:$PATH CARGO_BUILD_JOBS=4 cargo test \
  --profile quick --features llvmlite-compat --lib \
  accepted_variable_recurrence_fixtures_reach_exe_with_final_values \
  -- --ignored --nocapture
result: 1 passed; emitted EXE outputs `6`, `24`, and `70`, each exit `0`
```

The test compiles the unchanged accepted parser/resolver/callable-loan fixture
and two finite source variants through the published callback and existing EXE
emitter, then executes each binary. Renamed bindings with bound 3/step 2 print
`24` (`acc=2`, `i=4`); zero iterations print `70` (`acc=7`, `i=0`), checking
both final values. Required tool/archive absence now fails an explicit manual
acceptance instead of silently returning success. The test uses a 32 MiB test
thread because the default Rust test-thread stack overflowed, and puts LLVM18
`opt`/`llc` first on PATH because the host's unversioned tools are LLVM14 while
the emitter resolves unversioned names first. This closes these three EXE
terminals; it does not close the remaining named-terminal matrix or broader R0
acceptance.

The callable-ledger accounting boundary was then tightened and tested:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --features llvmlite-compat \
  --lib variable_accum_accounting -- --nocapture
result: 2 passed; missing entry value and duplicate read/write consumption
       each stop at their named accounting terminal
```

`entry-value-missing` is now distinct from a mismatched/consumed read or write
site. The selected VAR dispatch's absent-ledger case also has direct evidence:
the accepted fixture is parsed and resolved, its callable handoff is projected,
and the VAR source Facts/Recipe path reaches `callable-ledger-missing` with no
Builder function/block and route witness `VAR=1 / fallback=0`:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --features llvmlite-compat \
  --lib candidate_without_callable_ledger_is_terminal_before_builder_effects -- --nocapture
result: 1 passed; exact absent-ledger terminal, no Builder effects, no fallback
```

The 2026-09-25 read-only owner audit found no source-valid way to construct a
same-site VAR/retained-owner overlap: VAR requires exactly five root statements
and a two-assignment loop body, while retained LoopBreak, composite break, and
LoopSimpleWhile owners require disjoint shapes. The production producer and
physical admission also consume the same exact source product/input/site, so a
rejection after a valid Candidate would require an implementation defect or
post-issuance corruption. Preserve these named fail-fast branches, but do not
invent synthetic authority to force actual-dispatch negatives. The existing
valid-source route witness proves VAR selection with no fallback; any remaining
owner-level verifier tests must use a genuine existing boundary. On 2026-09-25
the issuer-boundary terminals were exercised at their genuine boundaries:
`variable_accum_recurrence_rejects_unlocated_parent_source`,
`variable_accum_recurrence_rejects_non_loop_parent_site`, and
`variable_accum_recurrence_rejects_foreign_expected_owner` pass 3/3, each
stopping at `VariableAccumRecurrenceSourceUnavailable` before membership,
Builder effects, or fallback. Projection-level foreign owner, foreign scope
frame, different loop site, and incomplete observation were already covered by
`variable_accum_recurrence_projection_tests`. The post-issuance
identity/coverage rechecks in `issue_callable_variable_accum_recurrence`
remain fail-closed defensive branches: the attempt derives identity and
coverage from the same sealed input/site, so they are not constructible
without post-issuance corruption — same class as the audited overlap
negatives.

Current focused refresh on the shared worktree also passes the R0 winner-spine
tests 7/7, physical-admission tests 5/5, compatibility no-source terminal
1/1, retained source-backed LoopCond witness 1/1, and raw-loop entry suite
12/12 (including retained LoopCond/LoopTrue entry tests). The real G0
in-body-step fixture exits with its declared return value `3`; the nested
GenericLoop fixture stops at its recorded `accepted-typed-reject` terminal.
The shared tree contains the uncommitted M10b-I0-R0 route/registry retirement
draft as well as this VAR row. These focused checks do not close R0's complete
parity, fault-injection, acceptance, or old-symbol census.

### Backend-lane boundary confirmed (2026-09-25)

The `phase29bq_fast_gate_cases.tsv` row pins `--backend vm`, which is the
explicit legacy `BootstrapRustVmKeep` admission
(`VmKeepPostMacroProgramWithImports` — post-macro AST, `semantic_package ==
None`). Under that admission `Main.main` lowers through
`RawInvocationChildPortV1::new_with_cleanup_exit_policy` — no
`callable_loop_root_scope`, no callable ledger — so every loop reaches
`lower_non_callable_loop_legacy_v1` -> `route_loop` -> the node winner spine.
The VAR shape has no node-level owner by design, so the row now terminates at
`loop winner selection declined: zero selected family candidates` (exit `1`).
This is not VAR-specific: sampled LoopCond/LoopTrue gate fixtures freeze the
same way on that lane; all 40 `portable-owner` corpus rows resolve to
callable/canonical-side owners that exist only on the source-backed
admissions. Re-verified on the current quick `vm-reference` binary: the
unchanged fixture freezes under `--backend vm` and prints `6`/exit `0` under
the default `--backend mir` lane and the published EXE path. The selected
owner's acceptance is therefore the source-backed lane evidence above; whether
the vm-pinned gate row is re-pointed, re-dispositioned, or left to the
separate R0 retirement is a gate-contract decision recorded here for the R0
owner — it is outside this card's bounded callable connection and its
no-Compatibility-parity non-claim.

Update (2026-09-25, R0 gate migration): this card's VM-gate row is now
covered by the registered source-backed gate
`phase29bq_portable_owner_source_backed_gate_mir.sh` (fixture / expected
`6` / rc `0` / accepted preserved; 15/15 green on `target/quick`), and
the row was removed from `phase29bq_fast_gate_cases.tsv` after that
successor run passed. Details and the 3 held rows are in the R0 card's
gate-migration section.

Task 4's retained-owner and post-physicalization failure-cleanup checks pass;
the accepted fixture and both final-value EXE variants pass. Missing entry
value and duplicate source read/write consumption now have VAR-owner tests.
The overlap and forced producer/admission actual-dispatch negatives are
unconstructible under the accepted source profile and are not required as
synthetic tests. Wrong owner/site/frame and incomplete-coverage evidence is
now present at the issuer boundary (unlocated source, non-loop site, foreign
owner) and at the projection level (foreign owner/frame/site, incomplete
observation). What remains open for Task 4 is the broader R0 acceptance it
gates — the full parity/census run — so Task 4 is not closed.
The accepted VAR row no longer reaches the shared legacy fallback; that helper
remains needed by other shapes. Do not close global R0 or claim global
caller-zero until its separate selected delete-set and required acceptance are
verified.

Design was audited at HEAD `1aa7ac25b3` plus the shared R0 work on 2026-09-24.
The implementation and execution receipt above remain uncommitted shared-tree
changes. The accepted corpus row is unchanged; the VAR row itself removes no
global legacy route, fixture, or other family. The separate R0 route/registry
retirement draft is present in the same worktree and is not certified by this
child-card receipt.
