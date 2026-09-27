# MIRBUILDER-EXE-ACCEPTANCE-SUITE-RED-DISPOSITION-D21
## Census: post-S8 Gate-1 suite red disposition (D21)

Status: D21 census closed; D22 successor plan accepted (2026-09-27);
  field-resident Array/I64 owner design selected, implementation not yet selected
Date: 2026-09-27
Parent: MIRBUILDER-EXE-ACCEPTANCE-LOOP-CARRIER-BINDING-PUBLICATION-S8
  (landed — carrier header/final phis publish into
  `CallableSemanticLoweringState.values[binding]`; emit-mir-json green
  with 65 retained phis; `find/3` wrong-code closed)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D20 selection lineage.

## Problem

S8's named next blocker was `JsonStreamAggregator.ingest/1`'s
`loop(start < n)` declining every loop-family candidate
(`loop winner selection declined: zero selected family candidates`),
and the full `real-apps-exe-boundary` receipt needed re-classification
after S8. Census question: does any remaining Gate-1 failure sit inside
this lane's authority (callable source route -> document publication ->
MIR emission), and is there a bounded next slice?

## Fresh suite receipt (post-S8, release build)

`tools/smokes/v2/run.sh --suite real-apps-exe-boundary` — 4 pass /
7 fail:

| entry | result | first named terminal |
|---|---|---|
| typed_object_birth_min | PASS | — |
| typed_object_method_min | PASS | — |
| typed_object_birth_param_min | PASS | — |
| real_apps_exe_boundary_probe | PASS | — |
| typed_object_newbox_min | FAIL | backend `no_lowering_variant` |
| typed_object_untyped_field | FAIL | MIR emit `unsupported terminator Invoke` |
| boxtorrent_mini | FAIL | `callable-loop/route-not-front-selected` `LoopCondRouteRejected(SourceCallOutsideSelectedFamily)` `HakoAllocPage.seedBlocks/0` |
| binary_trees | FAIL | `SourceCallOutsideSelectedFamily` `BinaryTreesBench.iterationCheck/3` |
| mimalloc_lite | FAIL | `SourceCallOutsideSelectedFamily` `HakoAllocPage.seedBlocks/0` |
| allocator_stress | FAIL | `CoreMethodSource NamedArray(TextSourceMissing)` |
| json_stream_aggregator | FAIL | ny-llvmc `unsupported_newbox_type` at `new JsonStreamAggregator` (first_block=0 first_inst=3) |

## Census answers

### Q1 — which lane declines `ingest/1`, and is it a Gate-1 defect?

The decline is on the **ledger-free lane**, not the armed EXE lane.
`--backend vm` enters `VmHakoPostMacro` admission
(`compile_bridge.rs`) which builds the `Compatibility(ast)` program
root — no callable ledger is installed, so the loop lowers through
`lower_non_callable_loop_route_v1` -> `route_loop` -> winner spine
(`issue_loop_node_winner_recipe_v1` at router.rs:198 ->
`select_canonical_loop_family_v1` called at
`loop_node_winner_spine.rs:370`, defined at
`family_selector.rs:147`). All five spine
families contract-decline this shape (single `loop(cond)` with a
four-statement body holding a local decl, a nested if, an instance
call, and the carrier update):

| family | outcome |
|---|---|
| DirectAccum | `NotDirectAccumShape` |
| NestedPredicate | `NotNestedPredicateShape` |
| LoopTrue | `NotLoopTrueBreakContinueShape` |
| LoopCond | `BodyArity` |
| GenericG0 | `Structural(FunctionBodySchedule)` — nested two-loop profile only per generic-loop SSOT |

The armed EXE route (`--emit-mir-json` ->
`for_mir_mode_callable_source` -> callable ledger installed) compiles
`ingest/1` fully — S8 evidence: the module emits with 65 retained
phis and `ingest`'s phi spine is present in the MIR JSON. The
ledger-free decline is the named "non-callable Script owner"
compatibility boundary (final-pipeline SSOT: `Callable None` retains
its existing non-callable owner; `ParkedSealed__NoCallableNoneCaller`
governs branch retention) and was already a D17 recorded non-claim:
"`ingest/1` may later still be declined by the unchanged ledger-less
VM spine (Recorded Non-Claim — Gates 2-4)". It is not a Gate-1
blocker.

