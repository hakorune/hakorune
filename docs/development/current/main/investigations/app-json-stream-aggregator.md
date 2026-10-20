# APP-json-stream-aggregator

Status: User-requested pause for design discussion; JSON WIP retained. Date: 2026-10-21
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md), [reviewed simplification tasks](mirbuilder-structure-refactor-queue-d0-2026-08-23.md).

## Product evidence

- Latest fixed suite: 7PASS/4FAIL, actual EXE6 + empty marker1, no SKIP/timeout,
  runner324.680s at committed receiver-source change; current WIP suite not run.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --jobs 1 --timeout 1900 --format json`.
- WIP CLI: JSON stops owner26 Body5 statsFor at lexical-instance-call/source-pending;
  add field RHS/Bool condition and original Scalar actual-use census connect. No EXE.
- CLI build40.80s and C syntax-only PASS. WIP root tests, C shared library, runtime
  and suite unrun; prior checks are not WIP evidence. Other reds remain unchanged.

## Retained connected contracts

- One FieldResidence census carries every Array/Dynamic field, including Empty,
  through End/Invoke/publication. Whole reverse declaration drain precedes outer
  End; exact slot cannot stand in for Dynamic, no subset/reused-lifetime credit.
- Signed I64 keeps all original writer/read, SUM, finalized source and both scans.
  C shares prepare/commit/old-End/dispose; precommit keeps slot/caller Home,
  postcommit old-End Fault never rolls back. Nonempty outer End refuses.
- One synchronous MapEndContext covers detached outcomes and Array children;
  synthetic Faults enter at encounter, recorded callback Faults are not duplicated,
  InvalidContract is fatal. Complete End runs outside Map/object locks.
- First-write I64/Map uses original ConstructionStore/Empty guard; Birth cleanup
  follows committed prefix, full End reverse declaration. Fixed Array retains full
  census. Provided stays caller-owned until Normal; partial cleanup remains owed.
- Native Map: private stable move-only storage, no public Dynamic tag or
  Home/claim/End/Call/Return grant. has/get check field before Text; missing-only
  get Normal/0, exact class borrows identity, wrong kind/class Faults, non-Normal
  keeps out unchanged. Both native profiles PASS; SafeMutex only, child End once.
- Bool has shares Text comparison's scalar writer and both final consumers,
  retaining exact Me/input-use and complete original Map Birth replay. Duplicate
  operand/writer retired. Passive literal/formal Text/short-circuit source EXEs3
  exit7. Computed key effects stay unadmitted until receiver-first execution closes.
- Scalar Return source census selects demand independently of proof success;
  finite input alone grants no result class, Text stays separate.

## Current source connection

- Result PassB retains SAME field/constructor/entry before SUM. Array/Map share
  the original whole field-use walk; Array keeps selector/i64 policy. All original
  declarations/Birth, aliases/escape/foreign/captured uses and mutators are closed.
- statsFor hit has/get retain same field/key and exact true-arm Return. Miss keeps
  exact New/set/Return plus disjoint Array push; unknown intervening effects refuse.
- SAME LOCAL support separates FormalForward from MapSlot: original caller Me
  holder and key actual substitute independently; every Return root retains identity.
  Map formal root/ordinal are None. Generic composition still refuses Map; old
  formal-only ABI issuers reject it or keep pending. No Home/ABI/liveness granted.
- Map key selects SAME SourceInstance before domain/Completion success. Its exact
  key Binding uses the existing expression/TextResidence qualification callback;
  no legacy incoming, raw Binding proof or Text/Integer relabeling is fabricated.
- Shared Static Text/Bool loan retains ExactBool target, all actuals/Completion,
  exact publication/scalar writer and Rust/C Text-domain/null-receiver checks.
  Document verification borrows the same final root; source-less API stays strict.
- Map-only LOCAL receiver keeps the original payload class and same slot loan,
  independent holder/key and whole Return source. Incoming rechecks this loan;
  sole initializer, unrebound/uncaptured binding and exact target/arguments apply.
- SourceInstance retains Scalar actual kind without raw-I64 transport. Existing
  co-seal, object/borrowed/mixed finishers and Unit publication keep Map pending
  until slot ABI/lifetime closes. Unit Completion alone grants no receiver packet.

## Verification and retained reds

- Static Bool Local in indexed object context: true/false EXEs exit7/3; parser,
  Invoke42, Bool result2 previously PASS. Current publication1/prefix1/JSONsource1,
  nullable2/stored2 PASS; existing cases/API reused without a new assertion.
  Actuals82PASS/4FAIL unchanged; result23/1 reproduced with saved prior binary.
  All failures stay owed; no safety boundary or expected error was weakened.
- Prior Array15/2, publication3/1, result23/1, nullable publication2/1 remain owed.
- Earlier physical/source/Fault checks remain in Git; zero/ignored filters excluded.
- Prior Map/scalar Fault4 + mixed-Array Fault2: exit70/Fault101/reclaim and
  first-store candidate release checked with prior frozen pair; no red waived.
- Other owed reds: brand9, initializer2, terminal2, callback1, plan1, admission2,
  read-claim2, composition2, scalar1. Keep their original source/physical contracts.

## Next and full completion limits

- PAUSED: 38 implementation/doc paths kept uncommitted; no new test/assert/checker.
  WIP normalizes Scalar/SUM Bool, shared finite input and Assignment/condition
  references, plus C Bool view schema/producer/prologue. No legacy Integer/Object grant.
- Resume: review/focused-verify WIP; final Rust operand scans must retain exact
  source-owned Field RHS/Bool prefix+outer coordinates and every tracked operand.
  Do not drop scalar roots by domain; C Bool view still needs execution validation.
- Map slot result ABI/get+set Return/lifetimes, candidate Home transfer on Normal,
  Fault cleanup, complete UserStats End and users.push execution remain mandatory.
- UserStats Text must materialize independently, not copy input Home. Provided Box,
  nullable Dynamic, Shared/carrier and full Dynamic activation remain owed.
- Array holder/contents/End, exact empty-transfer census, canonical writer, signed
  SUM/rebind/to_i64 and source/physical prior reds remain. Direct terminal Text Call
  lacks original argument source; non-Text key inference panic is not a valid refusal.
- Unboxed native Bool cohort and generic canonical-JSON type profile still refuse.
- mimalloc PASS removed obsolete generic-JSON Invoke precheck; EXE already worked.
  Publication tests do not prove whole closure/every subtype.
- App completion requires all smoke contracts and real EXE/output/exit; full goal
  requires source/consumer closure, production cutover and physical retirement.
  Retire replaced exclusive edges; retain shared owners/safety checks.
