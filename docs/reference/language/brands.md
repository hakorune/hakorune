# Brand declarations and explicit conversion

Decision: A top-level `brand Name: Type` declares one program-wide nominal
scalar identity. `Name(value)` explicitly constructs it and
`Name.unwrap(value)` explicitly extracts its underlying scalar.

```hako
brand PageId: i64

local page = PageId(7)
local raw = PageId.unwrap(page)
```

## Namespace

The effective declaration inventory is collected after build-gate pruning and
is visible throughout the program. Duplicate effective Brand names are rejected
with `[brand/duplicate-declaration]`; declaration order never selects a winner.

At an exact bare call site, a declared Brand name belongs only to the Brand
constructor namespace. It is not simultaneously resolved as a free function,
TypeOp, math helper, `str` normalization, or another compatibility call.
`externcall "symbol"(...)` is a separate explicit syntax. Brand names are one
identifier, so dotted names such as `mem.addr` cannot be declared Brands.

## Constructor and unwrap

Both operations require exactly one argument. Membership and arity are decided
before argument evaluation:

```text
[brand/constructor-arity]
[brand/unwrap-arity]
[brand/unsupported-static-method]
```

Construction retains the exact declaration identity, name, and underlying type
as semantic Brand identity. MIR and backends may represent the value using the
underlying scalar, but that physical choice does not make the value an
unbranded scalar. Only explicit unwrap removes the Brand identity.

The declaration catalog and exact constructor/unwrap source relations are the
semantic authority. Parser AST names, mutable compiler maps, callable lookup
misses, Program JSON strings, and physical MIR types are not alternate Brand
classifiers.

Flow-sensitive mismatch checks and implicit-conversion diagnostics remain later
verifier work; they do not weaken explicit construction or unwrap semantics.
