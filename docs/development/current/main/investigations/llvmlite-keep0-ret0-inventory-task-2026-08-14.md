---
Status: Design stop; inventory, guard, and archive-approval split closed
Date: 2026-08-14
Parent: docs/development/current/main/investigations/llvm-native-library-llvmlite-graduation-task-2026-07-22.md
Current-row: LLVMLITE-ORACLE-COVERAGE-D0
Scope: frozen llvmlite keep-lane source, consumer, fixture/golden, artifact, and restore inventory
---

# LLVMLITE-KEEP0-RET0 inventory and archive admission

This is the bounded child task for G3. It prepares the evidence needed for a
later archive/deletion decision; it does not move or delete source, create an
external archive, or change any route.

## Six-line brief

```text
Decision: build one machine-readable inventory before any archive or deletion approval.
Source authority + canonical issuer: tracked keep roots, G0/G2/shared-smoke manifests, and explicit owner paths.
Non-authority: keep labels, comments, Python output, MIRBuilder completion, and default Boundary status.
Fail-fast boundary: unclassified root/consumer/fixture, missing provenance/checksum/restore evidence, or invented archive URI stops the row.
Smallest next slice: classify every keep-root path, census row, fixture/golden candidate, and preserved artifact in one manifest.
Non-claims: no source move/deletion, external archive publication, new llvmlite semantics, fallback, retry, or production switch.
```

## Evidence inputs

The inventory must reference, rather than duplicate, these existing owners:

```text
docs/development/current/main/investigations/llvmlite-production-ingress-census-v0.json
docs/development/current/main/investigations/llvmlite-default-independence-census-v0.json
docs/development/current/main/investigations/llvmlite-shared-smoke-caller-census-v0.json
src/llvm_py/**
tools/llvmlite_harness.py
tools/smokes/**/compat/llvmlite-monitor-keep/**
tools/historical/**/pyvm_vs_llvmlite.sh
```

At task creation, the measured baseline is 254 tracked `src/llvm_py` paths
plus `tools/llvmlite_harness.py`, 37 G0 rows, 17 G2 rows, and 29 shared-smoke
caller rows. The inventory must record the observed counts and reject drift,
not silently treat the counts as permanent truth.

## Proposed output

Create exactly one G3 machine-readable manifest at:

```text
docs/development/current/main/investigations/llvmlite-keep0-ret0-inventory-v0.json
```

The manifest is an inventory receipt, not an archive authority. Its top-level
sections should be:

```text
schema / status / source_commit
source_roots
consumer_rows
fixture_golden_candidates
artifact_matrix
restore_entries
archive_decision_fields
```

Each source or consumer entry must have an explicit classification:

```text
retain_keep | convert_to_fixture | archive_candidate | reference_only | blocked
```

Each fixture/golden candidate must record its path, semantic family, expected
output/exit-code evidence, independent-oracle status, and classification. Each
artifact row must record platform/target, path or `unavailable`, checksum and
provenance status. An unavailable artifact is an explicit gap, not a pass.

Each consumer row must retain its G0/G2/shared-smoke row ID, owner path,
selector/driver evidence, class, and whether it is production, explicit
compat/oracle, reference-only, or already archived. Duplicate row IDs and
unclassified direct consumers are errors.

Consumer scope is source-backed route ownership, not a raw text grep. Mentions
in historical docs, archived smoke names, capability tests, guard descriptions,
or README examples are `reference_only` unless a G0/G2/shared-smoke row names
the executable owner and selector. A broad reference sweep is useful for
review, but it cannot prove a runtime consumer or zero-consumer deletion state.

Archive fields are deliberately nullable until an owner exists:

```text
archive_owner
archive_uri
archive_tag
source_tree_or_commit
artifact_checksums
restore_command
deletion_approval
```

Do not invent an external URI, repository, tag, or checksum. The placement
policy remains `backend-legacy-preservation-and-archive-ssot.md`; this child
task does not create a competing archive root.

### Baseline inventory receipt (2026-08-14)

`llvmlite-keep0-ret0-inventory-v0.json` now records 255 tracked keep-root
paths, 8 restore/monitor support paths, 83 source-qualified G0/G2/shared-smoke
consumer rows, and 119 fixture/golden candidates. Four platform artifact rows
are explicit `blocked`/`unavailable`; archive owner, URI, tag, checksums, and
deletion approval remain null. The manifest was checked against `git ls-files`
and all three source manifests; the reusable guard is still a follow-on gap.

## Closed child: LLVMLITE-KEEP0-RET0-I0-GUARD-R0

```text
Decision: add one G3-specific inventory guard; do not extend G0/G2 guards.
Source authority + canonical issuer: the inventory manifest plus git ls-files and the three source manifests.
Non-authority: keep labels, comments, Python output, MIRBuilder completion, Boundary defaults, and missing archive URI.
Fail-fast boundary: schema/status, root drift, duplicate or missing source-qualified row IDs, invalid classifications, and missing evidence fields.
Smallest next slice: guard the 255 roots, 8 support roots, 83 consumer rows, 119 candidates, 4 artifact rows, and 3 restore entries.
Non-claims: no source movement/deletion, archive publication, new semantics, fallback, or retry.
```

