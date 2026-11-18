"""Shared measurement I/O; compiler sources are never rewritten."""
from pathlib import Path
import hashlib
import json
import os
import signal
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
CORPUS_MANIFEST = ROOT / 'apps/tests/corpus/manifest.json'
FLAGS = '-C instrument-coverage -C link-arg=-fuse-ld=lld -C codegen-units=4'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def write(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_suffix(path.suffix + '.tmp')
    temp.write_text(json.dumps(value, ensure_ascii=False, indent=2))
    temp.replace(path)


def llvm(name):
    sysroot = subprocess.check_output(['rustc', '--print', 'sysroot'], text=True).strip()
    host = next(x.split(': ', 1)[1] for x in subprocess.check_output(
        ['rustc', '-vV'], text=True).splitlines() if x.startswith('host: '))
    return str(Path(sysroot) / 'lib/rustlib' / host / 'bin' / ('llvm-' + name))


def build_env(work):
    return os.environ | {'CARGO_BUILD_JOBS': '2', 'RUSTC_WRAPPER': '',
        'CARGO_BUILD_RUSTC_WRAPPER': '', 'RUSTFLAGS': FLAGS,
        'LLVM_PROFILE_FILE': str(work / 'build-%p-%m.profraw'),
        'RUST_MIN_STACK': '67108864'}


def configured_env(template, binary, work):
    settings = json.loads(Path(template).read_text())
    # Retain the frozen language/runtime settings without copying account secrets.
    env = os.environ | {k: v for k, v in settings.items()
                        if k.startswith(('HAKO_', 'NYASH_')) or k in ('PATH', 'LD_LIBRARY_PATH')}
    env.update(HAKO_ROOT=str(ROOT), NYASH_ROOT=str(ROOT), NYASH_BIN=str(binary),
               RUST_MIN_STACK='67108864', TMPDIR=str(work / 'tmp'))
    env.pop('HAKO_MIR_BUILDER', None)
    (work / 'tmp').mkdir(parents=True, exist_ok=True)
    return env


def wait_guarded(child, timeout):
    from resources import active
    return active().wait(child, timeout)


def run(command, log, env, timeout=180):
    start = time.monotonic()
    Path(log).parent.mkdir(parents=True, exist_ok=True)
    with Path(log).open('wb') as stream:
        from resources import active
        child = active().launch(list(map(str, command)), cwd=ROOT, env=env,
                                stdout=stream, stderr=subprocess.STDOUT)
        code, timed_out, resource_stop, peak = wait_guarded(child, timeout)
    if resource_stop:
        raise MemoryError(f'memory guard stopped measurement, log={log}, peak_kib={peak}')
    return dict(command=list(map(str, command)), exit=code, timeout=timed_out,
                seconds=time.monotonic() - start, log=str(log), peak_rss_kib=peak)


def manifest_rows(data):
    """Resolve and verify both the portable corpus and existing measurement manifests."""
    portable = data.get('version') == 1
    unique = {}
    for row in data['rows']:
        path = Path(row['path'])
        source = (ROOT / path).resolve()
        if portable and (path.is_absolute() or '..' in path.parts or not source.is_relative_to(ROOT)):
            raise ValueError(f'corpus input must be relative to the repository: {path}')
        digest = sha(source)
        if row.get('sha256', digest) != digest or portable and 'sha256' not in row:
            raise ValueError(f'input changed: {source}')
        key = str(source)
        if portable and key in unique:
            raise ValueError(f'duplicate corpus path: {path}')
        record = unique.setdefault(key, dict(row, path=key, sha256=digest, groups=[]))
        record['groups'] = sorted(set(record['groups']) | set(row.get('groups', ())))
    if not unique:
        raise ValueError('empty input manifest')
    return list(unique.values())


def read_manifest(manifest):
    return manifest_rows(json.loads(Path(manifest).read_text()))


def fixed_inputs(manifest):
    data = json.loads(Path(manifest).read_text())
    rows = sorted(manifest_rows(data), key=lambda row: row['path'])
    if data.get('version') == 1:
        rows = [row for row in rows if 'A' in row['groups']]
        return rows, {row['path'] for row in rows if row['coverage_exe']}
    declared = data['unique_sources']
    if type(declared) is not int or not 0 < declared == len(rows) == len(data['rows']):
        raise ValueError(f'invalid fixed input count: {declared}')
    executables = {str((ROOT / source).resolve()) for group in data['groups'].values()
                   if group['mode'] == 'exe' for source in group['sources']}
    return rows, executables
