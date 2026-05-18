---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-ROW-CADENCE-001 mimalloc / hako_alloc row validation cadence.
Related:
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
  - docs/tools/check-scripts-index.md
  - tools/checks/run_proof_app.sh
  - tools/checks/run_row_guard.sh
  - tools/checks/lib/pure_first_exe_guard.sh
---

# Mimalloc Row Validation Cadence

## Decision

Mimalloc / hako_alloc rows stay proof-first, but each row should use the
smallest sufficient validation level.

The rule is:

```text
validate the row's new contract directly
validate touched upstream/downstream compatibility narrowly
run pure-first EXE only when the row touches backend routes, introduces the
first pattern in a family, or closes out a batch
avoid broad gates unless the row changes a broad default or release boundary
```

This SSOT does not weaken existing guards. It prevents every new row from
guessing that all previous proof apps, allocator-wide gates, and full closeout
guards must run.

## Validation Levels

| Level | Use for | Required evidence | Do not run by default |
| --- | --- | --- | --- |
| `L0 static` | planning rows, docs-only selection rows, manifest/index checks | `bash tools/checks/current_state_pointer_guard.sh`, `git diff --check`, focused static guard if the row has one | proof apps, EXE guards |
| `L1 VM proof` | scalar `.hako` proof rows with an already-live route shape | one dedicated proof app through `run_proof_app.sh --only <id>` or the public row guard VM section | allocator-wide, unrelated row guards |
| `L2 MIR contract` | scalar proof / composition rows that must publish MIR metadata, route-preflight, or schema evidence | MIR JSON emit, route/preflight, metadata/schema checks owned by the dedicated guard | pure-first EXE unless the row meets an EXE-required rule |
| `L3 pure-first EXE` | backend route rows, first-pattern rows, C shim / lowering-plan / return-shape / value-demand changes, or closeout rows | exact MIR artifact -> route preflight -> `--mir-in` EXE build/run through the row guard | all historical EXE guards |
| `L4 batch regression pack` | closeout groups, release checkpoints, weekly/heavy validation | named batch/closeout pack, often one representative EXE app plus VM/MIR for the family | daily row work |

Broad gates such as `dev_gate.sh quick` and allocator-wide are explicit release
or default-change evidence. They are not part of the daily L0-L4 row cadence
unless the active card names them.

Compatibility note: older landed cards and guards may still use the phrase
`L2 proof`; read it as the current `L2 MIR contract` row unless that card
explicitly requires pure-first EXE.

## Validation Profiles

Manifest-backed rows may carry:

```toml
row_kind = "scalar-composition"
validation_profile = "scalar-mir"
first_pattern = false
closeout_pack = "segment-map-readiness"
exe = "auto"
```

Stable runner support:

```text
tools/checks/run_proof_app.sh --validation-profile scalar-mir --dry-run
tools/checks/run_proof_app.sh --row-kind inventory --dry-run
tools/checks/run_proof_app.sh --closeout-pack segment-map-readiness --dry-run
```

Current fields are selection metadata first. Existing public guard bodies still
own their full historical evidence until a later row splits VM/MIR/EXE sections
into separate manifest commands.

For split guards, no-argument public wrappers remain full L3. Manifest level
selection may call a level-specific command such as:

```toml
cmd_l2 = ["bash", "tools/checks/<guard>.sh", "--level", "L2"]
```

`--level L2` means static + VM + MIR JSON + route preflight, without EXE
build/run. `--level L3` or no argument keeps the existing EXE behavior.

Recommended profiles:

| Profile | Meaning | Default max level |
| --- | --- | --- |
| `planning` | row selection / docs-only | L0 |
| `inventory` | metadata or source inventory, no new behavior route | L0/L1/L2 as named by the row |
| `scalar-proof` | scalar behavior proof on existing routes | L2 |
| `scalar-mir` | scalar proof or composition requiring MIR/route preflight evidence | L2 |
| `first-pattern` | first row of a new proof shape or composition family | L3 |
| `backend-route` | lowering plan / C shim / route vocabulary change | L3 |
| `closeout` | family boundary freeze | L3/L4 |
| `batch` | named multi-row regression pack | L4 |

## Row-Type Rules

### Planning Rows

Planning rows select exactly one next row. They do not add allocator behavior.

Evidence:

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Planning rows must not require proof apps unless they also changed a durable
policy SSOT.

### Allocator Behavior Rows

Behavior rows add or change one contract in one owner family.

Evidence:

