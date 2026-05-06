# Phase 293x: real-app bringup

- Status: Active
- Purpose: use real applications to expose compiler/runtime seams after the
  Program(JSON v0) cleanup lane, without adding `.hako` workarounds for real
  compiler blockers.
- Active lane token: `phase-293x real-app bringup`
- Current blocker token: `phase-293x real-app bringup order: BoxTorrent mini -> binary-trees -> mimalloc-lite -> allocator port`

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

## Current Status

- `293x-001`: BoxTorrent mini local content store landed.
- Next: binary-trees allocation/shape benchmark app.
