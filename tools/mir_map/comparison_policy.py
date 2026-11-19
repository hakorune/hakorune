"""Fail-closed eligibility for reusing measured executable observations."""
from collections import Counter
from pathlib import Path
import hashlib
import json
import subprocess

from loop_mir_text import extract
from support import ROOT

MAX_JOBS = 12

# These paths are consumed only by this corpus/measurement work, outside
# compilation, backend selection, runtime and link. Unclassified paths force
# a full comparison. Include both sides of a rename through --no-renames.
COMPARISON_ONLY = ('tools/mir_map/',)
COMPARISON_ONLY_FILES = {'apps/tests/corpus/manifest.json', 'src/mir/README.md'}


def harmless_paths(paths):
    return all(path in COMPARISON_ONLY_FILES or path.startswith(COMPARISON_ONLY)
               or path.startswith('docs/') and path.endswith('.md') for path in paths)


def downstream_unchanged(before, after, root=ROOT):
    if not before or not after:
        return False
    try:
        def git(*args):
            return subprocess.check_output(['git', '-C', str(root), *args], stderr=subprocess.DEVNULL)
        if git('status', '--porcelain=v1'):
            return False
        endpoints = [git('rev-parse', '--verify', '--end-of-options', f'{ref}^{{commit}}').strip().decode()
                     for ref in (before, after)]
        if git('rev-parse', 'HEAD').strip().decode() != endpoints[1]:
            return False
        paths = git('diff', '--name-only', '-z', '--no-renames', *endpoints, '--').decode()
        return harmless_paths(paths.rstrip('\0').split('\0') if paths else [])
    except (OSError, UnicodeError, subprocess.CalledProcessError):
        return False


def auto_jobs(log, available_kib):
    with Path(log).open() as lines:
        peak = max((json.loads(line)['peak_rss_kib'] for line in lines if line.strip()), default=0)
    if peak <= 0:
        return 1, None
    return max(1, min(MAX_JOBS, available_kib // (4 * peak))), peak


def exact_mir(before, after):
    if not before or not after:
        return False
    original, current = extract(before), extract(after)
    return original is not None and original == current


def can_reuse_exe(before, after):
    # The printed standalone MIR is produced by a different route than the
    # closed EXE. Keep the same compiler and measured artifacts as prerequisites
    # for inferring the closed route from a prior observation.
    return (before.get('artifact_roles') is not None
            and before['artifact_roles'] == after.get('artifact_roles')
            and before['artifacts'] == after.get('artifacts')
            and before.get('cli_sha256') is not None
            and before['cli_sha256'] == after.get('cli_sha256')
            and before.get('execution_env') is not None
            and before['execution_env'] == after.get('execution_env'))


def env_fingerprint(env):
    # Per-run output location and the compiler identity are separately recorded.
    excluded = {'TMPDIR', 'PWD', 'OLDPWD', 'SHLVL', '_'}
    relevant = sorted((name, value) for name, value in env.items() if name not in excluded)
    return hashlib.sha256(json.dumps(relevant).encode()).hexdigest()


def eligible_rows(rows):
    return {row['path']: row for row in rows
            if row['dump']['kind'] == 'success'
            and row['compile']['kind'] == 'success'
            and row['compile'].get('emitted')
            and row.get('run', {}).get('kind') == 'executed'
            and not row.get('exe_skipped')}


def summarize_skip(results):
    return dict(Counter(row['exe_skipped'] for row in results if 'exe_skipped' in row))
