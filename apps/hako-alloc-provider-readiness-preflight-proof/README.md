# hako-alloc-provider-readiness-preflight-proof

Row: MIMAP-362A
Owner: `HakoAllocProviderReadinessPreflight`
Profile: `scalar-mir`

This proof preflights provider readiness from provider boundary diagnostic
vocabulary while keeping provider activation and host-facing replacement/hook
behavior closed.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-362A --level L2
```
