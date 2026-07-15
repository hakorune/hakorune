---
Status: Consultation Required
Date: 2026-07-15
Decision: pending — exact static `i64` source-call authority
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0C-I64-DESIGN-STOP-001
Related:
  - mirbuilder-ssa-i1-compat-static-i64-parameter-selection-2026-07-15.md
  - mirbuilder-ssa-i1-compat-static-i64-return-selection-2026-07-15.md
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/mir-canonical-callsite-lane-ssot.md
---

# P0c-i64 Exact Static Source Call Design Stop

## Consultation objective

Select the smallest source-call row that connects the closed P0a exact `i64`
parameter ingress and R0a exact `i64` return egress without introducing a
second callable, Binding SSA, or name-resolution authority.

No source Call grammar is active while this card is pending.

## Current evidence

### Canonical ingress owns one function only

```text
VerifiedResolvedSourceUnitV1::resolve_function(root)
  root = exactly one FunctionDeclaration
  forest = that function plus nested semantic owners
  module-level sibling callable catalog = absent
```

The current product therefore cannot prove an arbitrary sibling static target.
The existing module-global function table is a runtime/materialization surface,
not pre-Builder source target authority.

### Source syntax has three distinct call families

```text
FunctionCall { name, arguments }
  direct source spelling: name(args)

Call { callee, arguments }
  general value/indirect call

MethodCall { object, method, arguments }
  receiver call
```

P0c must name one family. The proposed first row is `FunctionCall` only.
General `Call`, `MethodCall`, receiver routing, closures, externs, and builtins
remain outside it.

### Resolved semantics does not own direct-call targets yet

The current resolver recursively resolves FunctionCall arguments, but it does
not publish:

```text
SourceExprSiteV1 -> exact callable owner
exact formal/result ABI at the call site
call argument cardinality/representation rows
```

The legacy Builder resolves a global source name during lowering and contains
unique-static and tail-based recovery paths. Those paths are explicitly not
canonical P0c authority.

### Runtime substrate already supports the narrow execution law

The Rust MIR interpreter already executes `Callee::Global`, re-enters the final
callee function, validates exact parameter-entry contracts, and validates the
final return contract before publishing the result to its caller. Existing
recursive-call fixtures exercise final-callee contract rechecking.

This is reusable runtime substrate. It does not prove source admission or
call-site ABI by itself.

## Candidate slices

### A — exact current-owner self call (smallest executable slice)

```text
source:
  FunctionCall whose source name resolves to the current static function owner

callee:
  same FunctionOwnerIdV1 as caller

signature:
  one or more exact `i64` parameters
  exact `i64` result

arguments:
  exact arity
  every argument profile = InlineI64

result:
  InlineI64
```

Why this is smallest:

```text
new multi-owner source unit = 0
new callable catalog = 0
new runtime Call opcode = 0
new backend ABI = 0
new ownership operation = 0
```

It requires one new resolved self-target record and one co-sealed call-site ABI
row, but no arbitrary module name lookup.

The first executable fixture can be a finite recursive countdown using the
existing fallthrough If and one final explicit return.

Limitation: A proves call-site sealing, materialization, frame entry, and frame
exit, but it does not prove cross-owner callable lookup or atomic multi-function
publication. It is acceptable only if its target/ABI row is directly reusable
by B rather than becoming a current-owner-only parallel authority.

### B — arbitrary sibling static call

This is the more useful user-facing row, but current one-function ingress
cannot prove it. B requires a preceding module-level callable catalog or a
multi-owner resolved source unit that owns:

```text
exact source callable declaration identity
canonical symbol/arity
parameter/result ABI witnesses
duplicate/ambiguous declaration rejection
callee-before/callee-after publication law
```

Selecting B means the next implementation is the callable-catalog substrate,
not source Call lowering.

### C — reuse legacy global name resolution

Reject.

```text
raw FunctionCall.name lookup in Lower
unique-static recovery
tail/suffix recovery
runtime function-table lookup as semantic proof
```

all create a second or fallback authority and violate the D-prime route law.

## Proposed sealed product if A is selected

Names are provisional until the consultation accepts the row.

```rust
pub enum ResolvedStaticCallTargetV1 {
    CurrentFunction(FunctionOwnerIdV1),
}

pub struct VerifiedTrivialStaticCallV1 {
    site: SourceExprSiteV1,
    target: ResolvedStaticCallTargetV1,
    symbol: VerifiedStaticCallableSymbolV1,
    arguments: Box<[SourceExprSiteV1]>,
    parameter_abi: Box<[ExactTrivialParameterAbiV1]>,
    return_abi: ExactTrivialReturnAbiV1,
}
```

Co-seal law:

