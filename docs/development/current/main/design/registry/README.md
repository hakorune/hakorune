# Design Registry V1 — sharded storage

This directory is the **sole authority** for the design registry
(membership, role, precedence, sidecars, retirement of direct files in
`docs/development/current/main/design/`). `../INDEX.md` is a navigation
index only — it does not grant authority.

Status: **production** (V1 cutover landed at `DESIGN-REGISTRY-V1-C0`;
the V0 embedded block was physically removed at R0).

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

Do not edit `manifest.toml` or `shards/*.toml` by hand — use the
maintainer helper below, which rewrites only the canonical shard with
deterministic ordering.

## Maintainer helper

`tools/docs/design_registry.py` is the single helper:

```bash
python3 tools/docs/design_registry.py check               # validate V1
python3 tools/docs/design_registry.py locate <path>       # canonical shard
python3 tools/docs/design_registry.py add --path ... --role ... \
    --owner ... --retire-when ...                          # writes one shard
python3 tools/docs/design_registry.py update <path> --set key=value \
    [--sidecar S ...] [--supersedes S ...]
```

`add` requires all mandatory fields; scalar fields left empty are
explicit `""`, never inferred. `update --set` accepts scalar fields
only; use `--sidecar`/`--supersedes` for the list fields. `update`
cannot change `path` (rename = remove + add).

SSOT: `../design-registry-v1-sharded-manifest-ssot.md`
Taskboard: `../investigations/design-registry-v1-sharded-manifest-task-2026-07-14.md`
