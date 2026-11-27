# Benchmark measurement copies (BM-13)

These files are measurement copies of benchmarks in `benchmarks/`. They are not
corpus inputs and are not listed in `apps/tests/corpus/manifest.json`. The
originals in `benchmarks/` are unchanged.

- **Primary measurement uses the originals.** The five originals that compile
  today run all their work and normal cleanup, then stop with the
  `ExitCodeOutOfRange` Fault because the result is outside `0..=255`
  (`docs/reference/language/function-exit-and-entry-result.md`, reason 102).
  A run is accepted only when it ends with `exit=70`, `reason=102`, the listed
  `site`, and `details=(full expected value, 0)`.
- **`*_m256.hako` is the paired normal-exit copy.** Only the final `return` is
  reduced with `% 256`. Different results can share the same low 8 bits, so
  the exit code alone is a weak check.
- **Rewrites** are copies of originals that the current compiler rejects. Each
  rewrite has a full-result file (ends with the same reason 102 Fault) and a
  `% 256` pair. Three rewrites change the work and carry a different workload
  name (marked "different workload").

## How to build and run

```bash
hakorune --backend mir --verify --emit-exe OUT --emit-exe-nyrt <runtime-dir> FILE.hako
./OUT    # stderr: [fault:primary] reason=102 site=<site> details=<value>,0
```

The expected values below were measured with the CLI built from `e8049c89c3`,
the pure-first recipe (`HAKO_BACKEND_COMPILE_RECIPE=pure-first`,
`HAKO_BACKEND_COMPAT_REPLAY=none`) and LLVM codegen O0. The `site` number is a
wire site of the compiled program; it can move when the MIR shape changes.

## Expected results

"Expected value" was computed independently of Hakorune (plain Python loops,
and closed forms where noted). "C ref" is `benchmarks/c/<original>.c`.

### Originals (primary) and their `% 256` pairs

| Original | Pair | Measured operation | Iterations | Expected value | Original ends with | Pair exit | C ref |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `benchmarks/bench_compare_reuse_small.hako` | `bench_compare_reuse_small_m256.hako` | `i % 31`, the same `m < 20` compare twice, integer adds | 120,000 | 232,260 | exit 70, reason 102, site 0, details 232260,0 | 68 | prints 232260 |
| `benchmarks/bench_kilo_meso_substring_concat_len.hako` | `bench_kilo_meso_substring_concat_len_m256.hako` | 2 `substring`, `left + "xx" + right`, `length()` | 180,000 | 3,240,016 = 180,000 × 18 + 16 | exit 70, reason 102, site 19, details 3240016,0 | 80 | same value, returns `& 0xFF` (80) |
| `benchmarks/bench_kilo_micro_concat_hh_len.hako` | `bench_kilo_micro_concat_hh_len_m256.hako` | `text + suffix`, `length()` | 800,000 | 14,400,018 = 800,000 × 18 + 18 | exit 70, reason 102, site 2, details 14400018,0 | 18 | same value, returns `& 0xFF` (18) |
| `benchmarks/bench_kilo_micro_len_substring_views.hako` | `bench_kilo_micro_len_substring_views_m256.hako` | `length()` of two substrings made before the loop | 300,000 | 4,800,016 = 300,000 × 16 + 16 | exit 70, reason 102, site 6, details 4800016,0 | 16 | same value, returns `& 0xFF` (16) |
| `benchmarks/bench_kilo_micro_substring_only.hako` | `bench_kilo_micro_substring_only_m256.hako` | 2 `substring`, 2 `length()` | 300,000 | 4,800,016 = 300,000 × 16 + 16 | exit 70, reason 102, site 6, details 4800016,0 | 16 | **different tail**: C has no final `+ len`; C value 4,800,000, returns `& 0xFF` (0) |

### Rewrites (full result and `% 256` pair)

