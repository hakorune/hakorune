---
Status: P0 normalized proof closed; I0 activation is at one design stop
Date: 2026-07-17
Decision: B-prime exact-i64 conditional callable-result catalog
Baseline: 06a49e5aa6
Parent: hmi-s0-v0-r0-same-module-call-result-representation-task-2026-07-17.md
Scope: declaration-order-independent pre-body result representation for same-module static calls
---

# Callable result exact-i64 catalog task

## Current progress

`R0-CALLABLE-RESULT-I64-CATALOG0-S0b` is closed. One lifetime-bound non-Clone
result catalog co-validates the exact declaration and source-target brands,
borrows exact target rows, substitutes required argument facts, and composes a
bounded String receiver with generated Core result rows. A deterministic
construction-only solver closes acyclic wrappers independent of declaration
order and classifies unresolved direct/mutual cycles without SCC inference.
Actual `skip_ws`, `to_i64`, and `_digit_value` rows are exact. Focused result
tests are 23/23; source-target 42/42, Core 5/5, malformed manifest 9/9, quick
66/66, structural guards, cargo check, and pointer guard are green. Production
producers/consumers and Builder/MIR/runtime/backend deltas remain zero.

`R0-CALLABLE-RESULT-I64-CATALOG0-P0` is also closed. One test-only normalized
snapshot projects result rows and exact-site call rows to canonical callable
keys or generated Core contract identity. It excludes catalog pointers,
borrowed AST identity, route-product addresses, and declaration order. Fresh
catalog/reorder parity, site multiplicity, unavailable/recursive rows, and the
actual StringHelpers/Parser wrapper chain are green. Production API, producer,
consumer, Builder, MIR, runtime, and backend deltas remain zero. The I0 audit
found a missing exact-site carrier and a borrowed-product lifetime cycle. The
durable brief is
`callable-result-i64-catalog0-i0-activation-design-stop-2026-07-17.md`.

`R0-CALLABLE-RESULT-I64-CATALOG0-L0a` is closed. One disconnected Builder
module now seals the complete static-box declaration inventory into structured
canonical keys, owned declaration rows, and deterministic method/arity
candidate lookup. The seal validates duplicate owners and keys, method-map
pairing, checked arity, and parameter/`ParamDecl` correspondence while
preserving optional return spelling and the paired source body.

Production producers and consumers remain zero. Existing
`static_method_index` and `lowered_method_asts` behavior is unchanged. Focused
catalog tests are 4/4, the current-state pointer guard is green, `cargo check`
is green, and quick is 66/66. Every added source file remains below 800 lines.
Candidate A defines the now-closed L0b cutover. One complete per-root
Static/Instance declaration catalog is installed before remaining declaration
indexing; two static recovery consumers share one zero/unique/ambiguous
decision, structured helper lookup uses the same catalog, and both old partial
stores are retired. Fifteen sources produce 11 pass and 3 reject executions in
debug/release. G0 freezes one catalog definition/producer/install, two recovery
consumers, one static-only candidate index, and zero retired authorities,
result-representation consumers, or GenericLoop consumers. The next
code-facing row is `R0-CALLABLE-RESULT-I64-CATALOG0-S0`.

### L0b recovery decision

The complete catalog cutover exposes an existing lowering-order-dependent
bare-static recovery split. Baseline rejects a provider-first bare call after
the provider has been lowered and duplicated in `static_method_index`, while
the canonical unique catalog accepts it. Caller-first succeeds in both.

This is a real semantic delta, so L0b may not retain its behavior-neutral
claim. Candidate A is selected: the complete immutable catalog makes exact-one
bare-static recovery declaration-order independent. The durable authority,
task order, fixtures, atomic cutover, guards, stash law, and stop conditions
are fixed in
`callable-catalog-l0b-canonical-unique-recovery-task-2026-07-17.md`.

## Decision

Three independent worker audits and a source-level feasibility review select
Candidate B-prime as the full-S0 target:

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

Target examples after the missing authorities are supplied:

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

