# MIR Metadata Facts (SSOT)

Status: Canonical for emitted MIR function metadata  
Primary sources:

- `src/mir/function.rs`
- `src/runner/mir_json_emit/mod.rs`
- `src/mir/printer.rs`

This document covers the inspection-only metadata emitted under
`functions[].metadata` in MIR JSON. These facts do **not** create a second MIR
dialect. They annotate canonical MIR so backends and diagnostics can make
placement/entry decisions without guessing from helper names.

## Current emitted metadata keys

| Key | Shape | Purpose |
| --- | --- | --- |
| `value_types` | object map `{value_id: type_hint}` | Per-value type hints (`i64`, `i1`, `f64`, `void`, `{kind:"handle"}` etc.) |
| `storage_classes` | object map `{value_id: storage_class}` | Current storage-class inventory for value lanes |
| `string_corridor_facts` | object map `{value_id: fact}` | Canonical string corridor facts (`str.slice`, `str.len`, `freeze.str`) keyed by produced value |
| `string_corridor_candidates` | object map `{value_id: [candidate, ...]}` | Placement/effect candidate inventory derived from string corridor facts |
| `thin_entry_candidates` | array | Candidate sites for public-entry vs thin-entry selection |
| `thin_entry_selections` | array | Manifest-bound thin-entry decisions |
| `sum_placement_facts` | array | Observed sum objectization / local-aggregate facts |
| `sum_placement_selections` | array | Selected sum path (`local_aggregate` vs compat fallback) |
| `sum_placement_layouts` | array | LLVM-side local aggregate layout choice for selected sums |
| `sum_variant_tag_seed_route` | object or null | Exact Sum `variant_tag` seed route selected from Sum placement metadata |
| `sum_variant_project_seed_route` | object or null | Exact Sum `variant_project` seed route selected from Sum placement metadata |
| `userbox_local_scalar_seed_route` | object or null | Exact UserBox Point local/copy scalar seed route selected from thin-entry field metadata |
| `exact_seed_backend_route` | object or null | Function-level backend route tag for one already-proven exact seed payload |

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
