---
Status: decision locked; L0a is next
Date: 2026-07-17
Decision: B-prime exact-i64 conditional callable-result catalog
Baseline: 06a49e5aa6
Parent: hmi-s0-v0-r0-same-module-call-result-representation-task-2026-07-17.md
Scope: declaration-order-independent pre-body result representation for same-module static calls
---

# Callable result exact-i64 catalog task

## Decision

Three independent worker audits and a source-level feasibility review select
Candidate B-prime:

```text
complete same-module static-callable declaration catalog
  -> exact-i64 conditional result-contract seal
  -> target key + ordered argument representations co-seal
  -> successful Call emission
  -> existing type_ctx.value_types[dst] publication
```

This is neither a declared-ABI-only migration nor a general return-type
inference engine. A callable row may state:

```text
ExactI64 {
  required_i64_arguments: sorted unique argument ordinals
}

Unavailable(reason)
```

Examples:

```text
declared `: i64` result:
  ExactI64 {}

untyped `StringHelpers.to_i64/1`:
  ExactI64 {}

untyped `StringHelpers.skip_ws/2`:
  ExactI64 {1}

untyped forwarding `ParserStringUtilsBox.skip_ws/2`:
  ExactI64 {1}
```

At a call site, `ExactI64 {1}` publishes an exact Integer result only when
ordered argument 1 already has exact `MirType::Integer`. The callable is not
given a false monomorphic Integer ABI for calls with unknown/different
arguments.

## Why the other candidates are rejected

### Declared ABI only

An explicit source return annotation is a valid unconditional seed and CAT0
already owns this law for its exact-i64 function-only route. It is not enough
for the selected selfhost blocker:

```text
ParserStringUtilsBox.skip_ws/2:
  untyped wrapper

StringHelpers.skip_ws/2:
  untyped body
```

Adding annotations only to pass the fixture would be a source workaround.
The normal declaration index also currently drops `return_type_name`, which
explains why the typed-forward S0 control remains Missing.

### General module-wide return inference

A domain containing Float, String, Box, union, dynamic result, recursive SCC,
and coercion would be a second whole-language type system. It is outside this
row. B-prime seals only an exact-i64 sufficient-condition contract.

### Lowering-order and retry variants

The following remain rejected:

```text
callee-first lowering
declaration order as authority
provisional MIR followed by re-lowering
mid-lowering semantic/route refresh
final MirFunction metadata lookup
function/callee/HMI-name whitelist
GenericLoop default or definition-based inference
```

## L0 declaration authority

The current normal Builder has two mutable partial views:

```text
static_method_index:
  method name -> [(box, arity)]

lowered_method_asts:
  physical-looking function string -> params/param_decls/body
```

Creating a third body/result lookup beside these would duplicate callable
identity. L0 therefore introduces one primary product:

```rust
pub(crate) struct CanonicalSameModuleCallableKeyV1 {
    namespace: SameModuleCallableNamespaceV1,
    owner: Box<str>,
    name: Box<str>,
    arity: u32,
}

pub(crate) enum SameModuleCallableNamespaceV1 {
    StaticBoxMethod,
}

pub(crate) struct VerifiedSameModuleCallableDeclarationV1 {
    key: CanonicalSameModuleCallableKeyV1,
    params: Box<[String]>,
    param_decls: Box<[ParamDecl]>,
    return_type_name: Option<Box<str>>,
    body: Box<[ASTNode]>,
}

pub(crate) struct VerifiedSameModuleCallableDeclarationCatalogV1 {
    rows_by_key:
        BTreeMap<CanonicalSameModuleCallableKeyV1,
                 VerifiedSameModuleCallableDeclarationV1>,
    keys_by_method_and_arity:
        BTreeMap<(Box<str>, u32), Box<[CanonicalSameModuleCallableKeyV1]>>,
}
```

The catalog is built once from the complete Program before body lowering. It
is non-Clone. Rows may be Clone only where tests/diagnostics require it.
Physical MIR symbol spelling is a derived projection, never parsed back into
identity.

L0 migrates existing static resolution and narrow body-inspection consumers to
borrowed catalog queries, then retires the two partial primary maps. It owns no
result representation and changes no accepted source/runtime behavior.

## Result-contract product

```rust
pub(crate) struct VerifiedSameModuleCallableResultCatalogV1 {
    rows_by_key:
        BTreeMap<CanonicalSameModuleCallableKeyV1,
                 VerifiedCallableResultDispositionV1>,
}

pub(crate) enum VerifiedCallableResultDispositionV1 {
    ExactI64 {
        required_i64_arguments: Box<[u32]>,
    },
    Unavailable(CallableResultUnavailableReasonV1),
}
```

