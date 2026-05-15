# 293x-373 METADATA-PROMOTE-004 Placement Fold-Up Plan

Status: landed
Date: 2026-05-15

## Decision

`METADATA-PROMOTE-004` documents and guards the consumer fold-up plan for
`placement_effect_routes`.

This is a BoxShape docs / guard row only. It does not change MIR JSON shape,
Rust metadata structs, verifier behavior, backend lowering, runtime behavior,
or C shim reader behavior.

## Responsibility

Canonical wording lives in:

```text
docs/reference/mir/metadata-facts-ssot.md
```

Guard owner:

```text
tools/checks/mir_metadata_catalog_guard.sh
```

## Guarded Fold-Up Families

- String corridor route windows prefer `placement_effect_routes` rows with
  `source=string_corridor`; `string_corridor_candidates` remains a
  compatibility fallback until folded rows carry window/proof parity.
- Sum placement local aggregate reads prefer `placement_effect_routes` rows
  with `source=sum_placement`; `sum_placement_facts` and
  `sum_placement_selections` remain compatibility fallbacks until selected
  value/source-sum/manifest proof parity is covered.
- Sum local aggregate layout reads prefer `placement_effect_routes` rows with
  `source=agg_local_scalarization`; `sum_placement_layouts` remains a
  compatibility fallback until layout detail is represented by a folded route
  detail or generic layout route.
- Thin-entry reads prefer `placement_effect_routes` rows with
  `source=thin_entry`; `thin_entry_selections` remains a compatibility
  fallback until public/thin entry consumers no longer need family rows.
- String direct kernels stay on `string_kernel_plans` until their
  borrow/publication/carrier/text-consumer verifier facts have an equivalent
  generic route shape.

## Stop Lines

- Do not delete family-specific readers before the folded route carries the
  same proof, demand, publication boundary, and selected value identity.
- Do not make backend readers infer placement from raw helper names or app
  shapes while migrating.
- Do not fold `string_kernel_plans` into `placement_effect_routes` without a
  verifier-equivalent generic route shape.

## Evidence

```text
bash tools/checks/mir_metadata_catalog_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
