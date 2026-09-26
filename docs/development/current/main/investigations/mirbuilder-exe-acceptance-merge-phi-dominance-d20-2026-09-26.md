# MIRBUILDER-EXE-ACCEPTANCE-MERGE-PHI-DOMINANCE-D20
## Census: Loop-Carried Phi Spine Emitted But Reads Bypass Phi Dst — Binding Authority Desync (D20)

Date: 2026-09-26
Status: decision recorded — next slice S8
Family: callable / gate1 / Gate 1 unified lane / SSA dominance /
  EXE acceptance
Row reference: workstream row H (Gate 1 unified selfhost lane)
Blocking observation: after S6+S7 document publication landed,
`--emit-mir-json` on `apps/json-stream-aggregator/main.hako` now
fails strict verification with 13 SSA dominance violations — every
one inside `lang/src/shared/common/string_helpers.hako` functions:

```text
StringHelpers.index_of/3      %32,%33  def bb75  used bb71   (def AFTER use — backedge)
StringHelpers.int_to_str/1    %49      def bb88  used bb90
StringHelpers.json_quote/1    %29      def bb148 used bb150
StringHelpers.last_index_of/2 %62,%63  def bb184 used bb180  (def AFTER use — backedge)
StringHelpers.read_digits/2   %16      def bb200 used bb193  MergeUsesPredecessorValue
StringHelpers.skip_ws/2       %24      def bb211 used bb208,bb210 MergeUsesPredecessorValue
StringHelpers.to_i64/1        %163     def bb285 used bb276  MergeUsesPredecessorValue
```

## Boundary covered by this census

起点: LoopCond-family loop lowering emitting loop-carried bindings
(header/join merge convention); 終点: strict verifier dominance +
merge-Phi checks (`dom::check_dominance_with_policy`,
`cfg::check_merge_uses_with_policy`).
includes: which lane lowered each violating function's `loop(cond)`,
the merge convention it emitted (Phi vs edge-copy/predecessor value),
and which authority owns the merge convention contract.
excludes: call-edge domain policy (landed S7), artifact/document
completion (landed S6), VM-lane behavior, object/instance admission.

## Measured shape

- All 7 violating functions contain `loop(cond)` (LoopCond family)
  over local string-scan state (`i`, `j`, `pos`, `acc`, `out`).
- Two violation kinds, both PHI-shaped:
  - `DominatorViolation` with `def_block > use_block` — a value
    produced on the loop backedge is read at an earlier join/header
    block: the classic missing loop-carried Phi signature.
  - `MergeUsesPredecessorValue` — a merge block reads a value defined
    in exactly one predecessor without a Phi.
- Loop-carried bindings in these functions (`i`, `pos`, `acc`, `out`,
  `last`) are precisely the values named in the violations.

## The authority question — resolved

Is the non-Phi merge an **accepted MIR dialect** the document must
serialize as-is, or a **missing-Phi lowering defect**?

**Answer: a lowering defect — specifically a binding-authority
desync, not a missing-phi-emission defect and not an accepted
dialect.** The measured facts below resolve every census question.

## Measured shape (instrumented census, all diagnostics reverted)

### Lane census — all loops funnel into `lower_loop_generalized`

Two entry ports reach the same generalized lowerer
(`CorePlan::Loop` → `PlanLowerer` → `lower_loop_generalized`,
`lowerer/loop_lowering.rs`):

- **Lane-A (callable child port)**: `RawInvocationChildPortV1::lower_loop`
  → `issue_once` → `LoopCondReady`/`LoopTrueReady` →
  `lower_loop_cond_break_continue_source` /
  `lower_loop_true_break_continue_source` → `PlanLowerer::
  lower_with_source_publication`.
  Dispatched: `int_to_str` ✗, `json_quote` ✗, `trim` ✓,
  `split_lines` ✓, `ingest/1` ✓.
- **Lane-B (legacy child port)**: `RawLegacyChildLoweringPortV1::
  lower_loop` → `lower_loop_or_freeze_v1` → `try_cf_loop_joinir`
  (structure-only routing, default ON) → JoinIR recipe → `CoreLoopPlan`.
  Never reached `issue_once`: `to_i64` ✗, `skip_ws` ✗,
  `last_index_of` ✗, `index_of` ✗, `read_digits` ✗, `find/3`
  (semantically broken, verify-clean).

### Phi spine IS emitted — instrumented counts

- `loop_plan.phis` non-empty for **every** function:
  `to_i64`=6..24, `int_to_str`=6, `json_quote`=9, `find/3`=3,
  `trim`=9..18, `skip_ws`=3, `read_digits`=6, `index_of`=6,
  `last_index_of`=6, `ingest`=6, `starts_with`=3,
  `is_numeric_str`=3, `intField`=3.
- `LoopCondBreakContinuePhiMaterializer` allocates header/step/
  after phis; `insert_provisional_phis` + `patch_all_phis` emit real
  `MirInstruction::Phi` (emitted counts >0 at loop-lower end) and
  `validate_no_unpatched_phis_after_patch` passes — every phi input
  is patched.
- Functions enter the module with phis intact
  (`add_cataloged_callable`: `to_i64`=24, `int_to_str`=6,
  `find/3`=3).

