---
Status: Closed design boundary; P4-S1 paused on M10a D2-S5
Date: 2026-08-03
Decision: accepted boundary — `JOINIR-LOOP-ACCUM-MIR-PHYSICAL-SNAPSHOT-DESIGN0-M5-P4`
Scope: define the test-only physical-parity seam before the first production
       Loop physicalizer exists
Related:
  - joinir-loop-accum-semantic-parity-readbinding-m5-task-2026-08-03.md
  - joinir-loop-physical-edge-path-p1b-task-2026-08-03.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# DirectAccum physical-parity snapshot: design stop

## Why this is a stop

The M5 ReadBinding task has closed the portable/legacy operation, dataflow, and
final-carrier digest. A full MIR parity test is not yet implementable without
changing the ownership boundary:

```text
portable Recipe -> JoinSig -> LoopPhiMaterializerV1
```

currently materializes only verified PHI sites through the existing
`phi_lifecycle` and Binding SSA owners. It does not emit the loop's CFG,
instructions, or result value. The legacy `RecipeComposer` -> `CorePlan` ->
`PlanLowerer` remains the only complete physical producer.

Creating a second CFG/instruction lowerer in a test to make the parity test
green would be a second semantic authority. It would also make a future
physicalizer pass its own copy of the legacy implementation, which defeats the
M6/M10 design. Therefore this row defines the seam and stops; it does not add a
synthetic physical producer.

The prerequisite structural P1b witness is closed: explicit physical paths,
sealed predecessor rows, Standard5 `Body -> Step -> Header`, and the
alpha-normalized JoinSig/map/PHI receipt digest are deterministic and
caller-zero. This card therefore owns the next design question; it must not
reopen P1 topology or extend `LoopPhiMaterializerV1` into an operation/CFG
lowerer.

## Decision

Define a test-only `MirPhysicalAlphaSnapshotV1` contract with two producers:

1. **Legacy oracle producer** — observes the existing candidate MIR after the
   legacy PlanLowerer succeeds.
2. **Future physicalizer producer** — observes the same candidate after the
   single production physicalizer exists.

The snapshot is a comparison format, not an IR and not a new lowering API. It
must erase allocation identity while retaining semantic structure:

- CFG roles and explicit edge-path roles (`preheader`, `header`, `body`,
  `step`, `after`, and nested path prefixes);
- terminator shape and successor role;
- instruction/dataflow rows with canonical binding/value provenance;
- PHI binding/class rows and predecessor port roles, sourced from existing
  `PhiTxn`/`phi_lifecycle` receipts and Binding SSA evidence;
- final binding/result/unit semantics and MIR type classes.

Raw `ValueId`, `BasicBlockId`, allocation order, AST nodes, `CorePlan`, and
route names are not snapshot authority. Canonicalization must use the verified
JoinSig/path witness and binding roles, never guess from block numbers or
route-local indices.

## Required API shape (test-only until M10a)

The first implementation may live in a separate `cfg(test)` child module (the
existing 793-line materializer test parent must not grow). The minimal shape is:

```rust
struct MirPhysicalAlphaSnapshotV1 {
    cfg: Box<[CfgRoleRowV1]>,
    instructions: Box<[InstructionRowV1]>,
    phis: Box<[PhiRoleRowV1]>,
    results: Box<[ResultRowV1]>,
}
```

The row types are comparison DTOs only. They must not expose a constructor that
accepts AST/CorePlan or provide emission/mutation methods. A legacy observer
may read those types internally, but the future consumer receives only the
candidate MIR plus the already verified JoinSig/physical map and existing PHI
receipt. No snapshot helper may allocate a second PHI/SSA writer.

## Ordered work

1. **P4-D0 (closed):** document row membership, canonical role grammar, and
   the exact legacy observer boundary. No code producer was added.
2. **P4-S0 (closed):** follow
   `joinir-loop-accum-mir-physical-snapshot0-m5-p4-s0-task-2026-08-03.md` to
   implement the legacy observer in a separate test-only child and compare it
   against the already-green P1b structural/path digest. This is evidence
   collection, not a portable physicalizer.
3. **M6 completion:** finish the logical JoinSig evidence and canonical
   CFG/Binding-SSA physical owner boundary. The existing PHI lifecycle and
   function-owned Binding SSA remain the sole production writers; the M6-B
   `LoopPhiMaterializerV1` remains a caller-zero observer.
4. **M10a pilot:** implement one real Accum physicalizer that emits CFG,
   operations, and result values through canonical CFG + Binding SSA + shared
   `PhiTxn`. Only then add
   the second snapshot producer and full MIR/PHI/type/result parity.
