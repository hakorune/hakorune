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
| 9b | DONE — `MIRBUILDER-GATE1-UNRELEASABLE-HOME-ARTIFACT-ADMISSION-D0` landed below: `Unavailable` homes are NOT admittable — the stop is designed at three independent layers (observation `artifact-source-unavailable`, lifecycle coverage `artifact-unowned-lifecycle-site`, physical ABI `object-destruction`/`layout-field-drift`). Releasability for owning/non-`i64` fields is the parked `OWN-FIELD-CONTAINER-DEST-D0`/`VerifiedTerminalHomeDropPlanV1` family (+ tagged dynamic slot ABI for `init{}` storage) — a named Gate-1 dependency, not unilaterally reopened. binary-trees, untyped-field-min, boxtorrent-mini, and json-stream-aggregator await it. |
| 9c | DONE — `MIRBUILDER-GATE1-MIMALLOC-LITE-EXE-ROUTE-S0` landed: `mimalloc_lite_exe.sh` re-pointed off `selfhost_build.sh --exe` (generic JSON ingress) onto the direct `--emit-exe` physical route with the designed `Invoke` rejection kept as a negative pin (lifecycle-v4 + llvm-c-api trace pins). `MiWorkload` is fieldless -> `PlainI64NoHook`, so the observation gate passes; the smoke is correctly red at the true terminal `artifact-unowned-lifecycle-site` (builtin-box method Invokes the ledger does not own). allocator-stress / boxtorrent-mini / json-stream-aggregator smokes keep the same generic-ingress attribution issue for their own frontier rows. |
| 9d | DONE — `MIRBUILDER-GATE1-ARTIFACT-LIFECYCLE-COVERAGE-D0`/`S0` landed below: coverage stays universal; a child owner carrying a `RetainedUnavailable` ordinary commit row now freezes `artifact-source-unavailable` before coverage checking (root-order parity, plus it closes the no-birth bypass where `NewBox`-only emission passed coverage entirely). Claimless non-initializer `new` keeps the designed `artifact-unowned-lifecycle-site`. mimalloc-lite's observed terminal is unchanged (allocate's `return new` sorts first). |
| 9e | DONE — `MIRBUILDER-GATE1-RETURN-POSITION-NEW-CLAIM-D0` accepted below: a bounded return-position-only result-new commit family is admitted (OrdinaryNewClaimLedgerV1 extended with a result-position variant; `TerminalReturnedSourceV1::Construction` required; selected Invoke-shape emission + bindings; ReturnHandoff-style transfer-to-caller). Argument/field positions stay rejected (`artifact-unowned-lifecycle-site`); Unavailable constructions keep `artifact-source-unavailable`. Result-ABI Handle arm is a separate downstream family — no suite app goes green on this lane alone. |
| 9f | DONE — `MIRBUILDER-GATE1-RETURN-POSITION-NEW-CLAIM-S0` landed below: `OrdinaryNewClaimLedgerV1` + `ResultNewHomePrefixV1` + `NewResultCommitV1` share the single ledger/emission owner; membership is proven by `exact_stmt` (`Return{value}` parent), never by the shared `Value` segment; validation requires the emitted object at `Return{value}` exactly once. Focused: 6 semantic tests + 2 artifact tests green; `return new Point(1,2)` (child and `Main.main`) now passes artifact validation entirely. Argument/field positions stay out; retained-unavailable keeps `artifact-source-unavailable`. |
| 9g | DONE — `MIRBUILDER-GATE1-RETURN-HANDLE-RESULT-ABI-D0` accepted below: `InvokeCallResultKind::Handle` admitted for callees whose sealed terminal is `Value(Construction)`; `callable_result_classes` stays the sole class-name authority, `call_result_kind` the sole issuer; the caller installs the received object as an owned Home owing exactly one release (ReturnReceive precedent). S0 bounds to the direct-call lane; the lexical `w.make()` row lacks any result field and is the named sibling `MIRBUILDER-GATE1-LEXICAL-CALL-HANDLE-RESULT-D0`. No root-ABI or annotation lanes touched. |
| 9h | DONE — `MIRBUILDER-GATE1-RETURN-HANDLE-RESULT-ABI-S0` landed below: direct-call `local h = make(0)` (bare-name `FunctionCall` lane; `Work.make()` is a `Call`/lexical sibling) installs the callee's canonical object as an owned Home owing one `HomeRelease`; `InvokeCallResultKind::Handle` + `InvokeNormalResult` emit once, `MirType::Box("Point")` typed, no Copy/NewBox; negatives reject (returned-Home binding, annotated-construction mismatch); 29/29 lifecycle tests green. |
| 9i | DONE — `MIRBUILDER-GATE1-LEXICAL-CALL-HANDLE-RESULT-D0` accepted below: `local h = w.make()` (AST `Call`/`method_calls`) joins the Handle-result lane for claim-local receivers only (`local w = new W()`); the disposition row gains a co-sealed `result` at issue (issuer.rs:727 sees terminal_relation + callable_result_classes + result_contracts); a `method_calls` observation arm mints the Handle `LocalCallObservationV1` during scan; `CallReceivedCommitV1`, the verifier pairing (`Callee::SameModuleInstance × Handle`), and `OrdinaryHandle` publish carry over unchanged. Emission must be Invoke-shaped — the existing `CoreEffectPlan::DeclaredInstanceCall` emits a plain `Call` and stays the non-lifecycle path; a port-mediated emit mirroring `emit_local_lifecycle_call_v1` takes Handle rows. Parameter/nested/rebound receivers and the raw member route keep their current reject/dynamic boundary — a Handle-sealed site reaching the dynamic route freezes, never degrades. |
| 9j | DONE — `MIRBUILDER-GATE1-LEXICAL-CALL-HANDLE-RESULT-S0` landed below: `local w = new W(); local h = w.make()` seals one Handle `LocalCallObservationV1` from `method_calls` membership (claim-local receiver, non-rebound, sole `new` initializer, unique selected `InstanceBoxMethod`, all-i64 exact formals, callee construction result); the disposition row co-seals `result=Handle` at issue against the callee's retained `Value(Construction)` terminal + unannotated contract + result-class claim. Emission is Invoke-shaped (`Callee::SameModuleInstance` × `Handle`) through the port hook; the receiver Home unwinds on the call fault path and releases once per exit path; the received object installs as owned Home releasing once on the normal path. Instance-method `return new` retains its `Value(Construction)` terminal relation even when receiver-entry demands make home-prefix flow unavailable (exact-site evidence; flow gaps stay in `result_prefixes`/claims). Negatives stay dynamic (rebound receiver, non-construction callee); result-kind/cleanup drift reject. 32/32 lifecycle tests green. Gate 1 remains unsatisfied. |
| 10 | NEXT — `MIRBUILDER-GATE1-EXE-SUITE-ACCEPTANCE-S0`: fixed 11-entry EXE suite under the recorded LLVM 18 profile, after changed owners' focused checks. Gate 1 remains unsatisfied until its actual acceptance closes; then follow language conformance -> mimalloc gate -> Facts migration/selfhost. |

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