### Q2 — post-S8 red map: who owns each of the seven?

Zero unowned failures. Every red terminates at a named typed stop
inside an existing sealed/parked lineage — except `newbox_min`,
which is the D0/D1-confirmed environment/toolchain debt:

| app | terminal | owning record |
|---|---|---|
| boxtorrent_mini | `SourceCallOutsideSelectedFamily` (`free_stack.push` on field-read local) | D5 fork (a): B3-ArrayPush lineage — `NoSafeSlice__B3SelectedRuntimeAndOperationOutcomeAuthorityMissing` |
| binary_trees | `SourceCallOutsideSelectedFamily` (`builder.make`, `positive.itemCheck` on param/local receivers) | D5 fork (b): ordinary user-box instance calls — no coverage issuer exists; parked DeclaredInstance lineage |
| mimalloc_lite | `SourceCallOutsideSelectedFamily` (`seedBlocks`) | D5 fork (a) |
| allocator_stress | `CoreMethodSource NamedArray(TextSourceMissing)` | D5-inventoried NamedArray source-demand family (B3 vocabulary) |
| typed_object_newbox_min | `no_lowering_variant` | environment/toolchain debt — **verified**: identical-source rerun under `PATH=/usr/lib/llvm-18/bin:$PATH` (resolved `opt`/`llc` = LLVM 18.1.8) **passes**; D0 recorded `environment/toolchain debt (resolved opt too old)`, D1 confirmed `newbox_min = EnvironmentDebt`. Reopen trigger = toolchain provisioning (`opt-18` ahead of `opt` on the runner PATH), not semantic work |
| typed_object_untyped_field | `unsupported terminator Invoke` | D0 `NoSafeSlice` untyped-storage sentinel |
| json_stream_aggregator | `unsupported_newbox_type` (`new JsonStreamAggregator` — handle-typed fields, construction-ineligible per D18) | D0/D18 ordinary-new eligibility family: "box-typed fields, nested construction reclamation ... its own family" |

The callable loop route's coverage contract
(`into_selected_relation`) admits exactly two issuers —
singleton `SelectedStatic` exact targets and `CoreMethod` rows
(StringBox ops on lexical-local receivers in loops, plus
`ArrayBox.push` only under `NamedArrayConstructionRequirementV1`).
Uncovered call kinds (param/local-receiver instance calls,
non-StringBox core methods outside NamedArray scope) are
deliberately outside the selected family; the freeze is the
designed fail-fast, and widening coverage is each fork's own card —
D5 already sealed both forks family-local: "reopen only through
their own cards".

Toolchain verification note (D1 requirement, executed): with
`PATH=/usr/lib/llvm-18/bin:$PATH` the suite effectively reports
5/11 — `newbox_min` passes (`no_lowering_variant` was the stale
resolved `opt` = LLVM 14; `resolve_opt_tool` probes `["opt",
"opt-18"]` in that order, so the unversioned stale binary wins when
both exist), while `unsupported_newbox_type` and `unsupported
terminator Invoke` fail identically, confirming them as
env-independent semantic families. The env fix is runner PATH
provisioning, not a lane/code slice.

### Q3 — bounded in-lane slice?

None. Gate-1's lane authority covers the callable source route
through document publication and MIR emission; every remaining red
stops either (a) inside a sealed coverage fork the route contract
names by design, (b) past MIR emission at backend-cohort newbox
admission (`unsupported_newbox_type`), (c) in the untyped-storage
`NoSafeSlice` sentinel (`unsupported terminator Invoke`), or
(d) at the D0/D1-confirmed environment/toolchain debt
(`no_lowering_variant` — stale resolved `opt`). Per the family
scheduler the in-lane inventory is exhausted; the reds reopen
through their owning cards or the toolchain fix, not here.

## D21 Decision (narrow-lane census; external-wait wording corrected by D22)

