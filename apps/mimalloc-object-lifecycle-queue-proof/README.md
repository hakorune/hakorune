# mimalloc object lifecycle queue proof

Decision: accepted for `MIMAP-012`.

This app proves the object-backed lifecycle queue route. The queue retains real
`HakoAllocPageModel` objects in `ArrayBox`, skips decommitted pages, reuses a
retired page by calling page lifecycle methods, then scans past an unavailable
third slot and selects an active fourth-slot page.

Acceptance backend: LLVM/EXE primary.

VM remains diagnostic only for this object-heavy route. VM green is useful but is
not a MIMAP-012 completion requirement.
