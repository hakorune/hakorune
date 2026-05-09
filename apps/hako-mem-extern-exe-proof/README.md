# hako-mem-extern-exe-proof

Purpose: M14 pure-first EXE proof for direct `hako_mem_alloc` /
`hako_mem_free` extern routes.

## Accepted Shape

- Source uses direct `externcall "hako_mem_alloc"(size)` and
  `externcall "hako_mem_free"(ptr_bits)`.
- MIR owns the route through `extern_call_routes`.
- pure-first emits native pointer calls from those route facts:
  - `call ptr @hako_mem_alloc(i64)` followed by `ptrtoint`.
  - `inttoptr` followed by `call void @hako_mem_free(ptr)`.
- The returned pointer is transported as the current i64 value representation.

## Non-Goals

- No `hako_mem_realloc` lowering.
- No RawBuf/RawArray EXE parity.
- No strong LLVM pointer attrs.
- No ownership, nonnull, dereferenceable, or alignment proof.
- No allocator policy or native layout ownership.
