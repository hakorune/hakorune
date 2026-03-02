# lang/src/runtime — Runtime Layer Boundary

Responsibilities:
- `kernel/`: `.hako` runtime kernel logic (default optimization/edit target).
- `host/`: host boundary facade only (`HostFacadeBox.call` entry).
- `collections/`, `numeric/`, `memory/`, `gc/`: low-level runtime helper boxes.

Rules:
- Put new `.hako` kernel behavior under `runtime/kernel/**`.
- Do not add host routing logic under `runtime/kernel/**`.
- Do not add kernel policy logic under `runtime/host/**`.
