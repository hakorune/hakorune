---
Status: Ready for implementation
Date: 2026-07-16
Decision: A-prime accepted
Baseline: 29ca7fee9e
Parent: hmi-s0-v0-r0-storage-helper-ownership-consultation-question-2026-07-16.md
Scope: generic ordinary-static-formal MapBox mutation visibility proof
---

# HMI-S0-V0-R0 DELTA0 MapBox-formal mutation task

## Decision lock

Three read-only worker audits accept A-prime:

```text
storage owner:
  caller local or user-box field

static helper formal:
  ordinary untyped formal in current grammar

helper behavior:
  mutate MapBox in place
  no MapBox return
  no field reassignment

runtime ownership operation:
  0
```

The next code-facing row is `R0-DELTA0-S0`.

Exact order:

```text
R0-DELTA0-S0
  -> R0-DELTA0-M0
  -> R0-DELTA0-V0
  -> R0-DELTA0-G0
```

If the selected token is `A-PRIME-AUTHORIZED`, HMI-S0-V0-R0 is rebuilt from
clean source. The old register stash remains evidence only and is never
restored as authority.

## Claim boundary

Current source grammar has no active `borrow` or `noescape` parameter
annotation.

DELTA0 may claim only:

```text
an ordinary static formal receives a MapBox
the helper does not let the MapBox escape
in-place mutation is visible from the caller's original owner
the helper returns no MapBox
the caller never reassigns its storage field
```

DELTA0 must not claim:

```text
verified borrowed ABI is production-active
ownership grammar is active
View/Shared/Move semantics are implemented
all object formals preserve identity
```

It is a source-shape proof aligned with the future A-prime ownership law.

## Durable artifacts

```text
apps/map-formal-borrowed-mutation-proof/
  README.md
  main.hako
  test.sh

tools/checks/lib/
  map_formal_borrowed_mutation_proof.py

tools/checks/manifests/proof_apps/
  compiler_map_field_owner.toml
```

Targets:

```text
main.hako:
  <= 260 lines

checker:
  <= 450 lines

every source/check file:
  < 800 lines
```

No new manifest include or shell guard is added.

Final manifest row:

```toml
[[proof_apps]]
id = "MAPFIELD-R0-DELTA0"
app = "apps/map-formal-borrowed-mutation-proof"
label = "generic borrowed MapBox-formal mutation proof"
profiles = ["pilot"]
row_kind = "validation"
validation_profile = "scalar-mir"
first_pattern = false
exe = "auto"
cmd = ["tools/checks/lib/map_formal_borrowed_mutation_proof.py", "."]
```

The app-local `test.sh` uses only:

```bash
exec bash tools/checks/lib/proof_app_test_entry.sh MAPFIELD-R0-DELTA0
```

## Exact source owners

Use one static command owner:

```hako
static box MapFormalMutationCommandV1 {
    put_literal(storage, value) {
        storage.set("2", value)
        return
    }

    put_id(storage, id, value) {
        local key = "" + id
        storage.set(key, value)
        return
    }

    contains_id(storage, id, expected) {
        local key = "" + id
        if storage.has(key) {
            if storage.get(key) == expected {
                return 1
            }
        }
        return 0
    }
}
```

Use one field owner:

```hako
box MapFormalFieldOwnerV1 {
    init { storage }

    birth() {
        me.storage = new MapBox()
    }

    command_literal(value) {
        MapFormalMutationCommandV1.put_literal(me.storage, value)
        return
    }

    command_id(id, value) {
        MapFormalMutationCommandV1.put_id(me.storage, id, value)
        return
    }
}
```

All method names receive a repository-global `map_formal_mutation_v1_`
prefix except the language constructor `birth`.

Forbidden source shapes:

```text
return storage
return me.storage
local result = helper.put(...)
me.storage = helper.put(...)
share / move / clone
raw MapBox accessor
result MapBox or ArrayBox ledger
```

## Exact fixture matrix

Every case uses fresh state unless repetition or isolation is the subject.

### Baselines

```text
B1 local_direct_baseline
  local MapBox direct set
  caller direct has/get
  expected 1

B2 field_direct_baseline
  owner direct field mutation
  separate owner direct observation
  expected 1
```

### Static-formal mutation

```text
D1 local_formal_literal_direct
  static put_literal(local_map, 10)
  caller direct has/get
  expected 1

D2 local_formal_dynamic_direct
  static put_id(local_map, 3, 10)
  caller direct has/get
  expected 1

D3 local_formal_dynamic_helper
  static put_id(local_map, 4, 10)
  static contains_id(local_map, 4, 10)
  expected 1

D4 field_formal_literal_direct
  owner command_literal(10)
  owner direct observation
  expected 1

D5 field_formal_dynamic_direct
  owner command_id(5, 10)
  owner direct observation
  expected 1

D6 field_formal_dynamic_helper
  owner command_id(6, 10)
  owner helper observation
  expected 1

D7 repeated_mutation
  same field owner and helper
  id 7 receives 10 then 20
  direct and helper observations both see 20
  expected 1

D8 instance_isolation
  owner A helper mutation id 8 = 10
  A sees 10
  owner B has id 8 = false
  expected 1
```

## Stable source output

The app prints observation only:

```text
map-formal-borrowed-mutation-proof
case.local_direct_baseline=0|1
case.field_direct_baseline=0|1
case.local_formal_literal_direct=0|1
case.local_formal_dynamic_direct=0|1
case.local_formal_dynamic_helper=0|1
case.field_formal_literal_direct=0|1
case.field_formal_dynamic_direct=0|1
case.field_formal_dynamic_helper=0|1
case.repeated_mutation=0|1
case.instance_isolation=0|1
selection=UNCLASSIFIED-S0
summary=observed
```

