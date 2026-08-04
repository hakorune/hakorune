---
Status: SSOT; accepted target; implementation parked
Decision: accepted
Date: 2026-08-04
Scope: canonical Box value-member surface and retirement order for computed/once/birth_once properties
Related:
  - docs/reference/language/EBNF.md
  - docs/reference/language/README.md
  - docs/reference/ir/json_v0.md
  - docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md
  - docs/development/current/main/design/ownership-home-model-ssot.md
  - docs/development/current/main/workstreams/language-v1-convergence-current.md
---

# Box Member Field / Method Surface SSOT

## Decision

Hakorune retires the source-level Property subsystem. The final ordinary Box
surface has one storage concept and one behavior concept:

```text
field:
  stored place

method:
  behavior / computation
```

The user-visible law is exact:

```text
obj.x    = storage access
obj.x()  = method call
```

No registry lookup, generated getter name, cache state, or hidden call may
change an `obj.x` read into behavior.

This is an accepted language target, not current production behavior. The
current parser and MIR still accept and execute legacy Property forms. No
parser, AST, MIR, runtime, backend, or default-profile behavior changes in this
docs row. Implementation starts only when `CURRENT_STATE.toml` selects this
parked language workstream.

## Canonical member surface

### Stored fields

```hako
box X {
    name
    name: Type
    name = expr
    name: Type = expr
}
```

The four spellings are one concept:

```text
name:
  untyped stored field

name: Type:
  stored field plus declared-type metadata

= expr:
  per-construction initializer
```

The initializer follows the existing constructor lifecycle:

```text
allocate fresh identity
-> evaluate stored field initializers in declaration order
-> run matching birth
-> publish usable object
```

Stored initializer order, per-instance evaluation, `birth` ordering, and
partial-construction failure belong to the constructor/Home owners. Property
retirement must preserve the successful lifecycle and must not invent a second
constructor path.

### Weak fields

Weak fields remain storage with a distinct non-owning relation:

```hako
weak parent
weak parent: Type
public weak parent
```

`public weak parent` remains visibility sugar for
`public { weak parent }`. Weak storage is not a Property kind and is not part
of the retirement.

### Methods and birth

Behavior is always called with parentheses:

```hako
value(): Type {
    return me.computeValue()
}

birth(arg: Type) {
    me.value = arg
}
```

Methods and `birth` keep block bodies. The removed rule is narrower and exact:

```text
field declaration syntax admits neither `=>` nor a block body
```

## Retired source spellings

The following eleven Property spellings leave the canonical language:

```hako
get name: Type => expr
get name: Type { ... }

name: Type => expr
name: Type { ... }

{ ... } as name: Type

once name: Type => expr
once name: Type { ... }
{ ... } as once name: Type

birth_once name: Type => expr
birth_once name: Type { ... }
{ ... } as birth_once name: Type
```

The identifiers remain contextual. These ordinary declarations remain legal:

```hako
get: Type
get(): Type { ... }
once: Type
once(): Type { ... }
birth_once: Type
birth_once(): Type { ... }
```

Retirement removes Property declaration heads. It does not reserve `get`,
`once`, or `birth_once` as global hard keywords.

The legacy `init { field, ... }` stored-slot list is a separate compatibility
surface, not a Property kind. Its later migration or retirement requires an
independent grammar row and is not changed by this pack.

## Migration contract

### Computed property

```hako
get greeting: String => "Hello " + me.name
```

becomes an explicit method:

```hako
greeting(): String {
    return "Hello " + me.name
}
```

Call sites change from `obj.greeting` to `obj.greeting()`. The visible call is
intentional: field access cannot silently acquire allocation, I/O, failure, or
unbounded computation.

### birth_once

When the required behavior is only construction-time initialization:

```hako
birth_once config: Config => loadConfig()
```

becomes:

```hako
config: Config = loadConfig()
```

This is not a promise of complete semantic parity. The old hidden getter,
read-only-looking facade, and dependency-cycle mechanism are intentionally
retired. Encapsulation, readonly fields, and visibility enforcement are
separate Decisions and must not be smuggled into this row.

### once

`once` loses its hidden cache, poison, and retry/rethrow behavior. Existing
users must choose an explicit cache field plus method, or stop using the
capability. A future `OnceCell`, `Lazy`, or `memo method` requires independent
corpus evidence and a separate language/runtime Decision.

```text
future memo/lazy admission from this retirement = 0
```

## Why the old authority is retired

Current production does not carry Property meaning as a typed semantic
product. It reconstructs meaning after desugaring:

```text
source Property syntax
-> synthetic ordinary method names (`__get_*`)
-> MIR prefix recognition
-> PropertyKind recovery
-> dynamic field-read rerouting
```

The current parser also mixes stored initializer prologue emission with
computed/once/birth_once emission. `once` synthesizes hidden state and
protected control, while `birth_once` owns a separate dependency scan and
constructor prologue. This creates multiple authorities for storage, calls,
construction, failure, and Home relations.

The retirement replaces those authorities with existing owners:

```text
storage declaration and access:
  field metadata + ordinary field route

per-construction initialization:
  stored field initializer + constructor lifecycle

behavior and result Home relation:
  ordinary method + generated callable Home ABI

explicit caching:
  ordinary fields and methods
```

## JSON and artifact boundary

Property is not admitted as a new Program JSON v0 node. The current static
census found no production/runtime JSON artifact that requires Property
recovery. One formatter oracle JSON contains expected `__get_*` spellings; it
retires with the formatter.

After cutover:

```text
method named `__get_x`:
  ordinary method name only

Property meaning recovered from method spelling:
  forbidden
```

