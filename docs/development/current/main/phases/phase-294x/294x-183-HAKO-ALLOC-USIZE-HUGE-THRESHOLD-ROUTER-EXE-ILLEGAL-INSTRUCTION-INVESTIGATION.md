---
Status: Landed
Date: 2026-05-24
Scope: investigation row for the deferred huge-threshold router exact `usize`
  observer migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-182
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-180-HAKO-ALLOC-USIZE-HUGE-THRESHOLD-OBSERVER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-181-HAKO-ALLOC-USIZE-HUGE-THRESHOLD-OBSERVER-DEFER-ALIGNED-PADDED-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-182-HAKO-ALLOC-USIZE-ALIGNED-SMALL-PADDED-SIZE.md
  - lang/src/hako_alloc/memory/huge_threshold_router_box.hako
  - lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako
---

# 294x-183 Hako Alloc Usize Huge-Threshold Router EXE Illegal Instruction Investigation

## Decision

Keep the direct migration of these `HakoAllocHugeThresholdRouter` observers
deferred:

- `last_padded_size`
- `last_good_size`
- `last_huge_threshold`

This row records the pure-first EXE failure that appears when those three
router-local observers are temporarily migrated from `i64` to exact `usize`.
It does not change source code or broaden the migrated field group.

## Finding

The failure is not in parsing, MIR emission, route preflight, or EXE linking.
Those phases complete.

The failure is in the generated pure-first EXE at runtime:

```text
Program received signal SIGILL, Illegal instruction.
0x000000000041084a in HakoAllocHugeThresholdRouter.allocateAligned/2 ()
=> 0x41084a <HakoAllocHugeThresholdRouter.allocateAligned/2+1002>: ud2

#0 HakoAllocHugeThresholdRouter.allocateAligned/2
#1 HakoAllocHugeThresholdRouter.allocateAlignedUsize/2
#2 ny_main
#3 main
```

The generated `ud2` is the fail-fast target for exact numeric field helper
failures in this function. The temporary migration causes the emitted code to
exercise the `field_get_u64_hii` / `field_set_u64_hiu` path for the router
observer slots.

## Reproduction Shape

Temporary source-only probe:

```hako
box HakoAllocHugeThresholdRouter {
    last_padded_size: usize = 0
    last_good_size: usize = 0
    last_huge_threshold: usize = 0
}
```

Reproduced with the huge/OSVM comparison proof app:

```text
apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/main.hako
```

Artifacts were written under:

```text
target/investigations/294x-router-usize-illegal/
```

Key phase results:

```text
MIR emit:       ok
route preflight ok: [pure-first-route][ok] layer=route-preflight functions=101 plans=226
EXE build:      ok
EXE run:        SIGILL / run_rc=132
```

The optional missing `libnyash_integer_plugin.so` warning is not treated as the
root cause here. The same warning appears in green comparison guards, and the
program reaches `HakoAllocHugeThresholdRouter.allocateAligned/2` before the
fail-fast instruction.

## Disassembly Evidence

The failing function contains a single shared `ud2` fail-fast target. After the
temporary router migration, the exact `usize` observer slots are lowered through
the exact unsigned field helper lane:

```text
field_set_u64_hiu(router, slot=0xb, ...)
field_set_u64_hiu(router, slot=0xc, ...)
field_set_u64_hiu(router, slot=0xd, ...)
```

The nested small-path call path then reads the child path's exact padded-size
observer and copies it back to the router exact observer:

```text
field_get_hii(router, slot=0)                  ; small_path
HakoAllocPageMapAlignedSmallPath.allocateAlignedSmall/2
field_get_hii(router, slot=0)                  ; small_path
field_get_u64_hii(small_path, slot=0xd)        ; child last_padded_size
if negative -> ud2
field_set_u64_hiu(router, slot=0xb, value)     ; router last_padded_size
if helper returns 0 -> ud2
```

At the captured fail-fast point, registers included:

```text
rax = -1
rbx = -6
rip = 0x41084a <HakoAllocHugeThresholdRouter.allocateAligned/2+1002>
```

This suggests the crash is in the exact typed-object field helper / generated
EXE fail-fast lane, not in source-level allocator policy.

## Interpretation

The known-good row `294x-182` proves that
`HakoAllocPageMapAlignedSmallPath.last_padded_size: usize` is accepted when the
router observers remain signed.

The failing probe adds exact `usize` router observers in the parent function
that:

1. stores exact `usize` fields on `me`;
2. calls a child object method;
3. reads an exact `usize` field from that child object;
4. stores the read value back into an exact `usize` field on `me`;
5. increments exact `usize` route counters in the same function.

That is the next minimal compiler/backend fixture. Do not continue allocator
field migration for the router observers until that shape is fixed and covered.

## Next Compiler Fixture

Add a minimal pure-first EXE fixture with no allocator policy noise:

```text
parent box:
  child: Child
  copied_size: usize = 0
  success_count: usize = 0
  failure_count: usize = 0

method route(size: usize): i64:
  me.copied_size = size
  local ok = me.child.run(size)
  me.copied_size = me.child.last_size
  if ok == 1 { me.success_count += 1 } else { me.failure_count += 1 }
  return ok

child box:
  last_size: usize = 0
  run(size: usize): i64:
    me.last_size = size
    return 1
```

The fixture should prove:

- exact `usize` parent field stores work before and after a nested method call;
- exact `usize` child field reads work after the nested method call;
- exact `usize` parent counters are not corrupted by the child call;
- generated EXE does not route helper failures to `ud2`;
- the same MIR passes route preflight and pure-first EXE execution.

## Stop Line

Do not migrate the huge-threshold router size observers yet:

- `HakoAllocHugeThresholdRouter.last_padded_size`
- `HakoAllocHugeThresholdRouter.last_good_size`
- `HakoAllocHugeThresholdRouter.last_huge_threshold`

Do not work around this by inlining the `.hako` source path or avoiding the
child method call. The compiler/backend route should support the source shape,
or fail with a smaller fixture and clearer owner.

## Verification

Docs-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
