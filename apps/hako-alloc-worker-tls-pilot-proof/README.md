# hako-alloc worker/TLS pilot proof

Row: MIMAP-350A

This proof records a bounded worker/TLS fact from an accepted OSVM/page-source
report. It uses the existing internal `HakoAllocWorkerTlsCache` seam and keeps
source-level concurrency, worker scheduling, provider activation, host allocator
replacement, hooks, and backend matchers closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-350A --level L2
```
