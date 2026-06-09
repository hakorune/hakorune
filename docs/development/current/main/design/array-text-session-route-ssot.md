---
Status: SSOT
Decision: accepted
Date: 2026-06-10
Scope: array-text string indexOf compat/export split, selected route truth, and session-based hot path lowering.
Related:
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - docs/development/current/main/design/helper-boundary-policy-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-508-array-text-residence-session-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-509-array-text-observer-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-517-array-text-route-root-export-prune-card.md
---

# Array Text Session Route SSOT

## Naming Policy

Prefer `hako.*` route names for selected array-text truth. Keep `nyash.*`
exports as compat / legacy aliases until the route split is complete.

## Decision

Array-text `indexOf` is split into three layers:

```text
compat / public array-string route:
  nyash.array.string_indexof_hisi
  nyash.array.string_indexof_hih
  public handle ABI
  fail-safe adapter
  old lowering target

selected array-text route:
  hako.array_text.session_indexof_const_utf8
  hako.array_text.session_indexof_handle_needle
  MIR / RouteDecision / report truth

compat bridge aliases:
  hako.array_text.slot_indexof_const_utf8_hisi
  hako.array_text.slot_indexof_handle_hih
  stay exported for legacy callers and compatibility probes while the selected
  truth is owned by the session route

selected session route:
  hako.array_text.session_indexof_const_utf8
  hako.array_text.session_indexof_handle_needle

final route:
  selected array-text route
    -> TextSlotReadSession / ArrayTextSession
    -> handle borrow once
    -> repeated slot_text read/search
    -> no publish inside the selected region
```

`nyash.array.string_indexof_hisi` must stay a compat/public adapter. It should
not become the semantic owner for the selected array-text hot route.

## Why

The current hot owner is no longer the string search algorithm itself. The
remaining cost is the repeated return to the public handle/object world inside
the hot corridor.

That means the right split is not:

```text
keep optimizing nyash.array.string_indexof_hisi
```

It is:

```text
separate compat export from selected route truth
borrow once
search many
publish never inside the selected region
```

This is the same structural pattern used by the typed-object exact-slot split:
compat exports stay available, but the selected route truth moves into a
`hako.*` namespace with an explicit lowering form.

## Route Decision Contract

Example selected route:

```json
{
  "route_id": "route.decision",
  "semantic_op": "ArrayTextIndexOf",
  "access_kind": "array_text_slot_indexof_const_utf8",
  "preferred_route": "hako.array_text.session_indexof_const_utf8",
  "selected_route": "hako.array_text.session_indexof_const_utf8",
  "fallback_route": "nyash.array.string_indexof_hisi",
  "fallback_policy": "fail_fast",
  "source_plan_kind": "ArrayTextSlotAccessPlan",
  "proof_ids": [
    "receiver_origin_arraybox",
    "slot_index_i64",
    "slot_text_capable",
    "needle_const_utf8",
    "publication_not_required",
    "no_handle_publication_inside_region",
    "array_text_session_boundary_known"
  ]
}
```

The selected route must not silently rediscover legality through the compat
export after selection.

Observer-route metadata mirrors the same selected truth. The read-side
`array_text_observer_routes` entries carry the `selected_route`,
`selected_bridge_symbol`, `fallback_route`, and `fallback_policy` fields so
the selected region boundary stays visible in MIR JSON and hako_check.

## Helper Bridge Contract

The compat bridge aliases are legacy-only. The selected truth now points at the
session route, and the hot region should not depend on the public handle world
after selection.

```json
{
  "selected_route": "hako.array_text.session_indexof_const_utf8",
  "lowering_form": "session_lowering",
  "bridge_symbol": "hako.array_text.session_indexof_const_utf8",
  "native_session_ready": true
}
```

Rules:

```text
selected route + fallback_policy=fail_fast:
  compat fallback is forbidden inside the selected region
  helper internal dispatch is not keeper evidence
  repeated public handle borrow is not keeper evidence

compat / public route:
  invalid handle or legacy boundary behavior may stay fail-safe
  it is not the keeper route for the hot corridor

compat bridge aliases:
  hako.array_text.slot_indexof_const_utf8_hisi
  hako.array_text.slot_indexof_handle_hih
  remain available for legacy callers but are not the selected truth
```

## Session Bridge

The long-term route shape is session-based.

```text
ArrayTextSession:
  borrow once per cached epoch
  keep read-only slot access local
  avoid publication inside the selected region
  end the session explicitly

The current runtime lowering prefers the cached session helper so selected
indexOf/store paths avoid the direct `with_handle_ready` path on every call.
```

The session is runtime-private lowering, not a public text ABI. It exists to
remove repeated `with_array_box_ready` style borrow/closure cost from the hot
corridor.

## Observation Surface

`hako_check` should read metadata and route reports only.

Useful report vocabulary:

```text
array_text_selected_route_count
array_text_selected_indexof_const_utf8_count
array_text_compat_string_indexof_hisi_count
array_text_with_array_box_ready_selected_count
array_text_session_begin_count
array_text_session_reuse_count
array_text_session_end_count
array_text_publication_in_selected_region_count
array_text_registry_carrier_in_selected_region_count
array_text_silent_fallback_after_selected_route_count
```

Useful explain surfaces:

```text
hako_check array-text-explain
hako_check hot-boundary-check
hako_check publication-boundary-check
hako_check route-diff
```

These are read-only surfaces. They explain whether the selected route is in
place; they do not become the optimization owner themselves.

## Task Ladder

```text
ARRAYTEXT-ROUTE-000:
  Land the array-text session route SSOT.

ARRAYTEXT-ROUTE-001:
  Add route / session / compat report vocabulary and explain surfaces.

ARRAYTEXT-ROUTE-002:
  Route the selected array-text indexOf path through the helper bridge.

ARRAYTEXT-ROUTE-003:
  Add session begin/end lowering and remove repeated borrow from the selected
  region.

ARRAYTEXT-ROUTE-004:
  Add publication / registry guards and remove the helper bridge from the
  selected hot region.
```

## Stop Line

- do not make `nyash.array.string_indexof_hisi` the selected route truth
- do not keep the selected region borrowing the public handle world on every
  call
- do not silently fall back after a selected route is chosen
- do not turn the session into a public text ABI
- do not treat helper-internal dispatch as keeper evidence
