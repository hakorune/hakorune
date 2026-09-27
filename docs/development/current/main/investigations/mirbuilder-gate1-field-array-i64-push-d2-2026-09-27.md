# MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-D2

Status: closed — Decision accepted 2026-09-27
Parent: D22 series row 3 (selected caller cutover + retirement) in
`mirbuilder-exe-acceptance-suite-red-disposition-d21-2026-09-27.md`.
Follows D0 (owner design) / S0 (contract) / D1 (provider store
decision) / S1 (provider store implementation), all landed.

## Question

For the four `HakoAllocPage.seedBlocks` field-resident Array/I64
push sites: which callers still consume this membership's
omission/reconstruction path, what is the exact delete-set, and what
proves runtime owner acceptance before retirement?

## Census (worker + direct probes)

### Seam 1 — `src/mir/source_call_target/named_array_method.rs`

No membership-specific omission arm exists. Every `continue` /
`Ok(None)` is a shared boundary for still-uncovered populations:
non-`push` ArrayBox ops (get/set/has/length — line 64-65), pushes
outside an armed loop body (67-79 `resolved_loop_placement ==
Body` requirement), direct `me.field.push` receivers
(`detect_field_residence_claim` Ok(None), 165-169), non-ArrayBox or
undeclared fields, static-box callers outside ordinary coverage.
The FieldResidence arm (`issue_field_residence_requirement`
164-261) is family-level code shared by every future field-resident
site — not scoped to these four pushes.

### Seam 2 — `normal_callable_loop_source_route_items.rs` + `raw_loop_child_port.rs`

`uncovered`/`core_methods`/`SourceCallOutsideSelectedFamily` are
generic fail-fast boundaries shared with the static-publication
family. Nothing membership-specific exists. Nuance recorded:
`into_selected_relation` 313-316 lets uncovered items through when
a singleton static is selected — generic reconstruction stays
reachable there and is load-bearing for mixed loops; tightening is
a family-wide decision, not this slice.

### Seam 3 — `loop_body_lowering_associated_input.rs`

`exact_source_statement_call` (line 172-173 →
`take_source_array_push`) is consulted only inside loop-body plan
lowering — the sole consumption surface for push rows. The
`ArrayPush` arm (186-190 → `CoreEffectPlan::NamedArrayPush`) is the
typed retained owner, shared by Construction (Text) and
FieldResidence (I64) families. The generic receiver arms below it
serve the raw/legacy lanes, armed-lane uncovered items, env/user-
box/`me`/FieldAccess receivers — all shared.

### Lane finishing dispositions (probed directly)

- `into_parts` (`normal_default_root_final_validation.rs:64-88`)
  calls `reject_unretained_module` unconditionally and deliberately
  ignores the callable cohort (`let _callables`, pinned at
  `5a2dea9b3c`). Probes: a loop-position claimed push on the
  default `mir` lane and `--backend llvm` both stop at
  `[freeze:contract][named-array/retained-source-required]` —
  a designed typed rejection, not a leak.
- A straight-line `a.push(1)` on the same receiver mints **no row**
  (issuer loop-placement `continue`), lowers through the generic
  `array.write` path, and compiles — the designed coverage split.
  No residual-row leak exists because the row is never minted.
- `compile_normal_for_mir_json`/`into_artifact_parts` discharge via
  `take_named_array_emissions` → finalized handoff →
  `validate_named_arrays` (`normal_default_root_final_validation.rs:
  104-215, 237+`).
- `ExplicitCompatibility` route + retained markers → reject
  (`normal_default_pipeline.rs:623,696`) — designed.

### Runtime execution reachability (probed)

- `NamedArrayPush` emits a plain `MirInstruction::ArrayElementWrite`
  (not an Invoke); VM executes it receiver-origin-agnostically
  (`backend/mir_interpreter/handlers/array_write.rs`). Published
  transport emits `PublishedCallKindV1::ArrayPush` and the
  `FieldResidence` allocation resolves through the provider's
  `NewBox` + `Named` capability check
  (`map_named_allocations.rs:38-81`).
- In-tree EXE (`--emit-exe`) on a Holder fixture stops at
  `ordinary-new/local-commit/artifact-source-unavailable` (outer
  `new <UserBox>` in `Main.main` — the queued D18-successor scope),
  then `published-lifecycle/admission-function-not-birth` for a
  non-birth static `new`. Boxtorrent's first terminal on both lanes
  stays `static-result-ingress/foreign-lineage` at `Main.main`.

## D2 Decision

1. **Cutover for this membership is already complete.** The four
   sites have exactly one armed-lane owner — contract row →
   `take_source_array_push` → `NamedArrayPush` → plain
   `ArrayElementWrite` + retained marker → finalized-handoff
   `validate_named_arrays`. No second armed-lane writer exists for
   them; VM/legacy callers keep the shared generic path by design.
2. **Membership-scoped delete-set is empty.** All three D22
   candidate seams are shared boundaries; nothing deletable exists
   for this membership. The family-level delete-set (FieldResidence
   issuer arm, `FieldResidence` requirement/marker variants,
   provider table/recording state) is recorded for a future
   caller-zero closure that must also cover every other
   field-resident site and the Text family — not permitted now.
3. **Runtime owner = published/EXE lane** (`PublishedCallKindV1::
   ArrayPush` → `nyash.array.push_hi`/`checked_append_i64_v1`).
   In-tree positive execution of a field-resident push is blocked
   by queued upstream families (outer `new` artifact admission,
   lifecycle admission, static-result ingress). Row 3's runtime
   owner acceptance therefore stands as **dependency evidence**:
   the published-transport contract is proven in-tree
   (artifact-lane pins from S0/S1), while observable execution
   waits on the next selected dependency family — it is not folded
   into this slice and does not block the cutover verdict.
4. **Boundary pins encode the dispositions** (bounded S2):
   claimed-loop-push → `retained-source-required` on
   `compile_normal`; straight-line field-resident push → generic
   `array.write` with zero obligations (loop-placement split);
   existing document/artifact-lane discharge stays green.
5. Next selected row after S2 lands: the first observed app
   terminal `static-result-ingress/foreign-lineage` — its own
   design row (per "next observed required terminal").

## Bounded S2 (emitted)

`MIRBUILDER-GATE1-FIELD-ARRAY-I64-PUSH-S2` — boundary pins only:

- Pins in `named_array_source_tests.rs`:
  - claimed loop pushes → `compile_normal` typed reject
    `named-array/retained-source-required`;
  - straight-line field-resident push on `compile_normal` →
    compiles, `ArrayElementWrite` present, zero
    `named_array_write_obligations` (designed coverage split).
- `src/mir/source_call_target/README.md` — record the lane
  disposition table + empty membership delete-set verdict.
- No deletions, no new source admission, no runtime changes,
  no switch.

## Non-claims

- Family-level (FieldResidence/Text) retirement: deferred to a
  caller-zero census across all field-resident and Text sites.
- In-tree runtime execution of retained writes: blocked upstream.
- `into_selected_relation` 313-316 singleton-static uncovered
  reachability: unchanged (family-wide decision).
- `physical_program_json` LiteralAppend-only Invoke gate: unchanged.
- Gate 1 completion: not claimed.
