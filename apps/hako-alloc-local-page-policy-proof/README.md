# hako-alloc-local-page-policy-proof

Purpose: M47 proof that local page allocate/free policy is reachable through
`HakoAllocProductionFacade`.

The app uses only the production-facing facade. It validates:

- small and medium allocation success
- oversize rejection
- release success
- double-free rejection
- local reuse through final free-count/accounting shape

It intentionally avoids:

- remote-free policy
- OS VM page-source ownership
- backend allocator replacement hooks
- new MIR route rows
- new NyRT exports
- direct `.inc` app/facade matchers
- pointer `fetch_add`
- native pointer attrs

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh
```
