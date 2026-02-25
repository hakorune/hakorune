# MIR Shape Guard

A minimal `.hako` utility that inspects emitted MIR(JSON) and detects collapsed shape regressions.

## Purpose

- App-first guard for `--hako-emit-mir-json` / selfhost emit route
- Detects accidental collapse to single-block `const+ret`
- Keeps expression-shape debugging short and reproducible

## Usage

```bash
MIR_SHAPE_INPUT=/tmp/out.mir.json \
MIR_SHAPE_STRICT=1 \
./target/release/hakorune apps/tools/mir_shape_guard/main.hako
```

Profile mode:

```bash
MIR_SHAPE_INPUT=/tmp/out.mir.json \
MIR_SHAPE_STRICT=1 \
MIR_SHAPE_PROFILE=1 \
./target/release/hakorune apps/tools/mir_shape_guard/main.hako
```

## Output Contract

Single-line output only:

```text
SUMMARY blocks=<n> branch=<n> compare=<n> phi=<n> ret=<n> collapsed=<0|1>
```

When `MIR_SHAPE_PROFILE=1`, one extra line is emitted:

```text
PROFILE read_ms=<n> scan_ms=<n> total_ms=<n>
```

## Exit Codes

- `0`: success (`collapsed=0` or strict off)
- `1`: invalid input (missing env/file)
- `2`: collapsed shape detected with strict mode enabled

## Related

- fixture: `apps/tests/mir_shape_guard/collapsed_min.mir.json`
- smoke: `tools/smokes/v2/profiles/integration/apps/mir_shape_guard_vm.sh`
