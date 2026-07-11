---
Status: Accepted
Date: 2026-07-11
Owner: repository-artifact-lifecycle-current.md
Decision: accepted (Candidate A)
---

# H3 Design Authority Registry Consultation

## Stop Reason

The design root has enough contradictory evidence that authority membership
cannot be selected mechanically. No design document moves in this stop.

## Measured Current State

Source:
`tools/checks/manifests/repository_artifact_lifecycle_v0.json`

```text
design direct files = 848
markdown files = 827
non-markdown sidecars = 21

status active_like = 127
status closed = 11
status other_or_missing = 689

README referenced direct files = 122
DOCS_LAYOUT referenced direct files = 52
AGENTS referenced direct files = 14
CURRENT_STATE referenced direct files = 2
seed union = 160
unseeded = 688
```

Existing `design/README.md` is a navigation list, not a closed authority
registry. It names only part of the root and does not define precedence,
supersession, sidecar ownership, or retirement rules.

## Hard Constraints

```text
1. Filename suffix `-ssot` is not sufficient authority evidence.
2. Status text is not sufficient; 689 documents have no usable status.
3. README/DOCS_LAYOUT/AGENTS/CURRENT_STATE references are seed evidence only.
4. A TOML sidecar moves with its owning design document or stays with it.
5. No unlisted document is bulk-moved before its disposition is explicit.
6. Registry rollout starts warning-only and becomes strict after backlog closeout.
7. Existing links must resolve through move, rewrite, resolver, or forwarding stub.
```

## Candidate Designs

### A. Explicit Closed Registry

Create `design/INDEX.md` with one typed row per retained document:

```text
document
role = authority | navigation | supporting | status-ledger
owner
precedence_parent
sidecars
supersedes
retire_when
```

Only `role = authority` participates in normative precedence. Unregistered
files remain in place during migration, then move to `design/superseded/` only
after an explicit disposition row exists.

### B. Seed Union Auto-Authority

Treat the 160 documents referenced by current entry surfaces as authority and
move the other 688 files after reference closure.

Risk: navigation popularity becomes semantic authority and supporting ledgers
are promoted accidentally.

### C. Status/Filename Inference

Infer authority from `Status:` plus `-ssot` naming.

Risk: most files have no usable status, and historical files may still carry
SSOT-shaped names. This cannot provide a closed precedence relation.

## Recommended Selection

Adopt A. Use the current seed union only as the first review queue, never as
automatic authority. Keep migration additive until every move has an explicit
registry disposition.

## Questions Requiring Decision

```text
1. Is design/INDEX.md the sole membership and precedence owner?

2. Are the allowed roles exactly:
     authority
     navigation
     supporting
     status-ledger
     superseded
   or should supporting/status-ledger stay outside the registry?

3. Must every top-level design file receive a row before strict mode, including
   TOML sidecars through their owning row?

4. May unregistered files remain temporarily with warning diagnostics, or must
   they fail immediately once INDEX.md lands?

5. Does supersession require both `superseded_by` and reference-closure proof
   before physical movement?

6. Should `design/README.md` become navigation-only and be generated or checked
   against INDEX.md rather than independently curated?
```

## Accepted Answers

```text
1. INDEX.md is the sole design-root membership and precedence owner.
   The language charter remains the normative language-law owner.

2. All five roles stay inside the registry:
     authority
     navigation
     supporting
     status-ledger
     superseded

3. Every direct design file must have a row before strict mode.
   Sidecars are owned through the parent row.

4. Rollout is warning-first. Unregistered count may decrease but must not grow.

5. Physical supersession requires superseded_by plus reference closure.

6. README is navigation-only and must be generated or checked against INDEX.
```

## Minimum Implementation After Acceptance

```text
1. Add typed design registry schema and parser.
2. Register the accepted seed review set without claiming completeness.
3. Add warning diagnostics for unregistered direct files and broken sidecars.
4. Add precedence-cycle and missing-owner checks.
5. Produce an explicit unregistered review queue.
6. Move only rows marked superseded with reference closure.
7. Enable strict mode only when unregistered count reaches zero.
```

## Non-Claims

```text
design_registry_decided = 1
design_registry_complete = 0
seed_union_is_authority = 0
unseeded_documents_superseded = 0
design_file_move_started = 0
strict_design_registry_guard = 0
failure_outcome_design_accepted = 0
selfhost_claim = 0
```
