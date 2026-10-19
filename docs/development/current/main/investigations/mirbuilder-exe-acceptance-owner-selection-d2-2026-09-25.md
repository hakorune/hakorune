# APP-allocator-stress

Status: selected; implementation in progress.
Date: 2026-10-19
Scope: unchanged allocator-stress, physical EXE, exact output/exit and retained handle safety.
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[existing retained-handle obligation, row 7](mirbuilder-exe-acceptance-suite-red-disposition-d21-2026-09-27.md),
[parked untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md).

## Selection and current stop

- User authorized skipping the parked untyped-field app and selecting one runnable-owner target.
- Select allocator-stress: it shares HakoAllocHeap with passing mimalloc-lite and adds
  local arrays retaining allocator results. Binary-trees/JSON also depend on untyped slots;
  JSON further needs unfinished Text Return/Call and Loop connections.
- Fixed suite at revision `751dae566b`: 6PASS/5FAIL; actual EXE successes 5, empty marker 1,
  SKIP/timeout 0. Mimalloc is green. No claim that the remaining apps or full goal are complete.
- Both existing selfhost-build smoke and explicit direct EXE stop before artifact generation:
  `CoreMethodSource { NamedArray(TextSourceMissing) }`.
- First source demand: `apps/allocator-stress/main.hako:16`,
  `small_handles.push(heap.allocate(1 + (i * 4)))` inside the first loop.
- `named_array_method.rs` selects the named-Array Text push requirement;
  `resolved_semantics/named_array_requirement.rs` requires literal Text or TextToCaller.
  The allocator result is a nullable owned object handle, not Text.

## Implementation direction

- Existing `named_array_append_only_source.rs` retains original handle/nullable append candidates,
  but currently permits only top-level push sites (`segments.len() == 1`).
- Reuse `selected/owned_array.rs::emit_append` and `ArrayHolderLifecycleRecipeV1` for this demand.
  Connect exact Array acquisition, instance-call result, element residence, loop iteration,
  later get/release, and Normal/Fault cleanup before publication.
- Do not remove the TextSourceMissing check, classify the handle as Text/Integer,
  infer ownership from MIR, or weaken existing named-array negative tests.
- Preserve the original source and all expected accounting, including rejects=4 and exit0.
- Retire only replaced exclusive adapters once callers and required validation close.
  No new card/test file/checker/hook; reuse current focused source/Array contracts.

## Verification and next step

- Selection probes used the source-compatible quick CLI, explicit lifecycle runtime and
  C API FFI; all three candidate direct routes reproduced their known pre-artifact stop.
- Existing smoke: `bash tools/smokes/v2/profiles/integration/apps/allocator_stress_exe.sh`.
- Next: extend original append source control coverage together with Loop Home observation and
  the existing owned-array physical consumer; do not merely remove the top-level restriction.
- Reuse `array_actual_retains_original_shared_loan_and_refuses_executable_upgrade` and
  current owned-array emission/cleanup tests. Preserve candidate cancellation, outer holder
  lifetime and original nullable result identity across each iteration.
- Prior D2 root-wrapper selection history remains in this path's Git history;
  this existing card is reused instead of adding a new app card.