The product consumes only a borrowed complete declaration catalog and retains
canonical keys, not duplicate AST bodies or a second callable index.

### Exact first abstract domain

```text
ExactI64(requirements)
KnownNonI64
Unknown
Conflict
```

`requirements` is a sorted set of parameter ordinals whose call-site values
must already be exact Integer. Union means all listed conditions are required.

### Accepted first source proof

```text
integer literal
declared exact-i64 result seed
parameter i -> requirement {i}
local declaration / read / assignment
ordinary Copy-shaped alias
i64 +, -, *, /, % over proven i64 operands
finite fallthrough If dataflow
early Return collection
Loop-carried local whose init and every update are proven i64
same-catalog static call contract substitution
all reachable Return values converge to exact i64
```

The proof is representation-only. It does not claim purity, termination,
constant folding, numeric range, or ownership behavior. Loop analysis proves
only an exact-i64 invariant; it does not prove that the loop terminates.

### Call substitution

For a callee contract `ExactI64 {r0, ...}`, each required argument expression
must itself have an exact-i64 requirement set. The caller result requirement is
the sorted union after parameter substitution.

```text
callee ExactI64 {}:
  call result ExactI64 {}

callee ExactI64 {1}, caller argument1 = parameter0:
  call result ExactI64 {0}

callee ExactI64 {1}, caller argument1 = unconditional i64 expression:
  call result ExactI64 {}
```

The solver uses a canonical-key-sorted monotone worklist. Declaration reorder
must produce the same normalized rows. Unsupported dependency cycles remain
`Unavailable(RecursiveDependency)`; SCC result inference is not activated.

### Unavailable versus error

Dynamic or unsupported source remains valid source. It receives no exact row:

```text
missing/Unknown expression fact
known heterogeneous untyped returns
String/Float/Box/union result
unsupported method/property/call target
recursive or unresolved dependency
unsupported control-flow shape
```

These are `Unavailable`, not arbitrary Integer and not a new whole-program
compile error.

Hard seal errors are structural only:

```text
duplicate canonical key
parameter/ParamDecl cardinality or name mismatch
arity overflow
unknown catalog target in an otherwise exact call row
required argument ordinal outside target arity
declaration/catalog/result cardinality mismatch
```

An explicit `: i64` remains an unconditional ABI seed. Existing runtime return
contract validation remains the body/result conformance authority; this row
does not add a second annotation/body type checker.

## Production connection

The only production sequence is:

```text
complete Program declaration catalog seal
  -> result-contract catalog seal exactly once
  -> ordinary body lowering exactly once
```

Static-call resolution yields a canonical key before emission. One prepared
call-result row co-seals:

```text
target canonical key
ordered argument ValueIds
ordered current argument representations
matched exact-i64 result contract
result disposition
```

The call emitter must decide before mutation, emit the Call, and publish
`type_ctx.value_types[dst] = MirType::Integer` only after successful emission.
Failed emission publishes no type fact.

The selected same-module path must not parse the MIR symbol or consult
`current_module.functions`. Existing known builtin heuristics remain separate
legacy rows and may not serve as this catalog's authority.

## Exact task order

```text
R0-CALLABLE-RESULT-I64-CATALOG0-L0a
  -> R0-CALLABLE-RESULT-I64-CATALOG0-L0b
  -> R0-CALLABLE-RESULT-I64-CATALOG0-S0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-P0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-I0
  -> R0-CALLABLE-RESULT-I64-CATALOG0-G0
  -> R0-GENERICLOOP-CARRIER-TYPE0-G0
```

### L0a — disconnected complete declaration catalog

```text
production behavior delta: 0
production consumers: 0
```

Add the structured key, immutable declaration rows, catalog seal errors, and
source-independent tests. Do not touch existing partial maps yet.

Suggested structure:

```text
src/mir/builder/callable_declaration_catalog/
  README.md
  mod.rs
  key.rs
  catalog.rs
  error.rs
  tests.rs
```

### L0b — behavior-neutral declaration-index cutover

```text
production behavior delta: 0
catalog producers: 1
```

Build the catalog once in declaration indexing, migrate existing static method
resolution and narrow body-inspection consumers to borrowed queries, and
retire `static_method_index` plus `lowered_method_asts` as primary stores.

