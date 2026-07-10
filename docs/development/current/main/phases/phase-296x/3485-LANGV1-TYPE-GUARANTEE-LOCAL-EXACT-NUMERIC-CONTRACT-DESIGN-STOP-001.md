# 3485 - LANGV1-TYPE-GUARANTEE-LOCAL-EXACT-NUMERIC-CONTRACT-DESIGN-STOP-001

## Status

Active design consultation stop after 3484 closes exact-numeric return exit.

Decision: required before parser, MIR, VM, or backend behavior changes.

```text
implementation_before_decision = forbidden
worktree_at_consultation = clean required
```

## Objective

Select one stable `LocalSlotContractOwner` and one runtime write boundary for
explicit exact-numeric local annotations. Close initialization, reassignment,
shadowing, branch PHI, loop-carried values, Any refinement, and proof
invalidation without using variable names, current ValueIds, MirType, or
representation facts as semantic authority.

## Accepted Inherited Laws

```text
local x: T means semantic contract T
runtime check elision requires a fresh verifier proof
Any crossing into T requires a runtime check
unsupported backend rejects before effects
FunctionEntryContractOwner and FunctionReturnContractOwner remain separate
ExactNumericRuntimeValueChecker may be shared only as a subordinate checker
runtime/backend fallback = 0
broad static type checker = 0
```

## Current Evidence

### Source carrier exists but Stage1 drops it

`ASTNode::Local` carries parallel vectors:

```text
variables
initial_values
declared_type_names
```

The ordinary MIR dispatch in `builder/exprs.rs` forwards only `variables` and
`initial_values` to `build_local_statement`. The declared local type does not
reach a typed function metadata carrier or runtime owner.

### Initialization and reassignment have different physical shapes

```text
local initialization:
  initializer -> fresh ValueId -> Copy into fresh local ValueId
  -> declare_local_in_current_scope

reassignment:
  evaluate RHS -> replace variable_map[name] with RHS ValueId
  no dedicated local-write MIR operation
```

Therefore current ValueId is not stable slot identity. A contract attached
only to the initialization Copy would miss reassignment.

### Shadowing and control flow invalidate name/ValueId authority

```text
scope shadowing:
  same source name may denote distinct local contracts

if PHI:
  variable_map is replaced with a merged PHI ValueId

loop carriers / JoinIR exits:
  variable_map reconnects to remapped carrier/exit ValueIds

Any/dynamic merge:
  exact_numeric_value_facts deliberately drop mixed exact/dynamic truth
```

Existing exact-numeric facts and MirType are representation/advisory evidence,
not local-contract proof.

## Authority / Non-Authority

Authority candidates to decide:

```text
source local declaration annotation
stable lexical LocalSlot identity
typed function-owned local contract carrier
runtime write/check event owned by LocalSlotContractOwner
fresh verifier proof for optional future elision
backend capability manifest
```

Non-authority:

```text
variable name alone
current variable_map entry
ValueId alone
MirType/value_types
exact_numeric_value_facts alone
PHI type_hint
parser acceptance
successful VM execution
backend layout or coercion
```

## Decision Questions

### A. Stable local identity

Choose one:

```text
A1. typed LocalSlotId allocated at lexical declaration and carried through
    shadow scopes, assignment, PHI, and loop-carrier publication (recommended)

A2. source variable name + lexical depth composite identity

A3. declaration ValueId as identity, with aliases/remaps recorded
```

The decision must state how macro/JSON AST paths obtain the same identity and
how duplicated or missing slot rows fail fast.

### B. Runtime write boundary

Choose one:

```text
B1. explicit MIR LocalContractWrite operation/check event for every init and
    reassignment; backend capability owns support

B2. normalize every local init/reassignment into a fresh Copy and attach a
    typed LocalWriteContract metadata row to that exact site

B3. function-entry table maps final ValueIds to slots and VM checks lazily on
    reads/PHI publication
```

B3 is not recommended because it checks after publication and makes reads an
owner of write contracts. The decision must preserve RHS evaluation once and
reject before the local binding becomes observable.

### C. PHI and loop policy

Decide whether each incoming write is checked before merge, or whether the
merge publication itself is also a contract boundary. Required cases:

```text
both incoming exact and valid
one incoming Any valid at runtime
one incoming wrong type
same exact type across loop backedge
mixed exact source types
unreachable incoming edge
break/continue/JoinIR exit remap
```

Recommendation: every source write is checked; PHI/loop publication validates
slot identity and proof freshness but does not duplicate successful runtime
checks unless an unchecked Any/dynamic edge enters.

### D. Uninitialized annotated local

Current `local x` lowers to Null/Void-like initialization. Decide one:

```text
D1. reject `local x: T` without an initializer for non-optional exact T
    before effects (recommended)

D2. permit declaration but use an explicit Uninitialized local state that
    cannot be read and is not Void

D3. initialize to Void and fail only when read or overwritten
```

