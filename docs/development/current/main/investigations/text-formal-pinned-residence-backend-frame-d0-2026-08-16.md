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

Decision: require one typed compile-time backend-frame contract keyed by the
existing physical-signature occurrence order, plan stamp, and root-row index;
the direct binder may consume and project its validated view, but may not
issue it. The actual runtime frame remains a separate Residence-owned object.
Source authority: the existing physical signature/pair lanes provide the
occurrence order, while the runtime atomic lease-set/residence receipt
provides immutable StableText backing and finish ownership. Canonical issuer:
the existing Residence owner (`TextFormalCallResidenceV1`, acquired by
`acquire_text_formal_residence_v1`) remains the sole issuer of actual frame
state (revision, root count/order, backing rows, and exactly-once finish),
while the not-yet-landed `PinnedTextBackendFrameContractIssuerV1` is intended
to be the sole compile-time projection issuer; the ny-llvmc Boundary
pure-first C binder is a mechanical consumer.

## Private bridge contract (design-only)

The compile-time side is a projection contract, not a second Text or lifetime
authority. The successor issuer consumes the existing physical-signature
occurrence order, the function-local `PinnedTextAccessPlanTableV1`, and the
final pinned-lifetime census, then co-seals one compile-time contract named
`PinnedTextBackendFrameContractV1` containing:

```text
plan_stamp
residence_frame_revision
target triple + data-layout capability
pointer width/alignment and ABI revision
header/root-row offsets and sizes
occurrence-ordered root count and coverage
```

It contains no `ValueId`, pointer, length, runtime token, `BindingRef`, or
route choice. This compile-time contract is not the runtime frame and does
not claim a live residence. The binder receives a scoped validated view of
this contract and may only project the three already-fixed `PinnedTextOp`
leaves. A JSON residence table and a raw pointer/length field in MIR are
forbidden.

The compile-time issuer is a mechanical aggregate seam named
`PinnedTextBackendFrameContractIssuerV1` for the successor implementation. It
consumes existing plan/lifetime facts and the target-layout capability; it
does not become a new Text semantic owner. The runtime issuer remains
`TextFormalCallResidenceV1` and is the only owner of actual backing pointers,
pin records, and finish. The two products are linked by the same function
stamp and occurrence order, never by a caller-reconstructed pointer/token
pair. The runtime frame remains owned by `TextFormalCallResidenceV1`; the
compile-time contract is only its target/layout projection.

### Typed JSON metadata handoff (candidate BoxShape; not issuable yet)

Once the missing package-to-function co-seal exists, the existing MIR JSON
function metadata is the preferred single transport boundary for the
compile-time contract. It would carry a versioned descriptor, not residence
state:

```json
{
  "pinned_text_backend_frame_contract_v1": {
    "stamp": 123,
    "frame_revision": 1,
    "target_profile": "nyrt_text_residence_v1_ptr64_as0",
    "header_size": 32,
    "root_row_size": 16,
    "root_count": 2,
    "receiver_lane_count": 1,
    "physical_formal_lane_count": 4,
    "physical_callable_lane_count": 5,
    "rows": [
      {"root_index": 0, "formal_ordinal": 0, "slot_lane": 1, "generation_lane": 2},
      {"root_index": 1, "formal_ordinal": 1, "slot_lane": 3, "generation_lane": 4}
    ]
  }
}
```

The descriptor is a transport projection of the existing physical-signature
and Residence facts. The ny-llvmc consumer parses it into one scoped,
non-rebindable validated view and rejects unknown/missing fields, stamp or
plan-census drift, non-contiguous or duplicate lane rows, receiver overlap,
and target/layout mismatch. `target_profile` is a fixed capability id, not a
free-form triple from which the backend may infer ABI facts. `root_index`
names ExactText formal occurrences only; the receiver and ordinary scalar
formals never receive Residence root rows. JSON must never contain
`lease_token`, pointer, byte length, runtime slot/generation values, or
`ValueId`.

