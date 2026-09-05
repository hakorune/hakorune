---
Status: Active implementation ledger
Date: 2026-09-05
Scope: Mutable Language v1 type-contract activation, carrier, backend, and representation-consumer status.
Normative-Law: docs/reference/language/types.md
Code-Matrix: src/mir/type_contracts/guarantee_matrix.rs
Refresh-Owner: src/mir/semantic_refresh/contracts.rs
---

# Type Contract Status Ledger

This file records mutable implementation state. It does not define the meaning
of `x: T`; normative type semantics remain in `docs/reference/language/types.md`.

## Active Exact-Numeric Island

| Site | Current implementation | Carrier owner | Runtime/backend boundary |
| --- | --- | --- | --- |
| Box field write | verifier proof or runtime guard | `ExactNumericBoxFieldContract` | dynamic guard capability required |
| parameter entry | runtime checked | `FunctionEntryContractOwner` | final callee, before binding/effects |
| return exit | runtime checked | `FunctionReturnContractOwner` | final outcome, before caller publication |
| local init/reassignment | runtime checked | `LocalSlotContractOwner` | `LocalContractWrite`, before publication |
| record construction/update | exact-numeric fields runtime checked | `RecordValueContractOwner` | field check before `RecordValuePublish` |

All five families are rebuilt and validated by
`semantic_refresh::refresh_and_validate_for_boundary`. Runtime-check elision is
not active for parameter, return, or local contracts. Unsupported backends
must reject before effects.

## Remaining Annotation Sites

| Site | State | Next owner decision |
| --- | --- | --- |
| static table element | readonly U16 closeout in progress | `StaticTableElementContractOwner` |
| ordinary collection element | `Any` dynamic default | no typed activation |
| typed `Array<T>` element | seven-type exact-numeric state contract active on the reference VM | `TypedArrayElementContractOwner` |
| Weak field | runtime checked through a declaration-indexed weak slot | `WeakFieldContractOwner`; product backend remains deferred |
| FFI ingress/egress | transitional non-guarantee | dedicated FFI boundary decision |
| backend preservation | capability preflight | representation boundary only |

## Representation Consumer Inventory

The classification is an API/owner contract, not a claim that every current
consumer is already migrated.

| Family | Classification | Current owner or anchor | Rule |
| --- | --- | --- | --- |
| declared parameter/return/local/field types | semantic contract source | declaration metadata + site contract owners | may rebuild semantic carriers only in `semantic_refresh` |
| `FunctionSignature` / `MirType` / `value_types` | derived representation fact | MIR function/type metadata | routing/lowering input; never contract proof |
| exact-numeric value/return facts | derived verifier fact | exact-numeric fact owners | may optimize after a check; never replace one |
| runtime type tags/specs | runtime semantic observation | `runtime_type_tag`, `runtime_type_spec` | observe runtime values; do not infer source contract activation |
| declared-type storage selection | migration debt | `declared_type_storage`, record/storage plans | must derive through an accepted representation projection |
| packed-array source-type autouse | migration debt | packed-array autouse plan owners | source spelling must not become storage authority directly |
| route/call plans reading `MirType` | derived representation fact | route-plan owners | allowed only as conservative routing evidence |
| Rune plans | explicit plan input | rune refresh/verification owners | source `:T` cannot synthesize Rune authority |
| backend layouts | derived representation fact | backend capability/layout owners | cannot satisfy a missing semantic carrier |

## Integer field migration: bounded handoff

