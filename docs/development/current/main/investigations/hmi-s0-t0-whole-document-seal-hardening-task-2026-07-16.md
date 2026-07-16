---
Status: HMI-S0-T0-L0 closed; S0 is next
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
