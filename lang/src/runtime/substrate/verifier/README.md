# lang/src/runtime/substrate/verifier — Minimum Verifier Staging

Responsibilities:
- Current home for the minimum verifier layer under `runtime/substrate/`.
- Current live boxes:
  - `bounds`
  - `initialized-range`
  - `ownership`
- Future home for the remaining smallest fail-fast checks, in this order:
  - `bounds`
  - `initialized-range`
  - `ownership`

Rules:
- One verifier box should answer one question only.
- Do not mix bounds math, initialized-range reasoning, and ownership transfer in one box.
- Keep the remaining verifier boxes docs-first until they are explicitly widened.

Current live subset:
- `bounds` lives at `bounds/README.md`
- `initialized-range` now lives at `initialized_range/README.md`
- `ownership` now lives at `ownership/README.md`

Non-goals:
- No `RawMap` logic here.
- No allocator / TLS / atomic / GC policy here.
- No AST rewrite or normalization logic here.
