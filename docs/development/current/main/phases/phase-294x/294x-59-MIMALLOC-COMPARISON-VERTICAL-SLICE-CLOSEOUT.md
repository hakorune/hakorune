---
Status: Landed
Date: 2026-05-23
Scope: V5 mimalloc comparison vertical-slice closeout.
Blocker: MIMALLOC-COMPARISON-VSLICE-007
Related:
  - docs/development/current/main/phases/phase-294x/294x-58-MIMALLOC-COMPARISON-HUGE-OSVM-SLICE-PILOT.md
  - apps/hako-alloc-mimalloc-comparison-vertical-slice-closeout-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh
---

# 294x-59 Mimalloc Comparison Vertical Slice Closeout

## Decision

Close the comparison-quality vertical slice by aligning the selected `.hako`
`hako_alloc` slices with the existing C mimalloc explicit runner planning
surface.

The closeout proof records:

```text
V2 small fixed-size alloc/free/reuse schema
V3 realloc same-class / grow fallback / aligned-small schema
V4 huge / OSVM-backed schema
C mimalloc explicit runner planning contract fields
```

This is a schema closeout, not a native allocator replacement claim. The C
mimalloc runner evidence remains the explicit external-runner planning surface,
while the `.hako` side exposes comparable requested-byte, committed/live,
operation-count, failure/reject, and closed stop-line fields.

## Stable Output

The closeout proof emits:

```text
schema
hako_slices
hako_requested
hako_evidence
hako_details
c_mimalloc
schema_bridge
closed
summary
```

The hako side still has no RSS/memory-use evidence. The C mimalloc lane has RSS
planning evidence. This asymmetry is explicit in `schema_bridge` and is the
boundary for the next row.

## Validation

The closeout guard reuses the existing V2, V3, V4, and C mimalloc planning
guards, then runs the V5 proof app through VM execution, MIR JSON emit, and
route preflight.

V4 remains the representative exact-MIR pure-first EXE evidence for the
OSVM-backed slice. This row does not add a second EXE owner.

## Stop Line

This row does not open:

- C mimalloc execution beyond existing explicit runner planning evidence;
- provider activation;
- host allocator replacement;
- hooks;
- `#[global_allocator]`;
- TLS / worker-local behavior;
- remote-free stress;
- atomic bitmap execution;
- native allocator replacement claims;
- backend owner-name matchers.

## Next Blocker

```text
MIMALLOC-COMPARISON-VSLICE-008:
  select the post-closeout follow-on: either add hako-side memory-use evidence
  for the comparison schema or return to the next explicit `usize` field-group
  row. Do not open provider activation or host replacement by default.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
