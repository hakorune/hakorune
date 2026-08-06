# Generic raw structured body-item source canonicalization S3

Status: `closed 2026-08-07; implementation row GENERIC-RAW-STRUCTURED-BODY-ITEM-SOURCE-CANONICALIZATION-S3-I0`

Parent receipt:

- `docs/development/current/main/design/fixtures/generic-raw-structured-method-receiver-receipt-s2-i0-v1.json`
- `src/mir/builder/raw_invocation_source_transport.rs::body_item_site`
- `src/mir/resolved_semantics/shadow/stmt.rs::stmt_body_item_path`
- `src/mir/resolved_semantics/source_path_policy.rs::SourceBodyKindV1`

## Problem

S2-I0 correctly transports the MethodCall `Receiver` receipt. The canonical
probe then stops because raw lowering names a nested body item as:

```text
[Body(1), IfThenBody, IfThen(0), Value, Receiver]
```

while the resolver's exact variable receipt is:

```text
[Body(1), IfThen(0), Value, Receiver]
```

`IfThenBody` is a root/region receipt, not part of the resolver's item site.
The same distinction applies to the existing rootless nested body-item family.

## Accepted design boundary

```text
source authority:
  SourceBodyKindV1::root_segment/item_segment
  ShadowResolverV0::stmt_body_item_path
  RawInvocationSourceContextV1::body_statement/body_item_site

physical owner:
  RawInvocationSourceContextV1::body_item_site

root contract:
  child_body(...) keeps the rootful region receipt
  body_statement(...) emits the canonical item site

rootless item-site kinds:
  Scope, TaskScope, FastMem, IfThen, IfElse, Loop, BlockExprPrelude

rootful item-site kind:
  Program (`ProgramBodyRoot` + `ProgramBody(index)`)

existing special:
  Function remains direct `Body(index)`

non-goals:
  resolver schema changes, variable_map/by-name lookup, Lambda/FirstCatch/
  Try/Cleanup admission, Generic Recipe/selector/production, Loop physical
  lowering, retry/fallback, AST rewrite, try-both source fallback
```

The canonicalization must be one shared source-path policy, not a new
per-branch exception. It may strip an active nested root segment only for the
accepted rootless item-site kinds; it must never strip `ProgramBodyRoot`.

## Minimum implementation slice

`GENERIC-RAW-STRUCTURED-BODY-ITEM-SOURCE-CANONICALIZATION-S3-I0`:

1. centralize the rootless item-kind decision next to `body_item_site`;
2. preserve existing Function, Scope, TaskScope, and FastMem behavior;
3. add IfThen/IfElse/Loop/BlockExprPrelude root stripping;
4. preserve rootful `child_body` receipts and Program item paths;
5. add a focused chained IfThen root→item test and Program regression;
6. rerun the canonical probe and retain the first fresh primary diagnostic.

The implementation may not add a resolver alias, normalize by name, or retry
with both path forms. A green result claims only source-path alignment and
primary-error advancement; it does not open the Loop production caller.

## Acceptance

- `cargo test raw_invocation_source_transport --lib` is green;
- `cargo test method_call_descent --lib` remains green;
- `body_statement` under an IfThen root yields `[Body(n), IfThen(i)]`;
- Program item paths remain `[ProgramBodyRoot, ProgramBody(i)]`;
- canonical VM probe no longer stops at the S2 `IfThenBody` mismatch;
- no Generic selector/Recipe/physical/production caller changes;
- current/reference/workstream docs and the exact immutable receipt are
  updated in the implementation closeout commit;
- the reference documentation update is repeated after the implementation
  cutover, not deferred to a later cleanup.

## Implementation closeout

S3-I0 is closed. The rootless item-kind policy now lives in the dedicated
`src/mir/builder/raw_invocation_source_item_site.rs` owner. It strips only
`Scope`, `TaskScope`, `FastMem`, `IfThen`, `IfElse`, `Loop`, and
`BlockExprPrelude` body roots; `Program` remains explicitly rootful and
`Function` remains direct `Body(index)`. `child_body(...)` still returns the
rootful region receipt, while `body_statement(...)` emits the canonical item
site. No resolver schema, variable admission, selector, Recipe, physical
lowering, retry, or fallback path changed.

Focused evidence:

```text
cargo test raw_invocation_source_item_site --lib  # 3 passed
cargo test raw_invocation_source_transport --lib  # 13 passed
cargo test method_call_descent --lib              # 5 passed
```

The fresh release probe exits at the next real boundary instead of the S2
path mismatch:

```text
[plan/freeze:contract] generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(3) }
```

The immutable S3 receipt is
`docs/development/current/main/design/fixtures/generic-raw-structured-body-item-source-canonicalization-s3-i0-v1.json`.
The next design stop is
`GENERIC-RAW-STRUCTURED-GENERIC-LOOP-CARRIER-REPRESENTATION-D0`; Loop
production selection, physical cutover, legacy retirement, and retry/fallback
remain closed. The reference documentation update is part of this closeout
and must be repeated again in the implementation/cutover commit.
