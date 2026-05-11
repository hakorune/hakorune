# mimalloc-size-class-policy-proof

M163 proof fixture for the `.hako` mimalloc port.

This app proves the first production-side size-class policy owner:

- `SizeClassBox.size_to_bin(size)`
- `SizeClassBox.bin_size(bin)`
- `SizeClassBox.good_size(size)`
- the existing `LayoutBox` small/medium compatibility facade

It intentionally avoids allocator pages, free lists, RawBuf/RawArray, OSVM,
TLS, atomics, provider activation, hooks, and process allocator replacement.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_size_class_policy_guard.sh
```

Expected tail:

```text
summary=ok
```
