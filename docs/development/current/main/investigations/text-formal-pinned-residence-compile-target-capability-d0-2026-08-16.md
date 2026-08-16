---
Status: selected design stop; A/B authority split accepted, pre-MIR target-capability issuer unsealed
Date: 2026-08-16
Work mode: design_stop
Classification: T2 BoxShape decision
Parent: TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BINDER-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-COMPILE-TARGET-CAPABILITY-D0

```text
Decision: A is the canonical contract authority, B is a mandatory realization validator, and C is rejected.
Source authority + canonical issuer: one outer Rust compile invocation issues the explicit target capability before MIRBuilder; ny-llvmc consumes and validates it against the actual TargetMachine.
Non-authority: JSON, MIR, ValueId, NumericTarget::host(), runtime layout checks, environment/default target probing, and C TargetMachine discovery.
Fail-fast boundary: missing/late/foreign capability, invocation/function/target drift, default inference, duplicate ABI tables, or inability to lend the capability at complete_before_restore rejects before collection.
Smallest next slice: name the pre-MIR invocation owner, explicit V1 profile catalog, and typed capability transport into the resolved-session HRTB; no JSON or C consumer yet.
Non-claims: no binder contract, JSON projection, TargetMachine receipt, GEP/load, lifecycle/session route, production caller, fallback/retry, or C-speed claim.
```

## Decision

The canonical target contract is issued on the Rust side from an explicit
compile-invocation profile. The later ny-llvmc/LLVM boundary is a mandatory
validator of the realized target, not a second issuer. Host/default inference
is rejected.

The current code order is important:

```text
MIRBuilder / complete_before_restore
  -> collector
  -> MirFunction metadata / JSON
  -> Rust ny-llvmc boundary driver
  -> hako_llvmc_compile_json_pure_first
  -> LLVM TargetMachine
```

`crates/nyash-llvm-compiler` and the host-provider C-ABI route currently
receive MIR JSON after the function HRTB has ended. They may transport or
consume a capability, but they cannot be the sole canonical issuer if the
same capability must participate in `complete_before_restore`. The next
Decision must therefore name an outer compile invocation that exists before
MIRBuilder, or introduce one explicit pre-MIR request boundary. A downstream
driver value cannot flow backward and bless an already-collected function.

## Authority partition

One outer non-Clone `PinnedTextCompileTargetCapabilityV1` may expose scoped
views, but the facts remain partitioned:

```text
TargetIdentityAndLayoutExpectation
  compile invocation brand
  fixed target profile id
  target triple
  expected data-layout fingerprint
  endianness
  address-space-specific pointer width/alignment
  selected ny-llvmc consumer revision

ResidenceAbiLayout
  owned by the existing TextFormal Residence ABI
  frame/header/root-row revision, size, alignment, offsets

FunctionDerivedFrameLayout
  derived later from physical-signature occurrence order + plan/census
  root count = ExactText formal occurrence count
  checked total size = header size + root count * root-row size
```

The compile-target capability does not own Residence ABI facts, root count,
frame size, callable identity, plan rows, `ValueId`, runtime pointer/length,
or lease token. The later binder input borrows all three sibling authorities
and co-seals their exact relation. This prevents Rust and C from publishing
parallel layout tables and prevents target configuration from becoming a
second callable-signature authority.

## V1 profile catalog requirements

The issuer input must select one explicit, versioned profile from a closed
catalog. A free-form triple, environment variable, host probe, or missing-field
default is not a profile. Each row fixes:

```text
profile id
target triple
data-layout fingerprint
endianness
pointer address space / width / ABI alignment
consumer ABI revision
supported Residence ABI revision set
maximum root count / maximum private frame bytes
```

The maximums are target admission limits. Actual `root_count` and
`total_frame_size` are function-derived and are never accepted as profile
inputs. Checked derivation rejects overflow or limit excess without mutation.

## Current census

- `src/host_providers/llvm_codegen` chooses the pure-first route only after it
  receives MIR JSON; its `Opts` has no target capability.
- `crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs` chooses an FFI
  symbol and forwards JSON/object paths; it has no target/layout request.
- the C pure-first path writes a hard-coded x86-64 triple in several emitters;
  its optional TargetMachine path uses default-target/environment selection
  and does not provide the required strict layout receipt.
- `NumericTarget::host()` owns only a host numeric-width projection.
- `TextFormalResidenceFrameHeaderV1` and `target_layout_supported()` validate
  the running Rust ABI; they do not describe the selected ny-llvmc target.

Therefore A/B is the accepted direction, but the source owner and transport
are not implemented and no fast row is authorized yet.

## Ordered taskization

1. **Compile-target capability D0 (current):** name the outer pre-MIR compile
   invocation owner, V1 profile catalog, invocation brand, and the typed route
   by which the capability reaches the selected-normal resolved-session HRTB.
2. **Compile-target capability I0:** implement only the Rust capability,
   private constructor/issuer, explicit profile selection, and caller-zero
   positive/negative transport tests. Do not emit JSON or touch C.
3. **Backend-frame binder I0:** inside
   `PendingFunctionSessionCloseV1::complete_before_restore`, co-seal the same
   invocation capability, physical-signature loan, active plan/census, and
   Residence ABI capability into private owned
   `PinnedTextBackendFrameContractV1` before collector mutation.
4. **Binder transport/validator I0:** project the co-sealed contract into
   versioned typed JSON, parse it in a new C include owner below the 760-line
   split threshold, and validate it against the actual LLVM TargetMachine and
   module data layout. C issues only a private realization receipt.
5. **Direct lowering successor:** only after (1)-(4), decide GEP/load and the
   three `PinnedTextOp` leaves. Lifecycle, route admission, and production
   cutover remain separate rows.

The Pro proposal's single vertical Binder I0 is split at (2)/(3)/(4). These
are distinct owners and test surfaces; combining them would mix target
authority issuance, function co-seal, transport schema, and backend
realization while the current C owners are already at or near the 760/800-line
limits.

## Acceptance

The current D0 is accepted only when all of the following are named without
host/default inference:

```text
one pre-MIR Rust compile invocation owner
one closed V1 target profile catalog
one non-Clone invocation-branded capability
one scoped transport into complete_before_restore
one later ny-llvmc realization-validator direction
zero reverse authority from C/LLVM to MIRBuilder
zero duplicated Residence ABI/layout table
```

Required negative matrix:

```text
missing / late / foreign invocation capability
unknown/free-form target profile
triple / data-layout / endianness drift
address-space pointer width/alignment drift
consumer or Residence ABI revision drift
host/default/environment inference
root count or frame size supplied by target profile
capability loan return/store/escape
collector reached before the four-way co-seal
```

## NoSafeSlice

Keep `NoSafeSlice::PinnedTextCompileTargetCapabilityUnsealed` when the outer
pre-MIR issuer is absent, the only candidate receives JSON after collection,
or the capability requires environment/default/host inference. Keep the
parent `NoSafeSlice::PinnedTextBackendFrameBinderUnsealed` until the target
capability I0 and the four-way function co-seal are both landed.

No implementation may add JSON fields, C parsing, TargetMachine probing,
GEP/load, lifecycle CFG, session adoption, route selection, production
callers, fallback, retry, literal/StringBox origins, TopLevel support, or main
integration during this D0.
