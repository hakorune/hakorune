# 3500 - LANGV1-WEAK-FIELD-CONTRACT-DESIGN-STOP-001

## Status

Decision: accepted. The selected design is A + W1 + S1 + R1 with one atomic
runtime owner and a Rust VM semantic-reference consumer only.

Implementation remains stopped until the WeakRef value-law corrective in the
task order below closes. Do not combine lifecycle-value correction and Weak
field activation in one implementation commit.

## Accepted Decision

```text
semantic owner:
  WeakFieldContractOwner
canonical write:
  WeakFieldWrite
runtime storage:
  InstanceBox-owned declaration-indexed WeakSlotState
empty read:
  unified absence
accepted write values:
  WeakRef or unified absence
runtime order:
  resolve declaration identity -> validate runtime value -> publish weak slot
  -> infallible lifecycle/barrier bookkeeping
alias law:
  runtime InstanceBox declaration layout always re-resolves and enforces
first backend:
  Rust VM semantic-reference adapter only
grammar:
  direct weak field Canonical; visibility/init aliases Compat2025-only
runtime check elision:
  forbidden
```

Lifecycle remains the owner of target token, Alive/Dead/Freed, upgrade, and
WeakRef equality. `WeakFieldContractOwner` consumes those laws but does not own
the ownership kernel.

## VM Retirement Boundary

The Rust VM has retired from the product/default mainline, not from the small
semantic-reference subset. The implementation may add only the minimum
adapter needed to observe the accepted language law.

```text
allowed:
  backend-neutral WeakFieldRuntime owner
  InstanceBox declaration-indexed weak-slot storage
  thin vm-reference dispatch from WeakFieldWrite to that owner
  focused semantic-reference fixtures

forbidden:
  new VM product route or fallback
  VM CLI, runner, optimizer, or fast-path widening
  VM-specific weak storage policy
  generic obj_fields expansion
  new proof-only VM smoke families
  claiming VM success as product/backend completion
```

EXE/AOT remains product proof. Until a later backend card implements the same
typed carrier and runtime law, active Weak field contracts fail backend
preflight before effects. No VM fallback is allowed.

## Implementation Task Order

The consultation response proposed one large 3501. It contains two different
semantic owners and is split to preserve the one-owner-per-card rule.

```text
3501 - LANGV1-WEAKREF-VALUE-LAW-CORRECTIVE-001
  owner: lifecycle / WeakRef value law
  scope: target-token equality, Dead/Freed upgrade, strict WeakRef.Load
  non-claim: weak_field_contract_activation = 0

3502 - LANGV1-WEAK-FIELD-CONTRACT-OWNER-001
  owner: WeakFieldContractOwner
  prerequisite: 3501 green
  scope:
    source spec and schema fingerprint
    declaration-indexed WeakSlotState
    WeakFieldWrite and refreshed carrier
    alias-complete backend-neutral runtime owner
    thin vm-reference adapter and MIR JSON
    non-VM backend fail-fast

3503 - LANGV1-WEAK-FIELD-PRODUCT-BACKEND-SELECTION-DESIGN-STOP-001
  prerequisite: 3502 closeout
  purpose: select the first EXE/AOT consumer
  guard: existing LLVM WeakRef handles do not prove Weak field contracts
```

## Trigger

3499 closes invariant exact-numeric Typed `Array<T>` state contracts. The next
in-language guarantee row is Weak field. Current code has three disconnected
pieces:

```text
source declaration:
  weak field / Compat2025 weak aliases

builder-local evidence:
  WeakFieldValidatorBox accepts MirType::WeakRef or MirType::Void

runtime behavior:
  WeakRef New/Load; failed weak_to_strong() returns the unified absence value
```

There is no source-owned Weak field carrier, semantic-refresh owner, explicit
publication operation, runtime field-write consumer, or central backend
capability row. `MirType::WeakRef` is representation evidence and cannot close
the semantic contract.

## Current Implementation Inventory

The worker inventory narrows the design stop to one storage/publication owner
problem rather than a parser or WeakRef-operation problem.

### Source and declaration identity

```text
parser / AST transport:
  UserBoxFieldDecl { name, declared_type_name, is_weak }

module metadata:
  user_box_field_decls[box_name][field_index]

runtime InstanceBox:
  class_name
  declared_field_index
  weak_fields_union
```

The source declaration already has the required semantic coordinates. The
carrier should use box declaration identity plus declaration-order field index;
field names remain diagnostic data only.

### Builder-only acceptance and alias gap

`WeakFieldValidatorBox` accepts only `MirType::WeakRef` and `MirType::Void`.
However, weak-field discovery is currently:

```text
ValueId
  -> type_ctx.value_origin_newbox
  -> class name
  -> weak_fields_by_box[field name]
```

