Status: ACTIVE
Phase: 271x

# 271x-90 Closure Thin-Entry Specialization Owner Seam SSOT

## Decision

- accepted

## Scope

- extend `closure_split_contract` with thin-entry eligibility classification
- keep lowering behavior unchanged in this cut
- reserve actual closure ABI specialization for a later phase if needed

## Owner

- `src/llvm_py/builders/closure_split_contract.py`

## Contract

- thin-entry specialization is exactly one of:
  - `thin_entry_candidate`
  - `public_entry_only`
- empty and single-capture envs are eligible
- aggregate envs are not eligible in this cut
- lowering may observe `thin_entry_eligible` and `thin_entry_subject`, but must not change ctor route here

## Out Of Scope

- changing closure ctor ABI
- closure call lowering changes
- IPO / cross-function cloning
