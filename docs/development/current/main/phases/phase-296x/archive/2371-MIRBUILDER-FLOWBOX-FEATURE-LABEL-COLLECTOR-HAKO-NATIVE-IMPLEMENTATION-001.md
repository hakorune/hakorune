# 2371 MIRBUILDER-FLOWBOX-FEATURE-LABEL-COLLECTOR-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for
`flowbox_feature_label_collector`.

## Boundary

The implementation accepts seven scalar flag tokens and returns a single CSV
string in Rust oracle order.

## Non-Claims

- No converter-generated native edit authority.
- No FlowBox facts analysis migration.
- No tag emission or MIR mutation migration.

## Next

`MIRBUILDER-FLOWBOX-FEATURE-LABEL-COLLECTOR-PARITY-GATE-001`
