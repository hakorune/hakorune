# mimalloc-huge-threshold-routing-proof

M179 proof app. It freezes huge threshold/routing without implementing a huge
page model.

The proof keeps four boundaries explicit:

1. the last regular size-class block size is the huge threshold;
2. padded requests above the threshold route to the huge lane;
3. huge requests fail fast while M180 is still absent;
4. small aligned requests still execute through the M178 aligned small-path
   owner.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
```
