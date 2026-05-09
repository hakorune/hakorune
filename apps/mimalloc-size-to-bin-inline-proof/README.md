# mimalloc-size-to-bin-inline-proof

M24 proof fixture for the mimalloc substrate ladder.

This app proves that an allocator fast-path helper can remain idiomatic source
code while still reaching pure-first EXE as expanded MIR:

```hako
@rune Profile(allocator.fast)
size_to_bin(size) { ... }
```

The helper computes a narrow two-class bin, MIR verifies the required inline
plan, the optimizer consumes the helper call before backend lowering, and the
result feeds `MI_SIZE_CLASS[bin]` / `MI_CLASS_CAP[bin]`.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh
```

Expected tail:

```text
summary=ok
```
