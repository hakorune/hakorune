---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M89 allocator provider activation decision diagnostic report.
Related:
  - docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-owner-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-v0.toml
  - tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh
---

# Allocator Provider Activation Decision Diagnostic Report (SSOT)

## Goal

Add a diagnostic-only runtime report for the M87 activation decision fixture.

M89 parses caller-provided TOML text only. It does not expose an implicit CLI
route or discover files implicitly. It does not select a provider, consume
proof bundles, prepare rollback, open the activation gate, activate hooks, or
replace the process allocator.

## Runtime Owner

```text
src/runtime/allocator_provider_activation_decision.rs
```

Public diagnostic entry:

```text
validate_allocator_provider_activation_decision_from_text(text)
  -> AllocatorProviderActivationDecisionReport
```

## Output Contract

For a complete reserved fixture, the report is ready but blocked:

```text
status = ReadyBlocked
diagnostic = [allocator-provider/activation-decision-blocked]
activation_decision_surface_status = reserved_fixture
activation_decision_allowed = false
would_select_provider = false
would_consume_proof = false
would_prepare_rollback = false
would_open_activation_gate = false
would_install_hook = false
would_replace_process_allocator = false
would_activate = false
```

For missing or malformed input:

```text
status = MissingFacts
diagnostic = [allocator-provider/activation-decision-reserved]
parse_error = Some(...) only for malformed TOML
missing_facts = stable fact names
missing_diagnostics = stable diagnostic tags
```

## Stop Line

M89 itself keeps these inactive:

- implicit activation decision CLI route or runtime file discovery;
- provider selection;
- provider proof consumption;
- rollback preparation/execution;
- activation gate opening;
- hook activation implementation;
- implicit runtime file-system manifest/report/proof discovery;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Row

M90 may expose this report through an explicit CLI diagnostic surface:

```text
hakorune --allocator-provider-activation-decision <ACTIVATION_DECISION_TOML>
```

It must remain explicit-input only and keep all activation fields false.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh
cargo test -q activation_decision -- --nocapture
git diff --check
```
