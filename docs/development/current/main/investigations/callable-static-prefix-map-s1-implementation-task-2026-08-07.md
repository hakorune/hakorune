# Callable Static Prefix Map S1 Implementation Task

Status: `closed; bounded caller-zero source-map cell`
Date: 2026-08-07
Parent: `LOOP-PHYSICAL-PREPARE-STATIC-CALL-FIXTURE-D0`

## Scope

Consume the resolver-backed `int_to_str(n: i64) -> i64` / `to_i64(n: i64) -> i64`
fixture from `CALLABLE-STATIC-PREFIX-S0` and extend only the test-only callable
source map. The map must retain the explicit `FreeStatic` source shape and the
resolver-issued direct target; it must not issue a Recipe, ABI, physical plan,
Builder effect, selector, retry, fallback, or production caller.

## Required relation

```text
caller owner != callee owner                 allowed
caller compilation brand == callee brand    required
foreign compilation brand                    typed reject
```

The resolver callable index and direct-call ledger remain the sole target
authority. The map must compare compilation brands, not require owner identity
equality and not recover a target by name, ordinal, AST re-resolution, or a
lowering `variable_map`.

## Implementation slice

1. Change `map_prefix` to accept a different callee owner when the resolver
   proves the same compilation brand.
2. Add a focused positive map test for the static fixture and assert that the
   retained target is the resolver-issued `to_i64` callable.
3. Add a foreign-brand negative using two independently sealed catalogs; the
   rejection must occur before any physical effect.
4. Keep the existing MethodCall fixture as a typed negative with no direct
   callable target.

## Non-goals

```text
ABI derivation / Prepared positive        -> later LOOP-PHYSICAL-PREPARE-P0
LoopRecipe / JoinSig                      -> closed until co-seal design
physicalizer / CFG / PHI / ValueId        -> closed
production selection / I0                  -> closed
retry / fallback / legacy deletion        -> closed
```

## Acceptance

- `FreeStatic` and `Method` remain distinct source-shape evidence.
- Same-brand different-owner static target maps successfully.
- Foreign-brand target maps to a typed `ForeignOwner`/foreign-compilation
  rejection before Builder effects.
- No raw owner, target injection, name lookup, or AST rematch is added.
- Focused source-facts/source-map tests and existing prepare negatives remain
  green.
- Every touched source/check file stays below 800 lines.
- The implementation commit updates the compiler and lowering READMEs, exact
  `docs/reference/**` contracts, this task receipt, current workstream, and
  `CURRENT_STATE.toml` together.

## Verification

```text
cargo test --lib callable_single_loop_static_fixture_tests --no-fail-fast
cargo test --lib callable_single_loop_source_map --no-fail-fast
cargo test --lib callable_single_loop_syntax_facts --no-fail-fast
cargo test --lib loop_physical_prepare --no-fail-fast
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation receipt (2026-08-07)

The source map now accepts a resolver-issued static callee owned by a
different function when both caller and callee carry the same compilation
brand. A foreign compilation brand is rejected as `ForeignOwner` before any
source-map product is issued. The positive test retains the exact `to_i64`
callable from the resolver catalog; the foreign-brand test uses independently
sealed catalogs. The existing MethodCall fixture remains a typed negative.

No ABI, Prepared product, Recipe, physicalizer, Builder effect, selector,
retry, fallback, publication, or production caller was opened. Every touched
source/check file remains below 800 lines.

After this cell closes, open exactly one ABI/Prepared implementation cell. Do
not open physicalization or production selection from this task.
