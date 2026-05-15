# MIR Metadata Facts (SSOT)

Status: Canonical for emitted MIR metadata
Primary sources:

- `src/mir/function/types.rs`
- `src/mir/semantic_refresh.rs`
- `src/runner/mir_json_emit/root.rs`
- `src/runner/mir_json_emit/metadata.rs`
- `src/runner/mir_json_emit/decls.rs`
- `src/mir/printer.rs`

This document covers inspection-only metadata emitted in MIR JSON. These facts
do **not** create a second MIR dialect. They annotate canonical MIR so backends
and diagnostics can make placement/entry decisions without guessing from helper
names.

## Metadata Classes

`metadata` is not one semantic bucket. Rows must belong to one of these classes
so ownership and retirement stay visible:

| Class | Owner | Meaning | Runtime effect | Backend use |
| --- | --- | --- | --- | --- |
| `SourceAttrs` | Parser / Stage0 / Stage1 transport | Declaration-local source facts such as runes, declared signatures, user boxes, records, and enums | none by itself | no direct lowering unless a later MIR plan consumes it |
| `SemanticFacts` | MIR semantic refresh | Facts derived from canonical MIR values, calls, loops, types, effects, and exact numeric observations | none by itself | verifier/backend may consume typed facts without rediscovering from helper names |
| `Contracts` | Verifier / fail-fast owners | Obligations, prohibitions, capabilities, and guarantees used to accept or reject behavior | verifier/fail-fast only unless promoted by a route | backend sees only derived routes, not raw contract source |
| `LayoutPlans` | MIR layout owners | Typed object, record, static data, and packed-column layout truth | none unless a runtime/backend row explicitly consumes it | backend may consume active rows; inactive rows must remain metadata-only |
| `PlacementPlans` | MIR placement/effect owners | Objectization, materialization, residence, escape, and publication boundary decisions | none until a transform/lowering row consumes the plan | prefer generic `placement_effect_routes` over family-specific rows |
| `LoweringRoutes` | MIR route planners | Backend-facing route, symbol, proof, value-demand, and effect rows | no source/MIR rewrite | backend may emit from these rows and must not reclassify raw helper names |
| `DiagnosticsMetadata` | Builder/MIR diagnostics owners | Source spans, origin callers, debug/provenance data | none | diagnostics only |
| `ExperimentalSeedRoutes` | Narrow proof owners | Temporary exact-shape payloads for current micro/proof bridges | no canonical MIR replacement | backend may select a guarded emitter; each row needs a retire condition |

New rows should document:

```text
key:
class:
owner:
producer:
consumer:
state:
backend_active:
fallback_allowed:
coreplan_promotion:
retire_condition:
```

## Row State Vocabulary

Each row should expose its activation state. Use these states instead of prose
such as "maybe active":

| State | Meaning |
| --- | --- |
| `transport_only` | Source or declaration data is carried forward, but no meaning is decided at this layer |
| `inspection_only` | MIR/compiler records an observation or candidate; no verifier, optimizer, runtime, or backend behavior changes |
| `semantic_layout_truth` | MIR owns a layout or declaration-derived semantic truth, but execution still needs an explicit consumer |
| `verifier_active` | The row can accept/reject behavior or produce fail-fast diagnostics |
| `optimizer_active` | A MIR/CorePlan transform may consume the row |
| `backend_active` | Backend emitters may lower from the row |
| `runtime_active` | Runtime behavior/storage changes are active |
| `retired` | The row is kept only for historical compatibility or should no longer be emitted |

## Naming Contract

Metadata suffixes carry meaning:

| Suffix | Meaning | Backend may consume directly? |
| --- | --- | --- |
| `*_decls` | Source declaration inventory | no, except legacy declaration handoff |
| `*_facts` | Facts observed or derived from canonical MIR | no direct lowering; a plan/route should consume them first |
| `*_contracts` | Verifier/fail-fast obligations or runtime-check contracts | no direct lowering; backend sees accepted routes |
| `*_plans` | Compiler-owned plan, candidate, selected layout, or future placement | only if explicitly marked `backend_active` |
| `*_routes` | Lowering or consumer route with proof/demand/effects | yes when `backend_active` |
| `*_seed_route` / `*_micro_seed_route` | Temporary exact-shape proof bridge | yes only as `ExperimentalSeedRoutes` with a retire condition |

Plan and route are intentionally different:

```text
plan:
  compiler-owned plan/candidate/selection; may still be metadata-only

route:
  backend/VM-consumable lowering contract with proof, demand, effects, and
  fallback policy
```

## Stage Boundary Contract

Stage0 metadata is transport only.

Allowed Stage0 payload:

- `@rune` declarations;
- type annotation text;
- brand declaration text;
- `requires` / `ensures` / `invariant` source text;
- transition source relations;
- `uses` capability names;
- delegate exposes lists.

Forbidden at Stage0:

- layout decisions;
- packed eligibility;
- record materialization decisions;
- capability legality;
- contract insertion;
- backend routes;
- optimizer routes.

The intended pipeline is:

```text
Stage0 metadata = transport
Stage1 metadata = meaning / facts / contracts / plans
CorePlan metadata = placement / lowering decision
Backend metadata = route consumption
```

## Record And PackedArray Metadata Split

Record and PackedArray rows are split by responsibility:

```text
RecordSpec Metadata:
  record_decls
  record_layout_plans

PackedResidence Metadata:
  array_record_storage_plans
  array_record_autouse_eligibility_plans
  array_record_materialization_boundary_plans
  array_record_packed_autouse_pilot_plans
  source_packed_array_autouse_pilot_plans
  source_packed_array_direct_read_consumption_plans

AllocatorPackedStore Pilot:
  hako_alloc_aligned_small_packed_store_pilot_plans
  hako_alloc_huge_page_packed_store_pilot_plans
```

