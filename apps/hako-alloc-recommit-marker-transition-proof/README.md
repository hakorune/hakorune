# hako-alloc-recommit-marker-transition-proof

Status: M204 proof app.

This proof closes the marker transition after a successful M202/M203 recommit.

It proves:

- decommitted pages remain blocked before recommit marker transition
- successful recommit reports can mark one recommit transition
- duplicate recommit transitions are rejected
- the M200 precondition sees the page as reusable after transition
- a later decommit mark can open a new decommit generation without removing
  old marker entries

Run:

```bash
bash apps/hako-alloc-recommit-marker-transition-proof/test.sh
```