| Full-result file | Original | What changed | Measured operation | Iterations | Expected value | Full file ends with | Pair exit | C ref |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `bench_kilo_micro_userbox_point_sum_typedfield.hako` | `benchmarks/bench_kilo_micro_userbox_point_sum.hako` | fields `i64`, set by `birth(x: i64, y: i64)` / `new Point(1, 2)` instead of IntegerBox fields assigned after `new` | instance call `p.sum()` that reads two fields | 2,000,000 | 6,000,003 = 3 × (n + 1) | exit 70, reason 102, site 7, details 6000003,0 | 131 | prints 6000003; gcc -O2 folds the loop |
| `bench_kilo_micro_userbox_counter_step_typedfield.hako` | `benchmarks/bench_kilo_micro_userbox_counter_step.hako` | field `i64`, set by `birth(value: i64)` | instance call `c.step()` that reads one field | 2,000,000 | 86,000,043 = 43 × (n + 1) | exit 70, reason 102, site 6, details 86000043,0 | 171 | prints 86000043; gcc -O2 folds the loop |
| `bench_kilo_micro_userbox_counter_step_chain_typedfield.hako` | `benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako` | same as counter_step | `c.step_chain()` that calls `me.step()` | 2,000,000 | 86,000,043 = 43 × (n + 1) | exit 70, reason 102, site 6, details 86000043,0 | 171 | prints 86000043; gcc -O2 folds the loop |
| `bench_method_call_only_small_literal.hako` | `benchmarks/bench_method_call_only_small.hako` | receiver is the literal `"nyash"` instead of `new StringBox("nyash")` | `s.length()`; MIR lowers it to the length of a static text, no real call | 5,000 | 25,000 | exit 70, reason 102, site 0, details 25000,0 | 168 | prints 25000; gcc -O2 folds the loop |
| `bench_method_call_only_literal.hako` | `benchmarks/bench_method_call_only.hako` | same as above | same as above | 2,000,000 | 10,000,000 | exit 70, reason 102, site 0, details 10000000,0 | 128 | same value, returns `& 0xFF` (128) |
| `bench_kilo_micro_userbox_point_step_method.hako` (**different workload**) | `benchmarks/bench_kilo_micro_userbox_point_add.hako` | the per-iteration field read/write moves into `Point.step()`, the final sum into `Point.total()`; fields `i64` set by `birth` | instance call that reads 2 fields and writes 2 fields | 2,000,000 | 6,000,009,000,003 = Σ(3 + 3i) + (3 + 3n) | exit 70, reason 102, site 9, details 6000009000003,0 | 67 | original work: same value, returns `& 0xFF` (67) |
| `bench_kilo_micro_userbox_flag_i64_methods.hako` (**different workload**) | `benchmarks/bench_kilo_micro_userbox_flag_toggle.hako` | the Bool field becomes an `i64` 0/1 field; `hit()` and `set_on()`/`set_off()` replace the Bool compare/assign | 1 instance call that reads a field, 1 that writes it | 2,000,000 | 1,000,001 (hits for i = 0..1,000,000) | exit 70, reason 102, site 10, details 1000001,0 | 65 | original work: same value, returns `& 0xFF` (65) |
| `bench_numeric_mixed_flat_branch.hako` (**different workload**) | `benchmarks/bench_numeric_mixed_medium.hako` | `if / else if / else` becomes three flat `if`s chosen by `k = m / 10` (one more division, 1-2 more compares per iteration) | integer `%`, `/`, `*`, compares | 800,000 | 218,800,319 (same as the original) | exit 70, reason 102, site 0, details 218800319,0 | 191 | original work: same value, returns `& 0xFF` (191) |

Each rewrite's pair is the same file name with `_m256` before `.hako`.

## Notes for comparisons

- For C, check that the loop and the final sum survive. `gcc -O2` folds the
  four loops marked above; compare with `gcc -O0` or a `volatile` copy instead.
- The rewrites with a different workload must be compared only with themselves
  (for example O0 against O1 of the same file), not with the original.
- The instruction count of a `% 256` pair differed from its original by
  0.006–1.9% in BM-12; compare runs of the same binary where possible.