```text
bash tools/checks/run_proof_app.sh --only <ROW_ID>
bash tools/checks/<public-row-guard>.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The public row guard should own:

- file existence and executable checks;
- owner / card / SSOT / index / manifest wiring;
- VM output for the new proof shape;
- MIR JSON checks for the new contract surface;
- pure-first EXE parity only when the row is `first-pattern`,
  `backend-route`, or `closeout`;
- stop-line leak checks for the row's forbidden concepts.

### EXE Required Rules

L3 pure-first EXE is required when a row:

- adds or changes a backend route / `lowering_plan` proof;
- adds or changes `return_shape`, `value_demand`, or `definition_owner`;
- touches the C shim route reader / allowlist / emit behavior;
- changes `pure_first_route_preflight.py` reason vocabulary or schema
  contract;
- introduces the first pattern in a row family;
- opens a new object handle / typed object / user-box method / global-call
  route shape;
- affects exact MIR artifact, ny-llvmc, linker, or runtime ABI behavior;
- is an explicit closeout or batch pack row.

EXE may be omitted for existing-route scalar composition, metadata-only
inventory, docs-only planning, guard/index/manifest cleanup, and repeated rows
inside a family that already has first-pattern L3 evidence plus a closeout pack.

### Compatibility Guards

If a behavior row changes a report object, owner field, method return shape, or
shared counter used by older rows, run only the affected older guards.

Examples:

```text
release report changed
  -> run MIMAP-097A release guard
  -> run MIMAP-100A recycle guard if recycle consumes the release report

readiness report changed
  -> run readiness guard
  -> run modeled consume guard
```

Do not run unrelated reclaim, purge, facade, or provider guards for a local
segment-ledger report change.

### Closeout Rows

Closeout rows freeze a group boundary. They should not add allocator behavior.

Evidence:

```text
bash tools/checks/run_row_guard.sh --only <closeout-id>
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The public `k2_wide_*` wrapper may remain stable, but common routing should stay
manifest-backed where the family has already migrated.

### Broad Gates

Broad gates are explicit, not reflexive.

Use them when:

- the card changes `dev_gate.sh`, allocator-wide, or manifest defaults;
- a provider / host allocator replacement boundary is reopened;
- a release checkpoint asks for broad assurance;
- a row touches cross-family runtime substrate.

Do not use broad gates to compensate for an underspecified row guard. Improve
the row guard instead.

## Stop-Line Guarding

Rows should keep stop-line leak checks focused on the forbidden concepts for
that row. A segment-ledger row should not grow a generic repository-wide grep
bundle.

Common forbidden families for current segment-ledger rows:

```text
real segment allocation/free
free-list mutation
arena backing allocation
raw pointer residence
segment-map pointer membership / lookup
atomic bitmap execution
page-source / OSVM execution
thread scheduling / worker spawning
source-level concurrency feature changes
provider activation / hooks / host allocator replacement
backend .inc app/name matchers
```

## Current Practice

For the current segment allocation modeled lane:

