---
Status: HMI-S0-T0-L0 closed; worker-audited S0 execution packet locked
Date: 2026-07-16
Decision: harden the T0 prototype into one producer-backed bounded seal
Parent: hmi-s0-strict-reader-interpreter-task-2026-07-16.md
Scope: disconnected HMI-S0; production execution callers remain zero
---

# HMI-S0-T0 whole-document seal hardening task

## Decision

The first T0 prototype is a useful schema sketch, but it is not a sufficient
whole-document authority. T0 will land as a short BoxShape series:

```text
HMI-S0-T0-L0
  -> HMI-S0-T0-S0
  -> HMI-S0-T0-P0
```

```text
L0:
  typed error and bounded-view vocabulary

S0:
  one root/instruction/CFG/value/PHI/ownership seal pipeline

P0:
  Rust-emitter-backed fixtures, mutation matrix, guard, closeout
```

All three rows are disconnected from product execution. No state machine,
opcode handler, Rust fallback, V0 adapter, or product caller is activated.

## Worker re-audit and recommended execution order

Three independent read-only workers re-audited the live S0 worktree after the
first seal pipeline reached a green focused execution. The result is:

```text
semantic direction:
  accepted

current WIP:
  useful and executable
  not yet landable

first remaining owner:
  BoxShape cleanup inside the existing S0 row
```

The following are implementation checkpoints, not new durable semantic rows.
The public task order remains `L0 -> S0 -> P0`.

```text
S0-BS0
  split overloaded instruction/context responsibilities

S0-PUB0
  make whole-document finish precede every Verified view construction

S0-VALUE0
  seal the first exact cross-block value-use admission law

S0-I1
  close the disconnected whole-document seal and push one milestone

P0-EMIT0
  replace handwritten positive authority with Rust-emitter-backed fixtures

P0-MUT0
  complete the mutation matrix and zero-publication failures

P0-G0
  close producer/inventory/constructor guards and push one milestone
```

Do not create separate current-state rows for these checkpoints. They are the
exact implementation order inside the active S0/P0 card.

### Why the current WIP is not landable

The focused S0 program currently reaches all semantic checks and exits green,
but the following authority defects remain:

```text
unconditional source debug prints:
  3

temporary stale constructor probe:
  present and failing

Verified function-view construction:
  occurs after each function seal
  before whole-document correspondence finish

constructor guard:
  contradicts the current nested constructor locations

instruction_shape.hako:
  646 lines
  owns DTOs, opcode field law, block scan, PHI placement,
  terminators, definitions, uses, and ownership sites

cross-block non-PHI use:
  global definition existence is checked
  dominance is not proved

producer drift guards still missing:
  opcode first subset
  21 opaque root arrays
```

Passing the focused test does not override these structural blockers.

## S0-BS0 — BoxShape prerequisite

Behavior and accepted T0 grammar delta: zero.

Recommended physical owners:

```text
seal/results.hako
  document/function seal result products

seal/function_context.hako
  typed HmiFunctionSealContextV1
  no MapBox string-key context authority

seal/instruction_facts.hako
  terminator summary
  value definition/use facts
  ownership instruction site facts

seal/instruction_contract.hako
  accepted opcode vocabulary
  exact required/allowed field and field-kind law

seal/instruction_inventory.hako
  block scan
  PHI-prefix law
  terminator-final law
  fact aggregation
```

`instruction_shape.hako` may remain as a thin facade during the split, but it
must stop owning all five responsibilities. No accepted opcode, field, CFG,
value, or ownership behavior changes in this checkpoint.

Targets:

```text
document coordinator:
  <= 180 lines

each instruction/context owner:
  <= 350 lines

hard source/check boundary:
  < 800 lines
```

Do not add another generic MapBox context or another handwritten opcode list.

## S0-PUB0 — one post-finish view publisher

Verified view construction is publication for this card. Therefore no
`VerifiedHmi*View` object may be constructed until every function and every
whole-document correspondence check has succeeded.

Add one physical constructor owner:

```text
view/publication.hako
  VerifiedHmiDocumentView construction
  VerifiedHmiFunctionView construction
  VerifiedHmiBlockView construction
  VerifiedHmiInstructionView construction
```

