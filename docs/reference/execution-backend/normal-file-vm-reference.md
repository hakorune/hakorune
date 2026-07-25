---
Status: reference contract
Decision: accepted
Date: 2026-07-26
---

# Normal-file VM-reference lane

`normal-file-vm-reference` is an explicit, feature-gated normal-file semantic
reference lane. It is default-off and does not replace `--backend mir`,
`compile_with_source`, or any existing normal caller.

## Invocation

```bash
cargo build --release --features vm-reference
target/release/hakorune --backend normal-file-vm-reference program.hako
```

Without `vm-reference`, selection reports usage status `2` before source I/O.
The lane also rejects `--no-optimize` before source I/O: its profile is fixed
to `CanonicalDefaultOptimizedV1`.

## Fixed profile

```text
grammar       = Canonical
source        = one UTF-8 file, read once and parsed once
imports/using = rejected
macros/plugins/REPL/JSON/script arguments = rejected
result        = Unit / Integer / Bool / Float / String
execution     = fresh VM reference instance
process       = CanonicalProcessExitV1
fallback      = forbidden
```

The normal front door consumes its one sealed request, then reuses the Raw
publication and exact-target VM owners. It does not invoke the Raw CLI front
door, reparse the source, search for an entry, or reconstruct process status.

Usage/profile errors return `2`; read, parse, source-profile, compile, and
activation failures return `1`. Executed program results keep the status
sealed by the canonical process projection: Unit is `0`, an in-range Integer
is exact, and unsupported results or program faults are `70`.

## Scope

This is a production semantic-reference lane, not automatic promotion. A
future `NORMAL-ENTRY-PROMOTION-D3` decision must select an exact old caller,
product backend, corpus, performance budget, and retirement rule before any
default-route replacement.
