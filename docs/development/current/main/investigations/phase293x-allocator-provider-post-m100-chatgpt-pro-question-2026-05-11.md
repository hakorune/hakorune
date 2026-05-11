---
Status: external-review-question
Date: 2026-05-11
Scope: phase-293x allocator provider ladder after M100
Audience: ChatGPT Pro external design review
Related:
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# Phase 293x Allocator Provider Post-M100 Review Question

## Copy-Paste Prompt

Please review the allocator provider / activation ladder after M100 and advise
how to continue without turning the work into endless diagnostic-only cards.

This repository follows a structure-first rule. Do not suggest activating the
process allocator directly. First evaluate the staging, ownership boundaries,
and minimum safe next implementation row.

Please read these files first:

```text
docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-ssot.md
docs/development/current/main/design/allocator-provider-proof-bundle-consumption-cli-surface-ssot.md
docs/development/current/main/design/allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md
docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md
docs/development/current/main/CURRENT_STATE.toml
```

Optional code/guard files:

```text
src/runtime/allocator_provider_registry.rs
src/runtime/allocator_provider_proof_bundle_consumption.rs
src/cli/allocator_provider_proof_bundle_consumption.rs
tools/checks/lib/allocator_provider_forbidden_patterns.sh
tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh
tools/checks/k2_wide_allocator_gate.sh
```

## Current State

The allocator provider ladder is closed through M100.

Recent commits:

```text
2c83c49a9 Add proof bundle consumption entry contract
911202c43 Sync guard contracts after route splits
3da24ccc9 Add proof bundle consumption CLI diagnostic
37d7674b3 Sync allocator provider restart pointers
e92610adc Extract MIR JSON metadata emitter
0fdcc5eba Tighten stale internal helper surfaces
```

Verification already passed after M100:

```text
bash tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh
bash tools/checks/k2_wide_allocator_gate.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## What M100 Fixed

M100 reserves the future proof-bundle consumption behavior owner and entry:

```text
owner = src/runtime/allocator_provider_activation.rs
entry = allocator_provider_proof_bundle_consumption_attempt
```

M100 intentionally does not implement behavior. These are still inactive:

```text
provider selection
proof consumption
rollback preparation
activation gate opening
hook activation
native activation
process allocator replacement
```

The M99 CLI remains diagnostic-only:

```text
hakorune --allocator-provider-proof-bundle-consumption <TOML>
```

It reads only caller-provided TOML and prints inactive report fields.

## Concern

The ladder has become highly defensive and documentation-heavy. That has helped
avoid unsafe activation, but it may now be delaying actual implementation. The
repo currently has no numbered M101+ plan. The only post-M100 fixture text says:

```text
later proof bundle consumption implementation row
later rollback preparation entry row
```

We need a concrete next sequence that starts useful implementation while still
preserving the stop line.

## Current Working Hypothesis

One safe path is:

```text
M101 post-M100 task order / selected-provider precondition contract
M102 proof bundle consumption implementation proposal + fixture
M103 runtime fail-fast entry in allocator_provider_activation.rs
M104 actual proof validation / consumption report
M105 rollback preparation entry contract
```

The main uncertainty is whether M101/M102 are necessary, or whether M103 can be
the next row if it is strictly fail-fast and keeps `proof_bundle_consumed=false`
until a selected provider exists.

## Constraints

Please assume these constraints are hard unless you give a strong reason to
change them:

- no `#[global_allocator]`;
- no `GlobalAlloc`;
- no process allocator replacement;
- no implicit environment provider selection;
- no implicit manifest/report/proof discovery;
- no `.inc` provider/facade/policy name matching;
- no route widening for allocator activation;
- no provider selection hidden inside proof consumption;
- no proof consumption hidden inside the M99 CLI;
- no rollback/gate/hook behavior before explicit rows.

## Questions

1. Is M101 as a task-order / selected-provider precondition card necessary, or
   should the next row be a small runtime fail-fast implementation?
2. Should future proof consumption live directly in
   `src/runtime/allocator_provider_activation.rs`, or should that owner only
   orchestrate a narrower internal module?
3. What is the minimum useful M101/M102/M103 sequence that reduces diagnostic
   churn and starts implementation safely?
4. Should actual proof consumption require a real selected provider first, or
   can it consume the explicit proof bundle while still reporting
   `selected_provider_id_absent=true`?
5. What should the first behavior row return on incomplete input:
   a structured inactive report, a fail-fast error enum, or both?
6. How should the guard strategy change so each row does not add another large
   docs and shell-script burden?
7. Is there a better stop point than "proof consumed" before rollback work
   begins?
8. Are there any signs that the current owner split is wrong:
   registry diagnostic facade, activation decision diagnostics, CLI diagnostic
   surface, and future activation owner?

## Requested Answer Format

Please answer with:

```text
1. Recommended next row name and scope
2. Rows to delete, merge, or skip
3. First implementation file(s) to touch
4. First return/report type shape
5. Guard strategy
6. Risks in the current plan
7. A concise M101-M105 proposed ladder
```

Focus on whether we can safely move from diagnostic-only work into a small
runtime implementation row without accidentally activating allocator replacement.