```text
Decision: no bounded implementation slice exists inside D21's original
  route/publication lane — the
  ingest/1 family-selection decline is the D17-recorded ledger-free
  compatibility boundary (non-callable Script owner), and all seven
  EXE reds are owned by existing sealed/parked lineages or the
  confirmed environment/toolchain debt. Gate-1 remains unsatisfied;
  D22 below selects internal prerequisite design through those owners.
  Being outside this narrow lane does not establish an external wait.
Source authority + canonical issuer: suite red map -> D5-sealed
  coverage forks (B3-ArrayPush, parked DeclaredInstance),
  D5-inventoried NamedArray source demand, D0/D18 handle-field
  newbox-admission family, and the D0/D1 environment/toolchain debt
  for newbox_min; VM spine decline -> ledger-free VmHakoPostMacro/
  Compatibility owner per the final-pipeline SSOT.
Non-authority: this lane does not reopen B3/DeclaredInstance/
  NamedArray or newbox-admission cards, does not extend the loop
  coverage contract, does not weaken family selection, does not
  reclassify the newbox_min toolchain debt as semantic work, and
  does not cross VM/EXE lanes without a production caller census.
Fail-fast boundary: SourceCallOutsideSelectedFamily,
  NamedArray(TextSourceMissing), no_lowering_variant,
  unsupported terminator Invoke, unsupported_newbox_type, and the
  winner-spine NoCandidate decline all remain named terminals —
  no fallback, no silent no-op.
Smallest next slice at the D21 census: none in-lane. The D22 successor
  below now selects owner design; implementation still belongs to the owning
  records: B3-D2 (ArrayPush + NamedArray authorities), parked
  DeclaredInstance admission, D0 handle-field newbox admission
  (unsupported_newbox_type only), runner toolchain provisioning
  for newbox_min (opt-18 ahead of opt — verified green under
  PATH=/usr/lib/llvm-18/bin); a ledger-free single-loop family
  is that spine's own card.
Non-claims: gate-1 stays unsatisfied (4/11 — 3 apps + probe —
  default env; 5/11 verified under opt-18 PATH); no VM claim
  for ingest/1; no newbox admission claim; the sealed
  forks' reopen conditions are not asserted met; overall
  MirBuilder is not complete.
```

## D21 census boundary

- Includes: post-S8 `real-apps-exe-boundary` receipt classification;
  the `ingest/1` decline lane attribution; owner mapping to existing
  cards; the sealed disposition with reopen triggers.
- Excludes: implementation; reopening D5/B3/DeclaredInstance/D0/D17/
  D18 decisions; backend toolchain work; Gates 2-4.

## D21 census exit

- [x] Census recorded: zero unowned failures in the post-S8 Gate-1
      receipt; VM decline attributed to the ledger-free lane.
- [x] Named sealed disposition with observable reopen triggers;
      no bounded slice emitted.
- [x] Pointers and workstream row H synced; guard re-run.

## MIRBUILDER-GATE1-DEPENDENCY-PLAN-D22 — successor plan

Decision accepted for task selection, 2026-09-27, at `24e796b0f4`.
The user requested investigation and forward planning after review R0.
One read-only worker audited the owner/reopen premise; the primary checked
receipts, source boundaries, local tools and downstream gate order.
This was design work: field residence/provider and append outcome are not
settled, so no semantic implementation was attempted.

### Corrected stop and evidence

- D21 correctly exhausted its route/publication slice. Its dependencies are
  internal owner design/construction, not evidence of an external blocker.
  B3-D2 itself says missing source type/effect/Fault issuers are internal
  design tasks (the section before its ordered implementation tasks).
- Review R0 is landed. Receipt integrity checked: 8164 unique test names,
  127 unique failure names and both hashes match; 7981 pass / 127 fail /
  56 ignored is known-red evidence, not a green full suite. Scope guard and
  the six split-file line counts were checked; all are below 800.
- Local `opt`/`llc` resolve to LLVM 14.0.0; the documented invocation-local
  `PATH=/usr/lib/llvm-18/bin:$PATH` resolves both to installed LLVM 18.1.8.
  No install or external provisioning event is needed on this machine.
  `bef41c25a1` records the focused unchanged-source newbox success; D21's
  effective 5/11 must not be labelled a fresh full-suite run in D22.
- No Cargo, compiler, EXE or full-suite run was performed in this design
  review. Existing D21 terminals remain the dynamic evidence. No observed
  semantic reopen proof, whole-app success or overall completion is added.

### Selected next design: MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-D0