No JSON schema bump is selected by this Decision. The implementation census
must still classify AST JSON, Program JSON v0, generated fixtures, and cached
artifacts before deletion. Unknown or stale property-bearing inputs reject;
they do not reactivate magic-name inference.

## Home Flow boundary

This retirement precedes Home Flow production adoption because it makes the
source distinction structural:

```text
obj.x:
  place / stored Home relation

obj.x():
  call / GeneratedHomeAbi result relation
```

Property retirement does not implement Home Flow, field take, readonly,
visibility enforcement, or ownership promotion. It only removes the ambiguous
`obj.x` behavior route so those later owners receive one source meaning.

## Ordered task pack

The entire pack is parked behind the active MirBuilder lane. Rows below are one
BoxShape retirement series. No row may add an accepted member shape.

### 1. `BOX-MEMBER-FIELD-METHOD-SURFACE0-D0`

Change:
  Accept this final surface, intentional capability loss, artifact policy, and
  Home boundary. Production behavior remains unchanged.

Contract:
  Property is not a final semantic category; field and method are the only
  storage/behavior authorities.

Done:
  Reference grammar and navigation point here and distinguish accepted target
  from current production compatibility.

Stop:
  Any proposal to add memo, readonly, setter sugar, visibility enforcement, or
  same-name namespace admission returns to an independent Decision.

### 2. `BOX-MEMBER-PROPERTY-CENSUS0-D1`

Change:
  Close one parser/AST/MIR/JSON/selfhost/env/docs/guard/artifact census.

Contract:
  The census classifies authorities and consumers; usage counts alone do not
  decide semantics.

Done:
  Every old syntax producer, synthetic-name producer, PropertyKind consumer,
  field-read reroute, fixture, env gate, formatter, and artifact is assigned to
  preserve, migrate, reject, or delete. Production/runtime property artifact
  consumers remain zero or the row stops.

Stop:
  A previously unknown public artifact or external ABI consumer requires a
  compatibility Decision before implementation.

### 3. `BOX-MEMBER-STORED-INITIALIZER-OWNER0-S0`

Change:
  Move stored field initializer prologue construction out of the Property
  emitter into one field/constructor-owned module.

Contract:
  Source order, per-construction evaluation, all birth overloads, synthetic
  zero-argument birth behavior, and failure behavior remain unchanged.

Done:
  Stored initializer fixtures and constructor lifecycle gates pass while old
  Property syntax and production routing are still unchanged.

Stop:
  Any lifecycle or partial-construction semantic delta returns to the
  constructor/Home Decision rather than being patched in the extractor.

### 4. `BOX-MEMBER-PROPERTY-USERS0-S1`

Change:
  Migrate canonical repository sources to explicit methods or stored
  initializers. Convert old feature fixtures into migration/rejection evidence.

Contract:
  Computed users adopt `()`. `birth_once` parity is claimed only for the
  construction-initializer subset. `once` exact hidden semantics are not
  silently approximated.

Done:
  Canonical source use of all eleven retired spellings is zero, with explicit
  positive field/method fixtures and negative retirement fixtures prepared.

Stop:
  A real consumer requiring exact once/poison or birth_once facade semantics
  becomes a separate product Decision; it does not enlarge this row.

### 5. `BOX-MEMBER-PROPERTY-RETIRE0-I0-R0-G0`

Change:
  Atomically delete old parser edges, synthetic Property emission, dependency
  scan, PropertyKind/PropertyRegistry, dynamic field-read reroute, env gates,
  selfhost formatter/oracle, and old-only guards/docs. Land exact removed-syntax
  diagnostics in the same cutover.

Contract:
  Stored fields, weak fields, field initializers, methods, birth, delegate,
  invariant, transition, visibility grouping, and gate members remain intact.

Done:
  `obj.x` reaches only the ordinary field route; `obj.x()` reaches only method
  dispatch; no semantic `__get_*` inference remains; old syntax fails with a
  direct method/field migration hint; quick and language gates are green.

Stop:
  Do not retain a hidden compatibility registry, silent fallback, or default-
  off environment switch. A required compatibility profile needs an explicit
  source/profile contract and is not authorized here.

### 6. `BOX-MEMBER-SAME-NAME0-D0`

Change:
  After retirement, independently audit whether `x: T` and `x(): T` already
  coexist across parser, AST, MIR, runtime, JSON, delegate, visibility, and
  diagnostics.

Contract:
  This row is not required for Property migration. `getConfig()` remains a
  valid migration even when same-name coexistence is absent.

Done:
  If the shape already works, document and guard the existing contract. If any
  production layer rejects or conflates it, classify later activation as a
  BoxCount row with its own fixture and gate.

Stop:
  Do not bundle acceptance expansion into the retirement series.

## Hard stops

```text
do not call stored fields "properties" in the final model
do not retain `__get_*` spelling as semantic authority
do not turn old Property forms into a permanent compatibility profile
do not add setter DSL during retirement
do not rename once to memo without a new observable-semantics contract
do not claim private/readonly enforcement from current visibility metadata
do not combine Property retirement with Home Flow implementation
do not combine BoxShape deletion with same-name BoxCount activation
do not delete stored initializers or weak fields
do not silently include the legacy `init { fields }` list in this retirement
do not change the active CURRENT_STATE lane from this parked design
```

## Final acceptance

```text
canonical_value_member_kind_count = 2  # stored + weak storage relation
canonical_behavior_member_kind_count = 2  # method + birth hook
canonical_property_kind_count = 0
field_read_behavior_dispatch = 0
magic_getter_name_semantic_authority = 0
hidden_once_state = 0
hidden_birth_once_dependency_scan = 0
stored_initializer_owner_count = 1
property_compatibility_env_gate_count = 0
same_name_acceptance_added_by_retirement = 0
home_flow_production_delta = 0
```
