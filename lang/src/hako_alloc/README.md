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
- New state boxes in this root should use Unified Members stored declarations
  (`field: Type`) instead of legacy `init { ... }` slot lists.
- Do not treat this root as the owner for unrestricted raw memory, raw pointer,
  native layout, OS VM, or platform atomics/TLS.
- Current stop-line:
  - current live implementation row is GC trigger threshold policy
  - first landed policy rows are handle reuse policy and GC trigger threshold policy
  - third live allocator row is the page/free-list policy-state prototype
    (the original row name says VM-only; current EXE parity rides typed-object
    and pure-first routes, not native allocator fast-path ownership)
  - live Rust bodies still remain under `src/runtime/**`
  - `RawBuf` policy/state and `MaybeInit` stay reserved-only for now
  - `LayoutBox` is size-class policy only; it is not native layout/ABI ownership
  - `HakoAllocRemoteFreePolicy` owns the `.hako` remote-free retry policy only;
    pointer atomics remain substrate route facts.
  - `HakoAllocPageSourcePolicy` owns the `.hako` page-source policy seam only;
    OSVM reserve/commit/decommit metal remains substrate/native keep.
  - `SizeClassBox` owns mimalloc-shaped pure size-class policy. `LayoutBox`
    remains the small/medium compatibility facade until the page heap migrates.
  - `HakoAllocPageModel` owns page-local `free` / `local_free` / `used` /
    `capacity` / `reserved` invariants. Page queues, OSVM sourcing, TLS,
    atomics, and remote-free integration stay in later rows.
  - `HakoAllocPageQueue` owns page ordering/direct-page cache state. It chooses
    pages by observing `freeCount()` and must not pop allocation blocks.

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
- `RawBuf` policy/state, `MaybeInit`, native `Layout`, `repr`-like layout,
  `sizeof`, `alignof`, `no_alloc`, `no_safepoint`, TLS, atomics, and OS VM rows
  stay reserved until their docs/gates are named.
- The narrow `RawBufCoreBox` allocation facade lives under
  `lang/src/runtime/substrate/raw_buf/`; it is not this layer's allocator
  policy/state owner.

Production allocator port entry
- M45 fixes the production allocator port boundary:
  - `hako_alloc` owns allocator policy/control/facade names.
  - `runtime/substrate` owns raw capability facades.
  - native metal keep owns final libc/syscall/platform bodies.
- First implementation order:
  1. production facade boundary
  2. local page policy proof
  3. remote-free policy proof
  4. OSVM page-source proof
  5. stress production-facade parity
- Do not add allocator replacement hooks, pointer fetch_add, native pointer
  attrs, or app-specific `.inc` matchers as part of the entry plan.

Allocator replacement hook boundary
- M52 fixes the hook boundary before implementation:
  - `hako_alloc` owns policy/control shape; it does not install the process hook.
  - MIR/manifest HookPlan facts must become the backend-readable truth before
    any backend/runtime hook is activated.
  - `.inc` must not infer hook ownership from app, facade, or policy names.
  - hook environment toggles stay inactive until a named future row documents
    them with defaults and removal/rollback conditions.

Current modules
- `memory.arc_box`
- `memory.allocator_facade_box`
- `memory.layout_box`
- `memory.page_box`
- `memory.page_heap_box`
- `memory.page_queue_box`
- `memory.page_source_policy_box`
- `memory.remote_free_policy_box`
- `memory.refcell_box`
- `memory.size_class_box`
