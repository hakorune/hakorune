---
Status: HMI-P0-D0 closed; HMI-P0-I0 next
Date: 2026-07-16
Decision: accepted with current-source corrections
Previous row: P0c-MR-R0-G0 closed
Durable policy: ../design/vm-active-lane-retirement-ssot.md
Consultation: mirbuilder-post-p0c-selfhost-next-owner-consultation-2026-07-16.md
---

# HMI-P0 MIR JSON V1 Strict Ingress Inventory Task

## Decision lock

The sole future `.hako` MIR semantic-reference ingress is the serialized,
Rust-emitted `MirJsonExportDocument` with `MirJsonExportSchema::V1` and exact
root `schema_version: "1.0"`.

Authority is conditional. Arbitrary MIR JSON V1 is not accepted. A future
`HMI-MIR-JSON-V1-STRICT` verifier must first seal the complete document, after
which HMI handlers may consume bounded views over the same parsed JSON tree.
The strict profile is an HMI acceptance contract over existing V1 bytes, not a
new MIR schema.

Exact producer path:

```text
MirModule
  -> semantic_refresh::refresh_owned_for_boundary(MirJsonExport)
  -> build_mir_json_root (private implementation detail)
  -> MirJsonExportDocument / MirJsonExportSchema::V1
  -> serialize_document
  -> serialized JSON text
  -> future whole-document strict seal
  -> bounded views over the same JSON tree
```

The public current producer boundary is
`emit_mir_json_string_for_harness_bin`. HMI-S0 must add or select one thin HMI
facade that forces exact V1 and verifies the root discriminator. It must not
make the private root builder a new public authority or inherit the current
environment-controlled legacy-v0 selection.

## Non-authorities

```text
MIR JSON v0
json_v1_bridge::try_parse_v1_to_module
MirJsonV1Adapter.to_v0
extract_main_payload_json / payload_normalize
MiniMirV1Scan and substring scanners
raw MirModule access from .hako
AST or Program JSON v0
Rust VMValue layout
JSON -> Rust MirModule -> second HMI transport
backend/provider strings in ownership metadata
```

Existing compatibility readers remain valid for their current callers, but
none may become HMI semantic authority. They infer or normalize facts such as
first-block entry, symbol arity, unknown effects, permissive const values, or
legacy call/opcode shapes.

## Exact transport seams

### Bool

Rust emission encodes Bool Const as an `i64` payload. It is portable only when
the same function metadata has `value_types[dst] == "i1"` and the payload is
exactly `0` or `1`.

```text
payload i64 0|1 + exact metadata i1 -> InlineBool
missing/mismatched metadata         -> ValueTypeMismatch
payload outside 0|1                 -> LossyValueEncoding
```

### Null and Void

Null and Void Const both serialize as `{type:"void", value:0}`. The first
portable profile rejects every void Const as `LossyConstKind`. A terminator
`ret` with no value remains the distinct `NoValue` outcome. HMI does not claim
source/MIR-level Null-versus-Void identity parity.

### Entry and CFG

Function rows do not carry entry block. Root `cfg.functions[]` is the only
entry source after strict validation proves:

```text
function-name bijection
unique function names and block IDs
entry block belongs to that function
CFG block set equals serialized function block set
successor and terminator rows agree with serialized terminators
```

First/minimum serialized block is never an entry authority.

### Ownership

`copy_owned` and `destroy_owned` instruction fields plus
`metadata.ownership_ssa_v1` operation sites/value kinds are passively
transported. The strict profile must prove exact instruction/witness bijection.
The current `backend="llvm_py"` and `provider="nyash_kernel"` fields are
provenance, not portable semantic authority. Production BoxRef admission stays
blocked on SSA-I1-O1.

## HMI-P0 task order

```text
HMI-P0-D0  decision lock and exact current-source naming          CLOSED
HMI-P0-I0  checked-in normalized machine inventory                NEXT
HMI-P0-G0  freshness/coverage/lossiness report and guards
HMI-S0-D0  strict reader/seal and interpreter implementation packet
```

P0 remains inventory and ingress selection only. It changes no execution
owner and activates no opcode.

## HMI-P0-I0 — checked-in normalized machine inventory

Use the existing inventory/guard convention:

```text
tools/checks/fixtures/hmi_semantic_reference_inventory_v1.json
tools/checks/lib/hmi_semantic_reference_inventory.py
```

Do not create the nonexistent `tools/inventory/` convention. The JSON is the
single checked-in classification authority; the Python checker derives current
source sets and compares them against it. It must not contain a second copy of
the same classification policy. Register the reusable G0 guard through the
existing manifest/check-index entry when it lands.

The logical document contains five normalized tables:

```text
instructions
callers
fixtures
transports
value_classes
```

Vocabulary laws:

```text
MIR_INSTRUCTION_KEPT_TAGS:
  canonical instruction vocabulary owner

HMI inventory instruction rows:
  classification/coverage owner only

handler/caller/fixture/transport/value-class definitions:
  owned once

cross-table relations:
  ID foreign keys only
```

Every kept instruction tag is classified, including instructions outside the
first HMI subset. Do not hardcode the current tag count into the semantic
contract; the guard compares exact sets with `MIR_INSTRUCTION_KEPT_TAGS`.

Minimum instruction row:

```text
instruction_id
semantic_owner
dispatch_sites
execution_family
first_subset
transport_op
required_fields
required_metadata
input_value_classes
output_value_classes
lossiness
loss_reasons
caller_ids
fixture_ids
retirement_gate
```

Dispatch sites do not become independent semantics. Fast/traced paths point to
one semantic owner for the instruction.

The source inventory must cover all execution surfaces, not only one match:

