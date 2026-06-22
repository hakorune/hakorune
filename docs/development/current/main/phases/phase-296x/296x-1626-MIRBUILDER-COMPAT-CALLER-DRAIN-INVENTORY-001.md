# 296x-1626 MIRBUILDER-COMPAT-CALLER-DRAIN-INVENTORY-001

Status: landed
Date: 2026-06-22

## Purpose

Inventory the live callers that still use the legacy
`lang/src/compiler/mirbuilder/` Program(JSON) entries. This blocks a direct
physical retirement and defines the next redirect order.

## Scope

```text
BoxShape: one caller drain inventory
owner: MirBuilder compat caller drain
input: repo reference scan for legacy Program(JSON) entries
output: live caller classes before redirect implementation
```

## Observed Callers

```text
stage_a_bridge:
  src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs

active_joinir_smokes:
  tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*.sh
  tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh

docs_and_current_readmes:
  lang/src/compiler/mirbuilder/README.md
  docs/development/current/main/phases/phase-29bq/29bq-114-hako-cleanup-integration-prep-lane.md
  docs/development/current/main/phases/phase-29cv/P101-PROGRAM-JSON-V0-CAPSULE-CALLER-INVENTORY.md
```

## Redirect Order

```text
1. keep old entries live
2. add canonical compat entry under lang/src/mir/builder/compat/
3. repoint Stage-A bridge to the canonical compat entry
4. repoint active joinir smokes or their shared helper
5. turn old compiler-tree entries into thin forwarding wrappers
6. drain unique behavior family by family
```

## Acceptance

```text
live caller classes are recorded
physical deletion remains parked
next implementation owner is a redirect, not a semantic rewrite
```

## Stop Line

```text
do_not_delete_compiler_mirbuilder_yet=1
do_not_forward_before_canonical_compat_entry_exists=1
do_not_rewrite_semantics_during_caller_drain=1
```
