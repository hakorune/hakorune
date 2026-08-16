---
Status: design stop; transport I0 landed, direct AOT consumer not opened
Date: 2026-08-16
Work mode: design_stop
Classification: T2 BoxShape candidate; no backend implementation permission
Parent: LOOP-TEXT-SLICE-EXECUTION-D0
---

# LOOP-TEXT-SLICE-DIRECT-AOT-D0

The transport-only `PinnedTextOp` row is landed. This card names the next
authority question before any direct pointer loads or production route.

```text
Decision: choose one sole direct AOT consumer for the three typed PinnedTextOp leaves; the current candidate is `ny-llvmc(boundary pure-first)`, while llvm_py/VM/generic Load/FastMem remain closed and no fallback is allowed.
Source authority + canonical issuer: the function-local stamped PinnedTextAccessPlanTableV1 plus the final pinned-lifetime census provide root/kind/operand facts; a target-bound ny-llvmc capability binder must consume them mechanically and issue no new Text meaning.
Non-authority: MIR JSON, numeric plan ids, raw ptr/len, nearby Compare/Branch shape, FastMem layout receipts, StringSpan/ViewBox, llvm_py, VM, native_ir canaries, benchmarks, environment toggles, fallback, and retry.
Fail-fast boundary: reject missing target capability, foreign plan/root, READ/effect drift, unsupported leaf width/boundary, direct backend admission through transport-only allowlists, or any need to infer residence/UTF-8 safety during lowering.
Smallest next slice: read-only census of the existing ny-llvmc JSON/native-IR entry and a six-line lowering contract plus structural negative matrix; no GEP/load code, lifecycle CFG, session wiring, route admission, or production caller.
Non-claims: no C-speed result, no direct AOT implementation, no SIMD/memcmp/overread, no VM/llvm_py support, no TextEq production route, no fallback/retry, and no main integration.
```

Acceptance requires one named backend owner, one typed leaf-to-lowering
projection, one transport/backend rejection boundary, and assembly-test
requirements that keep loop calls, locks, allocations, handle births,
publication, retain/release, and environment reads at zero. Until then the
current state remains `NoSafeSlice::PrimaryAotDirectConsumerAmbiguous`.
