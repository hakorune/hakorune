---
Status: caller-zero frame-contract I0 landed; compile-time binder D0 is the next design stop, production/backend route still closed
Date: 2026-08-16
Work mode: design_stop
Classification: T2 BoxShape candidate
Parent: TEXT-FORMAL-PINNED-RESIDENCE-I0 / LOOP-TEXT-SLICE-DIRECT-AOT-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-D0

This row closes the missing bridge between the caller-zero StableText
residence owner and the named ny-llvmc direct-AOT binder. It must not create a
second residence authority or turn MIR/JSON IDs into pointers.

Decision: require one opaque backend-private residence-frame capability keyed
by the existing physical-signature occurrence order, plan stamp, and root-row
index; the direct binder may consume and project it, but may not issue it.
Source authority: the existing physical signature/pair lanes provide the
occurrence order, while the runtime atomic lease-set/residence receipt
provides immutable StableText backing and finish ownership. Canonical issuer:
the existing Residence owner (`TextFormalCallResidenceV1`, acquired by
`acquire_text_formal_residence_v1`) remains the sole issuer direction for the
opaque frame capability (revision, root count/order, and exactly-once finish);
the ny-llvmc Boundary pure-first C binder is a mechanical consumer.

## Private bridge contract (design-only)

The compile-time side is a projection contract, not a second Text or lifetime
authority. The successor issuer consumes the existing physical-signature
occurrence order, the function-local `PinnedTextAccessPlanTableV1`, and the
final pinned-lifetime census, then co-seals one opaque contract containing:

```text
plan_stamp
residence_frame_revision
target triple + data-layout capability
pointer width/alignment and ABI revision
header/root-row offsets and sizes
occurrence-ordered root count and coverage
```

It contains no `ValueId`, pointer, length, runtime token, `BindingRef`, or
route choice. The binder receives this contract plus one opaque frame
capability keyed by the same stamp and root order; it may only project the
three already-fixed `PinnedTextOp` leaves. A JSON residence table and a raw
pointer/length field in MIR are forbidden.

The compile-time issuer is a mechanical aggregate seam named
`PinnedTextBackendFrameContractIssuerV1` for the successor implementation. It
consumes existing plan/lifetime facts and the target-layout capability; it
does not become a new Text semantic owner. The runtime issuer remains
`TextFormalCallResidenceV1` and is the only owner of actual backing pointers,
pin records, and finish. The two products are linked by the same function
stamp and occurrence order, never by a caller-reconstructed pointer/token
pair.

The runtime side remains the existing `TextFormalCallResidenceV1` owner. A
future private C-facing projection uses a target-checked frame layout with
`uintptr_t` pointer fields and signed `int64_t` byte lengths (lengths must fit
the fixed leaf result contract). All ExactText pairs are preflighted and
root rows are filled under the existing atomic residence transaction. No
partially filled frame becomes visible; a post-acquire publication failure
consumes the move-only residence and rolls back all pins. Body effects begin
only after the frame is complete.

The private V1 wire shape is intentionally narrow and target-bound: a
`repr(C)` header carries revision, header size, total size, and occurrence
count; each row carries one actual `const uint8_t*` backing pointer and one
`int64_t` byte length. The implementation must prove the target triple,
pointer width/address space, alignment, ABI revision, maximum root count, and
stack frame size before lowering. It must not assume that a pointer can be
round-tripped through `u64`, hard-code x86-64, or serialize the frame through
MIR JSON. The frame is caller-owned for the invocation and the runtime does
not retain its output-buffer pointer.

The entry contract takes the already-published occurrence-ordered
`[slot,generation]` lanes plus a caller-owned output frame and a fixed status
wire. It validates all pairs, backing class, lengths, frame metadata, and
input/output separation before the first pin or output write. Success commits
all pins and rows together. Any reject, overflow, or output-publication
failure leaves the registry unchanged; a post-acquire failure consumes the
move-only Residence owner exactly once for rollback.

Finish is not a semantic cleanup obligation. The later lifecycle successor
must consume the same opaque frame/residence owner and hand one finish claim
to the existing Completion/DraftSeal normal-exit projection. Trap/unreachable
is admitted only under the existing no-unwind policy; fallback and retry are
not recovery paths.
Non-authority: MIR/JSON numeric IDs, raw ptr/len, TextFormalBorrowV1,
generation recapture, StringSpan/ViewBox, nearby CFG, generic Load/Store,
native/llvm_py/VM, environment selectors, benchmark, fallback, and retry.
Fail-fast boundary: reject missing/foreign capability or stamp, root
index/count/order drift, incomplete/duplicate root coverage, lease/frame
detachment, stale/non-Text/unstable backing, frame-size/pointer-width/ABI
revision mismatch, escaped scope, unsupported leaf width/boundary, and any
raw pointer crossing before IR effect.
Smallest next slice: implement only the private bridge contract and its
caller-zero enter/rollback/finish evidence, with one target-layout capability
and one occurrence-ordered mapping. Keep C/Rust frame code, GEP/load,
lifecycle CFG, session adoption, route, and caller behind the successor seam;
the implementation must not add a JSON residence table or public ptr/len ABI.
Non-claims: no callable ABI aggregate, public ptr/len, production TextEq
route, C-speed result, StringBox/literal origin, fallback/retry, or main
integration.