```text
handlers::execute_instruction
exec::block::execute_block_instructions hot/diagnostic dispatch
exec::phi::apply_phi_nodes
exec::phi::apply_owned_phi_nodes
exec::block::handle_terminator
```

Phi, Owned Phi, Jump, Branch, and Return must not disappear merely because
they are outside ordinary instruction dispatch.

Caller classes:

```text
semantic_reference
vm_only_compatibility
product
```

Every row has an exact retirement condition. HMI-P1 parity does not itself
delete Rust handlers; HMI-R1/R2 still require the selected cutover and exact
repository caller zero.

## HMI-P0-G0 — drift guards and report

Production behavior delta remains zero. The reusable guard proves:

```text
kept tags == inventory instruction IDs
unclassified/duplicate semantic owners = 0
unclassified dispatch sites = 0
unclassified MirInterpreter callers = 0
unclassified semantic-reference fixtures = 0
unclassified transport transformations = 0
unclassified VMValue variants = 0
HMI-P0 execution callers = 0
raw/V0/AST/ProgramV0/compact ingress = 0
V1-to-v0 conversion in HMI path = 0
runtime handler discovery = 0
source/check files at or above 800 lines = 0
```

G0 generates a normalized human report and a transport-lossiness matrix. It
does not implement or claim the strict document seal.

## HMI-S0-D0 packet boundary

HMI-S0-D0 will taskize, but not yet implement:

```text
S0-T0  direct strict JSON reader + whole-document opaque seal
S0-V0  disconnected block/predecessor/register state machine
S0-I0  exact portable handlers
S0-P0  Rust oracle parity with production callers still zero
```

No decoded `HmiInstruction` mirror enum, second function/block graph, V1-to-v0
conversion, or normalized execution payload is allowed. Verified views borrow
or index fields in the same strict JSON tree.

The strict reader must reject duplicate JSON object keys before constructing a
generic `serde_json::Value`-style tree, or prove an equally strict producer-
trusted boundary. It also rejects unknown fields in bounded semantic rows.
Silent last-key-wins parsing is not a whole-document strict seal.

Reserved first subset:

```text
Const
Copy
CopyOwned
DestroyOwned
BinOp
Jump
Branch
Phi
Return
```

Scalar execution begins with exact i64, exact Bool, and no-value Return.
CopyOwned/DestroyOwned transport and disconnected contracts may be proven, but
production value admission remains O1-blocked.

HMI-S0-D0 must also freeze an exact operation matrix instead of treating
`BinOp` or `Branch` as one undifferentiated row:

```text
allowed BinaryOp variants
overflow behavior
division/modulo-by-zero failure
shift policy
Bool And/Or policy
Branch condition representation = exact i1 only
```

## Stable failure family reserved for S0

Implementation will use precise tags under:

```text
[freeze:contract][hmi/mir_json_v1/document]
[freeze:contract][hmi/mir_json_v1/schema]
[freeze:contract][hmi/mir_json_v1/cfg]
[freeze:contract][hmi/mir_json_v1/value_type]
[freeze:contract][hmi/mir_json_v1/ownership]
```

Do not register these in the debug-contract SSOT until the implementation that
emits them lands.

Whole-document-before-effects law:

```text
parse strict JSON tree
  -> validate root/functions/CFG/instructions/types/PHI/ownership
  -> publish one opaque ingress seal
  -> allocate interpreter state
  -> execute
```

On seal failure, instructions executed, register writes, heap effects, and
Rust/v0 fallback are all zero.

## Required fixture families

Pass transport proofs:

```text
i64 Const -> Copy -> Return
Bool Const encoded as i64 plus value_types=i1
exact scalar BinOps
Jump / i1 Branch / multi-input Phi
entry block not equal to lowest block ID
multiple functions with exact CFG/function bijection
CopyOwned/DestroyOwned with exact ownership witness
Return without value
```

Reject transport proofs:

```text
non-exact schema or legacy/compact roots
missing/extra CFG function or block rows
entry outside function
duplicate function/block IDs
duplicate JSON object keys or unknown bounded-row fields
PHI outside prefix or predecessor mismatch
missing/multiple/non-final terminators
Bool metadata/payload mismatch
void Const, Unknown, unsupported value classes/operators/opcodes
CopyOwned/DestroyOwned witness mismatch
ReleaseStrong in the portable subset
```

## May claim after P0-G0

```text
one existing Rust-emitted MIR JSON V1 carrier is selected for future HMI
one strict acceptance profile is specified over that carrier
all Rust handlers/callers/fixtures/transports/VMValue classes are inventoried
known lossiness and retirement conditions are machine-readable and guarded
HMI-P0 execution-owner and opcode-activation delta is zero
```

## Must not claim

```text
the strict seal exists before HMI-S0-T0
all MIR JSON V1 is lossless
generic Const/VMValue/BoxRef/ExactNumeric/String/Float parity
Null/Void identity preservation
Rust permissive PHI/truthiness/operator fallback parity
.hako interpreter execution or semantic-reference cutover
Rust handler retirement or product VM replacement
parser/MirBuilder/Ownership V2 grammar work
another backend or Rust fallback
```

## Stop conditions

Stop if implementation requires:

1. a second MIR instruction or function/block schema;
2. public raw access to `build_mir_json_root` instead of a thin V1 producer;
3. env-dependent v0 selection on the HMI route;
4. JSON-to-Rust-MirModule reconstruction before `.hako` consumption;
5. first-block entry, symbol arity, tolerant scanner, or runtime inference;
6. Null/Void guessing or permissive PHI/undefined fallback;
7. raw `Arc`/VMValue layout as portable ownership truth;
8. opcode execution, product callers, or Rust fallback during HMI-P0;
9. BoxRef/O1, parser, MirBuilder, Ownership V2, or another backend in this row;
10. a source/check file at or above 800 lines.
