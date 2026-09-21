---
Status: closed__2026-09-22__WarningParserTestOnlyParseHelperI145
Task: MIRBUILDER-WARNING-PARSER-TEST-ONLY-PARSE-HELPER-I145
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i144-2026-09-22.md
Implementation permission: true; gate the production-zero parser helper pair
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I146
---

# MirBuilder parser test-only parse helper I145

## Six-line brief

```text
Decision: keep the parser helper pair available to repository tests while
  excluding it from the production library compilation.
Source authority + canonical issuer: the existing parser postpass entry and
  its NyashParser test-only wrapper; no new semantic issuer is created.
Non-authority: callsite names, AST/MIR inference, fallback, route selection,
  or a second parser entry.
Fail-fast boundary: any non-test caller, missing test compilation, or changed
  parser result returns this row to design_stop.
Smallest next slice: add cfg(test) to exactly the wrapper and its sole helper.
Non-claims: no parser behavior change, source admission change, production
  route switch, old-edge deletion, or warning suppression.
```

## Census and delete set

The selected symbols are:

1. `src/parser/normal_callable_program_source/mod.rs`:
   `NyashParser::parse_normal_callable_program_with_build_config`;
2. `src/parser/string_postpass_entry.rs`:
   `parse_normal_callable_program`.

The wrapper is the only caller of the helper. Every wrapper caller is inside
test modules/files; production parser consumers use
`parse_with_callable_parameter_source` or another explicit postpass entry.
The delete set is declaration-only: add `#[cfg(test)]` to both functions.
No test, parser implementation, or production caller is removed.

## Acceptance

Run one Cargo process at a time with the quick profile and four build jobs:

```text
cargo fmt --all -- --check
CARGO_BUILD_JOBS=4 cargo check --profile quick --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_callable_program_source
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib --no-run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The closeout records the before/after lib and lib-test warning counts and
proves that the focused parser suite remains green. A non-test reference or
any parser behavior difference reopens the design stop.

## Closeout evidence

The two declarations now carry `#[cfg(test)]`; no test source or parser
behavior changed. The focused parser suite passed **44/44**. The sequential
quick checks produced:

```text
baseline lib check (I144): 1,675 warnings
post-change lib check:      1,673 warnings
post-change lib-test:         543 warnings
```

`cargo test --profile quick --lib --no-run` passed and produced the same
543-warning lib-test baseline. `cargo fmt --all -- --check`,
`current_state_pointer_guard.sh`, and `git diff --check` all passed. The
production parser entry continues to use
`parse_with_callable_parameter_source`; only the test-only wrapper pair was
removed from the library warning surface.
