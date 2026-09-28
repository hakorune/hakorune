# MIRBUILDER-GATE1-CALLABLE-LOOP-STRING-INDEXOF-S0

Status: StringIndexOf S0 landed; call-free LoopCond S0 design accepted 2026-09-28
Date: 2026-09-27
Scope: original StringIndexOf receipt and its Gate-1 successor selection.
Related: CURRENT_STATE.toml; workstream row H; owner-selection D5 (F3c).
Emission: `mirbuilder-gate1-callable-loop-string-indexof-d0-2026-09-27.md`
Decision — admit `StringBox.indexOf/1` (`StringIndexOf`, I64Value,
pure_read, LoopBody placement) as a plain CoreMethod contract through
the existing StringBox issuer arm, gated by **receiver-text evidence**
(string-literal or `TextToCaller`-contract initializer, no rebind) and
a text-producing needle argument; no selector-only minting, no
write-requirement product.
Selection proof: `MIRBUILDER-GATE1-CALLABLE-LOOP-STRING-INDEXOF-S0`
-> workstream row H.
MirBuilder goal row 1 (sole Facts issuance — `method_calls()` /
`initializer_relations()` / `literal_source()` stay the only
inventories), row 3 (sole admission boundary — the probe's
complete-or-reject coverage, unchanged), row 4 (verified admission —
text evidence, not selector/name guessing).

## Exact boundary

- `src/mir/resolved_semantics/core_method_instance_target.rs`:
  new `CoreMethodHomeParameterRelationV1::TextParameter` variant
  (borrowed text argument — `TextRetainedByReceiver` stays
  push-specific); `(StringIndexOf,1)` arm in `issue` ->
  `StringBoxReceiver` + `[TextParameter]` + `I64ToCaller`
  (requires manifest `I64Value`, `ResultMismatch` otherwise).
- `src/mir/source_call_target/core_method.rs`:
  `allowed_placements` `(StringIndexOf,1) -> &[Body]`; a bounded
  private predicate `index_of_has_text_evidence` /
  `text_source_at` consulted only for `StringIndexOf`
  before target issue:
  - receiver `Lexical(Local)` -> exactly one `initializer_relation`
    -> initializer site text-producing
    (`literal_source == String` OR a minted `TextToCaller` contract
    row at that site); plus no `BindingRebind` assignment target on
    the receiver binding;
  - argument site: `literal_source == String`, or `variable_ref ->
    Local` with the same single-initializer text proof (bounded
    depth like `integer_source_at`);
  - failing evidence -> `continue` (unarmed), never Err.
- `src/mir/resolved_semantics/resolver_core_method_callable_contract.rs`:
  `allowed_target_placements` `(StringIndexOf,1) -> [Body]`;
  result tuple `(StringIndexOf,1,I64ToCaller)`;
  `expected_parameters` `StringIndexOf -> &[TextParameter]`.
  The `(None, StringBoxText)` plain-contract matcher already
  admits the schema — no named-array product.
- `source_call_publication.rs`: `I64ToCaller -> MirType::Integer`
  arm already exists — no consumer change.
- `tools/checks/mirbuilder_qualified_route_scope_guard.sh`: pin
  `StringIndexOf`, `TextParameter`, `(StringIndexOf,1)->[Body]`,
  and the new test names.

## Out of scope

- `StringIndexOf/2`, `lastIndexOf`, `find` on non-text receivers.
- Parameter/other receivers without a text-producing initializer
  (`pred_chars.indexOf` stays on the Dynamic-member lane).
- Non-text receiver evidence families (`new ArrayBox()` locals,
  `me` fields, qualified/other receivers).
- `length`/`substring` behavior changes (their existing arms are
  untouched; the runtime-router `ArrayBox.indexOf` row never
  leaks in).
- Any probe/route/physical-path change; no write-requirement
  product; no `me.<field>` receiver residence for strings.

## Negative/deletion proof (acceptance shape)

- `local alphabet = "..."` + `local ch = text.substring(i,i+1)` +
  loop-body `local idx = alphabet.indexOf(ch)` -> one contract row:
  `StringIndexOf`, `StringBoxReceiver`, `[TextParameter]`,
  `I64ToCaller`, Body placement, `named_array_requirement` None.
- `local a = new ArrayBox()` + `a.indexOf(x)` -> unarmed (no row).
- parameter receiver `s.indexOf(ch)` -> unarmed (no initializer).
- `local t = 1` + `alphabet.indexOf(t)` -> unarmed (non-text needle).
- `alphabet = "other"` anywhere -> unarmed (rebound receiver).
- `x.indexOf(ch)` in loop Condition -> unarmed (placement).
- `x.indexOf(ch, 0)` arity-2 -> unarmed (arity key).

## Pinned evidence (landed)