### The defect — reads never bind to phi dsts

- `finalize_loop_variables` publishes `final_values` into
  `variable_ctx.variable_map` (`out → %38`, `v → %41` for
  `int_to_str`) and `pre_loop_map` restore runs first — the
  name→ValueId publication is correct.
- **But carrier reads resolve through a second authority** — the
  binding-id SSA store (`binding_ctx` name→BindingId → per-binding
  value), which the carrier publication never updates. Observed:
  the header condition reads the pre-loop `%1` (init), body reads
  replay the initializer, and `int_to_str`'s exit `Return %49` reads
  the body's last-writer `ch + out` instead of the after-phi `%38`.
- Consequence: the emitted phi subgraph (header↔step mutual-use) is
  structurally valid but semantically dead — no read references the
  phi dsts. `CarriersOnly` edge args stay empty throughout.
- `JsonLine.find/3` is the verify-clean instance: its reads bind to
  entry-defined values (dominance-legal), so the loop replays the
  initial `start`/`end` forever — `i = i + 1` is consumed as carrier
  metadata but never transported. Silent wrong-code, infinite-loop
  semantics.

### The stripper — why phis are absent at verify time

- `finalize_module_with_canonical_root_v1`
  (`builder/module_lifecycle.rs:530`) runs
  `ssa::phi_input_materializer::materialize_all_phi_inputs` on every
  function → `prune_unused_phi_instructions` removes phis whose dst
  has zero uses — the after-phis (`%38`, `%41`) die first because no
  read binds to them. `NYASH_MIR_DISABLE_OPT=1` leaves exactly the
  mutually-used header/step phis and still fails verification with
  the same dominance/merge residuals.
- With the optimizer enabled, DCE/jump-merge additionally collapses
  the surviving phis — the optimizer is an unmasking amplifier, not
  the root cause.

### Answers to the original census questions

1. Route arms: Lane-A armed LoopCond for `int_to_str`/`json_quote`;
   Lane-B JoinIR recipe for the other five — both converge on
   `lower_loop_generalized` and share the defect.
2. The emitted merge is neither declared edge-copy nor wired phi:
   phis exist but carrier reads bypass their dsts.
3. Phi/emission owner: `LoopCondBreakContinuePhiMaterializer` +
   `loop_lowering` phi lifecycle (intact). The open authority is the
   **read/binding publication contract** — `variable_map`
   publication vs `binding_ctx` BindingId-SSA resolution.
4. Prior green: `main.hako`-local loops pass because their reads
   happened to bind dominance-legal values (see `find/3` — passing
   is not proof of correctness); this is the first strict traversal
   of `StringHelpers` loop carriers end-to-end.

## Six-line brief

```text
Decision: lower-layer binding desync — carrier phis are emitted
  but loop header/body/exit reads resolve through the binding-id
  SSA store that the carrier publication path never updates; fix
  belongs at the read/binding authority boundary.
Source authority + canonical issuer: the phi materializer +
  `finalize_loop_variables`/`publish_emission_cache` (variable_map)
  vs `binding_ctx`/BindingId-SSA read resolution — one owner must
  publish carrier phis where reads actually resolve.
Non-authority: strict verifier (correct), NYASH_MIR_NO_PHI /
  edge-copy dialect (not the emitted convention), optimizer/DCE
  (unmasking only), .hako source, legacy LoopBuilder (removed).
Fail-fast boundary: a read resolving a loop carrier through the
  non-phi binding store while an emitted phi owns that carrier.
Smallest next slice (S8): pin the intended read authority —
  census which resolver each carrier read uses (variable_map vs
  binding_ctx) in one minimal loop-carrier function, then wire
  carrier publication (header phi for body reads, after phi for
  exit reads, backedge transport for the step edge) into that
  resolver for both lanes.
Non-claims: no verifier weakening, no LoopBuilder revival, no new
  dialect declaration, no overall MirBuilder completion claim.
```

## Smallest next slice — S8 (bounded)

1. One minimal source probe (single-carrier `loop(cond)` +
   post-loop read) reproducing the desync as a focused negative
   test — assert header/after phi dsts are the values reads resolve.
2. Census the actual read resolver for the failing reads
   (`build_variable_access`/variable_map vs `binding_ctx`/
   `MirBindingSsaAdapterV1::read_binding`) — one authority chosen.
3. Wire carrier publication into that authority for Lane-A and
   Lane-B alike — both end in `lower_loop_generalized`.
4. Positive pins: header phi inputs (preheader init + backedge
   step), after-merge phi feeding exit reads, `find/3`-shape carrier
   actually transported (no more dead update statement).
5. Negative pins: merge block reading a bare predecessor value;
   empty `CarriersOnly` backedge when a carrier exists.
6. Re-run `NYASH_BIN=target/debug/hakorune` EXE smoke — expect the
   SSA family to close or surface the next residual.

## Non-claims

- Not a verifier defect: strict dominance/merge checks implement
  the PHI-ON contract correctly and caught real wrong-code.
- The fix is not "insert phis": phis are already emitted; the
  missing authority is read-side carrier binding.
- No weakening of strict verification, no no-phi env dialect as
  the answer, no fallback route.
- VM `ingest/1` ledger-less spine; Gates 2–4; overall completion.
