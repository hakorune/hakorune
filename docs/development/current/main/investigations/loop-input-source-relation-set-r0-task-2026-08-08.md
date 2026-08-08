# Loop input-source relation set R0 — execution brief

Status: `selected BoxShape prerequisite`
Date: 2026-08-08
Parent: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A-D0`
Next: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A`
Row: `LOOP-INPUT-SOURCE-RELATION-SET-R0`

## Change

Move callable's singular `VerifiedLoopInputRelationV1` into
`loop_recipe_contract/input_source.rs` as
`LoopInitializedLocalInputSourceRelationV1` rows inside one move-only
`VerifiedLoopInitializedLocalInputSourceSetV1`. Each row retains exact source
declaration, initializer, `BindingRefV1`, Recipe input value, and value class.
Delete the callable-local relation type in the same refactor series.

## Contract

`VerifiedLoopCoreProductV1` remains the Recipe/JoinSig/binding/effect owner;
the input set is a sibling source contract. Callable cardinality remains one.
V1 covers initialized-local Recipe inputs only; Generic parameter entries stay
under their existing separate contract. Resolver/profile production remains
the source-record authority, while the set verifies exact correspondence with
the already sealed declaration and Core binding relation.
All consumers iterate the complete set: no `first()`/filter API, hard-coded
declaration ordinal, partial verified row, new accepted source shape, or
Builder/CFG/PHI authority is allowed.

## Done

Focused tests prove exact callable cardinality one and reject empty, missing,
duplicate, foreign, declaration/initializer/binding, class/carrier, and Core
relation mismatches before Builder effects. Existing callable Recipe,
operation/effect, prepared-ingress, Prelude, and physical-canary receipts remain
equal. Every touched source/check file stays below 800 lines.

The implementation commit updates `src/mir/loop_recipe_contract/README.md`,
`src/mir/compiler/README.md` when its callable boundary text changes, and
`docs/reference/mir/loop-recipe-contract.md` with the landed common-set receipt.
Focused tests, the pointer guard, Recipe in-place/no-fallback guards, and
`git diff --check` must pass.

## Stop

Return to design if exact declaration/initializer authority cannot be retained
without AST reconstruction, if a second input authority or partial-row API is
needed, or if callable behavior changes. Do not implement S6A, add producer
provenance, widen family selection, or open M9/production/cutover in this row.