The implementation is a reusable focused guard at
`tools/checks/llvm_llvmlite_keep0_inventory_guard.py`; it must consume the
manifest as data, compare exact tracked sets, and remain independent of the
G0/G2 route/default guards.

The guard is landed and green. It verifies exact tracked/support roots,
source-qualified G0/G2/shared-smoke row IDs, fixture candidates, platform
artifact gaps, restore entries, nullable archive fields, and duplicate-root /
duplicate-consumer negative cases. It does not publish or delete anything.

## Acceptance

The row is complete only when all of the following are mechanically observable:

1. Exactly one manifest covers every tracked keep-root path and the named
   explicit harness/monitor/fixture support root.
2. Every G0, G2, and shared-smoke row ID appears exactly once or is explicitly
   classified as superseded by a named row.
3. Every fixture/golden candidate has a classification and independent-oracle
   status; missing evidence is `blocked`, never inferred from Python output.
4. Every artifact has platform/provenance/checksum state, including explicit
   `unavailable` entries where preservation has not happened.
5. Restore commands are recorded for retained/archive candidates, or the row
   is `blocked`.
6. Archive owner/URI/tag and deletion approval remain nullable when unknown;
   no external archive destination is fabricated.
7. A reusable focused guard rejects duplicate/unclassified rows, root drift,
   missing required fields, fallback/retry claims, and source deletion.
8. `CURRENT_STATE` and this card remain within their line budgets; no source,
   fixture, route, or backend behavior changes in this row.

## Stop conditions and next handoff

Stop and return to design review if the inventory requires source movement,
new llvmlite lowering, a second oracle authority, fallback/retry, or an
external archive owner that has not been supplied. After this inventory and
guard are green, archive publication remains a separate design stop; only a
later row may decide source deletion after zero-or-archived consumer evidence.

## Closed decision: LLVMLITE-KEEP0-RET0-ARCHIVE-D0

```text
Decision: split in-repo archive movement, external preservation publication, and source deletion into three separately approved transitions.
Source authority + canonical issuer: backend archive SSOT, inventory-v0, repository-owner instruction, and later exact registration/preservation manifests.
Non-authority: keep labels, existing archive directories, old mirrors/tags, MIRBuilder completion, Python output, and guessed URI/path names.
Fail-fast boundary: retained consumers, missing independent oracle, incomplete dependency closure, unavailable artifacts, or missing exact destination stops movement/publication.
Smallest next slice: classify the oracle/fixture closure before selecting an exact in-repo archive destination or moving tracked source.
Non-claims: llvmlite is unnecessary, exact move approved, external archive registered/published, source deleted, fallback/retry changed, or new semantics added.
```

The repository owner is the sole developer and has approved archive movement
in principle when the lane is proven unnecessary. This is
`policy_consent_only`: it is not an exact move approval because the source
paths and destination have not been selected. It is also not external archive
publication authority and not source deletion approval.

Read-only repository and GitHub census found no existing llvmlite archive
repository, release, or common immutable tag convention. Do not promote an
old mirror, the current `archive/` tree, or a guessed repository name into an
archive authority. The current inventory also still has 50 `retain_keep`
consumer rows, 119 unassessed fixture/golden candidates, and four unavailable
platform artifact rows, so “unnecessary” is not yet established.

### Approval model

Keep these transitions distinct in the eventual registration schema:

```text
in-repo archive movement:
  policy_consent_only | exact_move_approved | executed

external copy publication:
  not_requested | approved | published_verified | revoked

current-repo source deletion:
  not_requested | approved | executed
```

An exact move approval must name the complete source set and one destination.
External publication must name an owner-issued repository URI, immutable tag,
resolved tag object, and release policy. Deletion always needs a fresh,
separate approval after publication and consumer closure.

## Current design stop: LLVMLITE-ORACLE-COVERAGE-D0

```text
Decision: prove the smallest independent replay/oracle closure before declaring the keep lane unnecessary or selecting an archive destination.
Source authority + canonical issuer: inventory-v0, G0/G2/shared-smoke rows, tracked fixture dependencies, reviewed expected results, and the llvmlite opcode census.
Non-authority: llvmlite's own output, object-file existence, raw text grep, transport tests, and the general archive-movement consent.
Fail-fast boundary: any unclassified candidate/dependency, missing independent expected-result issuer, uncovered opcode, fallback/retry, or new lowering stops the row.
Smallest next slice: classify all 119 candidates, add the six replay dependencies, and map every supported opcode to a fixed case or archive-only disposition.
Non-claims: oracle bundle built, llvmlite unnecessary, exact archive path selected, source moved/deleted, artifact preserved, or external archive registered.
```

The read-only classification target is:

```text
retain_keep          6
archive_candidate   84
reference_only      27
blocked              2
convert_to_fixture   0 until an independent expected-result issuer exists
```

The dependency closure must add these six paths before staging a bundle:

```text
tools/smokes/v2/suites/integration/compat/llvmlite-monitor-keep.txt
tools/selfhost/examples/gen_v1_const42.sh
tools/selfhost/examples/gen_v1_compare_branch.sh
apps/tests/hello_simple_llvm_native_probe_v1.mir.json
apps/tests/ternary_nested.hako
apps/tests/loop_if_phi.hako
```

The minimal reviewed oracle seed is O1 const/return, O2 compare/branch, O3
print/extern, O4 StringBox call, O5 nested merge, O6 loop/PHI, and O7 an
unsupported CheckedCallOut reject-before-effect case. Each row needs fixed
input hash, independently issued expected stdout/exit/reject class, observed
legacy IR kept as non-authority, toolchain versions, artifact checksum, and
fallback/retry zero. All supported llvmlite opcodes must map to one seed or an
explicit `archive_only` disposition before this row closes.

## Shallow execution DAG

Keep this DAG in this card; do not create one task document per row.

```text
closed inventory + guard + approval split
  -> LLVMLITE-ORACLE-COVERAGE-D0
  -> LLVMLITE-ORACLE-BUNDLE-I0
  -> LLVMLITE-ORACLE-GUARD-R0
       |
       +-> LLVMLITE-REPO-ARCHIVE-PLACEMENT-D0
       |    -> LLVMLITE-REPO-ARCHIVE-R0
       |
       +-> LLVMLITE-ARCHIVE-ARTIFACT-CONTRACT-D0
            -> LLVMLITE-BUNDLE-STAGE-R0
            -> LLVMLITE-BUNDLE-VERIFY-R0
            + LLVMLITE-ARCHIVE-REG-I0
            -> LLVMLITE-ARCHIVE-PUBLISH-I0
                 -> LLVMLITE-ARCHIVE-DOCS-R0
                 -> LLVMLITE-ARCHIVE-CENSUS-R0
                      -> LLVMLITE-DELETE-D0
                      -> LLVMLITE-DELETE-R0
```

### In-repo archive track

`LLVMLITE-REPO-ARCHIVE-PLACEMENT-D0` selects exactly one tracked archive root,
the paths to move, import/runner rewrites, and restore entrypoint. It may open
only after the independent oracle bundle and source-backed consumer census are
closed. Existing archive names are precedent, not placement authority.

`LLVMLITE-REPO-ARCHIVE-R0` is a count-neutral physical relocation: move the
exact approved paths, update explicit compat/oracle callers and guards in the
same bounded series, keep automatic production ingress/fallback/retry at zero,
and prove the archive replay. This track may be the terminal outcome when the
source remains preserved in this repository; it does not claim external
preservation or deletion readiness.

### External preservation track

Archive registration and observed bytes are separate authorities:

```text
inventory-v0
  = current repository source/consumer universe

owner registration
  = destination, immutable tag, and publication authorization

external preservation manifest
  = actual source/artifact/provenance/restore bytes

admission review
  = read-only co-check of the three; never an authority that invents fields
```

`LLVMLITE-ARCHIVE-ARTIFACT-CONTRACT-D0` fixes, per platform, support status,
native runner, target triple, artifact kind, exact fixture set, pinned Python /
llvmlite / LLVM versions, naming, smoke result, and checksum/provenance issuer.
Linux is build-capable but unpreserved; Windows and macOS lack a llvmlite
artifact recipe; iOS remains unknown until explicitly marked supported or
unsupported by owner evidence.

`LLVMLITE-BUNDLE-STAGE-R0` copies, never moves, the exact commit/tree members
into a deterministic local bundle. `LLVMLITE-BUNDLE-VERIFY-R0` checks exact
membership, SHA-256, concrete fixed-fixture replay, and independent oracle
origin. The durable preservation SSOT requires both llvmlite and Rust backend
components, so this lane alone may reach `llvmlite_component_verified` but may
not claim a complete legacy bundle.

`LLVMLITE-ARCHIVE-REG-I0` accepts exactly one owner-issued external repository,
immutable tag/resolved object, release manifest location, and publication
credential handle. No URI or tag is inferred. `LLVMLITE-ARCHIVE-PUBLISH-I0`
requires the verified full bundle, performs remote readback checksum parity,
and does not move/delete current source. Current docs receive the exact link
only after verified publication.

### Optional deletion track

`LLVMLITE-DELETE-D0` opens only after external publication, exact current-doc
references, and a refreshed census where every executable consumer is zero or
mapped to an exact archived member. It requires a new owner decision naming
the deletion scope. The current archive-movement consent cannot satisfy it.
`LLVMLITE-DELETE-R0` is destructive retirement, not BoxCount; it must keep
fallback/retry zero and prove restore from the external bundle.

## Cross-row stop line

- Do not call llvmlite output an independent semantic oracle.
- Do not treat Linux local build capability as a preserved artifact.
- Do not treat generic Windows/macOS builds as llvmlite evidence.
- Do not interpret iOS `unknown` as `not_supported` or pass.
- Do not invent an in-repo destination, external URI, tag, checksum, artifact,
  or release convention.
- Do not move source while any executable consumer lacks an exact archive or
  fixture disposition.
- Do not conflate archive movement, external publication, and deletion.
- Do not add new llvmlite lowering, production ingress, fallback, or retry.