## D0 resolved: MIRBUILDER-GATE1-UNRELEASABLE-HOME-ARTIFACT-ADMISSION-D0 (2026-09-28)

A read-only worker census found the question already answered by the
existing design at three independent layers. `Unavailable` is not a
retained/allowed disposition — it is a bookkeeping state that only the
non-artifact lanes tolerate. Admitting it would require defeating all
three gates at once, and the designed path to releasability for
handle/non-`i64` fields belongs to a named parked family, not to this
lane.

1. Observation gate — `ordinary_new_local_commit/root_validation.rs`:
   artifact finishing requires `SourceCompleteAtFinalization` (or
   `NoSelectedLocalNew`); `Unavailable` freezes
   `artifact-source-unavailable`.
2. Lifecycle coverage gate — same file: every
   `requires_lifecycle_validation()` instruction must be ledger-bound;
   a `RetainedUnavailable` new lowers as generic `NewBox` + bare
   `Call{BirthConstructor}` and fails `artifact-unowned-lifecycle-site`
   even if gate 1 were loosened.
3. Physical ABI gate — `published_backend_view/physical_abi.rs`: every
   referenced object must seal `PlainI64NoHook` (`object-destruction`)
   and every field must store `I64` (`layout-field-drift`). The
   published physical v2 profile is I64-field-only by design.

Language-contract check: `docs/reference/language/lifecycle.md` owns
scope-end release as a contractual obligation — there is no
"process exit frees all" carve-out, and `home_release_plain_i64_v1`
explicitly requires the published PlainI64NoHook admission (slot tags,
names, absent hooks never establish permission). The designed terminal
protocol `hook -> reverse field release -> structural drop`
(`ownership-home-model-ssot.md`) exists only as the parked family
`OWN-FIELD-CONTAINER-DEST-D0` (destination matrix, parked in the
home-ownership task order) + `VerifiedTerminalHomeDropPlanV1` /
`OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0` (decision accepted via the
C′ SSOT; implementation stays parked). Untyped `init{}` storage
additionally awaits the tagged dynamic/opaque slot ABI owner —
`MIRBUILDER-UNTYPED-OBJECT-STORAGE-D0` already keeps
`untyped_field_min` as a `NoSafeSlice` sentinel.

```text
Decision: the artifact lane does NOT admit homes whose root exit stays
  Unavailable; Gate-1 acceptance requires releasable homes. The stop is
  designed and enforced at three independent layers, so no single-gate
  relaxation is available or wanted.
Source authority + canonical issuer: the sealed DestructionDisposition
  from instance_constructor_semantic/object_definition.rs (source-
  declared field types only); RootHomeExitProgress is the sole exit
  owner; the physical ABI gate is the final admission enforcer.
Non-authority: generic mir_json_emit; runtime slot tags/names/absent
  hooks; any retained-release or process-exit shortcut — none exists in
  the language contract.
Fail-fast boundary: artifact-source-unavailable stays; behind it sit
  artifact-unowned-lifecycle-site and object-destruction/
  layout-field-drift. No gate is loosened.
Smallest next slice (S0): record the named dependency and continue
  Gate-1 with suite members that do not require the parked family —
  re-point mimalloc_lite_exe.sh off generic JSON ingress onto
  --emit-exe (its MiWorkload home is fieldless -> PlainI64NoHook) and
  record the true terminal. Measured 2026-09-28: physical route reaches
  artifact-unowned-lifecycle-site — builtin-box method Invokes
  (heap.alloc/...) are lifecycle instructions the ledger does not own.
  That coverage owner is a separate frontier, named below.
Non-claims: no retained-release admission; no field-wise release
  implemented; no unilateral reopen of the parked
  OWN-FIELD-CONTAINER-DEST-D0 / VerifiedTerminalHomeDropPlanV1 family;
  no claim the 11-entry suite can close — binary-trees (bench object
  fields), typed_object_untyped_field_min (ArrayBox handle + non-I64
  storage), boxtorrent-mini (untyped store fields), and
  json-stream-aggregator (upstream route-not-front-selected; agg home
  also FieldType) all await that family or the slot-ABI owner.
```

Named Gate-1 dependency recorded: the parked
`OWN-FIELD-CONTAINER-DEST-D0` destination matrix (plus the tagged
dynamic slot ABI for `init{}` storage) is required before any suite
member with an owning handle/non-`i64` field can reach
`end_available`; loosening the observation gate alone is not a slice —
coverage and physical-ABI gates reject downstream regardless.

