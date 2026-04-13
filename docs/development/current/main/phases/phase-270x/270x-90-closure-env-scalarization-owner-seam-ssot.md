Status: LANDED
Phase: 270x

# 270x-90 Closure Env Scalarization Owner Seam SSOT

## Decision

- accepted

## Scope

- extend `closure_split_contract` with env scalarization classification
- keep lowering behavior unchanged in this cut
- reserve actual env materialization changes for a later phase

## Owner

- `src/llvm_py/builders/closure_split_contract.py`

## Contract

- env scalarization classification is exactly one of:
  - `scalar_none`
  - `scalar_single`
  - `aggregate_multi`
- `scalar_single` exposes `scalarizable_capture_id`
- lowering may observe `env_scalarizable`, but must not change ctor route in this cut

## Out Of Scope

- replacing `nyash.closure.new_with_captures` for scalar-single envs
- closure thin-entry specialization
- closure call-site ABI specialization
