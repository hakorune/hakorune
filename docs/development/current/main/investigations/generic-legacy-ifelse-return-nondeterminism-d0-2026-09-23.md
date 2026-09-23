---
Status: landed__repair_green_and_held_rows_classified
Task: GENERIC-LEGACY-IFELSE-RETURN-NONDETERMINISM-D0
Date: 2026-09-23
Parent: generic-legacy-disposition-d0-2026-09-23.md
PreviousCard: generic-legacy-disposition-d0-2026-09-23.md
NextCard: family_scheduler__GENERIC-LEGACY-DISPOSITION-D0_closeout_or_S0
Implementation permission: closed; the bounded repair slice landed at
e6178e7335 and the 4 held corpus rows are classified.
---

# Generic legacy if-else-return nondeterminism repair D0

## Six-line brief

```text
Decision: open the named repair row that the landed DISPOSITION-D0
  card holds four corpus rows behind; the defect is a deterministic-
  contract violation, not a route question — the same fixture and
  binary intermittently emits legacy `define void @main()` instead of
  canonical `define i64 @main()` on the callable lane.
Source authority + canonical issuer: the P1 batch-6 card's recorded
  nondeterminism (reproduced 2026-09-23: 3/8 `void @main` at HEAD
  `197ea80fa4`), the two fixtures
  `phase29bq_selfhost_blocker_parse_program2_if_else_return{,_var}
  _min.hako`, and the callable `--dump-mir` lane as the sole observed
  axis.
Non-authority: a single green run as proof of repair, compat-lane
  output, retry counts as evidence, or any route/admission change —
  the repair removes a nondeterminism source, it does not re-decide
  which route is canonical.
Fail-fast boundary: if the nondeterminism source cannot be named to
  an owner (e.g. unordered map/set iteration, process-address or
  env-dependent ordering), stop as NoSafeSlice with the named
  owner rather than masking by sort/retry; the four held corpus rows
  stay P0-frozen meanwhile.
Smallest next slice: under work_mode=fast, reproduce serially,
  isolate the ordering source in the route/emission path for this
  fixture shape, land the minimal determinism fix plus a repeat-run
  guard, then classify the 4 held corpus rows (portable-owner once
  the canonical route is stable).
Non-claims: no new accepted shape, no Recipe/selector arm, no
  fixture/source change, no corpus re-census, no disposition change
  beyond the 4 held rows, no S6E/S0 closeout claim.
```

## Fixed evidence boundary

```text
fixtures:
  apps/tests/phase29bq_selfhost_blocker_parse_program2_if_else_return_min.hako
  apps/tests/phase29bq_selfhost_blocker_parse_program2_if_else_return_var_min.hako
held corpus rows (P0-INVENTORY-ONLY, observation_state=accepted):
  selfhost_parse_program2_if_else_return_min     (fast-gate tsv:72)
  selfhost_parse_program2_if_else_return_var_min (fast-gate tsv:73)
  selfhost::phase29bq_selfhost_blocker_parse_program2_if_else_return_min.hako     (subset sub:25)
  selfhost::phase29bq_selfhost_blocker_parse_program2_if_else_return_var_min.hako (subset sub:26)
reproduction (HEAD 197ea80fa4, target/debug/hakorune):
  --dump-mir if_else_return_min x8 -> i64 @main x5, void @main x3
```

## Design boundary

- Sibling `if_else_return_local_min` (tsv:74, subset sub:27) and the
  rest of the `if_return` family are stable canonical — the defect is
  specific to the `if_else_return{,_var}` shape (else-branch return
  with/without a var binding). Diff these two fixtures against
  `if_else_return_local_min` before reading the route code: the shape
  delta bounds where ordering can leak.
- Suspect class, in audit order: (a) `HashMap`/`HashSet` iteration
  order feeding route selection or emission input; (b) `once_cell`/
  lazy registry with nondeterministic first-writer; (c)
  process-dependent address ordering leaking into a `BTreeMap`-less
  collection. Fix = replace with ordered iteration or a deterministic
  tie-break at the named owner — never by retrying or sorting the
  output.
- The repair commit carries a repeat-run guard (e.g. N=20 serial
  `--dump-mir` runs asserting `define i64 @main()` every time) so the
  determinism contract is enforced, not sampled.
- After the fix lands green, the 4 held rows are written in the same
  or the immediately following commit: `decision=D0-DISPOSITION-
  CHECKED`, `disposition=portable-owner`, `current_acceptance=
  accepted`, `observed_route=canonical-main`, `target_owner=
  canonical-main` — no guard vocabulary change needed (tokens exist).
- If the source cannot be named: card closes `NoSafeSlice` with the
  named owner, the 4 rows stay held, and S6E stays open — do not
  convert to Declined/Rejected by vote.

## Landed evidence (e6178e7335)

- Root cause (named owner): `infer_return_type_from_phi` in
  `src/mir/builder/return_type_strategy.rs` iterated
  `function.blocks` (a `HashMap`) and kept the first Return
  terminator's type. The synthesized void tail block could be
  visited before the concrete integer-returning branch blocks, so
  the same fixture intermittently emitted `define void @main()`.
- Fix: collect every `Return` terminator, sort candidates by
  `BasicBlockId`, and prefer concrete non-`Void`/non-`Unknown`
  types — `Void`/`Unknown` is kept only as a fallback via
  `get_or_insert`. No route, admission, or fixture change.
- Regression coverage:
  `return_type_strategy_tests::if_else_return_main_infers_integer_signature`
  (Ring0-initialized compile of a static Main with integer returns
  in both branches; asserts `MirType::Integer`).
- Repeat-run guard:
  `tools/checks/generic_ifelse_return_signature_determinism_guard.sh`
  — `OK 20x2 runs, canonical i64 @main every time` for both
  fixtures. Earlier isolation loop: 30/30 deterministic on each
  fixture.
- Held-row classification landed in the same commit: all 4 rows now
  `decision=D0-DISPOSITION-CHECKED`, `disposition=portable-owner`,
  `current_acceptance=accepted`, `observed_route=canonical-main`,
  `target_owner=canonical-main`. Corpus decision totals are now
  40 `D0-DISPOSITION-CHECKED` / 196 `D0-TYPED-REJECT` /
  162 `D0-PRE-LOOP-EVIDENCE`; zero rows remain P0-INVENTORY-ONLY.
- Gates: corpus universe guard tests 6/6 OK; current-state pointer
  guard OK; `cargo test --features vm-reference main0` 63/63 PASS
  (no regression in the shared inference path).
- Known baseline debt (not this slice): the pre-existing test
  `record_value_publish_is_a_void_script_result` fails identically
  at parent `5f0f831b3d` with
  `[freeze:contract][raw-compat/runtime-box-fate-retired/instance]`
  — retired raw-compat debt, left for a separate named row.
- Non-claims kept: no legacy fallback restored, no production
  switch, no cutover, no corpus re-census beyond the 4 held rows,
  no S6E/S0 closeout claim.