## D0 resolved: MIRBUILDER-GATE1-ARTIFACT-LIFECYCLE-COVERAGE-D0 (2026-09-28)

A read-only worker census plus a live `HAKO_COVERAGE_DEBUG` probe pinned
two distinct unowned-site producers in mimalloc-lite — and revealed that
`artifact-unowned-lifecycle-site` is the CORRECT terminal for one of
them.

Measured (release binary, `apps/mimalloc-lite/main.hako`):
`fn=HakoAllocPage.allocate/1` emits a bare
`Call{BirthConstructor{HakoAllocHandle.birth/3}}` in block 39 — the
`return new HakoAllocHandle(me.page_id, block_id, requested_size)` at
`page_heap_box.hako:102`. Non-`[Body, Initializer]` `new` sites are
outside the local-commit claim lane: `ordinary_new_coseal.rs:219-223`
`birth_site_index` admits only the typed `Callee::BirthConstructor`
edge and "issues no destination, home, or lifecycle authority". The
raw lane emits the instruction but no ledger binding exists — so
coverage correctly rejects it as genuinely unowned. Verified further
with a minimal probe: `return new Point(1,2)` in a plain-i64,
declared-birth box (`Maker.make/0`) hits the same freeze —
return-position `new` is ALWAYS artifact-inadmissible today, even for
a releasable box.

Second producer (separate child): `MiWorkload.run`'s
`local heap = new HakoAllocHeap()` is a CLAIMED initializer-site new
whose construction claim is `RetainedUnavailable` (HakoAllocHeap has
box-typed fields). The raw lane emits `NewBox` + bare
`Call{BirthConstructor}`; `root_validation.rs` contributes no binding
for `RetainedUnavailable` rows (`=> continue`). Before this slice the
freeze surfaced as `artifact-unowned-lifecycle-site` — a mislabel, the
site IS claimed, just unavailable. Worse, for a `NoBirthZero` box the
raw lane emits only `NewBox` — not lifecycle-required — so a child
holding an unreleasable no-birth home could pass coverage entirely and
defer to the physical ABI gate. The landed row-9b decision
("Unavailable homes are inadmissible") was enforced at the ROOT via
the observation gate but had NO child-level equivalent:
`validate_finalized_child_functions` went straight from
`validate_complete` to coverage.

Coverage scope question answered by existing design:
`docs/reference/mir/INSTRUCTION_SET.md:238-245` documents "every
lifecycle site must match its exact emitted binding" and the two-sided
check exists to catch stray lifecycle instructions
(`brand_catalog_new_completion_tests.rs:400-423` pins it). No
Home-only carveout is documented; narrowing coverage would rescind a
documented invariant — rejected.

```text
Decision: coverage stays universal — every requires_lifecycle_validation
  instruction must be ledger-bound. Two sub-fixes follow: (a) children
  holding RetainedUnavailable commit rows freeze
  `artifact-source-unavailable` BEFORE coverage, matching the root's
  observation ordering (label parity + closes the no-birth
  coverage bypass); (b) claimless non-initializer `new` sites
  (return/argument position) keep their designed
  `artifact-unowned-lifecycle-site` terminal — they are truly unowned.
Source authority + canonical issuer: the same OrdinaryNewClaimLedgerV1
  local_commits rows — `finalized_root_observation`'s
  NewEmissionUnavailable arm is the root-side precedent (the function
  itself is not reusable for children: it consults root_completion and
  root_exits, which child owners lack; the child check inspects the
  owner's own commit rows directly).
Non-authority: raw-lane Call{BirthConstructor} binding (would legitimize
  an Unavailable state inside the artifact lane — contradicts 9b);
  any coverage narrowing; birth_site_index entries (admit the call
  edge only, never lifecycle authority).
Fail-fast boundary: artifact-unowned-lifecycle-site remains correct for
  genuinely unowned sites (claimless return/argument-position `new`,
  stray lifecycle instructions). artifact-source-unavailable is the
  terminal for claimed-but-unavailable rows.
Smallest next slice (S0): in validate_finalized_child_functions, under
  `artifact`, before validate_artifact_lifecycle_coverage, freeze
  `artifact-source-unavailable` when the child owner's ordinary commit
  rows contain RetainedUnavailable. Verified: a minimal app whose child
  `Worker.run` holds `local h = new Holder()` (Holder has an ArrayBox
  field -> RetainedUnavailable) now reports `artifact-source-unavailable`.
  mimalloc-lite's observed terminal stays `artifact-unowned-lifecycle-
  site` because `HakoAllocPage.allocate`'s claimless `return new` sorts
  first — correct per this decision.
Non-claims: no Unavailable admission anywhere; no new binding family;
  non-initializer `new` claim coverage is a SEPARATE named family
  (required for allocator-style apps that return fresh handles);
  mimalloc-lite stays inadmissible on three grounds — claimless
  return-new, `HakoAllocHandle.requested_size: usize` FieldType, and
  box-typed HakoAllocHeap fields; allocator-stress/boxtorrent/
  json-stream smokes still carry the generic-ingress attribution issue
  for their own rows.
```

### S0 implementation receipt — `MIRBUILDER-GATE1-ARTIFACT-LIFECYCLE-COVERAGE-S0` (landed)

`root_validation.rs::validate_finalized_child_functions` — under
`artifact`, before `validate_artifact_lifecycle_coverage`, the child
owner's `local_commits` ordinary rows are inspected; any
`NewEmissionProgress::RetainedUnavailable` freezes
`artifact-source-unavailable` (`freeze()` = `ordinary-new/local-commit`
prefix, same terminal as the root's `NewEmissionUnavailable`
observation arm).

