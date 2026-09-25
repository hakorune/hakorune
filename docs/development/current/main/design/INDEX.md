---
Status: SSOT
Date: 2026-07-11
Decision: accepted
Scope: design root membership, role, precedence, sidecar, and retirement registry
---

# Design Authority Registry

The authority registry lives in `registry/manifest.toml` plus
`registry/shards/{0..f}.toml` (V1 sharded storage; see
`registry/README.md`). This file is the navigation index for direct
files in `docs/development/current/main/design/` — it does not grant
authority. The language charter remains the normative language-law
precedence owner; the registry only classifies design artifacts and
their relationships.

`README.md` is a navigation view. It does not grant authority. During warning
rollout, unregistered files remain in place and the baseline may only decrease.

Consumers and maintainers use `tools/docs/design_registry.py`:

```bash
python3 tools/docs/design_registry.py check          # validate V1 store
python3 tools/docs/design_registry.py locate <path>  # canonical shard
python3 tools/docs/design_registry.py add ...        # add one row
python3 tools/docs/design_registry.py update <path> --set key=value
```

Shard placement is `sha256-utf8-first-nybble-v1`: the first lowercase
hex nybble of SHA-256 over the exact registered path (UTF-8) selects
`shards/<nybble>.toml`. No semantic property chooses a shard.

## Role Vocabulary

```text
authority    normative design owner under its declared precedence parent
navigation   generated or checked human-facing view
supporting   durable explanatory evidence without normative precedence
status-ledger mutable implementation/status evidence
superseded   retained only until reference closure and physical archive
```

Strict mode is allowed only when every direct design file is represented by a
row or owned as a sidecar and the unregistered count is zero.
