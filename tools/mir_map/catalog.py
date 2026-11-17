"""Binary coverage descriptors joined by exact source declaration coordinates."""
from collections import defaultdict
from pathlib import Path
import hashlib
import json
from inventory import group_for
from support import ROOT, write


def source_file(name):
    marker = '/src/mir/'
    if marker in name:
        return 'src/mir/' + name.split(marker, 1)[1]
    if name == 'src/mir.rs' or name.endswith('/src/mir.rs'):
        return 'src/mir.rs'
    if name.startswith('src/mir/'):
        return name
    return None


def collect(binary_json, inventory, destination):
    data = json.loads(Path(binary_json).read_text())['data'][0]
    declarations = defaultdict(list)
    rows, symbols = {}, {}
    membership = json.loads((Path(__file__).parent / 'groups.json').read_text())['files']
    for item in inventory['items']:
        if item['kind'] not in ('function_item', 'closure_expression'):
            continue
        key = (item['file'], item['declaration_line'], item['declaration_column'])
        declarations[key].append(item)
        identity = f"source:{item['file']}:{item['declaration_line']}:{item['declaration_column']}"
        rows[identity] = dict(id=identity, file=item['file'], start=item['declaration_line'],
            end=item['end'], name=item['name'], kind=item['kind'], test=item['test'],
            group=group_for(item['file'], membership), span_verified=not inventory['files'][item['file']]['syntax_error'], represented=False, symbols=[])
    for function in data['functions']:
        regions = [region for region in function['regions'] if region[7] == 0]
        if not regions:
            continue
        region = regions[0]
        file = source_file(function['filenames'][region[5]])
        if file is None:
            continue
        declaration = declarations.get((file, region[0], region[1]), [])
        if len(declaration) == 1:
            identity = f'source:{file}:{region[0]}:{region[1]}'
        else:
            identity = 'llvm:' + hashlib.sha256(function['name'].encode()).hexdigest()
            rows[identity] = dict(id=identity, file=file, start=region[0], end=region[2],
                name=function['name'], kind='unmatched_coverage_function', test=None,
                group=group_for(file, membership), span_verified=False, represented=False, symbols=[])
        symbols[function['name']] = identity
        rows[identity]['represented'] = True
        rows[identity]['symbols'].append(function['name'])
    result = dict(mapper_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), method='Exact first CodeRegion source file/line/column to declaration; generic instances union, unmatched symbols remain opaque',
                  rows=rows, symbols=symbols, function_records=len(data['functions']))
    write(destination, result)
    return result


def ensure(binary_json, inventory_path, destination):
    expected = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    if not destination.exists() or json.loads(destination.read_text()).get('mapper_sha256') != expected:
        collect(binary_json, json.loads(Path(inventory_path).read_text()), destination)
