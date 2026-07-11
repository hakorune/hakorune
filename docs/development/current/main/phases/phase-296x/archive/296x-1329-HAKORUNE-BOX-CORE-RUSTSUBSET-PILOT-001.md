# 296x-1329 HAKORUNE-BOX-CORE-RUSTSUBSET-PILOT-001

Status: closed
Date: 2026-06-20

## Purpose

Run the selected `hakorune_box_core` RustSubset crate pilot far enough to
separate supported crate handoff from the next generated-function acceptance
blocker.

This row keeps the crate graph and Rust parser in the external adapter. It
does not add `.hako` syntax, Rust name resolution, `use` resolution, or trait /
generic semantics.

## Implementation

Checked-in adapter bundle:

```text
apps/rust-subset-to-hako/examples/hakorune_box_core_expected/
  crate-manifest.json
  modules/0000.json
  modules/0001.json
  modules/0002.json
```

Generated skeleton fixtures:

```text
apps/rust-subset-to-hako/examples/hakorune_box_core_expected.hako
apps/rust-subset-to-hako/examples/hakorune_box_core_leaf_expected.hako
```

Focused wrapper:

```text
apps/rust-subset-to-hako/convert_hakorune_box_core_crate_file.hako
```

The wrapper mirrors the existing synthetic crate handoff route with fixed
pilot paths. It is accepted at MIR emit level only; EXE execution is not added
to `smoke.sh` in this row because the existing synthetic crate wrapper also
currently hits `unsupported pure shape` when compiled as an executable in this
workspace.

## Findings

The pilot exposed two converter skeleton validity gaps:

```text
empty Rust struct -> empty .hako record
impl receiver param -> function Type_method(me: Type)
```

Fixes:

```text
empty struct skeleton -> TODO comment
self expression -> receiver
impl receiver param -> receiver: Target
```

The full generated skeleton now parses, but root-module MIR emit still stops
at generated top-level function acceptance:

```text
function BoxCoreBoundary_name(receiver: BoxCoreBoundary): String { ... }
```

Leaf modules (`crate::plugin`, `crate::policy`) are enum-comment-only and emit
MIR successfully. This makes the next blocker precise instead of broad.

## Acceptance

```bash
tmp=$(mktemp -d /tmp/hakorune_box_core_pilot.XXXXXX)
cargo run -q --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml -- \
  --crate-root crates/hakorune_box_core --out-dir "$tmp" \
  --crate-name hakorune_box_core --target-kind lib --target-name hakorune_box_core
diff -u apps/rust-subset-to-hako/examples/hakorune_box_core_expected/crate-manifest.json "$tmp/crate-manifest.json"
diff -u apps/rust-subset-to-hako/examples/hakorune_box_core_expected/modules/0000.json "$tmp/modules/0000.json"
diff -u apps/rust-subset-to-hako/examples/hakorune_box_core_expected/modules/0001.json "$tmp/modules/0001.json"
diff -u apps/rust-subset-to-hako/examples/hakorune_box_core_expected/modules/0002.json "$tmp/modules/0002.json"

python3 apps/rust-subset-to-hako/tools/crate_inventory.py \
  --manifest apps/rust-subset-to-hako/examples/hakorune_box_core_expected/crate-manifest.json

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/rust_subset_hakorune_box_core_crate_file.mir.json \
  apps/rust-subset-to-hako/convert_hakorune_box_core_crate_file.hako

./target/release/hakorune --dump-ast \
  apps/rust-subset-to-hako/examples/hakorune_box_core_expected.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_box_core_leaf_expected.mir.json \
  apps/rust-subset-to-hako/examples/hakorune_box_core_leaf_expected.hako

python3 apps/rust-subset-to-hako/selftest.py
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
rust_parser_owned_by_hako=0
crate_graph_discovery_owned_by_hako=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
new_hako_syntax_added=0
generated_full_root_mir_emit_claim=0
generated_program_exe_aot_claim=0
```

## Next

Continue:

```text
RUST-SUBSET-GENERATED-FUNCTION-MIR-ACCEPTANCE-001
```

Accept or deliberately reframe generated top-level function skeletons so the
root module can progress from parse-only to MIR emit without requiring Rust
name resolution or full executable semantics.
