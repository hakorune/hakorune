---
Status: selected fast slice; Rust compile-target capability transport
Date: 2026-08-16
Work mode: fast
Classification: T2 BoxShape implementation
Parent: TEXT-FORMAL-PINNED-RESIDENCE-COMPILE-TARGET-CAPABILITY-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-COMPILE-TARGET-CAPABILITY-I0

```text
Decision: implement the accepted one-row target profile as one non-Clone Rust capability and carry it from the LLVM compile invocation into the selected-normal close HRTB.
Source authority + canonical issuer: LlvmPipelinePlan/LlvmCompileOptions select the closed profile; PinnedTextCompileTargetCapabilityIssuerV1 issues the invocation-branded capability before MirCompiler collection.
Non-authority: host/default/environment probing, NumericTarget::host(), JSON/MIR length, ValueId, FunctionSignature, C TargetMachine, and any backend reissue.
Fail-fast boundary: missing/foreign profile, invocation drift, unsupported revision/layout, capability clone/escape/return, or a selected-normal close reached without the same capability rejects before collector mutation.
Smallest next slice: Rust-only profile/capability type, request/session/HRTB plumbing, and focused positive/negative tests; no JSON, C, GEP/load, lifecycle CFG, session residence, route, caller, fallback, or retry.
Non-claims: no LLVM realization proof, backend frame contract, direct Text lowering, C-speed claim, production caller, literal/StringBox origin, or main integration.
```

## Implementation boundary

The only accepted profile is
`nyrt-text-residence-ptr64-as0-v1`. The outer Rust chain is:

```text
NyashRunner::execute_llvm_mode
  -> LlvmPipelinePlan::current_default
  -> LlvmCompileOptions
  -> PinnedTextCompileTargetCapabilityIssuerV1
  -> NormalCompileRequestV1
  -> MirCompilerBox::compile_request
  -> selected-normal close HRTB
```

The capability owns target identity/layout expectation and the invocation
brand only. Residence ABI facts remain owned by the runtime Residence row;
function root count and frame size remain later checked derivations. The I0
must not make the capability `Clone`, place it in JSON, or let a backend create
a replacement from a triple or host probe.

## Required code seams

1. Add the closed profile and non-Clone capability in a compiler-owned module.
   The profile exposes fixed accessors for triple, layout fingerprint,
   endianness, address-space-0 pointer width/alignment, consumer revision,
   Residence ABI revision, and the two profile limits.
2. Make `LlvmCompileOptions` select that profile explicitly and issue one
   capability per compile invocation. Keep `FutureRewriteRoute` orthogonal.
3. Carry the capability through the LLVM `NormalCompileRequestV1` path and
   into the isolated Builder invocation used by the selected-normal lifecycle.
   Other normal/VM/MIR callers stay capability-free and do not infer a target.
4. At the resolved child close, lend the same capability by reference together
   with the existing physical-signature loan. The reference is scoped to the
   `complete_before_restore` HRTB and is not stored in the collector or module.

## Acceptance

```text
one catalog row, one issuer, one capability per LLVM compile invocation
profile accessors are fixed and testable without host probes
capability is non-Clone and private-constructor
LLVM request/session transport preserves the same invocation brand
selected-normal close sees the borrowed capability before collection
ordinary non-LLVM requests remain unchanged
missing/foreign/profile-drift/escape tests are present
source files remain below the 760-line split threshold
```

## Negative matrix

```text
unknown/free-form profile
host/default/environment target inference
foreign invocation brand or profile revision
capability clone, return, store, or collector retention
JSON/MIR/ValueId-derived replacement
selected close without capability
backend/C/TargetMachine reissuance
GEP/load, lifecycle CFG, route selection, fallback, and retry
```

## Non-claims

This I0 does not publish a JSON descriptor, validate an actual LLVM
TargetMachine, materialize pointers, add `PinnedTextOp` lowering, adopt a
Canonical session residence, place Completion finishes, select a TextEq route,
switch a production caller, admit literals/StringBox origins, or claim C-fast
performance. Those remain later bounded rows.
