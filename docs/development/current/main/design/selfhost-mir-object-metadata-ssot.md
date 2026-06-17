---
Status: SSOT
Decision: accepted
Date: 2026-06-16
Scope: Minimal object metadata that selfhost `.hako` MIRBuilder may emit.
Related:
  - docs/development/current/main/design/compiler-object-final-shape-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - lang/src/compiler/mirbuilder/README.md
---

# Selfhost MIR Object Metadata SSOT

## Decision

Selfhost `.hako` MIRBuilder may emit object meaning metadata only.  It must not
emit object representation, publication, or backend route truth.

```text
selfhost_mir_object_metadata_contract=hako-selfhost-mir-object-metadata-v0
selfhost_mirbuilder_metadata_only=1
selfhost_mirbuilder_representation_truth_enabled=0
selfhost_mirbuilder_publication_truth_enabled=0
selfhost_mirbuilder_backend_route_truth_enabled=0
```

## Allowed Metadata

The allowed metadata is intentionally small:

```text
source_span
receiver_origin
known_type_hint
field_key
call_site_id
newbox_origin
```

Meaning:

```text
source_span:
  where the source construct came from

receiver_origin:
  which emitted value is the receiver candidate

known_type_hint:
  declared or locally observed type hint, not a storage proof

field_key:
  source-level field name/key, not a direct offset

call_site_id:
  stable id for later analysis/reporting

newbox_origin:
  source origin for NewBox, not allocation representation
```

## Forbidden Metadata

These belong to facts/plans/backend, not selfhost MIRBuilder:

```text
object_storage_plan
object_plan
publication_site
publication_reason
hosthandle_bypass
arc_retirement
 exact_native_struct
 scalarized_fields
 flattened_nested_fields
closed_world_direct_call
backend_direct_route
helper_symbol_inference
method_name_special_case
variable_name_special_case
```

## Owner Handoff

```text
selfhost MIRBuilder:
  emits meaning metadata

SemanticRefresh / Analysis:
  derives facts from MIR and metadata

ObjectPlan:
  decides representation + publication sites

RoutePlan:
  decides callable execution

Backend:
  consumes plans
```

## Fail-Fast Rule

If `.hako` MIRBuilder cannot preserve a requested object metadata shape, it
must fail with the standard prefix instead of falling back silently:

```text
[freeze:contract][hako_mirbuilder]
```

Do not replace missing metadata with a backend shortcut.

## Stop Lines

```text
do not add representation truth to selfhost MIRBuilder
do not add publication truth to selfhost MIRBuilder
do not add backend direct route truth to selfhost MIRBuilder
do not use metadata as a HostHandle bypass proof
do not use metadata as an Arc retirement proof
```
