# RAW public cutover PARITY0 effect repair S0

Decision: `RAW-PUBLIC-CUTOVER-PARITY0-EFFECT-prime-r1`

Status: closed in the repair commit. This was the selected narrow repair; do not widen PARITY0
success fixtures until this repair is complete.

## Scope

Repair only the Raw BODY0 root `main` skeleton effect mismatch identified by
the first deterministic parity snapshot.

```text
Legacy main signature.effect = PURE
Raw main signature.effect    = READ
```

The Raw producer is `src/mir/builder/raw_root_body_lowering.rs`; the Legacy
reference is `src/mir/builder/module_lifecycle.rs`. No other effect policy is
opened by this task.

## Steps

```text
EFFECT-PRODUCER0
  change Raw root skeleton signature to EffectMask::PURE
  remove only now-unused imports

EFFECT-FIXTURE0
  focused BODY0 empty/scalar signature witness

EFFECT-PARITY0
  restore the empty Script Legacy-vs-Raw snapshot test

EFFECT-G0
  focused tests, parity snapshot, cargo check, diff check, pointer guard
```

## Forbidden

```text
omit signature effects from normalized parity
map READ to PURE in the test normalizer
change Legacy root contract
infer arbitrary effects from instruction streams
add fallback/retry/catch_unwind
change public ingress, JSON, executor, selfhost, fastmem, or CUT0
```

## Acceptance

```bash
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_root_body_p0 -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_public_cutover_parity -- --test-threads=1
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

All modified source/check files remain below 800 lines. After this row closes,
PARITY0 may resume its bounded test-only matrix. If another exact production
mismatch appears, stop and open another repair/design row instead of weakening
the snapshot authority.

## Next

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-PARITY0-S0
```
