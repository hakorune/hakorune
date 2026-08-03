# JOINIR Direct Accum Structural Projection D1

Status: Green; design closed, caller-zero implementation next.

Task: `JOINIR-LOOP-DIRECT-ACCUM-STRUCTURAL-PROJECTION0-D1`

## Purpose

Define the owner and exact boundary for the first real Direct Accum
structural-facts producer. The preceding selected-demand S0 proves only that a
policy winner, an AST-free identity witness, and an exact resolved source can
be consumed linearly. It does not yet prove that the facts witness describes
the loop's condition and update operations, nor that all three capabilities
belong to one execution frame.

This card is a design stop. It must close the facts/source/frame identity
contract before any production `route_loop` or physicalizer caller is added.

## Existing evidence and authority split

The existing `AccumConstLoopFacts` is a useful parity oracle, but it is not a
portable facts authority. It currently carries AST-bearing fields
(`condition`, `acc_update`, and `loop_increment`), variable names, and legacy
source topology. The neutral selected-demand module must not import it or
reconstruct semantics from its names.

The existing `LoopSourceProjectionV1` / `LoopSourceBodySiteV1` is an
analysis-only, AST-free view, but it is builder-local and currently identifies
body statements more precisely than it identifies the condition and operand
expression sites. It is therefore an input to this design, not the final
neutral contract.

The source authority remains the exact, non-`Clone`
`VerifiedResolvedLoopSourceV1` issued by `VerifiedResolvedFunctionV1`. It owns
function origin, semantic owner kind, exact loop statement site, and resolved
binding identity. It does not own MIR `ValueId`, `BasicBlockId`, PHI rows, or
route policy.

The policy winner remains provenance only. Its current opaque cursor seal is
linear and non-`Clone`, but it has no shared execution-frame brand. S0 must not
be promoted as if the cursor alone proves that the winner, facts, and source
were issued from one frame.

## Design target

The builder-side adapter shall observe one already-selected Direct Accum shape
and issue an AST-free, typed, sealed product. Exact topology is not invented by
the adapter: it is issued by the existing `FunctionSourceViewV1` through
`LocatedStmtV1` / `LocatedExprV1` and the shared
`BodyChildRoleV1` / `ExprChildRoleV1` navigation policy. The adapter may copy
the observed constants and binding handles once, then discard the AST view.

A future product is expected to be equivalent to:

```text
VerifiedDirectAccumFactsV1 {
    identity: (FunctionOriginV1,
               SemanticOwnerSourceKindV1,
               SourceStmtSiteV1),
    induction: BindingRefV1,
    accumulator: BindingRefV1,
    condition: (BindingRefV1, Less, I64Const),
    update:    (BindingRefV1, Add, I64Const),
    step:      (BindingRefV1, Add, I64Const),
    fixed_body_order,
    private seal,
}
```

The exact public schema is accepted only behind the source-view and resolved
binding owners above. The product must contain no AST,
variable-name lookup, raw statement index manufactured by the adapter,
`CanonicalLoopFacts`, Recipe, Builder, CorePlan, PlanLowerer, PHI, Binding
SSA, Retry, scheduler, or Generic-debt machinery.

## Closed owner decisions

1. **Expression topology owner — accepted**

   `FunctionSourceViewV1` is the sole topology navigator. The Direct Accum
   adapter must reach the exact sites using only these shared roles:

   ```text
   LoopBody
   LoopCondition
   AssignmentTarget / AssignmentValue
   BinaryLeft / BinaryRight
   ```

   Body statement coordinates alone are insufficient. A stale, swapped, or
   foreign `Located*` carrier is rejected before the neutral product exists.
   Hand-written `SourcePathSegmentV1` construction is forbidden.

2. **Binding identity bridge — accepted**

   `VerifiedResolvedFunctionV1::variable_ref` and
   `VerifiedResolvedFunctionV1::assignment_target` are the source identity
   owner. The adapter never reconstructs a binding from a name. Shadowed
   bindings, missing targets, wrong assignment targets, and a site from another
   owner are typed rejects.

3. **Execution-frame brand — accepted**

   `VerifiedResolvedLoopSourceV1` issues an opaque
   `LoopExecutionFrameKeyV1` containing the private source owner identity. The
   key is passed into policy evaluation and the Direct Accum facts producer;
   `VerifiedLoopPolicyWinnerV1`, the structural facts product, and the source
   capability retain the same key. The selected-demand issuer checks key
   equality as well as facts/source identity before sealing the handoff. The
   raw route cursor remains diagnostics only. A key without the source-issued
   seal, or a post-hoc rebrand, is invalid.

4. **Projection location — accepted**

   Keep the adapter at the builder-side boundary that already observes legacy
   facts. The neutral `loop_structural_facts` module may consume only a typed,
   AST-free observed DTO and the exact resolved-source capability. It must not
   become a second canonicalizer or route selector.

## Rejected shortcuts

- Passing `AccumConstLoopFacts` directly into the neutral module.
- Re-resolving variable names or synthesizing source paths from raw indices.
- Treating a matching source identity tuple as proof of one execution frame.
- Adding a PHI/SSA materializer, MIR physical ID, or Builder transaction here.
- Turning Generic V0/V1 post-effect debt into a hidden Direct Accum fallback.
- Calling the new product from production before the negative gates below are
  green.

PHI/SSA is already governed by the existing SSOT chain:
`CanonicalCfgSessionV1` + function-owned `BindingSsaBuilderV1` + `PhiTxn`.
This card must not create or modify that authority.

## D1 close gates

The design is closed because the following owners and boundaries are explicit;
the caller-zero S0 card below fixes them with tests:

1. A positive Direct Accum fixture produces one sealed AST-free facts product
   whose exact condition/update/step sites and `BindingRefV1`s match the
   resolved source.
2. Stale or mismatched expression site, missing topology, shadowed name,
   wrong loop owner, and facts/source identity mismatch all reject before any
   Builder effect.
3. The three-way execution-frame brand is the source-issued
   `LoopExecutionFrameKeyV1`; production remains caller-zero until its
   mismatch tests are green.
4. The neutral module has zero imports of AST, `CanonicalLoopFacts`, Recipe,
   Builder, CorePlan, PlanLowerer, PHI, Binding SSA, Retry, scheduler,
   physicalizer, and Generic debt machinery.
5. The existing PHI/SSA SSOT guards remain unchanged and green; no second PHI
   writer or `LoopPhiMaterializerV1` production caller is introduced.
6. The selected-demand S0 identity and linear-consumption tests remain green,
   and all touched Rust files remain below 800 lines.

## Stop / handoff

Stop and return to design if any owner proposes AST rescans, name-based
dispatch, raw-path synthesis, a second route evaluator, a second PHI/SSA
authority, or a production caller before the frame brand is sealed.

The next implementation slice is
`JOINIR-LOOP-DIRECT-ACCUM-STRUCTURAL-PROJECTION0-S0`. It may add only the
Direct Accum observed-shape adapter, source-issued frame key, and contract
tests. It may then feed the existing selected-demand issuer; it must not
physicalize or alter legacy route selection in the same change.
