Status: ACTIVE
Phase: 272x

# 272x-90 IPO Build-Policy Owner Seam SSOT

## Decision

- accepted

## Scope

- create one owner seam for LLVM/Python build-policy decisions
- keep current build behavior unchanged
- postpone actual `ThinLTO` and `PGO` enabling

## Owner Candidates

- `src/llvm_py/build_opts.py`
- `src/llvm_py/llvm_builder.py`

## Contract

- build-policy ownership must be centralized before enabling any IPO widening
- the seam must be able to represent:
  - `lto_mode = off | thin`
  - `pgo_mode = off | generate | use`
  - future profile-path ownership
- current phase is docs/policy only; behavior remains `off`

## Out Of Scope

- enabling `ThinLTO`
- generating or consuming PGO profiles
- link-driver changes outside the owner seam