`RecordSpec Metadata` is closest to language semantics. `PackedResidence
Metadata` and `AllocatorPackedStore Pilot` are staged compiler/runtime/backend
bring-up rows and must keep materialization, backend lowering, and boxed
fallback state explicit.

## CorePlan Promotion Criteria

Keep metadata as metadata when it is diagnostic provenance, inspection-only
facts, candidate inventory, debug trace, or temporary proof inventory.

Promote a row toward `Contracts`, `PlacementPlans`, `LoweringRoutes`, or
CorePlan ownership when it:

- supplies fail-fast evidence;
- decides backend lowering availability;
- encodes a silent-fallback prohibition;
- decides layout / ABI / storage;
- changes language semantics.

Example target shape for PackedArray:

```text
PackedArray<T> declared
  -> eligible plan exists?
  -> materialization boundary OK?
  -> backend capability OK?
  -> boxed fallback forbidden
  -> unsupported route fails fast
```

## Current Promotion Matrix

This matrix fixes the current interpretation of rows that already behave like
contracts, CorePlan inputs, or backend routes. It is a task-order guide, not a
request to change JSON shape in place.

### Promote / Treat As Active Now

| Key | Target | Reason | Required next action |
| --- | --- | --- | --- |
| `lowering_plan` | `LoweringRoutes`, `backend_active` | Backends already consume flattened route entries instead of helper names. | Keep backend consumers route-first; do not add new raw helper-name classifiers. |
| `typed_object_plans` | `LayoutPlans`, CorePlan layout/ABI truth | Type ids, slots, storage, and field counts are backend-visible layout truth. | Module verifier enforces unique plans, nonzero unique type ids, `runtime_slot_object_v0`, `field_count`, unique contiguous slots, and weak-field rejection. |
| `static_data_plans` | `LayoutPlans` / `LoweringRoutes`, `backend_active` | Static data load/emit already depends on MIR-owned table shape and bounds. | Module verifier enforces supported element/align/value ranges, unique source/symbol rows, and `StaticDataLoad` plan consistency. |
| `effect_plans` | `Contracts`, `verifier_active` | `Contract(no_alloc/no_safepoint)` is verified through effect plans, not raw runes. | Keep backend use indirect; backend should see only accepted routes. |
| `inline_plans` with `request=required` and `verified=true` | `Contracts` / `optimizer_active` | Required inline can fail fast and verified rows may drive narrow optimizer lowering. | Advisory `prefer/avoid/hot/cold` rows remain metadata; backends must not treat the row as an inline mandate. |
| `string_kernel_plans` | `Contracts` / `LoweringRoutes` when consumed | Publication, borrow, carrier, and consumer legality are already verified. | Preserve verifier ownership of legality; emitters consume only checked route shape. |
| `placement_effect_routes` | `PlacementPlans`, CorePlan placement/effect owner | It is the generic folded surface for publication/materialization/effect decisions. | Prefer this row over direct family-specific reads for new backend consumers. |
| `exact_numeric_runtime_check_contracts` | `Contracts`, `verifier_active` | Dynamic exact numeric checks and backend support already fail fast when unsupported. | Keep dynamic range-check capability tied to verifier/backend gates. |
| `hako_alloc_*_packed_store_pilot_plans` | `Contracts`, `verifier_active` | The verifier already enforces fixed record/column/sentinel/no-lowering invariants. | Do not promote to CorePlan lowering until a real storage owner and backend route exist. |

### Promote Soon / Prepare A Dedicated Row

| Key | Future target | Trigger | Required next action |
| --- | --- | --- | --- |
| `array_record_materialization_boundary_plans` | `Contracts`, `verifier_active` | Public `get`, returned record elements, or backend escapes become observable. | Fail fast unsupported materialization before public record materialization work begins. |
| `source_packed_array_direct_read_consumption_plans` | `LoweringRoutes`, CorePlan direct-read route | Real packed direct-read backend lowering is introduced. | Add backend proof, capability gate, and `boxed_fallback=false` contract in the same row. |
| `loop_range_facts` | `Contracts`, `verifier_active` | LoopRange facts start deciding accepted write/index policy. | Promote the write-policy booleans into a verifier-owned accept/reject contract before lowering depends on them. |
| `array_text_*` / `array_text_state_residence_route` | `LoweringRoutes`, CorePlan text/array route | C/EXE consumers depend on effects, materialization policy, byte-boundary proof, or executor mode. | Route consumers must read a proof-bearing route, not rediscover region legality. |
| enum use rows derived from `enum_decls` | `Contracts`, `verifier_active` | Sum construction/projection/runtime dispatch depends on declared tag/payload shape. | Keep `enum_decls` as source inventory; add verifier checks for declared variant/tag/payload use. |
| exact numeric binary/compare/shift route facts | `LoweringRoutes` | Backend lowering consumes op-specific exact numeric facts. | Move consumption behind route rows with backend capability rejection. |

### Keep As Metadata / Do Not Promote Directly

| Key | Keep as | Reason |
| --- | --- | --- |
| `record_decls`, `enum_decls`, `user_box_decls` | `SourceAttrs` | Declaration inventories are not lowering contracts. Promote derived rows, not raw declarations. |
| `record_layout_plans` | `LayoutPlans`, `semantic_layout_truth` | Record layout is canonical layout truth, but record materialization/backend lowering is still closed. |
| `array_record_storage_plans`, `array_record_autouse_eligibility_plans` | `LayoutPlans` / `PlacementPlans`, metadata-only | They describe storage candidates and eligibility; they must not mutate ArrayBox runtime storage by themselves. |
| `value_consumer_facts`, `storage_classes` | `SemanticFacts` | Raw facts are fold-up inputs. Promote only the route or contract that consumes them. |
| `string_corridor_*`, `sum_placement_*`, `thin_entry_*` | family facts / compatibility rows | Long-term backend consumption should fold through `placement_effect_routes` or family-specific route rows. |
| `*_micro_seed_route`, `*_seed_route`, `exact_seed_backend_route` | `ExperimentalSeedRoutes` | Seed payloads are temporary exact-shape bridges. They need retire conditions, not CorePlan promotion. |

