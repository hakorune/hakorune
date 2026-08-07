# Cross-profile Loop operation/effect parity S0

Status: `CLOSED`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-EFFECT-GENERIC-G0-ANCHOR-S0`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

Prove that the callable and nested Generic G0 adapters issue the same neutral
operation/effect contract without claiming that their source shapes or item
sets are identical. Callable has its seven-item single-loop profile; Generic
G0 has its explicit fifteen-item nested profile. Profile-specific item keys,
Tail/After contracts, and carrier ownership remain separate.

## Contract

```text
callable source adapter -> neutral operation/effect product
Generic G0 anchor ledger -> neutral operation/effect product
                                      ↓
                         one schema / one verifier / one owner model
```

Parity must compare only the shared contract:

```text
same neutral product type
same exact item-keyed coverage rule
same owner/source provenance rule
same Recipe-derived placement rule
same Core effect relation matching
same pure-operation binding rejection
same duplicate/missing/foreign/wrong-placement rejection family
```

It must not compare source preorder, role ordinals, fixture names, Generic
labels, or the two profile item counts. Generic item 3 remains the explicit
`DerivedCarrierEntry`; callable has no equivalent row. Callable Prelude/Tail
and Generic `After`/tail reads remain outside the operation product.

## Acceptance gates

```text
cargo test --lib operation_effect -- --nocapture
cargo test --lib generic_g0 -- --nocapture
cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

Add one positive parity fixture and typed negative coverage for foreign owner,
missing/duplicate evidence, wrong placement, and an invalid profile-specific
relabel. Keep all touched source/test files below 800 lines.

## Implementation receipt (2026-08-07)

Closed as caller-zero, passive parity evidence. The new diagnostic-only
`LoopOperationEffectParityReceiptV1` validates both profile products through
the same neutral `VerifiedLoopOperationEffectProductV1` contract:

```text
Callable: 7 item-keyed rows
Generic G0: 15 item-keyed rows
Generic item 3: DerivedCarrierEntry(child carrier 2)
```

The receipt checks non-empty ordered unique evidence and owner-branded source
anchors/bindings. The existing common product verifier remains the authority
for exact Recipe operation equality, placement, effect matching, and the
duplicate/missing/foreign/wrong-placement/pure-binding rejection family. The
profile-relabel negative confirms that item 3 cannot be relabeled as item 4.

The focused `operation_effect` suite passes 8 tests and the focused
`generic_g0` suite passes 43 tests. No profile count comparison, source-order
comparison, Tail/After fusion, operation MIR, ValueId/BasicBlockId/PHI,
physicalizer, selector, retry/fallback, publication, or legacy deletion was
opened. The receipt is diagnostic evidence only and is not a new semantic
owner.

The next row is the design-only
`LOOP-RECIPE-OPERATION-PHYSICALIZER-DESIGN-STOP`: fix the common physicalizer
input/owner/failure contract and the Generic item-3 carrier bridge before any
physical operation canary.

## Stop line

This row remains caller-zero and passive. Do not emit operation MIR, allocate
ValueId/BasicBlockId/PHI, open a physicalizer, select a production route,
retry/fallback, or delete legacy callers. If the two products cannot be
compared without copying Recipe/effect truth, return to the neutral product
boundary and record `NoSafeSlice`.

After this closeout, the next design stop must keep the same restrictions until
its explicit canary contract is accepted. The physicalizer may consume the
move-only operation/effect product, a session-local single-use `ReadyLoopEntryV1`,
and borrowed canonical CFG/BindingSSA/PhiTxn services only; it must not receive
AST/name/route/profile/Tail/ABI/Completion authority or create a second CFG,
SSA, PHI, or retry owner.

## Same-commit documentation obligation

The implementation must update in the same commit:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
src/mir/loop_recipe_contract/README.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
docs/development/current/main/10-Now.md
```
