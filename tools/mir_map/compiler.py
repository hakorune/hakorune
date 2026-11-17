"""Parent-only, serialized compiler invocations in an isolated target directory."""
import json
import os
from pathlib import Path
import subprocess
import time
from support import ROOT, build_env, write, sha, configured_env, wait_guarded


def invoke(work, label, command, extra_env=None):
    env = build_env(work) | (extra_env or {})
    env.pop('CARGO_ENCODED_RUSTFLAGS', None)
    start = time.monotonic()
    with (work / (label + '.txt')).open('w') as out, (work / (label + '.log')).open('w') as err:
        child = subprocess.Popen(command, cwd=ROOT, env=env, stdout=out, stderr=err, start_new_session=True)
        code, timeout, resource_stop, peak = wait_guarded(child, 7200)
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


def build_cli(work):
    return invoke(work, 'build', cargo(work, 'build') + ['--bin', 'hakorune'])


def dumps(work):
    results = []
    for label, flags in [('m2-forced', ['-Zno-codegen', '--force-warn', 'dead_code', '--force-warn', 'unused']),
                         ('hir', ['-Zunpretty=hir-tree']), ('thir', ['-Zunpretty=thir-tree'])]:
        results.append(invoke(work, label, cargo(work, 'rustc') + ['--lib', '-p', 'nyash-rust', '--', *flags],
                              {'RUSTC_BOOTSTRAP': '1'}))
    return results


def json_messages(path):
    # Rust dumps and Cargo JSON share stdout; unrelated dump lines are not diagnostics.
    for line in Path(path).open():
        if not line.startswith('{"reason":'):
            continue
        try:
            yield json.loads(line)
        except json.JSONDecodeError:
            continue


def build_test(work):
    invoke(work, 'test-build', cargo(work, 'test') + ['--lib', '--no-run'])
    candidates = [x['executable'] for x in json_messages(work / 'test-build.txt')
                  if x.get('reason') == 'compiler-artifact' and x.get('executable')
                  and x['target']['name'] == 'nyash_rust']
    if len(candidates) != 1:
        raise RuntimeError('test artifact identity missing or ambiguous')
    write(work / 'test-binary.json', dict(path=candidates[0], sha256=sha(candidates[0])))
    return candidates[0]


def test(work, template, expected_failures):
    binary = build_test(work)
    directory = work / 'T'
    directory.mkdir(exist_ok=True)
    env = configured_env(template, work / 'target/quick/hakorune', work)
    env['LLVM_PROFILE_FILE'] = str(directory / 'test-%p-%m.profraw')
    start = time.monotonic()
    with (directory / 'lib.log').open('w') as stream:
        child = subprocess.Popen([binary, '--test-threads=1'], cwd=ROOT, env=env,
                                 stdout=stream, stderr=subprocess.STDOUT, start_new_session=True)
        code, timed_out, resource_stop, peak = wait_guarded(child, 7200)
    if resource_stop:
        raise MemoryError('memory guard stopped test; profile is incomplete')
    content = (directory / 'lib.log').read_text()
    failure_block = content.rsplit('\nfailures:\n', 1)[-1].split('\ntest result:', 1)[0]
    failures = sorted(line.strip() for line in failure_block.splitlines() if line.strip()) if child.returncode else []
    expected = sorted(Path(expected_failures).read_text().splitlines())
    result = dict(binary=binary, binary_sha256=sha(binary), exit=child.returncode,
                  seconds=time.monotonic() - start, failures=failures,
                  new=sorted(set(failures) - set(expected)), missing=sorted(set(expected) - set(failures)),
                  failure_names_equal=failures == expected)
    write(directory / 'result.json', result)
    if 'test result:' not in content:
        raise RuntimeError('test process did not finish; coverage is incomplete')
    return result