- `core_method_source_tests` 11/11 green:
  `literal_receiver_index_of_arms_body_with_text_parameter`
  (op/placement/`[TextParameter]`/`I64ToCaller` pinned),
  `substring_contract_supplies_index_of_needle_text`
  (TextToCaller row supplies the needle proof),
  `find_alias_follows_the_same_text_evidence_gate`,
  `index_of_stays_unarmed_without_text_evidence`
  (ArrayBox/integer receiver, non-text needle, rebound receiver,
  arity-2, `lastIndexOf`, parameter receiver `pred_chars`),
  `index_of_condition_placement_stays_rejected` (Ok, zero rows).
- Regressions: core_method 74, source_call_target 83, source_call 87,
  callable_contract 18, named_array 31 — all green.
- Production: a `ContentHash.digest`-shaped program
  (`local alphabet` literal + `text.substring` needle + loop-body
  `alphabet.indexOf(ch)`) compiles end-to-end; emitted MIR carries
  `indexOf` as a `RuntimeDataBox` MethodCall — the same
  WarmDirectAbi runtime-dispatch shape `length`/`substring` emit.
- boxtorrent re-measurement: the sole remaining stop is
  `materialize/1` `store.readData(cid)` — the parked
  DeclaredInstance terminal (`route-not-front-selected`,
  `SourceCallOutsideSelectedFamily`). Recorded, not claimed.
- Cross-app census (`--emit-mir-json`, compile only; attribution corrected
  below on 2026-09-28): binary-trees `iterationCheck/3`
  (`builder.make` + `itemCheck`, 4 instance-call sites),
  mimalloc-lite `bin_size/1` (call-free loop, not DeclaredInstance),
  allocator-stress `handles.push(heap.allocate)` (instance-call
  result arg needs instance return provenance);
  json-stream-aggregator reaches MIR compilation; this is not its required
  EXE/output acceptance. The typed-object `Invoke` JSON emit gap remains an
  acceptance dependency: its registered smoke actually uses JSON emission.
  Neither entry is cleared by a compile-only census.
- Guards: `mirbuilder_qualified_route_scope_guard.sh` green
  (StringIndexOf pins added), pointer guard green.

## Next — frontier correction (2026-09-28)

The user requested the next task design. One read-only worker checked the
claimed receiver/owner boundary; this was not Fast path because the reported
family conflicted with the actual source. The primary checked the original
application once with the existing compiler, without a build or source edit.

`SizeClassBox` is a static box (`size_class_box.hako:5`). The three calls in
`bin_size/1` at lines 25, 26 and 40 are outside its loop (lines 34-37).
The accepted `StaticCurrentOwner` policy in
`docs/reference/language/function-call-evaluation.md` assigns these calls to
the exact static target, never a declared-instance receiver.

Observed first terminal: `LoopCondRouteRejected(SourceItemsMissing)`,
`function=SizeClassBox.bin_size/1`, static catalog owner, site `[Body(9)]`.
The loop contains only `scale = scale * 2` and `i = i + 1`.
This is D5's queued F3c call-free coverage task; it disproves the previous
blanket `RemainingTerminalsParkedDeclaredInstance` attribution.

Focused evidence (informational design probe, not a new regression or PASS):

```text
HEAD: dbd9f7eb39535d3c6c600eec10a5f87b54a87534
UTC: 2026-09-27T18:04:42Z
NYASH_DISABLE_PLUGINS=1 timeout 30s target/debug/hakorune \
  --emit-mir-json /tmp/hakorune-next-design-gb0l_5al/mimalloc.mir.json \
  apps/mimalloc-lite/main.hako
exit: 1; no MIR output
binary SHA256: 8015638bc191a8f7a127d3aa5addb73e099fd0a0a59366a54db203f79216da09
source SHA256: 63f688bef954d29ef91930e861b0721293789c4e74af2c6f1dae25b6ed84b772
```

Local detail: `/tmp/hakorune-next-design-gb0l_5al/{receipt.json,probe.log}`.
The existing binary's exact build commit was not independently established;
this receipt identifies its contents and the checked-out source separately.
Current source also deliberately rejects an empty source-call inventory, and
the existing call-free loop-scope test pins this rejection. No EXE/runtime or
whole-suite completion is claimed.

## Accepted next task: MIRBUILDER-GATE1-CALLFREE-LOOPCOND-S0

Design accepted at `dbd9f7eb39` on 2026-09-28; implementation landed in the
same workstream turn. This selects the already-queued D5 F3c source-coverage
responsibility, not a parked DeclaredInstance/backend or Gates 2-4 lane.

```text
Decision: admit bounded flat call-free LoopCond via explicit source coverage.
Source authority + canonical issuer: exact resolver input/ledger/body inventory;
  CallableLoopSourceBridgeV1::from_input projects coverage, loop_cond::issue co-seals it.
Non-authority: empty method list alone, fixture spelling, static call outside the loop, MIR.
Fail-fast boundary: incomplete/foreign coverage rejects before physical allocation; no retry.
Smallest next slice: CallFree/WithCalls transport through the existing LoopCond consumer.
Non-claims: no new numeric semantics, instance admission, other loop family, or Gate-1 PASS.
```

### Change / Contract

