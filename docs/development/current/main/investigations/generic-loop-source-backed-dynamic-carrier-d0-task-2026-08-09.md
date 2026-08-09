---
Status: accepted direction; design stop before I0 source issuer
Date: 2026-08-09
Row: `GENERIC-LOOP-SOURCE-BACKED-DYNAMIC-CARRIER-D0`
Blocks: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R1`
Mode: BoxShape / dynamic representation authority
---

# GENERIC-LOOP-SOURCE-BACKED-DYNAMIC-CARRIER-D0

## Decision

The current parser R1 source is valid. The first executable failure is:

```text
function = ParserScanLoopBox.skip_while/4
source   = local i = pos; loop(i < end) { ... i = i + 1 }
failure  = GenericLoop carrier i has Unknown representation
```

The compiler must accept a source-backed dynamic carrier. It must not repair
the source with an `i64` annotation and must not reinterpret missing type
evidence as Integer.

The durable distinction is:

```text
VerifiedDynamic
  = source deliberately declares an untyped/dynamic value and the exact
    parameter/local/carrier relation is sealed

Unknown
  = representation evidence is missing or was lost
```

Only the first may authorize a dynamic-wire Loop carrier and PHI. Raw
`MirType::Unknown`, `None`, or a failed lookup remains a typed failure.

## Why entry-I64 promotion is rejected

The existing callable result row for `skip_while/4` is:

```text
ExactI64 { required_i64_arguments = [1] }
```

This means that ordinal 1 must be exact I64 when a caller consumes the result
as exact I64. It does not declare a universal callee-entry type. The
counterexample is:

```text
id(x) { return x }
```

Its conditional result proof may require argument 0, but `x` is not thereby a
declared Integer parameter. `skip_while` also uses ordinal 2 (`end`) in the
comparison, while the result proof contains only ordinal 1. Result demand is
therefore neither the correct owner nor complete body-entry evidence.

Do not create a callable-entry Integer contract from this row. The existing
`ParameterEntryContract` remains valid for source-declared exact numeric
contracts; it is not a license to fabricate `declared_type_name = i64` for an
untyped parameter.

## Source authority

```text
exact untyped ParamDecl
+ resolver-issued formal BindingRef
+ exact local initializer relation (formal pos -> local i)
+ exact Loop carrier membership
+ verified body rebind relation
        -> VerifiedDynamicCallableParameterV1
        -> VerifiedDynamicLocalInitializationV1
        -> VerifiedDynamicLoopCarrierV1
```

The public physical input should be one move-only aggregate, for example:

```text
VerifiedDynamicLoopCarrierV1
  callable owner
  source parameter ordinal and BindingRef
  local/carrier BindingRef
  initializer source relation
  Loop source identity
  source-backed dynamic representation
```

Names and raw source ordinals are diagnostic only. The issuer consumes the
existing resolved callable source/ledger and produces an AST-free product.
It does not read Builder state, `ValueId`, `MirType`, result requirements, a
method name, or a route label as semantic authority.

For the first fixture the dynamic authority is needed for `pos -> i`. `end`
remains a separate dynamic operand at the comparison boundary; the design
must not silently classify it as exact I64. The first physical canary may
close only when the existing dynamic comparison/call emission path can consume
that operand without inventing an exact representation.

## Physical representation

Do not immediately add `MirType::Dynamic` as a second runtime type algebra.
The VM already executes values on the dynamic lane and existing MIR surfaces
carry `MirType::Unknown`. The first design should keep that wire encoding but
require a distinct authorization receipt:

```text
PreparedLoopCarrierRepresentationV1
  Exact(MirType)
  Dynamic(VerifiedDynamicLoopCarrierV1, wire = MirType::Unknown)
```

This keeps wire compatibility while preventing:

```text
missing type fact
  -> raw Unknown
  -> accidental dynamic acceptance
