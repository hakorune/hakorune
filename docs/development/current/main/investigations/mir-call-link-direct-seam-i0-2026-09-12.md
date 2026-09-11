---
Status: selected implementation slice
Date: 2026-09-12
Decision: MIR-CALL-LINK-DIRECT-SEAM-I0
Parent: `docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md`
ProductionCaller: existing FFI `hako_llvmc_link_obj` and `hako_llvmc_link_obj_v2` forwarders
ReplacementCell: existing `hako_aot_link_obj_with_archive` via a private direct seam
---

# MIR-CALL-LINK-DIRECT-SEAM-I0

## Six-line brief

Decision: remove the FFI link route's process-global `HAKO_AOT_USE_FFI`
save/set/restore and call the existing single link body through one private,
invocation-owned seam.

Source authority + canonical issuer: v2 receives its explicit runtime archive
from the selected Rust/Boundary caller; v1 keeps its current compatibility
archive resolution at the C seam. The existing AOT link body remains the sole
physical link owner.

Non-authority: ambient `HAKO_AOT_USE_FFI` after route admission, dlsym symbol
names, a new public ABI, source/MIR text, or linker output cannot issue or
replace the runtime archive or route mode.

Fail-fast boundary: reject invalid object/executable/archive inputs before
linker effects, preserve the current diagnostics and output cleanup, and keep
public AOT v1/v2 dlsym behavior unchanged for external compatibility callers.

Smallest next slice: add one hidden cross-translation-unit direct seam, route
the two FFI link forwarders through it, and extend the existing explicit-link
smoke with v1/v2 and unset/empty/present environment observations.

Non-claims: compile-options ownership, concurrent compilation, public AOT ABI
retirement, dlsym/public compatibility retirement, LegacyCallV0 retirement,
MIRBuilder semantic changes, backend parity, and whole R7 completion.

## Allowed implementation

- add one internal header declaration and one hidden direct-link owner beside
  `hako_aot_link_obj_with_archive`;
- route the existing v1/v2 FFI exports to that seam without adding a public
  third link ABI or another physicalizer;
- retain public `hako_aot_link_obj` / `_v2` dispatch and recursive dlsym support
  for external compatibility callers;
- add focused C/shell evidence for valid, missing, link-failure, and exact
  unset/empty/present `HAKO_AOT_USE_FFI` preservation;
- keep `tools/checks/mir_call_link_direct_seam_guard.sh` as the stable static
  ownership/line guard for this slice;
- update the C ABI README, owner card, final-pipeline pointer and current state
  in the same closeout series.

## Authority and deletion boundary

```text
Rust/Boundary selected caller
  -> existing hako_llvmc_link_obj(_v2) export
  -> private hako_aot_link_obj_for_llvmc(mode, archive)
  -> hako_aot_link_obj_with_archive
```

For v2, `archive` is required and is never resolved from
`NYASH_EMIT_EXE_NYRT`. For v1, the seam resolves the existing compatibility
archive exactly once from the current v1 environment contract and passes the
result to the same link body. The private function is hidden from the public
ABI; the existing public AOT entry points continue to own dlsym re-entry.

The exclusive I0 delete-set is limited to the two FFI route helpers that
temporarily mutate `HAKO_AOT_USE_FFI` and the calls from those helpers into
public AOT dispatch. No public symbol, dlsym path, v1 caller, or archive
resolver is deleted.

## Acceptance

```text
positive: v2 explicit archive links while ambient NYASH_EMIT_EXE_NYRT is invalid
positive: v1 compatibility archive resolution links through the same body
negative: missing v2 archive rejects before an executable is published
negative: invalid object and linker failure retain existing typed diagnostics
guard: HAKO_AOT_USE_FFI unset, empty, and present values are byte-for-byte
       unchanged across both FFI routes
guard: FFI routes contain no setenv/restore for HAKO_AOT_USE_FFI and no
       recursive call to public hako_aot_link_obj(_v2)
guard: one direct-link body and one private seam; source files remain <800 lines
```

No actual LLVM/Cargo acceptance is claimed until the focused C smoke and
pointer/line guards run. The compile-options I1 design remains the next
separate Task 3 dependency and is not silently closed by this link slice.

## Closeout evidence (2026-09-12)

Passed:

```text
bash tools/checks/mir_call_link_direct_seam_guard.sh
[mir-call-link-direct-seam] ok

bash tools/checks/dynamic_v2_w6_explicit_link_abi_smoke.sh
[dynamic-v2-w6-explicit-link-abi] ok

bash tools/checks/current_state_pointer_guard.sh
[current-state-pointer-guard] ok
```

The C smoke covers v2 explicit archive linking with an invalid ambient
`NYASH_EMIT_EXE_NYRT`, v1 compatibility archive resolution, missing archive
rejection, linker failure without executable publication, and preservation of
unset/empty/present `HAKO_AOT_USE_FFI`. The existing broad
`dynamic_v2_aot_activation_authority_guard.sh` remains a known baseline red at
its pre-existing DraftSeal proof-retirement count (`drop(completion)` and
`drop(receipt)` each occur twice); no DraftSeal file is part of this slice, so
that result is not attributed to the link seam.

The link direct seam I0 is closed. Compile-options I1, public AOT ABI
retirement, dlsym compatibility retirement, concurrent compilation, and whole
R7 completion remain open and are not claimed.
