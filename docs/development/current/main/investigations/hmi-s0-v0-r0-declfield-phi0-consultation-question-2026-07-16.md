---
Status: Resolved
Date: 2026-07-16
Resolved: 2026-07-17
Decision: Candidate A′ accepted
Baseline: 584e1f8829
Parent: hmi-s0-v0-r0-declfield0-current-receiver-task-2026-07-16.md
Scope: same-root receiver PHI provenance for declared-field lookup
---

# R0-DECLFIELD-PHI0 consultation

## Resolution

Candidate A′ is accepted.

```text
one bounded use-site Copy/Phi proof
finite acyclic nested PHIs accepted
every terminal root = exact implicit receiver parameter 0
every CFG loop/backedge PHI rejected
persistent provenance table = 0
value_origin_newbox backfill = 0
```

The implementation owner is:

```text
hmi-s0-v0-r0-declfield-phi0-same-root-task-2026-07-17.md
```

The next code-facing row is:

```text
R0-DECLFIELD-PHI0-S0
```

The candidate discussion below remains as the design record. Where it differs
from the accepted resolution, the A′ task card is authoritative.

## Why work is stopped

`R0-DECLFIELD0-M0` measured the actual fallthrough-validation field base:

```text
FieldGet(items)
  -> Copy
  -> Phi(current_receiver, current_receiver)
```

Both PHI inputs are the exact current method receiver. The PHI destination and
the following Copy retain:

```text
handle:DeclaredFieldOwnerV1
```

The exact user-box registry retains:

```text
DeclaredFieldOwnerV1.items = ArrayBox
DeclaredFieldOwnerV1.map = MapBox
```

But the later `FieldGet(items)` has neither `declared_type` nor destination
type, so its calls route as:

```text
RuntimeDataBox.push / Union
RuntimeDataBox.length / Union
```

Candidate B-prime deliberately admitted only:

```text
Copy* -> current_receiver
```

and made every PHI a hard stop. M0 therefore selects exactly:

```text
PHI-ROOT-DESIGN-REQUIRED
```

No compiler change is authorized by this document.

## Source authority

Already sealed facts:

```text
function.params[0]
declared_param_decls[0].implicit_receiver = true
parameter 0 type = Box(current owner)
current function SSA definitions
Phi incoming value/predecessor rows
existing CFG/SSA verification
existing user_box_field_decls
```

The source declaration registry remains the only field-name/type truth.

## Non-authorities

```text
variable_map["me"]
current_static_box
function or MIR symbol parsing
method name
field-name special cases
runtime object tags
TypedObjectPlan backfeed
generic method route result
emitted MIR table scan during Lower
HMI source shape
stash evidence
```

The fact that both current inputs happen to be parameter zero is evidence, not
yet a durable compiler authority.

## Required fail-fast boundary

Any first PHI admission must reject unless all of the following are sealed:

```text
every PHI input recursively follows only Copy or accepted PHI
every terminal root is the exact same implicit receiver parameter
unknown roots = 0
foreign parameters = 0
mixed user-box owners = 0
CopyOwned edges = 0
Call / FieldGet / NewBox / Select roots = 0
definition cycles = 0
Phi predecessor/CFG validity is already verified
```

Unsupported shapes retain their existing behavior. They must not be repaired
by runtime inference or fallback.

## Candidate A — use-site same-root PHI proof

Extend the read-only field-base classifier:

```text
seed
  -> Copy*
  -> Phi
       each input -> Copy*/accepted Phi -> same receiver param0
```

The classifier is bounded by a visited set and the function's definition
count. It publishes no persistent provenance map and does not mutate PHI
metadata.

Field lookup remains:

```text
existing direct origin
  else same-root receiver proof
  -> existing declared field registry
  -> existing FieldGet.declared_type/value_types publication
```

Advantages:

```text
one narrow consumer
no global propagation delta
no origin backfill
all barriers stay local and explicit
```

