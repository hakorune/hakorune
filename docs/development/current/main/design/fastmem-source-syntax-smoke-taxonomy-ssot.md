---
Status: SSOT
Scope: fastmem source-syntax smoke row taxonomy and discovery rules
Decision: accepted
Related:
- tools/hako_check/fastmem_source_manifest_runner.py
- tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
- tools/hako_check/README.md
- docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
---

# FastMem source-syntax smoke taxonomy SSOT

## Goal

Keep `fastmem_source_syntax_smoke.toml` readable while preserving the current
manifest-driven runner contract.

This taxonomy separates the stage that fails from the stage that still runs:

- fixture stage: AST / MIR / MIR emit
- producer stage: report / check

That split keeps new rows easy to classify and avoids the "one row, many
implicit meanings" problem that made the smoke suite slow to maintain.

## Canonical row kinds

### Fixture kinds

These kinds describe the source-to-MIR preflight stage for `[[fixtures]]`.

- `success`
  - AST and MIR emission succeed.
  - producer rows may still fail or pass independently.
- `ast_failure`
  - AST inventory is expected to fail.
- `mir_failure`
  - MIR inventory is expected to fail.
- `mir_emit_failure`
  - `--emit-mir-json` is expected to fail before inventory runs.

### Producer kinds

These kinds describe each `[[fixtures.producers]]` row.

- `success`
  - producer report succeeds
  - `fastmem-check` succeeds
- `report_failure`
  - producer report is expected to fail
- `check_failure`
  - producer report succeeds
  - `fastmem-check` is expected to fail

## Authoring rule

Prefer writing the kind explicitly when adding a new row.

- `kind` on `[[fixtures]]` names the fixture stage.
- `kind` on `[[fixtures.producers]]` names the producer stage.

If `kind` is omitted, the runner infers it from the existing boolean flags for
backward compatibility.

## Inference contract

The runner keeps the existing boolean flags as the source of truth:

- `ast_expect_failure`
- `mir_expect_failure`
- `mir_emit_expect_failure`
- `expect_failure`
- `check_expect_failure`

If `kind` is not present, the runner infers:

- fixture kind from the fixture-level failure flags
- producer kind from `expect_failure` / `check_expect_failure`

The inferred kind is used for validation and logging only. It does not change
execution semantics.

## Layout rules

Use the narrowest kind that explains the row.

- Do not mix a preflight failure row with a producer failure row in one new
  fixture if they can be split cleanly.
- If a fixture has multiple producers with different outcomes, keep the
  fixture kind at the preflight stage and classify each producer separately.
- Keep the row taxonomy stable; split new semantic families into new fixtures
  instead of overloading existing ones.

## Non-goals

- Do not change the smoke runner semantics.
- Do not change fastmem lowering behavior.
- Do not use the taxonomy to justify new fallback behavior.
- Do not turn this into a generic smoke taxonomy for unrelated suites.

