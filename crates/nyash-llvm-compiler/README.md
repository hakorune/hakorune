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
| `--driver <DRIVER>` | object emission driver select | default は `harness`。`native` は opt-in seam で、stable caller contract ではない |
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

- current `ny-llvmc` は compat keep として `python3` と `tools/llvmlite_harness.py` に依存している
- これは implementation detail であり、backend-zero の final target は `.hako -> thin backend C ABI/plugin boundary` を daily route にすること
- `--driver native` は bootstrap seam 用の opt-in selector で、final owner ではない
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
