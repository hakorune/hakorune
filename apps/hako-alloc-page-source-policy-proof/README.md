# hako-alloc-page-source-policy-proof

Purpose: M49 proof that OSVM page-source policy is reachable through
`HakoAllocProductionFacade`.

The app uses the production-facing facade for page-source operations:

- `pageSourceReserve`
- `pageSourceCommit`
- `pageSourceDecommit`

`HakoAllocProductionFacade` delegates to `HakoAllocPageSourcePolicy`, and the
policy uses the existing `OsVmCoreBox` substrate rows.

It intentionally avoids:

- OS VM unreserve/release rows
- backend allocator replacement hooks
- native pointer attrs
- pointer `fetch_add`
- new `.inc` app/facade matchers

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
```
