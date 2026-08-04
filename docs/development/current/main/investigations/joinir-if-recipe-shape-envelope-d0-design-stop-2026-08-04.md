---
Status: S0 closed — two-call arms design stop is next
Date: 2026-08-04
Decision: preserve the sole canonical If physical owner, first recover source
  headroom without behavior change, then admit exactly one direct static i64
  call in each explicit branch through the same owner
Outcome: the private seam is closed as a behavior-neutral BoxShape refactor;
  the two-call semantic row follows only after its focused D0/D1/D2 evidence.
  Global PHI/SSA retirement and further If shapes remain separate programs
Related:
  - joinir-if-recipe-d0-d-physical-adoption-design-2026-08-04.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/recipe-first-entry-contract-ssot.md
---

# Selected If Recipe shape envelope — D0 design stop

## Why this is the next boundary

The bounded implicit-fallthrough row is landed. The selected resolved-trivial
If path now has one producer/consumer chain:

```text
same-pass facts
  -> IfRecipeArtifactV1
  -> IfJoinSigV1
  -> CanonicalIfPhysicalDemandV1
  -> CanonicalIfRecipeTopologyV1
  -> CanonicalSsaFunctionSessionV2
  -> canonical CFG / Binding SSA / PhiTxn
```

Explicit `else` and implicit fallthrough are not interchangeable boolean
flags. Their false predecessor/value pairs differ, so they stay as two typed
topology variants inside the same selected physical owner. The next clean step
is to prove that every source claim, logical edge, and physical receipt agrees
for both variants. This is a BoxShape/design task, not a new acceptance shape.

## Current authority map

### Source and logical authority

* same-pass resolved facts own the source site, condition, binding, branch
  writes, continuation read, and (for implicit fallthrough) the entry baseline;
* `IfRecipeArtifactV1` owns the portable semantic shape and value classes;
* `IfJoinSigV1` owns the ordered ports/edges and the disposition-specific
  false edge;
* the mapper/verifier pair is the only place that turns sealed facts into a
  verified artifact/signature pair.

### Physical authority

`CanonicalSsaFunctionSessionV2` remains the sole physical owner for this
selected family. Its `CanonicalCfgSessionV1`, `BindingSsaBuilderV1`, and
`PhiTxn` are the only selected-path CFG/SSA/PHI state. The receipt is evidence
of the physical result; it is not a second writer.

### Explicitly non-authoritative paths

The following remain live elsewhere and are not silently claimed as retired:

* `resolved_lowering/located_if.rs` and `IfCfgSessionV1` for the older located
  family;
* `control_flow/plan/lowerer/plan_lowering.rs` → `features/if_join.rs` for the
  CorePlan/JoinIR family;
* raw `if_form.rs`/statement descent and JSON-v0/import bridges;
* route-local Loop/If PHI materializers and legacy repair utilities.

These paths must be listed in the census, not folded into the selected receipt
claim. Repository-wide PHI/SSA sole-writer retirement is a later design row.

## D0 — correspondence and receipt contract

Design and test the following without changing production behavior:

1. Cross-check the complete ordered correspondence for both variants:
   condition, entry/baseline, then, optional else, and continuation.
2. Require the explicit variant to have an else block/exit and an
   `ElseTransfer` edge; require the implicit variant to have no else block and
   an `ImplicitBaseline` edge carrying the entry value.
3. Require receipt predecessor/value pairs to match the JoinSig exactly:
   explicit `[then_exit, else_exit]`; implicit `[header, then_exit]`.
4. Reject mutated disposition, source claim, value class, binding, block, or
   predecessor data before any Builder effect. There is no retry or route
   reselection after a selected demand exists.
5. Keep the typed explicit/implicit values envelope. Do not reintroduce an
   `Option`-shaped physical topology or a shared nullable receipt field.

The D0 product is a documented contract plus fail-fast tests. It does not add a
new recipe profile, lowerer branch, PHI writer, or legacy caller.

## D1 — caller and shape census

Record exact production counts and paths for the selected family:

* selected physicalizer definition and sole production caller;
* selected lowerer helper and its canonical-session owner;
* artifact/JoinSig/receipt constructors and all test-only constructors;
* old `IfCfgSessionV1`, CorePlan `apply_if_joins`, raw IfForm, and JSON-v0
  callers as separate non-selected columns.

The census must distinguish production callers from tests and read-only
parity/repair utilities. A count alone is not retirement evidence.

## D2 — parity and candidate-abort proof

Use one explicit and one implicit fixture with the same outer binding and
continuation read. The proof must cover:

