# 3281 - MIRBUILDER-CONDSKELETON-IFCOND-CONSULTATION-001

Status: landed

## Purpose

Decide whether `CondSkeleton` should gain `IfCond` now that If and Loop
condition producers share the ProgramJSON compare reader and BoolRecipe
vocabulary.

## Decision

Defer `CondSkeleton::IfCond`.

Current evidence:

- Rust `CondSkeleton` is still a Loop analysis profile surface.
- `.hako` already represents If and Loop conditions through
  `RecipeItem.cond_recipe` and `BoolRecipe::Compare`.
- RecipeVerifier validate-only and RecipeMatcher-facing observe-only snapshots
  already consume `cond_recipe` without requiring a Rust If profile skeleton.

Adding `IfCond` now would expand Rust condition-profile authority before a Rust
If condition consumer needs it.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_condskeleton_ifcond_consultation_guard.sh`

The guard proves:

- `CondSkeleton` remains `LoopCond` only
- `BoolRecipeBox` keeps six-op compare vocabulary
- If and Loop `cond_recipe` constructors exist
- the selected next card is BoolRecipe compare publication parity

## Claims

- `condskeleton_ifcond_deferred = 1`
- `bool_recipe_publication_next = 1`

## Non-Claims

- `CondSkeleton::IfCond` added
- Rust CondProfile authority expansion
- `.hako` consumer change
- ProgramJSON consumer change
- RecipeMatcher input authority
- BoolRecipe lowering
- MIR compare/branch emission
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-PARITY-001`
