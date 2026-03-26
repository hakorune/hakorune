# PyVM Usage Guidelines (Historical)

Status: Historical / direct-only

## Summary

- Daily development and gate decisions use Rust VM + LLVM only.
- Runtime route toggles are removed from active execution paths.
- PyVM remains only for historical parity and diagnostics.

## Current policy

- Use Rust VM for normal runs:
  - `./target/release/hakorune --backend vm program.hako`
- Use the LLVM llvmlite keep lane for AOT parity:
  - `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm program.hako`
- Use PyVM only through historical direct tools:
  - `bash tools/historical/pyvm/pyvm_stage2_smoke.sh`
  - `bash tools/historical/pyvm/pyvm_vs_llvmlite.sh apps/tests/ternary_basic.hako`
  - `python3 tools/historical/pyvm/pyvm_runner.py --in /path/to/mir.json`

## Non-goals

- Do not use PyVM for daily program execution.
- Do not use PyVM for feature correctness sign-off.
- Do not add new runtime branching based on PyVM toggles.

## Notes

- Environment variable status is tracked in `docs/reference/environment-variables.md`.
- Retreat policy SSOT is `docs/development/current/main/design/pyvm-retreat-ssot.md`.
