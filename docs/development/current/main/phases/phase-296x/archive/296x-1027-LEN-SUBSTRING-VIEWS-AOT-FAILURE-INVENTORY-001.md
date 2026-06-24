Status: Done
Date: 2026-06-17
Scope: compiler coverage fix for `kilo_micro_len_substring_views`.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1026-FRESH-COMPILER-OWNER-SELECTION-003.md
Artifacts:
  - target/len-substring-views-aot-failure-1027/aot_asm_direct_only.log
  - target/len-substring-views-aot-failure-1027/ny_mir_builder.log
  - target/len-substring-views-aot-failure-1027/len_substring_views.after.mir.json
  - target/len-substring-views-aot-failure-1027/ny_mir_builder.after2.log
  - target/len-substring-views-aot-failure-1027/lanes.after.log

# LEN-SUBSTRING-VIEWS-AOT-FAILURE-INVENTORY-001

## Purpose

Turn the selected coverage blocker into one narrow accepted shape.

`kilo_micro_len_substring_views` previously failed AOT under the perf
pure-first route:

```text
reason=pure_first_unsupported_shape
first_block=0
first_inst=11
first_op=mir_call
owner_hint=backend_lowering
```

The unsupported MIR site is a `StringBox.substring` call outside the hot loop:

```text
b0.i11 StringBox.substring receiver=8 args=[3,20,21]
b0.i12 StringBox.substring receiver=8 args=[3,21,11]
```

The first argument is a redundant receiver carrier. The existing
`StringSubstring` route accepted only the logical substring arguments, so the
route metadata was not produced for this real MIR shape.

## Change

Accepted shape:

```text
StringBox.substring(receiver=R, args=[R_like, start, end])
```

Implementation:

```text
rust_route_producer:
  match_generic_substring_route strips redundant StringBox receiver carrier
  route_id=generic_method.substring
  route_kind=string_substring
  arity=2

cabi_arg_normalization:
  hako_llvmc_normalize_generic_method_logical_args strips the same carrier
  only when source_route_id=generic_method.substring and both first arg and
  receiver have String origin
```

This is not a benchmark/helper-name branch. It is a route-metadata accepted
shape for a concrete MIR call convention already seen in the active front.

## Evidence

Unit:

```text
cargo test -q generic_method_route_plan::tests::string_routes::substring_routes
```

MIR JSON after the fix:

```text
b0.i11 route_id=generic_method.substring route_kind=string_substring arity=2
b0.i12 route_id=generic_method.substring route_kind=string_substring arity=2
```

Boundary build after the fix:

```text
build_rc=0
consumer=mir_call_route site=b0.i11 route=generic_method.substring
consumer=mir_call_route site=b0.i12 route=generic_method.substring
```

Runtime result:

```text
Result: 4800016
run_exit=16
```

Perf harness status:

```text
aot_status=ok
c_kernel_instr=1501308
c_kernel_cycles=302121
ny_kernel_instr=73806308
ny_kernel_cycles=30512946
ratio_kernel_instr=0.02
ratio_kernel_cycles=0.01
```

Interpretation:

```text
coverage_blocker_fixed=1
perf_owner_selected=0
hako_slow_kernel_visible_after_coverage=1
next_task=LEN-SUBSTRING-VIEWS-POST-COVERAGE-OWNER-SELECTION-001
```

## Stop Lines

```text
do not treat this row as an optimization keeper
do not add generic fallback for unsupported substring shapes
do not infer from benchmark name
do not change StringBox product semantics
do not optimize len_fast_h or substring_hii internals in this row
```

