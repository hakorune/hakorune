# lang/src/hako_alloc — Hako Alloc Root

Scope
- Top-level physical root for the `hako_alloc` layer.
- First-wave home for policy-plane helpers that used to live under `lang/src/runtime/memory/`.
- Future home for `RawBuf`, `Layout`, `MaybeInit`, and collection/allocator policy layering.
- Current policy/state contract owner is fixed by `hako-alloc-policy-state-contract-ssot.md`.
- Substrate capability order is fixed by `substrate-capability-ladder-ssot.md`.

Principles
- Keep this root as the alloc/policy anchor.
- Do not move OS VM, LLVM, or other thin native keep concerns here.
- Treat `runtime/memory/` as historical location only; new work should land under `hako_alloc/`.
- Do not treat this root as the owner for unrestricted raw memory, raw pointer,
  native layout, OS VM, or platform atomics/TLS.
- Current stop-line:
  - current live implementation row is GC trigger threshold policy
  - first landed policy rows are handle reuse policy and GC trigger threshold policy
  - third live allocator row is VM-only page/free-list policy-state prototype
  - live Rust bodies still remain under `src/runtime/**`
  - `RawBuf / MaybeInit` stay reserved-only for now
  - `LayoutBox` is size-class policy only; it is not native layout/ABI ownership

Design owners
- Policy/state stop-line:
  `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`
- Capability ladder:
  `docs/development/current/main/design/substrate-capability-ladder-ssot.md`
- Minimal memory/pointer substrate:
  `docs/development/current/main/design/minimal-capability-modules-ssot.md`
- Minimum verifier:
  `docs/development/current/main/design/minimum-verifier-ssot.md`

Allocator fast-path rule
- `mimalloc-lite` and allocator policy models can live here as policy/state rows.
- mimalloc-grade native fast paths require the substrate ladder first.
- `RawBuf`, `MaybeInit`, native `Layout`, `repr`-like layout, `sizeof`,
  `alignof`, `no_alloc`, `no_safepoint`, TLS, atomics, and OS VM rows stay
  reserved until their docs/gates are named.

Current modules
- `memory.arc_box`
- `memory.layout_box`
- `memory.page_heap_box`
- `memory.refcell_box`
