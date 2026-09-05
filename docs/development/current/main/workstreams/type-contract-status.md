---
Status: Active implementation ledger
Date: 2026-09-06
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
   Reuse `exact_numeric_field_contracts`: unannotated parameters acquire no
   implicit Integer entry contract. Check each write at its source boundary;
   moving all checks before birth requires a separate ordering/elision proof.
   A later failed write must not erase earlier observable effects.
3. Close selected-C operand-kind/contract consumption before enabling birth:
   preserve exact source call -> formal binding -> FieldSet value relation in
   existing source/package owners. Only an active contract/fresh verifier proof
   permits Direct; loss of that relation across Copy/Phi/rebind or a foreign
   formal invalidates the proof, not a correctly preserved Copy by itself.
   Otherwise retain producer-issued kind for a Checked write, or reject before
   artifact. The bounded Birth wire-array direction below is accepted, not implemented.
   `OrdinaryScalar`, default `T_I64`, N+1, observed constant callers and storage
   setters are not Integer proof; reuse the published relation/capability gate.
4. Align actual EXE and OBJ capability with that consumer. Currently dynamic
   FieldSet admits `ny-llvmc-exe` but the typed EXE path rechecks `ny-llvmc-obj`;
   annotated parameter entry accepts only `mir-interpreter`. Do not whitelist
   OBJ or add parameter annotations merely to bypass either missing consumer.
5. Verify missing/duplicate/drifted contract, wrong receiver/argument kind,
   String/Bool/BoxRef/Void (including handle bits that fit i64), unsupported
   EXE/OBJ with no artifact, and failed write preserving its destination.
   Include first-write/effect -> later-write failure chronology and stale
   Copy/Phi/rebind/foreign-formal proof rejection; do not hoist checks by default.
   Extend existing tests; no per-row guard, fixture, or semantic receipt.
   Return directly to Birth Call/view -> EXE30 -> selected old-edge deletion.

If dynamic value-kind transport needs a new physical ABI, close that named
representation decision inside this series before code; do not insert a
general FieldGet identity system or recreate source semantics in C. Public
IntegerBox source-surface fate is a separate post-vertical decision; runtime/ABI
retirement is not authorized by this field migration.

Consumer premise audit (after 495d5fc9df): existing constructor/body owners
retain formal BindingRefs and FieldSet sites, but ordinary-new does not retain
actual argument sites as an actual -> formal -> write proof. Unannotated
formals therefore remain RuntimeCheckRequired. The C typed-object emitter
consumes storage plans, not ExactNumericRuntimeCheckContract or runtime kind.
That check contract describes the obligation, not the actual value's tag.
Review found no reusable selected-C runtime-kind carrier: MirValueKind is SSA
origin, PhysicalCallableLane is lane role, and TextScan's tags are family-private.
Neither a check obligation nor these classifications proves Integer.

Premise correction (two independent read-only reviews, 2026-09-05): prefer
bounded kind/payload transport, NOT whole-caller source-proof specialization.
`CallableSemanticSourceLedgerView::literal_source` already issues Integer
literal meaning, but ordinary_new_coseal covers selected direct initializers
only. The C function emitter has external linkage, and exact-numeric refresh
clears/rebuilds proof arrays with only ConstantInRange proof vocabulary.
Source-proof admission would therefore require all incoming callers AND an
external-entry boundary plus durable proof reconstruction; private linkage
alone is insufficient. FieldAccess proof expansion is not a Pair prerequisite.

Representation Decision: user accepted the bounded Birth transport direction
on 2026-09-05. Reuse only the
16-byte `DynamicV2WireValueV1` vocabulary from `src/abi/dynamic_call_slot_wire.rs`
and `include/nyrt_dynamic_call_slot_v2.h`: tag/reserved/payload. No Birth producer
or consumer exists for it yet. Dynamic invocation, Home, leases, suspension and
48-byte CallOut are NOT imported. No second tag enum or semantic receipt.

### Accepted Birth input ABI and remaining execution tasks

Decision: synchronous borrow-only wire-array, not expanded per-value machine
arguments or by-value aggregates. Canonical Call remains receiver + N source
ValueIds; `InstanceConstructorAbiV1` continues to validate N/N+1 canonical
parameters. Selected generated internal Birth has three source-input ABI lanes:
`receiver: i64, argv: *const DynamicV2WireValueV1, argc: u32`.
Each of N rows is 16 bytes, aligned to 8. The accepted common Invoke ABI adds
a hidden borrowed Fault-frame input and returns per-call Normal/Fault status;
Unit Birth has no normal-result out slot. These hidden lanes do not change
source arity or canonical N/N+1; the constructor lifecycle SSOT owns their layout.

The existing published C transport owner issues one physical descriptor from
the published key/definition: source arity, formal ordinal/ValueId relation,
wire revision 2 and `BorrowedWireArrayV1` input kind. This label denotes a
physical projection, not a new semantic receipt. Calls and definitions consume
the same descriptor. Any incompatible C frame layout uses a versioned ingress;
never change a V1 struct under the same ABI symbol or retry its old entry.

