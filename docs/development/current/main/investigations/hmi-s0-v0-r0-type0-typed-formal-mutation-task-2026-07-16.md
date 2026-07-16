---
Status: Active implementation task
Date: 2026-07-16
Decision: Candidate A accepted
Baseline: b7e0e189fd
Parent: hmi-s0-v0-r0-typed-formal-consultation-question-2026-07-16.md
Scope: one explicit MapBox formal -> no-result static mutation proof
---

# R0-TYPE0 typed MapBox formal mutation task

## Decision

Use:

```hako
put_proven(storage: MapBox, value_id, payload) {
    storage.set("" + value_id, payload)
    return
}
```

Authority split:

```text
value type:
  source formal `storage: MapBox`

callable representation:
  FunctionSignature.params[0] = handle:MapBox

helper parameter ValueId:
  handle:MapBox before body lowering

runtime argument identity:
  existing static Call transport

storage owner:
  caller local or field

ownership intent:
  no-return / no-publication source shape

verified borrow ABI:
  not present in this row
```

No ownership syntax is added.

```text
share / move / clone:
  0

MapBox return:
  0

storage field replacement:
  0
```

The central claim is narrow:

> An explicit source type on the callee formal seals the helper-side value
> representation without depending on caller field-type recovery.

## Exact order

```text
R0-TYPE0-S0
  -> R0-TYPE0-M0
  -> R0-TYPE0-V0
  -> R0-TYPE0-G0
  -> clean HMI-S0-V0-R0-I0
```

The next code-facing row is `R0-TYPE0-S0`.

Do not add a separate docs-only row. This card owns the decision lock.

## Existing source authority

The compiler already owns:

```text
ParamDecl:
  source name and declared type

set_current_function_declared_signature:
  callable declaration installation

project_declared_signature_representation:
  source type -> MIR representation

setup_function_params:
  formal ValueId type publication
```

Expected projection:

```text
source `MapBox`
  -> MirType::Box("MapBox")
  -> FunctionSignature.params[0]
  -> value_types[param0]
```

TYPE0 observes this existing path. It does not add another callable/type
authority.

## Non-authorities

```text
helper method name
storage field name
storage.set body shape
runtime class name
caller field declared type
HMI file path
MIR symbol parsing
```

The caller's late `field_get` may remain typed or Unknown. Authorization
depends on the callee's explicit formal and runtime identity transport, not on
repairing general declared-field propagation.

## Durable artifacts

```text
apps/map-typed-formal-mutation-proof/
  README.md
  main.hako
  storage_command.hako
  test.sh

tools/checks/lib/map_typed_formal_mutation_proof.py
```

G0 adds one row to:

```text
tools/checks/manifests/proof_apps/compiler_map_field_owner.toml
```

No new manifest family or row guard is allowed.

Target budgets:

```text
main.hako:
  <= 260 lines

storage_command.hako:
  <= 100 lines

checker:
  <= 520 lines

every source/check file:
  < 800 lines
```

## Two-file source law

`storage_command.hako` owns:

```hako
static box TypedMapMutationCommandV1 {
    put_proven(storage: MapBox, value_id, payload) {
        storage.set("" + value_id, payload)
        return
    }

    contains(storage: MapBox, value_id, expected) {
        local key = "" + value_id
        if storage.has(key) {
            if storage.get(key) == expected {
                return 1
            }
        }
        return 0
    }
}
```

`main.hako` imports it through the ordinary `using`/text-merge route.

```text
HMI names/imports/fixtures:
  0

runtime special case:
  0

direct helper source duplication:
  0
```

## Owner shape

Use one typed field owner:

```hako
box TypedMapFieldOwnerV1 {
    storage: MapBox

    birth() {
        me.storage = new MapBox()
    }
}
```

The fixture must include a complex owner method with:

```text
input validation
one or more early returns
late me.storage field_get
typed static no-result put
post-call observation
```

This exact shape reproduces the HMI I0 boundary without importing HMI.

## Exact runtime matrix

### P1 local typed put, direct observation

```text
local MapBox
typed static put
caller MapBox.has/get
expected 1
```

### P2 local typed put, helper observation

```text
local MapBox
typed static put
TypedMapMutationCommandV1.contains
expected 1
```

### P3 field typed put, direct observation

```text
field-held MapBox
typed static put
owner direct has/get
expected 1
```

### P4 field typed put, helper observation

```text
field-held MapBox
typed static put
typed static contains
expected 1
```

### P5 late field direct observation

```text
validation + early returns
late field_get
typed static put
owner direct observation
expected 1
```