5. **M10a gates:** add late-failure candidate discard and fresh-session reuse
   to the same physical parity child. A failure must return terminal `Freeze`,
   never retry or invoke another route.

## Explicit non-claims

- M5 does not yet prove full physical MIR CFG/instruction/result parity.
- `LoopPhiMaterializerV1` is not a complete Loop physicalizer.
- No production `route_loop` caller, Retry change, Generic disposition, or
  JoinIR fallback deletion is authorized by this row.
- PHI/SSA ownership is already SSOT'd; this row only defines how future
  evidence observes it. It must not introduce a competing PHI authority.

## Stop conditions

Stop and return to design if any implementation requires:

- rebuilding AST or `CorePlan` in the portable consumer;
- a test-only CFG/instruction lowerer that duplicates PlanLowerer semantics;
- direct PHI insertion outside `phi_lifecycle`/`PhiTxn`;
- route selection, Retry, raw suffix, or post-effect `Ok(None)`;
- claiming full parity before the real physicalizer producer exists.

## Acceptance for this design row

- the M5 logical operation/dataflow/final-carrier digest remains green;
- this contract identifies both observer boundaries and all erased identity;
- the next implementation row is explicitly blocked on the first shared
  physicalizer, not on another route-by-route oracle;
- all source changes remain under the 800-line limit and production callers
  remain unchanged.

## Design-stop execution brief and reference closeout

Source authority is the verified JoinSig, P1b physical-path witness, existing
candidate MIR, `PhiTxn`, and function-owned Binding SSA. The snapshot observer
is non-authoritative comparison data; it must not own AST/source policy,
CorePlan lowering, CFG construction, operation emission, route selection,
Retry, publication, or a second PHI/SSA lifecycle. Any missing role/path,
unknown physical meaning, candidate mutation outside the existing producer, or
need for a duplicate lowerer is a fail-fast return to this design stop.

The recommended next implementation is only after a single shared M10a
physicalizer exists: add its second snapshot producer, compare the legacy and
new candidates through `MirPhysicalAlphaSnapshotV1`, then prove late-failure
discard and fresh-session reuse. Do not implement a test-only substitute
physicalizer to satisfy the comparison.

After that implementation, completion requires exact evidence and synchronized
updates to this card, the P4-S0 observer card, the Loop pipeline and PHI/SSA
design SSOTs, `docs/reference/mir/phi_invariants.md`,
`docs/reference/mir/phi_policy.md`, `src/mir/builder/README.md`,
`CURRENT_STATE.toml`, and `10-Now.md`. Record the commands, test counts,
caller census, and all touched-file line counts; state explicitly that no
grammar, IR, Generic policy, Retry/fallback, or route behavior changed unless
a separate accepted M10a cutover card authorizes it. Reference-document
synchronization is part of implementation acceptance, not optional cleanup.

## Reconciliation (2026-08-04)

The M10a resolved DirectAccum pilot has now supplied the prerequisite shared
physicalizer and one canonical resolved production caller. Earlier
caller-zero/no-physicalizer wording in this design card is historical and no
longer selects the current frontier. The successor implementation task is
`JOINIR-LOOP-ACCUM-MIR-PHYSICAL-SNAPSHOT0-M5-P4-S1`:

```text
resolved DirectAccum candidate MIR
  -> immutable alpha snapshot
  -> semantic comparison with the legacy observer
```

The successor remains test-only and keeps `route_loop`, Retry/fallback,
Generic policy, PHI/SSA ownership, grammar, and IR behavior unchanged. Its
acceptance includes exact role/terminator validation, late candidate abort and
fresh-session reuse, and synchronized reference-document updates.

## Successor design stop — final carrier publication

The first actual resolved candidate exposed a production contract gap before
P4-S1 could honestly compare final carriers: the candidate's `After` block
has the Unit return but no final `i`/`sum` carrier reads. P1/D1 requires those
role-keyed reads at sealed `After` before the function-owned Binding SSA and
`PhiTxn` finish. P4-S1 must not derive them from header PHIs or fabricate
observer rows.

The separate accepted task
`JOINIR-LOOP-ACCUM-FINAL-CARRIER-PROJECTION-M10A-D2-S5` therefore owns the
caller-side fix: keep `CanonicalDirectAccumBindingPort` alive, split the
`After` seal so the port seals the verified predecessor witness, read the
verified carrier keys through `read_entry_for_key`, store a typed receipt, then
finish claims and the existing identity/PhiTxn/completion transaction. P4-S1
resumes only after D2-S5's focused success/failure/reuse gates are green.
