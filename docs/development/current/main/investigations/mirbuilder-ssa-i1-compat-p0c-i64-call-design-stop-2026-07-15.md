---
Status: Active — P0c-L0 closed; disconnected P0c-S0 next
Date: 2026-07-15
Decision: A′ — exact current-owner self call over a generic one-entry callable index
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0C-I64-S0-IMPLEMENTATION-001
Related:
  - mirbuilder-ssa-i1-compat-static-i64-parameter-selection-2026-07-15.md
  - mirbuilder-ssa-i1-compat-static-i64-return-selection-2026-07-15.md
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/mir-canonical-callsite-lane-ssot.md
---

# P0c-i64 A′ Exact Current-Owner Self-Call Task

## Decision

Select the current-owner self call as the first executable source-call family,
but do not encode “current function” as a permanent semantic target kind.

```text
first target family:
  exact current-owner self call

semantic target:
  generic ResolvedCallableRefV1

source lookup:
  source-unit-owned one-entry VerifiedCallableIndexV1

first syntax:
  ASTNode::FunctionCall only

first ABI:
  exact i64 parameters -> exact i64 result

first backend:
  Rust MIR interpreter only

effect:
  explicit conservative barrier

implementation order:
  P0c-L0 -> P0c-S0 -> atomic P0c-I1
```

The current one-owner source unit cannot honestly resolve an arbitrary sibling
function. A self call can be resolved exactly before Builder effects, while the
generic identity, index schema, call-site row, and materializer remain reusable
when a module callable catalog is added.

The central law is:

> A self call is not a special call kind. It is the first use of a generic
> callable identity resolved from a catalog whose accepted cardinality is one.

## Why A′ is the first row

The current canonical ingress owns exactly one root `FunctionDeclaration` and
does not own a module-level sibling callable catalog. Existing MIR module
symbols and the legacy global-call resolver are materialization/compatibility
surfaces, not pre-Builder source authority.

A′ nevertheless exercises the complete narrow call chain:

```text
exact FunctionCall source site
  -> pre-Builder callable resolution
  -> exact argument/result ABI
  -> ordinary expression result ValueId
  -> Callee::Global materialization
  -> recursive frame entry contract
  -> recursive return contract
  -> caller result publication
```

Selecting arbitrary sibling calls now would first require:

```text
multi-root source-unit inventory
all callable headers sealed before body analysis
duplicate/ambiguous key rejection
atomic unpublished multi-function draft set
declaration-order-independent resolution
mutual-recursion/SCC boundary
```

Those are follow-on tasks, not hidden work inside P0c-I1.

## Canonical callable vocabulary

### Semantic identity

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResolvedCallableRefV1 {
    owner: FunctionOwnerIdV1,
}
```

`ResolvedCallableRefV1` allocates no new identity. `FunctionOwnerIdV1` remains
the callable identity authority; the wrapper proves only that the owner is
valid in callable position.

Do not introduce:

```rust
ResolvedStaticCallTargetV1::CurrentFunction(...)
```

Such a variant would make the first implementation detail part of permanent
semantic vocabulary and require a second target family for sibling calls.

### Source key and physical symbol

```rust
pub struct CanonicalCallableKeyV1 {
    namespace: CallableNamespaceV1,
    name: Name,
    arity: u32,
}

pub struct CanonicalCallableSymbolV1 {
    // Physical MIR/backend spelling only.
}
```

The three concepts stay separate:

```text
CanonicalCallableKeyV1:
  source declaration/call lookup key

ResolvedCallableRefV1:
  semantic callable identity

CanonicalCallableSymbolV1:
  physical MIR/backend symbol
```

Never parse a physical symbol to recover source identity, use a symbol string
as semantic identity, or use the MIR function table as the source resolver.

### Exact trivial signature

```rust
pub struct ExactTrivialCallableSignatureV1 {
    params: Box<[ExactTrivialScalarAbiV1]>,
    result: ExactTrivialScalarAbiV1,
}

pub enum ExactTrivialScalarAbiV1 {
    I64,
}
```

This reuses the site-neutral exact scalar ABI already shared by P0a and R0a.
`MirType::Integer` alone is not source admission authority.

## One-entry callable index

```rust
pub struct VerifiedCallableIndexV1 {
    by_source_key:
        OrderedMap<CanonicalCallableKeyV1, VerifiedCallableHeaderV1>,
}