The coordinator first retains only sealed facts/products:

```text
strict root
sealed root envelope
sealed function envelopes
instruction inventories
CFG facts
value facts
scalar/PHI proof
ownership transport proof
whole-document correspondence finish
```

Only then may it call the publisher once.

View classes may own their storage and expose exact read-only accessors, but
they must not construct nested Verified views themselves. Replace nested
constructor helpers with exact attachment methods:

```text
document.add_function_view(...)
function.add_block_view(...)
block.add_instruction_view(...)
```

The no-argument `birth()` plus explicit `initialize(...)` pattern is retained
for these views. It avoids high-arity constructor instability and lets each
published view own its MapBox/ArrayBox storage. Each view module must directly
import the view types it names; it must not depend on import order from the
coordinator or tests.

Remove:

```text
tools/hako_shared/hmi/tests/view_constructor_probe.hako
all [hmi/s0-debug] prints
all progress-only success prints in the S0 test
```

The authority guard must enforce:

```text
Verified view constructor source files:
  view/publication.hako only

constructor occurrence:
  one textual site per Verified view type

direct constructor calls from tests/other modules:
  0

failed document seal:
  Verified view constructions = 0
```

## S0-VALUE0 — first cross-block use law

The first T0 profile does not introduce a dominator authority.

Seal this conservative law:

```text
ordinary non-PHI use:
  parameter definition
  OR same-block earlier instruction definition

ordinary non-PHI use of another block's instruction result:
  reject

PHI incoming:
  parameter definition
  OR value defined in the named predecessor block before its terminator

PHI incoming from an unrelated/foreign block:
  reject
```

This is intentionally narrower than general SSA dominance. A future dominator
product may widen the profile in a separate row. S0 must not claim general
cross-block defined-before-use.

Required focused fixtures:

```text
same-block prior definition:
  pass

same-block use before definition:
  reject

parameter used in a later block:
  pass

non-PHI use of predecessor-local result:
  reject

PHI value defined in its named predecessor:
  pass

PHI value defined in a different block:
  reject
```

## S0-I1 — disconnected seal closeout

The live handwritten document remains only a focused implementation fixture in
S0. It is not producer parity authority.

S0 closeout requires:

```text
root -> function envelope -> instruction inventory
  -> CFG -> value inventory -> scalar/PHI -> ownership
  -> whole-document finish -> one view publisher

unconditional source print:
  0

temporary probes:
  0

Verified view construction before whole finish:
  0

production/external HMI callers:
  0

execution state/handler files:
  0

fallback/retry/V0 conversion:
  0

source/check files >= 800 lines:
  0
```

Additional focused fixtures:

```text
two functions where the second fails:
  no document/function/block/instruction view is constructed

entry block not equal to first/lowest block:
  pass

unreachable block with reachable=false:
  pass

transported reachable drift:
  reject

branch then == else:
  successor set is deduplicated exactly as producer evidence requires
```

Milestone:

```text
commit:
  feat(hmi): seal bounded MIR JSON V1 profile

push:
  required before P0 work
```

## P0 execution packet

### P0-EMIT0 — producer-backed fixtures

Create checked-in fixtures only through the current Rust emitter:

```text
Rust builds minimal MirModule
  -> build_mir_json_root
  -> exact serialized bytes
  -> checked-in fixture equality test
  -> .hako strict ingress reads the same bytes
```

Required producer-backed documents:

```text
scalar CFG:
  Const i64
  Const Bool
  Branch
  Jump
  Phi
  Add
  Copy
  Return

scalar supplements:
  Sub / Mul / Div / Mod
  Bool return
  no-value return
  multiple functions

ownership transport:
  borrowed WidgetBox parameter
  CopyOwned to owned WidgetBox
  DestroyOwned
  no-value return
```

The handwritten `root()` and `plan_names()` helpers must stop being positive
authority. Mutation tests operate on checked-in producer bytes.

### P0-MUT0 — exact rejection matrix

Complete the mutation matrix already listed below and assert for every failure:

```text
result.accepted():
  false

result.document():
  null

Verified view constructions:
  0

execution/register/heap effects:
  0

Rust fallback and route retry:
  0
```

