---
Status: Active implementation task
Date: 2026-07-16
Decision: accepted
Baseline: 3aa458f550
Parent: hmi-s0-v0-r0-delta0-map-formal-mutation-task-2026-07-16.md
Scope: one disconnected typed scalar register and immutable snapshot owner
---

# HMI-S0-V0-R0 clean register storage task

## Decision

Consume the public:

```text
MAPFIELD-R0-DELTA0
selection = A-PRIME-AUTHORIZED
```

and implement HMI register storage from a clean tree.

```text
register/snapshot:
  owns one private typed MapBox field

static storage helper:
  ordinary untyped storage formal
  in-place mutation
  no MapBox return

caller:
  helper result assignment = 0
  storage replacement after birth = 0

runtime ownership operation:
  0
```

This is an operational source-shape contract. It does not activate
borrow/noescape syntax or claim a production borrowed ABI.

## Exact task order

These are checkpoints inside the existing `HMI-S0-V0-R0` semantic row:

```text
HMI-S0-V0-R0-S0
  -> HMI-S0-V0-R0-I0
  -> HMI-S0-V0-R0-P0
  -> HMI-S0-V0-R0-G0
```

`HMI-S0-V0-R0-S0` is closed. I0 reached a design consultation stop before any
I0 source landed. See:

```text
hmi-s0-v0-r0-typed-formal-consultation-question-2026-07-16.md
```

After G0, the distinct `HMI-S0-V0-S0` row may add execution state and a
session factory. R0 does not implement either.

## Old stash law

The retained stash label:

```text
wip/hmi-s0-v0-r0 register storage fails field mutation
```

is failure evidence only. It showed that direct local/field MapBox mutation
and direct scalar payload storage work, but its owner-roundtrip shape is not
A-prime:

```hako
put_payload(storage, value_id, payload) {
    storage.set("" + value_id, payload)
    return storage
}

me.register_map = put_payload(me.register_map, value_id, payload)
```

Forbidden:

```text
stash apply/pop/restore:
  0

copying stashed source as authority:
  0

static open() returning MapBox:
  0

put helper returning storage:
  0

field reassignment:
  0

raw public store/load API:
  0

shared generic initialize method:
  0
```

The stash remains until G0 as an audit artifact. Deletion is a separate
cleanup decision.

## Proof composition

Do not create a second HMI-specific MapBox MIR authority.

```text
MAPFIELD-R0-DELTA0:
  generic ordinary-formal mutation runtime/MIR law

hmi-t0-authority:
  exact HMI source/boundary law

v0_register_test:
  producer-backed HMI domain behavior
```

The composed proof requires:

```text
generic helper MIR:
  RuntimeDataBox / Union
  receiver root = param:0

HMI caller field:
  declared MapBox metadata
  caller remains owner

HMI source:
  MapBox return = 0
  mutator-result assignment = 0
  storage field replacement = 0

ownership:
  CopyOwned = 0
  DestroyOwned = 0
```

Do not add a new HMI register MIR checker, V0 guard, proof manifest family,
opcode inventory, schema, or transport row.

## Durable file layout

Add:

```text
tools/hako_shared/hmi/state/register_storage.hako
tools/hako_shared/hmi/state/register_result.hako
tools/hako_shared/hmi/state/register_file.hako
tools/hako_shared/hmi/state/register_snapshot.hako
tools/hako_shared/hmi/tests/v0_register_test.hako
```

Modify minimally:

```text
tools/hako_shared/hmi/state/README.md
tools/hako_shared/hmi/view/function_view.hako
tools/checks/lib/hmi_t0_authority.py
```

Target budgets:

```text
register_storage.hako <= 80
register_result.hako <= 140
register_file.hako <= 240
register_snapshot.hako <= 200
v0_register_test.hako <= 380
every source/check file < 800
```

## Storage representation

```text
key:
  "" + ValueId

MapBox value:
  exact i64 payload

kind authority:
  immutable VerifiedHmiFunctionView

register-owned kind map:
  0
```

Payload-only storage is selected because DELTA0 proves the exact scalar value
shape without introducing scalar Box identity/share semantics. Reads rebuild
an immutable `HmiScalarValueV1` from the sealed kind plus payload. NoValue is
not representable.

## Bounded view addition

Add:

```hako
VerifiedHmiFunctionView.has_value_type(value_id)
```