This is not alias-complete. A receiver arriving through a parameter, field
load, PHI, or other dynamic boundary can lose `value_origin_newbox`; the same
declared weak slot can then reach ordinary `FieldSet` without the builder-local
check or barrier emission. `MirType` and origin maps therefore cannot be the
semantic owner.

### Runtime storage split

The VM executes ordinary `FieldSet` through the generic `setField` path.

```text
primitive / Void:
  InstanceBox fields_ng / declared_field_values

strong BoxRef:
  InstanceBox box_fields

WeakBox:
  generic interpreter obj_fields fallback
```

`WeakBox` is explicitly excluded from the InstanceBox scalar conversion and
is not accepted by the strong `BoxRef` branch. Consequently, a live WeakRef is
stored outside the declaration-indexed InstanceBox field storage, while an
explicit clear (`Void`) is stored inside it. Reads search these stores through
different paths. The closeout must choose one weak-slot representation and
must not preserve this split as language authority.

### Read and absence behavior

The normative law says reading a weak field yields a `WeakRef` without
auto-upgrade. Current builder code annotates a read as `MirType::WeakRef` only
when origin inference succeeds. Runtime storage can instead return `Void` for
an uninitialized or explicitly cleared slot.

The consultation must therefore distinguish:

```text
empty weak slot value:
  what a direct field read returns

failed weak upgrade:
  unified absence (Void/null)
```

No second language-level absence is allowed, but an internal empty WeakRef
representation may still be needed if every weak-field read must remain a
WeakRef value.

### Publication and barrier reality

Current MIR order is:

```text
FieldSet
-> Barrier(Write, value)
```

The Rust MIR interpreter treats `Barrier` as a no-op. LLVM lowers it to a
memory fence, but neither backend has a typed Weak-field carrier consumer.
Existing LLVM support proves only `WeakRef(New/Load)` handle behavior; it does
not prove Weak-field publication semantics. A separate post-store barrier
cannot provide check-before-publication atomicity.

### Test coverage gap

The existing Weak-field smoke verifies direct builder-known receivers and
compile-time rejection. It does not structurally cover:

```text
parameter / alias receiver writes
PHI-selected receivers
runtime strong-Box rejection through an untyped path
empty or cleared weak-field reads
dead-target field reads and upgrade
storage location convergence
barrier/check/publication ordering
backend carrier silent-drop
```

These cases belong in the implementation closeout fixture matrix.

## Existing Authority

```text
grammar/profile authority:
  grammar registry weak_stored_field / weak_visibility_field /
  weak_legacy_init_field rows

lifecycle authority:
  docs/reference/language/lifecycle.md

absence authority:
  null / void / failed weak upgrade share one runtime absence

field declaration identity:
  box declaration identity + field declaration index
```

## Non-Authority

```text
MirType::WeakRef
value_types
WeakFieldValidatorBox acceptance alone
FieldSet shape alone
backend weak-handle layout
successful VM or LLVM execution
field/source names without declaration identity
```

## Consultation Questions

1. Select the single semantic owner:

```text
Candidate A (recommended):
  WeakFieldContractOwner owns declaration spec, write publication, carrier,
  runtime check, and backend capability.

Candidate B:
  lifecycle owner also owns Weak field value-shape acceptance.
```

2. Select the canonical MIR write boundary:

```text
W1 (recommended):
  explicit WeakFieldWrite { contract_id, base, field_index, value }

W2:
  FieldSet plus durable typed sidecar
```

3. Confirm the first-slice accepted values:

```text
WeakRef produced by canonical weak expression or weak-field read
unified absence value for explicit clear

reject:
  strong BoxRef
  primitive
  unknown/untracked value without runtime observation
```

4. Confirm absence law:

```text
explicit weak clear and failed weak_to_strong() observe the same canonical
absence; Weak field does not introduce a second null/none state
```

5. Confirm source identity and carrier:

```text
WeakFieldContractSpec {
  box_declaration_id
  field_index
  field_name_for_diagnostics
}

WeakFieldWriteContract {
  contract_id
  boundary
  base
  value
  runtime_check_required
  backend_capability_required
}
```

6. Decide runtime ownership and order:

```text
evaluate base once
-> evaluate value once
-> resolve final declared Weak field
-> validate WeakRef or absence
-> publish field mutation
-> execute write barrier/lifecycle bookkeeping
```

Does the write barrier occur only after successful publication, or is it part
of one atomic owner operation?

Recommended refinement:

```text
WeakFieldWrite runtime owner atomically performs:
  declaration identity resolve
  -> value-shape validation
  -> weak-slot publication
  -> owner-required lifecycle/barrier bookkeeping

The generic Barrier instruction may remain derived backend evidence, but it is
not the contract owner and cannot make a failed write observable.
```

