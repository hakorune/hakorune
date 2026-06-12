---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: Type ABI view boundary, TypeAbiPack snapshot role, and PlanStamp task order.
Related:
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/abi/ABI_INDEX.md
  - docs/reference/plugin-abi/nyash_abi_v2.md
  - docs/development/current/main/design/type-abi-box-domain-ssot.md
  - docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# Type ABI View And PlanStamp SSOT

## Decision

Type ABI is a read-only view over existing domain truth. It is not a new
execution ABI and not a new owner of type, method, field, layout, string, or
GUI facts.

```text
Type ABI:
  explain existing truth

TypeAbiPack:
  generated cold snapshot

Plan:
  execute selected route

PlanStamp:
  mark plan freshness
```

The canonical external ABI surfaces remain fixed:

```text
1. Core C ABI
2. TypeBox ABI v2
```

Type ABI vNext must not become a third canonical ABI surface. If a C cursor is
needed later, it is a cold descriptor cursor sub-surface, not a hot execution
ABI.

## Ownership

Existing domain truth remains the source of authority:

```text
call / method truth:
  type_registry / TypeBox MethodEntry

box lifecycle / plugin route truth:
  PluginLoader route resolver contracts

typed field truth:
  typed-object plans and exact slot route decisions

memory layout truth:
  FastMem layout/table contract and VerifiedMemAccessPlan

string route truth:
  StringKernelPlan / string corridor plans

future GUI truth:
  GUI domain component facts
```

Type ABI may encode those facts for tools and cold diagnostics, but it must not
own duplicate descriptor state.

Forbidden as new truth:

```text
generic TypeDescriptor replacing domain facts
generic MethodDescriptor replacing MethodEntry
generic FieldDescriptor replacing field / typed-object plans
generic LayoutDescriptor replacing FastMem contracts
generic ComponentDescriptor replacing GUI domain truth
```

## TypeAbiView

Use one thin common trait first. Add domain extension traits only when a domain
needs a stable helper API.

```rust
pub trait TypeAbiView {
    fn abi_tag(&self) -> TypeAbiTag;
    fn abi_id(&self) -> u32;
    fn abi_name(&self) -> Option<&str>;
    fn payload_schema(&self) -> u16;
    fn encode_payload(
        &self,
        out: &mut TypeAbiPayloadSink,
    ) -> Result<(), TypeAbiError>;
}
```

Rules:

```text
common trait stays thin
domain details stay in the domain
adapter reads existing truth
adapter does not mutate or reclassify truth
```

The first implementation target is `type_registry::MethodEntry`, because it
already carries `name`, `arity`, and `slot`.

## TypeAbiPack

`TypeAbiPack` is generated, read-only, and discardable.

```text
truth:
  existing domain data

pack:
  encoded snapshot of existing domain data

cursor:
  query surface over the snapshot
```

`TypeAbiPack` must not be used as the source that planners or lowerers consult
for hot decisions.

Allowed readers:

```text
hako_check
report generation
Python or tooling introspection
manifest validation
cold provider capability negotiation
debug dump / inspect bundle generation
```

Forbidden readers:

```text
field load hot path
method call hot path
string kernel hot path
allocator replacement front
provider hot dispatch
GUI frame inner loop
```

## Optional C Cursor

If a C surface becomes necessary, keep it to three operations:

```c
int nyrt_type_abi_load(
    const uint8_t* data,
    size_t len,
    NyrtTypeAbiCursor* out
);

int nyrt_type_abi_query(
    const NyrtTypeAbiCursor* cursor,
    NyrtTypeAbiKey key,
    NyrtTypeAbiEntry* out
);

void nyrt_type_abi_drop(
    NyrtTypeAbiCursor* cursor
);
```

Do not add domain-specific C functions such as:

```text
register_type
register_method
register_field
get_field_offset
resolve_method
```

Domain growth adds tags and payload schemas, not new C functions.

Enumeration must also stay inside `query`. Do not add a fourth C function for
iteration.

Allowed query-key shapes:

```text
ById
ByName
DomainIndex
EntryAt
```

Equivalently, an index entry may be encoded as a normal tagged entry:

```text
tag=INDEX
id=<domain>
payload_schema=index_v0
```

## PlanEnvelope And PlanStamp

`PlanStamp` is the plan freshness vocabulary. Do not call it a plain `epoch`,
because runtime handle caches already use `drop_epoch` for a different
invalidation domain.

```rust
pub struct PlanEnvelope {
    pub site_id: SiteId,
    pub domain: DomainId,
    pub source_span: SourceSpan,
    pub fallback_policy: FallbackPolicy,
    pub stamp: PlanStamp,
}

#[repr(transparent)]
pub struct PlanStamp(pub u64);
```

V0 meaning:

```text
PlanStamp = compile-session epoch
```

Future meanings may become more precise without changing the envelope field:

```text
v1:
  domain epoch

v2:
  stamp table id
  truth hash
  dependency domain set
```

## Hot Path Rule

Type ABI lookup and plan freshness checks stay out of hot inner loops.

```text
Type ABI query in hot path:
  forbidden

PlanStamp check in hot inner loop:
  forbidden

AOT:
  stamp is report / validation metadata
  runtime hot check = 0

JIT / REPL / hot reload:
  check at plan lookup or dispatch cache lookup
  do not check every field load or method call
```

Required report vocabulary:

```text
type_abi_mode=view_over_existing_truth
type_abi_pack_is_truth=0
type_abi_new_duplicate_descriptor_count=0
type_abi_c_api_function_count=0|3
type_abi_hot_lookup_count=0
type_abi_query_hot_path_count=0
type_abi_debug_lookup_count
type_abi_query_phase=planning|reflection|debug|hot

type_abi_view_adapter_count
type_abi_pack_generated_count
type_abi_pack_source_hash
type_abi_pack_entry_count
type_abi_pack_schema_version

plan_envelope_stamp_enabled=1
plan_stamp_mode=compile_session_epoch
plan_stamp_domain_epoch_enabled=0
plan_stamp_truth_hash_enabled=0
plan_stamp_hot_loop_check_count=0
plan_stamp_debug_check_count
plan_stale_detected_count
plan_regenerated_count
plan_fallback_due_to_stale_count
```

Domain source reports:

```text
type_abi_domain[call].truth_source=type_registry
type_abi_domain[field].truth_source=typed_object_plan
type_abi_domain[memory].truth_source=fastmem_access_plan
type_abi_domain[string].truth_source=string_kernel_plan
type_abi_domain[gui].truth_source=gui_domain
```

## Relation To Optimization Work

Current optimization work should not wait for Type ABI.

Use this order:

```text
1. perf identifies owner
2. domain planner selects route
3. verifier proves route
4. lowering consumes selected plan
5. report/hako_check exposes evidence
6. Type ABI may describe the selected truth for cold tools
```

Do not route performance work through Type ABI. For example:

```text
typed-object exact slot:
  RouteDecision remains execution truth
  Type ABI may describe MethodEntry / slot metadata later

fastmem:
  VerifiedMemAccessPlan remains lowering truth
  Type ABI may encode layout/table descriptors later

string kernel:
  StringKernelPlan remains route truth
  Type ABI may describe string route metadata later
```

The existing Type ABI route descriptor plane remains a valid allocator/provider
descriptor application of this rule:

```text
descriptor/control plane:
  Type ABI

execution plane:
  Provider ABI / replacement front / selected domain plan
```

## Invariants

```text
Type ABI is not a semantic owner.
TypeAbiPack is generated and discardable.
Domain truth remains vertical.
Hot path never queries TypeAbiCursor.
PlanStamp is checked only at plan/cache boundaries.
```

Diagnostic builds may add explicit debug checks, but those checks must be
reported separately from product or keeper paths:

```text
TYPEABI_DEBUG_STAMP_CHECK=1
plan_stamp_debug_check_count=N
plan_stamp_hot_loop_check_count=0

type_abi_debug_lookup_count=N
type_abi_hot_lookup_count=0
```

## Task Ladder

### TYPEABI-VIEW-000

Docs-only boundary.

Status: landed 2026-06-13.

Acceptance:

```text
Type ABI is documented as view, not truth
canonical ABI surface stays Core C ABI + TypeBox ABI v2
hot path Type ABI lookup is forbidden
```

### TYPEABI-STAMP-000

Add `PlanStamp` / `PlanEnvelope` vocabulary.

Acceptance:

```text
PlanStamp exists as distinct vocabulary from drop_epoch
v0 meaning is compile-session epoch
hot loop checks remain forbidden
```

### TYPEABI-VIEW-001

Add `TypeAbiView` skeleton and payload sink/error stubs.

Status: landed 2026-06-13.

Acceptance:

```text
trait is read-only
no domain truth is moved
no C API is added
```

### TYPEABI-VIEW-002

Add `type_registry::MethodEntry` adapter.

Status: landed 2026-06-13.

Acceptance:

```text
adapter reads name / arity / slot from MethodEntry
type_abi_domain[call].truth_source=type_registry
duplicate method descriptor count remains 0
```

The v0 adapter uses `MethodEntry.slot` as its local `abi_id`.
Type-qualified method ids belong to the later TypeBox / pack layer.

### TYPEABI-VIEW-003

Add in-memory query smoke before introducing pack bytes.

Status: landed 2026-06-13.

Acceptance:

```text
MethodEntry can be read through a TypeAbiEntryView-like API
no TypeAbiPack bytes are required
no C API is added
view adapter count is reported
```

The initial code surface is `TypeAbiEntryHeader::from_view`.
It is a transient cold query result and must not become domain truth.

### TYPEABI-PACK-000

Add internal Rust pack builder.

Status: landed 2026-06-13.

Acceptance:

```text
existing truth encodes to TypeAbiPack bytes
TypeAbiPack is documented and reported as snapshot
planners/lowerers do not consume pack for hot decisions
```

The v0 pack builder is internal Rust code only. It encodes `TypeAbiView`
entries into a generated snapshot and does not add a C cursor or planner
consumer.

### TYPEABI-CURSOR-000

Add optional C cursor only when an external consumer needs it.

Acceptance:

```text
C surface has only load / query / drop
canonical ABI matrix still has only two external ABI surfaces
hot path lookup count remains 0
```

### FIELD-DOMAIN-000

Add field domain adapter after typed-object exact route work needs descriptor
output.

Acceptance:

```text
field truth source remains typed-object plan / exact slot RouteDecision
Type ABI describes selected field route only
no field offset lookup is performed through Type ABI in hot lowering
```

### MEMORY-DOMAIN-000

Add memory domain adapter after FastMem verified plans need descriptor output.

Acceptance:

```text
memory truth source remains FastMem layout/table contract
Type ABI encodes verified layout/table descriptors only
lowering consumes VerifiedMemAccessPlan, not TypeAbiPack
```

### GUI-DOMAIN-000

Add GUI domain when GUI component truth exists.

Acceptance:

```text
GUI domain owns component truth
Type ABI describes GUI truth as cold metadata
GUI frame hot path reads WidgetPlan / EventRoutePlan, not Type ABI
```

## Stop Lines

```text
new canonical ABI surface added
TypeAbiPack used as planner truth
domain truth duplicated into generic descriptors
Type ABI query appears in a hot loop
PlanStamp check appears per field/method operation
existing TypeBox ABI v2 plugin dispatch semantics are widened by Type ABI
```
