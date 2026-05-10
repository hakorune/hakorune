# hako-alloc-remote-free-policy-proof

Purpose: M48 proof that the M43 remote-free retry-loop policy is reachable
through `HakoAllocProductionFacade`.

The app uses the production-facing facade for remote-free operations:

- `remoteInitHead`
- `remotePushRetry`
- `remotePeekHead`
- `remotePeekNext`

Fixture setup still allocates native pointer cells with the existing
`hako_mem_alloc/free` substrate route. The allocator policy/control seam remains
in `hako_alloc`: `HakoAllocProductionFacade` delegates to
`HakoAllocRemoteFreePolicy`, while pointer atomics remain in substrate routes.

It intentionally avoids:

- pointer `fetch_add`
- native pointer attrs
- OS VM page-source ownership
- backend allocator replacement hooks
- new `.inc` app/facade matchers

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh
```