### S0 — disconnected result-contract catalog

```text
production behavior delta: 0
production result consumers: 0
```

Suggested structure:

```text
src/mir/callable_result_representation/
  README.md
  mod.rs
  disposition.rs
  expression_proof.rs
  function_proof.rs
  solver.rs
  error.rs
  tests/
```

No file may reach 800 lines.

### P0 — normalized proof

Required positive fixtures:

```text
declared exact-i64 forward and backward calls
untyped unconditional integer literal result
parameter-conditioned result
local alias and reassignment
early i64 returns plus final i64 return
loop-carried i64 update
one forwarding wrapper
two forwarding wrappers
argument-requirement substitution
declaration reorder parity
actual StringHelpers.to_i64/skip_ws chain normalization
```

Required unavailable/reject fixtures:

```text
Unknown argument at call site -> no result publication
known non-i64 argument at required ordinal -> no publication
heterogeneous untyped returns -> Unavailable
Float/String/Box/union result -> Unavailable
unsupported method/property flow -> Unavailable
direct and mutual recursion -> Unavailable
unknown catalog target -> structural error only in an exact row
duplicate key / arity / parameter cardinality drift -> typed seal error
```

### I0 — one production activation

```text
catalog producer: exactly 1 before body lowering
result solver: exactly 1 per complete catalog
same-module call-result consumer: exactly 1
```

The forward/reverse S0 fixtures must both execute through the same normalized
result row. The actual `ParserBox.static_const_parse_add/2` selected init must
be exact Integer before GenericLoop skeleton verification.

### G0 — guards and closeout

```text
structured declaration catalog definitions = 1
declaration catalog production producers = 1
result catalog production producers = 1
same-module result publication consumers = 1

static_method_index primary stores = 0
lowered_method_asts primary stores = 0
physical symbol reverse parsing = 0
current-module signature authority on selected path = 0
final metadata lowering-time reads = 0
callee-first / re-lowering / retry = 0
function/callee/HMI-name conditions = 0
GenericLoop role/type-policy delta = 0
new persistent ValueId type/owner maps = 0

source/check files >= 800 lines = 0
```

## Implementation may claim

```text
one complete immutable same-module static-callable declaration authority
declaration-order-independent exact-i64 conditional result contracts
unconditional declared or body-proven i64 results
argument-conditioned i64 results
finite non-recursive wrapper composition
call-result publication only when required arguments are exact i64
one successful-emission publication consumer
unsupported/dynamic source remains unavailable without guessing
```

## Implementation must not claim

```text
general callable result typing
general parameter inference
Float/String/Box/union inference
runtime type checking beyond existing contracts
purity or termination
recursive/SCC result inference
source annotation migration
callee-first or two-pass lowering
GenericLoop result inference
property/method result typing outside the selected static-call catalog
fallback or recovery
```

## Stop conditions

Stop and reopen design if any is required:

1. A non-i64 representation must enter the abstract domain.
2. Mixed/union results must receive an exact row.
3. Final MIR, function metadata, route fixpoint, or runtime tags are needed.
4. Callee-first, provisional lowering, re-lowering, or retry is needed.
5. Function/callee/HMI names or source annotations are required as exceptions.
6. GenericLoop must inspect call definitions or synthesize a default type.
7. A second callable declaration/identity/body catalog is required.
8. Result proof writes a persistent `ValueId -> type/owner` map.
9. Recursive SCC inference is required for the selected chain.
10. An existing dynamic function must become a compile error merely because
    no exact result contract is available.
11. A source/check file reaches 800 lines.

## Decision lock

Candidate B-prime is selected. The durable authority is one non-Clone exact-
i64 conditional result catalog sealed from one complete immutable structured
same-module static-callable declaration catalog before body lowering. A row is
either `ExactI64 { required_i64_arguments }` or explicitly unavailable. The
selected `skip_ws` chain is represented as an argument-1 conditional contract,
not a false monomorphic Integer signature. Canonical-key-sorted monotone
composition is declaration-order independent; unsupported or recursive shapes
publish nothing. One unified same-module call consumer combines the sealed
target contract with current ordered argument representations and publishes the
existing call-result `ValueId` type only after successful Call emission.
Declared-only migration, general return inference, final metadata, name rules,
callee-first lowering, re-lowering, GenericLoop inference, and fallback remain
rejected. `R0-CALLABLE-RESULT-I64-CATALOG0-L0a` is the sole next code-facing
row.
