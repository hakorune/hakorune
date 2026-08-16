---
Status: design stop; backend-frame co-seal is not yet issuable
Date: 2026-08-16
Work mode: design_stop
Classification: T2 BoxShape decision
Parent: TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-COSEAL-D0

```text
Decision: keep one function-owned compile-time frame contract as a mechanical projection; do not publish JSON or open the backend until its four inputs are co-sealed before collector mutation.
Source authority + canonical issuer: the resolved installed physical-signature loan owns callable lane order, the stamped plan/census owns access coverage, the Residence owner owns frame ABI facts, and the compile-invocation capability owns target/layout expectation; one private bridge at complete_before_restore may co-seal their relation.
Non-authority: logical /N, argument-array length, receiver/name inference, JSON/MIR numeric IDs, ValueId, BindingRef, raw ptr/len, runtime token, generic Load/Store, host/default target probing, and backend-side reissue.
Fail-fast boundary: missing/foreign loan, plan stamp or target capability, receiver/formal/root count-order drift, receiver or ordinary row made a root, ABI/layout mismatch, incomplete/duplicate ExactText root coverage, loan escape, or collector entry before co-seal rejects before effect.
Smallest next slice: document the private input/contract fields, receiver/formal/root formulas, one HRTB lifetime, and positive/negative census; keep JSON/C/GEP/load, lifecycle, session, route, production caller, fallback, and retry closed.
Non-claims: no issued semantic receipt, runtime pointer publication, JSON schema, ny-llvmc validation, direct lowering, C-speed result, or production integration.
```

## Authority spine

The current Rust I0 already supplies the target capability, but the selected
normal close does not yet co-seal it with the other function-local facts. The
design-only bridge must consume all four sibling views in the same
`complete_before_restore` HRTB and return only an owned private contract after
the callback ends:

```text
resolved installed physical-signature loan
  + stamped PinnedTextAccessPlan table / final census
  + Residence-owned frame ABI capability
  + same compile-invocation target capability
  -> private PinnedTextBackendFrameContractV1 (candidate only)
```

The physical-signature loan remains the source of callable lane order and
receiver/formal roles. The Residence owner remains the only owner of actual
backing, pins, and finish. The target capability remains the only target and
layout expectation owner. The bridge compares/co-seals these facts; it does
not issue a second Text, lifetime, ABI, or route meaning.

## Current authority gaps (explicit blocker)

The design names the required views, but the current tree does not yet issue
all of them:

```text
ResolvedCallablePhysicalSignatureLoanV1 currently exposes counts and identity;
  a future scoped projection must lend the complete lane rows (role, ordinal,
  binding, and physical index) from the existing cohort without copying them.

TextFormalResidenceFrameHeaderV1 currently exposes runtime/C rows and private
  constants; a future Residence-owned `ResidenceAbiLayoutV1` view must lend
  revision, header/root-row offsets and sizes, alignment, and limits without
  lending a live pointer, token, or invocation residence.

PinnedTextCompileTargetCapabilityV1 is currently optional at compatibility
  edges; the backend-frame contract must require it and reject absence before
  collector mutation. `Option` may remain only outside the selected binder
  contract, never as a missing-field default.
```

Until these two scoped projections and the mandatory-target boundary are named
and co-sealed, this D0 remains `NoSafeSlice::PinnedTextBackendFrameCoSealUnsealed`.
Counts alone cannot prove root order, and runtime frame constants alone cannot
become a compile-time ABI authority by observation.

## Count and order law

The four axes must remain distinct:

```text
source_logical_arity          = explicit source formal count (/N)
receiver_lane_count           = 1 iff InstanceBoxMethod, otherwise 0
physical_formal_lane_count    = sum of explicit-formal lane widths
physical_callable_lane_count  = receiver_lane_count + physical_formal_lane_count
```

Physical order is `[InstanceReceiver?]` followed by ordinal-ordered explicit
formals. An `InstanceReceiver` is not an ExactText formal and never receives a
Residence root row. Each ExactText occurrence contributes adjacent
`[slot,generation]` lanes and one dense root row in formal occurrence order;
ordinary scalar formals and the receiver contribute no root row. Caller
argument occurrences may alias, but callee parameter lanes are pairwise
distinct within their own ValueId scope; the scopes are never compared.

## Candidate contract shape (not issued)

The design-only contract may contain only non-pointer facts:

```text
function/owner stamp
plan stamp and exact access-plan census
physical callable/formal lane counts and role/order rows
Residence frame revision, header/root-row sizes and offsets
target profile/layout fingerprint, endian, address-space width/alignment
consumer and Residence ABI revisions
derived ExactText root count and checked frame-size bounds
```

It must contain no `ValueId`, `BindingRef`, runtime slot/generation value,
lease token, pointer, byte length, route policy, or JSON-side default. Root
count and total frame size are checked derivations from the occurrence rows and
target limits, never caller-provided authority.

## Acceptance / negative census

This design row is accepted only when the following are written as one
co-seal protocol, before any collector mutation:

```text
static receiver count 0; instance receiver prefix count 1
mixed scalar + ExactText rows preserve formal ordinal and adjacent lanes
ExactText root rows are dense, occurrence-ordered, complete, and unique
caller alias occurrences do not deduplicate root rows
foreign owner/brand, plan stamp, target capability, or Residence revision reject
receiver/formal/root overlap, swap, gap, or count drift reject
frame-size/length/pointer-width/address-space/ABI overflow or mismatch reject
unknown runtime fields, JSON reconstruction, loan return/store/escape reject
collector entry without the four-input co-seal reject
missing target capability at a selected binder entry rejects (no optional default)
count-only signature loans or unissued Residence ABI views reject the design
```

The focused design evidence is a census and type-boundary review only. It does
not authorize code, JSON publication, C parsing, GEP/load, lifecycle CFG,
Canonical session adoption, route admission, production callers, fallback, or
retry. If the same HRTB cannot hold the four inputs until the co-seal, retain
`NoSafeSlice::PinnedTextBackendFrameCoSealUnsealed` and do not invent a sidecar.

## Next row after acceptance

Only after this D0 is accepted may the caller-zero I0 be opened for a private
contract issuer and typed transport projection. The eventual consumer remains
the single `hako_llvmc_compile_json_pure_first -> compile_json_compat_pure`
path, and any new C validator must be a separate sub-760-line owner. No other
backend or production route is implied.
