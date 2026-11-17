"""M-3 guarded build (one resource retry), test run, and one combined profile."""
from pathlib import Path
import gzip
import json
import time
from compiler import build_test, test
from coverage import merge, guarded, read_lcov
from support import FLAGS, llvm, sha, write
from resources import active


def authorized_retry():
    watch = active()
    if watch.failure and watch.failure['free_disk_bytes'] < 20 * 1024**3:
        raise MemoryError('disk threshold stopped the whole run; no retry')
    with watch.lock:
        if watch.children:
            raise RuntimeError('retry cannot start while earlier children remain')
        row = watch.check()
        if row['available_kib'] < 6 * 1024**2 or row['free_disk_bytes'] < 20 * 1024**3:
            raise MemoryError('resources have not recovered; no retry started')
        watch.failure = None  # Explicit one-retry authorization in measure-1b.
        watch.metrics.write(json.dumps(row | {'AUTHORIZED_TEST_RETRY': True}) + '\n')


def measure(work, template, expected_failures, deadline):
    watch = active()
    cli = work / 'target/quick/hakorune'
    identity = sha(cli)
    status_path = work / 'T-status.json'
    if status_path.exists():
        held = json.loads(status_path.read_text())
        if held.get('measured'):
            assert held['cli_sha256'] == identity
            assert sha(held['binary']) == held['binary_sha256']
            assert sha(work / 'T/membership.json.gz') == held['membership_sha256']
            assert sha(work / 'T/all.lcov') == held['lcov_sha256']
            return True
    write(work / 'T-status.json', dict(measured=False, reason='M3 has not completed', cli_sha256=identity))
    history = work / 'test-attempts.json'
    records = json.loads(history.read_text()) if history.exists() else []
    built = next((r for r in reversed(records) if r.get('status') == 'built'), None)
    binary = built['binary'] if built else None
    if built:
        assert sha(binary) == built['binary_sha256'], 'recorded test artifact changed'
    stops = sum(r.get('status') == 'resource_stop' for r in records)
    for attempt in ([] if binary or stops >= 2 else range(stops + 1, 3)):
        if time.time() >= deadline:
            break
        watch.stage = 'M3-build-' + str(attempt)
        override = None if attempt == 1 else dict(
            RUSTFLAGS=FLAGS.replace('codegen-units=4', 'codegen-units=64'),
            CARGO_BUILD_JOBS='1', CARGO_INCREMENTAL='0')
        try:
            binary = build_test(work, 'test-build-' + str(attempt), override, max(1,min(7200,deadline-time.time())))
            assert sha(cli) == identity, 'test build replaced measurement CLI'
            records.append(dict(attempt=attempt, status='built', binary=binary, binary_sha256=sha(binary), override=override))
            break
        except MemoryError as error:
            records.append(dict(attempt=attempt, status='resource_stop', reason=str(error)))
            write(work / 'test-attempts.json', records)
            if attempt == 2:
                break
            try:
                authorized_retry()
            except MemoryError as error:
                records.append(dict(status='retry_not_started', reason=str(error)))
                break
    write(work / 'test-attempts.json', records)
    if binary is None:
        write(work / 'T-status.json', dict(measured=False, reason='guarded builds unavailable', attempts=records))
        return False
    if time.time() >= deadline:
        write(work / 'T-status.json', dict(measured=False, reason='measurement deadline before test', attempts=records))
        return False
    watch.stage = 'M3-test'
    try:
        result = test(work, template, expected_failures, binary, max(1, min(7200, deadline - time.time())))
    except (MemoryError, RuntimeError) as error:
        write(work / 'T-status.json', dict(measured=False, reason=str(error), attempts=records))
        return False
    assert sha(cli) == identity, 'test build/run must not replace the measurement CLI'
    if deadline - time.time() < 1800:
        write(work / 'T-status.json', dict(measured=False, reason='less than worst-case 1800s profile-collapse allowance', attempts=records))
        return False
    directory = work / 'T'
    raws = list(directory.glob('*.profraw'))
    profile = directory / 'all.profdata'
    merge(raws, profile)
    common = [str(binary), '-object=' + str(cli), '--instr-profile=' + str(profile), '--skip-expansions']
    lcov = directory / 'all.lcov'
    guarded([llvm('cov'), 'export', *common, '--format=lcov'], lcov)
    functions, lines, universe = read_lcov(lcov)
    with gzip.open(directory / 'membership.json.gz', 'wt') as stream:
        json.dump(dict(functions=sorted(functions), lines=lines), stream, separators=(',', ':'))
    guarded([llvm('cov'), 'export', *common], directory / 'coverage.json')
    profile.unlink()
    for raw in raws:
        raw.unlink()
    baseline = set(Path(expected_failures).read_text().splitlines())
    status = dict(measured=True, failures_subset_of_155=set(result['failures']) <= baseline,
                  new_failures=sorted(set(result['failures']) - baseline), attempts=records,
                  observed_functions=len(functions), binary=binary, binary_sha256=sha(binary), cli_sha256=identity,
                  membership_sha256=sha(directory / 'membership.json.gz'), lcov_sha256=sha(lcov))
    write(work / 'T-status.json', status)
    return True
