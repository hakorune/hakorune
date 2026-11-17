"""Terminal observations and a separately labeled literal source-tag catalogue."""
from collections import Counter
from pathlib import Path
import json
import re
from support import ROOT, sha, write

TAG_CHAIN = re.compile(r'^(\[[^\]\n]+\](?:\[[^\]\n]+\])*)')
ERROR = re.compile(r'(?:❌\s*(?:MIR compilation error|Parse error in)|^Error:|^error:|using prelude parse)', re.I)


def outcome(command):
    if command['timeout']:
        return dict(kind='timeout', first_refusal=None)
    if command['exit'] < 0:
        return dict(kind='signal', signal=-command['exit'], first_refusal=None)
    if command['exit'] == 0:
        return dict(kind='success', first_refusal=None)
    text = Path(command['log']).read_text(errors='replace')
    terminal = next((line for line in text.splitlines() if ERROR.search(line)), '')
    parser = bool(re.search(r'Parse error in|using prelude parse|^Error:.*(?:parser|syntax) error', terminal, re.I))
    tag = re.search(r'(\[[^\]\n]+\](?:\[[^\]\n]+\])*)', terminal)
    name = tag[1] if tag and re.search(r'[/:-]', tag[1]) else None
    return dict(kind='parser_refusal' if parser else 'refusal', first_refusal=name, terminal=terminal)


def catalogue(inventory, destination):
    import tree_sitter
    import tree_sitter_rust
    parser = tree_sitter.Parser(tree_sitter.Language(tree_sitter_rust.language()))
    names, dynamic, hashes = {}, [], {}
    for file, metadata in inventory['files'].items():
        content = (ROOT / file).read_bytes()
        hashes[file] = sha(ROOT / file)
        tree = parser.parse(content)
        excluded = set(metadata['test_lines'])
        pending = [tree.root_node]
        while pending:
            node = pending.pop()
            pending.extend(node.named_children)
            if node.type not in ('string_literal', 'raw_string_literal') or node.start_point.row + 1 in excluded:
                continue
            literal = node.text.decode(errors='replace')
            if literal.startswith('r'):
                match = re.fullmatch(r'r(#{0,})"(.*)"\1', literal, re.S)
                text = match[2] if match else ''
            else:
                text = literal[1:-1]
            tag = TAG_CHAIN.match(text)
            if tag is None or not re.match(r'\[(?:freeze:|plan/freeze:|mir/)', tag[1]):
                continue
            site = dict(file=file, line=node.start_point.row + 1, literal=tag[1])
            if any(c in tag[1] for c in '{}\\'):
                dynamic.append(site)
            else:
                names.setdefault(tag[1], []).append(site)
    result = dict(scope='Exact visible contract-tag literal chains in non-test syntax; NOT all emitted refusal names',
        names=names, unresolved_templates=dynamic, source_sha256=hashes,
        full_name_universe_known=False,
        limitation='Helper suffix composition, dynamic names, escaped/macro-generated tags, inactive cfg and diagnostic-only literals are not proven complete. No string occurrence is a runtime hit.')
    write(destination, result)
    return result


def summarize(work, catalog):
    rows, groups, observed = [], {}, set()
    for group, expected in [('A', 5386), ('B', 1192)]:
        statuses, names, stages = Counter(), Counter(), Counter()
        compile_inputs, emitted = 0, 0
        for path in sorted((work / group).glob('[0-9][0-9][0-9][0-9]/result.json')):
            record = json.loads(path.read_text())
            compile_inputs += len(record['commands']) > 1
            emitted += record['emitted']
            parts = [outcome(command) for command in record['commands']]
            result = parts[-1]
            if result['kind'] == 'success' and len(parts) > 1 and not record['emitted']:
                result = result | dict(kind='success_without_exe')
            statuses[result['kind']] += 1
            if result['first_refusal']:
                names[result['first_refusal']] += 1
            for part in parts:
                stages[part['kind']] += 1
                observed.update(re.findall(r'(\[[^\]\n]+\](?:\[[^\]\n]+\])*)', part.get('terminal', '')))
            rows.append(dict(group=group, source=record['path'], source_sha256=record['sha256'],
                             result=result, stages=parts))
        groups[group] = dict(expected=expected, completed=sum(statuses.values()), outcomes=dict(statuses),
                             first_refusals=dict(names), top10=names.most_common(10), stage_outcomes=dict(stages),
                             exe_compile_inputs=compile_inputs, exe_emitted=emitted, dump_only_inputs=sum(statuses.values())-compile_inputs)
    fixed = set(catalog['names'])
    result = dict(groups=groups, inputs=rows, source_fixed_catalogue=len(fixed),
        fixed_catalogue_seen=sorted(fixed & observed), fixed_catalogue_unseen=sorted(fixed - observed),
        fixed_catalogue_unseen_count=len(fixed - observed),
        full_source_refusal_unseen_count=None, unresolved_source_templates=len(catalog['unresolved_templates']),
        definition='first_refusals count terminal names per input; catalogue seen considers all exact tag chains on terminal error lines from both dump and exe-compile stages, never debug logs')
    write(work / 'refusal-summary.json', result)
    return result
