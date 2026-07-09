# 3415 - MIRBUILDER-SCALAR-KNOWN-AUTHORITY-CLAIM-TAXONOMY-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-AUTHORITY-CLAIM-TAXONOMY-BASIS-001
```

## Purpose

Define a taxonomy for ScalarKnown authority and forbidden-proof claims before
the Delete surface decision. This is a naming and audit layer only.

Legacy claim names remain valid detail claims. New cards should map new claim
names to this taxonomy instead of adding ungrouped flag names.

## Taxonomy

```text
authority.surface.delete.route_decision
authority.surface.write.wide
authority.runtime.mutation
authority.runtime.publication
authority.scalar_known.global_route
authority.backend.lowering
authority.caller_orientation.runtime_path
authority.source_selfhost

proof.forbidden.manual_selection
proof.forbidden.counts
proof.forbidden.location_or_name
```

## Claims

```text
authority_claim_taxonomy_basis = 1
legacy_claim_names_preserved = 1
new_claims_must_map_to_taxonomy = 1
taxonomy_is_documentation_layer_only = 1
selected_next_card = MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-DELETE-SURFACE-AUTHORITY-DESIGN-STOP-001
```

## Non-Claims

```text
authority_semantics_changed = 0
legacy_claims_deleted = 0
route_authority_switch = 0
delete_hako_route_decision_authority_pilot = 0
mapdeleteany_authority = 0
write_surface_authority_closeout = 0
write_wide_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0
runtime_fallback = 0
```
