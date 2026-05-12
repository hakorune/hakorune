# mimalloc-secure-list-encode-decode-proof

Status: Active
Scope: M184 secure-list encode/decode small path.

This proof app validates `HakoAllocSecureFreeListPolicy` as a standalone
encoded-next policy owner. It proves reversible encode/decode, end-of-list
sentinel handling, cookie mismatch rejection, and capacity validation.

Non-goals:

- no page mutation
- no diagnostics counter ownership
- no entropy source
- no native allocator replacement

Run:

```bash
bash tools/checks/k2_wide_mimalloc_secure_list_policy_guard.sh
```
