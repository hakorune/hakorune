---
Status: S0 closed; M0 is the sole next row
Date: 2026-07-17
Decision: Candidate A-prime — one CorePlan Add-result representation owner
Baseline: 66c5eca052
Parent: hmi-s0-v0-r0-generic-loop-carrier-type-task-2026-07-17.md
Scope: repair the first false Add result fact exposed by loop_array_join
---

# HMI R0 CorePlan String Add representation task

## Current progress

`R0-COREPLAN-STRING-ADD-REP0-S0` is closed with production behavior and
production consumers both zero.

```text
module:
  normalizer/add_result_representation.rs

pure input:
  lhs/rhs Option<&MirType>

prepared output:
  exact MirType
  non-Clone

production calls:
  0
```

The disconnected decision classifies exact `String` and `Box("StringBox")`
first, exact Float second, and every remaining pair as the existing Integer
default. String therefore wins over Float, Unknown, missing, and other Box
facts without consulting route, function, variable, method, field, HMI, or
runtime identity. Focused 2/2, library check, formatting, pointer, diff, and
line budgets are green. M0 exact consumer/timing/differential inventory is
next; `helpers_value.rs` remains unchanged through S0.

## Decision

Select one pure CorePlan-normalizer result-representation decision. The first
production widening is exact and route-neutral:

```text
BinaryOp::Add + either exact String/StringBox operand
  -> exact MirType::String result

all other normalizer arithmetic cases
  -> preserve the current Float-else-Integer behavior in this row
```

The runtime reference already defines Add with either String operand as String
concatenation. This row repairs lowering-time representation metadata to that
existing semantic result; it does not add a new runtime coercion rule.

The selected product is CorePlan-local, for example:

```rust
enum CorePlanAddOperandClassV1 {
    String,
    Float,
    Other,
}

struct PreparedCorePlanAddResultRepresentationV1 {
    exact_type: MirType,
}
```

The implementation may use a smaller equivalent vocabulary. It must be a pure
decision made before destination allocation and must not receive function,
route, variable, method, field, or HMI identity.

## Worker-backed diagnosis

Three read-only audits agree on the first false fact:

```text
StringUtils.join/2

result init/header:
  exact String

result + separator:
  lhs = String
  rhs = Unknown or String

current PlanNormalizer::arithmetic_result_type:
  Float if either Float
  otherwise Integer

observed consequence:
  %14 = Add String, Unknown  -> incorrectly Integer
  %15 = If join             -> allocated Integer
  incoming %8               -> exact String
  TYPE-PUBLISH0             -> correct concrete-fact conflict
```

The selected route is `loop_array_join`, but the false fact is produced before
its nested If join. LoopArrayJoin facts own syntactic shape and variable
identity only. The shared LoopV0 carrier frame correctly derives `i: Integer`
and `result: String` from their init values. Keyed carrier maps preserve source
to destination correspondence; there is no positional swap.

## Authority boundary

```text
source Add semantics:
  existing language/runtime behavior

CorePlan operand facts:
  current transient value types

CorePlan Add result representation:
  new pure normalizer decision

destination allocation:
  existing PlanNormalizer lower_value_ast consumer

If join:
  existing build_join_payload

PHI conflict:
  unchanged TYPE-PUBLISH0
```

Explicit non-authorities:

```text
LoopArrayJoinFacts
array_join recipe reconstruction
GenericLoop carrier role/product
function or route name
result/separator variable spelling
join destination precedence
PHI overwrite or conflict weakening
runtime tags
backend lowering
```

## Exact first law

Operand classification reads only current representation facts.

```text
String:
  MirType::String
  MirType::Box("StringBox")

Float:
  MirType::Float

Other:
  Integer, Bool, other Box, Unknown, or missing
```

Decision priority:

```text
Add + String on either side:
  String

otherwise Float on either side:
  Float

otherwise:
  Integer, preserving current CorePlan behavior
```

String wins over Float because the existing runtime result is String whenever
either operand is String. Missing/Unknown does not become String by itself; it
only accompanies an independently exact String operand.

This row does not change the direct arithmetic Builder or final type-
propagation pipeline. M0 inventories their currently duplicated matrices and
records the divergence, but connecting those families would be a separate
BoxShape/semantic-parity row. The first I0 has exactly one production consumer.

## Task order

```text
R0-COREPLAN-STRING-ADD-REP0-S0
  -> R0-COREPLAN-STRING-ADD-REP0-M0
  -> R0-COREPLAN-STRING-ADD-REP0-I0
  -> R0-COREPLAN-STRING-ADD-REP0-G0
  -> R0-GENERICLOOP-CARRIER-TYPE0-G0
  -> clean HMI-S0-V0-R0-I0 rewrite
```

### S0 — disconnected pure decision

Production behavior delta is zero and production consumers are zero.

Add one pure decision plus focused tests. Keep it physically inside the
CorePlan normalizer boundary unless M0 proves a second immediate production
consumer is required.

