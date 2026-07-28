---
Status: closed execution
Date: 2026-07-28
Decision: FIELD-PROPERTY-GETTER-DESCENT0-D0
Pack: CALL-OBJECT0
Ceremony: T1
ReplacementCell: FIELD-PROPERTY-GETTER-DESCENT0-I0-R0
ProductionEdit: landed atomically with closeout
Parent:
  - docs/development/current/main/investigations/mirbuilder-next-edge-design-stop-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
NorthStar:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
---

# FIELD-PROPERTY-GETTER-DESCENT0-D0

## Decision boundary

The D0 accepted Candidate A with A1 parity-preserving terminal authority. The
ninth cell is now closed as `FIELD-PROPERTY-GETTER-DESCENT0-I0-R0`.

The next exact production authority break is:

```text
ASTNode::FieldAccess
-> selected RawAstChildLoweringPortV1
-> object expression descent through that port
-> materialized object ValueId
-> property getter selection
-> LegacyMethodCallArgumentsV1
-> raw catalog-helper child descent
```

The selected FieldAccess port is preserved while lowering the object, then
lost only when a resolved zero-argument property getter enters the standard
MethodCall executor.

The D0 question is:

> Can the selected FieldAccess port be lent to one exact zero-argument
> property-getter execution while object evaluation remains exactly once,
> getter lookup and route preparation remain exactly once, ordinary FieldGet
> fallback remains unchanged, no MethodCall source input is fabricated, and
> the old raw handler plus Legacy argument adapter are deleted atomically?

## Why this is the next responsibility

This is a live production edge, unlike the parked dead-facade and proof
consolidation cleanups.

```text
raw_expression_dispatch::ASTNode::FieldAccess
  -> fields::build_field_access_with_port_v1
  -> drive_legacy_expression_v1(builder, port, object)
  -> property_reads::try_lower_property_read(object_value, field)
  -> method_call_handlers::handle_standard_method_call(
       object_value,
       getter_name,
       &[],
     )
  -> LegacyMethodCallArgumentsV1
```

The object is already a `ValueId`; there is no faithful MethodCall receiver AST
to reconstruct. The missing capability is not a new source, identity,
provenance, route, or completion owner. It is one bounded projection of the
already-selected port across the property-getter boundary.

This closed as T1:

```text
new source/provenance owner    = 0
new identity issuer            = 0
new route selector             = 0
new publication/failure owner  = 0
property grammar delta         = 0
fallback / retry / reselection = 0
responsibility interface delta = bounded and landed
```

## Exact live census

```text
try_lower_property_read
  definition                         = 1
  non-test production callers        = 1

handle_standard_method_call
  definition                         = 1
  non-test production callers        = 1
  cfg(test) direct callers           = 1

LegacyMethodCallArgumentsV1
  definition / implementation        = 1
  production constructions           = 1

build_field_access
  definition                         = 1
  callers                            = 0

build_field_access_with_port_v1
  raw/default production caller      = 1

fake property MethodCall input       = 0
fallback / retry / route reselection = 0
```

## Preserved semantic owners

The accepted implementation must preserve:

```text
FieldAccess source partition
record-specialized field observation
object expression evaluation
property getter registry and name resolution
standard MethodCall route preparation
InlineRecord / InlineSetter helper execution
WeakLoad / Upgrade behavior
ordinary no-getter FieldGet fallback
standard terminal emission
receiver ValueId identity
failure and Builder reuse policy
```

It must not claim general MethodCall, me/static dispatch, property registry,
terminal-header, located, Stage-B, runtime, or backend authority.

## Existing useful capabilities

The selected port already supports:

```text
MethodCallDescentPortV1::lower_catalog_helper_child
```

The prepared standard executor already consumes:

```text
&mut dyn MethodCallArgumentDescentV1
```

for InlineRecord and InlineSetter. The gap is that the existing port-aware
standard handler accepts a concrete `AssociatedMethodCallArgumentsV1`, which
requires a real `MethodCallInput`. `FieldAccess` has no such input.

These are invalid shortcuts:

```text
construct a dummy RawLegacyMethodCallInputV1
reconstruct the already-evaluated receiver as AST
re-descend the receiver
use call-site argument roles or located ledger
```

## Candidate A — exact zero-argument port loan

Accepted and implemented shape:

1. Replace the sole `LegacyMethodCallArgumentsV1` authority with one exact
   zero-argument property-call adapter that borrows the selected FieldAccess
   port.
2. Generalize the existing standard execution seam only enough to accept both
   the real associated MethodCall adapter and the exact property adapter.
3. Keep standard route preparation and helper execution single-owned.
4. Let the property adapter provide:

```text
lower_all
  -> empty values, with no Builder descent

lower_index
  -> stable fail-fast invariant violation

lower_catalog_helper_child
  -> selected port's existing catalog-child loan
```

5. Thread the selected port through an exact property-read entry.
6. Delete the old raw handler and Legacy adapter atomically.

The adapter must not contain or synthesize:

```text
MethodCall AST or MethodCallInput
receiver or argument source
call-site child roles
located source or ledger
new header authority
```

## Terminal/header authority must remain explicit

The current property terminal deliberately uses raw completion with no header
lookup. A full port terminal under `RawInvocationChildPortV1` can observe the
collector-backed function header.

