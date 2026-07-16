---
Status: Ready for implementation
Date: 2026-07-16
Decision: one opcode-neutral disconnected scalar execution-state authority
Baseline: 777c63a621
Parent: hmi-s0-strict-reader-interpreter-task-2026-07-16.md
Previous: hmi-s0-t0-s0-worker-audited-execution-task-2026-07-16.md
Scope: HMI-S0-V0 only; production callers, opcode handlers, and execution remain zero
---

# HMI-S0-V0 disconnected scalar state task

## Outcome

Three read-only workers audited the closed T0 carrier from state ownership,
`.hako` API constraints, and proof/guard perspectives.

The selected V0 shape is:

```text
VerifiedHmiDocumentView
  -> exact function selection
  -> exact scalar argument validation
  -> HmiScalarExecutionStateV1
       document anchor
       selected function view
       current block id
       current instruction index
       predecessor = Entry | Block(id)
       typed scalar register file
       immutable block-entry register snapshot
       Running | ReturnedValue | ReturnedNoValue | Failed
       explicit harness-only step budget
```

V0 is not an interpreter. It owns the state that the later I0 handlers mutate,
but it does not know which opcode is being executed.

```text
production behavior delta:
  0

HMI production callers:
  0

opcode handlers:
  0

Rust fallback / route retry:
  0
```

## Most important laws

### One program-counter owner

V0 owns both:

```text
current block id
current instruction index
```

I0 must not introduce a second instruction cursor.

### One bounded-view lifetime anchor

Every state retains:

```text
the original VerifiedHmiDocumentView
the selected VerifiedHmiFunctionView
```

The function view alone must not outlive the document anchor that retains the
sealed JsonNode tree.

### Parallel-PHI preparation

On every block transition, V0 snapshots the live scalar register file before
the first instruction in the target block executes.

```text
ordinary register read:
  live register file

future PHI source read:
  immutable block-entry snapshot only
```

This prevents an earlier PHI destination write from changing the source read
of a later PHI in the same prefix. V0 does not match PHI predecessors or inspect
PHI instructions; it only owns the snapshot that I0 will consume.

### NoValue is not a register value

```text
register value:
  i64 | i1

ReturnedNoValue:
  outcome only

undefined register:
  typed failure
  never 0 / false / no-value
```

### Exact step-budget boundary

The step budget is a harness resource boundary, not a language termination
claim.

```text
max_steps:
  any integer >= 0

budget N:
  prepare_step succeeds exactly N times
  attempt N+1 fails before instruction or register effects

environment override:
  0
```

## Authority split

### V0 owns

```text
document/function anchor retention
exact function selection
scalar parameter ingress cardinality and kind validation
typed scalar register storage
immutable block-entry register snapshots
current block and instruction index
Entry/Block predecessor identity
terminal/failure state transitions
first-failure retention
harness-only step count and limit
```

### V0 does not own

```text
JSON or MIR schema admission
raw JsonNode fields
opcode vocabulary or dispatch
instruction field interpretation
CFG successor meaning
PHI predecessor matching
arithmetic semantics
Branch truth semantics
Return instruction semantics
call stack or function calls
heap / Box / ownership execution
backend capability
Rust VMValue or MirModule
Rust oracle parity
fallback / retry / V0 conversion
```

## Physical structure

Do not create one large `state.hako`.

```text
tools/hako_shared/hmi/state/
  README.md
    layer boundary and forbidden imports

  scalar_value.hako
    immutable tagged i64/i1 value

  error.hako
    typed V0 state failures

  outcome.hako
    Running / ReturnedValue / ReturnedNoValue / Failed

  predecessor.hako
    explicit Entry / Block(id)

  register_file.hako
    live typed registers plus immutable snapshot product

  step_budget.hako
    exact N/N+1 harness resource law

  execution_state.hako
    cursor, transition, terminal state, and state mutation boundary

  session_factory.hako
    exact function/argument validation and sole state publication

tools/hako_shared/hmi/tests/
  v0_scalar_value_test.hako
  v0_register_test.hako
  v0_state_test.hako
```

Targets:

