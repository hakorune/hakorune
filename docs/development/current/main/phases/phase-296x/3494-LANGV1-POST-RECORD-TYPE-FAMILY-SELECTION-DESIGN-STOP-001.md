# 3494 - LANGV1-POST-RECORD-TYPE-FAMILY-SELECTION-DESIGN-STOP-001

## Status

Active design consultation stop. Do not change parser, MIR, VM, backend, or
runtime behavior until one remaining family has a source-owned contract and a
single enforcement owner.

Decision: pending.

## Objective

Select at most one next semantic type-contract family after the landed Box
field, parameter-entry, return-exit, local-slot, and record-value slices.
Selection must come from current source evidence, not from `MirType`, storage
layout, backend routes, helper names, or existing green execution.

## Required Inventory

For every remaining candidate, record:

```text
source-owned contract declaration
stable semantic identity
all construction/write/publication boundaries
candidate single enforcement owner
semantic_refresh rebuild inputs
VM consumer boundary
unsupported-backend pre-effect gate
MIR JSON transport need
current bypasses and representation-only consumers
```

At minimum inspect typed `Array<T>`, static tables, ordinary collections, Weak
fields, FFI ingress/egress, and remaining guarantee-matrix families.

## Consultation Questions

1. Which candidate already has exactly one source-owned semantic contract,
   stable identity, and one complete write/publication boundary?
2. For typed `Array<T>`, what owns the source element contract, and which one
   operation owns initialization plus mutation without using `MirType::Array`
   or storage inference as proof?
3. Are static tables and ordinary collections separate semantic families, or
   projections of the same element-contract owner?
4. Is the current Weak-field path an active complete contract, or a narrow
   representation/runtime check requiring its own closeout audit?
5. Should FFI remain deferred until all in-language value boundaries close?
6. Which candidate supports one VM consumer and central non-VM fail-fast
   without opening backend lowering or a broad static checker?
7. Does a candidate require an absence, optional state, truthiness, equality,
   or ownership decision first?

## Authority

```text
docs/reference/language/types.md
typed guarantee matrix and implementation status ledger
source declaration metadata
semantic_refresh carrier owners
current MIR write/publication operations
central backend capability manifest
```

## Non-Authority

```text
MirType or value_types alone
storage/layout metadata
route availability or helper names
source path or use count
green VM behavior
backend fallback or environment-selected behavior
```

## Fail-Fast Boundary

```text
no source-owned contract -> candidate cannot be selected
multiple write owners -> consolidate owner before activation
representation fact used as proof -> reject
missing semantic_refresh rebuild path -> reject
unsupported backend without pre-effect gate -> reject
selection requiring fallback or environment activation -> reject
```

## Minimum Next Slice

The accepted consultation must name exactly one family, one owner, one typed
carrier, one VM consumer, and one backend capability. The implementation card
must not activate two families together.

## Worker Inventory

Current source evidence narrows the candidates as follows. This inventory is
evidence for consultation, not a family-selection decision.

| Candidate | Source-owned contract | Write/publication owner | Refresh carrier | Current judgment |
| --- | --- | --- | --- | --- |
| Typed `Array<T>` | not found; current element type is inferred into `MirType::Array` | array literal `push` plus method `push/set/insert` and planner-produced calls | none | blocked by owner convergence and source contract |
| Static table | `StaticConstTable.element_type` and values | readonly `StaticDataPlan`; no runtime element write | module metadata verifier, not semantic-refresh carrier | closeout-audit candidate, not a mutation slice |
| Ordinary collection | canonical target remains `AnyDefault` | many runtime collection methods | none required for `Any` | no activation work |
| Weak field | declaration `is_weak` is source-owned | `WeakFieldValidatorBox` at field read/write | none | current validator uses `MirType` as proof; corrective needed before claim |
| FFI boundary | no unified source contract carrier | extern/provider/backend-specific routes | none | defer; boundary and owner surface are too broad |

### Typed Array Evidence