```text
call site is exact FunctionCall
target is exactly current owner
source call spelling and arity equal the sealed current-owner callable symbol
source argument count equals parameter-entry count
each argument expression is already sealed InlineI64
callee parameter rows are exact i64
callee return witness is exact i64
call result profile is InlineI64
call coverage is exact once
```

The row is part of `VerifiedTrivialCanonicalOwnerV1`; it cannot be fetched
separately and paired in Lower.

## Authority split

```text
VerifiedResolvedFunctionV1:
  exact source call target identity

VerifiedTrivialCanonicalOwnerV1:
  call-site argument/result representation and callable ABI compatibility

BindingSsaBuilderV1:
  argument reaching ValueIds and uses of the call result

canonical call materializer:
  result ValueId allocation and MirInstruction::Call emission

FunctionEntryContractOwner:
  final-callee runtime parameter validation

FunctionReturnContract:
  final-callee runtime result validation

Rust MIR interpreter:
  execute the already-verified Callee::Global
```

Non-authorities:

```text
raw source name in Lower
legacy call resolver/recovery paths
MIR function table
runtime symbol normalization
MirType::Integer alone
method name or arity suffix heuristics
```

## Materialization law if A is selected

```text
1. claim the exact sealed call row
2. lower only its declared argument expression sites
3. verify actual argument profiles against the row
4. allocate one ordinary expression result ValueId
5. project the physical symbol from the sealed callable symbol and cross-check
   it against the current unpublished function signature; do not reread or
   resolve FunctionCall.name
6. emit one Call with an attached exact Callee::Global
7. record/verify result type = MirType::Integer
8. publish the result only through ordinary expression/Binding SSA flow
```

The existing legacy `build_function_call`, unique-static recovery, tail-based
resolver, and `callee: None` path must not be used.

The first row must also choose a conservative call effect. Until a callable-
effect product is separately sealed, P0c must not infer purity in Lower. Using
an explicit conservative effect is safe; claiming `PURE` requires the profile
to own a closed effect proof for the recursive body.

## Backend boundary

First supported backend:

```text
Rust MIR interpreter only
```

Other backends fail before backend effects through the existing callable-
contract capability boundary until an exact P0c consumer is separately proven.
There is no VM fallback after another backend is selected.

## Implementation order after acceptance

```text
P0c-L0 — behavior-neutral canonical call emission facade
  explicit verified target input
  no legacy resolution/recovery
  production callers = 0

P0c-S0 — disconnected resolved self-call + co-sealed ABI row
  exact site/target/arguments/result/coverage
  Builder connection = 0

P0c-I1 — atomic first source FunctionCall
  finite exact-i64 self-recursive fixture
  Rust MIR interpreter execution
  unsupported backend fail-fast
  fallback/retry = 0
```

If B is selected, replace this series with a callable-catalog design and keep
source Call activation at zero until that substrate closes.

## Required fixtures after acceptance

Pass:

```text
exact current-owner self call
zero/one/multiple runtime recursion depth
argument from parameter/local/If PHI
call result assigned to local and returned
exact parameter and return contracts rechecked in recursive frame
```

Reject before Builder effects:

```text
different/unknown function name
ambiguous static name
wrong arity
untyped or non-i64 parameter/result
Bool/Float/Null/Void call result
general Call / MethodCall / extern / builtin / closure
early Return / Loop / ownership operation
```

## Stop conditions

Stop implementation if any step requires:

1. Lower to resolve or compare raw source function names;
2. unique-static, suffix, tail, or runtime-table recovery;
3. `callee: None` or a const-string `func` as callable identity;
4. a separately fetched call target and ABI row combined in Lower;
5. a second BindingRef-to-ValueId map;
6. runtime type checks to discover the result representation;
7. source unit parent/child canonical/legacy mixing;
8. canonical failure retry through A+ or legacy call lowering;
9. MethodCall, receiver, Box/View/Shared return, or Ownership SSA activation;
10. non-VM silent execution or VM fallback.

## Requested decision

Please choose exactly one:

```text
1. first target family:
   A — exact current-owner self call
   B — module-level callable catalog, then arbitrary sibling static call

2. first syntax:
   FunctionCall only

3. first ABI:
   exact i64 parameters -> exact i64 result

4. backend:
   Rust MIR interpreter only

5. order if A:
   P0c-L0 -> P0c-S0 -> atomic P0c-I1
```

Recommended answer for the smallest reusable implementation proof:

```text
A
FunctionCall only
exact i64 -> exact i64
Rust MIR interpreter only
L0 -> S0 -> I1
```

The recommendation proves the call ABI with the current one-owner ingress.
It does not claim arbitrary static-call support or selfhost corpus coverage.

If the decision criterion is instead “the first P0c must immediately unlock
ordinary caller-to-sibling calls”, choose B. In that case the next task is the
module callable catalog and atomic multi-function publication contract; source
FunctionCall remains inactive until both close.
