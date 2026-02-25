# Runtime Rings Architecture (ring0 / ring1 / ring2)

Purpose: clarify responsibilities and naming for core runtime layers, and make provider selection and diagnostics consistent without large code moves.

Overview
- ring0 (Kernel): Core executor/runner/host-bridge. Responsible for fail-fast and process orchestration. No direct business logic of boxes.
- ring1 (Core Providers): Minimal, trusted, always-available providers (static). Example: FileBox Core-RO (open/read/close). Small, reproducible, and safe to enable under fail-fast.
- ring2 (Plugins): Dynamic, featureful providers. Swappable and extensible. Example: Array/Map plugins, full-feature FileBox plugin.

Mapping (current repo)
- ring0: src/ring0/ (facade + guard; re-exports are deferred). Existing core remains under src/*; ring0 is a conceptual anchor.
- ring1: src/providers/ring1/ (facade + guard). Concrete code still lives where it is; ring1 hosts documentation and future home for static providers.
- ring2: plugins/ (dynamic shared libraries, as before).

Selection Policy (Auto mode)
- Global: `HAKO_PROVIDER_POLICY=strict-plugin-first|safe-core-first|static-preferred`
  - strict-plugin-first (default): prefer dynamic/plugin; fallback to ring1 when allowed.
  - safe-core-first/static-preferred: prefer ring1 (static) when available; fallback to plugin.
- Per box (example: FileBox)
  - `NYASH_FILEBOX_MODE=auto|ring1|plugin-only`
  - `NYASH_FILEBOX_ALLOW_FALLBACK=0|1` (narrow dev override)

Diagnostics (stderr, quiet when JSON_ONLY=1)
- Selection: `[provider/select:<Box> ring=<0|1|plugin> src=<static|dynamic>]`
- Fail-Fast block: `[failfast/provider/<box>:<reason>]`

Design Invariants
- ring0 must not depend on ring2. ring1 contains only minimal stable capabilities.
- Fallback is disallowed by default (Fail-Fast). Allow only via per-box override or JSON_ONLY quiet pipe.

Migration Plan (small steps)
1) Add facades and guards (this change).
2) Keep existing code paths; introduce provider policy (done for FileBox).
3) Gradually move minimal static providers under `src/providers/ring1/` (no behavior change).
4) Add canaries to assert selection tags under different policies.

Notes on Naming
- Historical names like "builtin" referred to in-tree providers. To avoid confusion, use ring terms: ring0 (kernel), ring1 (core providers), ring2 (plugins).