```text
Decision: select D5 fork (a)'s field-resident Array/I64 append design;
  this is a dependency design intake, not parked implementation activation.
Source authority + canonical issuer: parser field declaration identity +
  same-session resolver field-read/initializer/binding/call sites, through
  CoreMethodInstanceTargetIssuerV1 -> ResolverCoreMethodCallableContractIssuerV1
  -> existing package CoreMethod path; CoreMethodContractBox owns semantics.
Non-authority: field/local spelling, physical I64 values, raw runtime status,
  B3's fresh-array/Text proof, and the deprecated VM route are not authority.
Fail-fast boundary: absent/foreign residence, provider or argument/outcome
  relation rejects before physical writes; preserve the existing Text family.
Smallest next slice: settle field residence/lifetime, provider selection,
  I64 input, NoValue result, failure class and append commit timing for the
  four seedBlocks call sites; name exact retained transport and consumer.
Non-claims: these relations are missing today; no new receipt, production
  connection, selected-C fate change, Gate-1 green or Gates 2-4 activation.
```

Finite source tuple:
`lang/src/hako_alloc/memory/page_heap_box.hako:58-70`,
`HakoAllocPage.seedBlocks/0`, field-to-local aliases of `free_stack`,
`block_used`, `use_counts`, `requested_sizes`, then four `push/1` calls
with values `i`, `0`, `0`, `0`. The recorded boxtorrent/mimalloc first
terminal shares this owner. The source spellings locate this inventory;
implementation must use declaration identity and resolver sites.

Existing contracts that the design must reconcile:

- `src/mir/resolved_semantics/named_array_requirement.rs`: a conditional
  requirement, not provider proof; requires same-function construction and
  retained Text. A field read cannot be made to satisfy it by default.
- `src/mir/resolved_semantics/core_method_instance_target.rs`: current
  ArrayTextAppend/TextRetainedByReceiver contract. Integer and handle
  demands need their own source meaning, never physical-value inference.
- `src/mir/named_array_obligation.rs`: physical marker requires one local
  Named(ArrayBox) NewBox. Field residence needs an explicit retained
  correspondence; do not invent a local allocation or skip this validator.
- Existing consumption/writer: `normal_callable_semantic_lowering_state/`
  `named_array.rs::take_source_array_push` and plan lowerer
  `effect_emission.rs`. Integer runtime transport exists in
  `crates/nyash_kernel/src/plugin/array_runtime_aliases.rs`, but is a
  consumer candidate, not source/provider or Fault authority.

The owning decision remains D5 fork (a), documented in
[the existing coverage-owner card](mirbuilder-exe-acceptance-owner-selection-d5-2026-09-25.md).
The original [B3-D2](mir-call-parser-array-push-b3-loopcond-carrier-relation-d2-2026-09-23.md)
StringHelpers fresh-array/Text/substring tuple is a separate scope. It is not
an invented prerequisite for this field-resident I64 task.

### First owner series and acceptance

| Order | Task | Exit / dependent work |
| --- | --- | --- |
| 0 | Fixed LLVM 18 execution profile | Use the invocation-local PATH above; record compiler binary/source revision, resolved tools and runtime. Reuse the recorded newbox proof; rerun its focused smoke only when validating the actual checkpoint. No compiler semantics change. |
| 1 — selected | Field-resident Array/I64 D0 | Resolve source residence/lifetime, exact provider and success/failure/commit meaning through the named issuer family; give the retained source-to-physical correspondence and selected old-edge map. Name any ordinary-new eligibility prerequisite as a successor identified by D18. Do not require the finished implementation or green tests to start this design. |
| 2 — after Decision | Source contract + physical connection | Implement the accepted relation in existing issuers/package transport, consume all four rows in LoopCond coverage and the existing writer; update the owning README/reference and focused tests in that slice. Preserve the proof through publication/backend validation. No separate loop solver or synthetic NewBox. |
| 3 — same bounded series | Selected caller cutover + retirement | Switch selected callers; pass the positive/negative runtime owner acceptance below and guards before deletion. Check the exact delete-set and caller-zero, then remove this membership's omission/reconstruction path; retain shared arms for other callers. Issuance failure is terminal, never a generic retry. |
| 4 | Original app acceptance | Run the two original app smokes after owner acceptance and retirement. Record any new first terminal as the next owner dependency, not success for the whole app. |

