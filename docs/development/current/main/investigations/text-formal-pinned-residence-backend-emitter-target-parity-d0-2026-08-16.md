---
Status: active design stop; choose the single realized-target owner before Binder close
Date: 2026-08-16
Work mode: design_stop
Parent: TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BINDER-I0
---

# TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-EMITTER-TARGET-PARITY-D0

```text
Decision: do not close the Binder on the C API preflight alone; choose one
actual pure-first object emitter/target observation owner and prove its parity
with the Rust-owned expected target profile.
Source authority + canonical issuer: Rust
PinnedTextCompileTargetCapabilityV1 remains the sole expected-profile issuer;
the selected object emitter (C API TargetMachine or external opt/llc) may issue
only a private invocation-scoped realization observation for comparison.
Non-authority: C profile constants, LLVMGetDefaultTargetTriple alone, host or
environment defaults, JSON/MIR lengths, llc flags, raw pointers, ValueId,
runtime leases, and a second backend target catalog.
Fail-fast boundary: mixed emitters, missing/foreign invocation, unsupported
triple, realized triple/data-layout drift, or an unproved opt/llc target
selection rejects before IR/object effect; no fallback or retry.
Smallest next slice: read-only owner census plus one parity probe that names
the chosen emitter and validates its actual triple/data-layout against the
Rust descriptor; keep GEP/load, lifecycle, session, route, and callers closed.
Non-claims: no Text execution, residence adoption, C-speed result, production
caller, cross-target support, fallback, retry, or main integration.
```

## Why this is a separate D0

The landed Binder transport performs two useful checks before the generic
lowering walk:

```text
Rust contract projection
  -> strict JSON/profile validation
  -> C API TargetMachine triple/data-layout observation
```

The normal pure-first object path can still finish through the external
`opt`/`llc` helper (`hako_llvmc_mem2reg_canonicalize_and_llc`), while the C API
TargetMachine path is optional (`HAKO_CAPI_TM=1`).  Treating those as one
realization owner would silently create a second target authority.  This D0
must either select the C API path as the sole transition emitter, or define an
explicit external-tool observation/parity contract.  It must not leave the
choice to environment defaults or infer parity from the IR target-triple line.

## Acceptance matrix

```text
positive: selected emitter is explicit; its observed triple/layout equals the
           Rust descriptor; one invocation/profile/ABI revision is retained
negative: C API vs external llc drift, missing tool/query, foreign invocation,
          default/host target, changed -mcpu/-mattr, unknown observation fields,
          mixed emitter completion, fallback/retry request
```

The result is a private realization observation only.  It does not modify the
Rust contract, add JSON meaning, or open `PinnedTextOp` lowering.  The D0 is
accepted only when the object path and the preflight use the same target owner
or when a mechanically checked parity receipt covers both.

## Explicit stops

```text
NoSafeSlice::EmitterTargetOwnerMissing
  if neither C API nor external opt/llc is named as the sole realized-target
  owner.

NoSafeSlice::EmitterTargetParityUnproved
  if the preflight observes one target while the object path may emit with
  another target/CPU/feature set.

NoSafeSlice::EmitterTargetFallbackHidden
  if missing query/tool silently changes emitter or retries through another
  target route.
```
