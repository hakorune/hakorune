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
| 3 | `MIRBUILDER-GATE1-CALL-RESULT-ARGUMENT-COVERAGE-D0`: boxtorrent-mini's new frontier — `ingest`'s loop freezes at `Argument(1)`, the `chunk_data.length()` core-method call nested inside `manifest.addChunk(cid, chunk_data.length())`. `chunk_data` is a call-result local (`data.slice(...)`), so the site needs either argument-position core-method coverage for call-result receivers or a census of what the core bucket already admits. Allocator `handles.push(heap.allocate(...))` shares the argument-position-call shape; re-measure it in the same census. Call-result *instance* receivers (`positive.itemCheck`) stay parked. |
| 4 | Complete required app evidence: json-stream-aggregator EXE/output and typed-object JSON ingress/EXE exit 7. An emit-interface failure still blocks its registered acceptance; record its owner instead of dropping the row. The observed `Invoke` terminator JSON emit gap (`--emit-mir-json` serializer) is this row's dependency, not a semantic-lane blocker. |
| 5 | Fixed 11-entry EXE suite under the recorded LLVM 18 profile, after changed owners' focused checks. Gate 1 remains unsatisfied until its actual acceptance closes; then follow language conformance -> mimalloc gate -> Facts migration/selfhost. |

### D0 decision — `MIRBUILDER-GATE1-ORDINARY-NEW-ARTIFACT-SOURCE-D0` (landed)

```text
Decision: the freeze is Main.main's root-instance-call disposition for
  `return workload.run()` — `issue_root_instance_call_dispositions`
  (`ordinary_new_root_instance_call.rs`) owes a Ready row, and the owed
  artifact is the callee's sealed result contract, not the `new` commit.
Source authority + canonical issuer: the declared return contract
  `run(): i64`; `completion_seed.rs` maps `Unannotated`/`Void` to
  `result = None`, so an unannotated callee can never mint the row.
Non-authority: MIR/body inference of the result type is forbidden by the
  final-pipeline SSOT ("must not infer i64 from MIR"); no direct-call
  fallback may be manufactured (`root_call_entry.rs` invariant).
Fail-fast boundary: unannotated or non-I64 callee stays
  `artifact-source-unavailable`; `results.row` absent stays the same
  named stop; no flag/Err/Ok(()) contract change to the issuer.
Smallest next slice: `MIRBUILDER-GATE1-ORDINARY-NEW-ARTIFACT-SOURCE-S0` —
  declare `run(): i64` on `MiWorkload.run` (same contract as the
  `sum(): i64` precedent in typed-object-method-min), re-measure the app
  through `--emit-mir-json`, and record the next observable terminal.
Non-claims: no unannotated-return inference authority is created here
  (a separate design row may open it later); no claim-lane widening;
  no InstanceReceiver-parameter or Dynamic-origin call admission;
  no Invoke serializer / VM-lane `birth-global-legacy-stopped` claim —
  both are downstream emit/backend families, not this terminal.
```

D0 census evidence (read-only worker + `/tmp` probes, no repo change):
freezing owner is `Main.main` (the expected-flag set is populated only
for the app-main batch slot; `MiWorkload.run` returns literals and is
never flagged). Probe `box W { run(): i64 {return 7} }` + `new W()` +
`return w.run()` lowers to Invoke emit stage; the identical unannotated
probe reproduces `artifact-source-unavailable` exactly — the seed row
exists and `result = None` is the sole blocker. A probe whose callee
body itself makes an instance call (`new H()` + `h.alloc(0)` in `run`)
still lowers, so the callee's declared contract is independent of its
body composition. The fully annotated mimalloc-lite copy then lowers
EVERY function — `heap.allocate`, `handles.push(heap.allocate(...))`,
field reads, `me.*` births — and stops only at the known `Invoke`
terminator JSON emit contract, which the queue already owns under the
acceptance row. `bin_size/1` evidence from the landed S0 stands.

### S0 implementation receipt — `MIRBUILDER-GATE1-ORDINARY-NEW-ARTIFACT-SOURCE-S0` (landed)

The whole slice is the source-contract completion the D0 named: declare
`run(): i64` on `MiWorkload.run` (`apps/mimalloc-lite/main.hako:10`). No
Rust change — the canonical issuer, disposition checks, and negative
pins already exist; the app source was contract-incomplete, not the
compiler.

Evidence (quick-profile `./target/quick/hakorune`):
`--emit-mir-json` on `apps/mimalloc-lite/main.hako` advances from
`ordinary-new/local-commit/artifact-source-unavailable` to `MIR JSON
emit contract violation: unsupported terminator Invoke` — every function
lowers on the package lane (`return workload.run()` emits the root
instance call; `run`'s body emits `heap.allocate`/`handles.push`/
field reads; corpus `HakoAllocHeap.birth` with its `me.*` calls lowers).
The remaining terminal is the JSON serializer's Invoke gap already owned
by the acceptance row — a semantic-lane pass, not a new blocker. The
`--backend vm` lane separately stops at
`ordinary-new/birth-global-legacy-stopped`, a different downstream
family (D18 lineage), unchanged by this slice. Focused pins:
`root_instance_call_*` suite 5/5 including
`root_instance_call_uses_selected_result_contract` (positive) and
`root_instance_call_without_result_contract_stays_unavailable`
(negative). Next observable frontier for Gate 1: queue row 2
(parameter/Dynamic-origin receiver calls — `builder.make`,
`store.readData`, `itemCheck`), since claim-proven local receivers are
now measured as emitted. Gate 1 remains unsatisfied.

