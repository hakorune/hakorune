# 293x-301 ASTCLEAN-008 test/dev helper dead_code allow prune

Status: complete

## Decision

Decision: accepted.

Test/dev helper allowances should not stay anonymous. This row removes an obsolete VM benchmark stub and requires explicit rationale comments for retained test/dev `#[allow(dead_code)]` attributes.

## Scope

- Delete the private `run_vm_benchmark` stub that only returned the removed VM legacy error.
- Add `ASTCLEAN-008` reason comments to retained test/dev helper allowances.
- Keep tests, demos, and JoinIR helper behavior unchanged.

## Non-goals

- No benchmark runner redesign.
- No demo surface change beyond deleting the private unused VM stub.
- No JoinIR test helper behavior change.

## Guard

- `tools/checks/k2_wide_astclean_test_dev_dead_code_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_test_dev_dead_code_guard.sh` passed locally.