## Promotion Task Queue

Use this order when turning the matrix into implementation cards:

1. `METADATA-PROMOTE-001`: harden catalog rows for active contracts and routes
   without changing MIR JSON shape.
2. `METADATA-PROMOTE-002`: typed-object/static-data verifier hardening
   (`typed_object_plans`, `static_data_plans`).
3. `METADATA-PROMOTE-003`: exact numeric / effect / required-inline /
   string-kernel contract wording and guard coverage.
4. `METADATA-PROMOTE-004`: `placement_effect_routes` consumer fold-up plan for
   family-specific string/sum/thin-entry readers.
5. `METADATA-PROMOTE-005`: PackedArray no-fallback contract before any packed
   record backend lowering flag is enabled.
6. `METADATA-PROMOTE-006`: seed route retirement ledger and generic route
   replacement plan.

Stop line: do not combine these with allocator behavior rows. Promotion tasks
are BoxShape work unless a card explicitly adds one new accepted lowering route
with proof and guard.

## Module-level metadata keys

| Key | Class | Producer | Primary consumer | backend_active | fallback_allowed | retire_condition |
| --- | --- | --- | --- | --- | --- | --- |
| `user_box_decls` | `SourceAttrs` | Stage1 / MIR builder declaration transport | typed-object planning, thin-entry facts, diagnostics, legacy builder input | yes for existing user-box routes | names-only compatibility still exists for legacy declarations | retire names-only compatibility after all frontends emit typed field declarations |
| `record_decls` | `SourceAttrs` | Stage1 record declaration transport | `record_layout_plans` | no | no ordinary user-box fallback | retain as record source truth |
| `enum_decls` | `SourceAttrs` | Stage1 enum declaration transport and prelude enum injection | sum placement, thin-entry, diagnostics | yes for existing sum routes | unsupported surfaces fail fast | retain as enum source truth |
| `typed_object_plans` | `LayoutPlans` | `refresh_module_typed_object_plans` | EXE typed-object newbox / field get / field set lowering, verifier rows | yes | no app-specific slot inference | retain while typed user boxes lower through runtime slot object layout |
| `record_layout_plans` | `LayoutPlans` | `refresh_module_record_layout_plans` | record storage descriptors, hako_alloc metadata verifier rows | no direct backend lowering | no ordinary user-box fallback | retain as record layout truth |
| `array_record_storage_plans` | `LayoutPlans` | `refresh_module_array_record_storage_plans` | packed ArrayBox eligibility and probes | no | no runtime storage mutation | fold into packed storage owner when production storage lands |
| `array_record_autouse_eligibility_plans` | `PlacementPlans` | `refresh_module_array_record_autouse_eligibility_plans` | materialization boundary and packed auto-use pilot | no | unsupported shapes remain rejected/fail-fast | retire only after production auto-use has a verifier-owned replacement |
| `array_record_materialization_boundary_plans` | `PlacementPlans` | `refresh_module_array_record_materialization_boundary_plans` | packed auto-use pilot and diagnostics | no | visible materialization has no boxed fallback | retire when public record materialization is implemented |
| `array_record_packed_autouse_pilot_plans` | `PlacementPlans` | `refresh_module_array_record_packed_autouse_pilot_plans` | source PackedArray pilot, backend capability gate | metadata-only today | boxed fallback disabled | retire or convert when packed backend lowering becomes production |
| `source_packed_array_autouse_pilot_plans` | `PlacementPlans` | `refresh_module_source_packed_array_autouse_pilot_plans` | direct-read consumption rows | no | boxed fallback disabled | fold into production PackedArray plan when available |
| `source_packed_array_direct_read_consumption_plans` | `PlacementPlans` | `refresh_module_source_packed_array_direct_read_consumption_plans` | backend capability gate / future direct-read lowering | no | boxed fallback disabled | retire when direct-read lowering has a generic plan |
| `hako_alloc_aligned_small_packed_store_pilot_plans` | `PlacementPlans` | `refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans` | hako_alloc metadata verifier | no | live scalar columns retained | retire after allocator metadata store migrates to production storage |
| `hako_alloc_huge_page_packed_store_pilot_plans` | `PlacementPlans` | `refresh_module_hako_alloc_huge_page_packed_store_pilot_plans` | hako_alloc metadata verifier | no | live scalar columns retained | retire after allocator huge metadata store migrates to production storage |
| `static_data_plans` | `LayoutPlans` | static const table lowering | static data emit/load lowering | yes | no runtime Array/Map materialization fallback | retain while static readonly data is MIR-owned |

### `typed_object_plans[]`

`typed_object_plans[]` is the module-level layout truth for the first direct EXE
typed-object route. It is derived from `user_box_decls` /
`user_box_field_decls` during MIR semantic metadata refresh.

Current accepted shape:

- non-weak user box fields
- declared i64 storage only: `IntegerBox`, `Integer`, or `i64`
- runtime slot object layout
- allocation plus slot `field_set` / `field_get`

Current intentionally unsupported shape:

- weak fields
- untyped / unknown / mixed storage fields
- handle fields such as String / Array / Map
- dynamic field add
- constructor inline or method-call lowering ownership

Example:

```json
{
  "typed_object_plans": [
    {
      "box_name": "Pair",
      "type_id": 1,
      "layout_kind": "runtime_slot_object_v0",
      "field_count": 2,
      "fields": [
        {
          "name": "left",
          "slot": 0,
          "declared_type": "IntegerBox",
          "storage": "i64",
          "weak": false
        },
        {
          "name": "right",
          "slot": 1,
          "declared_type": "IntegerBox",
          "storage": "i64",
          "weak": false
        }
      ]
    }
  ]
}
```

