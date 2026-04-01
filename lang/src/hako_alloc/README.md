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
  - no third live allocator row is open yet
  - live Rust bodies still remain under `src/runtime/**`
  - `RawBuf / Layout / MaybeInit` stay reserved-only for now

Current modules
- `memory.arc_box`
- `memory.refcell_box`
