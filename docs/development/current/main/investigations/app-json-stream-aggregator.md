# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Latest fixed suite: 7PASS/4FAIL, actual EXE6 + empty marker1, no SKIP/timeout,
  runner335.599s at source-first-write commit; not rerun for this source-only stage.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --jobs 1 --timeout 1900 --format json`.
- Current CLI/runtime use the same quick graph: CLI64s/runtime6.45s, root test
  compile110s. Frozen matching pair/FFI rechecks JSON: MIR failure, same owner26.
- JSON still stops at `borrowed-entry/source-use-completion-pending`, owner26
  JsonStreamAggregator.ingestLine, called at line152 inside ingest Loop.
  Outer `source-loop-header-after` wraps this callee failure; no JSON EXE is claimed.
- statsFor BOTH results and stats.add need Map-slot support; Bool keeps exact reads.
- Other reds: untyped-field activation, binary-trees source-not-i64, boxtorrent child.

## Connected field and Map End
- Shared FieldResidence mode carries the original complete Array/Dynamic census
  through End, Invoke and publication; every Dynamic slot, including Empty, drains
  before outer End. Rust/C require the WHOLE reverse sequence, not a valid subset.
  Dynamic12 corroborates the same declaration/layout and retained inventory.
- ExactSlot cannot replace Dynamic. Signed I64 keeps original all-writer/read,
  SUM, finalized source image and both final scans. C shares prepare/commit/old-End/
  dispose: precommit Fault preserves the slot; committed old-End Fault never rolls back.
- Same lifetime/interval proof requires exact acquisition, field/mode/order and once
  on every path; no later writes/Calls or previous-lifetime credit. Schema refuses
  overlapping/unmarked drains and legacy Dynamic stores. Nonempty outer End refuses.
- Dynamic opaque outcomes still require the same archive's V3 field ABI. Public
  value/End-descriptor geometry is compared with LLVM even for V2 static input.
- Map and detached outcomes now lend ONE synchronous MapEndContext through Array
  children too. Synthetic errors enter the same frame at encounter time; complete
  callbacks' already-recorded Faults are not duplicated. InvalidContract stays fatal.
- Indexed install retains the SAME complete-End descriptor and layout helper;
  preflight keeps caller Home, complete End runs outside locks and releases outer.
- Numeric/NoHook callers stay separate; canonical C requires complete End.
- Birth now connects source-issued first-write I64/Map and Normal-committed Dynamic
  prefixes. Whole-field census grants no early Home; Provided stays caller-owned
  until Birth Normal. Provided/nullable Dynamic partial cleanup remains unadmitted.

## Validation and retained failures
- Invoke42/construction4/prefix1/Provided1/Map17/binding8+archive1/C14 PASS.
  Two ignored tests excluded; no previous product failures waived.
- Native C field ABI PASS in both profiles (Map SafeMutex only), including repeated
  first-write refusal and preservation of a caller-owned candidate on occupied slot.
- Three original-source canonical EXEs (Map-first, scalar-first, mixed fixed Array)
  build/run exit30 with the final matching pair; source/final tests use no fallback.
- Prior Map/scalar Fault4 and mixed-Array Fault2 exit70/Fault101/reclaim. The
  mixed-Array first-store Fault also rechecked with prior frozen pair: same result;
  C emits candidate release on that edge. Array holder/contents/End remains owed.
- Current core Map15/kernel Map17/binding8+actual archive1/native both profiles PASS.
  Existing C case adds necessary presence/typed-borrow refusal and unchanged-out
  boundaries; the same child still Ends once. Zero-match extra filter excluded.
  No new test entry/file, checker, guard or production test-only hook.
- Owed prior reds: brand9, initializer2, terminal2, callback1, plan1, admission2,
  read-claim2, composition2, scalar1. Earlier frozen comparisons do not waive them.

## Next source connection and remaining contracts
- Stable Map placement now belongs to the SAME Dynamic field/outcome owner.
  Published scalar/Text/object/Array and native Map are a private move-only sum;
  Map has no public Dynamic tag. Dedicated SafeMutex install/borrow use stable bytes.
- Fallible allocation/validation precede source move under the SAME destination guard.
  Rejection keeps caller Home; Normal leaves source Unissued. End/dispose free only
  after Map Normal/Fault, never InvalidContract; generic get/replace refuse Map.
- Original ConstructionStore selects first-write mode; retained construction replay
  lends the same final store coordinates to existing source image/final consumers.
  Runtime Empty is checked under the commit guard, including repeated Birth.
- C uses the existing prepare/commit/outcome-End/dispose owner and Birth markers:
  Map candidate transfers only on Normal; precommit Fault ends it then the original
  committed prefix, including scalars, in actual commit order. Full End stays reverse
  declaration order. Canonically source-qualified fixed Array keeps its prior path
  across construction, owned census, physical ABI and final End; no subset census.
- Same CheckedMap now lends kind-independent presence and class-qualified indexed
  identity, without Home/claim/clone/retain. Map guard ends before object validation.
  Scalar, Array-length and new native reads share admission; former narrow module
  is replaced. No second source/result registry or general Call/Return permission.
- Original field Map and Text key now jointly close the SAME Core source before
  solving; exact Me/field and sole Map Birth writer retained, no Map SSA/entry grant.
  Manifest MapKeyPresence returns Bool independently of entry kind. Existing source
  case adds sole-writer/provider/key negatives: independent joint-premise boundary.
  Source family2/target7/source-ledger9/codegen14 PASS; zero-match filter excluded.
- Next: connect has Bool condition to physical writer and BOTH final consumers;
  then statsFor BOTH returns as Map-held borrows through stats.add/users.push.
  Retain exact has/get/set/escape uses and slot/key/class support through last use;
  field-assignment census alone is NOT all Map writers.
- UserStats Text materialization (independent value, not copied input Home), Box
  Provided initialization, Shared/carrier and full Dynamic source
  activation remain owed. Array holder/contents/End, exact empty-transfer census,
  canonical writer, signed SUM/rebind/to_i64 and original source/physical reds remain.
  Existing opaque draft stays unselected; no parallel inventory.
## Completion limits
- mimalloc PASS includes removal of an obsolete generic-JSON all-Invoke refusal
  precheck; direct EXE already worked. Publication tests do not prove that same
  mimalloc closure or every subtype. Preserve this harness/capability distinction.
- App completion needs every smoke contract and real output/exit. Full goal also
  needs production cutover, mandatory source/consumer closure and physical retirement.
  Retire replaced exclusive edges with proof; retain shared owners and safety checks.
