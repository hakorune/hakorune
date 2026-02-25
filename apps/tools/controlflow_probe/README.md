# Controlflow Probe

A minimal CLI tool to probe JoinIR/VM control flow handling.

## Purpose

- Tests conditional loop variable update (early exit)
- Tests `continue` in loop
- Tests nested if value carry
- Detects JoinIR/VM instabilities early in app-first development

## Usage

```bash
./target/release/hakorune apps/tools/controlflow_probe/main.hako
```

## Output Format

Single line only:

```
SUMMARY acc=8 continue=1 steps=6 nested=12 exit_i=100
```

## What it Tests

| Feature | Description |
|---------|-------------|
| Early exit | `if i == 5 { i = 100 }` in loop |
| Continue | `continue` at i==2 |
| Nested if | Value carry through nested conditions |

## Contract

- Output is exactly 1 line
- No FileBox, no env.get
- Pure static box Main implementation

## Related

- Smoke test: `tools/smokes/v2/profiles/integration/apps/controlflow_probe_vm.sh`
- Expected: `apps/tests/controlflow_probe/expected_summary.txt`
