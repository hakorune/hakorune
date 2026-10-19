# MirBuilder host build and link time measurement

Status: daily opt0 CLI/test commands adopted; keep LLD and incremental
Date: 2026-10-20
Scope: independent tooling improvement; not a finite_product_goal completion condition
Related:
  - ../CURRENT_STATE.toml
  - ../../../RULES.md
  - mirbuilder-compile-time-performance-owner-first-d0-2026-08-22.md
  - ../../../../../.cargo/config.toml

## Current decision

Daily CLI builds use `cargo quick-build-fast`, matching the existing manual
`cargo build --profile quick --bin hakorune --config profile.quick.package.nyash-rust.opt-level=0`.
Root-library tests use `cargo quick-test-fast`; both retain optimized dependencies,
incremental compilation, jobs4, assertions and LLD. `quick-build`/`quick-test`
retain their existing opt1 behavior for optimized validation. Release is unchanged.
This prevents accidental profile switching; it does not accelerate an already
opt0 build or demonstrate further compiler runtime speedup.

Verification on the preserved If/SSA WIP: latest required opt0 CLI build34.70s.
New alias with `--verbose`: exit0, root crate Fresh, Cargo0.26s/wall0.331s;
no rustc/link invocation or dependency rebuild. Existing incremental cohort reused.
No source, fixture, test, guard, cache or target was added/removed for this check.
The earlier controlled test-message edit (88.58s opt1 versus37.51s opt0) remains
the bounded codegen evidence below; it is not a new CLI speed comparison.
Host sccache remains installed for eligible dependencies; current server statistics
show incremental/crate-type exclusions, not hot-edit cache hits. Keep incremental.

## Unused test dependency retirement (2026-10-20)

Removed unused root `criterion` dev-dependency, its commented-out benchmark stub,
and redundant dev `tempfile` declaration (the normal dependency remains).
No active Rust source or benchmark target imports Criterion. Offline root
normal/build/dev dependency graph drops from133 to109 unique packages, including
the Criterion plotting, Rayon and Ciborium subtrees. This reduces first-build
dependency work; no hot-edit speedup is inferred. Existing caches are retained.
Verification: `cargo quick-test-fast original_text_argument -- --test-threads=1`
builds successfully and runs1 existing case PASS; `--list` still reports9263 tests,
0 benchmarks. Changing the dependency cohort incurred a one-time2m44s test build;
this is not an edit/build speed comparison. `cargo quick-build-fast --verbose`
reuses the existing CLI with no recompilation. No test/checker/source change.

## Native-only dependency pruning (2026-10-20)

Move wasm-bindgen, js-sys, web-sys and the browser panic hook into the existing
`cfg(target_arch = "wasm32")` target boundary. Their root source uses are already
wasm32-gated; native Box implementations remain unchanged. Remove unused root
build-dependency once_cell; the normal dependency remains. No source/test edits.
Offline locked Cargo trees, normal/build/dev edges, deduplicated by package:
native109 ->100 (nine browser packages removed); wasm32 stays100 with identical
package identities. Combined with Criterion retirement, native133 ->100.
This removes dependency compilation work from fresh native builds; it does not
establish a hot-edit speedup. Wasm target compilation is not verified on this
host (only the native target is installed); its dependency graph is preserved.
Existing incremental caches and compiler WIP are retained. A dependency cohort
change can cause a one-time rebuild; record it separately from hot-edit timings.
Verification with retained compiler WIP: locked quick-test-fast builds173.52s,
existing finite-entry provenance case1 PASS (.03s),9262 filtered; locked
quick-build-fast builds98.78s. These are dependency-cohort switch costs, not
controlled edit comparisons. A second unchanged CLI build is Fresh, wall.339s,
with no rustc/link invocation; CLI --version exits0. No tests/checkers added.

## Unused CLI macro dependency retirement (2026-10-20)

Remove unused root anyhow and Clap's derive feature: root CLI/bin sources use
Clap's builder API, with no derive/attribute consumers in source, tests or examples.
ny-llvmc retains its own derive dependency; no workspace tool capability is removed.
Locked offline native normal/build/dev graph drops100 ->97 unique packages:
anyhow, clap_derive and heck leave the root graph. Cargo.lock stays unchanged
because other workspace packages still need them. Combined native pruning133 ->97.