This is not an out-of-band sidecar and not a second authority once issued: the
future issuer must co-seal the descriptor with the installed physical-signature
row, the active `MirFunction` plan stamp/census, the target capability, and the
final lifetime census; the JSON emitter only serializes it; ny-llvmc only
validates and consumes it. The runtime frame is still entered, pinned, and
finished by the runtime Residence owner. At the current design stop no issuer
may publish this descriptor because that package-to-function co-seal path does
not exist yet.

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
This I0 scope is complete. Its evidence is limited to the caller-zero runtime
frame contract; it does not issue the compile-time binder contract or publish
JSON metadata. Keep the compile-time bridge, GEP/load, lifecycle CFG, session
adoption, route, and caller behind the successor design seam; no JSON residence
table or public ptr/len ABI is allowed.
Non-claims: no callable ABI aggregate, public ptr/len, production TextEq
route, C-speed result, StringBox/literal origin, fallback/retry, or main
integration.

Acceptance requires one issuer direction, one opaque capability input, one
occurrence-ordered row mapping, one target-layout capability, atomic
all-or-nothing publication/rollback, and no detached pointer/token ownership.
Those conditions are now the accepted BoxShape. The bounded successor,
caller-zero `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-I0`, is landed as the
runtime frame bridge; the compile-time binder below remains the live design
stop.

## TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-I0

Landed implementation scope was one behavior-preserving caller-zero substrate:
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

Current bridge census: the physical row is available through the installed
package/Port (`S6CInstalledCallableLoanRefV1::signature()`), while the active
`MirFunction` owns `metadata.pinned_text_access_plans`; the runtime Residence
owner is separate. No current path co-seals these three with the target
capability. The named issuer is therefore a successor design seam, not an
existing callable or a permission to reconstruct identity from a batch slot,
function name, JSON length, or `ValueId`.

The existing lowering order makes the missing seam concrete, but the
selected-normal route is not yet the canonical resolved route:

```text
NormalCallableSemanticPackagePortAdapterV1
  -> lower_cataloged_static_box_method / lower_cataloged_instance_box_method
RawInvocationChildPortV1
  -> lower_normal_*_with_source_v1
capture_*_pending_v1
  -> CanonicalFunctionLoweringSessionV1
  -> PreparedFunctionSessionCommitInputV1
  -> MirFunction metadata / JSON
```

For the selected static/instance cataloged methods, the concrete calls are
currently `capture_static_box_method_pending_v1` or
`capture_normalized_instance_box_method_pending_v1`; both delegate to
`capture_legacy_function_pending_session_v1` and then
`commit_normal_cataloged_box_method_pending`/`commit_legacy_symbol_pending`.
The inner session type is canonical in name, but the admission and collector
path are still `LegacyChildDraftAdmissionV1` and the legacy symbol collector;
they do not carry the installed physical-signature row or a resolved owner.
The binder must therefore not be attached to this legacy pending tuple, and a
legacy collector commit cannot be treated as the function-owned co-seal.
The existing `ResolvedChildDraftAdmissionV1`, `capture_resolved_pending`,
`complete_resolved_child`, and `commit_resolved_pending` are the disconnected
successor seams to audit first. Top-level rows remain explicitly unsupported
until a source-backed physical-signature row is named; they are not silently
folded into the cataloged method path.

Only the S6C callback currently exposes a physical-signature row, and that
loan is profile-specific. Ordinary selected methods reach the function
session without any signature row, so the binder cannot silently treat the
S6C loan, a physical header, or a function `ValueId` list as a universal
signature. The successor design must choose one combined package Port loan or
an equivalent function-entry handoff that covers every admitted method shape,
or explicitly keep unsupported shapes at `RejectBeforeEffect`.

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

Live blocker: no existing path co-seals
`InstalledNormalCallableSemanticPackageV1`'s physical-signature row,
`MirFunction::metadata.pinned_text_access_plans`, the runtime Residence frame
capability, and the selected ny-llvmc target capability. The typed JSON
descriptor remains unissuable until one canonical lowering/session handoff
names that issuer and validates owner/stamp/order without re-deriving source
identity. A JSON residence table, numeric-token lookup, out-of-band sidecar,
or backend-side reissue is not an acceptable handoff.

