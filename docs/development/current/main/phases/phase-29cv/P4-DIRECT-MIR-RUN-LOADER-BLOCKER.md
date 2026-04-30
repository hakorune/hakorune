# P4: direct MIR run loader blocker

Scope: record why `selfhost_build.sh --run` remains on the Program(JSON v0)
keeper route after P3.

Status note: superseded by P6. The `newbox` / `field_get` loader blocker is
solved for normal `--run`; diagnostic artifact routes still stay on the
Program(JSON v0) keeper.

## Probe

The direct source MIR path can emit MIR(JSON), and normal `--exe` can compile
that MIR through ny-llvmc. The execution loader is still narrower.

Minimal fixture:

```hako
static box Main { method main(args) { return 7 } }
```

Probe shape:

```bash
target/release/hakorune --backend mir --emit-mir-json /tmp/run.mir.json /tmp/run.hako
target/release/hakorune --mir-json-file /tmp/run.mir.json
```

Observed result:

```text
emit_rc=0 run_rc=1
MIR JSON parse error:
  v1: unsupported instruction 'newbox' in function 'main' (Gate-C v1 bridge)
  v0: unsupported op 'field_get' in mir_json_v0 loader
```

The same result appears for the quick binop fixture:

```hako
static box Main { method main(args) { local a=1; local b=2; local c=3; return a + b * c } }
```

## Decision

P4 kept `selfhost_build.sh --run` on the explicit Program(JSON v0) keeper route
owned by `tools/selfhost/lib/selfhost_build_run.sh`.

P6 later proved the MIR JSON execution intake accepts the current direct source
MIR dialect (`newbox` plus the entry-support `field_get` shape at minimum) and
moved normal `--run` to direct MIR execution.

## Next

The remaining cleanup work should avoid treating diagnostic artifact routes as
delete-ready. Prefer:

- Stage-B / smoke helper deduplication
- Stage1 contract keep narrowing
- JoinIR/MirBuilder fixture archive/owner review
- direct MIR execution loader proof as its own BoxCount/acceptance lane