Contract:

- MIR owns slot assignment and type ids.
- The backend reads `typed_object_plans[]`; it must not infer slots from raw
  declarations or app-specific names.
- The runtime owns opaque typed-object allocation and field storage helpers.
- `VM InstanceBox` remains reference semantics, not the EXE layout owner.

## Function-level metadata keys

| Key | Shape | Purpose |
| --- | --- | --- |
| `value_types` | object map `{value_id: type_hint}` | Per-value type hints (`i64`, `i1`, `f64`, `void`, `{kind:"handle"}` etc.) |
| `value_consumer_facts` | object map `{value_id: fact}` | Generic consumer facts derived from canonical MIR; backend consumers must not re-own legality scans |
| `loop_range_facts` | array | Stage1 LoopRange index/bound/step contract facts |
| `runes` | array | Declaration-local `@rune` attrs carried into MIR |
| `storage_classes` | object map `{value_id: storage_class}` | Current storage-class inventory for value lanes |
| `string_corridor_facts` | object map `{value_id: fact}` | Canonical string corridor facts (`str.slice`, `str.len`, `freeze.str`) keyed by produced value |
| `string_corridor_relations` | object map `{value_id: [relation, ...]}` | Structural relation facts derived from canonical MIR plus PHI queries |
| `string_corridor_candidates` | object map `{value_id: [candidate, ...]}` | Placement/effect candidate inventory derived from string corridor facts |
| `string_kernel_plans` | object map `{value_id: plan}` | Backend-consumable string kernel plans derived from corridor candidates |
| `string_direct_set_window_routes` | array | Source-window direct-set route plans |
| `thin_entry_candidates` | array | Candidate sites for public-entry vs thin-entry selection |
| `thin_entry_selections` | array | Manifest-bound thin-entry decisions |
| `inline_plans` | array | InlinePlan rows derived from declaration-local `Hint(inline/noinline/hot/cold)` and `Lowering(inline_required)` runes; M11c-soft-leaf may consume `request=prefer` for narrow same-module MIR leaf inline, and M13 may consume verified `request=required` for narrow same-module scalar leaf inline before backend emission |
| `effect_plans` | array | EffectPlan rows derived from live verifier-backed `Contract(no_alloc/no_safepoint)` runes and reserved `Profile(...)` expansions; consumed by the MIR verifier, not by backends |
| `capability_plans` | array | CapabilityPlan rows derived from reserved `Profile(...)` expansions; metadata only until capability verification lands |
| `generic_method_routes` | array | MIR-owned method route facts; backend shims consume these instead of reclassifying method strings |
| `extern_call_routes` | array | MIR-owned route facts for accepted `externcall` sites; pure-first reads these rows instead of classifying helper names locally |
| `global_call_routes` | array | MIR-owned global-call route / unsupported route facts |
| `user_box_method_routes` | array | MIR-owned typed user-box method route facts |
| `map_lookup_fusion_routes` | array | Metadata-only Map get/has same-key fusion preflight rows |
| `lowering_plan` | array | Flattened backend route entries derived from explicit route facts, including `extern_call_routes`; backends consume these entries as lowering decisions, not as semantic discovery |
| `sum_placement_facts` | array | Observed sum objectization / local-aggregate facts |
| `sum_placement_selections` | array | Selected sum path (`local_aggregate` vs compat fallback) |
| `sum_placement_layouts` | array | LLVM-side local aggregate layout choice for selected sums |
| `agg_local_scalarization_routes` | array | Folded agg-local route inventory over sum, thin-entry, and storage-class pilots |
| `placement_effect_routes` | array | Generic folded placement/effect route inventory; consumers should prefer this before family-specific rows |
| `array_rmw_window_routes` | array | Backend-consumable array RMW legality window |
| `array_string_len_window_routes` | array | Backend-consumable array string length observer window |
| `array_text_*` | array/object | Array/text loopcarry, edit, residence, observer, combined-region, and state-residence plans |
| `declared_param_decls` / `declared_return_type_name` | array / string or null | Source signature annotation transported without forcing the callable ABI |
| `exact_numeric_*` | arrays / maps | Exact numeric facts, route facts, rejection rows, and runtime-check contracts |
| `array_string_store_micro_seed_route` | object or null | Exact array/string-store micro seed payload |
| `array_getset_micro_seed_route` | object or null | Exact array get/set micro seed payload |
| `array_rmw_add1_leaf_seed_route` | object or null | Exact array RMW add1 leaf seed payload |
| `concat_const_suffix_micro_seed_route` | object or null | Exact concat const-suffix micro seed payload |
| `substring_views_micro_seed_route` | object or null | Exact substring views micro seed payload |
| `sum_variant_tag_seed_route` | object or null | Exact Sum `variant_tag` seed route selected from Sum placement metadata |
| `sum_variant_project_seed_route` | object or null | Exact Sum `variant_project` seed route selected from Sum placement metadata |
| `userbox_local_scalar_seed_route` | object or null | Exact UserBox Point local/copy scalar seed route selected from thin-entry field metadata |
| `userbox_loop_micro_seed_route` | object or null | Exact UserBox loop micro seed payload |
| `userbox_known_receiver_method_seed_route` | object or null | Exact UserBox known-receiver method seed payload |
| `exact_seed_backend_route` | object or null | Function-level backend route tag for one already-proven exact seed payload |

## Placement Route Fold-Up Contract

`placement_effect_routes` is the generic folded owner for placement/effect
decisions. Family-specific rows such as `sum_placement_*`,
`thin_entry_selections`, and `string_corridor_*` may remain as source-family
facts or compatibility inspection rows, but new backend consumers should prefer
the folded route first.

