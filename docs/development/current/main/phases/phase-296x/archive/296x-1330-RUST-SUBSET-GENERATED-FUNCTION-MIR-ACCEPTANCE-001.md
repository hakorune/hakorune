# 296x-1330 RUST-SUBSET-GENERATED-FUNCTION-MIR-ACCEPTANCE-001

Status: closed
Date: 2026-06-20

## Purpose

Accept generated top-level function skeletons at MIR emit level so the
`hakorune_box_core` root module can progress beyond parse-only acceptance.

This row does not add Rust name resolution, `use` resolution, cross-module
linking, or generated-program execution claims.

## Problem

The `hakorune_box_core` root module generated:

```hako
function BoxCoreBoundary_name(receiver: BoxCoreBoundary): String {
    return "hakorune-box-core"
}
```

The parser accepted this surface, but MIR lowering treated a top-level
`FunctionDeclaration` as an executable statement and failed with:

```text
Unsupported AST node type: FunctionDeclaration
```

## Change

Program lowering now treats top-level `FunctionDeclaration` as a declaration:

```text
Program first pass:
  lower top-level FunctionDeclaration as standalone MIR function
  function signature name = <name>/<arity>

Program entry lowering:
  remove top-level FunctionDeclaration from runtime statements
  execute remaining script statements normally
```

This uses the existing `lower_static_method_as_function` path. It does not
create a new function-lowering owner.

## Converter Contract Cleanup

The pilot also fixed two generated skeleton syntax issues:

```text
impl receiver parameter:
  old: me: Type
  new: receiver: Type

empty Rust struct:
  old: empty record
  new: TODO comment
```

These keep generated skeletons parser-safe without claiming executable Rust
semantics.

## Acceptance

```bash
cargo build -q --release --bin hakorune

cat > /tmp/hako_top_function_min.hako <<'EOF'
function hello(receiver: Any): String {
    return "hello"
}
EOF
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hako_top_function_min.mir.json \
  /tmp/hako_top_function_min.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_box_core_expected.mir.json \
  apps/rust-subset-to-hako/examples/hakorune_box_core_expected.hako

python3 apps/rust-subset-to-hako/selftest.py
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
rust_name_resolution_enabled=0
use_resolution_enabled=0
cross_module_linking_enabled=0
generated_program_exe_aot_claim=0
new_function_lowering_owner=0
```

## Next

Continue:

```text
RUST-SUBSET-NEXT-CRATE-PILOT-SELECTION-001
```

Pick the next small crate/module slice using the same inventory-first process.