* selected profile, artifact, JoinSig, topology, receipt, MIR PHI predecessor
  sets, and value classes;
* interpreter/result and diagnostic parity with the existing behavior;
* a late selected-physicalization failure that leaves the live module,
  function, ID cursors, catalog, and Binding SSA owner fingerprint unchanged;
  the stack-local `PhiTxn` is covered by its typed commit/drop lifecycle
  witness, not a live Builder fingerprint;
* a fresh compile on the same compiler succeeding after that failure.

The accepted evidence is shape-scoped. It must not be used to claim that raw,
CorePlan/JoinIR, nested/effect/return/call/record/match, or Loop PHI paths have
been migrated.

## D3 — handoff to the next shape

After D0–D2 are green, open exactly one new design stop for the next If shape.
Nested/effect/return/call/record/match and short-circuit control are not
automatic extensions of this envelope. Each needs its own source claims,
logical ports, physical receipt, and candidate-isolation proof. If a shape
requires widening `CanonicalTrivialSsaLowererV1` beyond the 800-line boundary,
extract a behavior-neutral seam in a separate Refactor Series before adding
acceptance.

## Acceptance and stop conditions

Done means the D0 contract, D1 caller census, and D2 parity/abort evidence are
written in the active card, focused tests are green, the existing shared guards
remain green, and every touched Rust/test file is below 800 lines.

Stop and open a new design row if any of the following appears: a second
selected production caller, a route/retry/fallback edge, source re-decision in
the physicalizer, an untyped nullable topology, a receipt that cannot be
cross-checked against JoinSig, a legacy writer needed to complete the selected
shape, or a broader PHI/SSA retirement claim.

## Accepted execution slice — `JOINIR-IF-RECIPE-SHAPE-ENVELOPE-D0`

The design boundary is accepted for one behavior-neutral contract slice.
Implement only the D0 correspondence/receipt checks and their tests:

* explicit and implicit dispositions must cross-check their complete ordered
  source claims, JoinSig ports/roles, topology variant, and receipt values;
* mutated receipt/disposition/source evidence must fail before Builder effects;
* the typed explicit/implicit values envelope remains the only physical shape
  input; no route, retry, fallback, or new PHI writer may be added.

Do not implement D1 caller retirement or D2 candidate-fingerprint expansion in
the same change. Those are the next bounded checks after D0 is green. Any
failure that needs a lowerer topology change, another source family, or a
global PHI/SSA claim reopens a design stop.

## D0 completion evidence — 2026-08-04

The D0 slice is complete and remains behavior-neutral:

* `verify_physical_receipt` now rejects every duplicate among the complete
  `{header, then, else, merge}` block set, not only adjacent duplicates;
* test-only correspondence construction exercises the selected receipt
  contract without exposing a production constructor;
* explicit/implicit `CanonicalIfPhysicalValuesV1` cross-shape inputs are
  rejected before receipt construction;
* a non-adjacent physical block collision is rejected with the stable
  `physical_blocks_overlap` contract error.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_physicalizer -- --test-threads=1  # 2 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_contract -- --test-threads=1       # 10 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_value_profile -- --test-threads=1   # 37 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_selected -- --test-threads=1       # 3 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_lowering -- --test-threads=1        # 130 passed
RUSTFLAGS='-Awarnings' cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

No new recipe profile, lowerer branch, route/retry edge, PHI writer, or
legacy caller was added. D1 caller census is the next bounded row.

## D1 caller census — 2026-08-04

The selected explicit/implicit If shape has one production chain. The counts
below are source counts, not retirement claims:

| Product | Production count | Exact path | Role |
| --- | ---: | --- | --- |
| `produce_trivial_if_physical_input_v1` | 1 | `resolved_lowering/mod.rs:437` | same-pass artifact/physical-input producer |
| `admit_trivial_if_recipe_v1` | 1 | `resolved_lowering/mod.rs:446` | one-shot source-site admission |
| `physicalize_if_recipe_v1` | 1 caller | `trivial_ssa/lowerer.rs:461` | selected demand consumer |
| `lower_if_recipe_selected` | 1 caller | `trivial_ssa/if_recipe_physicalizer.rs:356` | selected topology bridge |
| `selected_receipt` | 1 production caller | `trivial_ssa/lowerer.rs:667` | receipt constructor after canonical lowering |

The physicalizer owns both receipt variants. Production constructs the typed
topology in `if_recipe_physicalizer.rs:347-355`, emits typed physical values
in `trivial_ssa/lowerer.rs:652-664`, and calls the receipt constructor once.
The only other receipt/value constructor references are test-only fixtures:

* `if_recipe_adapter.rs:556-574` builds a demand fixture;
* `if_recipe_physicalizer.rs:577-706` uses the `#[cfg(test)]` correspondence
  helper, direct explicit receipt fixture, and cross-shape envelope tests.

The selected physicalizer therefore has one production caller and no
production route selector, retry edge, or second receipt writer in this
shape. The following remain live but are explicitly outside the selected
authority:

| Non-selected family | Live authority/path | Census disposition |
| --- | --- | --- |
| located legacy If | `resolved_lowering/located_if.rs` → `if_materialization.rs:52` (`IfCfgSessionV1`) | legacy/session path; tests also open the session |
| CorePlan/JoinIR If | `control_flow/plan/lowerer/plan_lowering.rs:218` → `features/if_join.rs:37` | plan PHI/CFG path; not a selected Recipe consumer |
| raw IfForm | `builder/if_form.rs` → `stmts/if_statement_descent.rs` and `control_flow/mod.rs` | raw statement path; not retired |
| JSON-v0/import bridge | `global_call_route_plan/program_json_emit_body.rs:309` and `compiler/normal_default_pipeline.rs:272` | bridge/import path; no selected receipt claim |

This census does not delete or redirect any of those paths. It proves only
that the selected recipe chain is locally single-entry/single-consumer; it
does not claim global If or PHI/SSA caller-zero.

## D2 opening boundary — candidate-abort proof

D2 is now the active design row. Its source authority is the existing
unpublished candidate/session lifecycle plus the selected chain recorded
above. The selected physicalizer may prove that a late failure is discarded
with the candidate; it may not invent a second Loop/If transaction or use the
live Builder as an undo target.

The smallest permitted D2 proof is one explicit and one implicit fixture with
the same outer binding and continuation read:

1. capture the live candidate/module/function/ID-cursor/Binding-SSA-owner
   fingerprint before selected lowering, and record the local `PhiTxn`
   lifecycle as a drop/commit witness rather than pretending it is a live
   Builder field;
2. inject one late selected-physicalization failure after the first physical
   mutation inside the unpublished candidate;
3. discard the candidate and prove the live compiler state is unchanged;
4. compile a fresh valid fixture on the same compiler and prove it succeeds.

D2 does not authorize a new rollback journal, detached per-If Builder,
symbolic MIR, route retry, legacy caller deletion, global PHI/SSA adoption,
or parity claims for raw/CorePlan/JoinIR/Loop/other If shapes. If the existing
candidate lifecycle cannot expose this fingerprint without widening an
owner or crossing the 800-line limit, stop and reopen the design before code.

## Accepted D2 execution slice — candidate abort proof

The existing candidate lifecycle is sufficient, so D2 is accepted for one
test-only proof slice. Reuse the test-only
`MirBuilder::loop_candidate_test_fingerprint` and the existing typed draft
seal-failure seam. Add one implicit-fallthrough fixture beside the existing
explicit-else fixture; do not add a production fault toggle or a second
transaction owner.

Each fixture must prove, in order:

* selected `TrivialBindingSsa` preflight and the same outer binding/continuation
  read;
* failure after selected If lowering has already emitted the candidate's
  physical CFG/PHI work;
* dropping the unpublished session leaves the live Builder fingerprint,
  current module, current function, and entry block unchanged;
* a fresh compile on the same `MirCompiler` succeeds afterward.

This slice is test-only and does not claim that every physicalizer failure is
covered, that stack-local `PhiTxn`/canonical session state has a direct live
fingerprint, that candidate internals need a public snapshot API, or that
legacy If/CorePlan/JoinIR/Loop/PHI writers are retired. Candidate isolation is
whole-compile replacement semantics: abort proves the live Builder was never
mutated, while success proves a fresh candidate can be committed.

## D2 completion evidence — 2026-08-04

D2 is green for the accepted paired proof:

* the existing explicit-else late draft-seal abort test remains green;
* `if_recipe_candidate_abort_d2_tests.rs` adds the implicit-fallthrough twin
  with the same outer binding and continuation read;
* both tests poison the candidate only after selected lowering has emitted
  physical CFG/PHI work, drop the unpublished whole-compile candidate, compare
  the live Builder fingerprint/current module/current function/entry block,
  and compile a fresh request on the same compiler;
