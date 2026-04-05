# Phase 153x: ny_mir_builder harness drop

- Status: Next
- 目的: `ny_mir_builder --emit obj|exe` が daily mainline では harness keep を強制しないようにする。
- 対象:
  - `src/bin/ny_mir_builder.rs`
  - `tools/ny_mir_builder.sh`
  - related smoke/tool callers
