---
Status: Active
Scope: valid `.hako` programs that reach Program(JSON) must be accepted by the canonical Hako MIR builder, one pinned shape at a time.
Related:
  - docs/development/current/main/phases/phase-29bq/README.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh
  - tools/hakorune_emit_mir.sh
---

# 29bq-119 - Hako MIR Builder Valid-Hako Acceptance Repair

## Purpose

This is not a new policy.

Existing rule:

```text
If the .hako syntax is valid and the parser emits valid Program(JSON),
then failure belongs to the MIR builder acceptance/lowering path unless a
separate parser or Program(JSON) contract violation is proven.
```

The task is to strengthen the canonical Hako MIR builder acceptance system,
not to change `.hako` syntax, add Stage-B workarounds, or add by-name
shortcuts.

## Current Red Edge

```text
front:
  tools/smokes/v2/profiles/integration/joinir/
    phase29bq_hako_mirbuilder_quick_suite_vm.sh

known pinned shapes:
  cleanup_try_min
  cleanup_try_finally_local_min
  cleanup_try_reject_nonminimal
  cleanup_try_finally_local_var_mismatch_reject
  multi_local_accept_min
  multi_local_reject
```

The current repair lane is allowed to add one narrow MIR builder accepted
shape per commit, with an adjacent reject pin when the boundary would otherwise
be ambiguous.

## Required Boundary

```text
do:
  fix canonical Hako MIR builder acceptance/lowering
  keep valid Program(JSON) as the input contract
  add or keep fast pins for accepted and rejected minimal shapes
  fail fast with [freeze:contract][hako_mirbuilder] for unsupported shapes

do_not:
  reinterpret valid .hako syntax as invalid
  patch Stage-B only to bypass MIR builder acceptance
  add parser changes unless Program(JSON) is proven invalid
  add by-name route selection
  add silent fallback
  mix multiple unrelated accepted shapes in one commit
```

## BoxCount / BoxShape Selection

Use `BoxCount` when a valid Program(JSON) shape has no MIR builder owner yet.

Use `BoxShape` when a fallback owner is too broad and steals a more specific
shape, or when the owner order makes diagnostics thin.

Do not mix them in one commit.

## Current Acceptance

Minimum gates for the active red edge:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_min_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_reject_nonminimal_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_finally_local_var_mismatch_reject_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_multi_local_accept_min_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_multi_local_reject_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_quick_suite_vm.sh
```

Guard/support checks:

```bash
bash -n tools/hakorune_emit_mir.sh
bash tools/checks/program_json_v0_compat_caller_guard.sh
git diff --check
```

## Commit Rule

```text
1 accepted shape = fixture/gate pins = 1 commit
```

If the quick suite exposes a second independent red edge, stop after the first
commit and open the next 29bq card or update this card's "Current Red Edge"
before continuing.

