# lang/src/runtime/substrate/verifier — Minimum Verifier Staging

Responsibilities:
- Docs-first reservation for the minimum verifier layer under `runtime/substrate/`.
- Future home for the smallest fail-fast checks, in this order:
  - `bounds`
  - `initialized-range`
  - `ownership`

Rules:
- One verifier box should answer one question only.
- Do not mix bounds math, initialized-range reasoning, and ownership transfer in one box.
- Keep this directory docs-first for the current phase.

Non-goals:
- No `.hako` verifier implementation yet.
- No `RawArray` / `RawMap` logic here.
- No allocator / TLS / atomic / GC policy here.
- No AST rewrite or normalization logic here.
