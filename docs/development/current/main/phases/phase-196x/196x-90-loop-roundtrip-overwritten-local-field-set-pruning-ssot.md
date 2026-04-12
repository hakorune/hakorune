# 196x-90 Loop-Roundtrip Overwritten Local Field-Set Pruning SSOT

Accepted
- prune a predecessor-local `FieldSet` across one backedge roundtrip only when:
  - the carried root stays same-root local
  - the backedge predecessor has a single reachable successor into the loop header
  - the loop header overwrites the same root/field before any same-field read or escape use

Rejected
- mixed-root phi merges
- multi-round loop reasoning
- generic `Store` / `Load`

Reading rule
- this cut stays inside lane A of `phase190x`
- after this cut, lane A is closed and the next DCE step is lane B0