Add explicit cases for:

```text
foreign/missing/extra function and block
duplicate value definition
parameter/dst collision
missing/extra/stale value type
cross-block use outside S0-VALUE0
ownership value_kinds cardinality drift
ownership operation duplicate/site/operand drift
owner outside signed-i64 portable range
```

### P0-G0 — drift guards and closeout

The reusable `hmi-t0-authority` guard owns:

```text
strict parser selector count
no compatibility/fallback/retry/conversion
constructor owner and caller zeros
root_for_seal consumer count
opcode first subset == HmiSemanticReferenceInventoryV1 projection
21 opaque root arrays == current Rust emitter projection
handwritten positive fixture authority = 0
Rust emitter fixture freshness owner = 1
production/external callers = 0
runtime state/handler files = 0
unconditional prints/probes = 0
source/check files < 800
```

Do not add a second shell guard. Extend the existing manifest-backed row guard.

Milestone:

```text
commit:
  test(hmi): prove producer-backed T0 seal parity

push:
  required before selecting HMI-S0-V0
```

## Exact validation order

Use cache-disabled focused runs so import changes cannot reuse stale EXE
artifacts:

```bash
HAKO_EMIT_EXE_CACHE=0 tools/bin/hako --backend mir --verify \
  tools/hako_shared/hmi/tests/l0_contract_test.hako

HAKO_EMIT_EXE_CACHE=0 tools/bin/hako --backend mir --verify \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako

HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/l0_contract_test.hako

HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako

HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/l0_contract_test.hako

HAKO_EMIT_EXE_CACHE=0 target/debug/hakorune --backend mir \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako

bash tools/checks/run_row_guard.sh --only hmi-t0-authority
bash tools/checks/run_row_guard.sh --only json-native-parser-authority
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

P0 additionally runs the exact Rust emitter fixture equality tests and the
complete mutation harness before the final quick gate.

## Evidence that forced the refinement

Three independent read-only audits compared the parked prototype with the
post-CUT0 parser, current Rust MIR JSON V1 emitter, and repository authority
rules.

The prototype got these important facts right:

```text
root schema/capability spelling
21 current root plan-array names
function/block/CFG carrier shape
Const i64 and Bool reconstruction carrier
Copy/BinOp/Jump/Branch/Phi/Return payload spelling
Phi incoming order = [value_id, predecessor_block_id]
exact-none edge witness spelling
```

The following gaps are blocking:

```text
raw/generic view access can bypass the seal
CFG reparses terminator fields before instruction shape admission
value definitions/uses/value_types have no exact inventory
CopyOwned source kind incorrectly requires Owned
handle box_type equality is not checked
CFG reachable is not co-validated
handwritten JSON is the only positive fixture
stable T0 error family is not the card-selected family
root opaque surfaces have no exact-empty/opaque policy
verified-view constructor sites are unguarded
```

These are BoxShape defects. They must not be repaired with by-name special
cases, tolerant coercion, retry, or narrower claims unsupported by the task
card.

## Preserved ingress decision

The sole carrier remains:

```text
serialized Rust-produced MirJsonExportDocument
  -> JsonParser.parse_with_policy(
       text,
       StrictJsonPolicyV1.exact_i64_document()
     )
  -> one JsonNode tree
  -> HMI whole-document seal
  -> bounded views over the same tree
```

Forbidden ingress:

```text
JsonParser.parse compatibility entry
tools/hako_shared tolerant JSON parser
MIR JSON v0
V1-to-v0 adapter
compact vm_hako payload
raw MirModule
AST / Program JSON
substring scanner
Rust fallback
route retry
```

## Authority pipeline

The order is fixed:

```text
1. strict JSON parse completes
2. root envelope and function/CFG name bijection
3. function/block envelope and exact instruction shape
4. one terminator summary per block
5. CFG topology, entry, successor, predecessor, reachability
6. exact value definition/use/value-type inventory
7. scalar and PHI profile verification
8. Ownership SSA transport bijection
9. whole-document correspondence finish
10. bounded document view publication
```

No view, execution state, register, or heap object is published before step 9
finishes.

## Physical structure

```text
tools/hako_shared/hmi/
  README.md
  strict_ingress.hako
  document_seal.hako

  view/
    document_view.hako
    function_view.hako
    block_view.hako
    instruction_view.hako

  seal/
    error.hako
    object_contract.hako
    root.hako
    function_envelope.hako
    instruction_shape.hako
    cfg.hako
    value_inventory.hako
    scalar_profile.hako
    phi.hako
    ownership.hako

  tests/
    document_seal_test.hako
    fixtures/
      scalar_cfg_v1.json
      ownership_transport_v1.json
