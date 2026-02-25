# PyVM legacy tools (historical / opt-in)

PyVM is not part of the daily route. This directory keeps legacy tooling for
diagnostics and parity checks only.

## Canonical runner

- `tools/historical/pyvm/pyvm_runner.py`
- `tools/historical/pyvm/pyvm_*.sh` (legacy smoke/parity helpers)

## Compatibility

- Top-level wrappers (`tools/pyvm_runner.py`, `tools/pyvm_*.sh`) are removed.
- Scripts and docs must reference `tools/historical/pyvm/pyvm_runner.py` and `tools/historical/pyvm/pyvm_*.sh`.

## Opt-in policy

- Use only via direct historical tools (`pyvm_runner.py` / `pyvm_*.sh` in this directory).
- Do not add this route to daily/quick gate profiles.
