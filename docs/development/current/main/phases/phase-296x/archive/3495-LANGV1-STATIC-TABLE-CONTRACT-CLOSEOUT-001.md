# 3495 - LANGV1-STATIC-TABLE-CONTRACT-CLOSEOUT-001

## Status

Complete. The readonly U16 Static Table contract is closed under one refreshed
owner. Commit: `c7d4b3e348`.

Decision: accepted implementation scope from 3494.

## Objective

Close the already-live readonly `u16[]` Static Table contract under one
`StaticTableElementContractOwner`, one source-owned spec, one refreshed typed
carrier, the existing VM load consumer, and one central backend capability.
Do not add syntax, element types, writes, or backend lowering.

## Existing Evidence

```text
ASTNode::StaticConstTable { name, element_type, values }
  -> static_data_plan::collect_static_data_plans_from_ast
  -> module.metadata.static_data_plans
  -> verification::module_metadata
  -> MirInstruction::StaticDataLoad
  -> MirInterpreter::handle_static_data_load
```

The parser, module verifier, and VM currently repeat parts of the shape law.
`semantic_refresh` does not carry Static Table, and central backend preflight
has no Static Table capability row.

## Structural Boundary

```text
source StaticConstTable declaration
  -> StaticTableContractSpec
  -> derived StaticDataPlan
  -> semantic_refresh StaticTableElementContractOwner
  -> VerifiedStaticTableContract
  -> verifier / MIR JSON / VM / backend preflight
```

`StaticDataPlan`, backend symbol, alignment, source path, plan index, and green
execution are derived evidence, not semantic identity.

## Identity And Carrier

```text
StaticTableId:
  module semantic identity
  declaration name

StaticTableContractSpec:
  table_id
  diagnostic name
  element = U16
  values = Vec<u16>

VerifiedStaticTableContract:
  table_id
  element = U16
  len
  derived plan symbol
  proof = SourceSpecAndPlanStructurallyMatch
```

Do not introduce a backend-symbol identity or a second declaration allocator.
The first slice rebuilds and structurally validates the carrier each refresh;
it does not add an epoch/hash subsystem.

## Ordered Tasks

1. Add source-owned `StaticTableId` and `StaticTableContractSpec` to module
   metadata beside, but distinct from, derived `StaticDataPlan`.
2. Publish spec and derived plan atomically from the source declaration
   snapshot. Keep initializer evaluation source-ordered and exactly once.
3. Make `StaticDataPlan` a projection from the accepted spec. Keep only U16.
4. Add `src/mir/type_contracts/static_table.rs` as the sole rebuild/validation
   owner and add its carrier to the semantic-refresh bundle/summary.
5. Validate duplicate identity, missing spec/plan, element/range/alignment/value
   drift, and every `StaticDataLoad` plan match under that owner.
6. Route direct verifier, MIR JSON, VM, backend preflight, and tools through the
   refreshed Static Table bundle. Local synthesis is forbidden.
7. Keep the existing VM `StaticDataLoad` as the sole runtime consumer. It may
   perform dynamic integer/bounds checks but must consume a refreshed carrier.
8. Add `StaticTableU16ReadonlyV1` to central backend capability preflight.
   Unsupported targets reject before effects; no fallback.
9. Export source spec, derived plan, and verified carrier through MIR JSON only
   after refresh validation.
10. Split mutable ledger completeness into source contract, single owner,
    semantic refresh, VM consumer, and backend preflight dimensions.
11. Add positive/negative fixtures and run focused verifier, JSON, VM,
    backend-preflight, current pointer, and changed-source-size checks.

## Stable Tags

Keep source/load tags:

```text
static-const/unsupported-element
static-const/value-out-of-range
static-const/unsupported-initializer
static-const/load-unsupported-element
static-const/load-index-out-of-range
static-const/load-missing-plan
static-const/load-plan-mismatch
```

Add owner tags:

```text
type/static_table_contract_duplicate_id
type/static_table_contract_spec_missing
type/static_table_contract_plan_missing
type/static_table_contract_carrier_missing
type/static_table_contract_drift
type/static_table_contract_refresh_bypass
type/static_table_contract_backend_unsupported
```

## Fixture Matrix

| Fixture | Expected |
| --- | --- |
| literal/const-expression/empty U16 table | refreshed carrier accepted |
| load | zero-extended Integer result |
| unsupported element/initializer/range | source fail-fast |
| duplicate identity | owner fail-fast |
| missing plan/spec/carrier | owner fail-fast |
| source-plan/load drift | owner fail-fast |
| direct verifier/JSON/VM bypass | refresh-bypass fail-fast |
| negative or OOB runtime index | stable load reject |
| unsupported backend | reject before effects |
| ArrayBox/MapBox lowering | structurally absent |

## Acceptance

```text
static_table_contract_owner_count = 1
static_table_source_spec_count_per_declaration = 1
static_table_derived_plan_authority = 0
static_table_semantic_refresh_complete = 1
static_table_vm_consumer_count = 1
static_table_backend_preflight_complete = 1
static_table_runtime_write_count = 0
static_table_additional_element_types = 0
new_semantic_activation = 0
changed_production_source_over_800_lines = 0
```

## Explicit Non-Claims

```text
typed_array_contract_activation = 0
weak_field_contract_activation = 0
ffi_contract_activation = 0
additional_static_element_types = 0
general_const_evaluator = 0
const_reference_or_const_fn = 0
runtime_element_writes = 0
arraybox_mapbox_static_table_lowering = 0
new_backend_contract_lowering = 0
runtime_check_elision_widened = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Next

After closeout, the next semantic expansion candidate is Typed `Array<T>`.
Its mandatory behavior-preserving prerequisite is
`ARRAY-ELEMENT-WRITE-BOUNDARY-OWNER-001`; do not combine that BoxShape work
with typed-array contract activation.

## Closeout Evidence

```text
cargo check --all-targets -q = green
Static Table owner tests = 5/5 green
Static Table backend capability tests = 1/1 green
Static Table MIR JSON carrier tests = 1/1 green
Static const parser tests = 6/6 green
Static Table VM E2E tests = 3/3 green
ProgramJSON source spec/plan transport tests = green
module metadata verifier tests = 5/5 green
LANGV1_GRAMMAR_FULL = final guard OK; 15/15 final-stage tests green
changed production source files >= 800 lines = 0
```

The broad `cargo test static` name filter also selects an unrelated existing
global-call route expectation test. That test fails identically in isolation
and is not claimed green by this card.
