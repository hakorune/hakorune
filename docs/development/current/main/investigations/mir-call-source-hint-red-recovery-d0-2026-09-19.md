---
Status: closed__corrected_test_contract__source_hint_red_recovery__2026-09-19
Task: MIR-CALL-SOURCE-HINT-RED-RECOVERY-D0
Date: 2026-09-19
Parent: mir-call-static-compatibility-catalog-target-d0-2026-09-14.md
Implementation permission: false; classify the current-change red before code or test edits
Classification: BoxShape; one source-hint lifecycle test with explicit positive and negative rows
---

# Source-hint red recovery D0

## Six-line brief

```text
Decision: recover the current source-hint lifecycle red only after separating the valid root Array<i8>=[7] row from prefix and typed-child rejection rows.
Source authority + canonical issuer: existing resolver-owned Script source product and ArraySourceLifecycleRows issuer; source_hint only transports prepared text/lineage and does not invent array ownership.
Non-authority: error-string guesses, imported alias maps, AST/MIR rescans, legacy compile fallback, baseline manifests, or a relaxed assertion that hides a changed terminal.
Fail-fast boundary: the exact source-hint test must identify its first named terminal; valid root-array admission must remain distinct from imported-prefix and invalid-child capability rejection.
Smallest next slice: reproduce the single failing test, record the actual terminal, then map each finite fixture to the existing owner guard before deciding whether one owner-local fix is safe.
Non-claims: no parser-loop promotion, static catalog/publication, VM repair, fallback restoration, baseline reclassification, or production switch.
```

## Red inventory and boundary

The historical A0-2 source-hint test was green at `8ee28a0d8f` and is red at
the current head. The owner is
`src/mir/builder/normal_script_array_source_lifecycle.rs`; the test is
`runner::modes::common_util::source_hint::normal_tests::normal_preparation_preserves_local_with_and_without_prelude`.
The current terminal is now recorded. With the original fixture, the
source-backed package stops at
`Resolver(... Function(AppMainDirectCall(IndexSeal)))`: the imported
`Nested.get/0` and the prelude `Helper.get/0` collide in the source-unit
FreeStatic `(name, arity)` index. This is a fixture-shape conflict, not an
Array owner failure.

The fixture contains two independent dimensions:

| Row | Shape | Required classification |
| --- | --- | --- |
| P | root `local a: Array<i8> = [7]`, followed by `local b = 9` and `return 30` | positive source lifecycle admission and completion through the existing Array owner |
| I | the same root plus a nested imported static box and local prefix | named rejection at the existing caller-prefix boundary; no fallback; static method names must be unique in the FreeStatic index |
| C | typed-child capability cases already covered by the Array lifecycle owner (`bool`, float, out-of-range, nested/unknown) | explicit negative capability rows; never folded into P |

The test also checks lineage shape (root, prelude, nested segment), runtime
alias compatibility, and malformed source rejection. Those observations are
transport evidence; they do not prove Array ownership or a production caller
switch.

## Observation receipt — 2026-09-19

The first focused run with the default quick incremental build did not reach
the test: the linker emitted `undefined hidden symbol` diagnostics. A clean
`CARGO_INCREMENTAL=0` run reached the test in 6m19s and exposed the actual
fixture terminal:

```text
[mir/callable-semantic-package/issue] Batch {
  Resolver(SourceBoundSelectedCallableResolverRejectV1 {
    source: Callable { diagnostic_owner: Some("Nested"), diagnostic_name: "get" },
    error: Function(AppMainDirectCall(IndexSeal))
  })
}
```

The same run with only the two fixture method names changed to unique names
(`nested_get` and `helper_get`) reached the intended negative terminal
`[freeze:contract][script-array/source-lifecycle-unavailable] caller-prefix-capability`
and passed. This proves the old assertion remains the right negative contract;
the fixture was invalid after selected static-child direct-call indexing became
source-unit-wide.

## Existing owner map

| Concern | Existing authority | Gap to resolve |
| --- | --- | --- |
| merged source and lineage | `prepare_normal_source_with_imports` and `PreparedSourceWithImports` | no source-hint change is authorized until the downstream terminal is known |
| Array source ownership | `ArraySourceLifecycleRows::issue/consume/complete/finish_root` | confirm whether the positive row reaches `Available` and whether the failure is a real owner drift |
| prefix capability | `caller-prefix-capability` terminal from the source continuation/array owner | preserve as a named negative; do not broaden the positive row to accept imported prefix work |
| invalid child | existing `array_cutpoints` primitive-child checks and focused lifecycle tests | keep capability negatives separate from transport/lineage failure |
| baseline reds | `cargo_lib_red_baseline.failures.txt` and the static parent receipt | not part of this card; do not reclassify the five normal-catalog reds or the two manifest reds |

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Reproduce the exact red | **Closed.** The clean focused run recorded `IndexSeal`; the unique-name replay recorded `caller-prefix-capability`. |
| 2 | Split the finite fixtures | **Closed at the test boundary.** Root `Array<i8>=[7]`, imported prefix, and typed-child capability rows remain distinct. |
| 3 | Map to the owner | **Closed.** The failure is a duplicate FreeStatic fixture key; existing resolver/index and Array lifecycle owners are correct. |
| 4 | Choose one bounded action | **Selected:** a test-only fast I0 renames the two fixture methods uniquely and retains the named prefix negative. No production issuer/fallback change. |
| 5 | Closeout | Update the owner test/docs and pointer only after the terminal is observable; preserve baseline receipts and run pointer/diff checks. |

## Acceptance and non-claims

Acceptance requires one recorded first terminal for each finite row, a
positive `Array<i8>=[7]` owner path that does not rely on empty/default
capability, and explicit negative evidence for imported prefix and typed-child
rejection. A passing assertion that merely checks an error string without
proving the row classification is insufficient.

This D0 does not authorize changes to parser loop admission, static target
catalogs, VM/compatibility routes, baseline manifests, or production edges.
The selected fixture correction is a separate fast I0 and changes no compiler
behavior.

## Closeout receipt — 2026-09-19

The current-change red is explained by a stale fixture collision exposed by
the source-unit FreeStatic index. The positive Array source row and the
caller-prefix negative are both valid once the fixture uses distinct method
names. The next card is the test-only fast I0; no production code, fallback,
or semantic receipt changes are authorized by this D0.
