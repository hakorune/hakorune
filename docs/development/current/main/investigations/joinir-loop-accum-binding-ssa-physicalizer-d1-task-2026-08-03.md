---
Status: Accepted design stop — caller-zero implementation not started
Date: 2026-08-03
Decision: `JOINIR-LOOP-ACCUM-BINDING-SSA-PHYSICALIZER-D1`
Scope: replace the superseded P1-S1 operation-emission shape with a
       Binding-SSA-first canonical session for one DirectAccum fixture
Related:
  - joinir-loop-accum-verified-recipe-consumer0-p1-s1-design-stop-task-2026-08-03.md
  - joinir-loop-phi-materializer-m6b-design-2026-08-03.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# DirectAccum Binding-SSA-first physicalizer design stop

## Decision

The previous P1-S1 proposal to extend `LoopPhiMaterializerV1` into an
operation emitter is superseded. The final production shape must use the
existing function-owned SSA/CFG owners:

```text
Verified Recipe / JoinSig
  = logical control, edge, and carrier-visibility obligations only

CanonicalCfgSessionV1
  = MIR blocks, terminators, cached predecessor checks, and seal witnesses

one BindingSsaBuilderV1 per function
  = BindingRef reaching values, ReadBinding/read, WriteBinding/define,
    provisional PHI creation, seal, and finish

one caller-owned PhiTxn + MirBindingSsaAdapterV1
  = the only low-level PHI lifecycle path used by that SSA owner
```

`LoopPhiMaterializerV1` and its P1-S0 handle remain caller-zero mechanical
evidence only. They are not inputs to this physicalizer and must not be
extended into a production operation writer. This resolves the conflict
between the M6-B map/receipt observer and the Binding-SSA-first SSOT.

## Identity boundary

Portable `LoopBindingKeyV1` is not a source name and cannot resolve itself.
The canonical function owner must issue a sealed, non-Clone projection:

```rust
VerifiedLoopBindingProjectionV1 {
    owner: FunctionOwnerIdV1,
    rows: Box<[(LoopBindingKeyV1, BindingRefV1)]>,
}
```

The projection validates duplicate keys and foreign owners but owns no
reaching values, PHIs, names, or source lookup. The physicalizer consumes this
capability; it never reconstructs identity from recipe labels or paths.

## Candidate-only session

The next implementation slice is a test-only `CanonicalLoopSsaSessionV1`
holding exactly one candidate function, one `CanonicalCfgSessionV1`, one
`BindingSsaBuilderV1<PhiToken>`, and one shared `PhiTxn` through
`MirBindingSsaAdapterV1`. It must not call `LoopPhiMaterializerV1`.

DirectAccum ordering:

1. Create the P/H/B/S/A blocks through an explicit CFG/block-creation owner;
   do not call raw `MirFunction::add_block` from the physicalizer.
2. Seed the owner-issued `BindingRefV1` definitions in the preheader.
3. Emit P→H and keep H open. Header carrier `ReadBinding` calls the sole
   `BindingSsaBuilderV1::read`, which defines provisional PHIs through the
   shared `PhiTxn` before body uses.
4. Emit H→B/A through `CanonicalCfgSessionV1`; body/step `ReadBinding` calls
   `read` and aliases the existing reaching value. Constants and arithmetic
   use canonical MIR instruction emission. `WriteBinding` calls
   `BindingSsaBuilderV1::define` after its result exists; no ephemeral binding
   cursor or name-keyed map is permitted.
5. Emit B→S→H and the exit edge through named CFG/exit owners. Seal each
   block with `VerifiedPredecessorsV1`; sealing H lets Binding SSA patch the
   incomplete carrier PHIs.
6. Call `BindingSsaBuilderV1::finish`, verify the CFG/type/result receipt,
   then commit the one shared `PhiTxn` or abort the unpublished candidate.

The current `CanonicalCfgSessionV1` lacks explicit block-creation and Return
helpers. P1-S1-D1 must either add thin candidate-only facade seams for those
operations or borrow the existing named owners; raw `add_block`/`set_terminator`
calls in the physicalizer are a hard stop.

## Required products

1. `VerifiedLoopBindingProjectionV1`: sealed owner-checked identity capability;
   no names or reaching state.
2. `CanonicalLoopSsaSessionV1`: one CFG session, one Binding SSA owner, one
   shared PHI transaction; test-only and non-Clone.
3. `LoopSsaEmissionReceiptV1`: non-Clone structural receipt for reads,
   definitions, sealed predecessor witnesses, PHI inputs, and final result.
4. DirectAccum alpha digest comparing CFG roles, PHI inputs, operation kinds,
   operands, and final binding values without raw IDs.

## Acceptance gates

- production `route_loop` and legacy scheduler remain unchanged and caller-zero
  for the new session;
- P/H/B/S/A edges and every seal pass through named CFG/exit owners;
- exactly one `BindingSsaBuilderV1` and one `PhiTxn` exist per candidate
  function;
- ReadBinding emits no new value beyond the SSA owner’s legitimate
  provisional PHI and returns only the reaching `ValueId` from `read`;
- WriteBinding uses `define(binding, block, value)` and never a local cursor;
- no `LoopPhiMaterializer*` call, direct PHI API, raw predecessor vector,
  source-name lookup, `Option`, Retry, CorePlan, PlanLowerer, or fallback;
- `ssa.finish` rejects open/incomplete blocks; injected post-effect failure
  drops the candidate and a fresh session remains reusable;
- all touched Rust files remain below 800 lines and existing M6-B/P1-S0
  observer tests stay green.

## Explicit non-claims

This row does not activate production recipe lowering, change route selection,
remove Generic debt, cover nested predicate/all-route families, retire the
legacy scheduler, or claim `.hako` PHI/SSA ownership. M6-B remains a
caller-zero witness until a later retirement decision.
