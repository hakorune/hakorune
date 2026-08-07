# Loop Recipe Operation Effect Product S0

Status: `IMPLEMENTATION-READY`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-EFFECT-PLAN-D0`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

Implement the passive, AST-free `VerifiedLoopOperationEffectProductV1` and its
focused verifier. This is a caller-zero contract cell. It prepares exact
operation/source identity for a later physicalizer but emits no operation MIR.

## Sole-owner model

```text
LoopRecipeV1
  = item / operation / value / block / loop meaning

VerifiedLoopCoreProductV1
  = Recipe + JoinSig + source BindingRef + binding-level effect rows

VerifiedLoopOperationEffectProductV1
  = moved Core + one profile-issued item/anchor evidence ledger

physicalizer
  = later consumer; never reconstructs source identity
```

The product is non-`Clone` and moves the Core exactly once. It must not copy
`LoopOperationV1`, operands, `BindingRef`, or effect rows. These are exposed
through typed views/references into the moved Core and evidence ledger.

## Minimal passive schema

```text
LoopOperationSourceEvidenceV1 {
  item: LoopItemKeyV1
  anchor: exact owner-branded expression/carrier anchor
  source_loop: exact source loop statement site
  placement_claim: { owner_loop, block }
}

VerifiedLoopOperationEffectProductV1 {
  core: VerifiedLoopCoreProductV1
  source_evidence: Box<[VerifiedLoopOperationSourceEvidenceV1]>
  private item index / typed Core relation views
}
```

`BindingRef` is optional for an evidence row because literals and pure
operations have no source binding. When a Recipe operation is
`ReadBinding`/`WriteBinding`, the verifier must connect it to the exact sealed
Core effect/binding relation where one exists. Pure literal/compare/binary
operations must not receive a fabricated binding effect.

`DerivedCarrierEntry`, structural carrier rows, and callable Tail/After reads
remain with their existing owners. Their explicit non-consumption is not a
silent drop and does not expand operation coverage.

## Implementation scope

Allowed in this row:

```text
src/mir/loop_recipe_contract/operation_effect.rs
  passive product, evidence types, verifier, typed reject surface

src/mir/loop_recipe_contract/source_bound_core.rs
  one non-authority anchor/class accessor or consuming join helper if needed

focused cfg(test) fixtures
  nested positive, duplicate, missing, foreign, wrong owner/loop/block,
  wrong operand/class, and repeated-ordinal negatives
```

The product may be test-only until a later production consumer is selected,
but the owner and reject semantics must be the final contract. No AST lookup,
name matching, source preorder rematch, Builder, ValueId, BasicBlockId,
operation MIR, Return, DraftSeal, selector, retry, fallback, or legacy route
deletion is allowed in S0.

The P0 `into_physical_boundary` path intentionally drops operation source
evidence. S0 must issue the operation product before that path; it must not
reuse P0 after anchors have been discarded. Generic G0 source evidence is a
later adapter row and must be retained at its producer boundary.

## Verifier obligations

The verifier accepts only when all are mechanically true:

```text
every Recipe Operation item has exactly one evidence row
item belongs to the canonical Recipe block and loop
anchor owner/source-loop matches the Core brand
placement claim matches the unique Recipe membership
ReadBinding/WriteBinding uses the exact Core effect relation when required
pure operation has no fabricated binding/effect relation
duplicate/missing/foreign/wrong placement rejects as typed NoSafeSlice
repeated role ordinals never determine item identity
```

The verifier does not require every Core effect row to be consumed. Tail/After
and structural carrier rows remain outside this product by explicit contract.

## Acceptance gates

```text
cargo test --lib loop_operation_effect -- --nocapture
cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

All touched source and test files remain below 800 lines. The S0 commit must
have no production caller and must preserve the existing P0 topology canary.

## Same-commit documentation obligation

When S0 code first lands, update in the same commit:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
src/mir/loop_recipe_contract/README.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
docs/development/current/main/10-Now.md
```

Reference pages may claim only the passive product and focused verifier that
actually landed. They must not claim operation physicalization, production
selection, backend parity, retry/fallback retirement, or legacy deletion.

## Exit and next row

S0 closes only when the product is move-only, the negative matrix is green,
the P0 path remains operation-free, and the same-commit references are exact.
The next row is a separate Callable adapter; Generic G0 anchor-ledger work,
cross-profile parity, and operation physicalization remain closed until their
own receipts are selected.