Smallest next slice: design the single package-to-function bridge at the
canonical lowering/session handoff. It must consume the installed signature
loan, the active stamped plan/census, and one target capability, then issue a
private non-pointer contract for the sole ny-llvmc consumer. Do not publish
JSON metadata or open GEP/load, lifecycle CFG, session values, route
admission, production caller, fallback, or retry until that co-seal is closed.

### Successor taskization (design stop; no implementation permission yet)

The next bounded design slice has five ordered seams; it is not yet an I0.
The first seam is a resolved-session cutover design, not a binder
implementation:

1. **Resolved-session handoff:** design one selected-normal transition from
   the package Port adapter through identity-only
   `ResolvedChildDraftAdmissionV1` into
   `capture_resolved_pending`/`complete_resolved_child`, while the same scoped
   HRTB closure lends a sibling
   `ResolvedCallablePhysicalSignatureLoanV1<'loan>`. The package-side seam is
   design-only `with_selected_cataloged_lowering_input_and_signature`; the
   module-side seam is design-only
   `complete_resolved_child_with_physical_loan<'loan>`, which receives the
   loan only as a synchronous completion-closure argument. The admission
   remains the canonical owner identity; the physical-signature loan is not
   stored in the admission, module port, collector, or legacy pending tuple.
   The loan must remain scoped until the draft's plan/census and target
   capability are co-sealed before
   `PendingFunctionSessionCloseV1::complete_before_restore` reaches the
   canonical `CanonicalRejectDuplicate` collector. The transition must cover
   the admitted static/instance method rows with one same-cohort loan;
   unsupported top-level rows stay `RejectBeforeEffect`. Do not attach the
   binder to `LegacyFunctionPendingSessionV1` or `commit_legacy_symbol_pending`.
2. **Bridge issuer after cutover:** name one canonical handoff at the
   resolved function/session boundary. It must consume the installed
   physical-signature loan (or a same-cohort all-method successor), the active
   function's stamped plan/census, `TextFormalCallResidenceV1`, and one
   target-layout capability through a private
   `PreparedPinnedTextBackendFrameContractInputV1<'loan>`. It must not infer
   package identity from a function name, `ValueId`, JSON length, or batch
   slot, and it must not make `ResolvedChildDraftAdmissionV1` a second
   signature authority.
3. **Private contract:** co-seal `PinnedTextBackendFrameContractV1` only after
   owner, stamp, receiver/formal order, root coverage, and target revision are
   proven. No runtime residence rows, pointer, length, token, slot/generation
   value, or `ValueId` may enter the product.
4. **Transport choice:** after (1)–(3), use the typed
   `pinned_text_backend_frame_contract_v1` JSON projection and a strict
   ny-llvmc scoped consumer. Until then, JSON publication is rejected rather
   than emitted as a partial descriptor. `llvm_py`, VM, native, and future Hako
   LLVM-text remain non-consumers.
5. **Evidence:** only after the issuer exists, cover mixed scalar/ExactText
   lanes, instance receiver prefix, aliasing caller occurrences versus distinct
   callee lanes, missing/duplicate rows, stamp/target/layout drift, unknown
   fields, and all forbidden runtime fields.

Completion of this design slice still does not authorize JSON publication,
GEP/load, lifecycle CFG, Canonical session adoption, route selection, a
production caller, fallback, or retry. Those remain separate bounded rows.

The design remains `NoSafeSlice::PinnedTextBackendFrameBinderUnsealed` if the
selected-normal route still enters `capture_legacy_function_pending_session_v1`
or `commit_legacy_symbol_pending`, if the signature loan is detached from the
same package/Port HRTB scope, if the admission is made to carry a second
physical-signature authority, or if the draft plan/census and target capability
cannot be co-sealed before canonical collection. It also remains stopped if
the completion closure can return, store, or otherwise outlive the physical
signature loan, or if the module port retains a package reference to make the
loan survive the HRTB callback.

Non-claims: this D0 does not make the direct backend C-fast, admit StringBox or
literal origins, or change the callable ABI. A missing single binder issuer or
target capability keeps `NoSafeSlice::PinnedTextBackendFrameBinderUnsealed`.
