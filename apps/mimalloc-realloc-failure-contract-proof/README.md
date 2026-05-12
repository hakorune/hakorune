# mimalloc-realloc-failure-contract-proof

M176 proof app. It freezes the realloc negative matrix without widening M174 or
M175.

The proof keeps seven outcomes explicit:

1. same-class success stays on M174 and returns the original ptr;
2. grow success stays on M175 and returns a replacement ptr after release order
   is satisfied;
3. zero-size requests reject before page-map lookup;
4. oversized requests reject before the grow fallback scans for replacement;
5. allocation failure stays distinct from oversized failure;
6. released, stale, and unknown ptr rejects keep their own failure kinds;
7. unexpected reject kinds stay at zero.

Failure kinds:

- `1`: zero request
- `2`: oversized request
- `3`: unknown ptr
- `4`: stale page
- `5`: released block
- `6`: allocation failure
- `7`: unexpected/internal mismatch

Run:

```bash
bash tools/checks/k2_wide_mimalloc_realloc_failure_contract_guard.sh
```
