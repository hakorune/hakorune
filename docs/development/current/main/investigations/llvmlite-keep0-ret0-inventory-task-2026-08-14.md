---
Status: Design stop; inventory task only
Date: 2026-08-14
Parent: docs/development/current/main/investigations/llvm-native-library-llvmlite-graduation-task-2026-07-22.md
Current-row: LLVMLITE-KEEP0-RET0-I0-GUARD-D0
Scope: frozen llvmlite keep-lane source, consumer, fixture/golden, artifact, and restore inventory
---

# LLVMLITE-KEEP0-RET0-I0-INVENTORY

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

## Next design stop: LLVMLITE-KEEP0-RET0-I0-GUARD-D0

```text
Decision: add one G3-specific inventory guard; do not extend G0/G2 guards.
Source authority + canonical issuer: the inventory manifest plus git ls-files and the three source manifests.
Non-authority: keep labels, comments, Python output, MIRBuilder completion, Boundary defaults, and missing archive URI.
Fail-fast boundary: schema/status, root drift, duplicate or missing source-qualified row IDs, invalid classifications, and missing evidence fields.
Smallest next slice: guard the 255 roots, 8 support roots, 83 consumer rows, 119 candidates, 4 artifact rows, and 3 restore entries.
Non-claims: no guard implementation in this design stop, source movement/deletion, archive publication, new semantics, fallback, or retry.
```

The later implementation must be a reusable focused guard at
`tools/checks/llvm_llvmlite_keep0_inventory_guard.py`; it must consume the
manifest as data, compare exact tracked sets, and remain independent of the
G0/G2 route/default guards.

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
external archive owner that has not been supplied. After this inventory is
accepted and its guard is green, a separate approval may choose archive
publication; only a later row may decide source deletion after zero-or-archived
consumer evidence.

The next row is not selected by this card. It must be chosen from the manifest
gaps after the inventory guard is green.