It reads the already sealed private `value_type_kinds` map. Call
`value_type_kind` only after membership succeeds.

```text
raw JsonNode access:
  0

seal/schema widening:
  0

second type authority:
  0
```

## Storage helper

`register_storage.hako` owns:

```hako
static box HmiScalarRegisterStorageV1 {
    contains(storage, value_id)
    put_proven(storage, value_id, payload)
    read_present(storage, value_id)
}
```

Exact mutation:

```hako
put_proven(storage, value_id, payload) {
    storage.set("" + value_id, payload)
    return
}
```

Laws:

```text
storage formal:
  ordinary untyped formal

put result:
  no-value

new MapBox / field access:
  0

read_present:
  called only after caller-owned contains proof
```

The helper does not own type admission, duplicate checks, definition order,
snapshots, or error construction.

## Result vocabulary

`register_result.hako` owns:

```text
accepted()
has_scalar()
scalar()
failure()

empty success
scalar success from proven kind/payload
failure
snapshot success
```

Use owner-specific setter names. `scalar()` reconstructs through
`HmiScalarValuesV1.from_proven_parts`. Raw MapBox/payload exposure is zero.

## Live register owner

Fields:

```hako
storage: MapBox
definition_order: ArrayBox
function_view
```

Birth assigns each field exactly once. Use:

```hako
bind_register_function_view(function_view)
```

Public API:

```text
has(value_id)
define(value_id, scalar_value)
read(value_id)
size()
snapshot()
```

Forbidden API:

```text
raw storage/order accessor
public store/load payload
overwrite/remove/clear
```

### Define order

Every reject precedes mutation:

```text
1. scalar is not null/void
2. function view is bound
3. has_value_type(value_id)
4. expected kind is i64 or i1
5. scalar kind matches
6. destination is undefined
7. put_proven(storage, value_id, payload)
8. postcondition has(value_id)
9. append definition_order
10. publish empty success
```

Errors:

```text
unknown/non-scalar/kind mismatch:
  invalid-scalar

duplicate:
  duplicate-register

failed define changes storage/order:
  0
```

Do not add runtime class-name introspection for tests.

### Read order

```text
1. has(value_id)
2. undefined-register if false
3. has_value_type(value_id)
4. exact kind lookup
5. read_present(storage, value_id)
6. scalar success
```

MapBox.get before has and undefined defaults are forbidden.

## Snapshot owner

Fields:

```hako
storage: MapBox
function_view
```

Snapshot birth creates an independent MapBox. The live register file is the
sole snapshot publication owner.

Unique internal methods:

```hako
bind_snapshot_function_view(function_view)
add_proven_snapshot_payload(value_id, payload)
```

Because source-private methods are not active, the guard requires exactly one
`add_proven_snapshot_payload` caller: `register_file.hako`.

Snapshot construction:

```text
create snapshot
bind sealed function view
iterate live definition_order
prove live has
read proven payload
put into snapshot storage
prove snapshot has
publish only after all rows succeed
```

Public snapshot API is only `has/read`. Existing snapshots never observe later
live definitions.

## Producer-backed fixture

`v0_register_test.hako` consumes:

```text
scalar_suite_v1.json
ownership_transport_v1.json
```

through existing strict ingress, whole-document seal, and bounded function
views. It never constructs a Verified view, uses a fake production type view,
reads raw JsonNode, scans opcodes, or executes an instruction.

Pass:

```text
i64 roundtrip
i1 payload 0 and 1 in fresh files
size after successful definitions
snapshot A/B then live C
snapshot retains A/B and lacks C
repeated independent snapshots
two register files remain isolated
failed define followed by fresh valid file succeeds
duplicate leaves original unchanged
```

Reject:

```text
undefined read -> undefined-register
duplicate -> duplicate-register
unknown ValueId -> invalid-scalar
i64 <- i1 -> invalid-scalar
i1 <- i64 -> invalid-scalar
handle destination <- scalar -> invalid-scalar
```

Null/void is tested only if an existing safe source check can express it.

## Checkpoints

### R0-S0

Add storage/result files and `has_value_type`. Update the state README.

```text
register owner:
  0

production callers:
  0
```

#### R0-S0 closeout

Added:

```text
state/register_storage.hako:
  16 lines

state/register_result.hako:
  78 lines

tests/v0_register_test.hako:
  53-line substrate fixture
```

Modified:

