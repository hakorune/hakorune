# Loop operation physical demand P0

Status: `landed Builder-free full-demand/preflight 2026-08-07; physical emission remains closed`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-PHYSICALIZER-DESIGN-STOP / Decision B`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Change

Implement the Builder-free, move-only full-operation physical demand and its
complete semantic preflight. This row does not emit a MIR instruction and is
not the Const leaf-emitter canary.

```text
VerifiedLoopOperationEffectProductV1
+ VerifiedLoopSemanticContextV1
+ VerifiedLoopContinuationContractV1
  -> VerifiedLoopOperationPhysicalDemandV1
  -> prepare_all
  -> PreparedLoopOperationProgramV1
```

Callable and Generic G0 adapters must issue the same neutral demand shape from
their complete seven- and fifteen-operation products. The demand must expose no
API that selects, filters, or extracts one operation.

## Product contract

The exact Rust field layout remains private, but the semantic shape is fixed:

```text
VerifiedLoopOperationPhysicalDemandV1 {
  context: moved neutral semantic context
  operation_effect: moved full exact-coverage product
  continuation: moved neutral Loop continuation
  index: private key-only lookup cache
}

PreparedLoopOperationProgramV1 {
  demand: complete moved demand
  schedule: Recipe-structure-derived exact operation order
  coverage: exact complete-coverage receipt
}
```

The private index may accelerate item/value/binding lookups. It cannot select
execution order, filter by profile, inspect source names, or duplicate Recipe,
JoinSig, source/effect, CFG, SSA, or PHI truth.

`prepare_all` verifies before any Builder effect:

```text
context owner/origin/source-kind/loop-site/frame/Scope/Region identity
every Recipe operation is present exactly once
every operation kind and value class is supported by the declared schedule
every operand relation is exact
every ReadBinding/WriteBinding has the matching Core effect
every logical Loop/Block placement is unique
continuation is compatible with the moved Core/JoinSig
schedule count equals complete Recipe operation count
```

The schedule is derived from Recipe Loop/Block/Item structure. Evidence-vector
or item-key sort order is not execution authority.

## Context/continuation issuer correction

The earlier two-field sketch was incomplete. The common demand must move the
already verified semantic context as well as the operation/effect product and
the logical After continuation. Otherwise `prepare_all` would have no exact
source for the required frame and Scope/Region checks and would be forced to
re-catalog or guess them.

The implementation closes these two mechanical issuer changes:

1. Move the existing test-only `VerifiedLoopSemanticContextV1` wrapper into
   the neutral `loop_recipe_contract` layer. This is a transport move of the
   resolver-issued owner/origin/source-kind/loop-site/frame/Scope/Region
   evidence, not a new semantic owner.
2. Move the existing test-only `VerifiedLoopContinuationContractV1` wrapper
   into the same neutral layer. Callable and Generic G0 must consume their
   existing After capability exactly once; they must not reissue or clone it.
   The Generic G0 window lease must retain the resolver-issued Scope/Region
   pair so its adapter can issue the same context shape.

The issuer correction is now represented in the neutral product boundary.
If either profile cannot supply the exact context, return typed `NoSafeSlice`;
do not fabricate a context or weaken the contract silently.

## Required tests

- Callable full fixture issues and preflights all seven operations.
- Generic G0 full fixture issues and preflights all fifteen operations.
- Both paths have Builder/MIR effect zero.
- Missing, duplicate, foreign-owner, wrong-placement, unsupported operation,
  unsupported value class, invalid operand relation, and continuation mismatch
  reject with typed errors.
- Source preorder, profile label, route name, and item count do not select a
  different product.
- Compile-time/API census proves there is no `first_operation`,
  `select_operation`, filter, `take_operation`, or equivalent extraction path.
- The moved Core and continuation cannot be reused after demand issuance.

## File boundary

Production-neutral product code belongs under:

```text
src/mir/loop_recipe_contract/operation_physical_demand.rs
src/mir/loop_recipe_contract/operation_physical_demand_tests.rs
```

These files must not import Builder, MIR instruction writers, BasicBlockId,
ValueId, PhiTxn, Completion, DraftSeal, route selection, or legacy scheduling.
Every touched source/test file stays below 800 lines.

## Done

- [x] Add the private move-only full demand.
- [x] Move the neutral context and continuation issuers without cloning or
      reissuing resolver/JoinSig evidence.
- [x] Add `prepare_all` and the complete Recipe-derived schedule/coverage
      receipt.
- [x] Add Callable seven-row and Generic G0 fifteen-row positive tests.
- [x] Add the typed context/continuation full-preflight reject coverage; the
      operation-effect product owns its earlier duplicate/missing/foreign/
      placement/value rejection matrix.
- [x] Prove Builder/MIR effect and single-operation extraction APIs are zero.
- [x] Update code README, references, current pointers, and workstream in the
      same implementation commit.

Focused gates:

```text
RUSTFLAGS='-Awarnings' cargo test --lib operation_physical_demand -- --nocapture
RUSTFLAGS='-Awarnings' cargo test --lib operation_effect -- --nocapture
RUSTFLAGS='-Awarnings' cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

## Stop

Return to design and record `NoSafeSlice` if the product must borrow a Core,
copy a continuation, select one operation, infer execution order from evidence
sorting, inspect AST/name/profile/route data, or import any Builder/CFG/SSA/PHI
authority. Do not add a synthetic one-operation Recipe to make this row pass.

Explicit non-claims:

```text
no Builder/MIR instruction emission
no physical block receipt or placement binding
no leaf operation emitter
no ReadyLoopEntry or function session
no Return/Completion/DraftSeal/publication
no production selector, retry/fallback removal, or legacy deletion
```

## Same-commit documentation obligation

The implementation commit must update:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
docs/development/current/main/investigations/loop-recipe-operation-physicalizer-design-stop-2026-08-07.md
src/mir/loop_recipe_contract/README.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
```

References may claim only a Builder-free full-demand/preflight receipt. The
next row is the behavior-neutral physicalizer module split; operation MIR and
all production/retirement claims remain closed.
