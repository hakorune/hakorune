# 3442 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001
```

## Purpose

Rerun the 3441 MapLoad-only caller-orientation contract authority pilot after
its first green implementation. This card validates freshness and preserves
the narrow authority boundary before any next proof-axis decision.

## Required Evidence

```text
single row = map_load_scalar_i64_routes
consumer input = PolicyRowIdOnly
consumer return = Unit
Rust oracle mismatch = fail-fast
runtime/backend/mutation/publication = forbidden
```

The rerun must prove no String, Collection, Write, Delete, ScalarKnown-wide,
or Source Selfhost authority entered the pilot.

## Stop Boundary

Do not promote from this rerun directly to ScalarKnown-wide, Delete, runtime,
backend, or Source Selfhost. Select the next proof axis in a dedicated design
decision card.
