---
Status: SSOT
Decision: accepted-for-tasking — deterministic sharded Design Registry V1
Date: 2026-07-14
Scope: design-root membership registry storage, loading, mutation, migration,
  validation, and V0 retirement boundary
Related:
  - current-docs-archive-policy-ssot.md
  - agent-current-entry-contract-ssot.md
  - ../workstreams/repository-artifact-lifecycle-current.md
  - ../investigations/design-registry-v1-sharded-manifest-task-2026-07-14.md
---

# Design Registry V1 — Sharded Manifest Constitution

## Decision

The typed Design Authority Registry remains one semantic registry, but its
physical storage moves out of the embedded 7,000-line Markdown block in
`design/INDEX.md`.

```text
design/INDEX.md:
  concise human authority charter and machine-manifest pointer

design/registry/manifest.toml:
  schema, warning/strict mode, backlog baseline, shard algorithm, exact shard list

design/registry/shards/{0..f}.toml:
  explicit document rows assigned mechanically from canonical document path

README.md:
  navigation-only view
```

The registry consumer loads the manifest plus every listed shard and publishes
one verified in-memory registry. No concatenated generated registry is kept as
a second SSOT.

This is a behavior-neutral BoxShape migration. It does not classify new design
documents, lower the warning backlog, change precedence, assign sidecars,
archive files, or enable strict mode.

## Current evidence

The pre-D0 inspection of the accepted V0 registry recorded:

```text
INDEX.md lines = 7,446
registered document rows = 656
direct design files = 852
unregistered current/baseline = 77 / 77
empty sidecars arrays = 576
rows with classification_basis = 651
```

Taskization adds this SSOT as one registered direct design file. P0 therefore
records the exact post-D0 baseline before migration instead of treating the
inspection counts above as a permanent manifest invariant.

The current loader extracts one embedded TOML block from Markdown using a
regular expression and parses the entire block at once. That contract works,
but its physical form causes unnecessary context cost, merge contention, and
high-risk manual edits as the registry grows.

For the current 656 paths, SHA-256 first-nybble sharding produces 33–46 rows
per shard. At the current explicit-row format, every shard stays comfortably
below 800 lines.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `INDEX.md` | registry purpose, precedence rules, role vocabulary, manifest pointer | document rows |
| `manifest.toml` | schema version, rollout mode, baseline, shard algorithm and shard membership | document classification rows |
| shard files | explicit document rows | semantic routing policy or implicit defaults |
| V1 loader | parse, aggregate, verify, publish one virtual registry | classification inference |
| mutation helper | deterministic locate/add/update/check mechanics | role/owner/precedence decisions |
| `README.md` | human navigation | membership or precedence authority |

The semantic authority is the verified virtual registry defined by one
manifest and its exact shard set. A shard is only physical storage; it cannot
override another shard or infer missing fields.

## Canonical path and shard law

The registered `path` remains the exact path relative to
`docs/development/current/main/design/` using UTF-8 and `/` separators.

```text
algorithm id:
  sha256-utf8-first-nybble-v1

input:
  exact canonical registered path bytes in UTF-8

output:
  lowercase first hexadecimal nybble: 0..f

shard path:
  registry/shards/<nybble>.toml
```

No semantic property chooses a shard. Role, owner family, subject, phase,
status, or README section therefore cannot cause physical row movement.
Renaming a document changes its canonical path and consequently may change its
shard; the checked mutation helper owns that mechanical move.

## Manifest schema

Conceptual V1 manifest:

```toml
schema_version = 1
mode = "warning"
unregistered_baseline = 77
shard_algorithm = "sha256-utf8-first-nybble-v1"

shards = [
  "shards/0.toml",
  "shards/1.toml",
  # ... exact 0..f set ...
  "shards/f.toml",
]
```

Each shard repeats only its local envelope and explicit rows:

```toml
schema_version = 1
shard_id = "a"

[[documents]]
path = "..."
role = "authority"
owner = "..."
precedence_parent = "..."
classification_basis = "..."
sidecars = []
supersedes = []
superseded_by = ""
retire_when = "..."
```

