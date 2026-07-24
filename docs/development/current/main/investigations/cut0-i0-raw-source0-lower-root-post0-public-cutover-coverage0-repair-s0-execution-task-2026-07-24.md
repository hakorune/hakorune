# RAW public cutover COVERAGE0-REPAIR-S0 execution task

Decision: `COVERAGE0-REPAIR-prime-r1`

Status: active implementation.

## Goal

Repair the stashed COVERAGE0 WIP without widening helper grammar or moving
the normal-entry cutover. Produce one public NarrowV1 helper-coverage proof
that is source-exact, PLAN0-ordered, pre-physical, and below the 800-line
source/check boundary.

## Required implementation

1. Restore `stash@{0}` and keep its useful `StaticHelper0` source witness.
2. Keep PLAN0's ordered locators as the only CHILDREN0 execution schedule.
3. Add an exact PLAN0↔coverage locator parity check before physical open.
4. Carry `RawPublicIngressPolicyV1::NarrowV1` as a branded public profile;
   do not narrow general internal Raw eligibility implicitly.
5. Move policy/profile and parity orchestration into a small sibling module;
   do not grow `raw_root_eligibility.rs` further.
6. Make closeout guards rely on durable `COVERAGE0 are closed` and closed
   task markers so they remain rerunnable after pointer advancement.

## Fixtures

```text
App zero helper
App one empty helper
App two helpers with reverse insertion order -> lexical PLAN0 receipts
PLAN0/witness locator mismatch -> typed pre-physical rejection
non-static helper -> typed coverage failure before physical open
override helper -> typed coverage failure before physical open
non-empty helper -> typed pre-physical rejection
metadata/params/uses/attrs/contracts helper -> typed pre-physical rejection
every rejection -> Builder/physical/collector/ledger delta = 0
```

## Non-claims

```text
PARITY0 success matrix
normal compile_with_source switch
JSON/executor/selfhost/fastmem changes
old Raw retirement
fallback/retry/legacy rebuild
CUT0 activation
```

## Guard contract

```text
PLAN0 schedule producer                         = 1
coverage witness producer                      = 1
PLAN0↔coverage exact parity                    = 1
CHILDREN0 consumes PLAN0 locators               = 1
CHILDREN0 coverage grammar re-run               = 0
HashMap/sorted_method_entries in CHILDREN0      = 0
public NarrowV1 profile handoff                = 1
hidden global eligibility narrowing             = 0
eligibility/source/check files < 800 lines      = 1
closed durable marker + pointer guard           = 1
normal/JSON/executor/old Raw/CUT0 consumers     = 0
```

## Acceptance commands

```bash
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_root_helper_coverage -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_root_children -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_root_eligibility_p0 -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_public_cutover_coverage0_guard.py
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_public_ingress_config0_guard.py
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_public_ingress0_guard.py
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

On green evidence, mark this task closed, add a compact CURRENT_STATE landed
entry, and advance the pointer to the already queued PARITY0 task. Do not
delete the WIP stash until the repair commit is verified and pushed.

