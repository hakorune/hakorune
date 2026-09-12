# LLVM18 toolchain installation task

Status: `verified__LLVM18Toolchain__G0WitnessClassified__2026-09-12`
Task: `LLVM18-TOOLCHAIN-INSTALL-I0`
Date: `2026-09-12`
Priority: restore the named LLVM18 object/EXE acceptance environment
Owner: development environment, not a MirBuilder semantic owner
Parent: `MIR-CALL-RUST-CAPI-AMBIENT-RETIRE-I0`

## Six-line brief

```text
Decision: install LLVM18 side-by-side with the host LLVM14 using the repository CI's apt.llvm.org route; do not replace or reinterpret the existing LLVM14 toolchain.
Source authority + canonical issuer: the pinned repository CI installation recipe and the host package manager issue the executable/tool/header installation; the task records observed versions and paths.
Non-authority: Cargo feature names, llvm-config discovery alone, test-only emitters, ambient compiler selection, and an LLVM18 installation do not issue MIR meaning or select a production route.
Fail-fast boundary: OS/architecture, package-manager availability, versioned binaries, C API headers, linker, and prefix checks must pass before LLVM18-dependent acceptance is rerun.
Smallest next slice: install the versioned LLVM18 packages, verify the complete tool/header/prefix surface, then rerun one named G0 object/EXE witness with jobs=1.
Non-claims: no compiler semantic change, backend parity, whole-MIRBuilder completion, public ABI change, concurrent-build support, or automatic LLVM upgrade policy.
```

## Installation contract

This is an environment-only task. It must not add a repository fallback, change
the selected backend, or make an LLVM18 check appear green when the tools are
absent. The host is Ubuntu 22.04 with LLVM14.0.0 at `/usr/lib/llvm-14` and
LLVM18.1.8 at `/usr/lib/llvm-18`, installed side-by-side from the official
`apt.llvm.org` Jammy route. LLVM14 remains installed and usable; the versioned
LLVM18 tools are the explicit acceptance surface.

The repository's existing CI recipe is the installation source:

```bash
sudo apt-get update
sudo apt-get install -y curl ca-certificates lsb-release wget gnupg python3-pip
curl -fsSL https://apt.llvm.org/llvm.sh -o llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
```

The installer may select the versioned LLVM18 package set for the detected
Ubuntu release. Do not remove LLVM14 or rewrite unversioned `/usr/bin/llvm-*`
links. If the host policy does not permit `sudo`, stop at the named external
dependency and record the exact missing permission; do not emulate LLVM18 with
LLVM14.

## Pre-install preflight result (historical, 2026-09-12)

The host was `x86_64` Ubuntu `22.04`; LLVM14.0.0 remained installed at
`/usr/lib/llvm-14`. Before the external installation, the session was UID 1000
and `sudo -n -v` reported `sudo: a password is required`. This historical
blocker is superseded by the host package installation recorded below.

The apt.llvm.org installation was performed outside Git. Its unrelated
GitHub-CLI repository `EXPKEYSIG` warning was not changed by this task.

## Post-install verification (2026-09-12)

The required versioned tools and C API surface are present in the same host
environment used for the witness:

```text
llvm-config-18 --version       -> 18.1.8
llc-18 --version               -> Ubuntu LLVM version 18.1.8
opt-18 --version               -> Ubuntu LLVM version 18.1.8
clang-18 --version             -> Ubuntu clang version 18.1.8
ld.lld-18 --version            -> Ubuntu LLD 18.1.8
llvm-config-18 --prefix        -> /usr/lib/llvm-18
llvm-config-18 --includedir    -> /usr/lib/llvm-18/include
llvm-config-18 --libdir        -> /usr/lib/llvm-18/lib
header                         -> /usr/lib/llvm-18/include/llvm-c/Core.h
```

`llvm-config-18 --libs --system-libs` exposes `-lLLVM-18`. The task's
`LLVM_SYS_180_PREFIX="$(llvm-config-18 --prefix)"` form is valid for the
LLVM-dependent Rust command.

## Preflight and acceptance

Before installation, record `uname -m`, `/etc/os-release`, the current
`llvm-config`, `llc`, and `clang` versions, and whether a versioned LLVM18
prefix already exists. After installation, all of these checks must be
observable:

```bash
llvm-config-18 --version
llc-18 --version
opt-18 --version
clang-18 --version
ld.lld-18 --version
llvm-config-18 --prefix
test -f "$(llvm-config-18 --includedir)/llvm-c/Core.h"
```

The versioned tools must report `18.x`, the C API header must exist, and the
prefix must contain the LLVM libraries needed by the selected C API build. For
LLVM-dependent Rust commands, use the existing versioned prefix explicitly,
for example:

```bash
LLVM_SYS_180_PREFIX="$(llvm-config-18 --prefix)" \
  CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 \
  cargo check -p nyash-rust --features plugins --profile quick -j1
```

Do not run more than one Cargo/rustc build at a time on the 16GB host.

## MirBuilder evidence to rerun

After the toolchain checks pass, rerun the existing named witness rather than
creating a new fixture or route:

```bash
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUST_MIN_STACK=16777216 \
  cargo test -p nyash-rust --features plugins --profile quick -j1 \
  normal_package_generic_g0_helper_reaches_existing_exe_emitter -- --ignored --nocapture
```

Record separately:

- LLVM18 object emission and executable exit status;
- source-backed `Main.main -> generic_g0(0, 0)` membership evidence;
- any missing runtime archive, FFI, linker, or application-library failure.

An LLVM18 installation removes only the environment skip. It does not turn the
existing root/EXE coexistence witness into helper execution evidence unless the
selected physical program actually contains and executes the helper call.

## Witness result (2026-09-12)

- The first invocation used the stale test name from this card and correctly
  ran zero tests (`7898 filtered out`); it is not acceptance evidence.
- The corrected named witness ran one ignored test after the LLVM18 toolchain,
  FFI library, `ny-llvmc`, and both runtime archives were checked. The first
  attempt exposed a path mismatch: the test hardcodes
  `target/release/libnyash_lifecycle_kernel.a`, while the existing archive is
  at `target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a`.
- A temporary generated-output symlink was used only to continue the unchanged
  witness. It then reached the existing typed EXE route and failed before
  object/link publication with
  `[freeze:contract][published-lifecycle-physical-abi/site-missing]`.
  The temporary symlink was removed; the Git worktree is clean of that change.
- Therefore LLVM18 availability is verified, but LLVM18 object emission,
  executable exit status, and helper runtime execution remain unclaimed. The
  remaining blocker is the existing lifecycle physical-ABI/site contract (and
  the test's runtime-path assumption), not LLVM18 installation.
- The same `-j1` Rust build completed successfully and exposed the warning
  baseline separately: `nyash-rust (lib)=1793`; `nyash-rust (lib test)=523`,
  including 255 duplicates. Warning reduction is owned by
  `MIRBUILDER-WARNING-SURFACE-CENSUS-R0` and its
  `MIRBUILDER-WARNING-BASELINE-REFRESH-I0` child; these counts are not a
  zero-warning claim.

## Closeout requirements

The installation portion is closed after verification in the same shell used
for acceptance, and the LLVM18-dependent G0 witness is retained as a
classified, still-open acceptance result in this card. The host package state
is not committed to Git. LLVM14 remains the usable side-by-side toolchain for
the existing non-LLVM18 lanes.