The finite production tuple is `SizeClassBox.bin_size/1 [Body(9)]` from
`lang/src/hako_alloc/memory/size_class_box.hako:34-37`. Select a flat,
nonempty `NoExitBody` of plain local binding assignments; cover the existing
one-carrier regression and this two-carrier loop. There is no branch, nested
loop, early exit, constructor, field/index write or opaque/transferred subtree
in the selected shape. Existing scalar expression/type semantics stay intact.
Finite S0 grammar: value = integer literal / lexical local / addition of two
values / multiplication of two values; condition = value `<` value;
statement = plain local `BindingRebind` to a value. All child sites must be
covered. Other operators (including unary), statements and conditions remain
outside this slice; this grammar admits both selected loops without a type default.

Represent source call coverage explicitly as `CallFree` or `WithCalls`
(the latter retains `CallableLoopSourceTargetRelationV1`). The proof is an
owned projection of existing source facts, not a new result/effect authority:

- Issue at `normal_callable_semantic_lowering_state/source_loop_bridge.rs`
  while the exact `ResolvedFunctionLoweringInputV1` is available. Use its
  branded body inventory and ledger's assignment targets, variable/literal/
  binary rows, direct calls, method calls and complete expression-site
  inventory. Every condition/body expression must belong to the selected
  scalar closure; every body statement must correspond to a typed Plain
  assignment with a `BindingRebind` target. Missing rows never prove absence.
- Check effects and complete expression closure, including child sites:
  ordinary binding writes are allowed only by those assignment rows;
  calls, allocation, await/control effects and opaque parents are outside S0.
  An empty method list cannot hide a direct call, constructor or lambda.
- Move coverage with the existing bridge/raw payload/route token, then co-seal
  at `normal_callable_loop_source_facts/loop_cond.rs::issue`: same owner,
  source lineage/frame, root site, one loop member, `NoExitBody`, no exits,
  and one-to-one flat `Stmt` Recipe/assignment correspondence. Do not pair
  independently reconstructed products by a matching name/key.
- `SourceLoopCondPhysicalInputV1` validates the sum and identities before
  allocation. `WithCalls` preserves current target/result/coverage checks
  and error order. `CallFree` requires the complete source proof and no
  selected/uncovered/CoreMethod probe residual. It has no call-site anchor
  or `ExactI64` call result to manufacture.
- Reuse `control_flow/plan/features/loop_cond_bc_source.rs::
  lower_loop_cond_break_continue_source`, its existing assignment items,
  carrier publication, PlanVerifier and sole PlanLowerer. They do not need
  a call target to implement this loop. No additional physical route.

Admission states and failure handling:

| State | Authority / behavior |
| --- | --- |
| BridgeAbsent / Unarmed | Preserve the existing source-bridge distinction and routing; neither becomes CallFree. |
| Proven call-free shape | Exact source closure plus selected NoExitBody co-seal admits CallFree. |
| Call-bearing source | Existing WithCalls selection and coverage apply unchanged; unsupported calls remain terminal. |
| Empty methods, incomplete or unsupported source | No CallFree product; retain named source/route rejection on the armed lane. |
| Foreign owner/site/frame, contradictory probe | Typed pre-effect rejection; no repair or fallback. |
| Repeated take or residual consumption | Existing one-shot/finish rejection; no retry after consumption. |

Selected old responsibility: remove the mandatory nonempty call-list,
call-site anchor and call-result requirement **for this proven call-free
shape** from route token issuance/drain and physical input validation.
Retain those checks for WithCalls and unproven empty inputs. This is removal
of an incorrect admission prerequisite, not deletion of the shared call owner.

### Done / Stop

1. If transport work grows `raw_loop_child_entry.rs` (currently 762 lines),
   first extract its source-entry preparation by responsibility in a separate
   BoxShape commit. Do not mix that split with semantic admission; keep all
   source files below 800, and design splits at 760. No unrelated cleanup.
2. Implement the source proof and transport/consumer changes together;
   replace the selected mandatory-call prerequisite and switch the tuple in
   this semantic slice, then prove it with focused acceptance and guards.
   Do not require green tests before this construction. Shared call-bearing
   checks remain; any separate physical-edge deletion requires prior caller
   switch/stop, acceptance and caller-zero under RULES section 4.
3. Update the existing loop-scope negative for the selected flat shape into a
   positive, retaining invocation-scope/ledger assertions. Execute a natural
   two-carrier source case with zero/one/multiple iterations; verify values,
   valid PHIs and post-loop carrier use. Keep outside-loop static calls intact.
4. Reject hidden method/direct calls in condition/RHS, construction, opaque
   expressions, unsupported statements, missing/foreign proof, duplicate take
   and residual target evidence. Preserve existing static/CoreMethod positives
   and missing-ledger/uncovered-call negatives. Use the existing
   `mirbuilder_qualified_route_scope_guard.sh`, not a new per-row guard.
