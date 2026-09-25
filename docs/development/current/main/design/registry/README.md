# Design Registry V1 — sharded storage (passive)

This directory is the **V1 physical storage** for the design authority
registry. It is generated deterministically from the embedded V0 block
in `../INDEX.md` by `tools/docs/design_registry.py generate`.

Status: **passive / pre-cutover**. The production authority remains the
V0 embedded block in `../INDEX.md` until row `DESIGN-REGISTRY-V1-C0`.
Do not edit `manifest.toml` or `shards/*.toml` by hand — regenerate
from V0 instead.

## Layout

- `manifest.toml` — schema version, rollout mode, unregistered
  baseline, shard algorithm (`sha256-utf8-first-nybble-v1`), and the
  exact ordered shard set `shards/0.toml` … `shards/f.toml`.
- `shards/<nybble>.toml` — document rows whose registered path's
  SHA-256 (UTF-8) first hex nybble equals `<nybble>`; rows ordered by
  exact path; every row carries the complete field set.

Shards are physical storage only. No semantic property (role, owner,
precedence, sidecars, retirement) chooses a shard, and a shard cannot
infer missing fields.

SSOT: `../design-registry-v1-sharded-manifest-ssot.md`
Taskboard: `../investigations/design-registry-v1-sharded-manifest-task-2026-07-14.md`
