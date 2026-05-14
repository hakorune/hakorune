# Local Type Annotation Metadata Capsule SSOT

Status: active
Date: 2026-05-14
Scope: `LOCALTYPE-001` Stage0 local type annotation metadata transport.

## Decision

Decision: accepted.

`local name: Type = expr` is accepted as metadata transport only. Stage0 parses
and carries the source type text; Stage1 owns type meaning, expected-type
propagation, and diagnostics.

## Syntax

```hako
local ids: Array<PageId> = []
local page: PageId = PageId(1)
```

MVP keeps typed local declarations single-binding:

```text
accepted:
  local x: T
  local x: T = expr
  local x: T = expr fini { ... }

rejected:
  local x: T, y
```

Untyped multi-local declarations remain unchanged:

```hako
local x, y, z
```

## Transport

AST `Local` carries `declared_type_names: Vec<Option<String>>`, aligned with
`variables` and `initial_values`.

Program JSON v0 local statements carry:

```json
{
  "type": "Local",
  "name": "ids",
  "declared_type": "Array<PageId>",
  "expr": ...
}
```

## Stop lines

```text
no Stage0 type checker
no type inference
no typed array literal semantics
no Result/Option expected-type inference
no PackedArray planner
```