```

The sole `setup_function_params` owner continues to allocate/publish formal
`ValueId`s. It must additionally publish or hand off the verified dynamic
origin for an untyped parameter; local Copy propagation must preserve that
origin to the exact final carrier value. GenericLoop only verifies and
consumes the prepared representation. It does not infer dynamic meaning.

PHI materialization must accept Unknown wire type only when the same
source-backed dynamic receipt authorizes every incoming carrier relation.
Mixed exact/dynamic, missing origin, foreign owner, or partially covered
incoming rows reject. No fact refinement from raw Unknown is added.

## Backend boundary

The MIR interpreter is the first supported consumer because it already
executes dynamic values. Backends that require a concrete carrier type must
reject before Loop block allocation or module publication with a stable
capability error. They must not erase the receipt, choose Integer, retry a
legacy Loop route, or defer failure until code generation.

## Reachability is not this repair

`VerifiedSelectedNormalCallableSourceInventoryV1` contains every selected
non-Main method; it is not a reachability proof. The whole-source static call
inventory does not seal external, runtime/provider, bare-function, or every
opaque source ingress, and it records bounded observation unavailability.
Moreover, the VM can invoke a module function by symbol.

Therefore these shortcuts are forbidden:

```text
zero observed calls -> unreachable
skip imported method -> fixed
selected source inventory -> closed-world call graph
```

Reachability pruning would require its own artifact/root/visibility and
complete-ingress Decision. It is not part of this blocker.

## Task order

### S0 — source-backed dynamic parameter/local carrier

1. Define the neutral dynamic representation vocabulary and source-backed
   callable parameter receipt.
2. Co-seal exact formal BindingRef, local initializer BindingRef, Loop source,
   and carrier membership for the first direct static-method cohort.
3. Prove that arbitrary constructors, foreign owners/sites, typed parameters,
   missing initializer relations, and duplicate rows reject.
4. Keep the product disconnected from Builder and GenericLoop.

### P0 — function-entry and local propagation

1. Add one origin-preserving handoff at the existing callable entry.
2. Keep `setup_function_params` as the sole formal ValueId/type publisher.
3. Preserve verified dynamic origin through the existing local Copy terminal.
4. Reject missing/foreign/duplicate/consumed origin; never synthesize it from
   `MirType::Unknown`.

### L0 — GenericLoop/PHI canary

1. Extend the prepared carrier representation with the explicit Dynamic arm.
2. Authorize Unknown-wire PHI only from complete dynamic carrier coverage.
3. Prove exact and dynamic lanes remain distinct and mixed inputs reject.
4. Run the unmodified `skip_while/4` canary on the MIR interpreter.
5. Verify unsupported backends fail before physical Loop effects.

### R1 resume

Resume the parser expression-product fixture only after S0/P0/L0 are green.
Observe the next first failure; do not pre-open instance-call/Box-result work.

## Acceptance matrix

```text
positive:
  untyped formal pos -> local i -> Loop carrier has VerifiedDynamic origin
  local Copy preserves the exact origin
  dynamic-wire PHI is authorized by complete incoming coverage
  VM executes the existing dynamic comparison/update path

negative:
  id(x) result requirement does not issue dynamic carrier authority
  raw Unknown without source receipt rejects
  typed exact parameter is not relabeled Dynamic
  foreign formal/local/Loop BindingRef rejects
  missing or duplicate initializer relation rejects
  mixed exact/dynamic PHI inputs reject in the first cohort
  unsupported backend rejects pre-effect
  consumed/missing selected handoff never falls back
```

## Nonclaims

```text
general dynamic type inference
general Unknown acceptance
exact-I64 parameter inference
parameter-entry numeric contract expansion
reachability/dead-method elimination
instance/provider/external ABI
dynamic Box representation redesign
all Loop profiles or all PHIs
source grammar/annotation changes
retry/fallback
```

## Stop condition

S0 stays in design stop until the exact resolver-issued formal/local/Loop
relations and their one canonical issuer are named from current code. If that
relation cannot be issued without AST rescan after the resolver boundary,
Builder state, a name heuristic, or a raw Unknown check, stop as `NoSafeSlice`
and improve the source ledger first.