### P6 late field helper observation

```text
same mutation shape as P5
typed helper observation
expected 1
```

### P7 repeated mutation

```text
same owner/key
10 then 20
direct and helper observations see 20
expected 1
```

### P8 instance isolation

```text
owner A receives mutation
owner B remains empty
expected 1
```

### P9 two-file annotation transport

```text
all typed helpers originate only in storage_command.hako
runtime pass proves using/text-merge route
expected 1
```

### P10 negative then fresh valid state

```text
absent-key observation returns 0
fresh owner then receives valid typed mutation
expected 1
```

The app prints observation only:

```text
map-typed-formal-mutation-proof
case.local_direct=0|1
case.local_helper=0|1
case.field_direct=0|1
case.field_helper=0|1
case.late_field_direct=0|1
case.late_field_helper=0|1
case.repeated=0|1
case.instance_isolation=0|1
case.two_file_transport=0|1
case.negative_then_fresh=0|1
selection=UNCLASSIFIED-S0
summary=observed
```

The app never owns classification.

## Source reject laws

The checker rejects:

```text
untyped storage formal
formal name other than storage
helper returning storage
helper returning a MapBox expression
caller binding put result
caller assigning put result to a field
storage field replacement after birth
share / move / clone
HMI imports or names
direct fallback mutation in late-field cases
```

`contains` may return an integer observation. Only the mutation command is
required to be no-result.

## M0 normalized MIR contract

Do not compare raw ValueIds, block ids, instruction indexes, or source sites.

### Declaration transport

For every typed storage helper:

```text
declared_param_decls[0].name:
  storage

declared_param_decls[0].declared_type_name:
  MapBox
```

The mutation helper is mandatory. Observer annotation may be admitted only if
the same exact law is green.

### Callable representation

```text
function parameter 0:
  handle:MapBox

metadata value_types[param0]:
  handle:MapBox
```

The checker records declaration transport and MIR parameter representation as
separate facts.

### Helper body

```text
set count:
  exactly 1

set receiver root:
  param:0

set route:
  MapBox / Known

source-visible MapBox result:
  0
```

The MIR call may have an internal destination for Void. Do not require
`Call.dst == None`.

### Caller

```text
callee:
  exact Global canonical helper

argument order:
  storage, value_id, payload

local argument root:
  newbox:MapBox

field argument root:
  field:storage<owner>

late field_get MIR type:
  recorded, not an authorization prerequisite
```

The caller must pass the original runtime MapBox value even when the late
field result metadata remains Unknown.

### Ownership

```text
CopyOwned:
  0

DestroyOwned:
  0

ReleaseStrong on:
  helper formal
  field storage
  helper result
  all 0

source ownership spelling:
  0
```

Ordinary unrelated cleanup is classified by operand root rather than rejected
globally.

## V0 exclusive classifier

Evaluate in this order and emit exactly one token.

```text
if source ParamDecl does not preserve storage: MapBox:
  TYPE-DECL-TRANSPORT-REQUIRED

else if FunctionSignature/parameter ValueId is not handle:MapBox:
  TYPE-PROJECTION-REQUIRED

else if helper set is not MapBox/Known rooted at param:0:
  TYPE-PROJECTION-REQUIRED

else if local cases fail:
  CALL-VALUE-TRANSPORT-REQUIRED

else if late field cases fail:
  FIELD-PROPAGATION-REQUIRED

else if ownership spelling/opcode/release boundary is nonzero:
  OWNERSHIP-WIDENING-DETECTED

else if any remaining runtime case fails:
  STOP-UNCLASSIFIED0

else:
  TYPED-FORMAL-AUTHORIZED
```

Only `TYPED-FORMAL-AUTHORIZED` permits HMI I0 to resume.

Candidate B opens only when:

```text
source ParamDecl = MapBox
parameter ValueId = MapBox
internal set = MapBox/Known
local cases pass
late field cases fail
```

An Unknown caller `field_get` alone is not sufficient to select B.

## Checkpoints

### R0-TYPE0-S0

Add the two-file generic fixture, README, and dormant app-local test entry.

```text
production compiler delta:
  0

HMI source delta:
  0

classifier:
  0

manifest connection:
  0
```

### R0-TYPE0-M0

Add:

```text
tools/checks/lib/map_typed_formal_mutation_proof.py
```

Prove:

```text
debug/release runtime equality
debug/release normalized MIR equality
source structural laws
typed declaration -> parameter -> MapBox/Known route
ownership/release boundary zero
```

Selection remains `UNCLASSIFIED-M0`.