Evidence (debug binary, `--backend mir --emit-exe`):

- `Worker.run` + `local h = new Holder()` where `Holder { init{items}
  birth(){} }` → `artifact-source-unavailable` (previously mislabeled
  `artifact-unowned-lifecycle-site`).
- Same with `Holder` carrying NO birth → `artifact-source-unavailable`
  (previously coverage passed silently — `NewBox` is not
  lifecycle-required — and the child escaped to
  `published-lifecycle-program/instruction-unsupported`; the bypass is
  now closed).
- Clean child (`Worker` only, `return w.run()` on `run(): i64`) → EXE
  written; admissible path unchanged.
- `apps/mimalloc-lite` → still `artifact-unowned-lifecycle-site`:
  `HakoAllocPage.allocate/1`'s claimless `return new` sorts before
  `MiWorkload.run`'s RetainedUnavailable row — the truthful first
  terminal.
- Focused test
  `artifact_child_rejects_retained_unavailable_commit_before_lifecycle_coverage`
  (both `init{items}` and `init{items}+birth` variants) — green.
  `normal_callable_semantic_package` filter: 356 pass / 3 fail, all
  three in `cargo_lib_red_baseline.failures.txt` — known debt.
  `root_catalog_lifecycle` filter: 55 pass.

Sequencing note: `return w.run()` only enters the armed-receiver lane
when the callee has a declared return type (`run(): i64`); an
unannotated `run()` stops earlier at the terminal-call
`artifact-source-unavailable` (`raw_ordinary_new_claim.rs:198`,
`root_instance_call_expected`) — a separate pre-existing admission
boundary, not this slice's concern.

Next design stop: row 9e
`MIRBUILDER-GATE1-RETURN-POSITION-NEW-CLAIM-D0`.

### D0 decision — `MIRBUILDER-GATE1-RETURN-POSITION-NEW-CLAIM-D0` (accepted)

Evidence (read-only audit + worker census):

1. `birth_site_index` (`collect_birth_site_index_v1`,
   `ordinary_new_coseal_issue.rs`) deliberately skips `[Body,
   Initializer]` sites and issues destination-less
   `VerifiedOrdinaryNewBirthRecipeV1`. `take_birth_site_recipe`
   (`ordinary_new_coseal.rs`) is consumed affinely in
   `new_expression.rs` only when no local-commit claim exists, and
   authorizes exactly the `Callee::BirthConstructor` edge — no
   destination, home, lifecycle, ownership, or ABI authority. No
   positive validation exists that every indexed recipe emitted its
   edge.
2. The terminal model has nothing to transfer:
   `TerminalReturnedSourceV1` carries no `Construction` arm, so
   `return new T()` fails terminal classification with
   `ReturnValueNotCovered`. `callable_result_classes` is provenance
   only (and `HakoAllocPage.allocate` — mixed `return null`/`return
   new` — does not qualify at all).
3. Emit shape is wrong for the artifact lane: the raw lane emits
   `MirInstruction::NewBox` + bare `Call{BirthConstructor}`
   (`ordinary_new_admission.rs`). Plain `NewBox` is NOT in the
   published program's supported set — `Invoke{NewBox}` and
   `Invoke{Call{BirthConstructor}}` are. `artifact-unowned-
   lifecycle-site` is therefore the designed, correct terminal today.
4. Result ABI is handle-free downstream:
   `InvokeCallResultKind = {Unit, I64, Map}`,
   `CompiledEntryRootResultV1 = {I64, Unit}`,
   `ExactTrivialScalarAbiV1` is i64-only. `allocate(requested_size)`
   and `make(depth, value)` are unannotated (`Unannotated` result
   contract — not seed-rejected), but a caller edge still cannot
   name a handle result kind; `local h = w.make()` needs a Handle
   arm that does not exist. The lifecycle SSOT's own blessed factory
   idiom (`constructor-birth-new-lifecycle-ssot.md` — `makeSmall(
   page_id: PageId): HakoAllocPageModel { return new ... }`) uses a
   declared box return type that `UnsupportedResultAnnotation`
   rejects today — the result-ABI family is a required companion.
5. Physical ABI gates beyond coverage: `PlainI64NoHook` requires
   every field declared i64; birth actuals must be `Integer|Bool` —
   `new HakoAllocHandle(me.page_id, block_id, requested_size)`
   passes a field-read actual → `actual-kind-unavailable`.
6. Upstream unavailability dominates both suite apps anyway:
   `TreeNode{init{left,right,value}}` (untyped + box fields) and
   `HakoAllocHandle.requested_size: usize` → construction
   `Unavailable(FieldType)` → any honest commit row lands
   `RetainedUnavailable` → `artifact-source-unavailable` (row-9d
   ordering covers children). The claim lane unblocks no suite app
   alone — handle-field ownership (parked
   `OWN-FIELD-CONTAINER-DEST-D0`) and usize fields are also required.
7. Position census: ~222 user-box `return new` in `lang/src`
   (carrier idiom; binary-trees 2, hako_alloc several); ~136
   `me.f = new` field-writes (field ownership — parked OWN lane);
   argument-position constructions common (callee transfer).
   Positions differ in ownership semantics — smearing them into one
   claim row violates one-meaning-one-authority.