Compile-time Rust frames are borrowed only during the compiler callback;
generated runtime argv is separate caller-owned stack storage, immutable and
borrowed only during the synchronous Birth call. It cannot escape/store/return.
HostHandle payload owners stay alive across that borrow; no new retain/lease
authority. Normal and Fault both end the borrow. Bound stack allocation per
function activation rather than accumulating an alloca on each loop iteration.
For N=0, no row may be dereferenced; N, length and overflow are checked explicitly.

These are checklist steps in the existing series, not new scheduler rows:

| Step | Change and existing owner | Required completion evidence |
| --- | --- | --- |
| 0 — accepted | input ABI above; `instance_constructor_abi`, published C transport | source N / MIR N+1 / three source-input ABI lanes plus hidden Fault frame remain distinct; Unit has no result-out slot |
| 1 — next | connect the accepted common-exit control/operand contract below | per-cutpoint ownership and explicit Normal/Fault control survive finalization/SSA; runtime cleanup proof is the series terminal, not a prerequisite for steps 2–4 |
| 2 | retain producer-issued kind/payload through existing Lower/ordinary-new and formal binding owners | each pair has one actual/ordinal; Copy copies both, rebind replaces both, Phi uses identical predecessor mapping; unsupported producers reject |
| 3 | published view/C transport + `capi_transport.rs` | versioned descriptor/row agreement, borrow lifetime, exact target/formal/ordinal; missing/foreign/duplicate/old ABI rejects before emission |
| 4 | selected same-module Birth body owner + exact-numeric FieldSet consumer | internal wire-array entry, runtime pair loads, check at each store, existing physical store only after success; Fault from step 1 never joins normal object publication |
| 5 | existing typed CLI/EXE/OBJ admission and fixed Pair acceptance | real EXE exit 30, OBJ validation/execution, negative chronology, no retry, selected tagless entry/projection callers zero and physical deletion |