Current C backend shims already follow this direction for several paths:

```text
placement_effect_routes
  -> preferred generic reader
family-specific rows
  -> compatibility fallback while consumers are being migrated
```

Retire condition: backend consumers stop reading family-specific rows directly
for a route family once the folded `placement_effect_routes` payload carries
the same proof, demand, publication boundary, and selected value identity.

### Placement Effect Consumer Fold-Up Plan

`placement_effect_routes` is the preferred generic reader for new
placement/effect backend consumers. Family rows remain compatibility fallbacks
until each family has proof parity through the folded row.

| Family | Current preferred reader | Compatibility fallback | Fold-up retire condition |
| --- | --- | --- | --- |
| string corridor route windows | `placement_effect_routes` rows with `source=string_corridor` and `decision=direct_kernel_entry` | `string_corridor_candidates` plan window readers | retire fallback once folded routes carry all source-root/window/proof fields needed by string concat/substring readers |
| sum placement local aggregate | `placement_effect_routes` rows with `source=sum_placement` and `decision=local_aggregate` | `sum_placement_facts` and `sum_placement_selections` | retire fallback once selected value/source-sum/manifest-row proof parity is covered by folded routes |
| sum local aggregate layout | `placement_effect_routes` rows with `source=agg_local_scalarization` and `decision=local_aggregate` | `sum_placement_layouts` | retire fallback once layout detail is represented by folded route detail or a generic layout route |
| thin entry | `placement_effect_routes` rows with `source=thin_entry` and selected entry decision | `thin_entry_selections` | retire fallback once public/thin entry selection consumers no longer need family rows |
| string direct kernels | `string_kernel_plans` | none; family-specific verified route remains active | do not fold into `placement_effect_routes` until borrow/publication/carrier/text-consumer verifier facts have an equivalent generic route shape |

Current C shim anchors:

- `hako_llvmc_placement_effect_routes`
- `hako_llvmc_has_thin_entry_selection`
- `hako_llvmc_sum_has_thin_internal_selection`
- `hako_llvmc_sum_has_local_aggregate_fact`
- `hako_llvmc_sum_has_local_aggregate_selection`
- `hako_llvmc_sum_has_layout`
- `hako_llvmc_string_corridor_read_route_window_from_placement_effect_routes`

Stop lines:

- Do not delete family-specific readers until the folded route carries the same
  proof, demand, publication boundary, and selected value identity.
- Do not make backend readers infer placement from raw helper names or app
  shapes while migrating.
- Do not fold `string_kernel_plans` into `placement_effect_routes` without a
  verifier-equivalent generic route shape.

## Experimental Seed Route Policy

Rows ending in `*_micro_seed_route` or `*_seed_route`, plus
`exact_seed_backend_route`, are `ExperimentalSeedRoutes`. They are allowed only
when they quarantine a temporary exact-shape proof in MIR metadata so a backend
can remain an emitter selector instead of a raw MIR/app-shape planner.

Contract:

- canonical MIR instructions remain unchanged;
- legality is owned by the source family plan (`array_rmw_window_routes`,
  `string_kernel_plans`, `sum_placement_*`, `thin_entry_*`, etc.);
- `exact_seed_backend_route` may select one already-proven payload, but does
  not own payload legality;
- each new seed row must document `retire_condition`;
- generic `LoweringRoutes` or `PlacementPlans` are the preferred long-term
  replacement.

## Metadata Namespace Boundary

Do not mix these similarly named structures:

| Name | Owner | Scope |
| --- | --- | --- |
| `MetadataContext` | MIR builder | builder-time provenance, source-file hints, hint scope, region trace, and diagnostics context |
| `FunctionMetadata` / `ModuleMetadata` | MIR semantic refresh / MIR JSON emit | semantic facts, layout plans, placement plans, lowering routes, diagnostics metadata |
| `PluginMetadata` | BID/plugin runtime | FFI plugin type/method table and plugin lifecycle state |

`MetadataContext` is not the MIR metadata ledger. If it is renamed later,
prefer a provenance/diagnostic name such as `BuilderProvenanceContext`, but do
not do that as part of route or layout work.

## Drift Guard

`tools/checks/mir_metadata_catalog_guard.sh` keeps this catalog synchronized
with the MIR JSON root emitter, `FunctionMetadata` seed rows, and semantic
refresh entry points. If a new metadata key is emitted, update this SSOT in the
same change with its class, owner, producer, consumer, backend-active state,
fallback policy, and retire condition.

## Active Function Contract Rows

The following function-level rows are already contract-active and must remain
verifier-owned. Backend consumers may only consume checked routes or accepted
lowering plans; they must not rediscover legality from raw runes, profile
names, helper names, or app shape.

| Row | Verifier owner | Contract |
| --- | --- | --- |
| `effect_plans` | `src/mir/verification/rune_contracts.rs` | `effect_plans` are the obligation source for live `Contract(no_alloc/no_safepoint)` checks. Raw `runes` are transport/provenance after refresh, not the verifier source of truth. |
| `inline_plans` | `src/mir/verification/inline_required.rs` | Required inline is contract-active only for `request=required`; accepted plans must have `verified=true`, required `no_alloc` / `no_safepoint`, supported narrow leaf shape, and `fallback=fail_fast`. Advisory `prefer/avoid/hot/cold` rows stay metadata/optimizer hints. |
| `string_kernel_plans` | `src/mir/verification/string_kernel.rs` | Direct-kernel string plans must carry verifier-visible borrow, publication, carrier, text-consumer, and stable-view provenance facts before emitters trust them. `StringKernelPlanVerifierOwner::LoweringDirectKernelEntry` owns the current lane. |
| `exact_numeric_runtime_check_contracts` | `src/mir/exact_numeric_field_contracts.rs` plus `src/mir/exact_numeric_backend_capability.rs` | Dynamic `Integer` to exact numeric field assignments become runtime range-check obligations. Backend use must pass `enforce_exact_numeric_backend_supported`; unsupported backends fail fast instead of silently lowering. |