### Full-S0 accepted source proof target

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
canonical-target static call contract substitution
all reachable Return values converge to exact i64
```

The proof is representation-only. It does not claim purity, termination,
constant folding, numeric range, or ownership behavior. Loop analysis proves
only an exact-i64 invariant; it does not prove that the loop terminates.

### Parked full-S0 call substitution

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

This composition is not part of S0a. A future solver may use a
canonical-key-sorted monotone worklist only after a canonical target product
exists. Declaration reorder
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
  -> R0-CALLABLE-RESULT-I64-CATALOG0-S0a
  -> R0-SOURCE-CALL-TARGET-AND-CORE-RESULT-D0
  -> canonical target/result authority implementation rows
  -> R0-CALLABLE-RESULT-I64-CATALOG0-S0b
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

Closed at L0a with production producers/consumers zero.

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

### L0b — canonical declaration-index/recovery cutover

```text
production behavior delta:
  provider-first unique bare-static recovery is newly accepted
catalog producers: 1
```

The accepted task is split into disconnected S0/P0 proofs followed by one
atomic CUT0. CUT0 builds and installs the catalog once before declaration-index
side effects, migrates both static recovery consumers and structured body
inspection, and retires `static_method_index` plus `lowered_method_asts` as
primary stores. Exact boundaries live in the linked canonical recovery task.

### S0a — disconnected local-body result catalog

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

S0a closed with call results unavailable. S0b subsequently supplied the
selected neutral target/Core authorities and now owns exact-site composition
plus the actual `to_i64`/wrapper rows without production publication.

### P0 — normalized proof

Closed evidence:

```text
test-only normalized snapshot definitions = 1
production snapshot definitions/consumers = 0
normalized proof tests = 4/4
callable-result focused tests = 27/27
source-target focused tests = 42/42
Core result-kind focused tests = 5/5
Core manifest malformed tests = 9/9
quick gate = 66/66
structural guards / cargo check / pointer guard = green
modified source/check files >= 800 lines = 0
```

The snapshot retains structured source-site multiplicity and semantic evidence
but never stores or compares pointer identity. Core evidence normalizes the
bounded receiver fact plus generated receiver/canonical/admitted-arity/result
row. It is `#[cfg(test)]` only and creates no new production authority.

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

Three worker audits found that this cannot be implemented as a direct emitter
connection. Legacy lowering carries no canonical caller plus
`SourceExprSiteV1`, and the declaration/target/result products cannot be
stored together in `CompilationContext` without self-reference or cloned
authority. The selected fixture also has an instance-method caller and a
static target contract. I0 is therefore split, pending one design decision:

```text
I0-ACTIVATION-D0
  -> I0-A0 owned single-use activation product
  -> I0-SITE0 exact legacy site ledger
  -> I0-CUT0 atomic candidate-Builder activation
  -> I0-G0
```

Candidate A, an owned non-Clone activation plan derived while the borrowed
proofs are live, is recommended. Candidate B, a stack-scoped borrowed lowering
session, remains the alternative. Exact questions, failure laws, fixtures,
and stop conditions are fixed in the linked design-stop card.

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

## S0a implementation may claim

```text
one complete immutable same-module static-callable declaration authority
declaration-order-independent local-body exact-i64 conditional result rows
unconditional declared or body-proven i64 results
argument-conditioned i64 results
actual `StringHelpers.skip_ws/2 = ExactI64 {1}`
all call results fail closed at one explicit target-authority boundary
production result consumers = 0
unsupported/dynamic source remains unavailable without guessing
```

## S0a implementation must not claim

```text
general callable result typing
source-call target projection
same-catalog call substitution
wrapper composition
`StringHelpers.to_i64/1 = ExactI64 {}`
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

## Current decision lock

Candidate B-prime S0b and its P0 normalized proof are closed. The exact
declaration/target/result identity chain, exact-site call rows,
required-argument substitution, bounded String Core composition, and
deterministic acyclic wrapper solver are the sole disconnected result
authority. The P0 snapshot is test-only and adds no production API. Bare
FunctionCall, general non-i64 typing, recursive/SCC inference, callee-first
lowering, retry, and fallback remain rejected. I0 code may proceed only after
the owned-plan versus scoped-session choice and exact source-site carrier are
decision-locked. The implementation must still retain one pre-body producer,
one successful-emission consumer, and failure-before-effects ordering.