Candidate old seams to confirm during D0 (not deletion permission):
`src/mir/source_call_target/named_array_method.rs` optional omission;
`src/mir/builder/normal_callable_loop_source_route_items.rs` uncovered-call
boundary; `control_flow/plan/normalizer/loop_body_lowering_associated_input.rs`
generic name/phi/variable-map method reconstruction. Shared helper existence
is not a stop condition, and generic fail-fast coverage is not deleted.

Focused acceptance specification:

- Positive: the four natural source relations above, alpha-renamed aliases,
  zero/one/multiple iterations, all four contents/lengths and exact I64 values
  including an integer numerically equal to a live handle.
- Negative: shadow ArrayBox, foreign field owner, missing residence/provider,
  reassigned alias, Text/handle argument in the I64 family, push used as a
  value, duplicate/missing/residual source row; reject before writes.
- Keep the existing Text-family rejection tests unchanged. Runtime failure
  and storage commit assertions must follow the accepted outcome contract;
  raw zero status does not define it. Caller-zero and successful execution
  are implementation/retirement outputs, not D0 entry conditions.

### Remaining dependency order (queued design, not blanket permission)

After the selected series, use the next observed required terminal. An exact
prerequisite discovered inside that series takes precedence over this queue.

| Order | Existing owner / exact scope | Needed contract and completion evidence |
| --- | --- | --- |
| 5 | D5 fork (b) admission successor; [D15 locator reference](mirbuilder-exe-acceptance-declared-instance-locator-d15-2026-09-26.md) | `BinaryTreesBench.iterationCheck/3` parameter/local calls: source receiver/target/result/effect coverage and backend consumer. Existing root-me relation/locator is not that coverage. Prove the original binary-trees source; keep selected-C fate parked. |
| 6 | Ordinary-new eligibility successor identified by [D18](mirbuilder-exe-acceptance-ordinary-new-lifecycle-d18-2026-09-26.md) | `new JsonStreamAggregator` handle/default fields: source definition eligibility, child construction, retained ownership and normal/Fault cleanup through existing owners. Prove original output and child cleanup; select earlier if step 1 needs it. D18's completed documentation slice stays closed. |
| 7 | D5 NamedArray source demand: `allocator-stress/main.hako` fresh array + `heap.allocate(...)` | Depends on instance-call result and handle lifetime. Resolve typed retained-handle demand through the existing source operation owner; neither Text nor integer acceptance is evidence. Verify retained identity/release and the unchanged allocator-stress smoke. |
| 8 | [Untyped-storage D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md): `typed_object_untyped_field_min` | Explicit tagged dynamic/opaque slot ABI, ownership and selected physical consumer; retain the exit-7 test. Do not fix by allowing generic Invoke JSON, rejecting an accepted program, or inventing I64. |
| 9 | Existing `real-apps-exe-boundary` owner | Once dependencies are connected, run the fixed 11 entries under one recorded toolchain. Keep original outputs/exit codes; classify each residual first terminal. Focused owner smokes serve intermediate slices; no repeated whole-suite census without changed evidence. |

Original B3 StringHelpers, ledger-free VM parity and general compatibility
retirement stay on their existing owner boundaries. Do not reopen them merely
because they are adjacent to an Array task. S9-S14/R0's 127-name baseline is
not a new warning/test-cleanup campaign or a Gate-1 completion substitute.

### Beyond Gate-1

Follow the existing unified resume order, selecting each gate only when its
prerequisites close: language-v1 conformance/rejection matrix -> pinned Hako
mimalloc correctness/lifecycle/performance gate ->
`MIRBUILDER-FACT-OWNER-PARITY-TEMPLATE-PILOT-SELECTION-001` with real caller
switch and Rust-owner retirement -> REGISTRY/Recipe -> symbolic commands ->
allocation/executor -> parser -> Stage1 builds a nondelegating Stage2 that
compiles and runs the fixed acceptance programs. These are queued handoffs,
not permission to jump Gates 1-3 or resume retired vocabulary pilots.

D22 closes selection/task planning only. Current execution returns to the
named field-array D0 in design_stop; next_execution stays none until its
source/residence/provider/outcome Decision is settled. An internal missing
relation selects its bounded owner task; it does not become an external wait.
