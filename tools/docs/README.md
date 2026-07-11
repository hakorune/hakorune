# Docs Tooling

This directory holds small helpers that reduce current-doc synchronization
work. Helpers here should be narrow and explicit: they may update compact
current-state pointers and create row skeletons, but they must not rewrite
historical phase ledgers or move docs.

## Phase Row Writer

Use `tools/docs/phase_row.py` for new current-row boilerplate.

Default mode is dry-run:

```bash
python3 tools/docs/phase_row.py create \
  --row 295x-200 \
  --row-number 200 \
  --slug EXAMPLE-ROW \
  --title "Example Row" \
  --scope "example scope" \
  --blocker EXAMPLE-BLOCKER-295X-001 \
  --summary "selected the example follow-on" \
  --previous-card docs/development/current/main/phases/phase-295x/295x-199-example.md \
  --queue-boundary "Select the example follow-on." \
  --land-row 199
```

Add `--write` only after checking the dry-run output.

The helper owns only repetitive row mechanics:

- create the phase card skeleton;
- update `CURRENT_STATE.toml` latest-card fields;
- update the short taskboard current blocker / queue when requested;
- append a check-script index row when `--guard` and `--guard-description` are
  provided.

It does not replace row-specific engineering judgment, implementation, or
evidence checks.

## Hako Alloc Segment Review Projection

Use `tools/docs/hako_alloc_segment_family_projection.py` to generate the C2
review projection for the 163 `hako-alloc-segment` documents:

```bash
python3 tools/docs/hako_alloc_segment_family_projection.py --write
python3 tools/docs/hako_alloc_segment_family_projection.py --check
```

The generated manifest records body evidence, subfamily candidates, and role
hints only. It deliberately leaves `owner`, `precedence_parent`, and
`sidecar_owner` empty. It is a review queue, never an authority or role
assignment mechanism.

## Failure/Outcome Evidence Queue

Use `tools/docs/failure_outcome_site_inventory.py` for the 3505 first-stage
evidence queue:

```bash
python3 tools/docs/failure_outcome_site_inventory.py --write
python3 tools/docs/failure_outcome_site_inventory.py --check --strict
python3 tools/docs/failure_outcome_semantic_site_graph.py --write
python3 tools/docs/failure_outcome_semantic_site_graph.py --check
```

The manifest scans `src` and `docs/reference` for named null-like and
outcome-related evidence. It records locations without inferring semantic
class, owner, or target carrier. The semantic-site graph keeps those evidence
occurrences as `evidence_refs`, adds line-independent operation/outcome sites,
and rejects invalid four-segment IDs, missing compatibility profiles, and
increasing `missing_argument_zero` pending counts. Pending rows are expected
until the S4 exhaustiveness checker accepts a classified inventory.
