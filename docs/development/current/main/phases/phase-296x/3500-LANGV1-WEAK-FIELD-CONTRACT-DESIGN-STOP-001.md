# 3500 - LANGV1-WEAK-FIELD-CONTRACT-DESIGN-STOP-001

## Status

Active design consultation stop. Do not change parser, MIR, runtime, backend,
or acceptance behavior until the questions below are accepted.

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

