---
Status: design stop; direct-AOT contract boundary accepted, implementation not opened
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
transport-only; the binder requires an opaque backend-private residence-frame
capability keyed by the same stamp/root order. Issuing that capability,
adopting it into a session, and finishing it are the separate Residence
successor; this card may name the input and reject its absence, but may not
invent or publish a second residence authority.

## Typed leaf contract (design-only)

    ByteLen(root) -> i64
    Utf8WidthAt(root, byte_offset:i64) -> i64 in 1..=4
    Utf8ScalarSliceEqWholeText(lhs_root, offset:i64, width:i64, rhs_root) -> i1

The binder consumes the stamped plan row, the final lifetime census, and the
opaque residence-frame capability. It emits only the three typed leaf results;
it never reconstructs a plan, reads raw JSON IDs as authority, or recaptures a
generation. Reject before IR effect on foreign/missing frame or stamp, root
row/count drift, incomplete or duplicate coverage, non-i64 offset/width,
negative/overflowing range, invalid UTF-8 boundary, whole-text RHS mismatch,
unsupported width/alignment, or non-READ effect. This is a contract matrix,
not GEP/load permission.

## D0 closeout: accepted authority boundary

The worker audit closes the design boundary but keeps the row in
`design_stop`. The existing pure-first C body emitter is the sole consumer;
the retained `PinnedTextTargetMachineSessionV1` owns object realization only.
The binder is a mechanical projection and does not become a second residence,
Text, UTF-8, or route authority.

```text
Decision:
  Consume one co-sealed plan/census/frame view and project exactly the three
  typed PinnedTextOp leaves in the existing pure-first C emitter.
Source authority + canonical issuer:
  PinnedTextAccessPlanTableV1 owns stamped leaf/kind/operand facts; the final
  lifetime census owns dominance/exit coverage; Residence owns immutable roots
  and finish; the backend-frame contract links those facts without reissuing
  them.
Non-authority:
  MIR/JSON numeric IDs, JSON lengths, raw ptr/len, ValueId shape, nearby
  Compare/Branch, generic Load/FastMem, StringSpan/ViewBox, target session,
  environment, benchmark, VM/llvm_py/native canaries, and fallback.
Fail-fast boundary:
  Missing/foreign plan stamp, frame or root row; root/count/census drift;
  non-READ or non-i64 operands; invalid offset/width/boundary; escaped or
  stored frame pointers; use after finish; unsupported leaf or route.
Smallest next slice:
  A caller-zero typed binder I0 only after the opaque residence-frame
  capability is available in the same function-owned plan. It may lower the
  three leaves and prove zero loop calls/locks/allocations/publication; it may
  not issue or adopt residence state.
Non-claims:
  No lifecycle CFG, session adoption, Completion placement, TextEq route,
  production caller, literal/StringBox origin, SIMD/memcmp, VM parity,
  performance keeper, fallback/retry, or main integration.
```

## Successor taskization (still design-gated)

The next implementation card must not be opened merely because the object
emitter is green. First make the existing Residence-owned capability lendable
to the same function plan; otherwise retain
`NoSafeSlice::PinnedTextDirectLoweringContractUnsealed`.

1. **Frame bridge:** expose one scoped, opaque frame view keyed by the existing
   plan stamp and occurrence-ordered root rows. It contains no public pointer,
   length, runtime token, or JSON-owned meaning. The Residence issuer remains
   the only owner of backing and finish.
2. **Typed binder:** add one pure-first C projection seam consuming the frame
   view and plan/census. Validate all rows before emitting any LLVM effect; no
   JSON reconstruction, generation recapture, or generic Text value.
3. **Three leaves only:** lower `ByteLen`, `Utf8WidthAt`, and
   `Utf8ScalarSliceEqWholeText` to the existing LLVM-text emitter. Keep
   integer arithmetic, branches, PHI, and placement in their existing owners.
4. **Structural gate:** generated IR/assembly must show zero loop-internal
   runtime calls, registry locks, allocations, handle/Box births, publication,
   retain/release, environment reads, and no post-finish root use. Contract
   failure publishes no object and never retries through legacy.

The I0 acceptance matrix is: mixed three-leaf positives with receiver prefix
and repeated caller aliases; missing/foreign frame or plan; duplicate,
omitted, or reordered roots; stamp/census/effect/type drift; invalid
offset/width/boundary; lifetime dominance/escape/finish failure; and an
external-tool sentinel proving the contracted path stays on the retained
TargetMachine. This matrix is evidence for the binder only, not a TextEq
production or C-speed claim.

## Next design seal: scoped backend-frame borrow

The phrase “opaque residence-frame capability” above is now narrowed to one
compile-time, non-pointer projection named
`PinnedTextBackendFrameBorrowV1`. It is not a runtime residence, does not lend
`ptr`, `len`, a lease token, or a generation value, and does not create a
second Residence/lifetime issuer. The existing Rust co-sealed
`PinnedTextBackendFrameContractV1` remains the only issuer; the pure-first C
lowerer borrows this projection for one lowering invocation and consumes the
retained `PinnedTextTargetMachineSessionV1` only as its realization context.

```text
Decision:
  Define one scoped PinnedTextBackendFrameBorrowV1 projection from the
  existing function-owned backend-frame contract; keep GEP/load and runtime
  residence lifecycle closed.
Source authority + canonical issuer:
  Rust issue_pinned_text_backend_frame_contract_v1 co-seals the plan/census,
  ResidenceAbiLayoutV1, physical lane order, and compile-target capability;
  the C binder only borrows/validates that projection.
Non-authority:
  TextFormalCallResidenceV1 runtime roots/tokens, raw pointers, JSON numeric
  IDs, C-created frame meaning, MIR shape, or a second lifetime issuer.
Fail-fast boundary:
  Missing/foreign invocation or plan stamp, root/count/size/revision drift,
  absent contract-bound session, non-READ/unsupported leaf, pointer/token
  escape, lifecycle mutation, call, allocation, lock, fallback, or retry.
Smallest next slice:
  Design the scoped borrow and typed C handoff only; after acceptance, I0 may
  validate the zero-effect handoff and exact three-leaf census without
  changing TextFormalCallResidenceV1.
Non-claims:
  No GEP/load, UTF-8 execution, lifecycle CFG, session adoption, route,
  production caller, performance claim, or object-emitter redesign.
```

The borrow must be scoped to the existing lowering callback and must not be
stored in JSON, `MirFunction` runtime state, a module port, or a backend-global
table. Its fields are limited to invocation/owner stamp, plan stamp,
occurrence/root count, Residence ABI revision and derived frame size, and the
target profile/session identity. A positive handoff proves only that the
three typed leaves can see the same co-sealed contract; it does not prove that
the runtime frame has been entered or that a pointer is live. If a consumer
needs a live frame, lifecycle entry/finish becomes a separate Residence D0 and
this row remains `NoSafeSlice` rather than smuggling that state through the
borrow.