5. Record focused compiler revision/binary and the original mimalloc source's
   next terminal. Advancing past `bin_size [Body(9)]` is the selected result;
   later app failure is an owned dependency, not permission to widen this S0.
   Update builder README and `docs/reference/mir/loop-recipe-contract.md`
   with the admission distinction in the implementation commit.

Return to design only if this mapping needs new source authority, statement/
expression shapes, arithmetic/effect semantics or a second physical owner.
Ordinary implementation failures inside this mapping are work to resolve.

### Queue after this slice

| Order | Task and completion boundary |
| --- | --- |
| 1 | DONE — `MIRBUILDER-GATE1-ORDINARY-NEW-ARTIFACT-SOURCE-D0` landed below: the terminal is `Main.main`'s root-instance-call disposition, and the missing artifact is the callee's declared result contract (`MiWorkload.run` is unannotated). The S0 below completes the declared contract in source and re-measures. |
| 2 | DONE — `MIRBUILDER-GATE1-PARAM-RECEIVER-CALL-SOURCE-D0`/`S0` landed below: parameter receivers proven by universal caller-edge ordinary-new claims and claim-local `local x = new C()` receivers both arm lexical `InstanceBoxMethod` dispositions; boxtorrent-mini `materialize`/`releaseFrom` loops now lower through the canonical `SameModuleInstance` emitter. |
| 3 | DONE — `MIRBUILDER-GATE1-BODY-LENGTH-TEXT-EVIDENCE-S0` landed below: `StringLen/0` admits `Body` placement gated by `receiver_has_text_evidence` (TextToCaller initializer or literal); boxtorrent-mini now lowers fully (remaining terminal is the Invoke emit gap); the mixed-loop `SelectedStatic` bucket-drop hazard is fixed. New named frontier for the receiver family: binary-trees `iterationCheck` leaves `builder.make` (field-read caller argument, vetoed by universal edge proof) and `itemCheck` (call-result receiver, parked) uncovered — see `MIRBUILDER-GATE1-FIELD-ARG-AND-CALLRESULT-RECEIVER-D0`. |
| 4 | DONE — `MIRBUILDER-GATE1-FIELD-WRITE-CLAIM-EDGE-TRANSPORT-S0` landed below: `(box, field) -> class` claims from uniform attributed `me.f = new C()` writes; binary-trees `iterationCheck` uncovered set halves to the two `itemCheck` call-result receivers; mimalloc-lite's semantic lane now completes to the shared Invoke emit gap. |
| 5 | DONE — `MIRBUILDER-GATE1-CALLRESULT-RECEIVER-RESULTCLASS-S0` landed below: `OrdinaryNewResultClassClaimDraftV1` seals `{selected key -> class}` only when a body ends in a value `return` and every `return` row constructs `new` of one agreed ordinary box; the lexical `MethodCall` initializer arm resolves the callee through the proven-receiver join. binary-trees `iterationCheck` call coverage closes — both `itemCheck` receivers arm — and the semantic lane now stops at the loop-handoff binding contract (`sum` carrier lacks `ConditionRead`), a different owner. |
| 6 | DONE — `MIRBUILDER-GATE1-LOOP-HANDOFF-NONCOND-CARRIER-D0`/`S0` landed below: write-only/body-rebound projected bindings flow into the existing `BodyOnlyRebind` outside cohort instead of freezing `incomplete-binding-coverage`; binary-trees' `run` loop coverage now passes. |
| 7 | DONE — `MIRBUILDER-GATE1-ROOT-INSTANCE-CALL-UNRELEASABLE-HOME-D0`/`S0` landed below: the issuer withholds the `Ready` row when a terminal home's sealed claim can never reach `end_available`; binary-trees `Main.main` now stops at the typed `[ordinary-new/local-commit/root-call-entry-missing]` boundary (premise corrected below: the Plain-exit tolerance covers `Emitted{Plain}` only, not `Unavailable`). |
| 8 | DONE — `MIRBUILDER-GATE1-ROOT-CALL-UNRELEASABLE-RECEIVER-D0`/`S0` landed below: `rebind_root_call_entry` tolerates the recorded `Unavailable` exit (same class as the existing non-Call-entry tolerance); binary-trees `Main.main` `--emit-mir-json` now stops at the designed `root-call-entry-unavailable` seal boundary and the non-sealing lane completes with the generic `call_method` + `Unavailable` observation. |
| 9 | DONE — `MIRBUILDER-GATE1-INVOKE-TERMINATOR-JSON-EMIT-D0`/`S0` landed below: an already-accepted SSOT decision keeps generic `mir_json_emit` closed to lifecycle `Invoke` (the emit "gap" is the designed fail-fast); `typed_object_untyped_field_min_exe` was re-pointed off `selfhost_build.sh --mir` + ny-llvmc generic ingress onto the direct `--emit-exe` physical route with the rejection kept as a negative pin. Its true terminal is now `artifact-source-unavailable` — the unreleasable-home family. |
| 9b | `MIRBUILDER-GATE1-UNRELEASABLE-HOME-ARTIFACT-ADMISSION-D0` (design_stop): several Gate-1 apps land on `artifact-source-unavailable` because a root home's sealed claim can never reach `end_available` (binary-trees `bench` object field, untyped-field-min `Holder.items` ArrayBox handle). Decide whether/how the artifact lane admits a program whose homes stay `Unavailable` (retained-release semantics) or whether Gate-1 acceptance requires those homes to become releasable; name the owner and the physical implications before implementation. json-stream-aggregator still stops upstream at `route-not-front-selected`. |
| 10 | Fixed 11-entry EXE suite under the recorded LLVM 18 profile, after changed owners' focused checks. Gate 1 remains unsatisfied until its actual acceptance closes; then follow language conformance -> mimalloc gate -> Facts migration/selfhost. |

