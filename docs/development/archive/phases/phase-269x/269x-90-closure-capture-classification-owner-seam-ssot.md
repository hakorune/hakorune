Status: LANDED
Phase: 269x

# 269x-90 Closure Capture Classification Owner Seam SSOT

## Decision

- accepted

## Scope

- add one shared owner seam for closure capture classification
- keep runtime behavior unchanged
- keep env scalarization and thin-entry specialization out of scope

## Owner

- `src/llvm_py/builders/closure_split_contract.py`

## Contract

- closure creation is classified into exactly one env shape:
  - `empty_env`
  - `capture_env_only`
  - `me_only_env`
  - `capture_env_with_me`
- lowering reads the shared contract to choose the ctor route
- modern and legacy closure lowering must not drift on env-shape choice

## Out Of Scope

- closure env scalarization
- closure thin-entry specialization
- capture aliasing or escape widening