### D0 decision — `MIRBUILDER-GATE1-PARAM-RECEIVER-CALL-SOURCE-D0` (landed)

```text
Decision: admit parameter-receiver instance calls inside armed callable
  loops by co-sealing a caller->callee argument-provenance edge (the
  `map_argument_edge` precedent in `direct_call_lifecycle.rs`) that joins
  every caller argument carrying an ordinary-new claim of exactly one
  class to the callee's `Parameter{index}` binding; admitted loop items
  then emit `Callee::SameModuleInstance` with the already-installed
  parameter ValueId through the same emission contract the declared-
  instance locator arm uses.
Source authority + canonical issuer: caller-side ordinary-new claim and
  initializer ledger + callee `Parameter{index}` binding + selected
  `InstanceBoxMethod` catalog key + callee result/signature contract —
  the ordinary_new co-seal cohort (`issuer.rs` ~700) already joins the
  same ingredient set and stays the canonical issuer family.
Non-authority: Dynamic-origin products (binding-only, no nominal class),
  declared-type spellings (`OpaqueHandle`/`DeclaredHandle` admit no user
  box), MIR types, name matching, runtime dispatch.
Fail-fast boundary: zero/multiple classes across caller edges, a rebind
  between claim and call, ambiguous or missing target/result contract,
  a missing caller-edge claim -> named reject; non-admitted uncovered
  items keep `SourceCallOutsideSelectedFamily` — no silent drop.
Smallest next slice: `MIRBUILDER-GATE1-PARAM-RECEIVER-CALL-SOURCE-S0` —
  boxtorrent-mini `materialize(store)`/`releaseFrom(store)`, whose every
  caller argument is the direct `local store = new BoxTorrentStore()`
  claim (single class, no field provenance needed).
Non-claims: no call-result receivers (`positive.itemCheck` stays a
  separate queued family — its callee result contract drops the Home
  terminal relation today); no field-read argument provenance
  (binary-trees `builder` is `me.builder`, an S1+ shape); no non-loop
  param calls; no Dynamic-lane widening; no declared-type parameter
  admission; no Invoke serializer or VM-lane claims.
```

Census evidence (read-only worker + source reads): the reject is
`CallableLoopSourceTargetProbeV1::into_selected_relation`
(`normal_callable_loop_source_route_items.rs:389-442`) — a param-receiver
item contributes nothing to `selected` (`VerifiedSourceCallTargetCatalogV1`
filters non-static rows at `source_call_target/model.rs:162-172`), so
`materialize`'s loop (`ids.get` core + `store.readData` param) dies on the
uncovered remainder. The only landed local-receiver authority is
`issue_root_instance_call_dispositions` (app-main + terminal + zero-arg
scoped); its receiver chain — initializer -> claim -> class -> selected
`InstanceBoxMethod` -> result contract -> `InstanceReceiver` lane-0 —
is the same join this family needs at the call edge. Parameter bindings
already carry `BindingKindV1::Parameter{index}` and installed ValueIds
(`install_entry_values`); the missing join is caller argument -> claim ->
class -> callee param binding. No declared-type or Dynamic evidence can
supply it today.

The historical selected-C UserBox row owns `UnsupportedBeforeObject` /
`RetireAfterReplacement` and remains parked. This design does not choose
`RetainTransition`. Queued instance work must name a canonical consumer and
settle its source contract; backend fate cannot be changed by a broad family
label. The user's design request is handled here without requiring another
lane-choice question for the unparked call-free prerequisite.

Design closeout: read-only worker premise/integration review completed;
current-state pointer guard, qualified-route scope guard and diff check PASS.
Only the original-source probe above was executed; no Cargo build/tests or
EXE suite was run. The next implementation owns the focused Done evidence.

### S0 implementation receipt (landed)

Transport chain: `VerifiedCallableLoopCallFreeCoverageV1` is issued by
`CallableLoopSourceBridgeV1::from_input` (`issue_call_free_coverage` proves
the bounded grammar from resolver rows only), rides the one-shot
`Armed { projection, call_free }` take through `raw_loop_child_entry.rs` and
the generic Facts issuer, and co-seals into the token as
`CallableLoopSourceCallCoverageV1::CallFree`; `SourceLoopCondPhysicalInputV1`
re-validates the sum before `lower_loop_cond_break_continue_source` lowers.
New named rejects: `SourceCoverageForeign`, `SourceCoverageSiteMismatch`,
`SourceCallResidualEvidence`; unproven empty inventory stays
`SourceItemsMissing`; `WithCalls` validation order is unchanged.