pub struct VerifiedCallableHeaderV1 {
    callable: ResolvedCallableRefV1,
    source_key: CanonicalCallableKeyV1,
    symbol: CanonicalCallableSymbolV1,
    signature: ExactTrivialCallableSignatureV1,
}
```

The index is source-unit-owned and is the sole callable-header authority.

For A′ it proves:

```text
entry count == 1
entry callable owner == current root FunctionOwnerIdV1
entry key == exact root namespace/name/arity
entry signature == exact all-i64 parameters and exact i64 result
duplicate or ambiguous entry == impossible/rejected
```

`FunctionSyntaxViewV1` currently lacks a complete callable header view. P0c-L0
must add one bounded header-view/index-builder seam rather than scatter raw AST
name, parameter, and return reads across resolver, analyzer, and Lower.

Future sibling-call work grows the index cardinality; it does not replace the
key, header, semantic reference, call-site row, or materializer.

## Resolved target product

`VerifiedResolvedFunctionV1` gains target identity only:

```rust
pub struct ResolvedDirectCallTargetV1 {
    callable: ResolvedCallableRefV1,
}

direct_call_targets:
    OrderedMap<SourceExprSiteV1, ResolvedDirectCallTargetV1>
```

Resolution order is fixed:

```text
1. observe the exact root callable header
2. build and seal the one-entry callable index
3. resolve the function body with that index in context
4. derive one CanonicalCallableKeyV1 per FunctionCall
5. resolve the key exactly once through the index
6. publish SourceExprSiteV1 -> ResolvedCallableRefV1
```

The A′ capability gate additionally requires the resolved target owner to equal
the current owner. The target record itself contains no “current” variant.

## Co-sealed direct-call row

The callable index owns full headers. A call row must not embed a second
independently authoritative `VerifiedCallableHeaderV1` copy.

Instead, the value profile co-seals a Lower-ready projection:

```rust
pub struct VerifiedTrivialDirectCallTargetV1 {
    callable: ResolvedCallableRefV1,
    symbol: CanonicalCallableSymbolV1,
    signature: ExactTrivialCallableSignatureV1,
    // Optional checked witness if the implementation needs drift detection.
    header_fingerprint: CallableHeaderFingerprintV1,
}

pub struct VerifiedTrivialDirectCallV1 {
    site: SourceExprSiteV1,
    target: VerifiedTrivialDirectCallTargetV1,
    arguments: Box<[SourceExprSiteV1]>,
    result: TrivialRepresentationV1,
    effect: VerifiedDirectCallEffectV1,
}

pub enum VerifiedDirectCallEffectV1 {
    ConservativeBarrier,
}
```

`VerifiedTrivialDirectCallTargetV1` is a co-sealed projection/foreign-key
witness derived from the index header. It is not a second header lookup or
mutation authority. Lower receives the complete verified direct-call row and
must not fetch target and ABI from separate products and combine them itself.

The owner profile contains:

```rust
pub struct VerifiedTrivialCanonicalOwnerV1 {
    // existing rows ...
    direct_calls: Box<[VerifiedTrivialDirectCallV1]>,
}
```

Co-seal law:

```text
source site is exact ASTNode::FunctionCall
resolved target exists at the same exact site
target is the one index entry and current owner
source key and index header agree exactly
argument site order/cardinality agree with the signature
every argument profile is InlineI64 and call-free
result profile is InlineI64
effect is ConservativeBarrier
call source coverage is consumed exactly once
```

## Exact first grammar

### Root and signature

```text
source unit:
  exactly one FunctionDeclaration owner

root:
  static, non-main, non-override
  no uses/contracts/attrs
  no nested owner/capture

parameters:
  one or more
  every source spelling exactly i64

return:
  exact source spelling i64

receiver:
  none
