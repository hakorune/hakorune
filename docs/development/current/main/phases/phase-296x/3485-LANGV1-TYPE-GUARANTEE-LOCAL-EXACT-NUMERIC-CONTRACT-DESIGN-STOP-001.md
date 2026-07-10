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
