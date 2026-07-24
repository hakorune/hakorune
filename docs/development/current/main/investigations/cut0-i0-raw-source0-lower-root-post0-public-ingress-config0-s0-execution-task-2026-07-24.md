# RAW public ingress CONFIG0 execution task

Decision: `RAW-PUBLIC-CUTOVER-prime-r1`

Status: closed. CONFIG0 implementation and focused evidence landed on 2026-07-24.

## Goal

Make the first public Raw request own exact no-import configuration without
mutating or inheriting the live Builder import map.

## Contract

```text
RawPublicImportDispositionV1::None
  is selected once by NarrowV1
  is owned by the ingress request
  projects candidate using_import_boxes = exact empty
  never clears or replaces live Builder imports

ambient Builder using_import_boxes as Raw authority = 0
fallible work after live Builder mutation            = 0
explicit-import Raw capability                       = 0
```

Use one shared Builder-configuration projection. Do not duplicate every
non-import configuration field in the compiler lane. The import disposition
is an explicit input to that projection; it is not an after-the-fact map
clear.

## Fixtures

```text
legacy compile installs imports
-> public bare Raw compile sees exact empty imports
-> live Builder imports remain unchanged on Raw failure

Raw failure with stale live imports
-> exact public rejection
-> subsequent Raw success
-> subsequent Legacy success with original imports

source-file hint and non-import Builder configuration
-> captured once
```

## Guard

```text
RawPublicImportDispositionV1 producer = 1
NarrowV1 disposition = None
public Raw ambient import snapshot = 0
pre-publication live import mutation = 0
explicit-import Raw entry = 0
normal/import-aware runner caller delta = 0
all modified source/check files < 800 lines
```

## Non-claims

```text
explicit Raw imports
helper coverage
normal-entry cutover
JSON/executor/selfhost/CUT0
```

## Closeout evidence

```text
RawPublicImportDispositionV1::None producer = 1
candidate using_import_boxes = exact empty
live Builder imports unchanged on projection and Raw failure = 1
explicit-import Raw capability = 0

python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_public_ingress_config0_guard.py
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_public_ingress_p0 -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib module_invocation_session_p0 -- --test-threads=1
cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Next row: `RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-COVERAGE0-S0`.

## Next row

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-COVERAGE0-S0
```
