"""Streaming resolved HIR paths/imports. Not a complete typed-method graph."""
from collections import Counter, defaultdict
from pathlib import Path
import json
import re
from inventory import group_for
from support import write

OWNER = re.compile(r'^DefId\(0:(\d+) ~ (.+)\) => OwnerNodes \{')
TARGET = re.compile(r'DefId\((\d+):(\d+) ~ ([^\n]+?)\)')
SPAN = re.compile(r'^                span: (.+?):(\d+):(\d+): (\d+):(\d+) \(#\d+\),?$')


def extract(work):
    membership = json.loads((Path(__file__).parent / 'groups.json').read_text())['files']
    owners, edges, references, res_kinds = {}, set(), {}, Counter()
    current, resolution_indent = None, None
    # At most one textual record and finite unique edges are retained in memory.
    with (work / 'hir.txt').open() as stream:
        for line in stream:
            match = OWNER.match(line)
            if match:
                current = match[2]
                owners[current] = dict(def_path=current, span=None, kind=None)
                resolution_indent = None
                continue
            if current is None:
                continue
            match = SPAN.match(line)
            if match and owners[current]['span'] is None:
                file, first, _, last, _ = match.groups()
                owners[current]['span'] = dict(file=file, start=int(first), end=int(last))
                owners[current]['group'] = group_for(file, membership)
            if line.startswith('                kind: ') and owners[current]['kind'] is None:
                owners[current]['kind'] = line.strip().removeprefix('kind: ').split()[0].strip('({,')
            indent = len(line) - len(line.lstrip())
            if resolution_indent is not None and indent <= resolution_indent:
                resolution_indent = None
            text = line.strip()
            if text.startswith('res: '):
                variant = re.split(r'[({, ]', text[5:])[0]
                res_kinds[variant] += 1
                if variant not in ('Local', 'PrimTy', 'Err'):
                    resolution_indent = indent
            if resolution_indent is not None:
                for target in TARGET.finditer(line):
                    path = target[3]
                    edges.add((current, path))
                    references[path] = dict(crate=int(target[1]), index=int(target[2]))
    joined, unresolved = [], []
    for source, target in sorted(edges):
        left = owners.get(source)
        right = owners.get(target)
        if not left or not left.get('span'):
            unresolved.append(dict(source=source, target=target, reason='source span missing'))
            continue
        if not right or not right.get('span'):
            unresolved.append(dict(source=source, target=target, reason='target owner span unavailable'))
            continue
        joined.append(dict(source=source, target=target, source_group=left['group'], target_group=right['group']))
    result = dict(method='Compiler HIR semantic res/PerNS references only; structural OwnerId/HirId excluded',
        complete_typed_graph=False, limitation='THIR SIGKILL/OOM: method dispatch, inferred nominal types, indirect call targets are not measured',
        owners=owners, edges=joined, unresolved_edges=unresolved, resolution_variants=dict(res_kinds))
    write(work / 'dependencies.json', result)
    return result


def graph(data):
    adjacency, incoming, outgoing, used_items, consumers = defaultdict(set), defaultdict(set), defaultdict(set), defaultdict(set), defaultdict(set)
    measured_edges = [e for e in data['edges']
                      if data['owners'].get(e['target'], {}).get('kind') != 'Mod']
    for edge in measured_edges:
        source, target = edge['source_group'], edge['target_group']
        if source == target:
            continue
        adjacency[source].add(target)
        incoming[target].add(source)
        outgoing[source].add(target)
        used_items[target].add(edge['target'])
        consumers[edge['target']].add(source)
    index, low, stack, active, components = {}, {}, [], set(), []

    def visit(node):
        index[node] = low[node] = len(index)
        stack.append(node)
        active.add(node)
        for target in adjacency[node]:
            if target not in index:
                visit(target)
                low[node] = min(low[node], low[target])
            elif target in active:
                low[node] = min(low[node], index[target])
        if low[node] == index[node]:
            component = []
            while True:
                target = stack.pop()
                active.remove(target)
                component.append(target)
                if target == node:
                    break
            if len(component) > 1:
                components.append(sorted(component))

    nodes = set(adjacency) | {t for ts in list(adjacency.values()) for t in ts}
    for node in sorted(nodes):
        if node not in index:
            visit(node)
    cycles = []
    for component in sorted(components, key=lambda c: (-len(c), c)):
        members = set(component)
        cycles.append(dict(groups=component, size=len(component), witnesses=[e for e in measured_edges
                      if e['source_group'] in members and e['target_group'] in members and e['source_group'] != e['target_group']]))
    return dict(scope='Resolved HIR non-namespace item references lower bound; module path-prefix references excluded; not full typed graph', namespace_edges_excluded=len(data['edges']) - len(measured_edges), measured_edges=len(measured_edges), cycles=cycles,
        groups={g: dict(incoming=len(incoming[g]), outgoing=len(outgoing[g]), used_items=len(used_items[g])) for g in nodes},
        bottlenecks=[dict(item=k, consumer_groups=sorted(v), count=len(v)) for k, v in sorted(consumers.items(), key=lambda p: (-len(p[1]), p[0]))[:20]])