### Landed slices tombstone (compressed 2026-09-28)

The following sections are retired to git. Each was committed before
compression; full decision text and receipts remain in history
(`git log -p -- <this file>`, landing commits `b16c3548ac` ..
`179c6c67c7`). The queue table above retains the frontier summary.

- `MIRBUILDER-GATE1-ORDINARY-NEW-ARTIFACT-SOURCE-D0/S0` — landed
  `b16c3548ac`: declared result contract is the missing artifact;
  `MiWorkload.run(): i64` completed in source.
- `MIRBUILDER-GATE1-PARAM-RECEIVER-CALL-SOURCE-D0/S0` — landed
  `660dafc7b1`: universal caller-edge + claim-local `new` claims arm
  lexical `InstanceBoxMethod` dispositions.
- `MIRBUILDER-GATE1-CALL-RESULT-ARGUMENT-COVERAGE-D0` — landed: census
  for call-result argument coverage; superseded by the receiver
  provenance slices.
- `MIRBUILDER-GATE1-BODY-LENGTH-TEXT-EVIDENCE-S0` — landed
  `901df40d18`: `StringLen/0` Body placement behind receiver text
  evidence; boxtorrent-mini reaches the shared Invoke emit gap.
- `MIRBUILDER-GATE1-FIELD-ARG-AND-CALLRESULT-RECEIVER-D0` — accepted:
  field-read argument + call-result receiver census.
- `MIRBUILDER-GATE1-FIELD-WRITE-CLAIM-EDGE-TRANSPORT-S0` — landed
  `43b706e07e`: `(box, field) -> class` claims from attributed
  `me.f = new C()` writes.
- `MIRBUILDER-GATE1-CALLRESULT-RECEIVER-RESULTCLASS-S0` — landed
  `179c6c67c7`: `OrdinaryNewResultClassClaimDraftV1` seals
  `{selected key -> class}` for agreed-return-`new` bodies.

### D0 decision — `MIRBUILDER-GATE1-LOOP-HANDOFF-NONCOND-CARRIER-D0` (landed)

```text
Decision: a write-only projected binding (BodyRebind receipts, zero
  in-loop reads) satisfies the existing outside cohort criterion
  (`has_rebind && !has_condition_read`) but is frozen earlier by the
  shared `has_read` pre-validation — admit it into `BodyOnlyRebind`
  outside rows alongside the `sum` shape instead of freezing.
Source authority + canonical issuer: sealed `variables`/`assignments`
  receipt maps feeding `CallableLoopSourceProjectionV1::project_disposition`;
  the ready/outside split already lives there —
  `normal_callable_loop_handoff.rs` is the sole issuer.
Non-authority: no physical PHI/carrier minting here; this schedule is a
  coverage proof — `consume_pre_effect` receipt is dropped after
  validation and JoinIR `condition_bindings`/`carrier_phis` (recipe Pass
  1 + `collect_carrier_inits` + phase_3_5 body-only-carrier remap) own
  the physical transfer of live-after-loop bindings.
Fail-fast boundary: ready rows keep every existing check
  (`build_callable_loop_ready_rows` retains `!has_read`, carrier
  completeness, `carrier-cardinality`); non-local/foreign/nested-loop
  receipts still freeze before the split.
Smallest next slice: `MIRBUILDER-GATE1-LOOP-HANDOFF-NONCOND-CARRIER-S0`
  — exempt outside-eligible bindings (has BodyRebind) from the
  pre-validation `has_read` requirement so they flow into
  `BodyOnlyRebind` rows; pin positive (write-only binding read after
  the loop) and negative (ready-row coverage unchanged) tests;
  re-measure binary-trees.
Non-claims: no change to carrier classification, iteration-local
  consumption, or call coverage; nested-loop receipts still veto; the
  `run` loop's other uncovered call sites, if any surface after this,
  are a separate named frontier.
```

