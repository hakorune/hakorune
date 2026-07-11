# 296x-1309 RUST-SUBSET-SYN-ADAPTER-SMOKE-ENTRY-001

Status: closed
Date: 2026-06-19

## Purpose

Make the host-side syn adapter handoff gate discoverable without changing
converter ownership.

## Implementation

Add a thin dedicated wrapper:

```text
apps/rust-subset-to-hako/smoke_adapter.sh
```

The wrapper only sets:

```text
RUST_SUBSET_RUN_ADAPTER=1
```

and delegates to the existing `smoke.sh`.

## Evidence

```bash
bash apps/rust-subset-to-hako/smoke_adapter.sh
```

Observed result:

```text
summary=ok
```

## Non-Goals

- No converter core change.
- No Rust parser inside Hakorune.
- No JSON parser bypass.
- No VM product-route validation.

## Next

The app-front is ready for the next source-shape selection or an explicit
external-adapter invocation UX row.

```text
next_blocker=RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001
```
