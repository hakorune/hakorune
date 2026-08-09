# LOOP-V2-DYNAMIC-LOCAL-SLOT-INDEX-P2

Status: parked polish; not a JoinSig blocker
Date: 2026-08-10

At the existing Dynamic local co-seal, retain private indices for the exact
binding, declaration, and read rows so `borrow()` no longer searches verified
roles. The indices are cache-only and are validated at seal time.

Do not create `VerifiedChLocalV1`, expose indices, infer Home, change Recipe
keys, or make V10 a carrier. Acceptance requires equality-identical borrowed
views and seal-time wrong-role/site/slot rejection without a test-only semantic
constructor. Raw private `usize` indices are cache-only. File-size compliance
and same-slice owner README/reference receipt updates are mandatory.
