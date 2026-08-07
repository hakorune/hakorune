# Callable Static Prefix P0 Implementation Task

Status: `closed; bounded ABI/Prepared source cell`
Date: 2026-08-07
Parent: `LOOP-PHYSICAL-PREPARE-STATIC-CALL-FIXTURE-D0`

## Scope

Consume the closed `CALLABLE-STATIC-PREFIX-S0` observer and
`CALLABLE-STATIC-PREFIX-MAP-S1` source-map products for the exact resolver
fixture:

```text
int_to_str(n: i64): i64
  local value = to_i64(n)
  loop (...) { ... }
  return value

to_i64(n: i64): i64
  return n
```

Add only declaration-derived ABI and one positive
`PreparedCallableLoopPhysicalizationV1`-shaped test product. The existing
`MethodCall` fixture stays a typed negative.

## Required authority

```text
source observer -> source map
resolver callable header/index -> exact parameter/result ABI
completion declaration -> terminal return contract
brand relation -> same compilation only
```

The Prepared product may borrow the exact resolved callable input, but it must
not mint a target, re-resolve an AST, infer an ABI from a name, or compare raw
owner identity for the callee relation.

## Non-goals

```text
LoopRecipe / JoinSig             -> no new producer in this cell
CFG / SSA / PHI / ValueId        -> closed
physicalizer / Builder session   -> closed
production selection / I0        -> closed
retry / fallback / legacy delete -> closed
```

## Acceptance

- the static prefix retains the resolver-issued `to_i64` callable;
- caller/callee owner identity may differ, but compilation brand matches;
- parameter and result ABI are derived from the sealed declaration/header;
- completion remains a distinct contract from ABI and Tail;
- the MethodCall negative still rejects with typed `MissingPreludeTarget`;
- no physical Builder effect or production caller is introduced;
- touched source/check files remain below 800 lines;
- implementation commit updates compiler/lowering READMEs, exact
  `docs/reference/**` contracts, this task receipt, current workstream, and
  `CURRENT_STATE.toml` together.

## Verification

```text
cargo test --lib callable_single_loop_static_fixture_tests --no-fail-fast
cargo test --lib callable_single_loop_source_map --no-fail-fast
cargo test --lib loop_physical_prepare --no-fail-fast
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

After this cell closes, stop at the design boundary before opening any
physicalization or production-selection implementation.

## Implementation receipt (2026-08-07)

The callable prepare entry now derives the caller result ABI from the sealed
completion declaration and exact callable header, then checks the callee
result ABI from the resolver-issued target header. The static fixture produces
one positive `PreparedCallableLoopPhysicalizationV1` relation with
`FreeStatic` receiver shape and the retained `to_i64` target. The existing
MethodCall fixture still rejects with typed `MissingPreludeTarget`.

The old externally supplied ABI argument was removed from this test-only
prepare boundary, so ABI cannot be injected by the caller. No Recipe/JoinSig
producer, physicalizer, Builder effect, selector, retry, fallback,
publication, or production caller was opened. The source file remains below
800 lines and focused prepare/static-map tests are green.

The next step is design-only: audit the common physicalizer entry and its
session/finish contract before any physical implementation is opened.