Required matrix:

```text
String + String    -> String
String + Unknown   -> String
Unknown + String   -> String
StringBox + Other  -> String
Other + StringBox  -> String
String + Float     -> String
Float + String     -> String
Float + Integer    -> Float
Integer + Float    -> Float
Integer + Integer  -> Integer
Unknown + Unknown  -> Integer (unchanged first-row behavior)
```

### M0 — exact consumer and timing inventory

Production behavior delta remains zero.

Seal:

```text
CorePlan normalizer decision consumers = 1
direct arithmetic Builder matrices = inventoried, unchanged
final propagation matrices = inventoried, unchanged
LoopArrayJoin facts type authority = 0
join/PHI repair consumers = 0
```

Record normalized timing for:

```text
result init String
-> LoopV0 header String
-> Add result currently Integer
-> If join Integer
-> exact String incoming conflict
```

Also census accepted CorePlan Add candidates in debug/release. Stop before I0
if an existing legitimate non-string case depends on String classification not
winning when the other operand is exact String.

### I0 — one production consumer

Replace only `PlanNormalizer::arithmetic_result_type` with the prepared
decision before `alloc_typed`.

Unchanged:

```text
LoopArrayJoin facts and recipe
LoopV0 carrier allocation
build_join_payload
If join lowering
TYPE-PUBLISH0
direct arithmetic Builder
final type propagation
runtime and backends
```

Post-change normalized result:

```text
Add result = String
If join destination = String
loop step/backedge result carrier = String
concrete-fact conflict = 0
```

### G0 — closeout

Use the existing integration smoke and guard families; do not add a per-row
shell guard unless the existing entries cannot express the structural count.

Required:

```text
debug/release StringUtils.join = a,b,c
numeric and Float CorePlan parity
TYPE-PUBLISH0 guard green
GenericLoop carrier guard green
fresh HMI document-seal canary advances or passes
same compiler/interpreter reusable after rejection
```

Three TSV expectations still use the historical `LoopSimpleWhile` label while
the selected route is `loop_array_join`. M0 must classify those expectations;
G0 may update stale route labels only when the runtime semantics are already
green and the route identity is machine-proven.

## Counters and guards

```text
CorePlan Add-result decision owners = 1
S0/M0 production consumers = 0
I0 production consumers = 1

route/function/variable/method/field name conditions = 0
runtime tag reads = 0
LoopArrayJoin facts MirType fields = 0
GenericLoop product consumers = 0

join destination precedence delta = 0
PHI conflict/overwrite delta = 0
type_hint additions = 0

direct arithmetic Builder delta = 0
final type-propagation delta = 0
runtime/backend delta = 0
fallback/retry/legacy probing = 0
stash restoration = 0

source/check files >= 800 lines = 0
```

## Implementation may claim

```text
CorePlan Add with an exact String/StringBox operand publishes String result
representation before destination allocation

StringUtils.join keeps its result accumulator String through the nested If
join and LoopV0 backedge

LoopArrayJoin facts, LoopV0 carriers, join construction, and PHI validation
remain separate authorities

no route/name special case, runtime inference, PHI weakening, or fallback is
introduced
```

## Implementation must not claim

```text
one global arithmetic type system is unified
all direct Builder/pipeline Add matrices are retired
general union/coercion inference
all Unknown arithmetic becomes exact
LoopArrayJoin owns MIR type semantics
GenericLoop owns Recipe LoopV0 carriers
runtime/backend widening
HMI register completion before the later clean rewrite
```

## Stop conditions

Stop before or during I0 if any implementation requires:

1. a route, function, variable, method, field, or HMI-name condition;
2. storing `MirType` in LoopArrayJoin facts or recipe rows;
3. changing join destination precedence to conceal a false operand result;
4. weakening, overwriting, or retrying after a PHI concrete-fact conflict;
5. changing GenericLoop carrier authority;
6. inserting runtime coercions or reading runtime type tags;
7. changing direct Builder, final propagation, runtime, or backend behavior in
   the same row;
8. more than one first-row production consumer;
9. fallback, retry, legacy route probing, or stash restoration;
10. a source/check file reaching 800 lines.

## Final lock

> Candidate A-prime is selected. The loop_array_join failure is not a loop
> carrier mismatch: LoopV0 already seals `result` as String. The first false
> fact is the CorePlan normalizer's Float-else-Integer Add result allocation.
> One pure route-neutral decision therefore makes an exact String/StringBox
> operand dominate the Add result representation, ahead of the existing Float
> and Integer behavior. S0 and M0 remain disconnected; I0 connects exactly one
> production consumer before destination allocation. Facts, recipes, carrier
> maps, join payloads, TYPE-PUBLISH0, direct arithmetic Builder, final type
> propagation, runtime, and backends remain unchanged. No name special case,
> PHI weakening, coercion insertion, fallback, retry, or stash restoration is
> permitted.
