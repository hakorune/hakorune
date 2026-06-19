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

Rust functions without an explicit return type map to `"return_type": "void"`.
Rust functions with explicit unit return type `-> ()` also map to
`"return_type": "void"`.
If the body has statements but no terminal `return`, converters emit the body as
ordinary statements and keep the function return type as `void`.

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

### Index

```json
{
  "kind": "Index",
  "target": {"kind": "Name", "name": "xs"},
  "index": {"kind": "Name", "name": "i"}
}
```

Rust `xs[i]` maps to RustSubset `Index` in v0. The converter emits `.hako`
`xs[i]`. Array storage, bounds, and element semantics remain owned by the
Hakorune compiler/runtime; this app-front row only transports the expression
shape.

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

Rust method calls on `Vec<T>` values are represented with the same
`MethodCall` node as other receiver calls. V0 does not add Vec-specific schema
nodes; `.hako` Array behavior remains owned by the Hakorune compiler/runtime.

### ArrayLiteral

```json
{
  "kind": "ArrayLiteral",
  "elements": [
    {"kind": "Literal", "type": "i64", "value": 1},
    {"kind": "Literal", "type": "i64", "value": 2}
  ]
}
```

Rust `vec![a, b]` maps to RustSubset `ArrayLiteral` in v0. The converter emits
`.hako` `[a, b]`. Empty and typed-context behavior is owned by the Hakorune
compiler; this app-front row only transports the literal shape.

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

### Assign

```json
{
  "kind": "Assign",
  "target": {"kind": "Name", "name": "x"},
  "value": {"kind": "Binary", "op": "+", "left": {"kind": "Name", "name": "x"}, "right": {"kind": "Literal", "type": "i64", "value": 1}}
}
```

V0 accepts name and field assignment targets. More complex assignment targets
must be represented as `Unsupported` until selected by a later row.

### If

```json
{
  "kind": "If",
  "cond": {"kind": "Binary", "op": "==", "left": {"kind": "Name", "name": "x"}, "right": {"kind": "Literal", "type": "i64", "value": 0}},
  "then": [
    {"kind": "Return", "value": {"kind": "Literal", "type": "i64", "value": 1}}
  ],
  "else": [
    {"kind": "Return", "value": {"kind": "Literal", "type": "i64", "value": 2}}
  ]
}
```

`else` is optional. Rust `else if` is represented as an `If` statement nested as
the single statement in the parent `else` array. This keeps the schema recursive
without adding a distinct `ElseIf` node.

### While

```json
{
  "kind": "While",
  "cond": {"kind": "Binary", "op": "<", "left": {"kind": "Name", "name": "i"}, "right": {"kind": "Name", "name": "limit"}},
  "body": [
    {"kind": "Assign", "target": {"kind": "Name", "name": "i"}, "value": {"kind": "Binary", "op": "+", "left": {"kind": "Name", "name": "i"}, "right": {"kind": "Literal", "type": "i64", "value": 1}}}
  ]
}
```

Rust `while cond { body }` maps to `.hako` `loop(cond) { body }`.
Rust `loop { body }` without `break` or `continue` maps to the same `While`
shape with a literal boolean `true` condition, and emits `.hako`
`loop(true) { body }`.

`break` and `continue` inside loop bodies remain out of this app-front row and
should be represented as `Unsupported` until the compiler Recipe/CorePlan
loop/break backlog is accepted.

Rust `match` expressions are out of v0 semantic scope. The syn adapter must
represent them as `Unsupported` with a stable reason instead of adding a
RustSubset `Match` node or desugaring match arms.

Rust `for` loops are out of v0 semantic scope. The syn adapter must represent
them as `Unsupported` with a stable reason instead of adding iterator semantics,
desugaring into `While`, or claiming `break` / `continue` support.

## Compatibility Rules

```text
unknown schema_version -> fail-fast
unknown item kind -> fail-fast
unknown expression kind -> fail-fast
known unsupported Rust construct -> represent as Unsupported node
Unsupported node -> emit TODO comment
```

## Crate-skeleton P0 Gaps

The current v0 schema is enough for the accepted single-file fixtures. Before
crate-wide skeleton generation, the adapter/schema layer must stop losing Rust
path and emitted-name identity.

P0 before crate pilot:

```text
structured path / symbol reference
source_name vs emitted_name
reserved-word escaping policy
tuple field normalization
duplicate emitted_name detection
non-identifier pattern fail-fast / Unsupported
```

Current limitation:

```text
crate::model::Config may collapse to Config
other::Config may also collapse to Config
```

That is acceptable for existing single-file fixtures but not for multi-module
handoff.

Candidate direction:

```json
{
  "kind": "NameRef",
  "source_path": ["crate", "model", "Config"],
  "symbol_id": "crate::model::Config",
  "emitted_name": "crate_model_Config"
}
```

The converter must not perform Rust name resolution. The external adapter owns
source-path observation and deterministic emitted-name selection; the converter
prints `emitted_name`.

Tuple structs and unnamed fields must not produce invalid `.hako` identifiers:

```text
field 0 -> _0
field 1 -> _1
```

Alternatively, a future row may classify tuple structs as `Unsupported`. Until
that row is accepted, do not emit numeric field names such as `0: i64`.

Rust identifiers that collide with `.hako` reserved words must carry both names:

```json
{
  "source_name": "type",
  "emitted_name": "rust_type"
}
```

The adapter owns this deterministic normalization. The converter must not grow
a scattered rename table.