All existing row fields remain explicit. V1 does not add inheritance,
owner-based defaults, omitted empty-list semantics, or heuristic role
assignment merely to reduce line count.

## Loader verification contract

Before publication, the V1 loader verifies:

```text
manifest schema/mode/baseline are valid
manifest shard list is exact, ordered, duplicate-free 0..f
shard path remains below design/registry/
each shard schema/id matches its manifest position
each document path hashes to the containing shard
rows are ordered by exact path inside each shard
document paths are globally unique
every registered path exists as a direct design file
roles and required fields satisfy the existing V0 contract
sidecars exist, have one owner, and do not also own document rows
precedence graph is cycle-free
README still declares navigation-only status
warning backlog never exceeds its baseline
```

Unknown manifest/shard fields are rejected in V1 unless a later schema
Decision admits them. A missing, extra, unreadable, malformed, misrouted, or
partially written shard fails before registry publication.

## Mutation boundary

After cutover, direct row editing remains possible but the stable entry is a
single helper, conceptually:

```text
tools/docs/design_registry.py check
tools/docs/design_registry.py locate <path>
tools/docs/design_registry.py add --row-file <toml>
tools/docs/design_registry.py update --row-file <toml>
```

The helper:

```text
parses and verifies the full virtual registry first
computes the shard from canonical path
rejects duplicate/missing/foreign paths
writes only the selected shard
keeps rows sorted by exact path
performs atomic temp-file replacement
re-verifies the full virtual registry after mutation
```

The helper never selects role, owner, precedence, sidecars, supersession, or
retirement policy. Those decisions remain explicit inputs reviewed in docs.

## Migration authority

There is never a period with two production membership authorities.

```text
before cutover:
  V0 embedded block = authority
  V1 files = disconnected shadow projection

parity gate:
  normalized V0 registry == normalized V1 virtual registry

atomic cutover:
  manifest pointer + V1 loader become authority together
  V0 loader becomes forbidden for production inventory

after cutover:
  embedded V0 block is removed
  V0 loader reaches exact caller zero and is deleted
```

Parity compares every field and every derived observation, including document
rows, sidecar ownership, unregistered set, warning baseline, roles, precedence
cycles, and violation ordering. A row-count-only comparison is insufficient.

## Clean-worktree prerequisite

This BoxShape series must not start implementation in a dirty tree. The paired
taskboard begins with `DR-V1-CLEAN0`, which resolves the active SSA-RC-L0 work
by green commit/push or a documented named stash, then proves:

```text
git status --porcelain=v1 output lines = 0
current pointer guard = green
active WIP and registry migration commits mixed = 0
```

No reset, checkout, deletion, or overwrite of user work is authorized by the
cleanup task.

## May claim

After this decision only:

```text
Design Registry V1 sharding is accepted and taskized
the selected physical shard function is deterministic and non-semantic
the migration is behavior-neutral and starts only from a clean tree
V1 production callers = 0
current registry authority remains V0 INDEX.md
```

## Must not claim

```text
V1 manifest/shards are implemented or active
INDEX.md is already slim
registry backlog or baseline decreased
strict mode is enabled
design files were classified, archived, or moved
V0 and V1 are both production authorities
semantic topic shards exist
generated combined registry is an authority
repository artifact lifecycle is complete
```

## Stop conditions

Stop the series if a row:

```text
starts before DR-V1-CLEAN0 proves a clean worktree
mixes SSA-RC-L0 source/check changes into the registry series
chooses shards from subject, owner, role, or README section
adds implicit row defaults or heuristic classification
keeps a generated aggregate as a second membership truth
accepts duplicate, missing, extra, or wrong-shard rows
compares only row counts instead of normalized full registry parity
changes mode, baseline, classification, sidecars, precedence, or retirement
falls back to V0 after V1 authority selection
removes V0 before V1 parity and all consumer cutovers are green
lets INDEX.md or README regain document-row authority
creates shard files above 800 lines without a schema-level repartition Decision
```

## Durable final form

```text
INDEX.md:
  small authority charter and manifest pointer

manifest + 16 deterministic shards:
  one typed membership/precedence registry

loader/helper:
  one checked parse/mutation boundary

README.md:
  navigation-only

embedded TOML registry:
  retired
```
