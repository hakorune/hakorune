## Phase 78: BindingId for Promoted Carriers (dev-only)

### Goal

Make LoopBodyLocal promotion (DigitPos/Trim) **BindingId-aware** without relying on fragile name-based hacks like `format!("is_{}", name)`.

This phase introduces a dev-only identity link:

- `BindingId(original LoopBodyLocal)` → `BindingId(promoted carrier)` → `ValueId(join carrier param)`

### Problem

Promotion creates synthetic carrier names (例: `is_digit_pos`, `is_ch_match`) that do not exist as source-level bindings.

- `PromotedBindingRecorder` expects both original/promoted names in `MirBuilder.binding_map`.
- For promoted carriers this is not true by construction.
- For LoopBodyLocal variables, promotion can happen before their `local` is lowered, so the original name may also be absent.

### Solution (current state)

#### 1) CarrierVar gets optional BindingId (dev-only)

- `src/mir/join_ir/lowering/carrier_info.rs`
  - `CarrierVar.binding_id: Option<BindingId>` (feature `normalized_dev`)

#### 2) CarrierBindingAssigner box

- `src/mir/join_ir/lowering/carrier_binding_assigner.rs`
  - Ensures both sides have BindingIds:
    - If `original_name` is missing in `builder.binding_map`, allocate a temporary BindingId.
    - If `promoted_carrier_name` is missing, allocate a temporary BindingId.
  - Records:
    - `carrier_info.promoted_bindings[original_bid] = promoted_bid`
    - `CarrierVar.binding_id = Some(promoted_bid)` for the promoted carrier
  - Restores `builder.binding_map` after recording (synthetic names do not leak into the source map).

#### 3) Pattern2/Pattern4 wiring

- Pattern2 (break): `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
  - Calls `CarrierBindingAssigner::assign_promoted_binding()` immediately after promotion.
  - Registers `BindingId → ValueId` via `ConditionEnv.register_carrier_binding()` after carrier join_id allocation.
  - Keeps legacy name-based aliasing (`digit_pos` → `is_digit_pos`) for now, but removes local “guess naming” by using the promoted carrier name returned by the promoter.

- Pattern4 (continue): `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`
  - Calls `CarrierBindingAssigner::assign_promoted_binding()` after promotion (analysis metadata only, since Pattern4’s current lowering path does not use JoinValueSpace params).

### Tests

- Unit test (dev-only): `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
  - `phase78_promoted_binding_is_recorded_for_digitpos`
  - Asserts:
    - `promoted_bindings` has one mapping
    - promoted carrier has `binding_id`
    - `ConditionEnv.binding_id_map[promoted_bid] == promoted_carrier.join_id`

### Open Follow-ups (Phase 79+)

1. Wire `ScopeManager::lookup_with_binding()` into ExprLowerer/ConditionLoweringBox call-sites (so BindingId actually drives resolution).
2. Extend coverage to Trim and other promotion shapes (additional unit/E2E tests).
3. Shrink/remove legacy name-based promoted lookup (`resolve_promoted_join_id`) after call-sites consistently provide BindingId.

