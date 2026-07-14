---
Status: SSOT
Decision: accepted
Date: 2026-07-14
Scope: Rust MIR interpreter retirement and `.hako` semantic-reference interpreter migration.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1075-VM-ACTIVE-LANE-RETIRE-001.md
  - apps/rust-subset-to-hako/probes/README.md
---

# VM Active Lane Retirement SSOT

## Decision

The VM is no longer a product-level app execution target for current compiler
construction work.

```text
rust_vm_active_product_target=0
rust_vm_semantic_reference_subset=temporary
rust_vm_semantic_reference_final_owner=0
hako_vm_active_product_target=0
hako_mir_interpreter_semantic_reference_target=1
primary_app_validation_route=exe_aot
```

The Rust `MirInterpreter` remains temporarily useful for small semantic smoke
tests and focused MIR reference checks. It is not the final semantic-reference
owner and is not the route where new product apps, JSON-heavy converter apps,
or selfhost compiler fronts must prove full runtime behavior.

The final direction is:

```text
canonical MIR contract
  -> `.hako` MIR interpreter as the semantic-reference owner

canonical MIR JSON/object lowering
  -> EXE/AOT as the production execution owner

Rust MirInterpreter
  -> exact caller-zero retirement after `.hako` cutover
```

This is an implementation-owner migration, not an AST-interpreter decision.
The `.hako` interpreter consumes canonical MIR and must not recover source
semantics from AST, ProgramV0, names, or backend behavior.

## Rationale

The RustSubset JSON -> `.hako` converter reached runtime through the VM route:

```text
filebox_read_enabled=1
json_tokenizer_probe_green=1
joinir_acceptance_blocker_cleared=1
global_mir_call_payload_normalized=1
```

The blocker moved from compiler acceptance to runtime collection semantics:

```text
mapbox_primitive_roundtrip=1
mapbox_user_box_roundtrip=0
arraybox_user_box_roundtrip=0
json_tree_parse_result=null
```

Keeping the VM as a product-level route would require continuing to grow:

```text
rust_vm_collection_user_box_semantics
hako_vm_collection_user_box_semantics
json_native_tree_runtime_surface
product_app_runtime_parity
```

That would split effort across too many execution engines while the current
goal is compiler construction and selfhost progress.

## Target Execution Ownership

```text
EXE/AOT:
  primary product/app validation route
  primary selfhost app-front validation route
  primary performance route

Rust VM:
  temporary small semantic reference subset
  focused MIR smoke tests
  payload normalization reference tests
  no broad runtime parity expansion
  exact caller-zero retirement target

.hako MIR interpreter:
  final small semantic-reference owner
  canonical MIR instruction semantics and normalized event oracle
  expanded only for a named compiler-semantic consumer
  never a product-app parity target
```

## Migration architecture

```text
canonical MIR
  stable opcode and ownership semantics
        │
        ├─ LLVM/object consumer
        │    production EXE/AOT
        │
        ├─ temporary Rust MirInterpreter adapter
        │    migration oracle only
        │
        └─ `.hako` MirInterpreter
             final semantic-reference owner
             normalized outcome + ownership/control events
```

MIR owns meanings such as:

```text
CopyOwned:
  produce a fresh independently consumable owner for the same object without
  consuming the source

DestroyOwned:
  consume exactly the named owned value and no same-object alias

Copy:
  value/representation copy, not an implicit ownership-policy authority

ReleaseStrong:
  legacy lifecycle vocabulary; canonical callers forbidden
```

`Arc`, `VMValue`, Rust register storage, runtime handle numbers, and backend
helper names are implementation details. They cannot appear in the portable
MIR semantic contract or normalized parity authority.

The `.hako` interpreter input transport is not selected by this policy. The
HMI-P0 inventory below must choose one sealed MIR transport. Raw Rust
`MirModule` access, source AST, reconstructed ProgramV0, and a second semantic
MIR schema are forbidden.

## Allowed VM Work

VM work is allowed only when it is narrow and directly protects a semantic
reference contract:

```text
allowed=payload_normalization_unit_tests
allowed=small_mir_semantic_smoke
allowed=fail_fast_diagnostic_for_unsupported_vm_surface
allowed=regression_test_for_already_supported_subset
allowed=bounded_rust_hako_mir_semantic_parity
allowed=caller_inventory_and_retirement_guard
```

## Disallowed VM Work

```text
disallowed=product_json_app_runtime_parity
disallowed=full_user_box_collection_semantics_for_app_execution
disallowed=feature_work_required_only_by_vm_product_route
disallowed=simultaneous_rust_vm_and_hako_vm_product_development
disallowed=silent_vm_fallback_to_hide_aot_gap
disallowed=whole_rust_vm_translation_before_subset_inventory
disallowed=ast_or_programv0_interpreter_as_mir_reference
```

## Required migration tasks

The migration is mandatory for the selfhost final form but is not allowed to
widen the active compiler row. It starts only after the MIR ownership
vocabulary and its supported-backend boundary are stable.

### HMI-P0 — ingress, opcode, and caller inventory

Inventory:

```text
all Rust MirInterpreter instruction handlers
all semantic-reference fixtures and callers
all VM-only product/compat callers
available MIR transports and their lossiness
backend-specific values hidden behind VMValue
```

Select one sealed input transport. The inventory changes no execution owner.

### HMI-S0 — closed portable semantic subset

Seal the first required subset instead of translating the Rust VM wholesale:

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

The exact list follows the accepted ownership instruction spelling. Unsupported
instructions fail before interpreter effects; there is no Rust fallback.

