---
Status: parked task map
Decision: documentation-only; no source spelling or implementation is selected
Date: 2026-08-18
Scope: future user-authored C-speed scalar/text kernels
Related:
  - docs/development/current/main/design/contract-region-v0-ssot.md
  - docs/reference/language/low-level-capabilities.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
---

# Scalar Kernel Region D0 task map

## Decision

Keep the `.hako` surface small for now. A future low-level scalar/text kernel
must be a profile-owned producer inside `ContractRegionV0`, not a universal
`fastpath` dialect and not a raw-pointer escape hatch. The automatic S6C
corridor remains the first implementation and the only active lane. This file
records a design task, not a parser or MIR change.

The useful distinction is:

```text
automatic corridor  -> compiler proves a source shape and emits one verified plan
future kernel region -> user writes the same bounded leaf vocabulary explicitly
                         and the compiler verifies/emits that same plan
```

The two forms must converge on one semantic plan issuer, one common dispatcher,
and one backend consumer. A hand-written kernel may not become a second
authority or a second production route.

## Six-line brief

```text
Decision: park a future ScalarKernelRegion profile under ContractRegionV0; do not add syntax now.
Source authority + canonical issuer: .hako semantic types plus the profile-owned verifier/plan issuer; the common dispatcher is the sole physical consumer.
Non-authority: raw ptr/len, MIR adjacency, helper names, StringBox::equals, nyash.string.eq_hh, benchmark wins, and backend-specific assembly spelling.
Fail-fast boundary: region inputs, effect/escape/no-safepoint obligations, leaf vocabulary, bounds/UTF-8 proofs, and target capability must close before the first effect.
Smallest next slice: after S6C production cutover, run a demand census and publish a ScalarKernelRegion D0 contract with one non-pointer leaf example.
Non-claims: no parser keyword, no user-visible `fastpath` spelling, no raw pointer type, no VM/AOT parity, no production switch, and no C-speed result.
```

## Why this is not “just fastpath”

`fastpath` is a route preference and diagnostic concept. It cannot own the
meaning of a Text operation, the lifetime of a residence, or a backend ABI.
The future feature is therefore a small *profile* with explicit obligations:

```text
ContractRegionV0 envelope
  └─ ScalarKernelRegion profile
       ├─ typed inputs/outputs
       ├─ allowed portable leaves
       ├─ effect and escape obligations
       ├─ verifier-issued corridor plan
       └─ common physical consumer
```

It may later be surfaced by syntax sugar, but syntax is not the feature’s
authority. If a spelling cannot lower to the same verified plan as the
automatic corridor, it is rejected or remains an external FFI contract.

## Candidate source shape (illustrative only)

This is intentionally not accepted grammar:

```hako
// future idea, not parseable today
scalar_kernel search_scalar(text: Text, needle: Text) -> i64 {
    // byte_len, utf8_width_at, scalar_slice_eq, loop, if, return
}
```

The first profile must expose a small portable leaf vocabulary, not machine
instructions:

```text
ByteLen
Utf8WidthAt
Utf8ScalarSliceEqWholeText
bounded integer add/compare
ordinary loop/if/return
```

No raw pointer arithmetic, arbitrary loads, hidden allocation, callback,
provider dispatch, or user-selected runtime handle may enter this vocabulary.

## Ordered tasks (all parked)

| order | task | exit condition |
| ---: | --- | --- |
| 0 | Demand census | At least two real source shapes require explicit kernels; otherwise keep the idea parked. Measure exact/meso/whole fronts and name the owner/state transition first. |
| 1 | `SCALAR-KERNEL-REGION-D0` | Define profile header, typed input/output, obligation states, allowed leaves, rejection vocabulary, and relation to `ContractRegionV0`; no parser change. |
| 2 | `SCALAR-KERNEL-SOURCE-CONTRACT-D0` | Choose one bounded source block form and prove type/effect/no-escape/no-safepoint rules. Raw pointers and backend names remain absent. |
| 3 | `SCALAR-KERNEL-PLAN-ISSUER-I0` | Emit the same verified corridor plan used by an automatic producer; source Facts/Recipe remain the only meaning authority and the issuer is sole. |
| 4 | `SCALAR-KERNEL-DIAGNOSTIC-I0` | Explain why automatic fusion was rejected and provide a stable “write a kernel region” diagnostic without silently changing route selection. |
| 5 | `SCALAR-KERNEL-COMMON-CONSUMER-I0` | Reuse the common dispatcher and one backend consumer; no kernel-specific MIR dialect or helper-name branch. |
| 6 | `SCALAR-KERNEL-PARITY-R0` | VM/reference/AOT behavior, Unicode/alias/lifecycle negatives, structural hot-loop zero gate, then exact/meso/whole C comparison. |
| 7 | `SCALAR-KERNEL-PRODUCTION-D0` | Select one named production edge before effects, observe zero fallback/retry, and retire any superseded route only with caller-zero evidence. |

## Acceptance gates

Positive evidence must cover ASCII and 2/3/4-byte UTF-8 scalars, match/miss
positions, multi-scalar needles, same-root aliases, empty input, and every
normal exit. Negative evidence must cover foreign/stale inputs, invalid
boundaries, width outside 1..4, escape/store/foreign-call, allocation,
callback, raw-pointer use, unsupported effect, and missing exit cleanup.

The structural gate is mandatory before benchmark claims. The hot loop must
have zero host-table lock, allocation, deallocation, callback, trait dispatch,
extern/indirect call, generation validation, lease operation, residence
enter/finish, and pointer escape. Benchmark wins cannot waive a failed
structural or semantic gate.

## Explicit non-claims

This task map does not authorize `scalar_kernel`, `fastpath`, `unsafe`,
`ptr<len>`, a new MIR dialect, a new frame/lifetime owner, `Arc<str>`
migration, `nyash.string.eq_hh` retirement, fallback/retry, or production
activation. Those require a later accepted Decision and a separate bounded
slice.
