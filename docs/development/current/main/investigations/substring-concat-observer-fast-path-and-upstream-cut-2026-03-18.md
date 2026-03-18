# Substring Concat: Observer Fast Path Rejected, Upstream Cut Candidate (2026-03-18)

## Scope

- Target: `substring_concat`
- Current question:
  - whether the observer-only fast path is still a keep candidate
  - what the next viable upstream cut is if it is not

## Rejected Observer Fast Path

The explicit observer fast path in `crates/nyash_kernel/src/exports/string.rs::string_len_from_handle(...)`
was tried with direct `StringBox` / `StringViewBox` downcasts.

Observed result:

- isolated `substring_concat` micro improved to `265893951 cycles / 68 ms`
- stable `kilo_kernel_small_hk` regressed to `1066 ms` median
  - `min=786`
  - `max=1841`

Conclusion:

- this is a rejected observer-only cut
- do not reopen this path without a stronger owner-level reason

## Next Viable Upstream Cut Candidate

The next safe candidate is the upstream reduction of `StringViewBox::new()` call count,
not any change to generic identity semantics in `BoxBase::new()`.

Concrete wave shape:

- work the `substring -> concat3 -> length` inner chain in `benchmarks/bench_kilo_micro_substring_concat.hako`
- keep `text = out.substring(1, len + 1)` as the escape boundary
- reduce transient `StringViewBox` churn upstream of `substring_hii`
- preserve the current flat short-slice policy `<= 8 bytes`

This aligns with the current design SSOT:

- `docs/development/current/main/design/perf-optimization-method-ssot.md`
- `docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md`
- `docs/development/current/main/design/box-identity-view-allocation-design-note.md`

## Bottom Line

- Rejected: observer-only downcast fast path in `string_len_from_handle`
- Next candidate: upstream transient/span-first cut that reduces `StringViewBox::new()` births in the inner string chain
