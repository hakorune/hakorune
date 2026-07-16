---
Status: Active implementation task
Date: 2026-07-16
Decision: Candidate B′ accepted
Baseline: 8c970a64b0
Parent: hmi-s0-v0-r0-array-field-propagation-consultation-question-2026-07-16.md
Scope: current instance receiver -> Copy-only -> explicit declared field
---

# R0-DECLFIELD0 current-receiver declared-field task

## Decision

Admit one exact compiler shape:

```text
field base:
  exact current user-box owner type

provenance:
  zero-or-more ordinary Copy
  -> exact implicit receiver parameter 0

field truth:
  existing explicit user-box field declaration

publication:
  existing FieldGet.declared_type
  existing destination value_types
```

No persistent provenance table, new field registry, runtime inference, or
downstream-plan backfeed is added.

## Exact order

```text
R0-DECLFIELD0-S0
  -> R0-DECLFIELD0-M0
  -> COPY-ROOT-DECLFIELD-AUTHORIZED only
     R0-DECLFIELD0-I0
  -> R0-DECLFIELD0-G0
  -> clean HMI-S0-V0-R0-I0 rewrite
```

The next code-facing row is `R0-DECLFIELD0-S0`.

## M0 hard gate

The consultation observed a receiver carried through Copy/PHI-like control
materialization, while the accepted first admission is Copy-only.

M0 must follow the actual selected field base:

```text
seed
  -> zero-or-more Copy
  -> exact current receiver parameter
```

If traversal encounters any:

```text
Phi
Select
CopyOwned
Call
FieldGet
NewBox
LocalContractWrite
foreign parameter
missing definition
cycle
```

the current row does not widen. In particular:

```text
Copy* -> Phi -> current receiver
  = PHI-ROOT-DESIGN-REQUIRED
  = I0 forbidden
```

This remains true even when every Phi input appears to share the same receiver
root.

## Current authority

### Receiver publication owner

Reuse the identity published by instance-method parameter setup:

```text
declared_param_decls[0].implicit_receiver:
  true

current function params[0]:
  exact receiver ValueId

MirValueKind:
  Parameter(0)

function signature params[0]:
  Box(current owner)

type_ctx.value_types[receiver]:
  Box(current owner)

value_origin_newbox[receiver]:
  current owner
```

These facts must agree before the new lookup is usable.

Do not use:

```text
variable_map["me"]
function_param_names
current_static_box
current_enclosing_box_name
function/symbol string splitting
```

`variable_map["me"]` may point at a merge carrier. It is not canonical receiver
identity.

### Copy definition lookup

Reuse:

```text
ssa::analysis::find_value_def
```

only as low-level definition lookup.

Do not reuse `same_block_copy_root` as the authority. It is same-block-only
and stops before a parameter root.

### Field declaration owner

The existing normalized user-box field declaration registry remains the sole
field truth.

```text
owner + field
  -> declared source type
  -> existing source-type-to-MIR representation
```

No ArrayBox, MapBox, method, or field name is compiled into the recovery rule.

## New neutral product

Recommended physical owner:

```text
src/mir/builder/current_receiver_field_facts.rs
```

Suggested vocabulary:

```rust
struct CurrentReceiverIdentityV1 {
    parameter: ValueId,
    owner_box: String,
}

enum CurrentReceiverRootBarrierV1 {
    Phi,
    Select,
    CopyOwned,
    ForeignParameter,
    OtherInstruction,
    MissingDefinition,
    Cycle,
}
```

Required bounded views:

```rust
fn current_instance_receiver_identity(
    builder: &MirBuilder,
) -> Option<CurrentReceiverIdentityV1>;

fn classify_copy_only_current_receiver_root(
    builder: &MirBuilder,
    seed: ValueId,
    receiver: &CurrentReceiverIdentityV1,
) -> Result<bool, CurrentReceiverRootBarrierV1>;
```

The helper is:

```text
read-only
use-site-local
bounded by visited ValueIds
non-persistent
```

It does not mutate `value_origin_newbox` or publish another owner map.

## Field lookup connection

The existing route stays first:

```text
value_origin_newbox[object_value]
  -> declared_field_type_name(owner, field)
```

Only when that route is absent may the fallback run:

```text
1. seal current receiver identity
2. require seed type == Box(receiver.owner)
3. require Copy-only root == receiver parameter
4. read existing declared field from receiver.owner
5. return existing MirType
```

Consumers remain:

```text
existing field-result allocation
existing FieldGet.declared_type
existing destination value_types
existing generic method route planner
```

Forbidden:

```text
origin backfill
router mutation
new MIR instruction
new metadata row
second ValueId -> type map
second ValueId -> owner map
```

## Durable artifacts

```text
apps/current-receiver-declared-field-proof/
  README.md
  main.hako
  test.sh

tools/checks/lib/current_receiver_declared_field_proof.py

src/mir/builder/current_receiver_field_facts.rs
```

G0 extends only:

```text
tools/checks/manifests/proof_apps/compiler_map_field_owner.toml
docs/tools/check-scripts-index.md
```

No dedicated shell guard or new manifest family is allowed.

Target budgets:

```text
main.hako:
  <= 320 lines

checker:
  <= 560 lines

current_receiver_field_facts.rs:
  <= 240 lines

every source/check file:
  < 800 lines
```

## R0-DECLFIELD0-S0

Production behavior delta:

```text
0
```

Add one HMI-independent app with:

```hako
box DeclaredFieldOwnerV1 {
    items: ArrayBox
    map: MapBox

    birth() {
        me.items = new ArrayBox()
        me.map = new MapBox()
    }
}
```

Required cases:

```text
A1 direct ArrayBox push/length
A2 one fallthrough validation then push/length
A3 nested validation + early returns then push/length
A4 explicit local Copy alias of me
A5 rejected validation changes state = 0
A6 repeated ArrayBox mutation
A7 two owner instances remain isolated
M1 MapBox declared-field has/set regression
C1 typed ArrayBox helper comparison control
N1 untyped-field negative control
N2 ordinary same-box typed parameter negative control
```

Each runtime case is independently selectable. The checker invokes cases
separately so one failing late case does not hide the remaining inventory.

The app prints observations only:

```text
selection=UNCLASSIFIED-S0
```

It does not classify compiler authority.

S0 source laws:

```text
HMI names/imports:
  0

field replacement after birth:
  0

ownership syntax:
  0

runtime special cases:
  0
```

The typed ArrayBox helper exists only as a disconnected comparison control.
The eventual HMI implementation must not call it.

## R0-DECLFIELD0-M0

Production behavior delta:

```text
0
```

Add the normalized checker and collect debug/release runtime plus MIR.

Do not compare:

```text
raw ValueIds
block ids
instruction indexes
source line offsets
```

Normalize receiver provenance as:

```text
current_receiver
Copy(current_receiver)
Copy*(current_receiver)
Phi(...)
foreign_parameter
other_instruction
missing
cycle
```

Required pre-change evidence:

```text
direct ArrayBox:
  declared_type = Box(ArrayBox)
  dst type = Box(ArrayBox)
  ArrayBox / Known

late ArrayBox:
  declared_type absent
  dst type Unknown
  RuntimeDataBox / Union

late base:
  existing type = Box(DeclaredFieldOwnerV1)

root:
  measured from actual MIR

registry:
  items = ArrayBox
  map = MapBox
```

Exclusive classifier order:

```text
if direct baseline is broken:
  BASELINE-RUNTIME-BROKEN

else if implicit receiver identity cannot be co-validated:
  CURRENT-RECEIVER-IDENTITY-MISSING

else if exact owner/field declarations are missing:
  DECLARED-FIELD-REGISTRY-MISSING

else if selected seed type is not Box(current owner):
  BASE-TYPE-MISMATCH

else if Copy traversal encounters Phi:
  PHI-ROOT-DESIGN-REQUIRED

else if traversal reaches exact receiver parameter:
  COPY-ROOT-DECLFIELD-AUTHORIZED

else:
  CURRENT-RECEIVER-IDENTITY-MISSING
```

Only `COPY-ROOT-DECLFIELD-AUTHORIZED` permits I0.

M0 must also prove:

```text
typed helper control:
  green

MapBox regression:
  green

untyped/foreign controls:
  not recovered

CopyOwned / DestroyOwned / selected ReleaseStrong:
  0
```

## R0-DECLFIELD0-I0

Allowed only after M0 publishes:

```text
COPY-ROOT-DECLFIELD-AUTHORIZED
```

Production behavior delta:

```text
one new declared-field provenance shape
```

Implementation order:

```text
1. add neutral receiver identity view
2. add bounded Copy-only root classifier
3. test every barrier independently
4. connect one fallback consumer in declared_field_type_for_value
5. reuse existing FieldGet/result type publication
6. re-run normalized proof
```

Required post-change evidence:

```text
selected ArrayBox FieldGet:
  declared_type = Box(ArrayBox)
  dst type = Box(ArrayBox)

selected calls:
  ArrayBox.push / Known
  ArrayBox.length / Known

selected RuntimeDataBox / Union:
  0

MapBox regression:
  Known

untyped/foreign controls:
  unchanged

new Copy/Phi/MIR instructions:
  0
```

I0 must not change global Copy propagation, PHI metadata propagation, runtime
field storage, or the generic method route planner.

## R0-DECLFIELD0-G0

Production behavior delta:

```text
0
```

Register:

```text
MAPFIELD-R0-DECLFIELD0
```

in the existing MapFieldOwner proof family.

G0 validation order:

```text
1. current pointer
2. MAPFIELD-R0-STOP0
3. MAPFIELD-R0-DELTA0
4. MAPFIELD-R0-TYPE0
5. MAPFIELD-R0-DECLFIELD0
6. manifest/test-entry health
7. neighboring exact-numeric field mutation
8. HMI semantic inventory/T0 isolation
9. json_native authority
10. source/check file sizes
11. quick gate
```

After G0 only, rewrite clean HMI R0-I0 from the clean tree. Do not restore or
copy:

```text
wip/hmi-s0-v0-r0-i0 ArrayBox late field receiver is Void
```

## Required pass fixtures

```text
zero-copy receiver root
one Copy
multiple Copies
cross-block Copy-only chain
ArrayBox push
ArrayBox length
repeated ArrayBox mutation
MapBox has/set
validation rejection preserves state
instance isolation
debug/release normalized parity
```

## Required barrier/no-recovery fixtures

```text
Phi
Select
CopyOwned
Call result
FieldGet root
NewBox root
LocalContractWrite
ordinary explicit parameter
foreign receiver parameter
static function param0
implicit_receiver absent
signature/type/origin owner mismatch
missing field
untyped field
field declared on another box
copy cycle
seed type Unknown
seed type another Box
```

These controls need not become source compile errors. They must receive no new
declared-field recovery.

## Counters

```text
current-receiver identity definitions = 1
Copy-only classifiers = 1
declared-field fallback consumers = 1

accepted root kinds:
  receiver parameter
  Copy

accepted Phi / Select / CopyOwned / foreign params:
  0

current_static_box reads:
  0

function/symbol parsing:
  0

method/field/HMI-name conditions:
  0

runtime type reads:
  0

TypedObjectPlan reads:
  0

new type/owner/field maps:
  0

origin backfill:
  0

selected RuntimeDataBox/Union:
  0 after I0

new MIR instructions:
  0

field replacement:
  0

CopyOwned / DestroyOwned / selected ReleaseStrong:
  0

HMI source delta:
  0 through DECLFIELD0

fallback / retry / legacy probe:
  0

new manifest family:
  0

source/check files >= 800:
  0
```

## Implementation may claim

After I0/G0 only:

```text
an explicit declared field type is recovered for the exact current receiver
or its Copy-only alias

receiver identity comes from the existing implicit receiver parameter facts

field truth comes from the existing user-box declaration registry

existing FieldGet.declared_type and value_types publish the recovered type

ArrayBox and MapBox use existing Known method routes

unsupported provenance shapes preserve prior behavior
```

## Implementation must not claim

```text
general declared-field propagation
arbitrary or same-root PHI support
Select provenance
dynamic receiver inference
all aliases of me
all receiver expressions typed
symbol/name identity
TypedObjectPlan as Builder authority
borrow/noescape ABI
ownership/receiver ABI widening
HMI register completion before clean post-G0 rewrite
```

## Stop conditions

Stop if:

1. The selected late root contains Phi.
2. Receiver identity needs `current_static_box` or function-name parsing.
3. `variable_map["me"]` must become canonical root.
4. Runtime/method/field/HMI-name inference is required.
5. Downstream MIR products must feed Builder.
6. `value_origin_newbox` must be backfilled on aliases.
7. A second type/owner/field authority is required.
8. PHI inputs must be analyzed in the same row.
9. AST rewrite or production typed-helper detour is required.
10. Ownership/backend/fallback/retry widening is required.
11. More than one provenance shape must be admitted.
12. A source/check file reaches 800 lines.

## Final law

> B′ admits only an exact current instance receiver or a zero-or-more ordinary
> Copy chain ending at that receiver. M0 must prove the actual selected source
> has this shape; Phi is an unconditional design stop. The implementation may
> recover only an existing explicit field declaration and publish it through
> existing FieldGet/result type channels. No name inference, runtime tag,
> downstream backfeed, second authority, origin backfill, typed-helper detour,
> ownership widening, fallback, retry, or stash restoration is permitted.
