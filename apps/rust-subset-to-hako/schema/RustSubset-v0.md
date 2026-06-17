# RustSubset JSON v0

Status: design-only schema

## Document Shape

```json
{
  "schema_version": 0,
  "kind": "RustSubsetModule",
  "module": "sample",
  "items": []
}
```

## Item Kinds

### Struct

```json
{
  "kind": "Struct",
  "name": "Point",
  "identity": false,
  "fields": [
    {"name": "x", "type": "i64"},
    {"name": "y", "type": "i64"}
  ]
}
```

`identity=false` maps to `record`.

`identity=true` may map to `box`, but v0 should require an explicit reason:

```json
{"identity": true, "identity_reason": "resource_or_mutable_state"}
```

### Enum

```json
{
  "kind": "Enum",
  "name": "ParseStatus",
  "variants": [
    {"name": "Ok", "fields": []},
    {"name": "Err", "fields": [{"type": "String"}]}
  ]
}
```

V0 emits comments for enums.

### Function

```json
{
  "kind": "Function",
  "name": "add",
  "params": [
    {"name": "a", "type": "i64"},
    {"name": "b", "type": "i64"}
  ],
  "return_type": "i64",
  "body": []
}
```

### Impl

```json
{
  "kind": "Impl",
  "target": "Point",
  "methods": []
}
```

Method params may include a receiver:

```json
{"receiver": "self_ref"}
```

Receiver values:

```text
self_ref
self_mut
self_value
none
```

V0 maps all receivers to explicit `me: TargetType` in emitted `.hako`.

## Expression Kinds

### Literal

```json
{"kind": "Literal", "type": "i64", "value": 1}
```

### Name

```json
{"kind": "Name", "name": "x"}
```

### Field

```json
{"kind": "Field", "base": {"kind": "Name", "name": "self"}, "field": "x"}
```

### Binary

```json
{
  "kind": "Binary",
  "op": "+",
  "left": {"kind": "Name", "name": "a"},
  "right": {"kind": "Name", "name": "b"}
}
```

### Call

```json
{
  "kind": "Call",
  "callee": "add",
  "args": [{"kind": "Name", "name": "x"}]
}
```

### MethodCall

```json
{
  "kind": "MethodCall",
  "receiver": {"kind": "Name", "name": "p"},
  "method": "len2",
  "args": []
}
```

## Statement Kinds

### Let

```json
{
  "kind": "Let",
  "name": "v",
  "type": "i64",
  "value": {"kind": "Literal", "type": "i64", "value": 1}
}
```

### Return

```json
{
  "kind": "Return",
  "value": {"kind": "Name", "name": "v"}
}
```

### Expr

```json
{
  "kind": "Expr",
  "value": {"kind": "Call", "callee": "work", "args": []}
}
```

## Compatibility Rules

```text
unknown schema_version -> fail-fast
unknown item kind -> fail-fast
unknown expression kind -> fail-fast
known unsupported Rust construct -> represent as Unsupported node
Unsupported node -> emit TODO comment
```
