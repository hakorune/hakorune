---
Status: design stop; transport I0 landed, direct AOT consumer named but not opened
Date: 2026-08-16
Work mode: design_stop
Classification: T2 BoxShape candidate; no backend implementation permission
Parent: LOOP-TEXT-SLICE-EXECUTION-D0
---

# LOOP-TEXT-SLICE-DIRECT-AOT-D0

The transport-only `PinnedTextOp` row is landed. This card names the next
authority question before any direct pointer loads or production route.

```text
Decision: select the current `ny-llvmc` default Boundary `pure-first` C lowerer as the sole transition direct-AOT consumer for the three typed PinnedTextOp leaves; llvm_py/VM/generic Load/FastMem and the future Hako LLVM-text terminal remain closed and no fallback is allowed.
Source authority + canonical issuer: the function-local stamped PinnedTextAccessPlanTableV1 plus the final pinned-lifetime census provide root/kind/operand facts; a target-bound ny-llvmc capability binder must consume them mechanically and issue no new Text meaning.
Non-authority: MIR JSON, numeric plan ids, raw ptr/len, nearby Compare/Branch shape, FastMem layout receipts, StringSpan/ViewBox, llvm_py, VM, native_ir canaries, benchmarks, environment toggles, fallback, and retry.
Fail-fast boundary: reject missing target capability, foreign plan/root, READ/effect drift, unsupported leaf width/boundary, direct backend admission through transport-only allowlists, or any need to infer residence/UTF-8 safety during lowering.
Smallest next slice: read-only census of the default Boundary C entry (`hako_llvmc_compile_json_pure_first` -> `compile_json_compat_pure` -> active body emitter) and a six-line typed lowering contract plus structural negative matrix; no GEP/load code, lifecycle CFG, session wiring, route admission, or production caller.
Non-claims: no C-speed result, no direct AOT implementation, no SIMD/memcmp/overread, no VM/llvm_py support, no TextEq production route, no fallback/retry, and no main integration.
```

Acceptance now names the transition owner. The remaining design acceptance is
one typed leaf-to-lowering projection, one transport/backend rejection
boundary, and assembly-test requirements that keep loop calls, locks,
allocations, handle births, publication, retain/release, and environment reads
at zero. Until that contract is sealed the current state remains
`NoSafeSlice::PinnedTextDirectLoweringContractUnsealed`.

## Owner census (read-only, 2026-08-16)

The selected Boundary path is the only direct-AOT seam: the pure-first export
reaches `compile_json_compat_pure`, which reaches the same-module C body
emitter. `crates/nyash-llvm-compiler/src/native_driver.rs` is only an
i64 const/ret/print canary, `src/llvm_py/**` is a keep lane, and the future
Hako LLVM-text emitter is a later terminal owner. None may receive a duplicate
PinnedTextOp lowering in this row.

The typed binder must reject missing or foreign plan stamps/roots, census,
operand or effect drift, unsupported leaf width/boundary, and any attempt to
infer residence, UTF-8 safety, layout, or Text meaning. MIR JSON remains
transport-only; the next design result is this contract and its structural
negative matrix, not GEP/load implementation.
