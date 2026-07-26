---
Status: active closeout row
Date: 2026-07-26
Decision: NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-prime-r1
Row: NORMAL-CALLABLE-MODULE0-TX0-G0
Parent: NORMAL-CALLABLE-MODULE0-TX0-COMMIT0-S0
Scope: verify the completed disconnected normal callable transaction boundary
ceremony_tier: T0 mechanical closeout
---

# NORMAL-CALLABLE-MODULE0-TX0-G0

## Required evidence

```text
helper prefix -> Main -> physical -> schema -> candidate = one chain
candidate has helper/Main/physical exact membership
all lower/schema/precommit failures retain prior evidence
same Builder reuse is green
live publication / runner / backend caller = 0
existing transaction guard = green
all touched sources < 800 lines
```

No new owner, source shape, route, or publication is permitted in this row.
