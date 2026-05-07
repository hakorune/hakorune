# lang/src/hako_alloc — Hako Alloc Root

Scope
- Top-level physical root for the `hako_alloc` layer.
- First-wave home for policy-plane helpers that used to live under `lang/src/runtime/memory/`.
- Future home for `RawBuf`, `Layout`, `MaybeInit`, and collection/allocator policy layering.
- Current policy/state contract owner is fixed by `hako-alloc-policy-state-contract-ssot.md`.

Principles
- Keep this root as the alloc/policy anchor.
- Do not move OS VM, LLVM, or other thin native keep concerns here.
- Treat `runtime/memory/` as historical location only; new work should land under `hako_alloc/`.
- Current stop-line:
  - current live implementation row is GC trigger threshold policy
  - first landed policy rows are handle reuse policy and GC trigger threshold policy
  - third live allocator row is VM-only page/free-list policy-state prototype
  - live Rust bodies still remain under `src/runtime/**`
  - `RawBuf / MaybeInit` stay reserved-only for now
  - `LayoutBox` is size-class policy only; it is not native layout/ABI ownership

Current modules
- `memory.arc_box`
- `memory.layout_box`
- `memory.page_heap_box`
- `memory.refcell_box`
