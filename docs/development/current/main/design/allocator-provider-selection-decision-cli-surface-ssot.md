---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M97 allocator provider selection decision CLI surface.
Related:
  - docs/development/current/main/design/allocator-provider-selection-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-v0.toml
  - src/cli/allocator_provider_selection_decision.rs
  - tools/checks/k2_wide_allocator_provider_selection_decision_cli_surface_guard.sh
---

# Allocator Provider Selection Decision CLI Surface (SSOT)

## Goal

Expose the M96 diagnostic-only allocator provider selection decision report
through one explicit CLI path:

```text
hakorune --allocator-provider-selection-decision <SELECTION_DECISION_TOML>
```

The CLI reads only the caller-provided TOML path. It does not perform implicit
file discovery, read provider selection environment toggles, select a provider,
consume proofs, prepare rollback, open an activation gate, install hooks,
activate a native allocator, or replace the process allocator.

## CLI Owner

The CLI owner is:

```text
src/cli/allocator_provider_selection_decision.rs
```

The runtime report owner remains:

```text
src/runtime/allocator_provider_registry.rs
```

The CLI must call:

```text
validate_allocator_provider_selection_decision_from_text(...)
```

## Output Contract

For the complete reserved fixture, the CLI prints:

```text
diagnostic=[allocator-provider/selection-decision-inactive]
selection_decision_status=ready_inactive
parse_error=
missing_facts=
missing_diagnostics=
requested_provider_id=native_mimalloc
required_operations=alloc,realloc,free
candidate_provider_ids=native_system_malloc,native_mimalloc,hako_model_allocator,debug_guarded_allocator
selected_provider_id=none_reserved
selected_provider_id_absent=true
active_registry_built=false
would_build_registry=false
would_select_provider=false
would_consume_proof=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

Ready inactive reports exit `0`. Malformed or incomplete input exits `2` and
keeps every action boolean false.

Read failures use the stable tag:

```text
[allocator-provider/selection-decision-cli-read-error]
```

## Conflict Contract

Allocator diagnostic CLI modes are mutually exclusive. Combining this surface
with another allocator diagnostic mode must exit `2` and report:

```text
[allocator-diagnostic/cli-conflicting-modes]
```

## Stop Line

M97 keeps these inactive:

- active runtime provider registry construction;
- provider selection;
- proof consumption;
- rollback preparation or execution;
- activation gate opening;
- hook activation or native activation;
- provider selection environment toggles, including `NYASH_ALLOCATOR_PROVIDER`,
  `HAKO_ALLOCATOR_PROVIDER`, and broad `ALLOCATOR_PROVIDER_*` names;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_selection_decision_cli_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
