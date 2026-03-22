# lang/src/runtime/substrate — Substrate Capability Staging Root

Responsibilities:
- Physical staging root for future capability substrate modules.
- Home for the `.hako` surface of:
  - `hako.abi`
  - `hako.value_repr`
  - `hako.mem`
  - `hako.buf`
  - `hako.ptr`
  - `hako.atomic`
  - `hako.tls`
  - `hako.gc`
  - `hako.osvm`
- Keep this layer below semantic owner boxes and above native metal keep.

Current phase reading:
- This directory is docs-first and namespace-first.
- It exists to reserve the substrate root and keep future files out of `collections/`.
- Current implementation owner remains:
  - `runtime/collections/` for collection owner boxes
  - native/Rust substrate for metal helpers and low-level host services

Native keep stays outside this directory:
- OS virtual memory
- final allocator calls
- final GC hooks
- final ABI entry stubs
- platform fallback TLS/atomics

Relationship to existing runtime boxes:
- `runtime/collections/` keeps current owner boxes (`ArrayCoreBox`, `MapCoreBox`, `RuntimeDataCoreBox`, `StringCoreBox`).
- `runtime/substrate/` is the future home for capability-facing building blocks those owners may call later.
- Do not move collection owner logic here just because a lower-level helper exists.

Non-goals:
- Do not implement `hako.mem` / `hako.buf` / `hako.ptr` here yet.
- Do not add `RawArray` / `RawMap` here yet.
- Do not move allocator / TLS / atomic / GC policy here yet.
- Do not rewrite native metal helpers here.
- Do not disturb `runtime/collections/` ownership boundaries.