D3 conflicts with `x: T` as an immediate semantic guarantee and is not
recommended.

### E. Carrier and proof schema

Confirm the minimum typed carriers:

```text
LocalSlotContract:
  contract_id
  local_slot_id
  source_name_for_diagnostics
  declared_type_name
  contract_kind = ExactNumeric
  runtime_check_required = true
  proof_elision_allowed = false in first slice
  backend_capability_required = local_slot_exact_numeric

LocalWriteContract:
  local_slot_id
  block
  instruction_index or explicit operation identity
  incoming_value_id
  write_kind = Init | Reassign | PhiPublish | LoopCarryPublish
```

Decide freshness rules under CFG rewrite, SSA remap, and semantic refresh.

## Required Fail-Fast Boundary

```text
type/local_contract_carrier_missing
type/local_contract_carrier_drift
type/local_contract_duplicate_slot
type/local_contract_write_site_missing
type/local_contract_write_site_drift
type/local_contract_violation
type/local_contract_uninitialized_forbidden
type/local_contract_check_after_publication_forbidden
type/local_contract_name_authority_forbidden
type/local_contract_value_id_authority_forbidden
type/local_contract_mir_type_as_proof_forbidden
type/local_contract_value_fact_as_proof_forbidden
type/local_contract_proof_stale
type/backend_local_contract_capability_missing
type/backend_local_contract_silent_drop
```

Stable strings must be defined once by the selected owner. No by-name helper,
fixture-only branch, implicit fallback, or environment-selected activation.

## Minimum Implementation Slice After Decision

```text
exact numeric explicit local with initializer
ordinary straight-line reassignment
one stable LocalSlot identity
one LocalSlotContractOwner
runtime checks always on; proof elision = 0
VM support only
MIR JSON typed carriers
non-VM backend preflight rejection
shadowing + one if-PHI + one loop-carrier fixture
```

Do not include all local types, optional/uninitialized semantics beyond the
accepted decision, FFI, closure captures, backend lowering, broad inference,
or a broad static checker.

## Non-Claims

```text
local_contract_activation = 0
local_slot_identity_decided = 0
local_write_boundary_decided = 0
local_proof_elision = 0
all_local_types_activated = 0
closure_capture_contract = 0
ffi_contract_activation = 0
backend_contract_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Stop Rule

Stop here and request design review. Do not implement until A-E, fail-fast
ownership, first backend set, and minimum slice are accepted.

## Worker Inventory Rerun

The 2026-07-11 source rerun narrows the design surface further.

### Existing BindingId is the viable slot identity

The repository already has `hakorune_mir_core::BindingId` with exactly the
required law:

```text
lexical binding identity
independent from ValueId allocation
new identity on shadow declaration
stable across SSA renaming
restored with the outer binding on scope exit
```

Normal local declaration allocates it in
`builder/vars/lexical_scope.rs::declare_local_in_current_scope`. Ordinary
reassignment changes only the current ValueId and correctly retains the same
BindingId. Therefore a second `LocalSlotId` allocator would duplicate truth.

Mechanical conclusion:

```text
A1 is selected in shape:
  LocalSlotId = existing BindingId semantic role
  do not add a second slot identity namespace
```

The source AST does not need to persist BindingId. Rust, Hako/Program JSON,
and macro AST paths all retain declaration order and type annotations; MIR
lowering allocates the function-local BindingId at the lexical declaration
boundary.

### BindingId coverage is currently incomplete

Several CorePlan/branch lowering helpers publish locals by updating
`branch_bindings` and `variable_map` directly. They bypass
`declare_local_in_current_scope` and do not update `binding_ctx`. Snapshot
helpers also save only `variable_map` in some paths.

This is a hard prerequisite, not a local-contract fallback opportunity:

```text
all accepted local declarations -> one declaration API -> BindingId
all assignments/publications -> resolve existing BindingId
missing BindingId -> fail-fast
name-derived post-hoc BindingId -> forbidden
```

`region::FunctionSlotRegistry::SlotId` is not suitable authority because its
reverse map is name-based and does not model shadow declarations.

### Annotation transport is intact before MIR

The Rust parser, AST JSON roundtrip, JoinIR-compatible JSON decoder, and
Program JSON v0 lowering retain `declared_type_names`. Program JSON also uses
the annotation for typed Array/record context. The ordinary MIR statement
dispatcher is the first confirmed point that drops it.

This means parser changes are not required for the first Rust MIR slice.
Program JSON behavior is evidence and must not become local-contract runtime
authority.

### Copy metadata alone cannot be rebuilt safely

Initialization currently emits a fresh `Copy`; reassignment publishes the RHS
ValueId directly. Converting reassignment to `Copy` would give both writes a
physical site, but an ordinary Copy does not retain BindingId or contract
intent. After CFG rewrite, instruction movement, or semantic refresh, a
sidecar `(block, instruction_index)` row cannot be reconstructed from Copy
alone.

Therefore B2 is acceptable only if Copy receives durable typed local-write
provenance, which is effectively a new semantic operation hidden inside a
generic representation op. The cleaner recommendation is B1:

```text
LocalContractWrite {
  dst,
  src,
  binding_id
}

