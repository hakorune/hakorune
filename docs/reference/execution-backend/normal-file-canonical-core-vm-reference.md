---
Status: reference contract
Decision: accepted
Date: 2026-07-26
---

# Normal-file canonical-core VM reference

`normal-file-canonical-core-vm-reference` is a default-off, feature-gated
semantic reference lane for the canonical-core normal-file profile. It is not
the product default backend and does not replace `mir`, `raw-vm-reference`, or
`normal-file-vm-reference`.

Build and run it explicitly:

```bash
cargo build --release --features vm-reference
target/release/hakorune \
  --backend normal-file-canonical-core-vm-reference \
  program.hako
```

The route reads one UTF-8 file, parses once with the Canonical grammar, seals
one source plan, and uses the canonical source-entry publication and exact
VM-reference execution owners. It accepts only the currently sealed
canonical-core slice. Imports, `using`, plugins, macros, script arguments,
non-default optimization, and unsupported source families reject without a
fallback route.

Exit-status classes are fixed:

- Usage or unavailable feature: `2`.
- File, parse, source-plan, compile, or activation rejection: `1`.
- An executed program Fault or unsupported process result: `70`.
- Unit and accepted integer source results: canonical status `0..=255`.

This lane is evidence for later promotion only. It has no automatic default
cutover; `NORMAL-ENTRY-PROMOTION-D3` remains the owner of any future product
backend or legacy-caller replacement decision.
