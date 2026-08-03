---
Status: Design stop — accepted boundary; implementation caller remains zero
Date: 2026-08-03
Decision: `JOINIR-LOOP-ACCUM-PHYSICALIZER-CANDIDATE0-M10A-D1`
Scope: define one DirectAccum physicalizer candidate without adding a PHI/SSA
       authority or switching `route_loop`
Related:
  - joinir-loop-accum-binding-ssa-physicalizer-d1-task-2026-08-03.md
  - joinir-loop-accum-mir-physical-snapshot-design0-m5-p4-task-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
---

# DirectAccum candidate physicalizer: M10a design stop

## Source authority

The accepted input is one consuming, sealed pair produced from the same
verified recipe:

```text
VerifiedLoopPhysicalInputV1
  = VerifiedLoopRecipeV1 + its VerifiedLoopJoinSigV1
VerifiedLoopBindingProjectionV1
  = canonical-owner-issued LoopBindingKeyV1 -> BindingRefV1 capability
```

The pair is consumed once and cannot be rebuilt from AST, names, paths,
`CorePlan`, `PlanLowerer`, route facts, or a second selection. The binding
projection carries identity only; reaching values and PHIs remain owned by the
canonical function session.

The outer `ModuleBuilderInvocationSessionV1` is the abort boundary. The
physicalizer receives only a mutable borrow of the unpublished candidate
Builder and has no external commit capability. Its API returns a typed
terminal `Result<LoopPhysicalSuccessV1, LoopPhysicalizeErrorV1>`; it never
returns `Option`, `Retry`, a suffix, or a next-route continuation. Unit
completion is an explicit success disposition, not an `Option<ValueId>`.

## Existing SSOT owners (no new authority)

The candidate function contains exactly one instance of each existing owner:

```text
CanonicalCfgSessionV1
  blocks, terminators, predecessor witnesses, and seals
BindingSsaBuilderV1
  BindingRef reaching definitions, reads, writes, and provisional PHIs
PhiTxn + MirBindingSsaAdapterV1
  the only low-level PHI lifecycle path
```

`LoopPhiMaterializerV1` and the old P1 handle remain caller-zero mechanical
evidence only. They must not be extended into an operation writer. The
physicalizer must not call raw `MirFunction::add_block`, raw
`BasicBlock::set_terminator`, `update_phi_instruction`, or any test-only emit
helper.

## Required named seams before S0 implementation

`CanonicalCfgSessionV1` currently lacks two neutral seams required by a real
producer. Add or approve thin named facades before emitting production-like
MIR:

1. **Block/exit owner** — create the P/H/B/S/A blocks, emit the unit/terminal
   return, and preserve terminator-truth/predecessor-cache invariants.
2. **Candidate operation owner** — emit portable `ConstI64`, `CompareI64`, and
   `BinaryI64` operations without AST. It must publish typed values through the
   canonical `ValueId`/instruction path and return typed errors before any
   caller-visible commit. Reuse an existing neutral owner if one already
   satisfies this contract; otherwise add one small facade, not a new lowerer.

These facades are implementation seams, not new semantic policy. If they
cannot be added without raw MIR mutation, a second PHI/SSA writer, or a copy of
`PlanLowerer`, stop and reopen this design.

Two builder-free capabilities must also leave their current test-only homes
before the physicalizer can be a production-shaped candidate:

- `VerifiedLoopBindingProjectionV1` must be issued by the canonical function
  owner in a neutral module. It validates owner/duplicate/foreign bindings and
  is non-Clone; it does not resolve names or carry reaching values.
- a sealed physical path witness (for example
  `VerifiedLoopPhysicalRolePlanV1`) must carry the actual P/H/B/S/A endpoints
  and predecessor roles. The logical JoinSig backedge is `Body -> Header`,
  while DirectAccum's physical path is `Body -> Step -> Header`; the
  physicalizer must consume this witness and never infer `Step` from a route
  name or block number. Missing or ambiguous endpoints reject before MIR
  effects.