7. Decide alias/dynamic boundary behavior. If a field write reaches the same
Weak slot through an untyped alias, must runtime declaration identity recover
and enforce the same contract rather than trusting builder facts?

8. Select first supported backend set:

```text
Candidate V (recommended):
  Rust reference VM only; every other backend rejects before effects until it
  consumes the typed carrier.

Candidate H:
  VM plus existing LLVM weak-handle harness, but only after a typed carrier
  consumer and silent-drop guard are proven in the same implementation card.
```

9. Confirm that direct `weak field` is Canonical and visibility/init aliases
remain Compat2025-only; this card must not reopen grammar profile decisions.

10. Select the empty-slot read law:

```text
R1:
  an empty/cleared weak field reads as unified absence

R2:
  every weak field read returns a WeakRef value, including an internal empty
  WeakRef whose weak_to_strong() returns unified absence
```

The choice must also define uninitialized field state and dead-target behavior.

11. Select the runtime storage owner:

```text
S1 (recommended):
  declaration-indexed InstanceBox weak-slot storage owns WeakRef and empty
  state; generic obj_fields is not a Weak-field authority

S2:
  a separate declaration-indexed weak-slot table owned by InstanceBox
```

Keeping live WeakRef in `obj_fields` and clear state in `fields_ng` is rejected.

## Consolidated Consultation Packet

```text
Please review 3500 using the current implementation inventory below as
evidence, not authority.

Facts:
  - source metadata already carries UserBoxFieldDecl.is_weak and declaration
    order
  - builder acceptance depends on value_origin_newbox + MirType and is not
    alias-complete
  - VM FieldSet stores WeakBox in generic obj_fields, but Void clear in
    InstanceBox declaration-indexed scalar storage
  - weak-field reads are only annotated WeakRef when builder origin inference
    succeeds
  - FieldSet is emitted before Barrier(Write); VM Barrier is no-op
  - LLVM WeakRef New/Load support is not a typed Weak-field write consumer

Please decide:
  1. A: WeakFieldContractOwner as the single semantic owner, or B lifecycle
     owner also owns value-shape acceptance.
  2. W1 explicit WeakFieldWrite, or W2 FieldSet plus durable sidecar.
  3. S1 declaration-indexed InstanceBox weak-slot storage, or S2 a separate
     declaration-indexed weak-slot table.
  4. R1 empty/cleared field reads unified absence, or R2 every read returns an
     empty-capable WeakRef and only upgrade yields absence.
  5. Whether validation + publication + lifecycle/barrier bookkeeping must be
     one atomic runtime owner operation.
  6. Whether runtime declaration identity must enforce writes through aliases,
     parameters, PHI-selected receivers, and generic dynamic field paths.
  7. First backend set: VM only, or VM + LLVM only after a typed carrier
     consumer and silent-drop guard land together.
  8. Confirm accepted values are WeakRef and unified absence only; strong
     BoxRef, primitive, and unobserved dynamic values reject before mutation.
  9. Confirm grammar profiles stay closed: direct weak field Canonical;
     visibility/init spellings Compat2025-only.

Please return:
  - selected owner and storage law
  - exact read/clear/dead-target semantics
  - exact evaluation/check/publication/barrier order
  - carrier and MIR operation schemas
  - alias/runtime lookup law
  - backend capability policy
  - stable reject tags
  - fixture matrix
  - minimum implementation card and explicit non-claims
```

## Proposed Minimum Implementation

If Candidate A + W1 + V are accepted:

```text
LANGV1-WEAK-FIELD-CONTRACT-OWNER-001

1. source-owned declaration spec and stable field identity
2. explicit WeakFieldWrite operation
3. semantic_refresh carrier rebuild/drift validation
4. one VM runtime consumer with check-before-publication
5. dynamic alias path convergence
6. MIR JSON transport
7. central unsupported-backend preflight
8. retire WeakFieldValidatorBox as acceptance authority
9. positive/negative/absence/barrier-order fixtures
```

## Fail-Fast Boundary

```text
[type/weak_field_contract_carrier_missing]
[type/weak_field_contract_source_drift]
[type/weak_field_contract_refresh_bypass]
[type/weak_field_contract_violation]
[type/weak_field_contract_check_after_publication_forbidden]
[type/weak_field_contract_mirtype_as_proof_forbidden]
[type/weak_field_contract_runtime_bypass]
[type/weak_field_contract_backend_unsupported]
[type/weak_field_contract_backend_silent_drop]
```

## Non-Claims

```text
weak_field_contract_activation = 0
ownership_kernel_activation = 0
strong_field_cascade_policy_change = 0
new_absence_state = 0
grammar_profile_change = 0
backend_weak_contract_support = 0
runtime_check_elision = 0
selfhost_claim = 0
```