With existing compiler WIP preserved, quick-build-fast --locked --offline --timings
succeeds in106.58s. This is a dependency-cohort change plus WIP CLI build, not a
controlled edit-speed comparison. A second --verbose run takes.345s, root Fresh,
zero compilation/rustc invocations. Matching CLI --version/--help exit0 and invalid
--grammar-profile exits2. No test/checker/guard added; incremental, jobs4, LLD,
assertions and optimized dependency settings remain. Existing semantic test failure
and unfinished compiler WIP are outside this tooling change and remain uncommitted.
No hot-edit speedup or product acceptance is claimed by these CLI checks.

## Historical linker measurement

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

## First required-build observation (Root key transport)

At source revision 6c7d192311, the required focused quick build with unchanged
LLD flags/profile/jobs/target completed in 5m45s; focused6 tests took .05s and
same-binary regression573 took .43s wall time. A temporary read-only sampler,
attached after build start and before linker launch, sampled rustc descendants
at requested .2s intervals (plus enumeration overhead). Clang and ld.lld each
appeared in five samples spanning .864s immediately before rustc exit. The full
observed rustc window was 332.47s; this includes compilation/codegen/link, not
frontend-only time. /proc enumeration recorded 67 process-exit read races.
Evidence: /tmp/hako-object-root-key-process-timing.json, focused.log and
regression.json. This is sampled process residence, not exact linker CPU time or
an LLD/mold comparison. No complete link argv/temporary input set was retained.

For this workload, the observed link residence is a small part of build time;
keep LLD and prioritize compiler/codegen/incremental evidence for substantial
build reductions. A mold trial remains optional if another required workload
shows material link time. No config/profile change, new gate or speedup claim.


## MIRBUILDER-HOST-SCCACHE-I0 — user-selected cache improvement (2026-10-12)

Status: selected; installation/measurement pending.
Decision: user asks to prioritize build time. Install official prebuilt sccache
locally; share local cache under /mnt/workdisk via host Cargo wrapper, leaving
repository/CI linker, profiles, incremental compilation and existing targets intact.
Source authority: installed rustc/Cargo configuration, official sccache Rust/local
cache restrictions and actual cache statistics/timed matching compilations.
Non-authority: estimated daily wait, target disk size, hardware speed claims,
cache-hit capability alone or cold-versus-warm numbers as hot-edit improvement.
Boundary: preserve normal quick incremental compiler/test builds. sccache cannot
cache incremental Rust or crates invoking the system linker; it may still cache
eligible dependencies. Never disable incremental globally or erase caches/targets
just to make a cache hit. Do not add distributed services or hardware changes.
Acceptance: checksum/version verify the official binary; isolated matching
cross-directory Rust dependency builds show miss then hit, execute correctness
check, and a required real quick build records cache eligibility and timing.
Document actual scope/limitations and exact host configuration/undo. Then restore
Required Object Forward S0; its identity correction and both final positives remain
owed. Host tooling is user-requested, not a new finite_product_goal criterion.
References: https://github.com/mozilla/sccache/blob/main/docs/Rust.md,
https://github.com/mozilla/sccache/blob/main/docs/Local.md.


### Shared-cache receipt and next test-codegen experiment

Official sccache0.18.0 musl archive checksum verified, installed in host Cargo bin.
Host Cargo wrapper + SCCACHE_DIR=/mnt/workdisk/home_storage/.cache/sccache,
20G bound installed without repository/profile/linker/target cleanup changes.
Isolated serde/serde_json pinned to root lock, two identical standalone sources
in distinct directories/empty targets: first5.20s, second3.37s, both exit0 and
cache-probe=ok. Second build delta9 Rust hits/3 misses; linker/build-script/proc
macro requests remain uncacheable. These are isolated dependency-cache numbers,
not a full campaign or hot-edit speedup. Required real quick test build210.86s,
95 focused PASS; cache delta0hit/0miss, one incremental exclusion. Earlier294.08s
had different source/incremental work: the reduction is NOT attributed to sccache.
Logs/statistics/manifests: /tmp/hakorune-sccache-20261012/. Existing targets retained.

