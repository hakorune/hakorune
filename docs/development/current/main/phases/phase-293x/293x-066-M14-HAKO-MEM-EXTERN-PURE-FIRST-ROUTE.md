---
Status: done
Date: 2026-05-09
Scope: M14 hako.mem extern route for pure-first EXE
---

# 293x-066 M14 Hako Mem Extern Pure-First Route

## Decision

`M14 hako.mem extern pure-first route` accepts the first native pointer
runtime route in pure-first EXE:

```text
externcall "hako_mem_alloc"(size)
  -> MIR extern_call_routes
  -> pure-first emits call ptr @hako_mem_alloc(i64)
  -> ptrtoint to the current i64 transport value

externcall "hako_mem_free"(ptr_bits)
  -> MIR extern_call_routes
  -> pure-first emits inttoptr + call void @hako_mem_free(ptr)
  -> publishes an i64 zero sentinel for expression-shaped MIR
```

The route is a transport bridge only. It does not export `nonnull`, `noalias`,
`dereferenceable`, alignment, or ownership attributes.

## Owned

- `hako_mem_alloc` / `hako_mem_free` extern-call route facts in MIR metadata.
- pure-first `.inc` emission for native pointer return and void free call from
  those MIR-owned route facts.
- NyRT `nyash_kernel` exports for `hako_mem_alloc` and `hako_mem_free` so
  pure-first EXE links against the runtime/kernel owner rather than the FFI shim.
- A narrow direct fixture proving alloc/free reach EXE without
  `unsupported_pure_shape`.
- `GenericI64Body` acceptance of the new extern routes as scalar transport
  values so `MemCoreBox.alloc_i64/free_i64` can become direct same-module
  callees.

## Not Owned

- `hako_mem_realloc` pure-first lowering.
- RawBuf/RawArray EXE parity.
- Full `apps/mimalloc-raw-page-proof` EXE.
- Strong LLVM pointer attrs.
- Native layout, `MaybeInit`, TLS, atomic, or allocator ownership verification.
- Backend symbol-name guessing outside route-fact validation.

## Acceptance

```bash
bash tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh
cargo test -q refresh_function_extern_call_routes_records_hako_mem_alloc_route -- --nocapture
cargo test -q generic_i64_body_accepts_hako_mem_alloc_free_extern_routes -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result on 2026-05-09: `k2_wide_hako_mem_extern_pure_first_guard.sh` passes.

## Next Reading

After this row, the raw-page probe is expected to move from the inner
`hako_mem_alloc/free` blocker to the next RawBuf/RawArray wrapper or method-body
route blocker. That follow-up must be its own card and must not widen this row
into broad RawBuf parity.
