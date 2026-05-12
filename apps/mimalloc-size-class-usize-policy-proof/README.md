# mimalloc-size-class-usize-policy-proof

Status: Active
Scope: M187 exact `usize` size-class policy facade.

This proof app validates the `SizeClassBox` `usize` input facade without
changing the legacy `i64` size-class API. Sentinel-returning outputs remain
signed, so oversized requests still report `-1` through `good_size_usize(...)`.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_size_class_usize_policy_guard.sh
```