That may change:

```text
signature validation
type/origin annotation
diagnostic timing or text
MIR metadata
```

The D0 fixed the choice explicitly:

```text
A1 parity-preserving
  selected port is loaned only for catalog-helper child descent
  current lookup=None property terminal remains unchanged
  decision = accepted

A2 full port continuity
  selected port also owns property terminal completion
  accept only after focused proof shows no unintended diagnostic,
  type, origin, or MIR change
  decision = rejected for this cell
```

Do not silently bundle A2 into the descent cutover. If header visibility
changes behavior, stop and create a separate terminal-authority consultation.

## Rejected candidate B

Splitting a second property-only prepared executor could avoid an adapter, but
would duplicate standard route preparation, helper completion, WeakLoad /
Upgrade completion, and Unified terminal policy. Reject B unless Candidate A
is impossible without widening unrelated standard MethodCall policy.

## Atomic delete set

The atomic implementation deleted:

```text
property_reads::try_lower_property_read
  replaced by an exact with-port entry
method_call_handlers::handle_standard_method_call
calls::method_call_descent::LegacyMethodCallArgumentsV1
the direct facade-only property route test
fields::build_field_access
  only after final zero-caller census
```

It must retain:

```text
fields::build_field_access_with_port_v1
handle_standard_method_call_with_descent semantic ordering
standard prepare / execute owners
ordinary FieldGet fallback
property getter registry and resolution
selected MethodCall and FieldAccess source partitions
```

## Required evidence

Use existing source/test/check files only.

```text
real ASTNode::FieldAccess production ingress with registered getter
materialized receiver forwarded exactly once
receiver source re-descent                            = 0
getter lookup / standard preflight                   = exactly 1
zero property arguments                              = preserved
catalog helper child uses selected port              = exactly 1
raw helper-child re-entry                            = 0
final helper or standard terminal                    = exactly 1
ordinary no-getter FieldGet                          = unchanged
WeakLoad / Upgrade                                   = unchanged
failure stops later effects                          = preserved
same Builder reuse                                   = green
fake MethodCall source input                         = 0
fallback / retry / route reselection                 = 0
```

Existing evidence to reuse:

```text
src/tests/mir_unified_members_property_read.rs
src/mir/builder/member_route_descent_tests.rs
tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_route0.py
tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py
tools/checks/current_receiver_declared_field_proof.py
```

The direct property handler fixture must migrate to the real FieldAccess
ingress; no compatibility helper or new parity family may be added.

## Structural boundary

```text
current source files / LOC = 952 / 182430
source ceiling             = 952 / 182452
current test files / LOC   = 139 / 40820
test ceiling               = 139 / 40826
```

Closeout measurement:

```text
new source/test/check files       = 0
source files / LOC                = 952 / 182452
test files / LOC                  = 139 / 40809
source LOC delta                  = +22
test LOC delta                    = -11
five-cell production Rust rolling = -218
all modified source/check files   < 800 lines
new per-cell guard                = 0
```

The old raw handler and adapter contain roughly 59–61 gross removable source
lines. LOC is a result boundary, not the reason to select this responsibility.

## Hard stop

No hard stop fired. The retained stop conditions are:

```text
MethodCall AST/input fabrication is required
materialized receiver must be re-descended
zero-argument route demands lower_index or argument AST
port terminal changes diagnostics, type/origin, or MIR unexpectedly
new helper grammar, identity, publication, or failure authority is required
standard me/static/environment/reserved route policy must change
route preparation or helper completion becomes duplicated
old and replacement property adapters must coexist
ordinary associated MethodCall behavior or ordering changes
located/Stage-B production activation is required
fallback / retry / route reselection is required
new file or structural ratchet breach is required
```

## Parked work

Do not mix:

```text
RAW-BODY-FACADE-RETIRE0
DESCENT-PROOF-CONSOLIDATION0
non-Program root fallback
CANONICAL-DEFAULT-COMPILER-INGRESS0-D0
Function state / Control / lifecycle work
Stage-B / Ownership / selfhost / runtime / backend
```

The default compiler ingress debt remains part of the final north-star
contract: ordinary `--backend vm` still enters Legacy today. It is not safe to
cut over until the canonical typed ingress accepts the complete normal source
family without rejection-to-Legacy fallback.

## Closeout evidence

```text
Candidate A / A1 / T1                         = accepted
real registered property ingress             = 3 / 3 green
zero-argument property completion fixture    = 1 / 1 green
WeakLoad / Upgrade route fixture              = green
standard terminal failure / Builder reuse    = green
helper-body failure / Builder reuse           = green
M0 ROUTE0 helper                              = green
M0 V0 property-focused assertions             = green
shared replacement guard                      = green
current receiver declared-field proof         = green
old property symbols                          = 0
MethodCall source fabrication                 = 0
fallback / retry / reselection                = 0
```

The full historical M0 V0 parent still reaches its pre-existing unrelated
Stage-B F7 capture-capability drift before the property assertions. This cell
does not repair or claim Stage-B authority; the property-specific remainder is
green when that unrelated precheck is held constant.

Closeout:

```text
ninth manifest row     = exactly one closed
production source edit = landed
current execution row  = MIRBUILDER-NEXT-EDGE-DESIGN-STOP
```
