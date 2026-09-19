---
Status: closed__fast__source_hint_fixture_key_correction__2026-09-19
Task: MIR-CALL-SOURCE-HINT-RED-RECOVERY-I0
Date: 2026-09-19
Parent: mir-call-source-hint-red-recovery-d0-2026-09-19.md
Implementation permission: true; test fixture only
Classification: BoxShape; remove an invalid FreeStatic method-key collision without changing compiler behavior
---

# Source-hint red recovery I0

## Six-line brief

```text
Decision: keep the source-hint negative contract and make its imported static fixture valid under the current source-unit FreeStatic index.
Source authority + canonical issuer: existing source resolver/index and ArraySourceLifecycleRows; this I0 edits only fixture text in the owner test.
Non-authority: test method names as production namespace policy, error-string relaxation, AST/MIR inference, fallback, or a new receipt.
Fail-fast boundary: duplicate FreeStatic (name, arity) keys must be removed from the fixture before the test reaches the intended caller-prefix terminal.
Smallest next slice: rename Nested.get/0 and Helper.get/0 to distinct names, retain the exact caller-prefix assertion, and run the focused test once.
Non-claims: no production compiler fix, static publication, parser-loop promotion, baseline-red change, or backend acceptance.
```

## Exact change

In `src/runner/modes/common_util/source_hint_normal_tests.rs`, change only
the temporary Hako fixture method names:

```text
Nested.get/0   -> Nested.nested_get/0
Helper.get/0   -> Helper.helper_get/0
```

The imported box identity, lineage segments/edges, root
`Array<i8>=[7]` row, malformed-source row, and the expected
`caller-prefix-capability` negative remain unchanged. The resolver's
FreeStatic index intentionally keys one merged source unit by `(name, arity)`,
so the original two `get/0` declarations were an invalid fixture after static
child indexing became source-unit-wide.

## Acceptance

The focused test
`runner::modes::common_util::source_hint::normal_tests::normal_preparation_preserves_local_with_and_without_prelude`
passes with the unique names and still asserts the exact named prefix
terminal. `git diff --check`, `cargo fmt --all -- --check`, and
`current_state_pointer_guard.sh` pass. No production Rust file changes and no
new test count are allowed.

## Closeout receipt — 2026-09-19

The fixture now uses `nested_get` and `helper_get`. The focused command with
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4` completed in 6m20s:

```text
1 passed; 0 failed; 8145 filtered out
```

The final test retains the exact
`[freeze:contract][script-array/source-lifecycle-unavailable] caller-prefix-capability`
negative. This I0 changed one test fixture only; compiler, resolver, Array
owner, publication, fallback, and production caller edges are unchanged.
