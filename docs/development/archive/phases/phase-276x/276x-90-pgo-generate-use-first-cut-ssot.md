Status: ACTIVE
Phase: 276x

# 276x-90 PGO Generate/Use First Cut SSOT

## Decision

- accepted

## Scope

- add the first narrow `PGO` generate/use cut
- consume the landed `PGO` scaffold owner seam
- keep `ThinLTO` unchanged

## Owners

- `pgo_build_policy`
- `ipo_build_policy`

## Contract

- `PGO` remains a hotness overlay owner
- `PGO` generate/use must stay separate from closure meaning and callable/edge facts
- this phase may widen one generate/use path only

## Out Of Scope

- changing closure semantics
- widening `ThinLTO`
- full optimization-lane closeout
