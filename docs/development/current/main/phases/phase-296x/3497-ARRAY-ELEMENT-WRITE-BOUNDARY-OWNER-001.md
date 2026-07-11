# 3497 - ARRAY-ELEMENT-WRITE-BOUNDARY-OWNER-001

## Status

Active implementation card. Decision: accepted scope from 3496.

## Objective

Converge every accepted Array mutation onto one explicit canonical MIR
operation and one owner without changing acceptance, runtime behavior, or
Typed `Array<T>` contracts.

```text
source producers / legacy imported Call
  -> ArrayElementWriteOwner
  -> MirInstruction::ArrayElementWrite
  -> refreshed ArrayElementWriteWitness + ArrayStateTerm
  -> planner references by ArrayWriteSiteId
  -> VM direct consumer or validated backend projection
```

Runtime mutable state is `Arc<ArrayStateCell>` containing an opaque
`ArrayStateIdentity` and `RwLock<ArrayStorage>`. Raw identity values, Arc
pointers, Box IDs, storage variants, and `MirType` never cross MIR JSON or
become semantic authority.

## Canonical Vocabulary

```text
ArrayElementWriteKind = LiteralAppend | Push | Set | Insert

ArrayElementWrite:
  site_id: ArrayWriteSiteId
  dst: Option<ValueId>
  kind: ArrayElementWriteKind
  receiver: ValueId
  index: Option<ValueId>
  value: ValueId

ArrayElementWriteWitness:
  site_id, kind, receiver/index/value, state_term
  producer = Literal | MethodCall | IndexAssignment
           | CompoundIndexAssignment | LegacyCanonicalized
```

LiteralAppend/Push require no index. Set/Insert require one. Mutation succeeds
before optional Void `dst` publication.

## Identity Law

```text
fresh literal/new -> Fresh
deep clone -> Fresh
share_box/ordinary alias/Copy -> SameAs
parameter/field/return -> preserve runtime token
PHI/Select/loop -> Select(inputs)
unknown dynamic crossing -> DynamicBoundary
storage promotion -> preserve
```

Different PHI inputs are legal. They prohibit same-state optimization claims
but do not prohibit the write.

## Refactor Series Mode

One card, at most five buildable commits. BoxShape only.

### S1 - Vocabulary And Observation

Add kind/site/op plus typed instruction methods, remap, query, printer, JSON,
owner README, and stable tags. Existing runtime Call behavior remains until S3.

### S2 - Runtime State And VM

Add opaque shared-state identity and prove new/deep-clone freshness plus
share/alias preservation. Add one VM executor that delegates to existing
`ArrayMethodId`/`invoke_surface`; do not duplicate storage policy.

### S3 - Producer Convergence

Route literal append, ArrayBox push/set/insert, index assignment, compound
index final write, and statically-resolved legacy/imported Calls through the
owner. Preserve evaluation order and exactly-once behavior. User-defined
`.push()` remains generic unless runtime dispatch resolves ArrayBox.

### S4 - Identity Refresh And Planner Migration

Rebuild state terms after CFG/SSA mutation for Copy, PHI, Select, loop,
parameter, field, and return boundaries. Migrate generic/direct/RMW/text and
micro-seed planners from raw names to site ID/kind. Replacement rows carry
typed covered-site operand parity.

### S5 - Projection, Verifier, Closeout

Require zero known residual Array write Calls and one witness per op. Validate
shape, identity, planner coverage, operands, overlap, and one-to-one projection.
Backend mode is NativeV1, ValidatedLegacyCallProjectionV1, or Unsupported.
Unsupported rejects before effects without VM fallback.

## Planner Inventory

```text
generic_method_route_plan/write_routes
direct_array_access_plan
array_rmw_window_plan
array_text_edit_plan
array_getset_micro_seed_plan
array_string_store_micro_seed_plan
```

Raw Array write method matching is allowed only in `ArrayMethodId`, the owner
legacy canonicalizer, runtime dynamic dispatch, and focused tests.

## Stable Tags

```text
mir/array_write/unclassified_surface
mir/array_write/residual_call
mir/array_write/invalid_shape
mir/array_write/identity_missing
mir/array_write/identity_drift
mir/array_write/representation_as_identity
mir/array_write/planner_bypass
mir/array_write/covered_site_drift
mir/array_write/overlapping_selected_routes
mir/array_write/projection_drift
mir/array_write/backend_unsupported
mir/array_write/raw_runtime_bypass
```

Existing OOB and method error tags remain unchanged.

## Required Fixtures

```text
literal/push/set/insert order and publication
index assignment and compound evaluated Place
Copy/alias/PHI/loop/parameter/field/return identity
share preserves identity; deep clone is fresh
set append/OOB, insert, push-promotion parity
direct/RMW/text planner site references
VM consumer, MIR JSON, legacy projection parity
residual Call, invalid shape, missing/stale identity
planner raw match, missing/drifted/overlapping site
Box ID/MirType/storage identity misuse
missing/duplicate projection, raw helper bypass, backend reject
```

## Acceptance

```text
array_element_write_owner_count = 1
known_array_write_residual_call_count = 0
planner_raw_array_write_method_match_count = 0
array_write_vm_consumer_count = 1
array_write_legacy_projection_owner_count = 1
array_write_identity_witness_complete = 1
share_preserves_array_state_identity = 1
deep_clone_preserves_array_state_identity = 0
typed_array_contract_activation = 0
changed_production_source_over_800_lines = 0
```

## Explicit Non-Claims

```text
typed_array_contract_activation = 0
source_owned_array_element_contract = 0
runtime_array_element_type_check = 0
runtime_check_elision_widened = 0
new_array_acceptance = 0
backend_array_lowering = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Stop Conditions

Return to design if shared-state identity changes clone/share semantics, or if
a planner still requires raw method recognition after write-site references
are available.
