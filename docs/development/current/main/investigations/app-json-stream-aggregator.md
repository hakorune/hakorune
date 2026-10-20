# APP-json-stream-aggregator

Status: Original Static Return domains and Map Return sources connected; Map execution incomplete. Date: 2026-10-21
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md), [reviewed simplification tasks](mirbuilder-structure-refactor-queue-d0-2026-08-23.md).

## Product evidence

- Frozen fixed suite: 7PASS/4FAIL, actual EXE6 + empty marker1, 401.795s; no SKIP/timeout.
- Command: `tools/smokes/v2/run.sh --profile integration --owner-profile integration
  --suite real-apps-exe-boundary --jobs 1 --timeout 1900 --format json`.
- JSON now refuses `ordinary-new/object-input/callee-exit` with
  `PrefixNotCovered(Body2)` before MIR/EXE. Callee is absent from this diagnostic;
  statsFor set and ingestLine condition are candidates, not confirmed source lines.
- CLI40.30s/root95s PASS; unchanged runtime/C pair reused. Source-domain proof
  grants no native result ABI; the separate Reader/Sink EXE still refuses entry.

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
- Current JSONsource1, object-return5, borrowed-use27, Map terminal13, nullable2 PASS.
- Initializer7/1, entry64/2, source33/8, actuals82/4, result49/3 match parent failures
  and diagnostics exactly except compilation session IDs. These reds remain owed.
- Existing case extensions protect an independent source-only Static Return domain
  when catalog authority is unavailable, and SAME whole Map-source loan identity.
  No new test/function/file/hook/checker/guard/card; inventory9263 unchanged.
- Prior Invoke42, publication2/1 and Array15/2 remain prior evidence, not rerun here.
- Prior native Map/scalar/Array Fault evidence remains; no runtime/C changes here.
- Other owed reds: brand9, initializer2, terminal2, callback1, plan1, admission2,
  read-claim2, composition2, scalar1. Keep original source/physical contracts.

## Next and full completion limits

- LANDED: result kind/physical role share Formal or ReceiverFieldSlot support; JSON has one encoder and unchanged Formal wire.
  Map remains SourcePending until physical get/set Normal/Return/lifetime closes.
- LANDED: SAME whole Map Return source reaches terminal Home observation and
  entry-use closure; each Present get retains its own receiver/read and Text key.
  Miss set remains unavailable; selected storage never falls through to Fresh/scalar.
- LANDED: original Static callee Returns feed the existing SUM/SCC graph once per
  callee. Every incoming Rc/target/contract and sealed omitted mapping is checked;
  complete Return/flow identity retained, independent of old catalog result success.
- Next: identify callee in the current Body2 refusal, then connect original get/set
  physical Normal/Return/slot lifetime; preserve receiver-first Text-key evaluation.
- Next set: reuse TransferHome Normal delta and InstallIndexed/EndOutcome cleanup;
  after commit retain a slot-supported payload, never the old candidate Home.
- Map slot result ABI/get+set Return/lifetimes, candidate Home transfer on Normal,
  Fault cleanup, complete UserStats End and users.push execution remain mandatory.
- UserStats Text must materialize independently, not copy input Home. Provided Box,
  nullable Dynamic, Shared/carrier and full Dynamic activation remain owed.
- Array holder/contents/End, exact empty-transfer census, canonical writer, signed
  SUM/rebind/to_i64 and source/physical prior reds remain. Direct terminal Text Call
  lacks original argument source; non-Text key inference panic is not a valid refusal.
- Unboxed native Bool cohort and generic canonical-JSON type profile still refuse.
- App completion requires all smoke contracts and real EXE/output/exit; full goal
  requires source/consumer closure, production cutover and physical retirement.
  Retire replaced exclusive edges; retain shared owners/safety checks.
