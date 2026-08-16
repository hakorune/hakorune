---
Status: accepted Decision; contract-bound object-emitter I0 landed
Date: 2026-08-16
Work mode: fast
Parent: TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BINDER-I0
---

# TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-EMITTER-TARGET-PARITY-D0

```text
Decision: choose C: every contract-bearing module uses one retained C API
PinnedTextTargetMachineSessionV1 for both realization validation and object
emission; only an uncontracted module may use the external opt/llc legacy path.
Source authority + canonical issuer: Rust PinnedTextCompileTargetCapabilityV1
remains the sole compile-invocation expectation owner. Its target-layout and
object-emitter profile are non-separable sibling projections; JSON projects
them, and the C session only realizes and checks them.
Non-authority: HAKO_CAPI_TM, host/default target, PATH tools, free-form flags,
-mcpu=native, JSON/MIR lengths, /N, module target text, raw pointers, ValueId,
runtime leases, and a separately issued backend target catalog.
Fail-fast boundary: module census, LLVM C API revision/symbols, target/codegen
tuple, triple/layout, and consumer revision seal before IR effect. Missing or
mixed contracts, a second TargetMachine, external fallback, retry, or partial
object publication reject the invocation.
Smallest next slice: TEXT-FORMAL-PINNED-RESIDENCE-CONTRACT-BOUND-OBJECT-EMITTER-I0
adds the module gate, one retained session, same-session emit, success-only
publication, legacy separation, focused evidence, and one reusable guard.
Non-claims: no GEP/load, PinnedTextOp lowering, lifecycle CFG, Canonical
residence adoption, Completion finish, route admission, performance claim,
production caller, literal/StringBox origin, fallback/retry, or main closeout.
```
## Accepted owner graph

```text
PinnedTextCompileTargetCapabilityV1
  +-- TargetLayoutExpectationV1
  `-- PinnedTextObjectEmitterProfileV1
        (private child projection; no independent issuer/catalog)
              |
              v
strict versioned JSON projection
              |
              v
PinnedTextModuleContractCensusV1
  0 rows                  -> LegacyUncontracted
  same-invocation rows >0 -> ContractBoundCapi
              |
              v
one PinnedTextTargetMachineSessionV1
  open -> validate -> retain -> emit once -> close
              |
              v
same-directory temporary object -> success-only rename
```

The first object-emitter profile is closed and explicit: LLVM C API 18,
explicit generic CPU and empty feature set, aggressive LLVM codegen level,
and default relocation/code models. Empty CPU/features are catalog values,
not host inference. Adding these required fields bumps the transport schema
and consumer revision atomically; the contract-bound path does not accept the
older incomplete projection.

The module census is an aggregate, not a target issuer. It runs before
exact-seed, pattern, compat, generic, or external dispatch. One contract row
makes the whole module contract-bound; all contract rows must carry one
compile invocation, target/layout, emitter profile, Residence ABI, and
consumer revision. Every transported `PinnedTextOp` must belong to a
contracted function. A contract-bearing module may use only the generic
pure-first path; every bypass rejects instead of reaching legacy fallback.

The retained session owns one LLVM library handle, one TargetMachine, and one
TargetData. It writes or sets the module triple and data layout from the Rust
projection, never from the current hard-coded IR header. Preflight and emit
must not recreate the TargetMachine. Contracted emission writes a temporary
object in the destination directory and renames only after successful C API
emission; any failure removes the temporary artifact and returns without
trying another emitter.

## Active execution row

`TEXT-FORMAL-PINNED-RESIDENCE-CONTRACT-BOUND-OBJECT-EMITTER-I0`

```text
Change:
  Replace validate-and-discard plus env-selected C API emit plus external
  fallback, for contracted modules only, with one retained TargetMachine
  session. Extract the existing optional emitter from the 788-line generic
  lowerer into responsibility-owned sub-760-line session/publication files.

Contract:
  Contract absence alone selects the unchanged legacy path. Contract presence
  requires a complete same-invocation module census, the explicit LLVM-18
  emitter child profile, one session before IR effect, contract-owned module
  target/layout, same-session emit exactly once, and no fallback or retry.

Done:
  Contracted static/instance/mixed and same-invocation multi-function modules
  emit without consulting HAKO_CAPI_TM or external llc; uncontracted parity is
  retained. Drift, mixed/missing contract, PinnedTextOp without contract,
  missing LLVM-18 API, emit/publication failure, unknown/old transport, and an
  external-tool sentinel all reject with no published object. Extend only
  `tools/checks/pinned_text_backend_frame_transport_smoke.sh` to prove module
  census, one session/TM, success-only rename, and no retry; keep every source
  below 800.

Stop:
  Return to NoSafeSlice if the emitter profile needs an independent catalog,
  a second TargetMachine, host/env/default inference, external optimization
  before its target-neutral boundary is designed, contracted legacy fallback,
  partial publication, GEP/load, lifecycle/session work, or production callers.
```

## I0 closeout evidence (2026-08-16)

The module census, LLVM-18 C API session, and success-only object publication
are landed. Contract-bearing modules retain one `TargetMachine` from
realization validation through emission; they reject compatibility replay,
external opt/llc fallback, and a second target-machine path. Contract-free
modules keep the existing legacy route. The generic lowerer remains 713 lines;
the legacy C API probe is isolated in a responsibility-owned include.

Focused evidence:

```text
bash tools/build_hako_llvmc_ffi.sh                         green
bash tools/checks/pinned_text_backend_frame_transport_smoke.sh green
cargo check -q                                              green (baseline warnings only)
cargo test --lib mir::compiler::pinned_text_backend_frame -- --nocapture green
git diff --check                                            green
bash tools/checks/current_state_pointer_guard.sh             green
```

The smoke covers successful contract-bound emission with a missing external
llc sentinel, emission/publication failure, layout/target/unknown transport
drift, uncontracted `PinnedTextOp`, and mixed invocation rows. This closes only
the object-emitter I0; direct `PinnedTextOp` lowering, lifecycle/session
residence, route admission, performance, and production callers remain closed.
