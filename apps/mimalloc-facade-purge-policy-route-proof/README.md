# mimalloc-facade-purge-policy-route-proof

Purpose: MIMAP-019A proof for the object-lifecycle facade purge/reclaim policy
route.

The proof creates facade-shaped page lifecycle views, constructs a
`HakoAllocObjectLifecycleFacadeStatsSnapshot`, then classifies known page
lifecycle state through the MIMAP-019A route. The route delegates to the
existing M211 purge candidate inventory and M213 abandoned reclaim inventory,
but does not execute decommit, reclaim, page-source, OSVM, provider, or backend
behavior.

Guard:

```text
bash tools/checks/k2_wide_mimalloc_facade_purge_policy_route_exe_guard.sh
```
