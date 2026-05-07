# Phase 293x: real-app bringup

- Status: Active
- Purpose: use real applications to expose compiler/runtime seams after the
  Program(JSON v0) cleanup lane, without adding `.hako` workarounds for real
  compiler blockers.
- Active lane token: `phase-293x real-app bringup`
- Current blocker token: `phase-293x birth/method call route expansion before real-app EXE parity`

## Order

1. BoxTorrent mini
2. binary-trees
3. mimalloc-lite
4. real allocator port

## Policy

- Real app code should stay simple and idiomatic.
- If an app needs a compiler expressivity improvement, fix the compiler seam
  first instead of hiding the issue in the app.
- Keep BoxShape cleanup separate from BoxCount acceptance expansion.
- Keep `phase-137x` observe-only unless app evidence reopens a real blocker.
- Do not start allocator optimization work before the preceding apps provide
  concrete ownership / allocation evidence.

## Smoke Entry

```bash
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
```

## EXE Boundary Entry

```bash
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```

This is a blocker probe, not EXE parity. TypedObjectPlan now covers declared
i64 fields, init-only untyped fields, handle storage, observed empty user
boxes, observed `newbox` argument storage flowing into same-module `birth`
parameters, same-module call argument storage propagation, and same-module
string-like global return storage. Conservative same-module `birth` and scalar
user-box method routes are available for the minimal typed-object fixtures.
BoxTorrent allocator page seeding now lowers through a MIR-owned void
side-effect global route, and `BoxTorrentManifest` now has a typed object plan;
the real-app boundary still stops at the broader `BoxTorrentChunker.ingest`
method body route seam.

## Current Status

- `293x-001`: BoxTorrent mini local content store landed.
- `293x-002`: binary-trees allocation/shape benchmark app landed.
- `293x-003`: mimalloc-lite allocator-shaped app landed.
- `293x-004`: real-app EXE boundary probe landed.
- `293x-005`: pure-first general-newbox owner decision landed.
- `293x-006`: `hako_alloc` VM-only page/free-list policy-state port landed.
- `293x-007`: allocator-stress app landed.
- `293x-008`: BoxTorrent allocator-backed store landed.
- `293x-009`: JSON stream aggregator app landed.
- `293x-010`: smoke env Hako alias cleanup landed.
- `293x-011`: config env Hako root/bin alias cleanup landed.
- `293x-012`: typed object EXE plan for general user-box `newbox` landed.
- `293x-013`: declared-i64 typed object EXE route for `newbox` plus
  `field_set` / `field_get` landed.
- `293x-014`: init-only untyped fields, handle storage, and observed empty
  user-box allocation landed.
- `293x-015`: typed user-box `birth` same-module EXE route landed for the
  conservative single-block body shape.
- `293x-016`: typed user-box scalar method same-module EXE route landed for the
  conservative single-block body shape.
- `293x-017`: typed-object birth-param storage inference landed for untyped
  fields initialized from observed `newbox` constructor arguments.
- `293x-018`: typed void side-effect global route landed for the BoxTorrent
  allocator page seeding chain.
- `293x-019`: typed object call-param / global-return storage flow landed,
  making `BoxTorrentManifest` plan-backed.
- Next: expand pure-first route coverage for `BoxTorrentChunker.ingest`; do not
  claim real-app EXE parity until that method seam lands.