Guard anchors:

- `tools/checks/k2_wide_effect_capability_plan_guard.sh`
- `tools/checks/k2_wide_inline_required_verify_guard.sh`
- `tools/checks/mir_metadata_catalog_guard.sh`

Stop lines:

- `capability_plans` remain metadata until capability verification lands.
- `Profile(...)` names must not be backend-consumed.
- seed routes remain `ExperimentalSeedRoutes`; their legality comes from
  source-family plans or generic route rows.

## InlinePlan metadata

`inline_plans` records MIR-owned inline metadata in MIR JSON. M11c-soft-leaf
may consume advisory `request = "prefer"` inside the MIR optimizer for narrow
same-module pure leaf calls. M11c-required-verify accepts or rejects
`request = "required"` plans with verifier diagnostics and marks accepted plans
as `verified = true`. M13 lets the MIR optimizer consume verified required
plans for the scalar allocator-fast proof. Backends still must not consume this
row as an inline mandate.

Example:

```json
{
  "inline_plans": [
    {
      "function": "Main.align_up/2",
      "request": "prefer",
      "hotness": null,
      "max_ir": null,
      "requires": [],
      "verified": false,
      "fallback": "keep_call",
      "source": "rune_hint"
    }
  ]
}
```

Current request mapping:

- `Hint(inline)` -> `request = "prefer"`
- `Hint(noinline)` -> `request = "avoid"`
- `Hint(hot)` -> `request = "none"`, `hotness = "hot"`
- `Hint(cold)` -> `request = "none"`, `hotness = "cold"`
- `Lowering(inline_required)` -> `request = "required"`,
  `requires = ["no_alloc", "no_safepoint"]`, `fallback = "fail_fast"`,
  `source = "rune_lowering"`; `verified` is true only after
  M11c-required-verify accepts the required contracts and narrow leaf shape

Soft inline is accepted only for one-block same-module `Callee::Global` bodies
with no nested call/control and a narrow pure instruction vocabulary. Failed
soft inline keeps the original call. Required inline verifier acceptance is
live-narrow, but backend-required lowering remains reserved.

## EffectPlan / CapabilityPlan Metadata

M11d added MIR-owned effect/capability boundaries. M12c adds reserved
`Profile(...)` parser acceptance and expands profiles into those existing
metadata boundaries without adding backend use.

```text
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
-> metadata.effect_plans = [
     {
       function,
       requires: ["no_alloc", "no_safepoint"],
       verified: false,
       source: "rune_contract"
     }
   ]
-> metadata.capability_plans = []

@rune Profile(allocator.fast)
-> metadata.inline_plans = [
     { request: "none", hotness: "hot", source: "rune_profile:allocator.fast" },
     { request: "required", source: "rune_profile:allocator.fast" }
   ]
-> metadata.effect_plans = [
     { requires: ["no_alloc", "no_safepoint"], source: "rune_profile" }
   ]
-> metadata.capability_plans = [
     { allow: ["hako.mem", "hako.ptr", "hako.tls"], source: "rune_profile" }
   ]
```

The rune contract verifier consumes `effect_plans` as the obligation source.
`Contract(pure)` / `Contract(readonly)` are not live EffectPlan requirements
yet. `Capability(...)` is not accepted parser surface.

Refresh owner:

```text
src/mir/rune_plan_refresh.rs
  refresh_function_rune_plans(function)
```

MIR JSON emit owner:

```text
src/runner/mir_json_emit/plan_metadata.rs
```

## ExternCallRoute Metadata

`extern_call_routes` records narrow, MIR-owned facts for accepted
`externcall` sites. The backend must read these facts through `lowering_plan`
and must not re-infer accepted routes from raw symbol strings, fixture names, or
box names.

Current allocator-substrate extern families include:

- `hako_mem_alloc` / `hako_mem_free`
- `hako_atomic_ptr_store_ordered` / `hako_atomic_ptr_load_ordered` /
  `hako_atomic_ptr_cas_ordered`
- `hako_osvm_reserve_bytes_i64` / `hako_osvm_commit_bytes_i64` /
  `hako_osvm_decommit_bytes_i64`
- `hako_tls_cache_slot_get_i64` / `hako_tls_cache_slot_set_i64`
- `hako_atomic_slot_cas_i64`
- `hako_atomic_slot_load_i64`
- `hako_atomic_slot_store_i64`
- `hako_atomic_slot_fetch_add_i64`

Runtime-decl native leaves may include additional symbols such as
`hako_mem_realloc` and `hako_osvm_page_size_i64`. Do not list them as
`extern_call_routes` unless a route row exists; runtime-decl presence alone is
not a language acceptance rule.

Required row facts include:

- `route_id`
- `core_op`
- `symbol`
- `tier`
- `emit_kind`
- `return_shape`
- `value_demand`
- `effects`
- source and operand value ids

`lowering_plan` mirrors the same decision with backend-facing fields such as
`route_id`, `core_op`, `symbol`, `arity`, `return_shape`, and `value_demand`.
The row is a lowering contract; it does not create a second source of language
semantics.

## Value maps

### `value_types`

`value_types` stores string or object hints keyed by MIR value id as strings:

```json
{
  "1": "i64",
  "2": "i1",
  "3": {"kind": "handle", "box_type": "StringBox"},
  "4": "void"
}
```

Current emit mapping comes from `src/runner/mir_json_emit/mod.rs`:

- `MirType::Integer` -> `"i64"`
- `MirType::Bool` -> `"i1"`
- `MirType::Float` -> `"f64"`
- `MirType::Void` -> `"void"`
- `MirType::String` -> `{"kind":"string"}`
- `MirType::Box(name)` -> `{"kind":"handle","box_type": name}`