```

Files may be combined only when the resulting owner still has one obvious
responsibility and a plausible growth path below 800 lines.

```text
coordinator/view/error target:
  <= 150 lines

root/instruction/value/CFG/profile target:
  <= 350 lines

source/check hard limit:
  < 800 lines
```

## HMI-S0-T0-L0 — passive contracts and bounded views

Production behavior delta: zero.

Status: closed.

Add:

```text
one typed HMI ingress/seal error product
one-way strict parser ingress
exact bounded view vocabulary
repository authority guard skeleton
```

### Error law

Stable external families:

```text
[freeze:contract][hmi/mir_json_v1/document]
[freeze:contract][hmi/mir_json_v1/cfg]
[freeze:contract][hmi/mir_json_v1/value_type]
[freeze:contract][hmi/mir_json_v1/ownership]
```

Typed error fields:

```text
family
kind
path
expected
actual
detail
```

Rendered text is a projection. Parser MapBox errors may be wrapped by HMI, but
HMI must not inspect the iterative parser engine or create a second JSON-error
authority.

### Bounded view law

Forbidden view methods:

```text
raw_root()
raw_node()
raw_cfg_node()
field(name)
metadata(name)
generic object_get passthrough
```

Views expose only exact admitted fields needed by later HMI handlers:

```text
document:
  function_count / function_name / function_view

function:
  name / entry_block / parameter IDs / block IDs / value type facts

block:
  id / instruction_count / instruction

instruction:
  op plus exact operand/result accessors for the admitted opcode family
```

The views reference the sealed JsonNode tree. They do not decode a second MIR
instruction enum or second CFG.

### L0 may claim

```text
one typed failure vocabulary
one bounded read-only view vocabulary
one strict ingress spelling
zero execution/state/handler callers
```

### L0 must not claim

```text
whole-document admission
CFG/value/PHI/ownership correctness
Rust-emitter fixture parity
opcode execution
```

### L0 closeout

```text
strict JSON ingress selectors:
  1

compatibility parser / fallback / retry:
  0

stable HMI error families:
  document / cfg / value_type / ownership

published bounded views:
  exact accessors only
  raw/generic JsonNode accessors = 0

VerifiedHmi view constructor calls:
  0

production/external HMI callers:
  0

focused MIR verify:
  green

release/debug Rust MIR interpreter:
  [hmi/s0-t0-l0] ok

authority guards:
  hmi-t0-authority green
  json-native-parser-authority green

quick gate:
  66/66

largest L0 source/check file:
  below 100 lines
```

## HMI-S0-T0-S0 — sealed whole-document products

Production behavior delta: zero.

Status: next code-facing row.

### Instruction shape authority

`instruction_shape.hako` is the only owner of:

```text
accepted opcode set
allowed/required fields per opcode
ID/scalar/container field kinds
PHI-prefix placement
terminator-final placement
one terminator summary per block
```

Accepted T0 opcode vocabulary:

```text
const
copy
copy_owned
destroy_owned
binop
jump
branch
phi
ret
```

CFG consumes the sealed terminator summary. It must not read raw
`target`/`then`/`else` fields and decide instruction shape again.

### Exact value inventory

`value_inventory.hako` owns:

```text
parameter definitions
instruction dst definitions
exactly-once definition law
all operand uses
defined-before-use policy selected for the T0 carrier
parameter/dst collision rejection
exact value_types key correspondence
no stale/extra/missing value type entry
```

Admitted value types:

```text
i64
i1
exact handle row for passive ownership transport
```

Rejected value types:

```text
f64
void value definitions
string
Unknown/null type
unrecognized object type row
```

Exact handle row:

```json
{
  "kind": "handle",
  "box_type": "non-empty exact string"
}
```

### CFG law

CFG owns:

```text
function/CFG name bijection
block/CFG-block identity bijection
CFG-owned entry block
successor set equality with terminator summaries
predecessor inventory
reachability computed from entry
exact equality with transported reachable flags
```

Declaration or JSON array order is not entry authority.

### Scalar/PHI law

```text
Const i64:
  payload type i64
  metadata type i64