### HMI-S1 — normalized observation contract

Compare portable meaning only:

```text
function outcome
ordered block/edge visits
normalized scalar values
CopyOwned / DestroyOwned ownership events
typed failure category and exact source MIR instruction position
```

Do not compare:

```text
Arc identity
VMValue representation
runtime handle numbers
raw host pointers
hash iteration order
backend helper names
```

### HMI-I0 — disconnected `.hako` MIR interpreter core

Implement the closed S0 subset in small responsibility boxes:

```text
ingress/
  sealed MIR reader only

frame/
  ValueId environment and current predecessor

control/
  block entry, edge selection, Phi input selection and Owned parallel move

ownership/
  VerifiedOwnershipSsaV1 reader and CopyOwned / DestroyOwned semantic adapter

execute/
  instruction dispatch and typed outcome
```

No new or modified source/check file may reach 800 lines. The dispatch layer
does not own opcode semantics; each small box implements the sealed contract.

### HMI-P1 — independent Rust/`.hako` parity

Run the same sealed MIR fixtures through both interpreters and compare the S1
normalized result. Required fixtures include straight-line arithmetic,
diamond/Phi, Loop backedge/Phi, owned copy/destroy, Owned Phi forwarding,
self-assignment
law, BlockExpr tail ownership, and typed unsupported-op rejection.

Rust is a temporary comparison oracle only. Matching Rust behavior does not
override the MIR contract when the Rust implementation is wrong.

### HMI-C0 — semantic-reference owner cutover

After parity is green, make `.hako` the default semantic-reference runner for
the closed subset. EXE/AOT remains the product validation route. Rust fallback
is forbidden; unsupported `.hako` instructions fail with a typed capability
error.

### HMI-X0 — consumer-driven subset expansion

Add one MIR instruction family per capability slice, only when a named
compiler-semantic fixture requires it. Every expansion updates the sealed
subset, `.hako` implementation, normalized parity, and unsupported boundary
in one commit.

Broad collection, JSON application, Box, plugin, or REPL parity is not an
implicit goal.

### HMI-R1 — Rust caller isolation

Classify every remaining Rust `MirInterpreter` caller as:

```text
replaced semantic reference
explicit legacy diagnostic
test-only awaiting migration
dead
```

No new Rust semantic-reference caller is allowed after HMI-C0.

### HMI-R2 — physical Rust interpreter retirement

Delete Rust interpreter handlers and support state only after repository-wide
exact caller zero. If an explicit legacy consumer remains, isolate it and do
not claim physical retirement.

## Dependency order

```text
SSA-RC-A1c stable MIR ownership vocabulary/backend boundary
  + SSA-RC-V0 ownership verifier
  + SSA-RC-RET-P0 legacy ReleaseStrong isolation inventory
  -> HMI-P0
  -> HMI-S0
  -> HMI-S1
  -> HMI-I0
  -> HMI-P1

HMI-P1 + first exact BoxRef SSA-I1-O1 owner
  -> HMI-C0
  -> HMI-X0
  -> HMI-R1

repository-wide Rust MirInterpreter caller zero
  -> HMI-R2
```

The `.hako` interpreter may grow beside later If/Loop/exit/family expansion,
but it never blocks a production EXE/AOT capability merely to obtain product
VM parity. Physical Rust retirement is a selfhost completion condition, not a
hidden requirement for one D-prime compiler slice.

## Migration acceptance

```text
semantic reference owners after HMI-C0 = 1 (`.hako`)
Rust fallback after HMI-C0 = 0
product app validation remains EXE/AOT
source AST / ProgramV0 execution authority = 0
normalized parity covers every accepted HMI opcode
unsupported opcode typed fail-fast before effects
new Rust semantic-reference callers after cutover = 0
Rust physical deletion only after repository caller zero
```

## Migration non-claims

```text
`.hako` compiler Lower parity
product VM parity
interactive interpreter or REPL readiness
all MIR opcodes supported at HMI-C0
Box/collection/plugin runtime parity
Rust interpreter physically deleted before HMI-R2
```

## Interactive Interpreter / REPL Parking

Interactive interpreter and Python-like REPL product work is parked.

```text
repl_active_product_target=0
interactive_interpreter_active_product_target=0
rust_mir_interpreter_repl_extension_allowed=0
hako_mir_interpreter_required_before_python_like_repl=1
```

Existing REPL and `MirInterpreter` documentation is historical or
semantic-reference material unless a later accepted current-state card
explicitly reopens the lane.

Do not expand REPL or interpreter behavior for product/app execution. The
`.hako` migration above is a bounded semantic-reference migration, not a
reopening of the product VM or interactive interpreter lane.

Allowed work remains limited to narrow semantic-reference smoke tests,
regression tests for already-supported behavior, and fail-fast diagnostics for
unsupported VM surfaces.

## Converter Implication

The RustSubset converter should not be blocked on the Rust VM route.

```text
rust_subset_converter_primary_route=exe_aot
rust_subset_converter_vm_route=diagnostic_only
json_native_vm_collection_gap_blocks_vm_only=1
```

## Stop Lines

```text
do not fix broad VM runtime parity unless a semantic reference smoke requires it
do not require JSON/native app converter to pass on Rust VM
do not treat VM failure as compiler-construction failure when EXE/AOT is the selected route
do not add VM-specific workarounds to .hako source
do not use silent fallback to mask unsupported VM runtime surfaces
do not translate the Rust VM file-for-file
do not treat Rust parity as authority over the sealed MIR contract
do not delete Rust interpreter code before exact repository caller zero
```