Census evidence: `/tmp/probe-noncond/writeonly.hako` (`out = i` inside
`if` in a loop, read only after exit) reproduces
`incomplete-binding-coverage` exactly — binary-trees `run`'s loop has
four such bindings (`depth4_iterations`, `depth4_check`,
`depth6_iterations`, `depth6_check`). The accumulator probe
(`sum = sum + i`) instead reaches `callable-loop/facts-absent` (a
call-free loop takes a different route), confirming `sum`-shaped
bindings pass the `has_read` check and flow into `ReadyWithBodyOnly`.
Receipts are only ever minted for reads and rebinds, so the
pre-validation `has_read` loop can only ever fire on rebind-only
bindings — the same bindings the outside criterion then selects.
```

### S0 implementation receipt — `MIRBUILDER-GATE1-LOOP-HANDOFF-NONCOND-CARRIER-S0` (landed)

`validate_projection_rows`
(`normal_callable_loop_handoff_validation.rs`) no longer rejects
bindings that carry no read receipt — owner, source-site containment,
role/source suffix, duplicate-site, and iteration-local coverage checks
are all retained, and `build_callable_loop_ready_rows` still applies its
own `!has_read` freeze, so only bindings the outside criterion selects
(`has_rebind && !has_condition_read`) reach `BodyOnlyRebind` rows.
`seal_loop_true` keeps its own `has_read` check and still refuses
write-only bindings under the loop-true cohort. New pin
`write_only_rebind_moves_with_ready_remainder_as_body_only_row` (11/11
loop-handoff tests); probe `/tmp/probe-noncond/writeonly.hako` now
advances past `incomplete-binding-coverage` to the call-free-loop route
boundary (`route-not-front-selected`, a separate named lane).

Production re-measure (`--emit-mir-json`): binary-trees `run`'s loop
coverage passes; the lane first stopped at
`[ordinary-new/local-commit/root-call-entry-missing]` because
`BinaryTreesBench.run()` lacked the declared result contract
(precedent: `MiWorkload.run(): i64`) — completed as `run(): i64` in
source — and now reaches
`[mir/callable-semantic-package/port] IncompleteOrdinaryNewCoverage`:
the issued `Ready` row for `return bench.run()` is never taken because
the sole consumer's sole entry (`prepare_root_home_exit` → per-home
`end_available`) declines `bench` forever. Queued as row 7 D0.
mimalloc-lite / boxtorrent-mini remain at the shared `Invoke` emit gap
— no regression.

### D0 decision — `MIRBUILDER-GATE1-ROOT-INSTANCE-CALL-UNRELEASABLE-HOME-D0` (accepted)

Ledger-instrumented census (temporary probes, removed):

- `is_empty` reports only `root_instance_call` non-empty; the stranded
  row is `OwnedExprSiteV1 { owner: Main.main, site: [Body(1), Value] }`
  — exactly `bench.run()`.
- `prepare_root_home_exit` for `Main.main` reaches the homes loop and
  finds `bench` with `end_available = false`, so the exit is recorded
  `Unavailable` and `return bench.run()` lowers through the regular
  value path — the `Ready` row is therefore un-takeable by
  construction.

```text
Decision: the issuer owes a Ready row only when the single consumer can
  take it. `issue_root_instance_call_dispositions` must withhold the
  row when any terminal home's `new` claim proves it can never reach
  `end_available` — destruction != `PlainI64NoHook`, construction
  ineligible, or home prefix unavailable. `bench` is
  `RetainedUnavailable` because `BinaryTreesBench`'s `init`-declared
  fields carry no i64-only destruction profile, so the root-instance-call
  lane is structurally unavailable for `bench.run()`; the Plain-exit +
  `root_instance_call_expected` path (`validate_call_entry`) already
  covers this by design.
Source authority + canonical issuer: the sealed claim's destruction /
  construction / home-prefix products (`issue_ordinary_source_cohort_v1`)
  joined with the root completion's `terminal_homes`; the issuer is the
  sole authority on whether a consumable duty exists.
Non-authority: MIR emission state, `RetainedUnavailable` progress, and
  the physical invoke all remain consumer facts; no new product family.
Fail-fast boundary: `root_instance_call_expected` stays set, the
  `Plain`-exit tolerance and the strand detection in
  `root_instance_call_is_empty` are unchanged — a Ready row that is
  issued and not taken still freezes.
Smallest next slice:
  `MIRBUILDER-GATE1-ROOT-INSTANCE-CALL-UNRELEASABLE-HOME-S0` — gate row
  insertion in `issue_root_instance_call_dispositions` on the homes'
  sealed release profile; pin `Holder{init{inner}}`-style negative (row
  absent, expected kept) and keep the `Pair` positive; re-measure
  binary-trees (expected terminal: the shared Invoke emit gap).
Non-claims: does not extend the exit lane to unreleasable homes (an
  artifact-release family question, out of scope), does not weaken
  coverage, does not mint keys/physical IDs, changes no consumer.
