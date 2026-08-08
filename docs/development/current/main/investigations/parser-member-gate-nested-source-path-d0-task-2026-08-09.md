---
Status: parked design
Date: 2026-08-09
Decision: separate the known baseline red from the I0-A AST postpass cutover
Parent: `parser-public-ast-postpass-i0-a-implementation-task-2026-08-09.md`
---

# PARSER-MEMBER-GATE-NESTED-SOURCE-PATH-D0

## Problem

The existing test
`ordinary_nested_selected_else_keeps_outer_to_inner_source_path` fails on
parent `72b3471e55` before the parser opens the postpass product:

```text
member-level gate branches must preserve the same public signature
```

The failure is raised by the existing member-level build-gate signature
validator in `src/parser/declarations/box_def/body.rs`. The fixture's outer
then/else branches expose different public member signatures, so the failure
is a baseline contract mismatch, not an I0-A postpass regression.

## Scope

This row decides one of the following, with a language/reference decision
before implementation:

```text
1. repair the fixture so nested selected paths retain equal branch signatures;
2. revise the member-gate language rule and its source-seal obligations.
```

The row must not weaken the existing signature check by name, add a catch or
fallback, or make postpass admission decide a source-body semantic rule.

## Non-claims

```text
no I0-A cutover change
no postpass fallback
no AST/name scan
no member-signature relaxation without a language Decision
no source-seal or resolver authority
```

## Evidence

The exact parent-baseline command is:

```text
cargo test -q -p nyash-rust --lib \
  tests::parser::parser_box_method_inventory_r2::ordinary_nested_selected_else_keeps_outer_to_inner_source_path \
  -- --nocapture
```

It fails before `open_postpass_product` with the member-gate signature error.
I0-A records this as a known baseline red and does not claim the full
`parser_box_method_inventory_r2` suite is green.

## Exit criteria

```text
language policy or fixture contract is written in the owning reference
one focused positive/negative test fixes the exact boundary
the parser member-gate suite is green
owner README and current task map are updated in the same commit
```
