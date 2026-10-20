# APP-json-stream-aggregator

Status: JSON selected; common untyped-storage target accepted. Date: 2026-10-20
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence

- Latest fixed suite: 7PASS/4FAIL, actual EXE6 + empty marker1, no SKIP/timeout,
  runner342.642s with this Bool/source/final-consumer change; no previous red waived.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --jobs 1 --timeout 1900 --format json`.
- Current quick CLI35.39s/runtime7.02s, root test build69s. Frozen matching
  pair/FFI rechecked JSON: MIR failure, same owner26; no app PASS claimed.
- JSON still stops at `borrowed-entry/source-use-completion-pending`, owner26
  JsonStreamAggregator.ingestLine, called at line151 inside ingest Loop.
  Outer `source-loop-header-after` wraps this callee failure; no JSON EXE is claimed.
- statsFor BOTH results and stats.add need Map-slot support; Bool keeps exact reads.
- Other reds: untyped-field activation, binary-trees source-not-i64, boxtorrent child.

## Connected field and Map End
- Shared FieldResidence mode carries the original complete Array/Dynamic census
  through End, Invoke and publication; every Dynamic slot, including Empty, drains
  before outer End. Rust/C require the WHOLE reverse sequence, not a valid subset.
- ExactSlot cannot replace Dynamic. Signed I64 keeps original all-writer/read,
  SUM, finalized source image and both final scans. C shares prepare/commit/old-End/
  dispose: precommit Fault preserves the slot; committed old-End Fault never rolls back.
- Same lifetime/interval proof requires exact acquisition, field/mode/order and once
  on every path; no later writes/Calls or previous-lifetime credit. Schema refuses
  overlapping/unmarked drains and legacy Dynamic stores. Nonempty outer End refuses.
- Map and detached outcomes now lend ONE synchronous MapEndContext through Array
  children too. Synthetic errors enter the same frame at encounter time; complete
  callbacks' Faults are not duplicated; InvalidContract stays fatal.
- Birth now connects source-issued first-write I64/Map and Normal-committed Dynamic
  prefixes. Whole-field census grants no early Home; Provided stays caller-owned
  until Birth Normal. Provided/nullable Dynamic partial cleanup remains unadmitted.

## Validation and retained failures
- Invoke42/construction4/prefix1/Provided1/Map17/binding8+archive1/C14 PASS.
  Two ignored tests excluded; no previous product failures waived.
- Prior Map-first/scalar-first/mixed Array source EXEs run exit30, no fallback.
- Prior Map/scalar Fault4 and mixed-Array Fault2 exit70/Fault101/reclaim. The
  mixed-Array first-store Fault also rechecked with prior frozen pair: same result;
  C emits candidate release on that edge. Array holder/contents/End remains owed.
- Core Map15/kernel Map17/binding8+archive1/native both profiles PASS.
  Existing C case keeps typed-borrow/out/Fault boundaries and once-only child End.
  No new test entry/file, checker, guard or production test-only hook.
- Owed prior reds: brand9, initializer2, terminal2, callback1, plan1, admission2,
  read-claim2, composition2, scalar1. Earlier frozen comparisons do not waive them.

## Next source connection and remaining contracts
- Stable Map placement now belongs to the SAME Dynamic field/outcome owner.
  Published scalar/Text/object/Array and native Map are a private move-only sum;
  Map has no public Dynamic tag. Dedicated SafeMutex install/borrow use stable bytes.
- Under the destination guard, fallible validation precedes move; rejection keeps
  caller Home, Normal leaves Unissued. End/dispose follows Map Normal/Fault only.
- Original ConstructionStore selects first-write mode and lends the retained final
  store coordinates; runtime Empty is checked under commit guard, even repeated Birth.
- C uses the existing prepare/commit/outcome-End/dispose owner and Birth markers:
  Map candidate transfers only on Normal; precommit Fault ends it then the original
  committed prefix, including scalars, in actual commit order. Full End stays reverse
  declaration order. Canonically source-qualified fixed Array keeps its prior path
  across construction, owned census, physical ABI and final End; no subset census.
- Synchronous field-has borrows field before shared Text decoding, then same Map
  presence. Existing C case extends the independent Text lease/out/alias boundary;
  native both profiles/binding8+archive1 PASS (Map SafeMutex only), no new entry.
  Source/final consumers retain receiver-before-key for admitted passive Text keys.
- Original field Map and Text key now jointly close the SAME Core source before
  solving; exact Me/field and sole Map Birth writer retained, no Map SSA/entry grant.
  Manifest MapKeyPresence returns Bool independently of entry kind. Existing source
  case adds sole-writer/provider/key negatives: independent joint-premise boundary.
- Bool has now shares scalar-invocation input/writer/final replay with Text compare;
  the duplicate operand wrapper is retired. Same original Me read and Map Birth
  provider/store are replayed; both final consumers retain typed borrowed Text use.
  Literal/formal Text/short-circuit source EXEs build/run exit7 with frozen pair.
  Borrowed-use27/core-source2/scalar-loan1/terminal8/selected-route1/loop2 PASS;
  existing Invoke42/expression4/Text-condition1 also PASS. Zero filters excluded.
- Finite input alone cannot classify results: SAME original scalar Return census
  selects demand independently of scalar proof success. JSON exposed and verified
  correction of an overbroad request; Text reportUser keeps its own result owner.
- Next: statsFor BOTH returns as Map-held borrows through stats.add/users.push.
  Extend existing result witness/borrowed-local graph with same field/key/class
  and full operation/holder interval support, never fake formal0/Fresh/HomeToCaller.
  Set Normal moves Home; precommit keeps candidate cleanup; old-End Fault never
  rolls back. UserStats full End/users.push remain obligations.
  Retain exact has/get/set/escape uses and slot/key/class support through last use;
  field-assignment census alone is NOT all Map writers.
- Direct terminal Call with Text actual still lacks original argument source;
  non-Text key probe panics in type inference, not a verified refusal. Neither waived.
- UserStats Text materialization (independent value, not copied input Home), Box
  Provided initialization, Shared/carrier and full Dynamic source
  activation remain owed. Array holder/contents/End, exact empty-transfer census,
  canonical writer, signed SUM/rebind/to_i64 and original source/physical reds remain.
## Completion limits
- mimalloc PASS includes removal of an obsolete generic-JSON all-Invoke refusal
  precheck; direct EXE already worked. Publication tests do not prove that same
  mimalloc closure or every subtype. Preserve this harness/capability distinction.
- App completion needs every smoke contract and real output/exit. Full goal also
  needs production cutover, mandatory source/consumer closure and physical retirement.
  Retire replaced exclusive edges with proof; retain shared owners and safety checks.