```

### S0 receipt — `MIRBUILDER-GATE1-ROOT-INSTANCE-CALL-UNRELEASABLE-HOME-S0` (landed, premise corrected)

Implementation: `issue_root_instance_call_dispositions`
(`ordinary_new_root_instance_call.rs`) now consults the owner's sealed
`terminal_homes` after stamping `root_instance_call_expected`. When any
terminal home's `new` claim can never reach `end_available`
(destruction != `PlainI64NoHook`, `construction()` ineligible, or
`home_prefix()` unavailable) the `Ready` row is withheld instead of
being issued into a ledger no consumer can drain. No consumer was
changed; no fallback was added.

Focused pins (`ordinary_new_terminal_result_tests`, 6/6 green):

- Negative: a `Holder{init{inner}}`-style receiver home that is
  `RetainedUnavailable` produces no `root_instance_call` row while
  `expected` stays set — `is_empty` therefore reports clean coverage.
- Positive: the existing `Pair`-shape releasable receiver still issues
  and consumes the row unchanged (`uses_selected_result_contract`).

Production re-measure (`--emit-mir-json`, fresh binary incl. this gate):

```text
binary-trees            -> [ordinary-new/local-commit/root-call-entry-missing]
mimalloc-lite           -> MIR JSON emit contract violation: unsupported terminator Invoke
boxtorrent-mini         -> MIR JSON emit contract violation: unsupported terminator Invoke
allocator-stress        -> [callable-semantic-package/issue] NamedArray(TextSourceMissing)
json-stream-aggregator  -> [callable-loop/route-not-front-selected]
                           LoopCondRouteRejected(SourceCallOutsideSelectedFamily)
                           JsonStreamAggregator.ingest/1 [Body(2)]
```

Premise correction to the D0 above: the D0 predicted the Plain-exit +
`expected` tolerance would absorb the withheld call, so the expected
terminal was the shared `Invoke` gap. That tolerance covers only
`Emitted{Plain}` exits. With `bench` `RetainedUnavailable`, the exit is
recorded `Unavailable` and finishing's `rebind_root_call_entry`
(`terminal_relation == Call` requires an `Emitted` root-call entry)
freezes with `root-call-entry-missing` before the `Invoke` serializer
gap is reached. The token is still the designed typed failure — the
ledger no longer strands an un-takeable `Ready` row, and the freeze
names the missing artifact at the enclosing contract — but the
observable terminal is `root-call-entry-missing`, not the `Invoke` gap.
`seal_finalized_root_birth_handoff` has the same `Call => Some(entry)`
requirement; both checks stand unmodified by design.

Boundary statement: `return bench.run()` on an `init`-field receiver
whose destruction is `Unavailable(FieldType)` is a Gate-1-typed stop —
this slice does not extend release semantics to unreleasable homes.

### D0 decision — `MIRBUILDER-GATE1-ROOT-CALL-UNRELEASABLE-RECEIVER-D0` (accepted)

Census of the four gate points that see a Call terminal:

- `validate_root_home_exit` (`root_home.rs`): `Unavailable` → `Ok` —
  already tolerant.
- `rebind_root_call_entry` (`root_call_entry.rs`): `Unavailable` →
  `root-call-entry-missing` — the lone over-firing check; its job is to
  re-project an `Emitted{Call}` entry through finishing, but when the
  exit is `Unavailable` there is no entry by construction.
- `finalized_root_observation` / `artifact` gate: `Unavailable` →
  `artifact-source-unavailable` — designed artifact stop.
- `seal_finalized_root_birth_handoff` → `take_finalized_root_call`:
  `Unavailable` → `root-call-entry-unavailable` — designed seal stop.

```text
Decision: tolerate `RootHomeExitProgress::Unavailable` at
  `rebind_root_call_entry` — the same non-Call tolerance the function
  already grants `Emitted{Plain}`/`Emitted{MapGet}` entries. The
  `Call => Emitted` premise predates the S0 withhold; the truthful
  invariant is "an entry exists iff the lifecycle lane emitted the
  call". This corrects the finishing check, not the release family:
  no Ready row is issued, no `RootHomeExitEntry::Call` is fabricated,
  and the sealing lanes stay the sole rejection authority.
Source authority + canonical issuer: the ledger's recorded `root_exits`
  progress; `Unavailable` is the deliberate disposition implied by the
  withheld row, not an absent duty.
Non-authority: `take_finalized_root_call`, `seal_finalized_root_birth_handoff`,
  the artifact-observation gate, and all physical emission paths stay
  unchanged; the generic return lowering already emitted the invoke and
  the return truthfully.
Fail-fast boundary: document/artifact sealing keeps its designed typed
  stop (`root-call-entry-unavailable` / `artifact-source-unavailable`).
  No EXE or artifact claim is produced for this shape.
Smallest next slice (S0): `rebind_root_call_entry` — `Unavailable` arm
  returns `Ok(())`; focused test pins non-artifact finishing accepting
  the recorded `Unavailable` exit on a Call terminal and the seal still
  rejecting it.
Non-claims: `Unavailable` is not `Emitted{Plain}`; no release semantics
  extend to unreleasable homes; binary-trees' `--emit-mir-json` terminal
  moves to the designed `root-call-entry-unavailable` seal boundary —
  the plain `compile_normal` (non-sealing) lane may now complete, which
  is honest because the module it emits carries the recorded
  `Unavailable` observation. The direct-call sibling shape
  (`return helper(30)` + unreleasable home) still strands its port-side
  loan at `DirectCallLoanNotConsumed`; it is the same family but is not
  exercised by any Gate-1 app — left as a named sibling boundary.
