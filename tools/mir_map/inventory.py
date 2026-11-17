"""Syntactic item spans and explicit cfg(test) masks, not dependency resolution."""
from pathlib import Path
import json
import re
from support import ROOT, write

KINDS = {'function_item', 'closure_expression', 'struct_item', 'enum_item', 'trait_item', 'type_item',
         'mod_item', 'enum_variant', 'field_declaration', 'const_item', 'static_item'}


def group_for(file, membership):
    if file in membership:
        return membership[file]
    if file == 'src/mir/builder.rs':
        return 'builder/entry'
    if file.startswith('src/mir/'):
        relative = file.removeprefix('src/mir/')
        first = relative.split('/')[0]
        return 'mir/' + (first if '/' in relative else 'top-level')
    return 'outside/' + '/'.join(file.split('/')[:2])


def collect(output):
    import tree_sitter
    import tree_sitter_rust
    parser = tree_sitter.Parser(tree_sitter.Language(tree_sitter_rust.language()))
    membership = json.loads((Path(__file__).parent / 'groups.json').read_text())['files']
    files, items, links, test_roots = {}, [], [], set()
    paths = sorted((ROOT / 'src/mir').rglob('*.rs')) + [ROOT / 'src/mir.rs']
    for path in paths:
        if not path.exists():
            continue
        file = str(path.relative_to(ROOT))
        content = path.read_bytes()
        tree = parser.parse(content)
        files[file] = dict(lines=len(content.splitlines()), group=group_for(file, membership),
                           syntax_error=tree.root_node.has_error, test_lines=set())
        stack = [(tree.root_node, False, path.parent if path.name == 'mod.rs' else path.with_suffix(''))]
        while stack:
            parent, inherited_test, module_dir = stack.pop()
            attributes = []
            for node in parent.named_children:
                if node.type == 'attribute_item':
                    attributes.append(node)
                    continue
                if node.type in ('line_comment', 'block_comment'):
                    continue
                texts = [re.sub(r'\s+', '', a.text.decode()) for a in attributes]
                test = inherited_test or '#[cfg(test)]' in texts or '#[test]' in texts
                prefix_end = None
                # Rust attributes on a destructuring field may be an ERROR in this grammar.
                # Limit that attribute to the first direct field, not all later siblings.
                if test and not inherited_test and node.type == 'ERROR':
                    tokens = list(node.children)
                    comma = next((c for c in tokens if c.type == ','), None)
                    if comma and any(c.type == ':' for c in tokens[:tokens.index(comma)]):
                        prefix_end = comma.end_point.row + 1
                        test = inherited_test
                start = min([a.start_point.row for a in attributes] + [node.start_point.row]) + 1
                end = node.end_point.row + (1 if node.end_point.column else 0)
                if prefix_end is not None:
                    files[file]['test_lines'].update(range(start, prefix_end + 1))
                if test:
                    files[file]['test_lines'].update(range(start, end + 1))
                name = node.child_by_field_name('name')
                if node.type in KINDS:
                    items.append(dict(file=file, kind=node.type, name=name.text.decode() if name else None,
                        start=start, end=end, declaration_line=node.start_point.row + 1, declaration_column=node.start_point.column + 1, start_byte=node.start_byte, end_byte=node.end_byte,
                        name_start=name.start_byte if name else None, name_end=name.end_byte if name else None,
                        local_test=test, group=files[file]['group']))
                child_dir = module_dir
                if node.type == 'mod_item' and name:
                    body = node.child_by_field_name('body')
                    child_dir = module_dir / name.text.decode()
                    if body is None:
                        explicit = next((re.search(r'#\[path="([^"]+)"\]', x) for x in texts if x.startswith('#[path=')), None)
                        candidates = [path.parent / explicit[1]] if explicit else [child_dir.with_suffix('.rs'), child_dir / 'mod.rs']
                        targets = [str(p.relative_to(ROOT)) for p in candidates if p.exists()]
                        links.extend((file, target) for target in targets)
                        if test:
                            test_roots.update(targets)
                stack.append((node, test, child_dir))
                attributes = []
    changed = True
    while changed:
        old = len(test_roots)
        test_roots.update(target for source, target in links if source in test_roots)
        changed = len(test_roots) != old
    for file, data in files.items():
        if file in test_roots:
            data['test_lines'] = set(range(1, data['lines'] + 1))
        data['test_lines'] = sorted(data['test_lines'])
        data['production_lines'] = data['lines'] - len(data['test_lines'])
    for item in items:
        item['test'] = item.pop('local_test') or item['file'] in test_roots
    result = dict(method='tree-sitter-rust spans; explicit cfg(test)/test attributes and module propagation; inactive feature cfg retained',
                  files=files, items=items, syntax_error_files=[f for f, d in files.items() if d['syntax_error']])
    write(output, result)
    return result