Decision: use existing `i64` contracts, not an IntegerBox-only field-read rule.
Law: [integer values](../../../../reference/language/types.md#integer-values-and-implementation-boxes). Execution and status
remain with the MirBuilder acceptance card; this section owns the type/backend
dependency, not a second scheduler. Read-type repair alone is not completion:
unchecked stores and tagless parameter carriers cannot prove Integer.

Read-only audit boundary: declaration -> FieldGet/Add type publication and
exact-numeric FieldSet/parameter contract -> published selected-C admission.
Includes ordinary scalar birth; excludes weak/FFI/general Box/other backends.
No runtime or Cargo verification was performed for this design review.

| State | Required boundary |
| --- | --- |
| `i64` with verifier-proven integer value | existing exact-numeric proof; Direct only with its valid value relation |
| `i64` with dynamic/unannotated parameter | existing type-check obligation; Checked consumer or reject before artifact |
| `i64`-annotated parameter | existing entry contract; selected-C currently rejects, no proof-elision waiver |
| IntegerBox/weak/untyped/foreign receiver | no scalar promotion from spelling, origin String or storage |
| missing/drifted/lost contract or unsupported consumer | pre-artifact reject; no retry and no destination mutation |

Ordered tasks inside the existing birth series:

1. Source migration: first Pair birth's two annotations -> `i64`. Existing
   parser mapping, `fields/post_success` and type propagation must publish
   Integer consistently for reads/Copy/Add/main return, including final refresh;
   preserve declaration identity, receiver evaluation and initialization order. Keep generic Box and
   `add_does_not_promote_unknown_plus_integer_to_integer` behavior unchanged.
2. Preserve both FieldSet obligations through semantic refresh and publication.
   Reuse `exact_numeric_field_contracts`; changing field spelling must not
   discard dynamic checks or presume unannotated birth parameters are integers.
3. Close selected-C operand-kind/contract consumption before enabling birth:
   use the existing published call/definition relation and capability preflight,
   not default `T_I64`, N+1 counts or symbol-derived birth parameter flow.
   Existing storage setters check storage/range, not Integer versus handle bits.
   No consumer may claim a dynamic type check without lossless value-kind data.
4. Align actual EXE and OBJ capability with that consumer. Currently dynamic
   FieldSet admits `ny-llvmc-exe` but the typed EXE path rechecks `ny-llvmc-obj`;
   annotated parameter entry accepts only `mir-interpreter`. Do not whitelist
   OBJ or add parameter annotations merely to bypass either missing consumer.
5. Verify missing/duplicate/drifted contract, wrong receiver/argument kind,
   String/Bool/BoxRef/Void (including handle bits that fit i64), unsupported
   EXE/OBJ with no artifact, and failed write preserving its destination.
   Extend existing tests; no per-row guard, fixture, or semantic receipt.
   Return directly to Birth Call/view -> EXE30 -> selected old-edge deletion.

If dynamic value-kind transport needs a new physical ABI, close that named
representation decision inside this series before code; do not insert a
general FieldGet identity system or recreate source semantics in C. Public
IntegerBox source-surface fate is a separate post-vertical decision; runtime/ABI
retirement is not authorized by this field migration.

### Adjacent follow-ups (read-only review, 2026-09-05)

Boundary: selected field declaration/write/read -> refresh -> construction
publication. Includes the existing field-origin and typed-object refresh owners;
excludes general GC/Home rollback, external callers, FFI and other backends.
The Pair source has two straight-line stores before reads. These three broader
tasks are queued after its vertical, not extra prerequisites for its annotation
migration. Reclassify a task as an in-scope blocker if the selected test exposes
that exact path; do not waive a demonstrated failure or reopen the broad census.

- [ ] **Preserve canonical result types through storage refresh** (existing D1).
  `src/mir/typed_object_plan.rs::refresh_function_typed_object_field_value_types`
  currently inserts Integer from storage; `semantic_refresh`/`route_fixpoint`
  call it. Preserve/check established declared result types, rather than
  overwriting them; separate compatibility missing-type inference. Before
  widening beyond matching i64 storage, test Bool/Box/other scalar conflicts
  plus i64 FieldGet/Copy/Add invariance. Delete the selected canonical overwrite
  edge; storage/layout is not semantic authority. General untyped inference is excluded.
- [ ] **Invalidate stale field origin on successful rebind** (existing field-facts owner).
  `builder/fields.rs::build_field_assignment_from_value_id` retains old origin on scalar stores;
  `field_facts.rs` class-wide fallback and `fields/post_success.rs` can reissue it.
  Before mixed Box/scalar or cross-instance field coverage, invalidate successful
  scalar/unknown stores and prevent exact scalar reads inheriting class-wide Box
  origin. Test Box->Integer->read, two instances, and failed-store state retention.
  Remove the selected stale reissue; class/name history is non-authority. Copy
  and LocalSSA already preserve established scalar types; no wholesale SSA rewrite.
- [ ] **Close uninitialized exact-field read/publication contract** (lifecycle owner).
  Use `constructor-birth-new-lifecycle-ssot.md` with its existing `OWN-HOME-BIRTH-D0`
  dependency. Ordinary runtime fields start Null; declared FieldGet type and
  FieldSet checks do not prove complete initialization. Before accepting reads
  before stores, conditional-only stores or early birth return, decide the exact
  rejection/publication boundary and test those cases. No zero-fill authority,
  backend-derived default, new field receipt, or general rollback implementation.

No initializer-order repair is selected: `property_emit::prepend_stored_field_initializers`
precedes constructor source seal, and `new_expression` applies explicit overrides
after birth, matching `lifecycle.md`. Current Pair acceptance claims only its
store-before-read sequence and exit 30, not all-Box initialization safety.

## Carrier Completeness

| Family | Source contract | Single owner | Semantic refresh | VM consumer | Backend preflight |
| --- | --- | --- | --- | --- | --- |
| static table readonly U16 | complete | complete | complete | complete | complete |
| typed `Array<T>` | complete for seven exact-numeric spellings | complete | complete | complete | complete; unsupported backends reject |
| Weak field | complete | complete | complete | complete | complete; unsupported backends reject |
| FFI | incomplete | incomplete | incomplete | provider-specific | incomplete |

## Known Debt Queue

```text
D1:
  queued typed-object refresh overwrite removal above; record-state projections separate

D2:
  audit packed-array autouse decisions that read declared source type names

D3:
  keep MirType route users conservative and prohibit semantic-proof promotion

D4:
  typed Array exact-numeric first slice closed in 3499; keep u64, pointer-sized,
  nested, alias, non-numeric, and non-VM support inactive
```

The null/void/Option relation, truthiness, equality compatibility, ownership,
and capability/effect rows are owned by their later Language v1 cards. They are
not redefined by this ledger.

## Implementation Anchors

| Concern | Anchor |
| --- | --- |
| guarantee matrix | `src/mir/type_contracts/guarantee_matrix.rs` |
| refresh facade | `src/mir/semantic_refresh/contracts.rs` |
| record value contract | `src/mir/type_contracts/record_value.rs` |
| static table contract | `src/mir/type_contracts/static_table.rs` |
| typed Array contract | `src/mir/type_contracts/typed_array.rs`, `src/boxes/array/runtime_contract.rs` |
| Weak field contract | `src/mir/type_contracts/weak_field.rs`, `src/runtime/weak_field.rs` |
| runtime type tags/specs | `src/backend/runtime_type_tag.rs`, `src/backend/runtime_type_spec.rs` |
| VM truthiness/equality | `src/backend/abi_util.rs` |
| MIR binary operations | `src/backend/mir_interpreter/helpers.rs` |

These paths are navigation evidence. Moving code does not change normative
semantics.
