# `ny-llvmc` CLI Contract

## Purpose

- `ny-llvmc` は `MIR(JSON) -> {object, executable}` を受け持つ backend CLI だよ。
- current implementation は `backend helper / compat wrapper` だけど、caller は内部実装ではなく CLI contract だけに依存する。
- backend-zero の final target は `native_driver.rs` そのものではなく、`.hako -> thin backend boundary` だよ。
- backend-zero `BE0-min1` では、この文書を stable caller contract の入口にする。

## Stable Caller Contract

次の flags は caller が依存してよい stable surface として固定する。

| flag | meaning | note |
| --- | --- | --- |
| `--in <FILE>` | MIR JSON input path | `-` で stdin を読む |
| `--out <FILE>` | output path | `--emit obj` では `.o`、`--emit exe` では実行ファイル |
| `--emit {obj|exe}` | emit kind | default は `obj` |
| `--dummy` | dummy `ny_main` を生成 | `--in` を無視する |
| `--nyrt <DIR>` | `libnyash_kernel.a` search dir | `--emit exe` でだけ意味を持つ |
| `--libs <FLAGS>` | linker extras | `--emit exe` でだけ意味を持つ |

## Implementation Detail Flag

次の flag は current harness route の実装補助であり、stable caller contract ではない。

| flag | current role | rule |
| --- | --- | --- |
| `--driver <DRIVER>` | object emission driver select | default は `boundary`。`harness` と `native` は opt-in keep lanes で、stable caller contract ではない |
| `--harness <FILE>` | Python harness path override | wrapper/debug 用。上位 caller はこれに結合しない |

## Fixed Semantics

1. `--emit obj`
   - `MIR(JSON)` から object file を生成する
2. `--emit exe`
   - object file を生成した後、既存 static-first link line で executable を生成する
3. `--dummy`
   - input を使わず `ny_main -> i32 0` の最小 object/executable を生成する
4. failure policy
   - invalid emit kind や link/runtime prerequisites missing は fail-fast する
   - silent fallback しない

## Current Implementation Note

- current `ny-llvmc` default route first enters the boundary-owned C ABI lane
- default boundary compile now tries the pure C subset first for supported seeds such as `apps/tests/mir_shape_guard/ret_const_min_v1.mir.json`, `apps/tests/hello_simple_llvm_native_probe_v1.mir.json`, `apps/tests/mir_shape_guard/string_length_ascii_min_v1.mir.json`, and `apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json`
- unsupported shapes now replay directly from `lang/c-abi/shims/hako_llvmc_ffi.c -> ny-llvmc --driver harness`, so `llvmlite` remains an explicit compat keep inside the boundary fallback lane
- `lang/c-abi/shims/hako_aot_shared_impl.inc` compile command now uses `--driver boundary`, so the default `hako_aot` command route matches the boundary-owned daily path
- `Boundary` / `Native` default routes do not resolve the Python harness path unless the explicit `--driver harness` keep lane is selected
- this is still implementation detail であり、backend-zero の final target は `.hako -> thin backend C ABI/plugin boundary` を daily route にすること
- `--driver native` は bootstrap seam 用の opt-in selector で、final owner ではない
- current internal default driver は `boundary` で、`native` defaulting はしない
- next migration step is boundary fallback reliance の縮小であり、`harness` と `native` は explicit replay lanes のまま keep する
- Rust backend lane をこの repo から retire するのはまだ先で、もし retire する場合も source + artifact を external archive repo に保存してからだけ行う
- current native subset (`BE0-min3` / `BE0-min4`):
  - entry function `main` or `ny_main`
  - `const(i64)` and `ret`
  - object emission via `llc`
  - executable emission via the existing static-first link line

## Minimal Examples

```bash
./target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/collapsed_min.mir.json \
  --out /tmp/collapsed_min.o
```

```bash
./target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/collapsed_min.mir.json \
  --emit exe \
  --nyrt target/release \
  --out /tmp/collapsed_min.exe
```

## Harness Replay (implementation detail / compat keep)

```bash
./target/release/ny-llvmc \
  --driver harness \
  --in apps/tests/mir_shape_guard/collapsed_min.mir.json \
  --out /tmp/collapsed_min.harness.o
```

## Native Driver Canary (implementation detail / temporary seam)

```bash
./target/release/ny-llvmc \
  --driver native \
  --in apps/tests/mir_shape_guard/collapsed_min.mir.json \
  --out /tmp/collapsed_min.native.o
```

```bash
./target/release/ny-llvmc \
  --driver native \
  --in apps/tests/mir_shape_guard/collapsed_min.mir.json \
  --emit exe \
  --nyrt target/release \
  --out /tmp/collapsed_min.native.exe
```