Step 1's accepted design, phase matrix and four ordered tasks now have one owner:
[failed construction](../design/constructor-birth-new-lifecycle-ssot.md#failed-construction-minimal-integration-contract).
User approved this design/task scope on 2026-09-05; no repeated scope or input-ABI
approval is pending. Common exit owns locals/temporaries/outer propagation;
construction owns committed fields/native payload and the unpublished receiver.
Only verified Home demand transfers ownership; ordinary copies and borrowed
wire inputs do not. This replaces the earlier scalar-reclaim-only premise.

Implementation gaps remain explicit: `VerifiedFunctionCompletionV1` alone
does not prove the New Fault cutpoint. The existing `ordinary_new_coseal`
ledger now carries source Home prefixes, NewFaultContinuation and exact local
completion, but has not materialized their physical cleanup/control operands.
`typed_object_store_backend` has no unpublished discard API. Its index handles
must not move during reclaim; `host_handles::drop_handle` is a different registry.
TextScan CheckedCallOut, setter 0/1 status, raw alloc/free and bare llvm.trap are
not the missing semantic exit owner. A prior live caller object is a required
negative witness, not grounds for a Pair-only or assumed-empty admission.
Constructor tasks 1–4 and these steps are one connected series, not two nested
completion gates. After the control/operand mapping checkpoint, proceed through
kind/payload and published transport to the typed consumer while connecting
construction reclaim. Full runtime cleanup/propagation is proved at step 5;
do not require that proof before implementing its steps 2–4 dependencies.
No broad Home/GC/syntax activation, early executable admission or ABI-only
series closeout is permitted.

Producer contract: ImmediateI64 comes from an existing source/canonical Integer
producer, HostHandle only from a real live host-handle carrier. Never re-tag
Bool/Float/Void/raw object bits as handles or infer kind from storage/name/bit
patterns. The first supported producer inventory is finite and independent of
Pair's name; unsupported producer/merge coverage rejects before artifact.

| Failure boundary | Expected result |
| --- | --- |
| compile-time missing/foreign/drifted descriptor, unsupported kind producer or legacy ABI | reject, no artifact, no old ingress retry |
| malformed runtime wire: Invalid (even zero payload), unknown tag, reserved or arity mismatch | selected contract failure before body/store, cleanup/no-object-result per step 1 |
| valid HostHandle reaches an i64 FieldSet | runtime type Fault immediately before that write; earlier writes/effects stay ordered; no successful object result |
| Birth Normal Unit | continue the new transaction; publish once only after overrides and the complete transaction succeed; never expose store status or synthetic Birth result |

Acceptance extends existing ABI/package/view tests and the fixed Pair proof:
N=0/N>0, equal Integer/handle bits, swapped ordinals, stale Copy/Phi/rebind,
second-write-only mismatch, borrow end on both exits, failed-store destination
unchanged, and EXE/OBJ capability agreement. Use existing test owners; an
over-760-line owner must split by responsibility before growth, all source <=800.
No baseline rewrite, ignored tests, new guard or new fixture file. Retarget or
delete superseded cohort-only tests only after replacement coverage/caller checks.

Until this contract is closed, UnsupportedBeforeObject remains. This is a
physical transport decision, not permission to change unannotated parameters
into i64 source contracts or widen to all backends/types. The Global/manual
prefix/fixed-IO edges removed at 495d5fc9df are prior progress, not a new delete-set.

### Birth result/effect handoff (next consumer prerequisite)

- [x] Constructor source retention: reuse `verify_function_completion_v1` / `VerifiedFunctionCompletionV1`
  at the existing constructor semantic row, bound to its exact source ID/key;
  check its owner at `with_instance_constructor_lowering_input`; recipe/view borrowing remains open. The normal-callable
  result cohort is keyed differently and is not a constructor lookup authority.
- [x] Preserve resolver-issued body shapes through `instance_constructor_semantic`
  and its lowering input, including nested function owners and residual checking.
  `effects()` contains source-site event kinds, NOT a complete semantic effect/
  failure contract. Never infer that contract from these events, `EffectMask::IO`,
  signature or physical lanes. This corrects the initial handoff-only premise.
- [x] Birth Call effect disposition: source-issued OpaqueObservable for the
  unannotated constructor contract. Existing method-effect issuers are
  different cohorts; do not transplant them by name or classify all events as IO.
  Decision (worker-reviewed): existing OpaqueObservable vocabulary is co-sealed
  by the Birth recipe with exact source/key and Unit Completion. The recipe's
  explicit physical policy projects all currently defined observable barrier
  bits, excluding Pure; this is not body-effect inference or NoFailure proof.
  Inject once into canonical Birth Call; no analyzer/READ/IO overwrite. Exact
  write-site coverage and FieldSet failure/operand transport remain separate.
  Reject Pair-name or exactly-two-writes production admission; two is the proof's
  expected count, not semantic authority. No synthetic method identity or new receipt.
- [x] `ordinary_new_coseal` now checks source/key, Unit Completion and effect;
  selected Birth Call keeps the mandatory receiver separate from source args.
  Selected Global reconstruction, manual prefix and fixed IO are removed.
  First cohort accepts verified implicit Unit or explicit Void separately;
  value-returning birth, absent/foreign/duplicate effect, source/key/root drift
  and missing Completion reject before publication. `dst=None` is not Unit proof.
  Verify existing product field/access mapping before implementation; no new
  semantic receipt, second resolver or widening to other constructor families.
  FieldSet chronology/operand-kind and EXE/OBJ obligations above remain independent.

Source-retention checkpoint: package 74/74 at d582dc223c. Current Birth Call
consumer is still UnsupportedBeforeObject; typed C activation, exact write-site
proof/kind consumption, failure chronology and EXE30 remain open. A valid MIR
Call and conservative physical mask do not prove executable construction.

### Adjacent follow-ups (read-only review, 2026-09-05)

Execution order (task registration, not implementation permission): finish the
existing Birth result/effect, operand-kind and EXE/OBJ obligations first, then
Pair EXE30 plus selected old-edge deletion. The following queue reuses existing
owners; it adds no new lane, receipt, guard or broad audit prerequisite.

| Priority | Existing task | When to select | Observable finish line |
| --- | --- | --- | --- |
| Current vertical | Birth contract/physical consumer handoff above | before enabling Birth execution | exact source products preserved, write checks consumed in order, unsupported input produces no artifact, Pair exits 30, selected old edges removed |
| Follow-up 1 | D1 storage-refresh overwrite removal | after the vertical; earlier only on its demonstrated failing path | canonical declared result survives refresh; selected storage-to-semantic overwrite removed |
| Follow-up 2 | successful-rebind field-origin invalidation | before mixed-value/cross-instance field coverage | stale origin cannot reappear; failed store retains prior state |
| Follow-up 3 | uninitialized exact-field lifecycle contract | before conditional/early-return/uninitialized-read coverage | one explicit initialization/publication decision and positive/rejection evidence; no fabricated default |
| Conditional | canonical missing-type diagnostic below | selected boundary failure or next owner change | quick/release agree on rejection; supported Dynamic is preserved |

Do not count these queued items as fixed. Each implementation closes its own
selected tests and old edge, then returns to the MirBuilder acceptance card.
IntegerBox public-surface/runtime retirement and general Home rollback remain
separate scope decisions, not hidden prerequisites for this scalar vertical.

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

### Canonical missing-type diagnostic follow-up

- [ ] Owner: `builder/return_type_strategy.rs`, selected canonical finalization.
  Quick currently panics where release warns and returns Unknown. Distinguish
  legitimate Dynamic/unannotated contracts from missing required result facts;
  the latter must reach one explicit rejection before canonical publication in
  both profiles, never a fabricated Integer or an Unknown success allowance.
  Trigger: next change to this boundary, or a remaining selected gate failure
  after i64 migration. Do not preempt the migration with a global type-system rewrite.
  Done: same malformed selected input rejects in quick/release without artifact;
  legitimate supported Dynamic remains unchanged, scalar result succeeds, and
  the selected warn-and-continue edge is removed. Reuse existing tests/diagnostics;
  no baseline rewrite, per-profile fallback, or new guard. Runtime evidence pending.

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
