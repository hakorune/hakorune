# String `+` Coercion Contract Drift (2026-03-16)

Status: active investigation / cleanup input

## Summary

The codebase currently has a real drift between executable behavior and some user-facing reference docs.

- Live VM semantics still allow legacy-compatible mixed string concat:
  - `String + <non-void>` -> string
  - `<non-void> + String` -> string
  - `Void` / `Null` mixed with string `+` still fail-fast
- LLVM/selfhost lowering still preserves that contract through `nyash.any.toString_h`.
- `.hako` selfhost owners still use `"" + x` broadly as the generic stringify idiom.
- Some reference docs had already drifted to the stricter future shape "`String + String` only; use `x.toString()` explicitly".

## Code Reality

- VM:
  - `src/backend/mir_interpreter/helpers.rs::eval_binop(...)`
  - mixed string add is still implemented as `other.to_string()` coercion
- LLVM / selfhost lowering:
  - `src/llvm_py/instructions/binop.py`
  - mixed string add still bridges through `nyash.any.toString_h` and `nyash.string.concat_hh`
- `.hako` residue:
  - `lang/src/runner/launcher.hako`
  - `lang/src/runner/stage1_cli_env.hako`
  - `lang/src/mir/builder/**`

## Doc Drift

- Matches executable behavior:
  - `docs/reference/language/types.md`
- Had stricter explicit-only wording:
  - `docs/reference/language/quick-reference.md`
  - `docs/reference/language/LANGUAGE_REFERENCE_2025.md`

Historical phase notes may still mention the stricter target shape. Treat those as migration intent, not as proof that the executable contract has already changed everywhere.

## Current Working Contract

- Current executable contract:
  - numeric `+` behaves normally with int/float promotion
  - string `+` still accepts non-void mixed operands via stringification
  - `Void` / `Null` mixed into string `+` must still fail-fast
- Preferred source style:
  - use `x.toString()` in new code when the intent is explicit stringify
  - treat `"" + x` as legacy compatibility, not as the long-term style target

## Cleanup Order

1. Sync reference docs so they stop disagreeing about current behavior.
2. Choose one generic stringify contract/helper before changing broad call sites.
3. Replace `"" + x` owner-by-owner only after proof exists for that owner.
4. Only then consider tightening the runtime contract, and only if VM/LLVM/selfhost all move together.
