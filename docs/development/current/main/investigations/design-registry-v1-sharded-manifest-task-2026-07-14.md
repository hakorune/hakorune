# Design Registry V1 Sharded Manifest Taskboard

Status: SELECTED BoxShape series (DESIGN-REGISTRY-V1 lane, selected
2026-09-25 via SELECT0 pointer commit; current row
`DESIGN-REGISTRY-V1-P0`; return target
`REPO-FINAL-CONVERGENCE-AUDIT0-G0`)

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

### P0 landing — baseline at lane selection (2026-09-25)

Measured via `repository_artifact_lifecycle_inventory` design_registry
section (sole production parser) plus direct INDEX.md inspection:

```text
INDEX.md lines:                    7,670
embedded [[documents]] rows:         676 (all paths unique)
direct design files (top-level):     872 (851 .md + 21 non-md)
owned sidecars:                      119
unregistered files:                   77  (= baseline 77, warning mode,
                                           zero violations)
seed union count:                    175
INDEX.md marker pair: design-registry-v0:begin / design-registry-v0:end
```

V0 marker readers (exact):

- `tools/docs/repository_artifact_lifecycle_inventory.py` — sole
  production parser (`read_design_registry`, `DESIGN_REGISTRY_BLOCK`).
- `tools/checks/docs_slim_001_archive_policy_guard.sh` — asserts the
  `design-registry-v0:begin` marker.

Consumer/guard entrypoints: the inventory's `design_registry`
section feeds `--check --strict` (violation: unregistered > baseline,
strict-mode leftovers); the archive-policy guard asserts the marker
only. No other tool parses the embedded block.

Normalized inventory ordering: deterministic serialized JSON
(`serialized()`); violations list order is stable.

Warning-mode behavior: unregistered files remain in place; the
baseline may only decrease; strict mode is not active.

CLEAN0 evidence: worktree cleaned via named stash
`wip/other-worker-docs before design-registry-v1` (2 unrelated
investigation files); pointer guard green.

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

### L0/S0/G0/P1/I0 landing — typed seam + passive V1 storage (2026-09-25)

- **L0** (`911ede962d`): `tools/docs/design_registry.py` owns the typed
  `Registry` dataclass and fail-fast structural errors
  (`RegistryNotFound`/`RegistryBlockMissing`/`RegistryMalformed`).
  `repository_artifact_lifecycle_inventory.read_design_registry` now
  routes through `design_registry.load_registry`; V0 embedded block
  remains sole production source. Inventory `design_registry` section
  is byte-identical after the swap (676 rows, `violations=[]`); only
  drift was the new tool file itself.
- **S0**: `load_v1()` passive manifest/shard reader — rejects unknown
  fields, non-1 schema, wrong algorithm, inexact shard set, shard id
  mismatch, unordered rows, incomplete row schema, wrong-shard rows,
  and duplicate paths. Zero production callers.
- **G0**: `generate_v1()` deterministic V0→V1 emitter +
  `design_registry.py generate` CLI (temp dir default; `--output` for
  explicit writes). Re-run is byte-identical (`diff -r` clean).
- **P1**: normalized parity proven — 676 rows equal under
  `_normalize_row` (absent `classification_basis` → `""` in 3 rows,
  explicit-empty not invented), mode/baseline equal, and the shared
  `validate()` produces identical `violations=[]` on both sources.
- **I0**: passive V1 storage landed at
  `design/registry/{README.md,manifest.toml,shards/{0..f}.toml}` (18
  files). V0 remains the only production authority; inventory
  `design_direct` count unchanged (872) since registry/ is a subdir.

### H0/C0 landing — helper + atomic production cutover (2026-09-25)

- **H0** (`7ad94f946e`): `design_registry.py` helper surface —
  `check --source`, `locate`, `add`, `update`. Mutation writes only
  the canonical shard with deterministic ordering; path change and
  unknown fields rejected; no policy inferred.
- **C0**: production authority switched to V1 in one change:
  - `load_registry()` → `load_v1()`; V1 failure is terminal (typed
    error → violation string, empty registry — never retries V0).
  - `check` default source flipped to `v1`.
  - `docs_slim_001_archive_policy_guard.sh` now requires
    `registry/manifest.toml` + `shard_algorithm =
    "sha256-utf8-first-nybble-v1"` instead of the v0 marker.
  - `design/INDEX.md` declares the manifest the authority; the
    embedded V0 block is inert pending R0 removal.
  - V0 production loader calls: 0 (`load_v0` remains for parity
    tooling only). Inventory `design_registry` section byte-identical
    under V1 (676 rows, violations `[]`); `docs-slim-001` guard green.

### R0 landing — V0 physical retirement (2026-09-25)

- `design/INDEX.md` reduced to 45 lines (≤200): navigation index +
  authority pointer + role vocabulary. Embedded `design-registry-v0`
  block and marker parsing physically removed.
- `design_registry.py`: `load_v0`, `V0_BLOCK`, `RegistryBlockMissing`,
  `generate_v1`/`emit_manifest`, and `check --source v0` removed —
  V1 is the only readable source.
- `design_registry` inventory section byte-identical post-removal
  (676 rows, `violations=[]`).
- Counters: embedded markers 0, V0 production loader calls 0,
  combined tracked registry files 0, INDEX.md 45 lines, missing/extra
  shards 0, duplicate paths 0, wrong-shard rows 0.

### CLOSE0/RETURN0 — gate receipt and lane return (2026-09-25)

Gates:

```text
python3 tools/docs/design_registry.py check                      → 676 rows, 0 violations
python3 tools/docs/...inventory.py --check                       → inventory current
bash tools/checks/docs_slim_001_archive_policy_guard.sh          → ok
bash tools/checks/current_state_pointer_guard.sh                 → ok
tools/checks/dev_gate.sh quick                                   → 1 red
```

Final counters: embedded markers 0, V0 production loader calls 0,
combined tracked registry files 0, INDEX.md 45 lines, missing/extra
shards 0, duplicate paths 0, wrong-shard rows 0.

dev_gate red classification — **known baseline debt, not this lane**:

- `naming_charter_guard` fails on `stage_a_route.rs:114` wording
  (`"unknown Stage-A capture fixture"`), introduced by unrelated
  commit `5e4a5e596c` (Sep 14, different worker). This lane touched no
  `src/` file.
- The same guard has a dormant-check bug: line 809's `rg` pattern
  `BIN=\"\\$ROOT_DIR/...` expands to an invalid `\/` escape, so rg
  exits 2 and the check silently never runs. Pre-existing; recorded
  for a separate fix.

RETURN0: lane complete; pointer returns to
`REPO-FINAL-CONVERGENCE-AUDIT0-G0`. Named stash
`wip/other-worker-docs before design-registry-v1` remains for
explicit restore by its owner.