- a preheader/input capability must bind recipe-local `inputs=[v0,v1]` to
  already existing current-function definitions. The physicalizer must not
  invent `Const(0)` values, clone the AST, or silently rewrite the entry CFG.
  Missing/terminated current blocks, foreign owners, or unavailable induction
  and accumulator bindings reject before creating new blocks or PHIs.

## DirectAccum S0 ordering

For the one accepted DirectAccum winner:

```text
qualify/consume input
  -> open unpublished candidate
  -> create P/H/B/S/A through named CFG owner
  -> seed preheader BindingRef definitions
  -> emit P -> H; header reads demand provisional PHIs through Binding SSA
  -> emit condition/body operations through the named operation owner
  -> every WriteBinding calls BindingSsaBuilder::define
  -> emit H -> B/A and B -> S -> H; emit the exit through the named owner
  -> seal CFG blocks; SSA patches and finishes through existing owners
  -> verify semantic CFG/ops/carriers/types/result
  -> commit PhiTxn on success, otherwise abort and drop the whole candidate
```

The physicalizer does not choose a route, rediscover facts, or publish the
module. M10a is an optional DirectAccum bridge after the caller-zero recipe
producer; it does not replace the ordered M7 cohorts and does not authorize
M10b cutover.

Before `BindingSsaBuilderV1::finish()` consumes the SSA owner, the candidate
must read the final role-keyed bindings through the same SSA adapter at the
sealed after block and store them in an owned success receipt. It must not add
a persistent/name-keyed map or change `LoopPhiMaterializerV1` to expose final
values. For DirectAccum Standard5, the after block's single header predecessor
provides the final carrier alias cleanly.

Do not add another parity observer before this seam is implemented. The M5/P4
legacy snapshot and P1-S1/R3 semantic-core/legacy-auxiliary policy are enough
to define the comparison boundary; the candidate-side snapshot gets its second
producer only after the real physicalizer emits MIR.

## S0 implementation progress

The caller-zero vertical slice now has neutral pair/input/role capabilities,
named CFG block-selection/return seams, an AST-free i64 operation facade, and
one DirectAccum physicalizer using exactly one CFG session, Binding SSA owner,
and PhiTxn. The fixture seeds existing preheader values instead of inventing
initial constants and returns explicit Unit plus final carrier values.

Focused evidence is green for the capability boxes, canonical CFG seams, the
operation facade, and the DirectAccum success/preflight cases. A cfg(test)-only
failure after header PHI/operation effects now exercises `PhiTxn::abort_on_err`,
drops the unpublished candidate, proves the live Builder fingerprint is
unchanged, and succeeds on fresh candidate reuse. A companion readiness
rejection test proves a physically-emitted candidate is still not externally
published while its function remains open. Production recipe, physicalizer,
and `route_loop` callers remain zero. Shared alpha semantic parity is the next
S0 gate; no production cutover is implied by this progress.

## Acceptance gates for the S0 implementation

- malformed recipe, JoinSig, role, projection, or operation input rejects
  before the first candidate MIR effect;
- semantic physical snapshot matches the legacy observer for CFG roles,
  operations/dataflow, carrier PHI inputs, final `i`/`sum`, and Unit/Void;
- injected failure after an operation, PHI, SSA, or CFG mutation aborts the
  shared `PhiTxn`, drops the unpublished candidate, leaves the live Builder
  fingerprint unchanged, and allows fresh reuse;
- production `route_loop`, scheduler, Retry, Generic, CorePlan, PlanLowerer,
  and `LoopPhiMaterializerV1` callers remain unchanged/zero;
- the physicalizer owns no commit authority, and every touched Rust file
  remains below 800 lines;
- existing PHI lifecycle, Binding SSA, logical recipe, and observer gates stay
  green.

## Explicit non-claims

This card does not claim full MIR parity, all-route recipe coverage, Generic
debt resolution, production wiring, scheduler removal, or selfhost authority.
It confirms that PHI/SSA is already SSOT'd and limits the next work to using
that chain once, behind the existing compile-candidate abort boundary.