```

### Body

Keep the closed P0a/R0a trivial grammar and add one shape only:

```text
one exact ASTNode::FunctionCall source site
source key resolves to the current root callable
exact argument count
every argument is a call-free InlineI64 expression
call result is InlineI64
one final explicit InlineI64 Return
```

The result may appear as:

```text
local initializer
assignment RHS
BlockExpr tail
binary operand
final return expression
```

Reject before Builder effects:

```text
second FunctionCall site
nested FunctionCall argument
different/unknown callable key
wrong arity
untyped or non-i64 signature
general Call or MethodCall
extern/builtin/closure call
NewBox/String/Box result
Loop or early branch Return
QMark/Try/Catch
ownership operation
```

The schema may hold multiple rows, but P0c-I1 activates exactly one call site.

## First executable fixture

```hako
static countdown(n: i64): i64 {
    local result = n

    if n > 0 {
        result = countdown(n - 1)
    }

    return result
}
```

This one fixture covers zero, one, and multiple recursive frames, parameter
read, argument arithmetic, fallthrough If, call-result assignment, final
return, and per-frame entry/return contract enforcement.

## Authority split

| Subject | Authority |
| --- | --- |
| source callable inventory | `VerifiedCallableIndexV1` |
| callable semantic identity | `FunctionOwnerIdV1` via `ResolvedCallableRefV1` |
| call site to target | `VerifiedResolvedFunctionV1` |
| argument/result ABI and effect | co-sealed `VerifiedTrivialDirectCallV1` |
| exact source traversal | located source views |
| argument reaching values | function-owned `BindingSsaBuilderV1` |
| call result ValueId | canonical Lower |
| physical target symbol | `CanonicalCallableSymbolV1` projection |
| MIR execution | `MirInstruction::Call` with exact `Callee::Global` |
| callee argument validation | `FunctionEntryContractOwner` |
| callee result validation | `FunctionReturnContract` |
| draft publication | `CanonicalFunctionLoweringSessionV1` |

Non-authorities:

```text
FunctionCall.name in Lower
MIR module function table
legacy global-call resolver
unique-static/tail/suffix recovery
call-result name heuristics
MirType::Integer alone
runtime value tags
ValueId equality
physical symbol strings
```

## Materialization law

Canonical Lower performs only:

```text
1. claim the exact co-sealed direct-call row
2. lower its argument sites in recorded source order
3. require each materialized argument representation to match InlineI64
4. allocate one ordinary expression result ValueId
5. project the already-sealed physical symbol
6. cross-check the projection against the unpublished current draft signature
7. emit Call with an exact Callee::Global
8. record result type MirType::Integer
9. return the result through ordinary expression/Binding SSA flow
```

It must not call:

```text
build_function_call
build_legacy_function_call
build_unified_function_call
annotate_call_result_from_func_name
```

It must not emit `callee: None` or encode function identity as a const string.

The result ValueId is an ordinary expression value. No parameter or
return-specific ValueId, second binding map, `CopyOwned`, `DestroyOwned`, or
legacy `ReleaseStrong` is introduced.

## Effect law

P0c-I1 uses `VerifiedDirectCallEffectV1::ConservativeBarrier`, materialized by
one explicit effect mapper. Lower must not infer `PURE` from recursion shape or
the current body.

Pure-call refinement requires a separate callable-effect product and SCC fixed
point. It is not part of this task.

## Backend capability boundary

P0c-L0 must introduce a passive, durable capability witness/tag for:

```text
canonical_direct_static_call_v1
```

P0c-I1 activates it for the Rust MIR interpreter only. Every other backend
rejects before backend effects.

Do not rely only on the current parameter/return backend gates. Those gates may
gain additional backend support later without proving the canonical direct-call
consumer. The direct-call witness must distinguish this canonical row from
legacy generic calls; scanning for any `MirInstruction::Call` is insufficient.

There is no selected-backend retry and no VM fallback.

## Function publication law

The self-call target is the function currently being lowered as an unpublished
draft. Builder must not search the module function table for it.

```text
1. whole-unit preflight and callable-index seal
2. open unpublished function session
3. install sealed callable signature
4. seed parameter Binding SSA
5. materialize body and exact direct call
6. finish profile/coverage/Binding SSA/CFG
7. refresh entry/return/direct-call capability witnesses
8. MirVerifier
9. session cleanup
10. publish the function draft atomically
11. backend capability preflight
12. execute the selected backend
```

No partially verified function is published.

## Implementation series

### P0c-L0 — behavior-neutral callable facade

Status: closed.

Add, with production callers zero:

```text
bounded callable header view
CanonicalCallableKeyV1
ResolvedCallableRefV1
CanonicalCallableSymbolV1
ExactTrivialCallableSignatureV1
VerifiedCallableHeaderV1
one-entry VerifiedCallableIndexV1 builder/sealer
verified canonical direct-call emission facade
conservative call-effect mapper
passive canonical_direct_static_call_v1 capability carrier/gate vocabulary
```

Acceptance:

```text
grammar delta = 0
route delta = 0
runtime behavior delta = 0
production source-call callers = 0
CurrentFunction-specific target variants = 0
legacy call resolver callers from the facade = 0
```

Closeout evidence:

```text
bounded body-free CallableHeaderSyntaxViewV1 = 1
one-entry exact-i64 VerifiedCallableIndexV1 = 1
generic ResolvedCallableRefV1 identity wrapper = 1
CurrentFunction-specific target variants = 0
physical name/arity spelling admitted from source = 0
exact lookup fallback/tail matching = 0
verified emission arity mismatch = typed reject
conservative effect mask = exact known non-Pure barrier set
production source-call callers = 0
production capability producers = 0
grammar / route / runtime behavior delta = 0
```

Focused callable-index, emission, and backend-capability tests, the dedicated
P0c-L0 caller-zero guard, the full resolved authority guard, release build,
and quick gate are green. Every touched source/check file remains below 800
lines.

### P0c-S0 — disconnected resolved self-call product

Add and seal:

```text
one-entry callable index
SourceExprSiteV1 -> generic ResolvedCallableRefV1 target
co-sealed VerifiedTrivialDirectCallV1
exact target/symbol/arguments/result/effect/coverage
header projection drift checks
```

Acceptance:

```text
Builder connection = 0
production activation = 0
Lower raw name reads = 0
target and ABI late pairing = 0
```

### P0c-I1 — atomic first source-call activation

Activate exactly:

```text
one exact current-owner FunctionCall
exact i64 arguments -> exact i64 result
canonical materializer only
conservative effect barrier
Rust MIR interpreter only
explicit direct-call backend capability gate
fallback/retry = 0
```

Do not begin P0c-S0 before P0c-L0 is green, or P0c-I1 before P0c-S0 is green.

## Required fixtures

Pass:

```text
recursion depth 0 / 1 / multiple
argument from parameter
argument from local
argument from post-If Binding SSA PHI
argument arithmetic
call result as local initializer
call result as assignment RHS
call result as final return
exact source call site consumed once
target owner equals current owner
physical symbol equals current draft signature
callee parameter contract checked on every frame
callee return contract checked on every frame
verified MIR
VM/reference result equality
```

Reject before Builder effects:

```text
different or unknown function name
physical name/arity spelling in source
wrong arity
zero-parameter self-call in the first slice
second FunctionCall site
nested FunctionCall argument
untyped/non-i64 parameter or result
Bool/Float/Null/Void call result
general Call / MethodCall / extern / builtin / closure
target-owner mismatch
header fingerprint/signature drift
early Return / Loop / ownership instruction
```

Backend/transport:

```text
canonical direct-call witness survives draft finalization
Rust MIR interpreter accepts the capability
every unsupported backend rejects before effects
legacy generic Call does not masquerade as the canonical witness
```

## Authority counters

```text
one-entry callable index rows = 1
source FunctionCall sites = 1
ordinary call result ValueId allocations = 1

