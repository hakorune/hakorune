# 209x-90 Agg Local Scalarization Owner Seam SSOT

Status: SSOT

## Goal

- fold the landed pilot metadata into one generic agg_local scalarization owner seam
- keep the current pilot rows as evidence, not as permanent top-level owners

## In Scope

- sum placement layouts
- thin-entry inline-scalar user-box local-body selections
- typed-slot storage inventory from current value storage classes
- the first read-only agg_local route view exported from MIR metadata

## Pilot Boundary Notes

- sum placement keeps the family-specific proof chain
- thin-entry keeps the family-specific selection chain
- storage-class inventory stays a read-only owner-neutral scaffold
- agg_local scalarization owns the folded view, not the family-specific proof owners

## Fixed Decisions

- the first code slice is behavior-preserving
- no new backend semantics are introduced in this phase
- the next follow-on after this owner seam is the broader actual agg_local scalarization layer

## Out of Scope

- lowering or backend consumer switch
- DCE / simplification bundle changes
- string / sum / user-box / array / map semantic widening

## Acceptance

- docs point at `phase209x` as the current code phase
- the MIR owner seam is implemented without changing runtime behavior