Risk:

```text
receiver equivalence is recomputed at each selected field access
```

## Candidate B — strengthen existing PHI metadata publication

Teach the existing conservative PHI metadata owner to publish enough receiver
provenance when all inputs are the same current receiver root.

Advantages:

```text
downstream consumers reuse one PHI fact
other legitimate receiver consumers may benefit
```

Risks:

```text
widens a global SSA publication rule
may create a persistent ValueId -> owner authority
can mix declared-field work with unrelated PHI consumers
requires an exact owner for metadata invalidation and cycles
```

This candidate must not simply backfill `value_origin_newbox` without a new
sealed contract.

## Candidate C — sealed receiver-equivalence product

Add a pre-Builder or per-function immutable product mapping exact SSA values to
one receiver-equivalence class.

Advantages:

```text
one reusable proof owner
explicit schema and verification
```

Risks:

```text
large authority for one field-access blocker
potential second ValueId -> owner/type map
overlaps function-owned Binding SSA and existing type/origin facts
```

This is appropriate only if multiple independent consumers already require the
same equivalence proof.

## Recommended first slice

Candidate A is the narrowest likely architecture:

```text
one read-only bounded Copy/Phi classifier
all terminal roots exactly receiver param0
one declared-field fallback consumer
no persistent table
no origin mutation
```

However, this is a semantic authority decision rather than a mechanical
extension. External review must confirm:

1. whether same-root PHI is valid receiver identity authority;
2. whether use-site proof or PHI metadata publication owns the fact;
3. whether nested PHIs are admitted initially or only one PHI layer;
4. whether the first row must exclude loop/backedge PHIs even with one root;
5. which existing CFG/SSA verifier result is sufficient predecessor proof;
6. how the result is normalized without declaration-order or ValueId identity.

## Exact proposed first grammar

If Candidate A is selected, the smallest first profile is:

```text
current instance method only
exact implicit receiver parameter 0
field base type already Box(current owner)
zero-or-more Copy around one fallthrough PHI
every PHI input Copy* -> exact same receiver parameter
explicit field declaration on exact owner
ArrayBox positive / MapBox regression
```

Initially reject:

```text
nested PHI
loop/backedge PHI
Select
CopyOwned
Call
FieldGet root
NewBox root
foreign parameter
mixed owner
missing definition
cycle
```

The reviewer may instead authorize nested acyclic PHIs if that does not create
a second authority or a larger implementation seam.

## Implementation may eventually claim

Only after a new decision and green implementation:

```text
one same-root receiver PHI shape preserves declared-field lookup
all PHI inputs are proven to be the exact current receiver
existing FieldGet type publication and Known routes are reused
unsupported provenance remains unchanged
```

## Implementation must not claim

```text
general PHI type propagation
arbitrary receiver alias equivalence
loop receiver PHI support unless explicitly selected
ownership transfer through PHI
general declared-field propagation
runtime class inference
HMI completion
borrow/noescape ABI
backend widening
fallback or retry
```

## Stop conditions

Stop the future implementation if it requires:

1. function-name or `current_static_box` inference;
2. `variable_map["me"]` as canonical identity;
3. a second mutable ValueId-to-owner/type map;
4. `value_origin_newbox` backfill without a separately sealed owner;
5. runtime tags or downstream method-plan backfeed;
6. loop/backedge support in the same row without explicit admission;
7. ownership operations or source restructuring;
8. HMI-specific compiler branches;
9. fallback after failed proof;
10. any source/check file reaching 800 lines.

## Question

Should the first durable same-root receiver PHI authority be:

```text
A. one bounded use-site Copy/PHI proof
B. strengthened existing PHI metadata publication
C. a separate sealed receiver-equivalence product
```

If A, should the first row admit exactly one fallthrough PHI layer, or all
finite acyclic nested PHIs whose terminal roots are the same receiver?