```text
README:
  <= 100 lines

scalar/error/outcome/predecessor/budget:
  <= 160 lines each

register file:
  <= 300 lines

execution state:
  <= 340 lines

session factory:
  <= 240 lines

fixtures:
  <= 380 lines each

hard source/check limit:
  < 800 lines
```

Every state file directly imports every type it names. Test import order must
not supply accidental dependencies.

## Product contracts

### HmiScalarValueV1

```text
kind:
  i64 | i1

payload:
  signed i64

i1 admission:
  0 | 1 only

mutation after construction:
  0
```

Use one tagged value box. Do not store kind and payload in separate maps.
Do not use raw Hako truthiness or a raw Bool as the i1 carrier.

Recommended facade:

```hako
HmiScalarValuesV1.i64(value)
HmiScalarValuesV1.i1(payload)
```

### HmiScalarRegisterFileV1

```text
storage:
  one private MapBox

key:
  string projection of ValueId

value:
  immutable HmiScalarValueV1
```

Required operations:

```text
has(value_id)
define(value_id, scalar_value, function_view)
read(value_id)
snapshot()
```

Rules:

```text
MapBox.get:
  only after same-owner has()

undefined read:
  typed failure

duplicate definition:
  typed failure

write kind:
  exact match with function_view.value_type_kind(value_id)

admitted kinds:
  i64 | i1

register map exposure:
  0
```

`HmiScalarRegisterSnapshotV1` owns a separate private map of immutable scalar
value references. It supports read only and is replaced atomically on block
transition.

### HmiPredecessorV1

```text
Entry
Block(id)
```

Do not encode Entry as null, `-1`, or a missing field.

### HmiExecutionOutcomeV1

```text
Running
ReturnedValue(HmiScalarValueV1)
ReturnedNoValue
Failed(HmiStateErrorV1)
```

Value and error payloads are mutually exclusive. Returned and Failed are
terminal. A second terminal publication is rejected and the first failure is
retained.

### HmiScalarSessionFactoryV1

Recommended entry:

```hako
HmiScalarSessionFactoryV1.open(
    document_view,
    function_name,
    argument_values,
    max_steps
)
```

The factory is the sole state-construction/publication owner.

Admission order:

```text
1. require non-null verified document view
2. exact function-name lookup
3. require max_steps >= 0
4. validate argument cardinality
5. validate every argument scalar kind
6. seed every parameter register
7. locate the exact CFG-owned entry block view
8. construct the initial empty block-entry snapshot
9. publish one Running state
```

Any failure before step 9 publishes no state.

The factory must not scan opcode names or reject a function because a later I0
handler is absent.

### HmiScalarExecutionStateV1

Required observations:

```text
is_running()
is_returned()
is_returned_no_value()
is_failed()
current_block_id()
current_instruction_index()
current_instruction_view()
predecessor()
step_count()
read_register(value_id)
read_block_entry_register(value_id)
```

Required state transitions:

```text
prepare_step()
define_register(value_id, scalar_value)
advance_instruction()
transfer_to(target_block_id)
finish_return(value)
finish_return_no_value()
fail(error)
```

Transition law:

```text
transfer_to(target):
  require Running
  require target block exists in selected function view
  snapshot live registers
  predecessor = Block(old current)
  current block = target
  instruction index = 0
```

Advance law:

```text
advance_instruction:
  require Running
  require next instruction exists in current bounded block view
  instruction index += 1

cursor overrun:
  typed failure
  no implicit block fallthrough
```

V0 does not call `instruction.op()` and does not compare opcode strings.

## Implementation checkpoints

These are checkpoints inside one V0 semantic row. Do not add a current-state
row for each checkpoint.

### V0-L0 — passive vocabulary

Add:

```text
state/README.md
state/scalar_value.hako
state/error.hako
state/outcome.hako
state/predecessor.hako
state/step_budget.hako
```

```text
production callers:
  0

state/session callers:
  0

accepted MIR/opcode delta:
  0
```

### V0-R0 — typed register and snapshot owner

Add:

```text
state/register_file.hako
```

Prove exact typed definitions, undefined reads, duplicate definition rejection,
and immutable block-entry snapshot behavior.

```text
document seal/view changes:
  0

opcode/CFG reads:
  0
```

### V0-S0 — disconnected session state

Add:

```text
state/execution_state.hako
state/session_factory.hako
```

Connect only focused HMI tests. Production/external selectors remain zero.

### V0-G0 — guard and closeout

Extend the existing reusable:

```text
hmi-t0-authority
```

Do not add an `hmi-v0-authority` guard.

Close the V0 API, fixture matrix, production-caller zero, import boundary,
and file-size checks, then hand off to HMI-S0-I0.

Milestone:

```text
commit:
  feat(hmi): seal disconnected scalar machine state

push:
  required before HMI-S0-I0
```

## Required fixtures

All integration fixtures consume the checked-in producer document:

```text
tools/hako_shared/hmi/tests/fixtures/scalar_suite_v1.json
```

### Pass

```text
exact selection:
  hmi_scalar_cfg
  entry block = 7
  current instruction = 0
  predecessor = Entry
  Running
  steps = 0

multi-function selection:
  exact requested name
  never first-function inference

parameter ingress:
  parameter_cross_block
  exact one i64 argument
  exact transported parameter ValueId

scalar:
  i64 roundtrip
  i1 payload 0 and 1 roundtrip

register snapshot:
  snapshot old A/B
  define later destinations
  snapshot reads remain old A/B

block transition:
  7 -> 8 -> 10
  predecessor after second transition = Block(8)
  current instruction resets to 0

cursor:
  exact within-block advance

outcomes:
  ReturnedValue(i64)
  ReturnedValue(i1)
  ReturnedNoValue
  Failed(error)

budget:
  0 permits zero prepared steps
  1 permits one prepared step
  N permits exactly N prepared steps

isolation:
  two states over one document do not share registers/outcome/cursor
  failed state followed by a fresh valid state succeeds
```

### Reject

```text
null or missing document
unknown function name
wrong argument cardinality
i64 parameter receives i1
i1 parameter receives i64
i1 payload outside 0/1
negative step budget
undefined register read
duplicate register definition
foreign or unknown ValueId definition
non-scalar value type definition
missing target block
cursor beyond block
define/advance/transfer/prepare after Return
define/advance/transfer/prepare after Failed
double return
return after failure
second failure replacing the first
```

Arbitrary foreign user-box runtime type introspection is not a V0 claim.
Accepted scalar values enter only through the HMI scalar-value facade.

## Guard contract

Extend `tools/checks/lib/hmi_t0_authority.py` to prove:

```text
V0 state/session construction owner:
  1

program-counter owner:
  1

test harness selectors:
  exact approved V0 fixture paths only

production/external V0 selectors:
  0

state raw JsonNode access:
  0

state object_get / array_get / root_for_seal:
  0

state MirModule / VMValue references:
  0

state opcode string comparisons:
  0

handler/runtime registry:
  0

fallback/retry/V0 conversion:
  0

BoxRef/CopyOwned/DestroyOwned execution:
  0

source/check files >= 800 lines:
  0
```

Keep `hmi-semantic-reference-inventory` unchanged. V0 introduces no opcode,
transport, caller, or fixture-family SSOT.

## Validation order

```bash
HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_scalar_value_test.hako
HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_register_test.hako
HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_state_test.hako

HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_scalar_value_test.hako
HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_register_test.hako
HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/v0_state_test.hako

HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako
HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/p0_mutation_test.hako

bash tools/checks/run_row_guard.sh --only hmi-semantic-reference-inventory
bash tools/checks/run_row_guard.sh --only hmi-t0-authority
bash tools/checks/run_row_guard.sh --only json-native-parser-authority
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

If the normal cargo test command overwrites `target/debug/hakorune` without the
reference VM feature, rebuild the focused debug binary explicitly before the
debug `.hako` runs. Do not weaken the fixture or switch routes.

## Closeout counters

```text
machine/state authority owners = 1
program-counter owners = 1
document anchors per state = 1
selected function views per state = 1
live register maps per state = 1
block-entry snapshots per state = 1
test harness state selectors = approved fixtures only
production/external selectors = 0

raw register map accessors = 0
undefined-register default values = 0
register overwrites = 0
NoValue register carriers = 0

