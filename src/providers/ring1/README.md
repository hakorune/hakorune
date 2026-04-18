ring1 — Core Providers (Static/Always-Available)

Status
- SSOT: `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`
- Promotion template: `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
- Current wired domains: `file`, `array`, `map`, `path`, `console`.

Scope
- Minimal, trusted providers with stable behavior (e.g., FileBox Core-RO).
- Prefer static linkage or in-tree implementations.

Guidelines
- Keep capabilities minimal (read-only where possible).
- Emit selection diagnostics via ProviderRegistry (stderr; quiet under JSON_ONLY).
- Do not depend on ring2 (plugins).

Migration Note
- Historical naming like "builtin" may still exist in the codebase. ring1 is the canonical concept; moves will be incremental and guarded.
- Box constructors that are already owned by ring1 should expose a local `new_*_box` seam instead of adding thin wrappers under `src/box_factory/builtin_impls/`.
