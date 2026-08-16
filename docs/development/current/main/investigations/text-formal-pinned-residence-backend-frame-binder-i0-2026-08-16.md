---
Status: active implementation row; Rust contract and strict profile transport landed, LLVM realization still closed
Date: 2026-08-16
Work mode: fast
Parent: TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-COSEAL-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BINDER-I0

```text
Decision: implement one private backend-frame contract issuer after the accepted four-input co-seal; A (Rust contract) is canonical, B (ny-llvmc actual-layout validation) is a consumer, and C host/default inference is rejected.
Source authority + canonical issuer: the existing physical-signature cohort lends complete lane rows, the Residence owner issues ResidenceAbiLayoutV1, the stamped access-plan census owns access coverage, and the same compile-invocation capability owns target expectation; one selected-normal close co-seals them into a non-Clone private contract.
Non-authority: logical /N, argument counts, JSON/MIR IDs, ValueId, BindingRef, raw ptr/len, runtime lease/token, NumericTarget::host(), environment variables, and backend reissue/defaults.
Fail-fast boundary: missing target/ABI/lane view, foreign invocation/owner/stamp, receiver/formal/root drift, overflow, unknown or missing transport fields, and actual LLVM layout mismatch reject before collector/IR effect.
Smallest next slice: add the scoped lane/Residence ABI projections, require target at the selected binder, issue unpublished typed metadata, and project only the versioned descriptor consumed by the existing pure-first ny-llvmc path; keep GEP/load and Text execution closed.
Non-claims: no pointer materialization, lifecycle CFG, Canonical session adoption, Completion finish, TextEq route, production caller, literal/StringBox origin, VM/llvm_py/native consumer, fallback, retry, or C-speed result.
```

## Contract boundary

The selected-normal close must hold these four sibling views in one
`complete_before_restore` HRTB:

```text
ResolvedCallablePhysicalSignatureLoanV1::lanes()
  + PinnedTextAccessPlanTableV1 final stamp/census
  + ResidenceAbiLayoutV1
  + &PinnedTextCompileTargetCapabilityV1
  -> private PinnedTextBackendFrameContractV1
```

The target reference is mandatory at this binder entry. `Option` remains
allowed only at compatibility edges before the selected binder narrows the
contract. The returned contract is owned and private; no loan, pointer, token,
or mutable package reference escapes the callback.

## Landed Rust subset

The first vertical slice is now implemented:

```text
ResidenceAbiLayoutV1 is issued by runtime/text_formal_residence
ResolvedCallablePhysicalSignatureLoanV1 lends complete lane rows
ExactText selected closes reject a missing target capability
the four-input contract is checked before collector mutation
unpublished MirFunction metadata retains the private contract summary
```

Focused evidence: the explicit Residence ABI layout test, physical-signature
lane tests (2/2), target-capability tests (3/3), and the normal package suite
(19/19) are green. This does not open the strict transport consumer.

## Required contract fields

The contract contains only mechanically checked facts:

```text
function/owner and compile-invocation stamps
physical callable/formal lane rows (role, ordinal, binding, physical index)
source logical arity, receiver count, formal/callable lane counts
ExactText dense occurrence/root count and checked frame-size bounds
plan stamp and exact access census
Residence ABI revision, header/root-row sizes, offsets, alignment, limits
target profile, triple/layout fingerprint, endian, address-space width/alignment
consumer and Residence ABI revisions
```

No `ValueId`, runtime slot/generation value, lease token, raw pointer, byte
length, route policy, or caller-provided root/frame count is admitted. Root and
frame sizes are checked derivations from the lane/occurrence rows and target
limits.

## Implementation order

1. Add a scoped complete-lane projection to the existing physical-signature
   loan; do not create a second cohort or copy lane rows.
2. Add the Residence-owned `ResidenceAbiLayoutV1` view over the existing frame
   ABI constants; do not expose live runtime residence state.
3. Change the selected binder seam to require the target capability and issue
   the private contract before collector mutation.
4. Store only unpublished typed metadata and emit a strict versioned transport
   projection for the existing pure-first ny-llvmc path. A dedicated validator
   remains a separate sub-760-line owner; it may compare actual LLVM layout but
   may not reissue the contract.

## Next bounded implementation cell

`BINDER-I0-TRANSPORT-STRICT` is landed as a transport-only cell.  The
reusable smoke proves the exact projection reaches the existing pure-first
consumer and that drift/unknown/missing fields reject before generic lowering.
It does not yet query the active LLVM `TargetMachine`/data layout.  The next
bounded cell is:

`BINDER-I0-TARGETMACHINE-LAYOUT`:

```text
MirFunction unpublished contract
  -> versioned JSON descriptor (required fields, no defaults)
  -> existing pure-first ny-llvmc parser
  -> actual LLVM triple/data-layout comparison
  -> reject before IR effect on any mismatch
```

The descriptor is a projection of the Rust-owned contract, not a second
authority. The current C consumer validates the selected profile row strictly;
it may issue only a private realization-validation receipt after a future
TargetMachine/data-layout query. It may not reconstruct lane/root counts,
derive a frame size from JSON lengths, probe the host, or replace the Rust
contract. Keep both cells caller-zero and transport-only: no GEP/load, pointer
materialization, lifecycle terminator, session adoption, TextEq route,
fallback/retry, or production caller is part of either task.

## Acceptance matrix

```text
positive: StaticBoxMethod, InstanceBoxMethod receiver prefix,
          mixed scalar + ExactText, repeated caller aliases, dense root rows
negative: missing/foreign target, ABI/lane/plan stamp drift, receiver/root
          overlap or reorder, lane gap/swap, root duplicate/omission,
          overflow, unknown/missing transport field, runtime field injection,
          loan escape, collector entry before co-seal, actual layout mismatch
```

The focused gate must prove contract identity and exact census. It must not
claim direct loads, pointer validity, actual LLVM layout realization,
lifecycle or production execution. The transport smoke is a required reusable
gate for the landed strict projection; the TargetMachine cell adds its own
positive/negative layout evidence before the blocker can move.

Reusable transport gate:

```bash
bash tools/checks/pinned_text_backend_frame_transport_smoke.sh
```

## Explicit stop conditions

```text
NoSafeSlice::PinnedTextBackendFrameCoSealUnsealed
  if the lane/ABI projections become a second authority, target is optional at
  the selected binder, or co-seal occurs after collector mutation.

NoSafeSlice::PinnedTextBackendFrameTransportReconstructed
  if JSON/C reconstructs meaning, fills defaults, or infers counts/layout.

NoSafeSlice::PinnedTextBackendFrameDirectLoweringMixed
  if GEP/load, Text execution, lifecycle CFG, or route selection enters this I0.

NoSafeSlice::PinnedTextBackendFrameTargetMachineLayoutUnimplemented
  while the pure-first consumer checks only the selected profile constants and
  has not compared the descriptor with the active LLVM TargetMachine/data
  layout.
```
