# hako-alloc-usize-field-probe

Purpose
- Proves that non-negative hako_alloc field shapes can be represented as
  exact `usize` stored fields in an isolated probe.
- Keeps production allocator state on `i64`.

Stop line
- No production facade migration.
- No native allocator replacement.
- No OSVM, TLS, atomics, remote-free, or page-map behavior.

Run

```bash
bash apps/hako-alloc-usize-field-probe/test.sh
```
