# lang/src/runtime — Runtime Layer Boundary

Responsibilities:
- `kernel/`: `.hako` runtime kernel logic (default optimization/edit target; stage1 can prove slices here without becoming final mainline).
- `host/`: host boundary facade only (`HostFacadeBox.call` entry).
- `collections/`, `numeric/`, `gc/`: low-level runtime helper boxes.
- `meta/`: compiler semantic tables and owner-policy boxes for stage2 cutover.
- alloc/policy-plane helpers belong to the `hako_alloc` layer (top-level root: `lang/src/hako_alloc/**`).
- `substrate/`: future capability substrate staging root (`hako.mem` / `hako.ptr` / etc).

Rules:
- Put new `.hako` kernel behavior under `runtime/kernel/**`.
- Put compiler semantic tables under `runtime/meta/**`.
- Do not add host routing logic under `runtime/kernel/**`.
- Do not add kernel policy logic under `runtime/host/**`.
- Do not add kernel runtime behavior under `runtime/meta/**`.
- Do not move collection owner boxes into `runtime/substrate/**` before the capability modules are explicitly staged.
- `runtime/memory/**` is legacy and not the canonical home for alloc/policy helpers.
- stage1 is bridge/proof for owner slices; stage2+ is the final mainline.
- Phase plan SSOT: `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`.
