Status: LANDED
Phase: 275x

# 275x-90 PGO Scaffold SSOT

## Decision

- accepted

## Scope

- add one owner seam for `PGO` policy / artifact vocabulary
- keep profile generate/use behavior disabled
- keep `ThinLTO` behavior unchanged

## Owners

- `ipo_build_policy`
- `pgo_build_policy`

## Contract

- `PGO` is a dynamic hotness overlay owner, not a closure-meaning owner
- `PGO` must remain separate from the landed callable-node / call-edge facts
- this phase only adds scaffold vocabulary for later generate/use cuts

## Out Of Scope

- enabling instrumentation
- reading merged profile artifacts
- changing `ThinLTO`
