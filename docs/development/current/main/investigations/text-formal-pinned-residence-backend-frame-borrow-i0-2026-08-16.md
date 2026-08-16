---
Status: active caller-zero implementation row
Date: 2026-08-16
Work mode: fast
Parent: LOOP-TEXT-SLICE-DIRECT-AOT-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BORROW-I0

```text
Decision: add one scoped Rust PinnedTextBackendFrameBorrowV1 view over the
existing PinnedTextBackendFrameContractV1; keep the C validator and JSON
schema unchanged and keep direct lowering closed.
Source authority + canonical issuer: PinnedTextBackendFrameContractV1 remains
the sole issuer over the physical-signature, plan/census, Residence ABI, and
compile-target facts; borrow() only lends those existing facts for one Rust
callback and never reissues them.
Non-authority: runtime TextFormalCallResidenceV1 roots/tokens, raw ptr/len,
ValueId, JSON numeric IDs, C-created meaning, generic MIR shape, target/env
inference, lifecycle state, and any second residence issuer.
Fail-fast boundary: foreign/expired owner or plan borrow, contract/schema
drift, runtime-field exposure, or an attempted escape rejects before any
backend effect; the view cannot be constructed from JSON or a runtime token.
Smallest next slice: implement the Rust-only borrow view, expose read-only
non-pointer getters, and prove projection equivalence plus scoped lifetime;
leave C/JSON/runtime/lifecycle/GEP/load untouched.
Non-claims: no typed leaf lowering, pointer materialization, UTF-8 execution,
runtime frame entry/finish, session adoption, route, production caller,
fallback/retry, or C-speed claim.
```

## BoxShape

`PinnedTextBackendFrameContractV1::borrow(&self)` returns a private,
non-`Clone`/non-`Copy` `PinnedTextBackendFrameBorrowV1<'_>`. The view carries
only a shared lifetime tied to the contract and read-only projections of its
already-co-sealed facts:

```text
contract/schema identity
owner + invocation stamp
plan stamp + row count
receiver/formal/callable lane counts
ExactText root count
Residence revision + derived frame size
target profile + consumer/object-emitter revisions
```

It must not expose a runtime pointer, byte length, lease token, slot,
generation value, `ValueId`, mutable reference, or JSON-owned field. The
existing `to_transport_json` remains the only serialization projection and
continues to receive the owned contract, not a borrow that escapes Rust.

## Implementation boundary

The only production file changed by this row is
`src/mir/compiler/pinned_text_backend_frame.rs`; focused tests live beside the
view. `src/runner/mir_json_emit/metadata.rs`, the C contract validator, the
TargetMachine session, `TextFormalCallResidenceV1`, and all MIR lowering code
remain untouched. This is a behavior-preserving ownership seal, not a new
semantic or runtime receipt.

## Acceptance

```text
positive: borrow exposes the existing contract identity/count/profile facts
          and its transport projection is semantically identical to the
          owned contract projection
negative: no Clone/Copy or constructor from JSON/raw fields; no mutable or
          runtime fields; borrow cannot outlive the contract; no second issuer
guard: source remains <800 lines and the existing transport smoke/pointer
       guard stays green
```

After this row, the next design stop is still the Residence/lifecycle bridge
needed for a live runtime frame. This row does not authorize GEP/load or a
direct `PinnedTextOp` consumer.
