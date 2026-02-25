Providers — Facade and Policy

This directory hosts ring1 (core providers) and related documentation.

Goals
- Centralize provider responsibilities and selection policy.
- Keep ring1 minimal and reproducible (static or in-tree providers).
- Registry groups factories by Box type (e.g., "FileBox") to allow future expansion (Array/Map/Console/Path) without changing selection policy.

See also: src/providers/ring1/ and docs/architecture/RINGS.md
