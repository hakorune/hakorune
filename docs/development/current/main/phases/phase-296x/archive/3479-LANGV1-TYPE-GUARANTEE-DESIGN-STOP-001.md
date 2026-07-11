# 3479 - LANGV1-TYPE-GUARANTEE-DESIGN-STOP-001

## Status

Complete design consultation after `LANGV1-GRAMMAR-001` closes in 3478.

Decision: accepted.

Implementation: delegated to 3480.

## Objective

Define one site-by-site Language v1 guarantee matrix for `: T` without mixing
semantic contracts with MIR/storage/planner hints.

Target direction inherited from the accepted Language v1 workstream:

```text
annotation omitted -> Any
x: T -> gradual semantic contract T
representation/planner hint -> MIR facts, Plan, or Rune; never : T
```

This card decides the matrix and first implementation slice. It does not
activate checks.

## Accepted Decision

```text
Canonical x: T = eventual gradual semantic contract T at every site
permanent metadata-only annotation site = forbidden
metadata_only_non_guarantee = explicit transitional state only
representation/storage/planner/Rune fact = non-contract axis
```

Guarantee class and enforcement owner are separate axes:

```text
guarantee class:
  any_default
  metadata_only_non_guarantee
  runtime_checked_contract
  verifier_proven_contract
  verified_runtime_guarded_contract
  unsupported_failfast
  representation_fact_non_contract

enforcement owner:
  none
  runtime_check
  verifier_proof
  verifier_proof_plus_runtime_audit
  backend_capability_preflight
  semantic_reject
```

The first implementation slice is the existing exact-numeric Box field write
boundary. Its single owner remains
`src/mir/exact_numeric_field_contracts.rs`; do not create a parallel owner just
to match a design label. Existing verifier, VM runtime-check, MIR JSON, and
backend-capability consumers remain subordinate projections.

Proof freshness for this first slice is structural and re-derived: semantic
refresh rebuilds proof/check decisions from the current function body,
declaration metadata, `FieldSet` site key, and value producer. Do not add fake
CFG/SSA epochs or hashes that the current compiler does not own. A future
cross-refresh proof cache must open a separate freshness decision.

## Current Evidence To Inventory

```text
already narrow/live:
  exact numeric field-write contracts
  record construction/update checks
  typed Array<T> element checks
  Weak field checks

mostly metadata today:
  local annotations
  parameter annotations
  return annotations
  ordinary Box field annotations outside named verifier rows

separate representation facts:
  MIR type facts
  exact numeric storage metadata
  backend layout plans
  Rune/planner hints
```

The inventory must name the current parser transport, compile-time check, MIR
verifier, runtime check, backend support, failure tag, and unsupported-backend
behavior for each site. Counts, source paths, and annotation spelling alone
are evidence, not semantic authority.

## Closed Site Set

```text
local initialization and reassignment
parameter entry
return exit
ordinary Box field initialization and write
record field construction and with-update
static table element
ordinary collection element
typed Array<T> element
Weak field
FFI boundary
backend boundary
```

## Consultation Questions

1. Confirm whether every canonical `x: T` site must eventually enforce the
   same gradual contract, with no permanent metadata-only exception.
2. Select the guarantee vocabulary per matrix cell:
   `metadata_only`, `compile_time`, `mir_verified`, `runtime_checked`, or an
   explicit combination.
3. Fix the single check owner for each boundary and the order of activation.
4. Define when verifier proof may elide a runtime check and what proof object
   authorizes elision.
5. Define fail-fast behavior when VM can enforce a contract but EXE/AOT cannot.
6. Decide how representation-only annotation uses are identified and migrated
   before canonical contract activation.
7. Select the first code-facing slice: locals+parameters, or a narrower
   inventory/matrix artifact if current ownership is not sufficiently closed.

## Source Authority

```text
language laws:
  docs/reference/language/semantic-contract-charter.md

current executable type evidence:
  docs/reference/language/types.md
  docs/reference/language/stage-profiles.md

ordered contract:
  docs/development/current/main/workstreams/language-v1-convergence-current.md
  docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md

grammar boundary:
  grammar/language-v1-registry.toml
  3478 grammar closeout
```

## Non-Authority

```text
annotation spelling alone
current parser acceptance alone
MIR type metadata alone
backend storage width alone
source path or occurrence count
existing test count
Rust/Hako implementation agreement without a semantic contract
```

## Required Fail-Fast Boundary

```text
metadata must not be claimed as semantic truth
runtime-check elision requires verifier-backed proof
unsupported backend rejects before user-visible effects
no VM-success fallback for unsupported EXE/AOT guarantees
no broad static type checker hidden inside the first slice
no type-contract activation before the matrix decision
```

## Allowed Next Implementation After Decision

At most one substantive card for the selected first slice. It must include the
matrix artifact, positive/negative fixtures for every newly live guarantee,
one check owner per boundary, and unsupported-backend fail-fast coverage. Do
not create inventory-only, fixture-only, or rerun-only numbered cards.

## Claims

```text
type_guarantee_design_consultation_open = 1
annotation_site_set_closed_for_consultation = 1
grammar_closeout_retained = 1
type_guarantee_design_decision = 1
all_colon_annotations_eventual_gradual_contract = 1
permanent_metadata_only_annotations_rejected = 1
guarantee_vocabulary_decided = 1
single_check_owner_matrix_decided = 1
representation_hint_split_decided = 1
first_slice_box_field_exact_numeric_selected = 1
```

## Non-Claims

```text
annotation_semantic_contract = 0
guarantee_matrix_accepted = 0
type_contract_activation = 0
local_contract_activation = 0
parameter_contract_activation = 0
return_contract_activation = 0
box_field_contract_activation = 0
record_contract_change = 0
array_contract_change = 0
ffi_contract_activation = 0
backend_contract_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Stop Rule

Implementation may proceed only through 3480's exact-numeric Box field slice.
All other annotation sites remain explicit non-claims.
