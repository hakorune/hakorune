---
Status: SSOT
Decision: M11b-decl and M11b-load accepted; M11b-eval provisional
Date: 2026-05-08
Scope: M11b static const table source surface, MIR/static-data ownership, and parser rollout order.
Related:
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/static-data-manifest-v0.toml
  - docs/reference/runtime/substrate-capabilities.md
  - docs/reference/language/EBNF.md
  - docs/reference/language/types.md
---

# Static Const Table Syntax (SSOT)

## Decision

M11b is not one monolithic feature.

Split it into three rows:

```text
M11b-decl:
  source static const table declaration
  parser + AST/Program metadata + MIR static_data_plan
  no table read expression yet

M11b-load:
  static table read route
  compile to direct static-data load
  no runtime ArrayBox / MapBox construction

M11b-eval:
  const expression / const fn evaluation
  source values may be derived at compile time
```

`M11b-decl` and `M11b-load` are implemented for the first narrow `u16` shape.
`M11b-eval` remains a follow-up row.

## Relationship To M11a

M11a already landed a backend-private static-data manifest row:

```text
docs/development/current/main/design/static-data-manifest-v0.toml
```

That manifest is for built-in backend-private data defaults. Source programs
must not mutate or append to that checked-in manifest.

M11b source declarations lower into module-owned metadata:

```text
source static const
-> AST/Program metadata
-> MIR module metadata static_data_plans
-> backend data row reader
-> LLVM readonly global
```

The backend reads rows. It must not rediscover table meaning from source names,
app names, or allocator names.

## First Accepted Source Shape

The first accepted source shape is intentionally narrow:

```hako
static const SIZE_CLASS: u16[] = [
  8, 16, 24, 32,
]
```

Acceptance rules:

- top-level declaration only
- `static const` keyword pair
- identifier name
- element type `u16` only for the first row
- array suffix `[]`
- initializer is a bracketed list of integer literals
- trailing comma is allowed
- every value must fit `0..65535`
- emitted data is readonly
- no runtime table object is constructed

Reserved for later rows:

- `u8`, `u32`, `u64`, signed integer element types
- explicit element count in the type
- literal suffixes such as `8u16`
- constant arithmetic in initializers
- references to other consts
- const fn
- mutation or publication as an `ArrayBox`

## Parser Rollout Contract

Because this adds source syntax, both parser fronts are in scope:

- Rust parser/tokenizer path
- `.hako` selfhost parser path

The implementation may be staged, but parser behavior must not silently diverge:

```text
M11b-decl implementation row:
  Rust parser accepts or fail-fasts with the same diagnostic class
  .hako parser accepts or fail-fasts with the same diagnostic class
  EBNF is updated in the same commit
  fixture covers both fronts or documents why one front is not active
```

If only one parser front can be implemented in a narrow commit, the other front
must get an explicit fail-fast diagnostic and a follow-up card. Silent ignore is
not allowed.

## MIR Ownership

The truth after parsing is a MIR-owned static data plan.

Proposed metadata vocabulary:

```text
static_data_plans: [
  {
    source_name: "SIZE_CLASS",
    symbol: ".hako.static.SIZE_CLASS",
    element: "u16",
    align: 2,
    linkage: "private",
    unnamed_addr: true,
    values: [8, 16, 24, 32]
  }
]
```

Rules:

- MIR/module metadata owns the table plan.
- Backend emitters consume the plan.
- Backend emitters do not scan AST declarations.
- `.inc` / C shim must not add allocator-specific table branches.
- Runtime does not allocate `ArrayBox` / `MapBox` for fixed static tables.

## Unsupported Diagnostics

Unsupported shapes must fail fast.

Suggested stable diagnostics:

```text
[static-const/unsupported-element]
[static-const/unsupported-initializer]
[static-const/value-out-of-range]
[static-const/parser-front-mismatch]
[static-const/backend-unsupported]
```

The exact string may be refined by the implementation card, but the diagnostic
must point at the source declaration, not at a downstream backend crash.

## Implementation Cards

### M11b-decl

Goal:

- parse the first accepted source shape
- preserve it as module metadata
- emit the same kind of LLVM readonly global as M11a

Non-goals:

- table load
- const eval
- const fn
- runtime object publication

### M11b-load

Goal:

- read from a static table without constructing a runtime collection
- route through a MIR/backend-owned static data load fact
- emit `static_data_load` in MIR JSON with table symbol/element/len/align
- VM reads the module metadata row and fail-fasts on out-of-bounds index

Non-goals:

- general pointer arithmetic
- LLVM-side dynamic index proof beyond the first narrow direct-load behavior
- allocator policy ownership

### M11b-eval

Goal:

- allow compile-time expressions and eventually const fn for table generation

Non-goals:

- executing arbitrary user code at compile time
- side effects during const evaluation
- allocation during const evaluation

## Acceptance For M11b-decl

Required outputs:

- EBNF reserved syntax becomes accepted for the first shape
- language manual states the live narrow surface
- MIR/module metadata exposes `static_data_plans`
- LLVM/.hako emitter consumes `static_data_plans`
- no runtime `ArrayBox` / `MapBox` allocation occurs for the table
- unsupported shapes fail fast
- guard or smoke proves the emitted global line

Suggested fixture:

```text
apps/smokes/static_const_table_decl_u16/main.hako
```

Suggested gate:

```bash
bash tools/checks/k2_wide_static_const_table_decl_guard.sh
```

## Acceptance For M11b-load

Required outputs:

- `NAME[index]` resolves to a static-data load only when `NAME` is declared in
  module `static_data_plans`
- MIR owns a canonical `StaticDataLoad` operation
- MIR JSON emits `op = "static_data_load"`
- VM executes the load from module metadata with negative/out-of-range fail-fast
- `.hako` ll_emit emits direct LLVM `getelementptr` + `load` + zero-extend to
  current `i64` lane
- unknown table names still use the existing Array/Map-only index diagnostic

Suggested fixture:

```text
apps/smokes/static_const_table_load_u16/main.hako
```

Suggested gate:

```bash
bash tools/checks/k2_wide_static_const_table_load_guard.sh
```
