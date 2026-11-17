"""Parent-only, serialized compiler invocations in an isolated target directory."""
import json
import os
import re
from pathlib import Path
import subprocess
import time
from support import ROOT, build_env, write, sha, configured_env, wait_guarded
from resources import active


def invoke(work, label, command, extra_env=None, timeout_seconds=7200):
    env = build_env(work) | (extra_env or {})
    env.pop('CARGO_ENCODED_RUSTFLAGS', None)
    start = time.monotonic()
    with (work / (label + '.txt')).open('w') as out, (work / (label + '.log')).open('w') as err:
        child = active().launch(command, cwd=ROOT, env=env, stdout=out, stderr=err)
        code, timeout, resource_stop, peak = wait_guarded(child, timeout_seconds)
    result = dict(command=command, exit=code, resource_stop=resource_stop, timeout=timeout, peak_rss_kib=peak, seconds=time.monotonic() - start,
                  output=str(work / (label + '.txt')), stderr=str(work / (label + '.log')))
    write(work / (label + '.command.json'), result)
    if resource_stop:
        raise MemoryError(f'{label}: memory guard stopped compiler')
    if child.returncode:
        raise RuntimeError(f'{label}: compiler failed; see preserved logs')
    return result


def cargo(work, verb):
    return ['cargo', verb, '--profile', 'quick', '--target-dir', str(work / 'target'),
            '--message-format=json', '--config', 'build.rustc-wrapper=""']


def json_messages(path):
    # Rust dumps and Cargo JSON share stdout; unrelated dump lines are not diagnostics.
    for line in Path(path).open():
        if not line.startswith('{"reason":'):
            continue
        try:
            yield json.loads(line)
        except json.JSONDecodeError:
            continue


def build_test(work, label='test-build', extra_env=None, timeout=7200):
    invoke(work, label, cargo(work, 'test') + ['--lib', '--no-run'], extra_env, timeout)
    candidates = [x['executable'] for x in json_messages(work / (label + '.txt'))
                  if x.get('reason') == 'compiler-artifact' and x.get('executable')
                  and x['target']['name'] == 'nyash_rust']
    if len(candidates) != 1:
        raise RuntimeError('test artifact identity missing or ambiguous')
    write(work / 'test-binary.json', dict(path=candidates[0], sha256=sha(candidates[0])))
    return candidates[0]


def test(work, template, expected_failures, binary=None, timeout=7200):
    binary = binary or build_test(work)
    directory = work / 'T'
    directory.mkdir(exist_ok=True)
    for raw in directory.glob('*.profraw'):
        raw.unlink()
    env = configured_env(template, work / 'target/quick/hakorune', work)
    env['LLVM_PROFILE_FILE'] = str(directory / 'test-%p-%m.profraw')
    env['CARGO_BUILD_JOBS'] = '2'
    env['CARGO_TARGET_DIR'] = str(work / 'target')
    start = time.monotonic()
    with (directory / 'lib.log').open('w') as stream:
        child = active().launch([binary, '--test-threads=1'], cwd=ROOT, env=env,
                                stdout=stream, stderr=subprocess.STDOUT)
        code, timed_out, resource_stop, peak = wait_guarded(child, timeout)
    if resource_stop:
        raise MemoryError('memory guard stopped test; profile is incomplete')
    content = (directory / 'lib.log').read_text()
    completed = re.search(r'^test result: (?:ok|FAILED)\. .*?; finished in [^\n]+\n*\Z', content, re.M)
    if timed_out or code < 0 or not completed:
        write(directory / 'result.json', dict(complete=False, exit=code, timeout=timed_out, seconds=time.monotonic()-start))
        raise RuntimeError('test process did not finish; coverage is incomplete')
    failure_block = content.rsplit('\nfailures:\n', 1)[-1].split('\ntest result:', 1)[0]
    failures = sorted(line.strip() for line in failure_block.splitlines() if line.strip()) if child.returncode else []
    expected = sorted(Path(expected_failures).read_text().splitlines())
    result = dict(complete=True, binary=binary, binary_sha256=sha(binary), exit=child.returncode,
                  seconds=time.monotonic() - start, failures=failures,
                  new=sorted(set(failures) - set(expected)), missing=sorted(set(expected) - set(failures)),
                  failure_names_equal=failures == expected)
    write(directory / 'result.json', result)
    return result
