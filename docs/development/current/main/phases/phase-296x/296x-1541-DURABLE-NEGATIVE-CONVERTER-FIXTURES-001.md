# 296x-1541 DURABLE-NEGATIVE-CONVERTER-FIXTURES-001

Status: landed
Date: 2026-06-22

## Purpose

Consolidate the MirBuilder converter fail-closed cases into a reusable
negative fixture corpus so the guard order, parked cases, and deny reasons all
live in one durable fixture manifest.

This row replaces the last inline case-ordering inside
`tools/rust_lifecycle/mirbuilder_negative_converter_fixtures.py` with a
fixture corpus manifest:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-negative-converter-fixtures-v0.json
```

The corpus keeps the parked `carrier_sensitive_alias` case explicit while the
rest of the cases stay green.

## Scope

```text
BoxCount: one reusable negative fixture corpus
owner: MirBuilder negative converter matrix
input: corpus manifest + existing fail-closed validators
output: one corpus-driven guard entrypoint
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_negative_converter_fixtures.py --all
bash tools/checks/rust_mirbuilder_negative_converter_fixtures_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```

## Acceptance

```text
one negative matrix guard runs the corpus manifest
case order comes from the fixture corpus, not inline guard ordering
carrier_sensitive_alias stays parked
each other case reports the intended Deny reason
no happy-path generated artifact changes
no silent hardcode / workaround markers added
```

## Stop Line

```text
do_not_reintroduce_inline_case_ordering=1
do_not_add_special_case_stringboxes=1
do_not_add_todo_null_placeholder_emission=1
do_not_open_nightly_rustc_adapter_path=1
```