```text
Decision: admit a bounded RETURN-POSITION-ONLY claim family — a
  result-new commit row co-sealed with the site: recipe + object
  identity + destruction disposition + emitted values +
  transfer-to-caller disposition + fault cleanup. The fresh object's
  ownership transfers to the caller at the Return edge (the map
  lane's ReturnHandoff/ReturnReceive pair is the semantic
  precedent); this frame owes no release. Coverage stays universal —
  the row contributes the Invoke{NewBox} +
  Invoke{Call{BirthConstructor}} bindings for its owner.
Source authority + canonical issuer: the existing non-initializer
  birth-site collection (birth_site_index /
  collect_birth_site_index_v1) is the membership source;
  OrdinaryNewClaimLedgerV1 is the canonical issuer — extended with a
  result-position commit variant (named, e.g.
  OrdinaryNewResultCommitV1), not a parallel ledger.
Non-authority: birth_site_index recipes alone (call edge only —
  unchanged); raw-lane emission (MirInstruction::NewBox + bare Call
  is replaced by the selected Invoke shape inside the claim, not
  bound in place); TerminalReturnedSourceV1 stays incomplete without
  its Construction arm (part of this family); caller-side Handle
  result kinds (no InvokeCallResultKind/CompiledEntryRootResultV1/
  FinalizedRootResultAbiV1 arm — separate ABI family, downstream).
Fail-fast boundary: non-initializer `new` OUTSIDE return position
  (argument position, `me.f = new` writes, field-decl initializers)
  keeps `artifact-unowned-lifecycle-site`. Return-position `new` on
  an Unavailable construction lands RetainedUnavailable →
  `artifact-source-unavailable` (row-9d ordering). `Return{value}`
  must carry exactly the claim's emitted object identity — mismatch
  freezes, never best-effort.
Smallest next slice (S0): MIRBUILDER-GATE1-RETURN-POSITION-NEW-CLAIM-S0
  — semantic foundation only: collect return-position sites into
  result-new commit rows; add TerminalReturnedSourceV1::Construction
  so terminal classification stops failing with
  ReturnValueNotCovered; emit Invoke{NewBox} +
  Invoke{Call{BirthConstructor}} under the claim and record bindings;
  verify Return{value} carries the emitted object. Evidence target:
  `Main.main { return new Point(1,2) }` (plain-i64 declared-birth
  box) moves past `artifact-unowned-lifecycle-site` to the truthful
  next boundary (root/caller-edge result ABI); `return new
  <unavailable box>` reports `artifact-source-unavailable`;
  argument-position `new` keeps `artifact-unowned-lifecycle-site`.
Non-claims: no suite app goes green — binary-trees and mimalloc-lite
  still fail upstream (Unavailable constructions / parked
  OWN-FIELD-CONTAINER-DEST-D0 / usize fields / field-read birth
  actuals). No argument- or field-position claims (distinct
  ownership semantics). No Handle arm in any result ABI. No
  caller-side ownership/release model for received handles (call
  results are not Homes — allocator-style manual release). No
  coverage narrowing. No proxy: birth_site_index entries do not
  mint lifecycle authority by themselves.
```

Next slice: `MIRBUILDER-GATE1-RETURN-POSITION-NEW-CLAIM-S0` — the
semantic claim foundation above.

### S0 landed — `MIRBUILDER-GATE1-RETURN-POSITION-NEW-CLAIM-S0`

Implementation (one semantic issuer, one physical owner):

- `OrdinaryNewSiteResolutionV1` (`ordinary_new_candidate.rs`) splits
  position-independent resolution — ordinary-box coverage, canonical
  object identity, destruction disposition, construction eligibility,
  Birth recipe + ABI handoff — from the destination-bound local claim.
- `OrdinaryNewResultClaimV1` (`ordinary_new_coseal.rs`,
  `ordinary_new_claim_access.rs`) is the destination-less claim: exact
  site, parser ordinary-box source row, class/arity, constructor
  disposition, `ResultNewHomePrefixV1` (prior Homes + outward fault
  continuation + covered statements; no destination, no local
  declaration), canonical object, selected/trivial argument rows.
- Membership (`ordinary_new_coseal_issue.rs`): construction rows whose
  final segment is `Value` are admitted only when `exact_stmt` proves
  the parent is `Return{value}` — `Value` also serves assignment,
  compound-assign, print and nowait children, so segment shape alone
  never decides. Nested non-statement constructions are non-membership;
  uncovered/builtin classes keep their existing lane. Claimed sites are
  skipped by `birth_site_index` (no double authority).
- The ledger stays the sole owner: `result_claims` map + affine
  `try_take_result` (exact site/class/arity, prefix-required-unwind and
  prior-Homes consistency checked); `LocalCommitV1::Result(
  NewResultCommitV1)` shares `NewEmissionProgress` with local rows —
  `mark_result_checked` replaces local installation because the emitted
  object leaves with `Return{value}`.
- `issue_result_new_fault_continuation_v1` (`resolved_control_flow`) is
  the destination-less sibling of the existing fault-continuation
  issuer — identical scope/outward-target checks, no initializer
  relation by design.
- Owners holding result sites enter the homes-aware verify so the
  `Value(Construction)` terminal relation and the attached homes flow
  are co-sealed; `retain_child_terminal_relation` retains `Value` only
  when `returned` is `Construction` — direct-call admission still
  rejects it (`(None, Value(Construction))` is uncovered), which is the
  designed boundary until the result-ABI family lands.
- Builder: `PreparedRawNewExpressionV1` takes the result claim when no
  local-initializer claim owns the site; emission reuses the selected
  Invoke shape (`Invoke{NewBox}` + `Invoke{Call{BirthConstructor,
  Unit}}` + bindings + optional reclaim). `complete_ordinary_new_
  expression` forwards only when `has_result_new_commit`, so non-return
  `Value` children stay untouched.
- `emission_validation` gained the Result arm: the emitted object must
  reach `Return{value}` exactly once (`result-return-drift`); the
  shared constructor/argument/reclaim/binding checks apply unchanged.

Evidence (all green):

