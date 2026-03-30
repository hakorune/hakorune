# RuntimeData Dispatch Contract

Status: accepted  
Applies to: LLVM/AOT `RuntimeDataBox` method lowering

## Overview

`RuntimeDataBox` method calls in LLVM/AOT are routed to kernel exports:

- `nyash.runtime_data.get_hh(recv_h, key_any) -> i64`
- `nyash.runtime_data.set_hhh(recv_h, key_any, val_any) -> i64`
- `nyash.runtime_data.has_hh(recv_h, key_any) -> i64`
- `nyash.runtime_data.push_hh(recv_h, val_any) -> i64`

These symbols are implemented in `crates/nyash_kernel/src/plugin/runtime_data.rs`.

Array receiver が型確定している callsite では、array-specialized route
（AS-03）を使う場合がある:

- `push -> nyash.array.slot_append_hh(recv_h, val_any) -> i64`
- `get -> nyash.runtime_data.get_hh(recv_h, key_any) -> i64`
- `set -> nyash.runtime_data.set_hhh(recv_h, key_any, val_any) -> i64`
- `has -> nyash.runtime_data.has_hh(recv_h, key_any) -> i64`

さらに key が i64 と判定できる callsite では、
array-specialized route を整数キー版へ縮退できる（AS-03b）:

- `nyash.array.slot_load_hi(recv_h, idx_i64) -> i64`
- `nyash.array.slot_store_hih(recv_h, idx_i64, val_any) -> i64`
- `nyash.array.slot_store_hii(recv_h, idx_i64, val_i64) -> i64`（AS-03c）
- `nyash.runtime_data.has_hh(recv_h, idx_i64) -> i64`

`push` は integer-key 縮退を持たず、AS-03 でも `nyash.array.slot_append_hh`
を使う。`has` はこの wave では常に `nyash.runtime_data.has_hh` を使う。

これらは array/runtime_data plugin surface で実装され、
`runtime_data` の ArrayBox 契約と同一の戻り値意味を持つ。

## Receiver Dispatch

`recv_h` runtime type decides behavior:

- `ArrayBox`
  - `runtime_data.get_hh`: index read
  - `runtime_data.set_hhh`: set/append (`idx == len` append)
  - `runtime_data.has_hh`: bounds check (`0/1`)
  - `runtime_data.push_hh`: append
- `MapBox`
  - `runtime_data.get_hh`: key lookup (missing key returns `0`)
  - `runtime_data.set_hhh`: key set
  - `runtime_data.has_hh`: key existence check (`0/1`, missing key returns `0`)
  - `runtime_data.push_hh`: unsupported (`0`)
- other types: fail-fast return `0`

### Array index semantics (contract)

For `ArrayBox` receiver:

- negative index:
  - `runtime_data.get_hh` returns `0`
  - `runtime_data.set_hhh` returns `0`
  - `runtime_data.has_hh` returns `0`
- index in range:
  - `runtime_data.get_hh` returns element value (mixed i64/handle contract below)
  - `runtime_data.set_hhh` returns `1`
  - `runtime_data.has_hh` returns `1`
- index == len:
  - `runtime_data.set_hhh` appends and returns `1`
  - `runtime_data.has_hh` returns `0`
- index > len:
  - `runtime_data.set_hhh` returns `0`
  - `runtime_data.has_hh` returns `0`

Key decode contract for `runtime_data.get_hh/set_hhh/has_hh` on `ArrayBox`:

- `key_any <= 0`: treat as immediate index directly
- `key_any > 0`:
  - if `key_any` is a live handle to `IntegerBox`, use that integer value
  - if `key_any` is a live handle to `StringBox` and parseable as `i64`, use parsed value
  - otherwise, treat `key_any` itself as immediate index

This keeps positive immediate indices stable even when unrelated positive handles are live.

## Return Contract

`get_hh` return is intentionally mixed to match RuntimeData semantics:

- `IntegerBox` result: raw integer `i64`
- `BoolBox` result: raw `0/1`
- other boxed values (`StringBox`, `ArrayBox`, `MapBox`, etc.): host handle `i64`
- failure/not found/unsupported: `0`

`set_hhh` returns:
- `1` when mutation succeeds
- `0` on invalid receiver/invalid index/unsupported path

`has_hh` returns:
- `1` true
- `0` false or invalid receiver/path

`push_hh` returns:
- new array length for `ArrayBox`
- `0` otherwise

Related ABI note (legacy, non-runtime_data route):

- `nyash.array.set_h` and `nyash.map.set_{h,hh}` keep legacy completion code `0`
  (apply/no-op is not encoded in return code).

## Lowering Rule (SSOT)

For MIR method calls where `box_name == "RuntimeDataBox"` and method in `{get,push,set,has}`,
LLVM lowerers must use shared dispatch helper (`runtime_data_dispatch.py`) and follow:

- default: `nyash.runtime_data.*`
- AS-03 成立時（ArrayBox receiver + arity/key条件）:
  `push -> nyash.array.slot_append_hh`, `get/set/has -> nyash.runtime_data.*`
- AS-03b 成立時（AS-03 + key VID が i64/integerish）:
  `get -> slot_load_hi`, `set -> set_hih`, `has -> nyash.runtime_data.has_hh`
- AS-03c 成立時（AS-03b + value VID も i64/integerish）:
  `set -> set_hii`

Implemented in:
- `src/llvm_py/instructions/mir_call/method_call.py`
- `src/llvm_py/instructions/mir_call_legacy.py`
- shared helper: `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`

## E2E Contract Fixture

The LLVM/AOT route is pinned with a prebuilt MIR fixture that forces
`RuntimeDataBox` dispatch for `push/get/has/set`:

- fixture: `apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json`
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`

Smoke checks:
- IR contains the pinned active symbols for each method:
  - `push -> nyash.array.slot_append_hh`
  - `has -> nyash.runtime_data.has_hh`
  - `get -> nyash.runtime_data.get_hh` または `nyash.array.slot_load_hi`
  - `set -> nyash.runtime_data.set_hhh` または `nyash.array.slot_store_hih/slot_store_hii`
- compiled executable returns `rc=4`
