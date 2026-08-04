---
Status: Design stop — selected If shape envelope before the next adoption
Date: 2026-08-04
Decision: keep explicit and implicit selected shapes in one physical owner, but
  close their correspondence/receipt contract before admitting another shape
Outcome: no implementation is authorized until the D0/D1/D2 evidence below is
  recorded; global PHI/SSA retirement remains a later adoption program
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
  function, ID cursors, catalog, Binding SSA, and `PhiTxn` fingerprint
  unchanged;
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