The app does not own classification.

## Exclusive classifier

The Python checker evaluates in this exact order:

```text
if !B1 or !B2:
  STOP-BASELINE0

else if !D1:
  STATIC-FORMAL-MUTATION0

else if D1 and (!D2 or !D3):
  STATIC-FORMAL-KEY-OR-OBSERVE0

else if !D4:
  FIELD-STATIC-FORMAL-MUTATION0

else if D4 and !D5:
  FIELD-STATIC-DYNAMIC0

else if D5 and !D6:
  FIELD-STATIC-OBSERVATION0

else if !D7:
  STATIC-FORMAL-REPEAT0

else if !D8:
  STATIC-FORMAL-ISOLATION0

else:
  A-PRIME-AUTHORIZED
```

Exactly one token is emitted.

## Normalized MIR contract

Separate helper evidence from caller evidence.

### Helper functions

For the untyped static formal, current MIR is expected to remain honest:

```text
receiver route:
  RuntimeDataBox

certainty:
  Union

storage root:
  param:0
```

Do not require `MapBox/Known` inside the generic helper.

Required helper assertions:

```text
put helpers:
  return no-value
  raw MapBox return = 0
  field_set(storage) = 0
  one RuntimeDataBox.set / Union
  receiver root = param:0
  argument roots = storage, key, value order

contains helper:
  one RuntimeDataBox.has / Union
  one RuntimeDataBox.get / Union on has-true branch
  receiver root = param:0
  key root = formal-derived key

helper CopyOwned = 0
helper DestroyOwned = 0
helper ReleaseStrong = 0
```

### Call sites

Required local call evidence:

```text
storage argument type:
  handle:MapBox

storage root:
  newbox:MapBox
```

Required field call evidence:

```text
one field_get(storage)
field_get result type:
  handle:MapBox

storage root:
  field:storage<param:0>
```

Required call laws:

```text
callee:
  exact Global canonical helper name

argument order:
  storage, key/id, value

direct caller observation:
  MapBox.has/get / Known

helper observation:
  storage argument root equals mutation-call storage root

storage field_set:
  birth exactly 1
  every other function 0

whole fixture CopyOwned:
  0

whole fixture DestroyOwned:
  0
```

Whole-fixture legacy `ReleaseStrong` may be nonzero for ordinary local cleanup.
The checker classifies its operand roots and rejects release of:

```text
field:storage<...>
helper storage formal
helper call result
```

Do not compare raw ValueId, block id, or instruction site numbers between
debug and release.

## Implementation order

### R0-DELTA0-S0

```text
generic source + README
runtime observation rows
manifest connection = 0
classifier = 0
```

### R0-DELTA0-M0

```text
debug/release VM observation
normalized MIR parity
no-return/no-reassign structural checks
manifest connection = 0
```

### R0-DELTA0-V0

```text
apply exclusive classifier
record immutable selected token in this card
compiler/HMI behavior delta = 0
```

### R0-DELTA0-G0

```text
add MAPFIELD-R0-DELTA0 manifest row
activate standard app-local test entry
add checks-index focused command
run closeout matrix
move current pointer by selected token
```

Do not land a half-connected manifest row or unregistered production proof
entry. S0 through V0 may use direct checker execution; G0 is the atomic public
proof registration.

## Validation order

```text
1. git status / current pointer
2. proof manifest list and 209-entry baseline guard
3. existing MAPFIELD-R0-STOP0 remains NONE-HMI-DELTA0
4. DELTA0 debug VM
5. DELTA0 release VM
6. normalized MIR checker
7. run_proof_app --only MAPFIELD-R0-DELTA0 at G0
8. neighboring exact-numeric helper field mutation
9. relevant generic/user-box route tests
10. unchanged HMI T0/inventory/json guards
11. diff-check / file-size / current pointer
12. quick 66/66
```

## Separation counters

Before HMI resumes:

```text
HMI source/check changes = 0
HMI authority/inventory changes = 0
V0-R0 stash restores = 0
MapBox runtime changes = 0
ownership grammar changes = 0
backend widening = 0
HMI production callers = 0
fallback/retry/env selector = 0
source/check files >= 800 lines = 0
```

## Stop conditions

Stop immediately if any of these is required:

1. Helper returns the MapBox or `storage`.
2. Caller receives a mutator result or reassigns the storage field.
3. Mutator result is used as mutation-visibility evidence.
4. `share`, move, clone, RC, or ownership opcode is introduced.
5. HMI register source or stash is imported/restored.
6. Direct field mutation is used as a fallback for a failed formal proof.
7. RuntimeDataBox/type-name special casing is added.
8. MapBox runtime semantics are changed.
9. Fallback, retry, environment selection, or backend widening is added.
10. Runtime green is described as production borrowed-ABI support.
11. Any source/check file reaches 800 lines.

## Result routing

```text
A-PRIME-AUTHORIZED:
  create a clean HMI-S0-V0-R0 implementation task

FIELD-*:
  create exactly one narrow field-to-static-formal compiler prerequisite

STATIC-FORMAL-*:
  return to generic compiler formal-object consultation

STOP-*:
  return to design consultation
```

The central law is:

> DELTA0 proves in-place mutation visibility through a non-escaping ordinary
> static formal. It does not implement or claim a borrowed ABI.