```text
src/mir/builder/types/array_element.rs
  receiver-local observation only
  homogeneous literals and push/set/insert update MirType::Array
  value_types, origin, method name, and arity drive the result

src/mir/builder/collection_literals.rs
  array literal lowers each element through runtime ArrayBox.push

No TypedArrayElementContract carrier or semantic_refresh projection exists.
No single semantic array-write MIR operation exists.
```

Therefore Typed Array cannot be activated by wrapping the current inference.
If selected, the first task must converge all accepted element writes onto one
operation without changing acceptance. A source-owned element contract must
then be projected independently of `MirType`.

### Static Table Evidence

```text
ASTNode::StaticConstTable
  -> static_data_plan::collect_static_data_plans_from_ast
  -> module.metadata.static_data_plans
  -> verification::module_metadata
  -> VM/backend readonly consumers
```

This is the strongest existing source-to-consumer chain, but it is readonly and
already classified `VerifierProvenContract`. The useful next work is a narrow
authority/coverage closeout audit, not a runtime write-contract activation.

### Weak Field Evidence

`WeakFieldValidatorBox` is one builder-local decision point, but it validates
assignments from `MirType::WeakRef` or `MirType::Void`. It has no typed carrier,
semantic-refresh rebuild, central backend capability, or runtime publication
owner. Because `Void` currently represents weak clearing and failed upgrade,
the absence/null decision may also constrain its eventual semantic contract.

## Conditional Task Queue

Only the branch selected by consultation becomes active.

### Branch A: Typed Array Selected

```text
A1 BoxShape prerequisite: ARRAY-ELEMENT-WRITE-BOUNDARY-OWNER-001
   - inventory every accepted literal/push/set/insert producer
   - define one canonical ArrayElementWrite semantic MIR operation
   - preserve evaluation order and behavior
   - no type-contract activation

A2 implementation: LANGV1-TYPED-ARRAY-ELEMENT-CONTRACT-OWNER-001
   - add source-owned ArrayElementContractSpec
   - stable array identity without ValueId/name authority
   - semantic_refresh typed carrier
   - runtime check before element publication
   - one VM consumer and one backend capability
   - MIR JSON transport after refresh

A3 closeout
   - literal, push, set, insert, alias, loop, and Any-boundary fixtures
   - unsupported backend rejects before effects
   - current MirType inference remains derived evidence only
```

### Branch B: Static Table Selected

```text
B1 closeout audit: LANGV1-STATIC-TABLE-CONTRACT-CLOSEOUT-001
   - prove element_type/value range under one verifier owner
   - prove JSON and backend consumers cannot bypass verified plan
   - add central capability/preflight only where a backend can drop semantics
   - do not add runtime element-write machinery
```

### Branch C: Weak Field Selected

```text
C0 prerequisite decision
   - settle weak clear/failed-upgrade absence representation

C1 corrective: LANGV1-WEAK-FIELD-CONTRACT-REFRESH-OWNER-001
   - replace MirType-as-proof with source-owned WeakFieldContract carrier
   - one write/publication owner and semantic_refresh rebuild
   - runtime check or fresh verifier proof before field publication
   - one VM consumer and backend capability gate
```

### Branch D: FFI Selected

```text
D0 remain at design stop
   - enumerate ingress/egress and final-provider resolution
   - select one FfiBoundaryContractOwner
   - do not activate FFI together with an in-language family
```

## Worker Recommendation

Do not select ordinary collections or FFI for the next implementation slice.
Static table is the cheapest closeout audit. Typed Array is the strongest next
semantic expansion but requires the explicit A1 BoxShape prerequisite. Weak
field should follow the absence/null decision or explicitly prove that its
first slice does not depend on that decision.

## Non-Claims

```text
next_type_family_selected = 0
typed_array_contract_activation = 0
static_table_contract_activation = 0
ordinary_collection_contract_activation = 0
ffi_contract_activation = 0
new_backend_contract_lowering = 0
runtime_check_elision_widened = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```
