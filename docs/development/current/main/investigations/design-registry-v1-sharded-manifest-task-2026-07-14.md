# Design Registry V1 Sharded Manifest Taskboard

Status: Parked BoxShape series — taskized through clean, cutover, V0 retirement, and lane return

Decision: deterministic sharded manifest V1

Current activation: 0

Current blocker replacement: none — the current active lane remains authoritative until `SELECT0`

## Objective

Split the 7,000-line embedded Design Registry V0 out of `design/INDEX.md` without changing document classification, precedence, sidecars, archive policy, or warning behavior.

The final storage shape is:

```text
design/INDEX.md                    concise authority charter and pointer
design/registry/manifest.toml      schema, shard law, and exact shard list
design/registry/shards/{0..f}.toml deterministic document rows
```

No generated combined registry becomes a second SSOT. Consumers load the manifest and exact shards into one verified virtual registry.

## Evidence at taskization

The pre-D0 inspection recorded:

```text
design/INDEX.md lines:                  7,446
embedded [[documents]] rows:              656
direct design files:                      852
unregistered current/baseline artifacts:   77
```

The only production parser is `tools/docs/repository_artifact_lifecycle_inventory.py`; the archive-policy guard also asserts the V0 marker directly. D0 adds the new design SSOT and its final V0 registry row, so P0 must record the post-D0 baseline rather than copying these numbers blindly.

## Execution DAG

```text
D0
  -> CLEAN0
  -> SELECT0
  -> P0
  -> L0
  -> S0
  -> G0
  -> P1
  -> I0
  -> H0
  -> C0
  -> R0
  -> CLOSE0
  -> RETURN0
```

Each row is one behavior-neutral BoxShape slice. Do not mix registry migration with SSA-RC, classification changes, or archive-policy changes.

## D0 — Taskization and authority card

Deliverables:

- `design-registry-v1-sharded-manifest-ssot.md`
- this taskboard
- navigation/current/workstream pointers
- final V0 registry row for the new design SSOT

Exit:

- V0 remains the only active registry authority
- V1 shards do not exist
- production activation remains 0
- the pre-existing active lane remains the current blocker

## CLEAN0 — Mandatory clean-worktree boundary

This is the first implementation task. No loader, generator, shard, or cutover edit may begin before it is green.

### Preferred path

1. capture `git status -sb` and `git diff --stat`
2. finish the selected active-lane slice recorded by `CURRENT_STATE.toml`
3. run its authorized active gates
4. commit and push that slice
5. verify `git status --porcelain=v1` is empty
6. run `bash tools/checks/current_state_pointer_guard.sh`

### Fallback when the selected active-lane slice cannot be closed

1. record the exact blocker and next action in its active taskboard
2. create a named stash including untracked files:

   ```bash
   git stash push -u -m "wip/<active-lane> before design-registry-v1"
   ```

3. verify `git status --porcelain=v1` is empty
4. run `bash tools/checks/current_state_pointer_guard.sh`
5. record the stash identity in the registry task handoff

Forbidden:

- `git reset --hard`
- destructive checkout of user changes
- deleting untracked WIP
- mixing prior-lane files into a registry commit
- starting SELECT0 while the worktree is dirty

Exit proof:

```text
git status --porcelain=v1: empty
current_state_pointer_guard: GREEN
prior workstream: committed/pushed or named stash recorded
```

## SELECT0 — Explicit lane activation

Only after CLEAN0, change `CURRENT_STATE.toml` and current mirrors so this parked series becomes the selected BoxShape lane.

Requirements:

- docs-only pointer change in its own commit
- previous lane and return target recorded
- V0 remains active authority
- no implementation change in the selection commit

Taskization does not perform SELECT0.

## P0 — Exact V0 authority and consumer inventory

Record the post-D0 baseline:

- exact registered row count
- exact direct design-file count
- all V0 marker readers
- all registry consumers and guard entrypoints
- normalized inventory output and ordering
- current warning-mode behavior

Exit: every production and guard consumer has an owner and retirement row.

## L0 — Typed loader seam with V0 adapter

Introduce one typed in-memory registry representation and loader interface.

Rules:

- V0 adapter remains the only production source
- parsing and validation errors are typed and fail-fast
- no classification or ordering change
- no V1 fallback path

Exit: all existing consumers can be routed through the seam with byte-for-byte or normalized-output parity.

## S0 — Passive V1 manifest and shard loader

