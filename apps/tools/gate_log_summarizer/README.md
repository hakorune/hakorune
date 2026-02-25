# Gate Log Summarizer

A minimal CLI tool to summarize gate test logs.

## Purpose

- Proves "app-first development" using only `.hako` code
- No Rust modifications required
- Demonstrates FileBox usage for real-world text processing

## Usage

```bash
./target/release/hakorune apps/tools/gate_log_summarizer/main.hako <logfile>
```

## Output Format

```
SUMMARY pass=<n> fail=<n> skip=<n>
FAIL_LINES <m>
[FAIL] ... (m lines in input order)
```

## Example

Input:
```
[PASS] test1: ok
[FAIL] test2: error
[SKIP] test3: skipped
```

Output:
```
SUMMARY pass=1 fail=1 skip=1
FAIL_LINES 1
[FAIL] test2: error
```

## Exit Codes

- `0`: Success
- `1`: Invalid input (file not found, missing args)

## Contract

- Output format is fixed (do not add extra lines)
- FAIL lines appear in input order (stable ordering)
- Only `[PASS]`, `[FAIL]`, `[SKIP]` prefixes are recognized

## Related

- Instructions: `docs/development/current/main/phases/phase-29y/70-APP-FIRST-OUTSOURCE-INSTRUCTIONS.md`
- Test fixture: `apps/tests/gate_log_summarizer/sample_mixed.log`
- Smoke test: `tools/smokes/v2/profiles/integration/apps/gate_log_summarizer_vm.sh`
