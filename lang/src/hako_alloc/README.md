# lang/src/hako_alloc — Hako Alloc Root

Scope
- Top-level physical root for the `hako_alloc` layer.
- First-wave home for policy-plane helpers that used to live under `lang/src/runtime/memory/`.
- Future home for `RawBuf`, `Layout`, `MaybeInit`, and collection/allocator policy layering.

Principles
- Keep this root as the alloc/policy anchor.
- Do not move OS VM, LLVM, or other thin native keep concerns here.
- Treat `runtime/memory/` as historical location only; new work should land under `hako_alloc/`.

Current modules
- `memory.arc_box`
- `memory.refcell_box`
