"""Identity transfers preserve Map lanes and any demanded original producer."""
import copy
import subprocess


def run_copy_cases(compile_case, witness, const, root, kernel, env, no_core):
    def execute(label, body, frame, kind, bits, handle='', text_value='cat'):
        obj, ir = compile_case(label, body, frame)
        exe = obj.with_suffix('.exe')
        linked = subprocess.run(['cc', '-no-pie', str(obj), str(root / 'lang/c-abi/tests/static_v2_runtime_probe.c'), kernel,
            '-Wl,--wrap=nyash.map.literal_store_v1', '-Wl,--wrap=nyash.box.from_i8_string_const_len_v1',
            '-ldl', '-lpthread', '-lm', '-o', str(exe)], capture_output=True)
        assert linked.returncode == 0, linked.stderr
        result = subprocess.run([str(exe)], capture_output=True, text=True, preexec_fn=no_core,
            env=dict(env, EXPECT_KIND=str(kind), EXPECT_BITS=str(bits), EXPECT_WRITES='1',
                EXPECT_HANDLE_TYPE=handle, EXPECT_TEXT=text_value, EXPECT_KEY_HEX='6b'))
        assert result.returncode == 0 and 'kernel-readback-ok' in result.stdout, (label, result)
        return ir.read_text()

    for nested in (False, True):
        for kind, bits in ((1, 30), (2, 1), (3, 0x7ff8000000000042), (4, 0), (5, 0)):
            for original in (False, True):
                if original and kind != 1: continue
                body, frame = witness(nested, kind, bits)
                fn = body['functions'][-1]
                ins = fn['blocks'][0]['instructions']
                frame['values'][0]['flags'] = int(original)
                if kind == 5:
                    ins[2] = const(3, 'cat', dict(kind='handle', box_type='StringBox'))
                    frame['values'][0].update(action=2, kind=5, encoding=1, flags=1, payload=0)
                ins[3:3] = [dict(op='copy', dst=7, src=3), dict(op='copy', dst=8, src=7)]
                ins[5]['value'] = 8
                frame['maps'][-1]['instruction'] = 5
                frame['values'] += [dict(function=fn['name'], value=v, action=3, flags=int(original)) for v in (7, 8)]
                if original: ins[-1]['value'] = 8
                label = f'copy-{nested}-{kind}-{original}'
                text = execute(label, body, frame, kind, bits, 'StringBox' if kind == 5 else '')
                assert '%map_kind_8 = add i32 %map_kind_7, 0' in text
                if kind == 3: assert '%r3 =' not in text and '%r7 =' not in text and '%r8 =' not in text
                bad = copy.deepcopy(frame); bad['values'].pop(1)
                compile_case(label + '-missing-source', body, bad,
                    'body-coverage' if not original and kind != 5 else 'value-reference-closure')
                if original:
                    bad = copy.deepcopy(frame); bad['values'][0]['flags'] = 0
                    compile_case(label + '-original-gap', body, bad, 'body-coverage')
                bad = copy.deepcopy(frame); bad['values'][-1]['kind'] = kind
                compile_case(label + '-extra-kind', body, bad, 'value-action')
                bad = copy.deepcopy(body); bad['functions'][-1]['blocks'][0]['instructions'][3]['src'] = 8
                compile_case(label + '-cycle', bad, frame, 'value-reference-closure')
                bad = copy.deepcopy(body); bad['functions'][-1]['blocks'][0]['instructions'][3]['op'] = 'copy_owned'
                compile_case(label + '-owned-stop', bad, frame, 'original-consumer-unsupported')

        for mode in ('integer', 'string', 'bool-i1'):
            body, frame = witness(nested)
            fn = body['functions'][-1]; ins = fn['blocks'][0]['instructions']
            if mode == 'bool-i1':
                ins[2:3] = [const(10, 0), dict(op='unop', dst=3, src=10, operation='not')]
                frame['values'] = [dict(function=fn['name'], value=10, action=1, kind=1, payload=0, flags=1),
                    dict(function=fn['name'], value=3, action=7, operation=6, kind=2, encoding=2, flags=1)]
                at = 4
            else:
                if mode == 'string': ins[2] = const(3, 'cat', dict(kind='handle', box_type='StringBox'))
                frame['values'][0].update(flags=1)
                if mode == 'string': frame['values'][0].update(action=2, kind=5, encoding=1, payload=0)
                at = 3
            ins[at:at] = [dict(op='copy', dst=7, src=3), dict(op='copy', dst=8, src=7)]
            frame['values'] += [dict(function=fn['name'], value=v, action=3, flags=1) for v in (7, 8)]
            ins[at+2]['value'] = 8
            frame['maps'][-1]['instruction'] = at+2
            kind, bits = (2, 1) if mode == 'bool-i1' else (5, 0) if mode == 'string' else (1, 60)
            if mode != 'bool-i1':
                rhs = const(10, 'dog', dict(kind='handle', box_type='StringBox')) if mode == 'string' else const(10, 2)
                ins[at+2:at+2] = [rhs, dict(op='binop', dst=9, lhs=8, rhs=10, operation='+' if mode == 'string' else '*')]
                ins[at+4]['value'] = 9; frame['maps'][-1]['instruction'] += 2
                frame['values'] += [dict(function=fn['name'], value=10, action=2 if mode == 'string' else 1,
                    kind=kind, payload=0 if mode == 'string' else 2, flags=1, encoding=1 if mode == 'string' else 0),
                    dict(function=fn['name'], value=9, action=7, operation=5 if mode == 'string' else 1, kind=kind, flags=1, encoding=1)]
            text = execute(f'copy-{nested}-operation-{mode}', body, frame, kind, bits,
                'StringBox' if mode == 'string' else '', 'catdog' if mode == 'string' else 'cat')
            if mode == 'bool-i1': assert '%map_payload_7 = add i64 %map_payload_3, 0' in text

        for original, kind in ((False, 5), (True, 5), (False, 3)):
            bits = 0 if kind == 5 else 0x7ff8000000000042
            body, frame = witness(nested, kind, bits)
            fn = body['functions'][-1]; ins = fn['blocks'][0]['instructions']
            if kind == 5: ins[2] = const(3, 'cat', dict(kind='handle', box_type='StringBox'))
            ins[3:3] = [dict(op='newbox', dst=7, type='StringBox', args=[3])]
            ins[4]['value'] = 7
            frame['maps'][-1]['instruction'] = 4
            if kind == 5: frame['values'] = [dict(function=fn['name'], value=3, action=2, kind=5, encoding=1, flags=1)]
            frame['values'].append(dict(function=fn['name'], value=7, action=8, flags=int(original)))
            # Same-module has a different retained physical outcome, never a StringBox exception.
            body['typed_object_plans'] = [dict(box_name='StringBox', type_id=11, field_count=0, fields=[])]
            label = f'alias-{nested}-{original}-{kind}'
            if nested:
                compile_case(label, body, frame, 'static_v2_named_alias_mismatch')
            else:
                if original:
                    ins.insert(-1, dict(op='copy', dst=9, src=7))
                execute(label, body, frame, kind, bits, 'StringBox' if kind == 5 else '')
                bad = copy.deepcopy(body); bad['functions'][-1]['blocks'][0]['instructions'][3]['type'] = 'ArrayBox'
                compile_case(label + '-wrong-outcome', bad, frame, 'static_v2_named_alias_mismatch')
