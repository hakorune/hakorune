# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Latest fixed suite: 7PASS/4FAIL, actual EXE6 + empty marker1, no SKIP/timeout,
  runner408.299s. mimalloc148.397s, allocator171.051s, typed21–22s.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --jobs 1 --timeout 1900 --format json`.
- Selected CLI `target/quick/hakorune`, lifecycle runtime `target/lifecycle-kernel/quick`,
  default/private C FFI agree. CLI92s/retirement28s, focused67s/run0.05s, runtime268s.
- JSON still stops at `borrowed-entry/source-use-completion-pending`, owner26
  JsonStreamAggregator.ingestLine, called at line152 inside ingest Loop.
  Outer `source-loop-header-after` wraps this callee failure; no JSON EXE is claimed.
- Body5 statsFor result and Body6 stats.add need Map-slot support. Bool condition
  uses the original exact read/producer owner; canonical probe prints yes/1/Result0.
- Other product reds: untyped-field source activation, binary-trees source-not-i64,
  boxtorrent stored-child receiver. These remain incomplete.

## Connected source and execution

- One shared FieldResidence mode carries Array/Dynamic from the original complete
  declaration census through the sealed inventory, End body, Invoke and publication.
  Every Dynamic field, including Empty, requires a reverse-order drain before outer End.
  Replaced static-execution wrapper retired; no extra Invoke variant or source classifier.
- Structural final verification requires the WHOLE End sequence, not a valid subset.
  Dynamic12 publication corroborates the same canonical declaration/layout and inventory.
  ExactSlot cannot stand in for Dynamic; signed I64 replacement keeps the original
  all-writer/read and SUM proof, same finalized source image and both final scans.
- C uses the same prepare/commit/detached-End/dispose owner for I64 replace and take.
  Get writes only on Normal. Precommit Fault preserves the slot; committed End Fault
  does not roll back. Function-entry placement is reused across Loop iterations.
- Complete helper walks Array/Object/Dynamic together in reverse declaration order,
  continues cleanup after End Fault, then calls explicit completed-fields outer release.
  Runtime still rejects nonempty Dynamic slots; outer release never repairs a missed End.
- Ordinary Dynamic outer cleanup reuses the existing interval proof: SAME acquisition,
  exact mode/field/child, once on every path, reverse order, no write/Call after drain.
  Source Birth Normal/received Call Normal must dominate; no previous lifetime credit.
  Schema rejects extra unmarked drains, overlapping marks and legacy Dynamic stores.
- Same selected archive supplies field ABI via the V3 session; Dynamic needs its
  admitted target geometry. Static V2 input keeps its old path.
- Constructor partial cleanup remains CLOSED for Dynamic: complete field requirements
  do not prove ownership before whole Birth Normal. Provided Homes remain caller-owned.
  Nullable received Dynamic cleanup also remains unadmitted; no broad if-live claim.

## Validation and retained failures

- Existing Invoke42/JSON35/root-order14/construction3/fault-cleanup2/matrix1/source-definition6PASS.
  Existing C interval case PASS; actual runtime field ABI PASS in both storage profiles.
- Physical-only two-Dynamic-slot ABI inputs compile/run with exit30 in both profiles.
  Eleven malformed inputs reject; missing target field ABI rejects before artifact.
  These mutations are consumer checks, not source acceptance or recursive Dynamic End proof.
- Existing source-issued owned-slot execution: 50 cases PASS, including all seven
  store Faults and recursive static End, with both prior and current FFI/runtime pairs.
  Harness now observes checked residence stores and initializes exact residence state.
- Existing V4 execution suite PASS, including 14 negative inputs/artifact preservation.
  Its stale Add negative now uses a defined Home receiver; guarded TAGGED arithmetic
  stays valid. Refusal expectation/case count remain unchanged.
- Minimal existing-case extensions protect independent Dynamic completeness/mode,
  interval invalidation/order and nonempty outer-release boundaries. No new test
  function/file, checker, guard or test-only production hook.
- Brand family66PASS/9FAIL; the older frozen binary has the same nine failures plus
  one additional failure. These remain owed; comparison is not a baseline waiver.
- Prior initializer79/2, terminal6/2, callback0/1, plan5/1, admission2/2,
  read-claim7/2, composition9/2 and scalar15/1 failures remain. Full history is in Git.

## Next connection and shared remaining contracts

- Read-only review: field-owned Map placement comes BEFORE the Map-held result bridge.
  `me.stats = new MapBox()` needs parent-lifetime stable aligned owning placement;
  storing a Birth stack pointer or inventing an opaque Map tag grants no permission.
  Reuse CheckedMap/MapStorage, storage_move, original Map progress and field transaction.
- Map-owned objects need the SAME complete-End descriptor. Existing residence End
  lacks frame/site context; lend it through the original detached-entry/Map End chain,
  run callbacks outside locks and preserve the already-recorded first Fault.
- statsFor BOTH returns are Map-held nonowning objects: hit get and miss set-then-return.
  Set commit moves Home into the slot. Precommit Fault keeps caller ownership;
  postcommit old-End Fault keeps Map ownership. Miss is never Fresh/HomeToCaller.
- Borrow support must retain the same Map residence/key/entry/class through last use,
  including stats.add effects and independence from users.push. Map live alone is
  insufficient; no clone/retain, invented result origin or literal-I64 terminal reuse.
- UserStats Text initialization/Provided fields and Birth Fault ownership remain
  separate requirements. Shared/carrier and full Dynamic source activation remain owed.
- Array holder/contents/End, exact empty-transfer census, canonical writer, original
  signed SUM/rebind/to_i64 and remaining source/physical failures stay in the finite goal.
  Existing opaque draft remains unselected; do not create a parallel inventory.

## Completion limits

- mimalloc PASS includes removal of an obsolete generic JSON all-Invoke refusal
  precheck; direct EXE already worked. Publication tests do not cover every subtype
  or that same mimalloc closure. Preserve this harness/capability distinction.
- Earlier Text input/Return/Call holds, Normal/Fault cleanup and Bool owners remain;
  history stays in Git. Retirement preserves CLI text/rodata; only source lines/build ID change.
- App completion needs every existing smoke contract and real output/exit. Full goal
  also needs production cutover, mandatory source/consumer closure and physical retirement.
  Retire replaced exclusive edges with proof; preserve shared owners and safety checks.
