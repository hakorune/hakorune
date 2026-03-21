# JSON Smoke Family

This directory hosts the `json_*` family split out of `integration/apps/`.
It keeps the JSON smoke pins together so the remaining `apps` bucket can
continue shrinking by semantic domain.

Contained scripts:

- `json_lint_vm_llvm.sh`
- `json_pp_vm_llvm.sh`
- `json_query_vm_llvm.sh`

Contract:

- keep these pins under the `integration` profile only
- keep them live and executable by `run.sh`
- keep the family small enough to remain an evidence pin bucket
