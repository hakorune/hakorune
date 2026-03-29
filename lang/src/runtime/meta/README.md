# lang/src/runtime/meta — Compiler Semantic Tables

Scope:
- Own compiler-side semantic tables for stage2 cutover.
- Keep runtime kernel behavior in `runtime/kernel/`.
- Keep host transport in `runtime/host/`.

Responsibilities:
- `mir_call` route policy vocabularies.
- `mir_call` prepass need-flag tables.
- `mir_call` constructor/global/string-extern accept surfaces.

Non-goals:
- No kernel behavior.
- No host transport.
- No raw substrate / allocator backend.
- No direct LLVM emission.

Rule:
- This layer owns tables and policy words only.
- Native seams remain responsible for lowering, probing, and final code emission.
