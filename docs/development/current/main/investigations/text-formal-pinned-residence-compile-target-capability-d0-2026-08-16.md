---
Status: accepted design; target-capability transport is landed and the
contract-bound emitter I0 is the current bounded slice
Date: 2026-08-16
Work mode: design_stop
Classification: T2 BoxShape decision
Parent: TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BINDER-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-COMPILE-TARGET-CAPABILITY-D0

```text
Decision: A is the canonical contract authority, B is a mandatory realization validator, and C is rejected.
Source authority + canonical issuer: one outer Rust compile invocation issues the explicit target capability before MIRBuilder; ny-llvmc consumes it through the strict versioned projection and the retained contract-bound TargetMachine session validates and realizes the same target.
Non-authority: JSON, MIR, ValueId, NumericTarget::host(), runtime layout checks, environment/default target probing, and C TargetMachine discovery.
Fail-fast boundary: missing/late/foreign capability, invocation/function/target drift, default inference, duplicate ABI tables, or inability to lend the capability at complete_before_restore rejects before collection.
Smallest next slice: implement the named Rust capability and typed transport into the selected-normal resolved-session HRTB; no JSON or C consumer yet.
Non-claims: no GEP/load, lifecycle/session residence route, production caller, fallback/retry, or C-speed claim.
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

## Concrete pre-MIR owner census

The current Rust call chain gives one precise place to close the missing
authority without inventing a downstream reverse edge:

```text
NyashRunner::execute_llvm_mode
  -> LlvmPipelinePlan::current_default
  -> CompileOptionsBox::compile_normal_callable
  -> MirCompilerBox::compile_request
  -> MirCompiler::compile_normal
```

`LlvmPipelinePlan` is already created before the MIR compilation call, so it
is the narrow outer compile-invocation owner candidate. Its current
`LlvmCompileOptions` carries only `FutureRewriteRoute`; it has no target
profile, target triple, data-layout fingerprint, address-space layout, or
Residence ABI revision. `NormalCompileRequestV1` likewise describes source and
imports, not the LLVM target. The candidate chain therefore names the place
where a capability could be issued, but it does not issue one today.

The later `crates/nyash-llvm-compiler` boundary and
`hako_llvmc_compile_json_pure_first` receive collected MIR JSON and cannot
retroactively bless the `complete_before_restore` HRTB. `ny_mir_builder
--target` is a separate wrapper contract, not this LLVM product invocation.
`NumericTarget::host()`, environment flags, JSON fields, and C
`TargetMachine` discovery remain non-authorities.

## Accepted V1 profile catalog

The D0 closes with one explicit catalog row. It is a named profile selection,
not a host probe or a default reconstructed from JSON:

```text
profile id:                nyrt-text-residence-ptr64-as0-v1
target triple:             x86_64-pc-linux-gnu
data layout:               e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128
endianness:                little
address space 0:          pointer width 64, ABI alignment 8
consumer ABI revision:    hako-llvmc-pure-first-v2
LLVM C API ABI revision:  llvm-c-api-18-v1
object CPU/features:      empty / empty (explicit generic profile)
codegen tuple:            opt=3, relocation=default, code model=default
Residence ABI revision:   text-formal-residence-v1
profile limits:            root_count <= 1024, private frame <= 65536 bytes
```

`LlvmPipelinePlan::current_default` selects this sole catalog row for the
current LLVM product invocation. The profile is explicit and versioned even
while the catalog has one row; adding another target requires a new catalog
decision and a new validator matrix. The target profile does not own
function-derived root count or frame size; those remain checked derivations
from the physical-signature occurrence order and plan/census.

The issuer chain is now fixed:

```text
LlvmPipelinePlan / LlvmCompileOptions
  -> PinnedTextCompileTargetCapabilityIssuerV1
  -> MirCompilerBox::compile_request
  -> selected-normal session HRTB
```

The capability is non-Clone, invocation-branded, and borrowed only inside the
function close. It is not serialized or reissued by C. This closes the D0
authority question; it does not open the later binder or backend rows.

## Ordered taskization

1. **Compile-target capability D0 (accepted):** the concrete outer-owner
   candidate, one-row V1 profile catalog, invocation brand, and typed route are
   fixed above. The implementation boundary remains Rust-only.
2. **Compile-target capability I0 (current):** implement only the Rust capability,
   private constructor/issuer, explicit profile selection, and caller-zero
   positive/negative transport tests. Thread the same non-Clone capability into
   the selected-normal close HRTB without storing or returning the borrow. Do
   not emit JSON or touch C.
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

The accepted D0 names all of the following without host/default inference:

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

## D0 closeout / I0 stop

The D0 `NoSafeSlice::PinnedTextCompileTargetCapabilityUnsealed` is closed by
the explicit issuer/profile decision above. During the next I0, use
`NoSafeSlice::PinnedTextCompileTargetCapabilityTransportUnsealed` if the
capability cannot reach the selected-normal close HRTB without cloning,
escaping, or reconstructing it. Keep the parent
`NoSafeSlice::PinnedTextBackendFrameBinderUnsealed` until the target capability
I0 and the four-way function co-seal are both landed.

No implementation may add JSON fields, C parsing, TargetMachine probing,
GEP/load, lifecycle CFG, session adoption, route selection, production
callers, fallback, retry, literal/StringBox origins, TopLevel support, or main
integration during this D0.