## MIRBUILDER-HOST-QUICK-TEST-CODEGEN-I0 — bounded experiment

Decision: user prioritizes build time; dependency cache cannot remove the observed
large incremental lib(test) build. Try opt-level=0 only for nyash-rust in a quick
unit-test command, using Cargo's package override. Preserve optimized dependencies,
quick debug/overflow assertions, incremental, jobs, linker and production CLI profile.
Keep this opt-in/local until evidence is recorded; never globally disable incremental
or use cached output/test-count reduction as test-codegen speed evidence.
Acceptance: same source manifest; compare full build conditions including distinct
incremental cohorts; run the same95 focused tests and preserve pre-existing failures.
If favorable, publish an opt-in daily-test alias with explicit package override,
retain quick-test for optimized validation, and measure a controlled small test-only
edit under both settings before claiming hot-edit speedup. No compiler semantic
change or finite-goal completion condition. Cold setup versus hot edit stays separate.
Official profile override: https://doc.rust-lang.org/cargo/reference/profiles.html#overrides.

Opt0 first-cohort build138.38s PASS against the exact required-test source manifest;
same95 tests PASS (execution roughly0.89s versus roughly0.24s optimized, not a
compiler runtime benchmark). No hot-edit speedup inferred from different cohorts.
Controlled edit now changes ONE assertion's failure-message string in the existing
missing-Completion test; no production/.hako semantics or acceptance expectation
changes. Owner: this card; retire_when: both settings measured and original bytes
restored by controlled_edit.py finally block. Store original/variant SHA, full argv,
build timing, binary hash and same95 outcomes; artifact owner `/tmp/hakorune-sccache-20261012`.
The temporary edit is never staged or committed. Restore by exact byte comparison,
protecting any unexpected external edit. Trial currently running optimized first,
then opt0, each starting from its previously warmed base-source cohort.


### User-requested design consultation pause (2026-10-12)

Goal paused explicitly; stop implementation/builds and await the user's discussion.
Controlled ONE test-message edit: warmed opt1 build88.58s, warmed opt0 build37.51s;
same variant SHA `7fc39b24380b0c8c3a13397e23b04560e9ade9612bd03b1589956103bf1197f5`,
both95 PASS. Original bytes restored; controlled-edit-restored.json proves equality.
This one-run comparison is a test-message edit, not arbitrary semantic edit speed.
Repository quick-test-fast alias and RULES daily root-test routing added UNCOMMITTED.
Alias validation had ALREADY completed at the stop request: exit0,65 entry tests
PASS, build+test37.30s; alias-check.* records exact command. No interrupted-build
success is claimed. Default quick-test/CLI profile/dependencies/assertions remain.
No Cargo/rustc running; this session's sccache server stopped (cache retained and
Cargo can restart it). Host Cargo config and official sccache installation persist.
No commit/push in this tooling iteration; HEAD remains1e92d4e62a.

Resume after consultation: review/close out selected tooling paths only, then restore
Required Object Forward S0 using the saved previous state/unfinished app card.
All compiler WIP remains uncommitted and S0 unaccepted. Its identity test correction
now passed with the original producer and all seven corruption refusals; both final
physical publication positives and common downstream input cause still owed.
Fresh app Completion is Ready; inspect original reconstruction/signature/whole
incoming gates, not a nonexistent missing-Completion gate. Preserve real-call
coverage and exact own result/cleanup authority; Maker.qualified/0 membership still
fails. No app EXE, compiler cutover/retirement or finite-product completion claim.


Disk-space housekeeping (user requested, 2026-10-12): immutable saved binaries
and three owned evidence directories moved to
`/mnt/workdisk/hakorune-artifacts/root-offload-20261012/`; original `/tmp` paths
remain resolving symlinks. SHA256/content manifests verified before/after; no
proof, baseline or source deleted. Root free space1.1GiB ->7.2GiB. Host Cargo
TMPDIR now `/mnt/workdisk/hakorune-artifacts/tmp`; existing target/cache settings
preserved. This moves retained evidence off the root filesystem, not repository
LOC reduction. No Cargo/probe run. Integration consultation now points back to
the unfinished app card, with goal paused and no execution permission.