* the separate implicit receipt test still fixes the `[header, then_exit]`
  predecessor contract.

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_candidate_abort_d2_tests -- --test-threads=1  # 1 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_candidate_discards_after_late_draft_seal_failure_and_reuses_compiler -- --test-threads=1  # 1 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_selected_implicit_fallthrough_uses_header_baseline_phi_input -- --test-threads=1  # 1 passed
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_lowering -- --test-threads=1  # 130 passed
RUSTFLAGS='-Awarnings' cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

The evidence is intentionally limited to the whole-compile candidate boundary:
it does not fingerprint stack-local `PhiTxn`, prove every failure stage, or
retire any legacy writer.

## S0 completion evidence — 2026-08-04

`JOINIR-IF-CANONICAL-MATERIALIZATION-SEAM0-S0` is closed. The existing If-only
topology/outcome vocabulary and `lower_if*` materialization methods now live in
the private `trivial_ssa/lowerer/if_materialization.rs` child. The parent
lowerer is 541 lines and the child is 275 lines; selected/legacy routing,
physicalizer ownership, candidate isolation, diagnostics, and CFG/Binding-SSA/
`PhiTxn` behavior are unchanged. The shared logical-demand guard was updated
only to observe the new private owner path; no new semantic guard or public API
was added.

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_physicalizer -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_contract -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_value_profile -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_selected -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_lowering -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_candidate_abort_d2_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_candidate_discards_after_late_draft_seal_failure_and_reuses_compiler -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_selected_implicit_fallthrough_uses_header_baseline_phi_input -- --test-threads=1
cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

The next task is design-only `JOINIR-IF-RECIPE-TWO-CALL-ARMS0-D0`. After any
implementation row lands, update the applicable language/reference or compiler
contract documents before closeout; this S0 changed no language or public
reference surface.

## Selected next task sequence — 2026-08-04

### 1. `JOINIR-IF-CANONICAL-MATERIALIZATION-SEAM0-S0`

Change:
: Move the If-only topology/outcome types and `lower_if*` materialization
  methods from the 799-line `trivial_ssa/lowerer.rs` into one private child
  module. Old authority: none.

Contract:
: BoxShape only. Accepted shapes, diagnostics and their order, selected versus
  legacy dispatch, production callers, candidate transaction, canonical CFG,
  Binding SSA, and `PhiTxn` ownership remain unchanged. Add no public API,
  physicalizer, route, retry, fallback, or dedicated guard.

Done:
: Parent and child remain below 800 lines with comfortable headroom. Existing
  explicit/implicit If, direct-call, nested D2, candidate-abort/fresh-reuse,
  cargo-check, current-pointer, and shared replacement gates are green.

Stop:
: Public field/API widening, duplicated CFG/PHI logic, changed error ordering,
  changed caller counts, or any touched file reaching 800 returns to design.

### 2. `JOINIR-IF-RECIPE-TWO-CALL-ARMS0-D0` → `...-I0-R0`

Change:
: Admit one root explicit-else If with one shared i64 merge binding and exactly
  one direct static i64 call as each branch assignment RHS. D0 seals the exact
  ordered source/call correspondence; D1 recounts the existing single call
  capability/emitter/physicalizer owners; D2 proves parity and abort/reuse.
  I0/R0 then removes this exact shape's `NotThisShape -> legacy If` edge by
  routing it through the existing selected owner.

Contract:
: Claim order is `IfNode, Condition, ThenAssignment, ElseAssignment,
  DirectCall(then), DirectCall(else)`. Each call is a root assignment value;
  condition calls, call arguments containing calls, continuation calls,
  method/dynamic calls, nested If calls, and arbitrary call cardinality remain
  rejected. The canonical lowerer and physical receipt topology do not change.
  Selected failure is terminal `Freeze`; fallback/retry is zero.

Done:
: Both runtime branches call their exact target, one two-input merge PHI uses
  actual predecessor/value pairs, call capability rows and source order are
  exact, late draft-seal abort leaves the live Builder unchanged, fresh reuse
  succeeds, and the exact legacy fallback edge is zero. Every touched source
  and test file remains below 800 lines.

Stop:
: Any needed edit to the canonical lowerer, missing call identity in the fixed
  one-per-arm mapping, a second call/emission/PHI owner, name-based lookup,
  retry, or broader effect/return/record/match/short-circuit vocabulary stops
  the row. Extract call-claim correspondence into a small private module if
  the 736-line mapper/adapter would otherwise approach 800.

After the exact I0/R0 and bounded reference audit, do not automatically add a
third If shape. Return to the existing Loop M4/M6 task-order SSOT and select
its exact live critical-path row. Generic overlap remains an honest
`UnresolvedStop`, so M10 production cutover is not authorized by this task.