CurrentFunction-specific target variants = 0
canonical legacy build_function_call calls = 0
canonical legacy resolver calls = 0
Lower raw FunctionCall.name reads = 0
call-result name heuristic calls = 0
callee: None canonical calls = 0
const-string callable identity calls = 0
fresh parameter ValueId allocations = 0
second BindingRef -> ValueId maps = 0
CopyOwned = 0
DestroyOwned = 0
selected-route ReleaseStrong = 0
A+ retry = 0
legacy call retry = 0
partial function publication = 0
```

## Implementation may claim after P0c-I1

```text
one exact static self-recursive FunctionCall is production-supported
the source call target is resolved before Builder effects
the target uses a generic callable identity reusable by sibling calls
arguments and result use the existing function-owned Binding SSA
recursive frames reapply exact parameter and return contracts
canonical call lowering performs no name lookup or fallback
production ownership operations remain zero
unsupported backends fail before effects
```

## Implementation must not claim

```text
arbitrary sibling static calls
complete module callable catalog
multiple source call sites
mutual recursion
declaration-order independence
atomic multi-function publication
MethodCall or receiver support
closure/indirect/extern/builtin calls
call purity proof or tail-call optimization
Box/View/Shared result ABI
selfhost corpus coverage increase
all backend support
```

## Stop conditions

Stop this series if any step requires:

1. a permanent `CurrentFunction` target variant;
2. Lower reading, comparing, or normalizing `FunctionCall.name`;
3. the MIR module table as source target authority;
4. legacy unique-static, tail, suffix, or global-call recovery;
5. target and ABI fetched separately and combined in Lower;
6. a duplicated mutable callable-header authority in each call row;
7. a physical symbol string as semantic identity;
8. `callee: None` or a const-string function identity;
9. runtime value tags to discover the i64 result profile;
10. `PURE` without a closed callable-effect proof;
11. failure retry through A+ or a legacy call route;
12. sibling calls or multi-function publication inside P0c-I1;
13. unsupported backend silent execution or VM fallback;
14. `CopyOwned`, `DestroyOwned`, or legacy RC on the selected route;
15. function publication before call/profile/SSA/CFG/MIR verification;
16. backend admission inferred only from generic parameter/return capability.

## Follow-on order

After P0c-I1 closes, preserve the same types and materializer:

```text
P0c-CAT0:
  module-level callable declaration catalog
  all headers sealed before body analysis
  duplicate/ambiguous key rejection

P0c-MP0:
  multi-function resolved source unit
  unpublished draft set
  all-or-nothing atomic publication

P0c-B1:
  arbitrary sibling exact-i64 FunctionCall

P0c-MR:
  mutual recursion and callable SCC

P0c-N:
  multiple call sites and nested exact calls
```

P0c-CAT0 grows `VerifiedCallableIndexV1`; P0c-B1 reuses
`ResolvedCallableRefV1`, `VerifiedTrivialDirectCallV1`, and the canonical
direct-call materializer unchanged.
