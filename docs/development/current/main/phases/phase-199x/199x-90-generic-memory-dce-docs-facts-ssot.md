# 199x-90 Generic Memory DCE Docs/Facts SSOT

Status: SSOT

## Goal

- finish lane B0 from `phase190x`
- define the first narrow facts contract before any generic `Load` / `Store` pruning

## Contract

- generic memory lane B owns only `Load` / `Store`
- local field lane A keeps owning `FieldGet` / `FieldSet`
- observer/control lane C keeps owning `Debug` / terminators
- first private-carrier roots are `RefNew`-rooted, definitely non-escaping local carriers with copy-only alias propagation

## Next

1. B1 dead `Load` pruning on private carriers
2. B2 overwritten `Store` pruning on the same private carriers

## Not Yet

- phi-carried pointer roots
- mixed public/private carriers
- store-to-load forwarding
- MemorySSA-style reasoning
