"""LLVM profile readers. Keep invocation counts separate from input membership."""
from pathlib import Path
import gzip
import json
import subprocess
from support import llvm


def merge(paths, destination):
    paths = sorted(map(str, paths))
    if not paths:
        raise ValueError('no raw profile: absence is not zero coverage')
    listing = Path(str(destination) + '.inputs')
    listing.write_text('\n'.join(paths) + '\n')
    guarded([llvm('profdata'), 'merge', '-sparse', '-f', str(listing),
             '-o', str(destination)], Path(str(destination) + '.merge.log'))


def guarded(command, output):
    from resources import active
    with Path(output).open('wb') as stream, Path(str(output) + '.stderr').open('wb') as err:
        child = active().launch(command, stdout=stream, stderr=err)
        code, timeout, stopped, peak = active().wait(child, 600)
    if stopped:
        raise MemoryError('resource supervisor stopped LLVM processing')
    if code or timeout:
        raise RuntimeError(f'LLVM processing failed: {command}; log={output}')


def read_lcov(path):
    counts, lines, universe = {}, {}, {}
    current = None
    for line in Path(path).open():
        if line.startswith('SF:'):
            source = line[3:].strip()
            marker = '/src/mir/'
            current = 'src/mir/' + source.split(marker, 1)[1] if marker in source else None
            if source.endswith('/src/mir.rs'):
                current = 'src/mir.rs'
        elif current and line.startswith('FNDA:'):
            count, name = line[5:].strip().split(',', 1)
            if int(count):
                counts[name] = counts.get(name, 0) + int(count)
        elif current and line.startswith('DA:'):
            number, count, *_ = line[3:].strip().split(',')
            universe.setdefault(current, set()).add(int(number))
            if int(count):
                lines.setdefault(current, set()).add(int(number))
    return counts, {f: sorted(v) for f, v in lines.items()}, universe


def save_counts(raw_directory, stem, binary):
    profile = Path(str(stem) + '.profdata')
    merge(Path(raw_directory).glob('*.profraw'), profile)
    lcov = Path(str(stem) + '.lcov')
    export(binary, profile, lcov, format='lcov')
    result, lines, universe = read_lcov(lcov)
    target = Path(str(stem) + '.counts.json.gz')
    with gzip.open(target, 'wt') as stream:
        json.dump({'functions': result, 'lines': lines}, stream, separators=(',', ':'))
    lcov.unlink()
    return dict(profile=str(profile), counts=str(target), functions=len(result),
                covered_lines=sum(map(len, lines.values())),
                instrumented_lines=sum(map(len, universe.values())))


def export(binary, profile, target, format='text'):
    guarded([llvm('cov'), 'export', str(binary),
             '--instr-profile=' + str(profile), '--skip-expansions',
             '--format=' + format], target)
