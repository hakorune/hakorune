# MirBuilder host build and link time measurement

Status: measurement selected; linker adoption unverified
Date: 2026-10-08
Scope: independent tooling improvement; not a finite_product_goal completion condition
Related:
  - ../CURRENT_STATE.toml
  - ../../../RULES.md
  - mirbuilder-compile-time-performance-owner-first-d0-2026-08-22.md
  - ../../../../../.cargo/config.toml

## Current decision

Measure the next required Cargo build before choosing a linker change. Preserve
the selected MirBuilder source slice, quick profile, jobs, target and cache.
Prior performance decisions and parked semantic owners remain in the related
historical card; this measurement does not reopen them.

Read-only audit confirmed the current Linux GNU target selects Clang 14 with
`-fuse-ld=lld`; Cargo fingerprints and the existing test executable identify
LLD 14. The host also has mold 1.0.3, and Clang's dry-run selects `ld.mold` for
`-fuse-ld=mold`. This verifies driver selection, not successful linking or speed.

Current series: quick test build 5m13s, focused execution .06s, regression
execution .28s. Earlier Cargo timings place approximately 351–379s in one
`nyash-rust lib(test)` rustc unit. Compile/codegen/link are not separated.
Incremental objects include local ThinLTO material; do not classify all of the
rustc unit as linker time or change the profile together with the linker.

Current min-gate, fast-smoke and portability workflows do not install mold.
A shared default would require explicit supported Linux host/CI provisioning.
Changing rustflags also invalidates matching compilation fingerprints; record
the first rebuild separately from ordinary edit/build measurements. No cache wipe.

## Bounded task and acceptance

1. Add `--timings` to a genuinely required build and observe its rustc/Clang/
   linker child processes with a temporary read-only sampler. Record sampling
   interval/error and build/link/test time separately. Do not add a Cargo run
   solely for this initial measurement or a permanent test/guard/sampler owner.
2. If link time is material, compare LLD and installed mold with the same source,
   profile, jobs, features and workload; distinguish initial cache misses from
   repeated builds. A process trace is diagnostic evidence with trace overhead.
3. Preserve required focused acceptance and confirm the linked executable's
   linker identity. No speed claim from linker selection, test counts, a cold
   versus warm comparison, or another mold version's public benchmark.
4. Adopt a shared default only with target host/CI availability and measured
   benefit; otherwise keep any trial local. Missing mold is a setup failure,
   not an implicit linker retry. Keep compiler semantics and profile changes
   in separate slices.

Existing incremental directories contain multiple cohorts and no retained link
response/argument files. Do not reconstruct a link from an arbitrary object set.
Link-only replay requires the complete original argv and temporary inputs and
uses a separate output path. Any trial receipt stays with this card and Git;
temporary sampling artifacts are removed after evidence is recorded.

Non-claims: no Cargo/config/profile change or speedup has been verified; no
additional MirBuilder semantic gate, mandatory task, benchmark suite or new test.

Primary references: [mold usage](https://github.com/rui314/mold#how-to-use),
[Cargo timings](https://doc.rust-lang.org/cargo/reference/timings.html),
[Cargo fingerprints](https://doc.rust-lang.org/stable/nightly-rustc/cargo/core/compiler/fingerprint/index.html),
[Cargo LTO profiles](https://doc.rust-lang.org/cargo/reference/profiles.html#lto).
