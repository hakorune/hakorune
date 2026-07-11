# P381GY APP-1 Archive Delete Wave

Date: 2026-05-06
Scope: delete the zero-ref archived APP-1 vm_hako smokes after ownership moved
to the integration app smoke.

## Context

After P381GX, the next safe per-script delete-last slice in
`tools/smokes/v2/profiles/archive` was the archived `vm_hako_caps/app1/` family.

The refreshed inventory showed both scripts as:

- `fullpath_ref_count = 0`
- `basename_ref_count = 0`
- `suite_hit_count = 0`
- `class = orphan_candidate`

The live owner has already moved:

- `tools/smokes/v2/profiles/integration/vm_hako_caps/README.md` says `app1/` is
  no longer suite-owned
- that README points the product owner for APP-1 summary behavior to
  `tools/smokes/v2/profiles/integration/apps/gate_log_summarizer_vm.sh`
- `phase-96x` docs record the APP-1 vm_hako rows as retired from active
  ownership after the `presubmit.txt` move

## Deleted Paths

- `tools/smokes/v2/profiles/archive/vm_hako_caps/app1/app1_stack_overflow_after_open_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/app1/app1_summary_contract_ported_vm.sh`

## Result

This is a narrow smoke delete-last cleanup slice:

- no compiler behavior changed
- no suite-protected archive bucket was touched
- no manual archive bucket was touched
- no active vm_hako or app smoke owner was removed

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=post_p381gy_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/app1/app1_stack_overflow_after_open_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/app1/app1_summary_contract_ported_vm.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