- `ordinary_new_result_claim_tests.rs` — 6 tests: positive claim +
  `Value(Construction)` relation + birth-index exclusion + covered
  prefix; non-trivial argument stays truthfully retained; argument-,
  field- and assignment-position constructions mint no claim; nested
  `if`-return `new` still mints claims but records `PrefixNotCovered`
  (the scan does not descend branch bodies — truthful unavailability,
  never silent fallback).
- `ordinary_new_coseal_tests::birth_site_index_covers_field_assign_
  sites_while_return_position_claims` — updated pin: claimed
  return-position sites leave the destination-less index.
- `artifact_child_accepts_return_position_new_under_its_result_claim`
  — `Work.make { return new Point(1,2) }` lowers under the claim and
  passes artifact validation.
- `artifact_root_return_position_new_reaches_the_truthful_next_
  boundary` — `Main.main { return new Point(1,2) }` validates fully;
  `artifact-unowned-lifecycle-site` is gone for the admitted family.

Boundaries kept: argument/field positions have no claim family;
`RetainedUnavailable` children still freeze `artifact-source-
unavailable` (row 9d); no Handle result-ABI arm, no caller-side
received-handle ownership, no coverage narrowing.

Regression: `cargo test --lib normal_callable_semantic_package` 359
pass / 3 fail — all three are recorded baseline debt
(`cargo_lib_red_baseline.failures.txt`); `mir::builder` filter showed
baseline failures plus one order-flaky `array_source_binding...` row
that passes standalone.

Next frontier (design, scheduler-selected): a caller-side Handle
result kind so `local h = w.make()` can name the transferred object —
`MIRBUILDER-GATE1-RETURN-HANDLE-RESULT-ABI-D0`. Gate 1 stays
unsatisfied until its actual acceptance evidence closes.

### D0 decision — `MIRBUILDER-GATE1-RETURN-HANDLE-RESULT-ABI-D0` (accepted 2026-09-28)

Census (read-only worker + direct read; boundary: `resolved_semantics`,
`normal_callable_semantic_package`, `instruction/invoke*` +
`verification/invoke*`, `builder/ordinary_new_admission/selected`,
`compiler/normal_default_pipeline` — backend internals and non-
lifecycle call lanes excluded):

1. `InvokeCallResultKind = {Unit, I64, Map}`;
   `InvokeNormalResultKind::Handle` already exists (NewBox lane) — the
   physical projection vocabulary is present; only the call-result
   arm is missing. ~118 `InvokeCallResultKind` hits in 37 files all
   consume the single enum column.
2. Sole result-class issuer: `call_result_kind`
   (`direct_call_lifecycle.rs`) classifies ONLY from the callee's
   sealed `terminal_relation`. Header `signature.result()` is an
   admission check for the annotated-I64 lane, never classification —
   the Map precedent: `None` header + sealed terminal proves the
   class (`direct_call_lifecycle.rs` doc). `Value(Construction)`
   today falls through to `I64` classification while the co-seal
   coverage arm is `false` → reject; fail-closed by design (S0
   boundary).
3. Two caller lanes end at the same missing arm: `Work.make(...)`
   direct (`DirectCallDispositionRowV1.result`, co-sealed) vs
   `w.make()` lexical (`LexicalInstanceCallDispositionRowV1` carries
   NO result field and no terminal co-seal at all — receiver-class →
   target only). The lanes share class authority but differ in
   transport row.
4. Caller local-flow: `LocalCallResultClassV1 = {I64, Map}`; an
   unrecognized `local h = <call>` lands `install_inventoried_call_
   result` → `BoundValue` + `PrefixNotCovered` — the received object
   never becomes an owned Home, so cleanup can never prove a release.
5. Ownership vocabulary already anticipates this:
   `HomeResultRelationV1::HomeToCaller`/`SharedHomeToCaller` are
   declared-but-unused (classify_result only issues I64UnitTrivial);
   the map lane's `ReturnReceive`/`MapDestinationV1::ReturnBoundary`
   is the provenance precedent — caller acquires at the Return edge
   and owes exactly one release.
6. Publish seams enumerated: `verification/invoke.rs` callee/result
   pairing (needs a `(Some(<handle repr>), Handle)` arm);
   `terminal_call.rs::result_type` (`_ -> Integer` fallback would
   silently mistype a Handle — must return `MirType::Box(class)`);
   `physical_program.rs` `(result, return_type)` pairing →
   `OrdinaryHandle` role; `physical_program_json.rs` wire tag;
   `compiled_entry_contract.rs` ordinary-call scan.
7. `callable_result_classes` already seals `{selected key -> class}`
   for callees whose every return row constructs `new` of one agreed
   box (row-5 lane) — the exact class-name provenance a `Handle` arm
   needs to type `MirType::Box(class)`.
8. `CompiledEntryRootResultV1`/`FinalizedRootResultAbiV1` are the
   root entry's return ABI, orthogonal to a call-site result class —
   they matter only when `Main.main` itself returns a handle (separate
   family). `ExactTrivialScalarAbiV1` is scalar-only by design — do
   NOT extend; `Handle` rides the `None`-header lane like `Map`.