```text
planning row:
  L0

existing-route scalar/inventory/composition row:
  L1/L2 dedicated proof or row guard
  L3 only for first-pattern/backend-route/closeout

segment-map readiness family:
  MIMAP-149A blocked substrate matrix:
    validation_profile = scalar-mir
  MIMAP-151A segment-map scalar lookup boundary inventory:
    validation_profile = inventory
  MIMAP-153A lookup guarded readiness composition:
    validation_profile = scalar-mir
  closeout pack:
    closeout_pack = segment-map-readiness

segment-map consume ledger family:
  MIMAP-157A accepted readiness modeled consume ledger:
    validation_profile = scalar-mir
    closeout_pack = segment-map-consume-ledger
    exe = deferred-to-closeout
  MIMAP-158A diagnostics:
    same proof app / same pack
  closeout pack:
    closeout_pack = segment-map-consume-ledger
    representative L3 EXE evidence in MIMAP-159A

segment-map consume ledger release family:
  MIMAP-161A segment-map consume-ledger release:
    validation_profile = scalar-mir
    closeout_pack = segment-map-consume-ledger-release
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-consume-ledger-release
    representative L3 EXE evidence in MIMAP-162A

segment-map consume ledger recycle family:
  MIMAP-164A segment-map consume-ledger released-token recycle:
    validation_profile = scalar-mir
    closeout_pack = segment-map-consume-ledger-recycle
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-consume-ledger-recycle
    representative L3 EXE evidence in MIMAP-166A

segment-map consume ledger released-span observation family:
  MIMAP-168A segment-map consume-ledger released-span observation:
    validation_profile = scalar-mir
    closeout_pack = segment-map-consume-ledger-released-span
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-consume-ledger-released-span
    representative L3 EXE evidence in MIMAP-170A

segment-map local-free candidate bridge family:
  MIMAP-172A segment-map released-span local-free candidate bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-candidate-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-candidate-bridge
    representative L3 EXE evidence in MIMAP-174A

segment-map local-free apply-plan bridge family:
  MIMAP-176A segment-map local-free apply-plan bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-apply-plan-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-apply-plan-bridge
    representative L3 EXE evidence in MIMAP-178A

segment-map local-free page-apply bridge family:
  MIMAP-180A segment-map local-free page-apply bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-page-apply-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-page-apply-bridge
    representative L3 EXE evidence in MIMAP-182A

segment-map local-free integration bridge family:
  MIMAP-184A segment-map local-free integration bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-integration-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-integration-bridge
    representative L3 EXE evidence in MIMAP-186A

segment-map local-free reuse bridge family:
  MIMAP-188A segment-map local-free reuse bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-bridge
    representative L3 EXE evidence in MIMAP-190A

segment-map local-free reuse ledger bridge family:
  MIMAP-192A segment-map local-free reuse ledger bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-bridge
    representative L3 EXE evidence in MIMAP-194A

segment-map local-free reuse ledger release bridge family:
  MIMAP-196A segment-map local-free reuse ledger release bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-release-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-release-bridge
    representative L3 EXE evidence in MIMAP-198A

segment-map local-free reuse ledger release apply bridge family:
  MIMAP-200A segment-map local-free reuse ledger release apply bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-release-apply-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-release-apply-bridge
    representative L3 EXE evidence in MIMAP-202A

segment-map local-free reuse ledger release-applied recycle bridge family:
  MIMAP-204A segment-map local-free reuse ledger release-applied recycle bridge:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-release-applied-recycle-bridge
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-release-applied-recycle-bridge
    representative L3 EXE evidence in MIMAP-206A

segment-map local-free reuse ledger release-applied recycle second-release diagnostic family:
  MIMAP-208A segment-map local-free reuse ledger release-applied recycle second-release diagnostic:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic
    representative L3 EXE evidence in MIMAP-210A

segment-map local-free reuse ledger lifecycle-token pilot family:
  MIMAP-212A segment-map local-free reuse ledger lifecycle-token pilot:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-lifecycle-token-pilot
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-lifecycle-token-pilot
    representative L3 EXE evidence in MIMAP-214A

segment-map local-free reuse ledger lifecycle-token observer diagnostic family:
  MIMAP-216A segment-map local-free reuse ledger lifecycle-token observer diagnostic:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic
    representative L3 EXE evidence in MIMAP-218A

segment-map local-free reuse ledger lifecycle-token release-key precondition family:
  MIMAP-220A segment-map local-free reuse ledger lifecycle-token release-key precondition observer:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition
    representative L3 EXE evidence in MIMAP-222A

segment-map local-free reuse ledger lifecycle-keyed release shadow family:
  MIMAP-224A segment-map local-free reuse ledger lifecycle-keyed release shadow:
    validation_profile = scalar-mir
    closeout_pack = segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow
    representative L3 EXE evidence in MIMAP-226A

source release-ledger lifecycle-key migration family:
  MIMAP-228A source release-ledger lifecycle-key migration pilot:
    validation_profile = first-pattern
    closeout_pack = source-release-ledger-lifecycle-key-migration
    exe = auto
  MIMAP-229A source lifecycle-keyed release ledger diagnostics:
    validation_profile = scalar-mir
    closeout_pack = source-release-ledger-lifecycle-key-migration
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = source-release-ledger-lifecycle-key-migration
    representative L3 EXE evidence in MIMAP-228A and MIMAP-230A

source lifecycle-keyed release apply/recycle continuation family:
  MIMAP-232A source lifecycle-keyed release apply/recycle continuation bridge:
    validation_profile = first-pattern
    closeout_pack = source-lifecycle-keyed-release-apply-recycle-continuation
    exe = auto
  MIMAP-233A source lifecycle-keyed release apply/recycle continuation diagnostics:
    validation_profile = scalar-mir
    closeout_pack = source-lifecycle-keyed-release-apply-recycle-continuation
    exe = deferred-to-closeout
  closeout pack:
    closeout_pack = source-lifecycle-keyed-release-apply-recycle-continuation
    representative L3 EXE evidence in MIMAP-232A and a future closeout pack

cleanup row using C199 compound assignment:
  L1 C199 guard
  L3 touched segment guards only

closeout row:
  L4 manifest-backed closeout guard
```

This is the expected cadence until a card explicitly reopens real segment
execution, allocator-wide defaults, provider activation, or host allocator
replacement.