### R0-TYPE0-V0

Apply the fixed exclusive classifier and record the immutable token in this
card.

```text
compiler behavior delta:
  0

HMI behavior delta:
  0
```

### R0-TYPE0-G0

Add:

```text
MAPFIELD-R0-TYPE0
```

to the existing:

```text
tools/checks/manifests/proof_apps/compiler_map_field_owner.toml
```

Suggested row:

```toml
[[proof_apps]]
id = "MAPFIELD-R0-TYPE0"
app = "apps/map-typed-formal-mutation-proof"
label = "generic typed MapBox formal mutation proof"
profiles = ["pilot"]
row_kind = "validation"
validation_profile = "scalar-mir"
first_pattern = false
exe = "auto"
cmd = ["tools/checks/lib/map_typed_formal_mutation_proof.py", "."]
```

Update the public check index and use the standard app-local proof entry. Do
not create a shell guard.

## Required pass fixtures

```text
P1-P10 runtime matrix = all 1
debug/release matrix parity
debug/release normalized MIR parity
typed ParamDecl exact name/type
parameter 0 exact handle:MapBox
internal set exact MapBox/Known param:0
two-file route only
caller field_get Unknown tolerated if runtime/typed callee law is green
repeated mutation
instance isolation
negative observation then fresh valid operation
```

## Required reject/mutation fixtures

```text
remove MapBox annotation
rename formal
change formal type
remove declared_param_decls row
change parameter ValueId type to Unknown
change internal set route to RuntimeDataBox/Union
return storage
bind put result
reassign field from put result
insert share/move/clone
insert CopyOwned/DestroyOwned
insert selected ReleaseStrong
add HMI-specific name/path
```

Every rejected proof publishes no authorization token.

## Validation order

```text
1. git status and current pointer
2. existing MAPFIELD-R0-STOP0
3. existing MAPFIELD-R0-DELTA0
4. TYPE0 debug VM
5. TYPE0 release VM
6. TYPE0 normalized MIR/source checker
7. TYPE0 mutation/reject fixtures
8. manifest/test-entry/pilot guards at G0
9. neighboring exact-numeric field mutation
10. HMI semantic inventory/T0/json isolation
11. diff/file-size/current pointer
12. quick 66/66
```

## Counters

```text
HMI I0 WIP restores = 0
owner-roundtrip WIP restores = 0
HMI source delta = 0
compiler production behavior delta = 0
MapBox runtime delta = 0

typed storage formal definitions = exact expected count
typed ParamDecl rows = exact expected count
signature/parameter MapBox facts = exact expected count
MapBox/Known set routes = exact expected count

method/runtime/HMI-name type inference = 0
MapBox return = 0
mutator-result binding = 0
field replacement after birth = 0

share/move/clone = 0
CopyOwned/DestroyOwned = 0
selected ReleaseStrong = 0

backend widening = 0
retry/fallback = 0
new manifest family = 0
source/check files >= 800 = 0
```

## Stop conditions

Stop if TYPE0 requires:

1. MapBox return or field reassignment.
2. share, move, clone, CopyOwned, or DestroyOwned.
3. Method-name, runtime-name, field-name, or HMI-path special casing.
4. Caller field-type recovery as the helper's type authority.
5. A second callable/type authority.
6. Direct HMI field mutation fallback.
7. Either HMI stash restore.
8. Legacy route, retry, fallback, or backend widening.
9. HMI-dependent fixtures.
10. A source/check file reaching 800 lines.

## Claims

If and only if V0 selects `TYPED-FORMAL-AUTHORIZED`, implementation may claim:

```text
explicit MapBox formal reaches helper MIR parameter before body lowering
helper parameter ValueId is handle:MapBox
helper set uses MapBox/Known without caller field-type inference
local and field-held runtime MapBox identity reaches the callee
two-file using/text merge preserves the formal annotation
validation/early-return late-field shape is green
no MapBox return, ownership syntax/opcode, fallback, or HMI special case
```

It must not claim:

```text
general declared-field propagation repair
all late field_get values typed
verified borrow/noescape ABI
runtime argument type checking
general object covariance/subtyping
all Box formals preserve identity
HMI opcode execution or production activation
backend widening or fallback
```

## Final law

> `storage: MapBox` is the value-type authority for the helper formal.
> Existing signature projection must publish it to parameter ValueId zero
> before helper-body lowering. The caller retains ownership, passes the
> original runtime value, binds no result, and performs no field replacement.
> TYPE0 proves this law generically across a two-file, validation-heavy,
> late-field shape before HMI I0 may resume.