second MIR/instruction/CFG products = 0
raw JsonNode accesses in state = 0
VMValue / MirModule references = 0
opcode comparisons/dispatch in state = 0
handler files / arithmetic execution = 0
call stack / heap / ownership execution = 0
fallback / retry / V0 translation = 0
source/check files >= 800 lines = 0
```

## Stop conditions

Stop and return to design review if V0 requires:

1. Reading raw JsonNode fields or calling `object_get` / `array_get`.
2. Comparing opcode names or dispatching an instruction.
3. Copying the sealed document into a second function/block/instruction graph.
4. Importing Rust `VMValue`, `MirModule`, Arc, BoxRef, or ownership runtime.
5. Completing an undefined register with 0, false, null, or no-value.
6. Encoding Entry predecessor as null, `-1`, or missing state.
7. Reading future PHI sources from the live register file instead of the
   block-entry snapshot.
8. Storing kind and payload in separately mutable maps.
9. Treating the step bound as termination proof or language semantics.
10. Scanning handler availability or opcode capability before I0.
11. Reusing legacy substring/compact/v0 interpreter state.
12. Adding a production caller, backend marker, fallback, or route retry.
13. Widening the T0 seal, MIR schema, value classes, or ownership execution.
14. Adding a second dedicated V0 guard or opcode inventory.
15. Letting any source/check file reach 800 lines.

## Implementation may claim

```text
one disconnected opcode-neutral scalar execution-state authority
one typed i64/i1 register file
one immutable block-entry snapshot for later parallel PHI execution
one current block/instruction cursor and explicit predecessor owner
exact terminal/failure and harness step-budget laws
state publication only after exact function/parameter validation
zero production callers, opcode handlers, ownership execution, and fallback
```

## Implementation must not claim

```text
MIR instruction execution
Const/Copy/BinOp/Jump/Branch/Phi/Return support
arithmetic semantics
CFG or PHI semantic validation
Rust interpreter parity
Call/MethodCall
BoxRef/CopyOwned/DestroyOwned execution
product VM replacement
backend activation
termination
general scalar or MIR JSON support
```

## Final decision lock

> HMI-S0-V0 adds exactly one disconnected opcode-neutral scalar execution
> state. It retains the sealed document and selected function bounded views,
> owns one tagged i64/i1 register file, one immutable block-entry register
> snapshot, one current block/instruction cursor, one explicit Entry/Block
> predecessor, terminal/failure outcomes, and one harness-only step budget.
> V0 never reads raw JsonNode fields, compares opcode names, decodes a second
> MIR graph, imports VMValue/MirModule, or executes arithmetic, control flow,
> ownership, or calls. The internal checkpoint order is
> `V0-L0 -> V0-R0 -> V0-S0 -> V0-G0`; only the existing
> `hmi-t0-authority` guard is extended. Production callers and fallback remain
> zero, and HMI-S0-I0 cannot begin until release/debug focused fixtures,
> retained T0 proofs, existing guards, quick 66/66, and the source/check
> 800-line boundary are green.

## V0-L0 closeout

V0-L0 is closed with one passive, disconnected vocabulary:

```text
state/error.hako
  typed [freeze:contract][hmi/state/*] failures

state/scalar_value.hako
  tagged i64/i1 value
  i1 payload 0/1 only
  NoValue unrepresentable

state/outcome.hako
  Running / ReturnedValue / ReturnedNoValue / Failed

state/predecessor.hako
  explicit Entry / Block(id)

state/step_budget.hako
  max_steps >= 0
  N successes / N+1 pre-effect failure
```

Result mutators use owner-specific names instead of repeated generic
`set_success` / `set_failure` spellings. This keeps each result box as its own
initializer authority under the current `.hako` resolver.

Validation:

```text
release/debug focused execution:
  [hmi/s0-v0-l0] ok

MIR verification:
  green

hmi / inventory / json authority guards:
  green

current pointer / diff:
  green

quick:
  66/66

largest V0-L0 source:
  123 lines

production callers / opcode handlers / fallback:
  0
```

V0-R0 then reached a storage BoxShape boundary. Its failing prototype is
preserved as:

```text
stash:
  wip/hmi-s0-v0-r0 register storage fails field mutation
```

Do not restore it as authority. Worker audit and a dedicated consultation must
select the register-storage owner before R0 resumes.