Const Bool:
  payload type i64
  payload value 0 or 1
  metadata type i1

Copy:
  exact matching i64 or i1

BinOp:
  + - * / %
  exact i64 operands/result
  optional dst_type, when present, exactly i64

Branch:
  exact i1 condition

Phi:
  remains in block prefix
  exact [value, predecessor] pairs
  each actual predecessor exactly once
  input and destination types equal

Return:
  exact i64, i1, or no-value
```

Dynamic truthiness, Float, String operations, void Const, and Unknown types
remain rejected.

### Ownership transport law

T0 validates transport only. It does not execute BoxRef operations.

```text
CopyOwned dst:
  ownership kind exactly owned

CopyOwned src:
  ownership kind borrowed or owned
  none rejected

CopyOwned handle type:
  exact source/destination box_type equality

DestroyOwned value:
  ownership kind exactly owned
  exact admitted handle row
```

Ownership instruction sites and `ownership_ssa_v1.operations` form an exact
bijection. Producer/backend/provider/schema spelling is validated exactly.

The emitter transports owner as `u64`, while strict JSON admits signed i64.
The first portable T0 profile therefore admits owner IDs only in
`0..i64::MAX`.

### Root opaque-surface law

The root envelope requires:

```text
schema_version
capabilities
metadata
functions
cfg
```

The current 21 producer plan-array surfaces are known opaque rows. The first
scalar profile requires each to be present and exactly empty. HMI does not
expose them through bounded views.

Unknown root fields reject. A guard ties the known list to the current Rust
emitter list so producer drift cannot pass silently.

## HMI-S0-T0-P0 — producer-backed proof and closeout

Production behavior delta: zero.

### Fixture authority

Handwritten concatenated JSON is not the positive authority.

```text
1. Rust test constructs a minimal MirModule directly.
2. build_mir_json_root serializes it.
3. Rust test compares exact output with the checked-in fixture.
4. .hako T0 test reads the checked-in bytes.
5. strict ingress seals the document and exercises bounded views.
```

Do not use ordinary `.hako -> --emit-mir-json` as the first positive fixture.
That route adds prelude/operator functions outside the T0 profile.

Do not use historical `*_v1.mir.json` fixtures. Many lack current CFG, attrs,
metadata, value types, or edge witnesses.

### Scalar fixture

```text
function:
  hmi_scalar_cfg

entry:
  block 7

block 7:
  v0 = Const i64 1
  v1 = Const Bool true
  Branch v1 -> block 8 / block 9

block 8:
  v2 = Const i64 2
  Jump block 10

block 9:
  v3 = Const i64 3
  Jump block 10

block 10:
  v4 = Phi [(v2, block8), (v3, block9)]
  v5 = BinOp Add v4, v0
  v6 = Copy v5
  Return v6
```

Additional emitter-backed fixtures cover the remaining four i64 BinOps, Bool
return, no-value return, and multi-function bijection.

### Ownership fixture

```text
v0 parameter:
  handle WidgetBox
  ownership borrowed

v1 = CopyOwned v0:
  handle WidgetBox
  ownership owned

DestroyOwned v1
Return no-value
```

Execution admission remains zero.

### Mutation matrix

At minimum:

```text
root/schema:
  unknown/missing root field
  non-empty opaque plan array
  capability/feature drift

function/CFG:
  missing/extra/duplicate function
  duplicate block
  foreign entry/target
  successor mismatch
  reachable drift
  missing/multiple/non-final terminator

instruction/value:
  unknown/missing instruction field
  unsupported opcode/operator
  duplicate value definition
  parameter/dst collision
  undefined use
  missing/extra/stale value type
  Bool payload/type mismatch
  unsupported type row

