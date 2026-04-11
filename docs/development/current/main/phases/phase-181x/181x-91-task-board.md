# Phase 181x Task Board

- `181xA` docs/pointer lock
  - `phase181x` as active DCE cleanup lane
  - `phase180x` stays landed as the prior string seam cleanup corridor
- `181xB` Safepoint candidate slice
  - remove `Safepoint` as the first no-dst pure cleanup target
  - keep `Debug` and terminators out of scope
- `181xC` regression fixed
  - add a focused DCE unit for `Safepoint` removal
  - keep `KeepAlive` and reachable-only behavior unchanged
- `181xD` closeout
  - update the current pointer docs and move back to the broader DCE backlog