```text
view/function_view.hako:
  has_value_type(value_id)

state/README.md:
  A-prime storage boundary
```

Release/debug:

```text
[hmi/s0-v0-r0-s0] ok
[hmi/s0-v0-l0] ok
```

Retained:

```text
MAPFIELD-R0-DELTA0:
  A-PRIME-AUTHORIZED

hmi-t0-authority:
  green

register owner / production callers:
  0 / 0
```

### R0-I0

Add the live register file and producer-backed fixture. Activate typed
define/read, exact failures, and instance isolation. Snapshot may remain
unconnected.

#### I0 stop evidence

The clean WIP uses no MapBox return or field reassignment, but the first valid
define is not visible after the static mutation command.

```text
simple has() field_get:
  declared/result handle:MapBox

late define() field_get:
  declared type absent
  result Unknown

contains helper formal:
  handle:MapBox

put_proven helper formal:
  Unknown
```

The I0 WIP is stashed by label and is not implementation authority. Do not
resume I0 until the typed-formal consultation is closed.

### R0-P0

Add immutable snapshot storage and complete the fixture. Re-run
`MAPFIELD-R0-DELTA0`; no HMI-specific MIR authority is added.

### R0-G0

Extend only:

```text
tools/checks/lib/hmi_t0_authority.py
```

Guard:

```text
storage helper definitions = 1
MapBox return = 0
mutator-result assignment = 0
storage replacement after birth = 0
raw map/order accessors = 0
snapshot internal add definitions/callers = 1/1
approved add caller = register_file.hako
state raw JSON/opcode comparisons = 0
external/production selectors = 0
execution state/session constructors = 0
```

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/run_proof_app.sh --only MAPFIELD-R0-DELTA0

HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_scalar_value_test.hako
HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_scalar_value_test.hako
HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_register_test.hako
HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_register_test.hako

HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako
HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/p0_mutation_test.hako

bash tools/checks/run_row_guard.sh --only hmi-semantic-reference-inventory
bash tools/checks/run_row_guard.sh --only hmi-t0-authority
bash tools/checks/run_row_guard.sh --only json-native-parser-authority
bash tools/checks/proof_app_manifest_test_entry_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dev_gate.sh quick
```

If a cargo test overwrites the debug binary, rebuild it with `vm-reference`.
Do not switch routes.

## Closeout counters

```text
live/snapshot payload MapBoxes = 1 each
definition order owners = 1
register/snapshot kind maps = 0
raw map/order accessors = 0
MapBox-returning mutation helpers = 0
storage replacements after birth = 0
helper-result assignments = 0
undefined defaults / overwrites = 0
CopyOwned / DestroyOwned = 0
new storage/helper ReleaseStrong = 0
stash restores = 0
production/external HMI callers = 0
opcode handlers = 0
execution state/session owners = 0
fallback/retry/V0 conversion = 0
source/check files >= 800 = 0
```

## Stop conditions

Stop if implementation requires:

1. Stash apply/restore or copied stash authority.
2. MapBox return, helper-result assignment, or field replacement.
3. Raw map/order exposure or public snapshot mutation.
4. Scalar Box identity sharing/clone policy.
5. Ownership syntax/opcodes or runtime MapBox changes.
6. Runtime type-name/HMI-specific compiler special cases.
7. Raw JsonNode, opcode reads, or a second MIR product.
8. Execution session/handler work inside R0.
9. A new V0 guard/schema/inventory/transport row.
10. Production caller/backend/fallback/retry.
11. Definition-order publication before mutation postcondition.
12. A source/check file reaching 800 lines.

## Claims

May claim:

```text
one disconnected typed i64/i1 register owner
one independent immutable snapshot
producer-backed function-view kind authority
A-prime no-result mutation with caller ownership retained
zero production callers/opcode handlers/ownership/fallback
```

Must not claim:

```text
borrow/noescape ABI activation
general object ownership theorem
MIR instruction/CFG/PHI execution
execution state/session factory
BoxRef ownership execution
product VM/backend activation
Rust interpreter parity or termination
```

## Final law

> HMI register and snapshot storage are caller-owned typed MapBox fields.
> `put_proven` is a no-result mutation command. The sealed function view owns
> kind facts; private maps own only exact scalar payloads. Definition order is
> published after mutation is observed, snapshots own separate storage, and
> no raw map, ownership operation, opcode execution, production caller,
> fallback, or second semantic authority is introduced.