### `storage_classes`

Storage-class inventory is emitted as a string map keyed by MIR value id:

```json
{
  "7": "inline_i64",
  "8": "handle",
  "9": "borrowed_text"
}
```

This is current-lane inspection metadata for value-lane planning. It must not be
treated as a replacement for canonical instructions.

## String corridor metadata

String corridor metadata records the current canonical string-lane reading without
inventing a second MIR dialect.

### `string_corridor_facts`

`string_corridor_facts` is emitted as an object map keyed by MIR value id:

```json
{
  "7": {
    "op": "str.slice",
    "role": "borrow_producer",
    "carrier": "method_call",
    "outcome": null,
    "objectize": "?",
    "publish": "?",
    "materialize": "?"
  }
}
```

Each fact object contains:

| Field | Meaning |
| --- | --- |
| `op` | One of `str.slice`, `str.len`, `freeze.str` |
| `role` | `borrow_producer`, `scalar_consumer`, or `birth_sink` |
| `carrier` | Current lowering carrier such as `method_call`, `runtime_export`, `canonical_intrinsic` |
| `outcome` | Optional Birth / Placement outcome name (`ReturnHandle`, `BorrowView`, `FreezeOwned`, etc.) |
| `objectize` | Objectization placement fact (`?`, `none`, `sink`, `deferred`) |
| `publish` | Publication placement fact. Current fact-level states remain `?`, `none`, `sink`, `deferred`; explicit `publish.text` operands are mirrored today via candidate-plan / kernel-plan / placement-route fields such as `publish_reason` and `publish_repr_policy` |
| `materialize` | Materialization placement fact (`?`, `none`, `sink`, `deferred`) |

### `string_corridor_candidates`

`string_corridor_candidates` is emitted as an object map from MIR value id to an
array of placement/effect candidate records:

```json
{
  "7": [
    {
      "kind": "direct_kernel_entry",
      "state": "candidate",
      "reason": "borrowed slice corridor can target a direct kernel entry before publication"
    }
  ]
}
```

Each candidate object contains:

| Field | Meaning |
| --- | --- |
| `kind` | `borrowed_corridor_fusion`, `publication_sink`, `materialization_sink`, or `direct_kernel_entry` |
| `state` | `candidate` or `already_satisfied` |
| `reason` | Stable explanation string |
| `plan.publish_reason` | Optional `publish.text` reason when MIR already knows the boundary demand |
| `plan.publish_repr_policy` | Optional public representation policy for `publish.text` |

### Future `publish.text` / `publish.any` operand reading

When explicit publication ops land, `publish` metadata remains the inspection mirror
for operand structure rather than a second source of truth.

- `publish.text(value, reason, repr_policy)`
  - string-only v1 bridge
  - `reason`: why publication is required (`escape_required`, `explicit_api_replay`, `stable_object_demand`, etc.)
  - `repr_policy`: which public representation is required (`stable_owned`, `stable_view`, etc.)
- `publish.any(value, reason, repr_policy)`
  - generic bridge, deferred until string-only `publish.text` proves out

Current phase-137x lock:

- explicit publish ops are not emitted yet
- fact-level `publish` stays coarse, but current MIR metadata already exports `publish_reason` / `publish_repr_policy` on candidate plans, string kernel plans, and placement-effect routes when known
- design authority stays in:
  - `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`

## Thin-entry metadata

Thin-entry metadata records where canonical MIR already exposes a site that could
later choose **public entry** or **thin internal entry** without inventing a new
call or field-access dialect.

### `thin_entry_candidates[]`

Each candidate object has the following fields:

| Field | Meaning |
| --- | --- |
| `block` | Basic block id |
| `instruction_index` | Instruction index inside the block |
| `value` | Optional MIR value id being produced |
| `surface` | One of `user_box_method`, `user_box_field_get`, `user_box_field_set`, `variant_make`, `variant_project` |
| `subject` | Human-readable subject (`Box.field`, `Enum::Variant`, etc.) |
| `preferred_entry` | `public_entry` or `thin_internal_entry` |
| `current_carrier` | `public_runtime`, `backend_typed`, or `compat_box` |
| `value_class` | `?`, `inline_i64`, `inline_bool`, `inline_f64`, `borrowed_text`, `handle`, `agg_local` |
| `reason` | Stable explanation string |

### `thin_entry_selections[]`

Selections bind manifest rows to candidates:

| Field | Meaning |
| --- | --- |
| `block`, `instruction_index`, `value`, `surface`, `subject` | Same site identity as candidate |
| `manifest_row` | Stable manifest row id |
| `selected_entry` | `public_entry` or `thin_internal_entry` |
| `state` | `candidate` or `already_satisfied` |
| `current_carrier` | Same current carrier classification |
| `value_class` | Same value-class classification |
| `reason` | Stable explanation string |

## Sum placement metadata

Sum placement metadata is the phase-163x proving slice for
aggregate-first / objectize-only-when-needed handling. It remains
inspection-only and should later fold into a generic placement/effect pass.

### `sum_placement_facts[]`

| Field | Meaning |
| --- | --- |
| `block`, `instruction_index`, `value`, `surface`, `subject` | Site identity |
| `source_sum` | Optional originating sum value id |
| `value_class` | Current thin-entry value-class view |
| `state` | `local_agg_candidate` or `needs_objectization` |
| `tag_reads` | Number of observed tag reads |
| `project_reads` | Number of observed payload projections |
| `barriers` | Array of objectization barriers |
| `reason` | Stable explanation string |

Current `barriers[]` values:

- `return`
- `call`
- `store_like`
- `phi_merge`
- `capture`
- `debug_observe`
- `unknown_use`

### `sum_placement_selections[]`

Selections map facts onto the currently chosen lowering path:

| Field | Meaning |
| --- | --- |
| `block`, `instruction_index`, `value`, `surface`, `subject`, `source_sum` | Site identity |
| `manifest_row` | Stable manifest row id |
| `selected_path` | `local_aggregate` or `compat_runtime_box` |
| `reason` | Stable explanation string |

### `sum_placement_layouts[]`

Layouts tell LLVM which local aggregate layout to use once a sum site is selected
for the local aggregate path:

| Field | Meaning |
| --- | --- |
| `block`, `instruction_index`, `value`, `surface`, `subject`, `source_sum` | Site identity |
| `layout` | `tag_only`, `tag_i64_payload`, `tag_f64_payload`, or `tag_handle_payload` |
| `reason` | Stable explanation string |

### `sum_variant_tag_seed_route`

`sum_variant_tag_seed_route` is the exact-seed bridge for the current
`variant_tag` local/copy seed family. It is derived from the Sum metadata above;
it does not replace canonical `variant_make` / `variant_tag` instructions.

| Field | Meaning |
| --- | --- |
| `kind` | `variant_tag_local_i64`, `variant_tag_local_tag_only`, `variant_tag_local_f64`, `variant_tag_local_handle`, or `variant_tag_copy_local_i64` |
| `enum`, `variant`, `subject` | Enum and variant identity |
| `layout` | Selected local aggregate layout |
| `variant_tag` | Discriminant value emitted by the exact helper |
| `make_block`, `make_instruction_index`, `tag_block`, `tag_instruction_index` | MIR sites proven by the route |
| `sum_value`, `tag_value`, `tag_source_value`, `copy_value`, `payload_value` | Value ids needed by backend validation |
| `proof` | `sum_variant_tag_local_aggregate_seed` |
| `consumer_capability` | `direct_sum_variant_tag_local` |
| `publication_boundary` | `none` |

### `sum_variant_project_seed_route`

`sum_variant_project_seed_route` is the exact-seed bridge for the current
`variant_project` local/copy seed family. It carries the literal payload needed
by the temporary backend helper while route legality stays in Sum placement
metadata.

| Field | Meaning |
| --- | --- |
| `kind` | `variant_project_local_i64`, `variant_project_local_f64`, `variant_project_local_handle`, `variant_project_copy_local_i64`, `variant_project_copy_local_f64`, or `variant_project_copy_local_handle` |
| `enum`, `variant`, `subject` | Enum and variant identity |
| `layout` | Selected local aggregate layout |
| `variant_tag` | Expected discriminant for the projected variant |
| `make_block`, `make_instruction_index`, `project_block`, `project_instruction_index` | MIR sites proven by the route |
| `sum_value`, `project_value`, `project_source_value`, `copy_value`, `payload_value` | Value ids needed by backend validation |
| `payload_literal_kind`, `payload_i64`, `payload_f64`, `payload_string` | Literal payload for the exact helper |
| `proof` | `sum_variant_project_local_aggregate_seed` |
| `consumer_capability` | `direct_sum_variant_project_local` |
| `publication_boundary` | `none` |

### `userbox_local_scalar_seed_route`

`userbox_local_scalar_seed_route` is the exact-seed bridge for the current
UserBox Point local/copy scalar seed pair. It is derived from thin-entry field
metadata; it does not replace canonical `newbox` / `field_set` / `field_get`
instructions.

| Field | Meaning |
| --- | --- |
| `kind` | `point_local_i64` or `point_copy_local_i64` |
| `box`, `x_field`, `y_field` | UserBox and field identity; current slice is `Point.x` / `Point.y` |
| `block`, `newbox_instruction_index`, `set_x_instruction_index`, `set_y_instruction_index`, `get_x_instruction_index`, `get_y_instruction_index` | MIR sites proven by the route |
| `point_value`, `copy_value`, `x_value`, `y_value`, `get_x_value`, `get_y_value`, `result_value` | Value ids needed by backend validation |
| `x_i64`, `y_i64` | Literal field payloads for the temporary exact helper |
| `proof` | `userbox_point_field_local_scalar_seed` |
| `consumer_capability` | `direct_userbox_point_local_scalar` |
| `publication_boundary` | `none` |

### `exact_seed_backend_route`

`exact_seed_backend_route` lets the backend choose one already-proven exact seed
payload before walking any legacy compatibility ladder.

| Field | Meaning |
| --- | --- |
| `tag` | Stable backend tag such as `sum_variant_tag_local` |
| `source_route` | Metadata field that owns the payload, such as `sum_variant_tag_seed_route` or `sum_variant_project_seed_route` |
| `proof` | Proof string copied from the selected source route |
| `selected_value` | Optional value id for plan-indexed routes; null for route-payload fields such as Sum and UserBox exact seeds |

## Text MIR / verbose MIR relation

`src/mir/printer.rs` also prints metadata in verbose mode. Current Rust-side
string metadata now uses the same vocabulary in both verbose MIR and MIR JSON:

- `string_corridor_facts`
- `string_corridor_candidates`

## Example

```json
{
  "metadata": {
    "thin_entry_candidates": [
      {
        "block": 0,
        "instruction_index": 3,
        "value": 7,
        "surface": "variant_make",
        "subject": "Option::Some",
        "preferred_entry": "thin_internal_entry",
        "current_carrier": "compat_box",
        "value_class": "agg_local",
        "reason": "variant.make can choose a thin internal aggregate-first route beneath canonical MIR"
      }
    ],
    "sum_placement_selections": [
      {
        "block": 0,
        "instruction_index": 3,
        "value": 7,
        "surface": "variant_make",
        "subject": "Option::Some",
        "source_sum": 7,
        "manifest_row": "variant_make.local_aggregate",
        "selected_path": "local_aggregate",
        "reason": "variant.make stays on the selected local aggregate route in this proving slice"
      }
    ]
  }
}
```