```text
Decision: admit InvokeCallResultKind::Handle as the call-result
  class for callees whose sealed terminal is
  TerminalRelationV1::Value(Construction) — a transferred owned
  object. Caller acquisition installs the received object as an
  owned caller Home owing exactly one release (the map lane's
  ReturnReceive is the semantic precedent; HomeToCaller is the
  declared-but-unused result vocabulary for it); the callee owes
  none — the result commit already forbids a callee-side End.
Source authority + canonical issuer: the callee's sealed
  terminal_relation (Value(Construction)) plus the existing
  callable_result_classes claim — sole class-name authority, both
  already issued upstream. call_result_kind remains the sole
  result-class issuer; co_seal_lifecycle gains the coverage arm.
Non-authority: header result annotations (Handle rides the
  None-header lane like Map, never the ExactTrivialScalarAbiV1
  lane); generic JSON paths; names; LexicalInstanceCallDisposition-
  RowV1 (no result field — separate row decision named below);
  birth_site_index.
Fail-fast boundary: opaque/unproven call results stay rejected; the
  existing local-call argument-shape gate is unchanged; a row/emit
  result-class mismatch freezes; a RetainedUnavailable callee
  freezes the caller artifact-source-unavailable — the caller never
  silently degrades a handle to BoundValue.
Smallest next slice (S0): direct-call lane only —
  MIRBUILDER-GATE1-RETURN-HANDLE-RESULT-ABI-S0. LocalCallResult-
  ClassV1::Handle + issue arm; call_result_kind Construction→Handle
  arm + (None, Value(Construction)) coverage arm; emit_local Handle
  arm typing MirType::Box(class); verifier pairing arm;
  OrdinaryHandle publish role + wire tag. Evidence target: `local h =
  Work.make()` with `make { return new Point(1,2) }` installing an
  owned caller Home; negatives: opaque callee reject, annotation
  mismatch reject, missing class claim reject.
Non-claims: no lexical `w.make()` handle result this slice (the row
  needs its own co-sealed result field — named sibling row
  MIRBUILDER-GATE1-LEXICAL-CALL-HANDLE-RESULT-D0); no
  CompiledEntryRootResultV1::Handle / FinalizedRootResultAbiV1 arm
  (root `return <handle>` is a separate ABI family); no field/
  argument-position families; no generic non-new call-result
  handles; no coverage narrowing; Gate 1 stays unsatisfied.
```

### S0 landed — `MIRBUILDER-GATE1-RETURN-HANDLE-RESULT-ABI-S0` (2026-09-29)

Landed the caller-side direct-call Handle result edge:

- `LocalCallResultClassV1::Handle` + `issue_local_call` arm +
  `StoredLocal::ReceivedHandle` install (observes like a live Home
  without a `new` acquisition site); the caller's `RootHomeFlow`
  local-call row carries the class through `scan_new_home_flow`'s
  `local_handle_call` predicate.
- `construction_result_callee` proves the callee from sealed body
  facts only: last statement is a value `return`, every return
  constructs `new` of one agreed class, and the callee's retained
  `Value(Construction)` terminal carries the transfer — never the
  annotation. `call_result_kind` maps `Value(Construction)` to
  `InvokeCallResultKind::Handle`; `co_seal_lifecycle` gains the
  `(None, Value(Construction))` coverage arm requiring
  `owned.owner() == callee` + `result_transfer_proven` +
  a valid `callable_result_classes` claim.
- Result-class claim coverage widened from `InstanceBoxMethod` to
  every cataloged callable key: a static-box sibling like
  `Main.make` returning `return new Point(1,2)` now carries a claim
  (the `finish` ordinary-box filter is unchanged; field-write
  claims keep their instance-box `owner_box`).
- `LocalCommitV1::CallReceived` + `CallReceivedCommitV1`
  (Emitting -> ExpressionCompleted -> Installed -> Checked) records
  the callee's `CanonicalObjectIdV1`, installs the invoke result
  verbatim, and owes exactly one `HomeRelease{object, value}`.
- `emit_local` Handle arm emits one `Invoke{result:Handle}` + one
  `InvokeNormalResult`, types the received value
  `MirType::Box(class)` from `callable_result_class(published_key)`,
  and `local_placement` reuses the invoke result (no Copy) via
  `call_received_initializer_matches`.
- `validate_call_received_emission` independently re-checks the
  artifact: exact membership, binding/install, exactly one Handle
  invoke, one projection, one matching `HomeRelease`.
- Publication/backend: `OrdinaryHandle` role + wire tag + i64-free
  return-exit grouping.

Evidence: `local h = make(0)` with `make(args: i64) { return new
Point(1,2) }` lowers, verifies strictly, and validates the
finalized root; the received value is `MirType::Box("Point")`, the
caller exit releases the transferred canonical object exactly once,
and no caller-side `NewBox`/`Copy` mints a second object.
Negatives: `return <home-local>` and annotated `make(): i64`
construction mismatches reject before artifact. Qualified
`w.make()` (AST `Call`) stays deferred to the named lexical sibling;
root return ABI unchanged. Focused gates: 29/29
`direct_call_lifecycle` tests green (3 handle-result + 26 map/i64
regressions). Full-suite diff vs parent commit: zero
current-change failures — `birth_receiver_non_escape_...` fails
identically at `1c717970ec` (pre-existing Upvar-seal debt from
`38c1d2c276`); remaining reds reproduce at parent or pass in
isolation (parallel-suite flake).

Non-claims hold: no lexical receiver handle result, no root return
ABI, no field/argument positions, no silent fallback; Gate 1 stays
unsatisfied. Next frontier is the named sibling
`MIRBUILDER-GATE1-LEXICAL-CALL-HANDLE-RESULT-D0` (lexical row needs
its own co-sealed result field).

### D0 decision — `MIRBUILDER-GATE1-LEXICAL-CALL-HANDLE-RESULT-D0` (accepted 2026-09-29)

Census findings that shaped the decision:

- `LexicalInstanceCallDispositionRowV1` carries only
  `call_site`/`receiver_site`/`receiver_binding`/`target`/
  `target_batch_slot` — no result field, no argument sites.
- Its take is affine but plan-lane only
  (`LoopPlanExpressionPortV1::exact_source_declared_instance_call_v1`
  → `CoreEffectPlan::DeclaredInstanceCall` → a **plain
  `MirInstruction::Call`**, no Invoke/fault-frame/lifecycle hooks).