Implement the disconnected V1 reader and validator.

It must verify:

- schema version
- exact shard set `0..f`
- shard identity and filename agreement
- `sha256(UTF-8 path)` first-nybble placement
- global path uniqueness
- no missing or extra tracked shard
- full row schema, including empty fields

Production V1 callers remain 0.

## G0 — Deterministic migration generator

Create a generator that reads V0 and writes V1 to a temporary output directory by default.

Rules:

- no tracked write without an explicit command
- deterministic row order and TOML spelling
- rerun produces no diff
- generator does not classify documents or invent fields

## P1 — Full normalized parity proof

Compare V0 and generated V1 after loading both through the typed seam.

Parity includes:

- every row and every field
- precedence and supersession edges
- sidecars
- derived inventory buckets
- warning/error violations and stable order
- registered/unregistered sets

Row-count-only parity is insufficient.

## I0 — Land passive tracked V1 storage

Add:

```text
design/registry/README.md
design/registry/manifest.toml
design/registry/shards/0.toml ... f.toml
```

Constraints:

- 16 deterministic shards
- each shard at most 800 lines at landing
- V0 remains production authority
- no combined generated registry file
- V1 validation and parity gates are green

## H0 — Maintainer helper

Provide one supported helper:

```text
tools/docs/design_registry.py check
tools/docs/design_registry.py locate <path>
tools/docs/design_registry.py add <path> ...
tools/docs/design_registry.py update <path> ...
```

The helper must preserve deterministic placement and schema validation. It must not infer policy fields silently.

## C0 — Atomic production cutover

In one slice:

- make V1 the production authority
- point `design/INDEX.md` to the manifest
- move inventory and archive-policy guards to the V1 loader
- move every known consumer to V1
- assert V0 production caller count is 0
- prohibit V1 failure from retrying V0

Cutover is forbidden unless P1 remains fully green against the tracked V1 files.

## R0 — Retire embedded V0

After C0 is green:

- remove the embedded TOML block
- remove V0 marker parsing
- remove V0-specific guard assertions
- reduce `design/INDEX.md` to at most 200 lines
- retain only authority charter, maintenance entry, and manifest pointer

Do not change registry semantics while deleting V0 storage.

## CLOSE0 — Series close and proof

Required gates:

```bash
python3 tools/docs/design_registry.py check
python3 tools/docs/repository_artifact_lifecycle_inventory.py --check
bash tools/checks/docs_slim_001_archive_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

Required counters:

```text
embedded registry marker count: 0
V0 production loader calls:      0
tracked combined registry files: 0
INDEX.md lines:                  <= 200
missing/extra shards:            0
duplicate paths:                 0
wrong-shard rows:                0
```

## RETURN0 — Restore the prior workstream

Only after the registry series is green, committed, and pushed:

1. restore the previous current pointer in a docs-only commit
2. verify the worktree is clean
3. if CLEAN0 used a stash, inspect it and restore it explicitly
4. run the current pointer guard again
5. resume the recorded prior row

Never pop an unidentified stash, and never restore it on top of dirty registry changes.

## Required fixtures

- missing shard
- extra shard
- duplicate path across shards
- wrong path hash placement
- shard filename/id mismatch
- malformed row field
- unknown schema version
- V0/V1 same count but one field differs
- V0/V1 same rows but warning order differs
- helper add/update preserves deterministic location
- V1 failure does not invoke V0 fallback

## Implementation may claim

After R0/CLOSE0:

```text
Design Registry storage is deterministically sharded.
INDEX.md is a concise authority entry.
All registry consumers use one verified virtual registry.
V0 embedded storage and fallback are retired.
```

## Implementation must not claim

```text
document classification changed
archive policy changed
precedence or supersession semantics changed
sidecar ownership changed
unregistered warning mode became strict
the registry became a generated second SSOT
```

## Stop conditions

Stop the series if any occurs:

1. worktree is dirty at SELECT0
2. generator or helper invents classification policy
3. semantic-topic sharding is introduced
4. a combined generated registry becomes tracked
5. V1 silently falls back to V0
6. parity checks only row counts
7. V0 is removed before all consumers reach V1
8. archive-policy behavior changes in the BoxShape series
9. SSA-RC edits enter a registry commit
10. prior WIP cannot be identified for RETURN0

## Immediate next action

None while another lane is active. When this series is explicitly selected,
begin with `CLEAN0`; do not jump to loader implementation.