order:
  evaluate RHS exactly once
  -> validate contract
  -> publish fresh dst as the binding's current ValueId
```

The operation carries identity, while `FunctionMetadata.local_slot_contracts`
carries declared contract policy. Semantic refresh can scan operations and
rebuild/validate write rows instead of trusting stale site metadata.

### PHI and loop consequence

If every source initialization/reassignment crosses `LocalContractWrite`, all
incoming values for one BindingId have already been checked. PHI and loop
publication should preserve BindingId and validate edge completeness, but not
repeat runtime checks. An incoming edge lacking a checked write is carrier
drift and must fail verification; it is not a reason for a lazy read check.

### Remaining semantic decisions

The inventory cannot mechanically decide these points:

1. whether `LocalContractWrite` is accepted as canonical MIR vocabulary or a
   durable typed extension of Copy is preferred;
2. whether `local x: T` without initializer is rejected or introduces a true
   Uninitialized state;
3. exact carrier freshness rules for CFG clone/remap and JoinIR carrier
   reconstruction;
4. whether first-slice MIR JSON exports the operation as transport evidence
   while every non-Rust-VM consumer remains capability-rejected.

These require the consultation below.

## Pro Consultation Packet

Please review 3485 as a design stop. No implementation has started.

### Fixed inherited contract

```text
local x: T = gradual semantic contract T
BindingId is existing lexical identity; ValueId/name/MirType are non-authority
RHS evaluates exactly once before the write check
contract violation rejects before binding publication
runtime check elision = 0 in first slice
VM is the only first-slice runtime consumer
unsupported backend rejects before effects; no fallback
```

### Requested decisions

#### 1. Identity and declaration entry

Accept this?

```text
LocalSlot identity = existing BindingId
one declaration API allocates BindingId for normal and CorePlan paths
assignment/PHI/loop routes must carry or resolve that BindingId
missing/duplicate identity fails fast
```

Should BindingId be exported directly in typed MIR carriers, or wrapped as a
domain-specific `LocalSlotId(BindingId)` newtype without a second allocator?

#### 2. Write operation

Choose:

```text
W1 recommended:
  explicit LocalContractWrite { dst, src, binding_id }
  semantic refresh rebuilds write inventory from the operation

W2:
  ordinary Copy plus durable typed local-write provenance
  must remain reconstructable after CFG/SSA rewrites
```

Please define whether `LocalContractWrite` remains in canonical MIR or is
verified and lowered to Copy only after the selected backend proves support.

#### 3. Publication and PHI/loop law

Accept this?

```text
initializer/reassignment:
  Eval RHS -> Check -> Publish fresh dst

PHI/loop:
  no duplicate runtime check when all reachable incoming writes are checked
  preserve one BindingId across reassignment/backedge
  shadow declaration always has a new BindingId
  unchecked/mismatched incoming edge = verifier fail-fast
  unreachable edges do not create obligations
```

Please specify how CFG clone/remap rewrites operation identity and how JoinIR
carrier lowering proves it retained BindingId.

#### 4. Uninitialized annotation

Choose:

```text
U1 recommended:
  local x: T without initializer is rejected for non-optional exact T

U2:
  introduce explicit Uninitialized state, prohibit reads, and require a
  checked first assignment before the binding becomes initialized
```

Do not use current Null/Void initialization for an active exact contract.

#### 5. Typed carrier schema

Confirm or revise:

```text
LocalSlotContract:
  contract_id
  binding_id
  diagnostic_source_name
  declared_type_name
  contract_kind = ExactNumeric
  runtime_check_required = true
  proof_elision_allowed = false
  backend_capability_required = local_slot_exact_numeric

LocalWriteContract (derived/rebuilt):
  binding_id
  operation identity
  dst
  src
  write_kind = Init | Reassign
```

Should PHI/LoopCarry be separate identity-preservation evidence rather than
`LocalWriteContract` rows, since they do not introduce a new source write?

#### 6. Transport/backend boundary

Confirm:

```text
MIR JSON exports typed slot contracts and write/identity evidence
Rust MIR interpreter validates and executes LocalContractWrite
PyVM/LLVM/AOT/Wasm reject centrally before effects
no backend infers support from Copy, MirType, names, or successful VM runs
```

### Requested response shape

Please return:

```text
selected identity wrapper/owner
selected W1 or W2 and exact operation order
PHI/loop identity and verifier law
selected U1 or U2
accepted carrier schemas and freshness rules
backend/MIR JSON boundary
stable fail-fast tags
minimum implementation slice and fixture matrix
claims/non-claims
conditions for proceeding to representation-only :T audit
```
