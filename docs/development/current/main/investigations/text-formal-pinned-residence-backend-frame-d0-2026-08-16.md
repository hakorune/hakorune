---
Status: design stop; no frame ABI or backend implementation permission
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
Non-authority: MIR/JSON numeric IDs, raw ptr/len, TextFormalBorrowV1,
generation recapture, StringSpan/ViewBox, nearby CFG, generic Load/Store,
native/llvm_py/VM, environment selectors, benchmark, fallback, and retry.
Fail-fast boundary: reject missing/foreign capability or stamp, root
index/count/order drift, incomplete/duplicate root coverage, lease/frame
detachment, stale/non-Text/unstable backing, frame-size/pointer-width/ABI
revision mismatch, escaped scope, unsupported leaf width/boundary, and any
raw pointer crossing before IR effect.
Smallest next slice: design the exact private input/output frame contract,
revision and target-layout capability, occurrence-row mapping, atomic
publication/rollback, and finish handoff; no C/Rust frame code, JSON
residence table, GEP/load, lifecycle CFG, session adoption, route, or caller.
Non-claims: no callable ABI aggregate, public ptr/len, production TextEq
route, C-speed result, StringBox/literal origin, fallback/retry, or main
integration.

Acceptance requires one issuer direction, one opaque capability input, one
occurrence-ordered row mapping, no detached pointer/token ownership, and a
typed successor task for implementation. Until then:
NoSafeSlice::PinnedTextResidenceTransportBoundaryUnsealed.