Evidence (quick profile, `--features vm-reference`): `loop_scope_tests` 6/6 —
the flipped positive compiles and the interpreter returns `5/1/4/9` for
zero/one/multi-iteration `(i, limit)` pairs plus the two-carrier
`acc = acc * 2; n = n + 1` case (`acc = 8`, PHI count ≥ 2); grammar negatives
pin their observed named terminals (`SourceItemsMissing` for `<=` and `-`,
`duplicate-source-site` for `+=`, `source-target-empty` for `break`); the
hidden-call pin proves `Scan.bump` inside the loop never mints CallFree.
`normal_callable_loop_source_route` suite 29/29 including six new
route-level coverage pins. One pre-existing red remains:
`program_block_with_exit_signals_prefers_recipe_only` fails identically on
parent `3c2ffba111` — baseline debt, not this change.
`mirbuilder_qualified_route_scope_guard.sh` registers the coverage surface
and the new/edited files in its 800-line boundary list.

Production evidence (quick-profile `./target/quick/hakorune
--emit-mir-json`): the corpus module
`lang/src/hako_alloc/memory/size_class_box.hako` compiles; emitted
`SizeClassBox.bin_size/1` has 10 blocks with 4 PHI, 3 `mir_call`, 13 `binop`
— the `Body(9)` `SourceItemsMissing` freeze is gone. Re-measuring
`apps/mimalloc-lite` now stops at
`ordinary-new/local-commit/artifact-source-unavailable` — an ordinary-`new`
(`new HakoAllocHeap()`) construction-artifact terminal, a different owned
family and the next observable frontier. Gate 1 remains unsatisfied; this S0
does not claim it.

### S0 implementation receipt — `MIRBUILDER-GATE1-PARAM-RECEIVER-CALL-SOURCE-S0` (landed)

Issuer: `ordinary_new_lexical_instance_call.rs` (new) issues
`InstanceMethod` lexical dispositions into `OrdinaryNewClaimLedgerV1`,
co-sealing (a) claim-local receivers (`local x = new C()`; sole
initializer, no rebind, claim class in the approved ordinary user-box
catalog) and (b) parameter receivers proven universally: *every* package
call edge matching the callee's selector+arity must be a lexical-receiver
call whose argument at the bound ordinal is a local binding with exactly
one initializer, no rebind, an ordinary-new claim, and one agreed claim
class. Any unproven/ambiguous/rebound/non-lexical edge vetoes the
parameter — the call stays unarmed (soft decline), so outside an armed
loop the existing dynamic path survives and inside one the route names
the site via `SourceCallOutsideSelectedFamily`.

Transport: the ledger is shared by `Rc` into
`CallableSemanticLoweringState` (`with_callable_source_scope`); lowering
consults it via `lexical_instance_call_covered` /
`take_lexical_instance_call` (one-shot per call site; bindings stay
reusable across distinct sites). Consumers: `common.rs`
`declared_instance_call_effect` consults the lexical map for `Variable`
receivers only — `me`/`this` keep the strict declared-instance locator,
and lexical misses return `Ok(None)` so unarmed core/dynamic calls are
untouched. Both the value-position and statement-position `Variable`
arms in the normalizer route through it; the emit stays
`emit_canonical_instance_call_at_v1` (`SameModuleInstance`, never
runtime dispatch).

Route coverage: `CallableLoopSourceItemDispositionV1::InstanceMethod` is
a new bucket; `source_target_for_loop` collects it, `loop_cond` physical
validation unions core+instance items against source order, and
`composite_physical` accepts the new arm — one item covers exactly one
site, no static-publication obligation is inferred from lexical rows.

Evidence (quick profile): new issuer suite
`lexical_instance_call_tests` 6/6 — parameter receiver armed for both
value (`readData`) and statement (`release`) positions with
`InstanceBoxMethod`/`ParamStore`/arity pins and the one-shot take
reject, claim-local receiver armed, and four unarmed pins (call-result
argument, call-result receiver, ambiguous caller classes, rebound
parameter). Suites: `normal_callable_loop_source_route` 29/29,
`loop_scope_tests` 6/6, `callable_loop` 89/89, `ordinary_new` 66/66.

Production evidence (quick-profile `./target/quick/hakorune
--emit-mir-json apps/boxtorrent-mini`): `materialize`/`releaseFrom` no
longer freeze — the param-receiver `store.readData`/`store.release`
items arm and emit canonically (earlier `ParamManifest` probe dump:
`call_same_module_instance ParamStore.readData(%21) [recv: %20]` where
`%20` copies param `%1`). The new frontier is `ingest`'s loop at
`Argument(1)` — `chunk_data.length()` nested inside
`manifest.addChunk(cid, ...)`; `chunk_data` is a call-result local, an
argument-position core-method coverage gap owned by queue row 3, not by
this slice. Gate 1 remains unsatisfied; this S0 does not claim it.
