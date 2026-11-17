"""Shared measurement I/O; compiler sources are never rewritten."""
from pathlib import Path
import hashlib
import json
import os
import signal
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
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
    (work / 'tmp').mkdir(exist_ok=True)
    return env


def memory_status(pid):
    available = next(int(line.split()[1]) for line in Path('/proc/meminfo').read_text().splitlines()
                     if line.startswith('MemAvailable:'))
    total, pending = 0, [pid]
    while pending:
        child = pending.pop()
        try:
            status = Path(f'/proc/{child}/status').read_text().splitlines()
            total += next((int(line.split()[1]) for line in status if line.startswith('VmRSS:')), 0)
            pending.extend(map(int, Path(f'/proc/{child}/task/{child}/children').read_text().split()))
        except (FileNotFoundError, ProcessLookupError):
            pass
    return available, total


def wait_guarded(child, timeout):
    start, peak = time.monotonic(), 0
    while child.poll() is None:
        available, rss = memory_status(child.pid)
        peak = max(peak, rss)
        resource_stop = available < 6 * 1024 * 1024 or rss > 16 * 1024 * 1024
        timed_out = time.monotonic() - start > timeout
        if resource_stop or timed_out:
            try:
                os.killpg(child.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            try:
                child.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait()
            return child.returncode, timed_out, resource_stop, peak
        time.sleep(0.1)
    return child.returncode, False, False, peak


def run(command, log, env, timeout=180):
    start = time.monotonic()
    Path(log).parent.mkdir(parents=True, exist_ok=True)
    with Path(log).open('wb') as stream:
        child = subprocess.Popen(list(map(str, command)), cwd=ROOT, env=env,
                                 stdout=stream, stderr=subprocess.STDOUT,
                                 start_new_session=True)
        code, timed_out, resource_stop, peak = wait_guarded(child, timeout)
    if resource_stop:
        raise MemoryError(f'memory guard stopped measurement, log={log}, peak_kib={peak}')
    return dict(command=list(map(str, command)), exit=code, timeout=timed_out,
                seconds=time.monotonic() - start, log=str(log), peak_rss_kib=peak)


def fixed_inputs(manifest):
    data = json.loads(Path(manifest).read_text())
    rows = sorted(data['rows'], key=lambda x: x['path'])
    assert len(rows) == 5386, len(rows)
    for row in rows:
        assert sha(row['path']) == row['sha256'], row['path']
    executables = {source for group in data['groups'].values()
                   if group['mode'] == 'exe' for source in group['sources']}
    return rows, executables
