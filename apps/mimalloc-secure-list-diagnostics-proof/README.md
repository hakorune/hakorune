# mimalloc-secure-list-diagnostics-proof

M183 proof app. It freezes secure free-list diagnostics without adding
encode/decode or hardening policy.

The proof keeps five observers explicit:

1. out-of-range free-list blocks;
2. duplicate blocks across `free` and `local_free`;
3. live blocks appearing in a free-list;
4. free-count mismatch;
5. local-free top bounds mismatch.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_secure_list_diagnostics_guard.sh
```
