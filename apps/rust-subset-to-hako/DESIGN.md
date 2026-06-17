# Rust Subset To Hako Design

Status: design-only handoff

## One Sentence

Convert a small Rust structural model into `.hako` skeletons, without claiming
full Rust compatibility.

## Architecture

```text
              optional, replaceable
Rust source --------------------------+
                                      |
                                      v
                          external Rust parser adapter
                          syn / tree-sitter-rust / rust-analyzer
                                      |
                                      v
                              RustSubset JSON v0
                                      |
                                      v
                           hako converter app
                                      |
                                      v
                              Hako skeleton text
```

## Ownership

```text
External adapter:
  owns Rust parsing
  owns macro expansion choice
  owns concrete syntax recovery

RustSubset JSON v0:
  owns stable app input contract
  hides parser-specific AST details

.hako converter:
  owns skeleton generation
  owns type-name mapping
  owns record/box/function emission policy

Human / later compiler lane:
  owns semantic migration decisions
  owns borrow/lifetime/manual rewrite decisions
```

## Main Rule

The `.hako` app must not infer Rust semantics that are not present in
RustSubset JSON.

If the JSON does not prove a construct, emit a stable TODO comment rather than a
wrong `.hako` translation.

## V0 Mapping

### Struct

RustSubset:

```json
{
  "kind": "Struct",
  "name": "Point",
  "fields": [
    {"name": "x", "type": "i64"},
    {"name": "y", "type": "i64"}
  ]
}
```

Hako:

```hako
record Point {
    x: i64
    y: i64
}
```

Default rule:

```text
plain data struct -> record
struct with methods -> still record if fields are data-only
stateful/resource struct -> box only when schema marks identity=true
```

The schema must carry `identity=true` before the converter emits `box`.

### Impl Method

Rust:

```rust
impl Point {
    fn len2(&self) -> i64 {
        self.x * self.x + self.y * self.y
    }
}
```

Hako skeleton:

```hako
function Point_len2(me: Point): i64 {
    return me.x * me.x + me.y * me.y
}
```

### Enum

V0 emits a structural skeleton, not full algebraic semantics:

```hako
// enum ResultKind
//   Ok
//   Err(String)
```

Later rows can decide whether Hakorune enum surface or tagged records should be
used.

## Type Mapping

```text
i8/i16/i32/i64/isize -> i64
u8/u16/u32/u64/usize -> i64 for v0 skeleton unless schema requests usize
bool -> bool
String -> String
&str -> String
Vec<T> -> Array<T> comment for v0
Option<T> -> nullable/TODO comment for v0
Result<T,E> -> TODO result surface comment for v0
```

Do not pretend exact Rust integer width semantics are preserved in v0.

## Body Mapping

V0 body support:

```text
literal
field access
binary operator: + - * / == != < <= > >=
return expr
let name = expr
call(name, args)
method_call(receiver, name, args)
```

Unsupported body node:

```hako
// TODO(rust-subset): unsupported <kind>
```

## Error Policy

Fail-fast on invalid schema.

Emit TODO comments for valid but unsupported RustSubset constructs.

```text
invalid_json -> fail
unknown_schema_version -> fail
missing_required_field -> fail
known_unsupported_node -> TODO comment
unknown_node_kind -> fail
```

## Handoff Tasks

### RUST-SUBSET-TO-HAKO-001: schema reader

Implement a JSON reader for `schema_version=0`.

Acceptance:

```text
reads_simple_subset_json=1
invalid_schema_fails=1
unknown_kind_fails=1
```

### RUST-SUBSET-TO-HAKO-002: declaration emitter

Emit structs and functions.

Acceptance:

```text
struct_to_record=1
impl_method_to_function_with_me=1
unsupported_enum_todo=1
```

### RUST-SUBSET-TO-HAKO-003: expression emitter

Emit simple expressions.

Acceptance:

```text
field_access_emit=1
binary_expr_emit=1
return_emit=1
unsupported_expr_todo=1
```

### RUST-SUBSET-TO-HAKO-004: golden fixture

Compare generated output to `examples/simple_expected.hako`.

Acceptance:

```text
simple_fixture_matches=1
full_rust_transpiler_claim=0
```

## Stop Lines

```text
do not implement a Rust parser in .hako for v0
do not claim borrow/lifetime correctness
do not translate macros
do not silently drop unsupported declarations
do not use source-name special cases for examples
do not make record vs box decisions without schema evidence
```
