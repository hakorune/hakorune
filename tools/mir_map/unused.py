"""Compiler dead/unused witnesses matched to source item identifier spans."""
from pathlib import Path
import bisect
import json
from compiler import json_messages
from support import ROOT, write


def diagnostics(path):
    result = {}
    for event in json_messages(path):
        if event.get('reason') != 'compiler-message':
            continue
        message = event['message']
        lint = (message.get('code') or {}).get('code', '')
        if message['level'] != 'warning' or not (lint == 'dead_code' or lint.startswith('unused')):
            continue
        for span in message['spans']:
            file = Path(span['file_name'])
            file = file if file.is_absolute() else ROOT / file
            try:
                file = str(file.resolve().relative_to(ROOT))
            except ValueError:
                continue
            if not file.startswith('src/mir/'):
                continue
            key = (lint, file, span['byte_start'], span['byte_end'], message['message'])
            result[key] = dict(lint=lint, file=file, span=span, message=message['message'])
    return result


def collect(work, inventory):
    ordinary = diagnostics(work / ('build.jsonl' if (work / 'build.jsonl').exists() else 'build.txt'))
    forced = diagnostics(work / 'm2-forced.txt')
    by_file = {}
    for item in inventory['items']:
        if item['name_start'] is not None:
            by_file.setdefault(item['file'], []).append(item)
    for values in by_file.values():
        values.sort(key=lambda i: i['name_start'])
    measured, unjoined = {}, []
    for key, diagnostic in forced.items():
        span = diagnostic['span']
        # Rustc also points at a live enclosing type as diagnostic context.
        if span.get('label') and 'in this ' in span['label']:
            continue
        candidates = by_file.get(diagnostic['file'], [])
        starts = [i['name_start'] for i in candidates]
        position = bisect.bisect_left(starts, span['byte_start'])
        matches = [i for i in candidates[max(0, position - 1):position + 3]
                   if span['byte_start'] < i['name_end'] and i['name_start'] < span['byte_end']]
        # A whole impl span contains many live methods: never use it as a dead-item assertion.
        if len(matches) != 1 or span['byte_end'] - span['byte_start'] > 200:
            unjoined.append(diagnostic | {'ordinary_warning': key in ordinary})
            continue
        item = matches[0]
        identity = (item['file'], item['start_byte'], item['kind'])
        row = measured.setdefault(identity, item | dict(lines=None if inventory['files'][item['file']]['syntax_error'] else item['end'] - item['start'] + 1,
                                                        witnesses=[], forced_only=True))
        row['witnesses'].append(diagnostic)
        row['forced_only'] &= key not in ordinary
    result = dict(method='rustc force-warn reachability/unused diagnostics, not direct caller counts',
                  items=list(measured.values()), unmatched_diagnostics=unjoined,
                  ordinary_diagnostic_spans=len(ordinary), forced_diagnostic_spans=len(forced),
                  forced_only_item_count=sum(r['forced_only'] for r in measured.values()),
                  limitations='Unjoined spans have unknown item LOC. forced-only does not by itself prove an explicit allow attribute.')
    write(work / 'unused.json', result)
    return result