PHI:
  non-prefix PHI
  missing/foreign/duplicate predecessor
  input/destination type mismatch

edge witness:
  missing witness
  schema/mode drift

ownership:
  missing/extra/duplicate operation
  site/operand drift
  source kind none
  destination kind borrowed
  DestroyOwned borrowed value
  source/destination box_type mismatch
  producer/backend/provider/schema drift
  ReleaseStrong
```

Every failed mutation proves:

```text
published document view count = 0
execution state allocations = 0
executed instructions = 0
Rust fallback = 0
```

## Guard requirements

One structural guard must prove:

```text
strict parser ingress selectors = 1
plain/tolerant HMI parser calls = 0
V0/compact conversion = 0
Rust fallback/retry = 0
HMI field names inside json_native parser = 0

instruction opcode vocabulary matches HMI P0 first-subset inventory
root known opaque fields match current emitter list

raw/generic bounded-view accessors = 0
VerifiedHmiDocumentView constructor sites = 1
other VerifiedHmi*View constructors confined to seal publication

decoded HMI instruction enum = 0
second CFG product = 0
runtime state/handler files in T0 = 0
HMI production callers = 0
ReleaseStrong admission = 0
unconditional print/debug output = 0

every source/check file < 800 lines
```

Reuse `HmiSemanticReferenceInventoryV1` for the first-subset vocabulary. Do
not create a second opcode manifest.

## Required validation

```text
focused .hako MIR verify
release Rust MIR-interpreter execution
debug Rust MIR-interpreter execution
Rust emitter fixture equality tests
mutation harness
HMI authority guard
current-state pointer guard
git diff --check
dev_gate quick
```

## Implementation may claim after P0

```text
one Rust-emitter-backed MIR JSON V1 bounded profile is fully traversed
one strict whole-document seal publishes views only after all admitted rows pass
CFG entry/successor/predecessor/reachability are co-validated
value definitions, uses, and admitted value types correspond exactly
selected scalar and PHI transport laws are sealed
CopyOwned/DestroyOwned transport is sealed but not executed
opaque producer metadata is inaccessible through views
HMI production execution callers remain zero
```

## Implementation must not claim

```text
all MIR JSON V1 semantics
all function metadata semantics
all u64 ownership owner IDs
BoxRef execution
Float/String/void Const execution
dynamic truthiness
Rust interpreter replacement
product VM cutover
fallback compatibility
V0 support
state machine or opcode handler completion
```

## Stop conditions

Stop this series if any implementation requires:

1. A second JSON grammar/tree product.
2. A decoded MIR instruction enum or second CFG.
3. CFG reparsing raw terminator fields after a separate instruction authority.
4. Raw/generic JsonNode access from a published view.
5. Runtime or MIR-table target discovery.
6. Rust fallback, tolerant retry, or V1-to-v0 conversion.
7. Ordinary `.hako` prelude output as a bounded emitter fixture substitute.
8. Ownership execution or raw Rust `Arc`/`VMValue` layout.
9. Expanding T0 to Float, String, MethodCall, or another backend.
10. A source/check file reaching 800 lines.

## WIP preservation

Original parked probe:

```text
66725ad4ddd5d52a50acc03dc7c5a0e470d8bcc0
```

Selectively restored prototype plus first encapsulation repair:

```text
e29886fcffd58540eb7861c97b03812a1dbdb9c1
```

Neither stash may be applied wholesale.

## Final lock

> HMI-S0-T0 lands as `L0 -> S0 -> P0`. L0 owns only typed errors, one-way
> strict ingress, and exact bounded views. S0 seals instruction shape once,
> supplies terminator summaries to CFG, co-validates reachability, builds one
> exact value definition/use/type inventory, verifies scalar/PHI laws, and
> seals the actual Borrowed-or-Owned CopyOwned transport contract. P0 uses
> checked-in documents whose bytes are reproduced by the current Rust emitter,
> runs a mutation matrix, and installs structural drift guards. No raw/generic
> view access, handwritten fixture authority, second MIR schema, execution
> state, product caller, fallback, or file at/above 800 lines is permitted.
