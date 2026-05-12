# mimalloc-result-contract-proof

Status: Active
Scope: M190 nullable / failure handle contract.

This proof app exercises the explicit `HakoAllocHandleResult` surface:

- `allocateResult(size) -> HakoAllocHandleResult`
- `reallocResult(handle, size) -> HakoAllocHandleResult`

The result shape is `ok/reason/handle`. Successful results carry a
`HakoAllocHandle`; failures carry `ok=0`, a stable numeric reason, and a null
handle. Existing `allocate/realloc` object-return APIs remain unchanged for
M189 compatibility.

Reason codes in this row:

- `0`: ok
- `1`: null handle
- `2`: invalid size
- `3`: invalid/stale handle
- `4`: allocation failed / unsupported size

Run:

```bash
bash tools/checks/k2_wide_mimalloc_result_contract_guard.sh
```