Acceptance requires one issuer direction, one opaque capability input, one
occurrence-ordered row mapping, one target-layout capability, atomic
all-or-nothing publication/rollback, and no detached pointer/token ownership.
Those conditions are now the accepted BoxShape. The bounded successor is the
caller-zero `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-I0`; its incomplete
implementation remains the live stop:
`NoSafeSlice::PinnedTextBackendFrameImplementationUnsealed`.

## TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-I0

Implementation scope is one behavior-preserving caller-zero substrate:
publish the private `repr(C)` frame layout/capability, validate target-layout
metadata, exercise all-or-nothing enter and rollback through the existing
`TextFormalCallResidenceV1`, and consume one move-only finish. Keep the
compile-time binder transport-only, and do not add lifecycle MIR, GEP/load,
JSON residence state, Canonical session adoption, route admission, literal or
StringBox origin, a production caller, fallback, or retry. Focused positives
and negatives must cover occurrence order/alias multiplicity, target revision
and pointer-width mismatch, frame size/length overflow, partial publication,
stale/non-Text pairs, duplicate finish, and finish-after-rollback.

### I0 evidence (caller-zero frame contract)

The bounded runtime projection now exists without opening the compiler route:

- `TextFormalResidenceFrameHeaderV1` and `TextFormalResidenceRootRowV1` are
  fixed `repr(C)` transport rows; the callable wire remains the separate
  `{slot,generation}` pair.
- `enter_text_formal_residence_c_v1` validates the one supported target layout,
  pair/frame separation, frame capacity, and all pairs before publishing any
  row; the existing registry transaction owns pinning and rollback.
- `finish_text_formal_residence_c_v1` consumes the opaque lease token once and
  clears it; semantic cleanup, MIR lifecycle, and backend return placement are
  not part of this row.
- `hako_text_formal_residence_enter_v1` / `finish_v1` and
  `include/nyrt_text_formal_residence_v1.h` are transport-only projections.

Focused evidence: `text_formal_residence` 7/7 and
`nyash_kernel::exports::text_formal` 1/1; `cargo check --lib`, formatting,
pointer guard, and `git diff --check` are green. This proves only the
caller-zero frame contract, not a production caller or a direct Text backend.

## TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BINDER-D0

Decision: keep one compile-time binder as a mechanical projection from the
package-owned physical-signature occurrence order plus the runtime residence
frame capability; it may not issue Text meaning, lifetime, or route facts.

Source authority + canonical issuer: the physical-signature cohort owns lane
roles/order and the runtime `TextFormalCallResidenceV1` owns actual roots and
finish; a single post-install `PinnedTextBackendFrameContractIssuerV1` may
co-seal their existing stamps and target-layout capability for the selected
ny-llvmc Boundary consumer.

The count/order projection is fixed, not inferred from a symbol or JSON length:
`source_logical_arity` counts explicit source formals;
`receiver_lane_count` is one only for `InstanceBoxMethod`; and
`physical_formal_lane_count` is the sum of explicit-formal lane widths, while
`physical_callable_lane_count = receiver_lane_count + physical_formal_lane_count`.
Physical order is `[InstanceReceiver?]` followed by ordinal-ordered explicit
lanes. The receiver is not an ExactText formal and never receives a residence
root row; only ExactText occurrences map to occurrence-ordered roots. Callee
parameter ValueIds are pairwise distinct per lane, while caller argument
occurrences may alias; the two ValueId scopes are never compared.

The consumer census is intentionally narrow: the transition owner is
`hako_llvmc_compile_json_pure_first -> compile_json_compat_pure` and its
same-module C lowering path. `llvm_py`, VM, native, and future Hako LLVM-text
paths remain non-consumers until a separate decision changes the pointer.

Non-authority: logical `/N`, JSON parameter counts, ValueId/ptr/len guesses,
MIR generic Load/Store, `TextFormalBorrowV1` generation recapture, llvm_py/VM/
native alternatives, environment or benchmark selection, fallback, and retry.

Fail-fast boundary: reject receiver/formal occurrence drift, missing or
duplicate root rows, mixed ABI revision/target data-layout, pointer-width or
frame-size mismatch, detached residence/token, and any attempt to serialize
the private frame through MIR JSON or expose raw pointers to common MIR.

Live blocker: the current MIR JSON transport carries only plan stamp, leaf
kind, and root IDs; it cannot carry or reconstruct the opaque frame capability.
The D0 is not accepted until one in-process/scoped handoff is named that keeps
the frame capability, occurrence mapping, and target capability non-rebindable
at the ny-llvmc boundary. A JSON residence table, numeric-token lookup, or
backend-side reissue is not an acceptable handoff.

Smallest next slice: design-only census and acceptance for the binder's
function stamp, target capability, occurrence mapping, and one scoped handoff
to the existing ny-llvmc consumer; no GEP/load, lifecycle CFG, session
adoption, route admission, production caller, fallback, or retry.

Non-claims: this D0 does not make the direct backend C-fast, admit StringBox or
literal origins, or change the callable ABI. A missing single binder issuer or
target capability keeps `NoSafeSlice::PinnedTextBackendFrameBinderUnsealed`.
