# LLVM18 toolchain installation task

Status: `blocked__ExternalSudoPermission__2026-09-12`
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
absent. The host is currently Ubuntu 22.04 with LLVM14.0.0 at
`/usr/lib/llvm-14`; the Ubuntu Jammy archive has no `llvm-18` candidate.

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

## Preflight result (2026-09-12)

The host is `x86_64` Ubuntu `22.04`; LLVM14.0.0 remains installed at
`/usr/lib/llvm-14`. `llvm-config-18`, `llc-18`, `opt-18`, `clang-18`,
`ld.lld-18`, and `/usr/lib/llvm-18` are absent, and Jammy has no apt candidate
for `llvm-18`, `clang-18`, or `lld-18`. The CI installer was not run because
the session is UID 1000 and `sudo -n -v` reports `sudo: a password is
required`. This is an external permission blocker; LLVM14 remains the usable
toolchain and no LLVM18 runtime claim is made.

Resume the installation block above after sudo access is granted, then record
the versioned tool/header/prefix checks below.

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
  normal_package_generic_g0_reaches_existing_exe_emitter -- --ignored --nocapture
```

Record separately:

- LLVM18 object emission and executable exit status;
- source-backed `Main.main -> generic_g0(0, 0)` membership evidence;
- any missing runtime archive, FFI, linker, or application-library failure.

An LLVM18 installation removes only the environment skip. It does not turn the
existing root/EXE coexistence witness into helper execution evidence unless the
selected physical program actually contains and executes the helper call.

## Closeout requirements

Close this task only after the installation is verified in the same shell used
for acceptance, the LLVM18-dependent G0 witness has a classified result, and
the result is recorded in its owning acceptance card. The host package state
is not committed to Git. If installation fails, retain the task as an explicit
environment blocker with command output and leave LLVM14 as the usable host
toolchain.
