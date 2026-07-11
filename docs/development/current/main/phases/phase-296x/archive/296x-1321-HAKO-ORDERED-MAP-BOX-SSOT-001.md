# 296x-1321 HAKO-ORDERED-MAP-BOX-SSOT-001

Status: closed
Date: 2026-06-20

## Purpose

Document and task the first `OrderedMapBox` boundary before implementation.

The immediate driver is MirBuilder migration planning: compiler-support code
needs deterministic `BTreeMap<String, T>`-style iteration, but that requirement
should not be folded into `MapBox` semantics or ring0 substrate design.

## Decision

```text
selected_collection=OrderedMapBox
v0_owner=.hako_library
v0_location=apps/lib/collections/ordered_map.hako
key_domain=String
ordering=deterministic_lexical
ring0_enabled=0
ring1_provider_enabled=0
mapbox_semantics_changed=0
rust_btreemap_api_claim=0
implementation_started=0
```

## Documentation

```text
reference_manual=docs/reference/boxes-system/ordered-mapbox.md
design_ssot=docs/development/current/main/design/ordered-map-box-boundary-ssot.md
```

The reference manual defines the user-facing provisional API. The design SSOT
defines placement, promotion gates, MirBuilder boundaries, and stop lines.

## Task Breakdown

```text
next=HAKO-ORDERED-MAP-BOX-V0-001

sequence:
  HAKO-ORDERED-MAP-BOX-V0-001
  MIRBUILDER-BINDING-CONTEXT-ORDERED-MAP-PROBE-001
  ORDERED-MAP-RING1-PROMOTION-INVENTORY-001
```

## Stop Line

```text
do not implement OrderedMapBox in this docs row
do not change MapBox
do not add ring0 substrate support
do not add ring1 provider registration
do not rewrite MirBuilder
do not claim Rust BTreeMap parity
```

## Evidence

Docs-only row.

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next

```text
HAKO-ORDERED-MAP-BOX-V0-001
```

Implement the small `.hako` library box and deterministic-order smoke before
using it in a BindingContext-style migration probe.