- `issue_local_call` sees only `direct_call_observations`; a
  `method_calls()` site falls to `install_inventoried_call_result`
  (`BoundValue` + `PrefixNotCovered`) — the current
  `local h = w.make()` goes out the dynamic member route.
- At disposition-issue time (`issuer.rs:727`) `terminal_relation`,
  `callable_result_classes`, and `result_contracts` are already
  sealed — so the row's `result` can be co-sealed there with the
  exact `RootInstanceCallDispositionRowV1.result` recipe; scan-time
  classification must instead work from `candidates` + `batch`
  (`construction_result_callee`-style walk), mirroring the
  direct-call scan predicate.
- Receiver provenance is bounded at scan time: only claim-local
  receivers (`local w = new W()`, sole-initializer `candidates`
  claim) are provable inside the per-declaration verify loop;
  parameter receivers need cross-function claims that do not exist
  yet at that point.
- Reuse is total downstream: `CallReceivedCommitV1`,
  `handle_call_source`/`begin_handle_call_emission`,
  `call_received_initializer_matches` placement, the verifier's
  `Callee::SameModuleInstance × Handle` pairing, and
  `OrdinaryHandle` publish all fit unchanged.

```text
Decision: admit `local h = w.make()` handle results in the lexical
  lane for claim-local receivers; co-seal `result` on the
  disposition row at issue; mint the Handle LocalCallObservationV1
  from a `method_calls` scan arm; emit Invoke-shaped (Invoke +
  fault landing + InvokeNormalResult), never plain Call.
Source authority + canonical issuer: `method_calls()` sealed rows +
  `candidates` claim-local receiver class + callee body-shape
  construction walk (scan); `call_result_kind` +
  `callable_result_classes` at disposition issue.
Non-authority: parameter/nested-handle/rebound receivers; raw
  member-route dynamic dispatch; name/type-based class guesses;
  the plain `DeclaredInstanceCall` Call path for Handle rows.
Fail-fast boundary: a Handle-sealed site reaching the dynamic
  member route freezes (unconsumed-Handle row audit +
  map_demands_consumed); unproven receivers keep NotSelected.
Smallest next slice: MIRBUILDER-GATE1-LEXICAL-CALL-HANDLE-RESULT-S0
  — scan arm + row field + Invoke emission + CallReceived install.
  Evidence: `local w = new W(); local h = w.make()` installs one
  owned Home with exactly one HomeRelease; negatives: parameter
  receiver, rebound receiver, unclassified callee.
Non-claims: no parameter/nested receivers; no raw-lane emission;
  no root return ABI; Gate 1 stays unsatisfied.
```

### S0 landed — `MIRBUILDER-GATE1-LEXICAL-CALL-HANDLE-RESULT-S0` (2026-09-29)

The lexical `recv.m(...)` Handle-result lane is wired end to end:

- **Scan** (`home_local_call_flow::issue_lexical_local_call`): reads the
  resolver-sealed `method_calls` inventory — never the AST again — admits
  only `Lexical(Local)` receivers owned by the current function and exact
  integer-literal arguments; mints `LocalCallObservationV1::Handle` with
  the live prior-home list.
- **Coseal predicate** (`lexical_handle_result_call`): proves the receiver
  binding is a `Local`, non-rebound, sole-initializer claim-local `new`
  (`candidates`), resolves one unique selected `InstanceBoxMethod` target,
  requires all-i64 exact formals from the callee's parameter contract, and
  requires `construction_result_callee` body-shape proof.
- **Disposition row** (`issue_lexical_instance_call_dispositions`): gains
  `callee_owner` + `argument_sites` + co-sealed `result`. `Handle` records
  only when caller observation AND callee `Value(Construction)` terminal
  AND unannotated declared result AND `callable_result_classes` claim all
  agree; half-sealed edges freeze (`handle-result-mismatch`).
- **Terminal relation retention** (`scan_new_home_flow`): an instance
  method's receiver-entry `EntryDemandMissing` no longer erases a
  `Value(Construction)` terminal — the exact-site construction evidence is
  retained while the flow gap stays in `result_prefixes`/the result claim;
  every other returned source still drops when homes are unavailable.
- **Emission** (`emit_local_lexical` via the `MemberCallRoutePlan::Standard`
  port hook): emits `Invoke{Call{callee: SameModuleInstance{key, receiver},
  result: Handle}}` with the sealed receiver value (never re-lowered) and
  sealed literal arguments; `InvokeNormalResult` projects the result typed
  `MirType::Box(class)`; the call's fault path unwinds prior Homes through
  `cleanup_chain` (same contract as `new` emission); `CallReceivedCommitV1`
  records begin/record/complete.
- **Non-fallback**: a Handle-observed site reaching the port without its
  disposition row freezes (`lexical-handle/disposition-missing`); sites
  with no observation keep the existing dynamic member route.

Evidence: `direct_call_lifecycle` 32/32 green —
`lexical_handle_result_call_installs_owned_home_and_releases_at_exit`
asserts one `SameModuleInstance`×`Handle` invoke targeting `Work.make`,
one `InvokeNormalResult` typed `MirType::Box("Point")`, path-sensitive
release accounting (receiver once on each of normal+fault paths, received
object once on the normal path), exactly one caller-side `NewBox`, no
`Copy` of the received handle, strict MIR verify, finalized-root
validation, and result-kind/cleanup drift rejection; the rebound-receiver
and non-construction-callee negatives stay dynamic.

Non-claims: parameter receivers, nested handle-result receivers, non-i64
arguments, annotated callee results, and `me`/`this` receivers stay on
their existing reject/dynamic lanes. Gate 1 remains unsatisfied — next
named frontier from row 10.