```

S0 landed (2026-09-28): `rebind_root_call_entry` returns `Ok(())` for a
recorded `Unavailable` progress before destructuring `Emitted` — nothing
is rebound, no entry is fabricated, and every other progress keeps the
existing `root-call-entry-missing` freeze. Focused pin
`unreleasable_root_call_receiver_passes_finishing_but_not_the_seal`
(`ordinary_new_terminal_result_tests.rs`) lowers
`return holder.run()` on an `init`-field receiver: draft validation,
non-artifact finishing, and the sealing rejection
(`root-call-entry-unavailable`) all behave per the census. binary-trees
`--emit-mir-json` now reaches the designed seal boundary; `--dump-mir`
completes and the emitted `Main.main` carries the truthful generic
`call_method BinaryTreesBench.run()` invoke.

## D0 resolved: MIRBUILDER-GATE1-INVOKE-TERMINATOR-JSON-EMIT-D0 (2026-09-28)

A read-only worker census found the ownership question already answered
by an accepted decision: `MIRBUILDER-INVOKE-LIFECYCLE-JSON-TERMINATOR-D0`
(`design/mirbuilder-final-pipeline-ssot.md`) — lifecycle `Invoke`,
`InvokeNormalResult`, and `ReturnFault` travel the
`hako.published-lifecycle-physical-program.v2` transport; generic
`src/runner/mir_json_emit` deliberately keeps rejecting `Invoke` as the
fail-fast boundary. This Gate-1 D0 therefore resolves to "no contract
change": the emit gap is a mis-attributed boundary, and the real work is
re-pointing app acceptance at the physical transport.

```text
Decision: lifecycle Invoke does not enter generic mir_json_emit; its
  typed rejection is the designed fail-fast (accepted SSOT D0). Gate-1
  app evidence for Invoke-carrying apps routes through the published
  physical v2 transport, not --emit-mir-json.
Source authority + canonical issuer: PublishedMirBackendView ->
  physical_program_json.rs emits hako.published-lifecycle-physical-
  program.v2 -> LifecycleInvocationInputV1 ->
  compile_published_lifecycle_physical_v4 -> C V4, invoked via
  emit_published_view_exe / compile_published_view_object.
Non-authority: generic --emit-mir-json, selfhost_build.sh --mir, and the
  ny-llvmc generic-JSON canary reader (const/ret/print only) stay
  harness lanes; no reader gains an `invoke` arm.
Fail-fast boundary: generic emit keeps its typed rejection; physical-
  route stops surface as typed upstream freezes, never emit-shape drops.
Smallest next slice (S0): re-point typed_object_untyped_field_min_exe
  off `selfhost_build.sh --mir` + ny-llvmc generic ingress onto the
  direct `--emit-exe` physical route, keeping the designed-rejection leg
  as a negative pin; plan pins re-observe through the physical document
  vocabulary or --dump-mir. Record the relocated true terminal
  (observed: artifact-source-unavailable — unreleasable handle-field
  home, the same family as the landed root-call D0). Also record the
  already-landed INVOKE-LIFECYCLE-PHYSICAL-V2-CUTOVER-I0 evidence:
  typed_object_method_min_exe PASS (exit 30, lifecycle-v4 + llvm-c-api).
Non-claims: no generic schema/reader change, no Invoke semantics change,
  no Gate-1 PASS; json-stream-aggregator still stops upstream at
  route-not-front-selected before any emit question arises.
```

Measured terminals (2026-09-28, release binary `d17246c4a2`-era):
`typed-object-untyped-field-min` `--emit-mir-json` -> designed
`unsupported terminator Invoke` rejection; `--emit-exe` (physical v2)
-> `[ordinary-new/local-commit/artifact-source-unavailable]`, the real
upstream admission boundary. `typed_object_method_min_exe` smoke PASS
end-to-end through the physical transport (exit 30).

S0 landed (2026-09-28): `typed_object_untyped_field_min_exe.sh` now pins
the designed Invoke rejection as a negative leg (`--emit-mir-json` must
fail with `unsupported terminator Invoke`) and runs the acceptance leg
through the selected physical route (`--emit-exe` + lifecycle V4 +
llvm-c-api trace pins + EXE exit 7). The unreachable generic-JSON plan
pins were dropped; `typed_object_plans` serialization stays covered by
the `mir_json_emit` unit tests. Observed honest terminal:
`[ordinary-new/local-commit/artifact-source-unavailable]` — `Holder`'s
`items` field is an ArrayBox handle, so the home can never reach
`end_available` and the artifact gate rejects the program. That is the
unreleasable-home family already named by the root-call rows; the
smoke is correctly red at the true boundary instead of at a
mis-attributed emit gap. No EXE artifact or acceptance is claimed.
