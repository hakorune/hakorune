"""Map control transfers through the existing PHI and Select owners."""
import copy
import subprocess


def run_control_cases(compile_case, witness, const, root, kernel, env, no_core):
    def execute(label, body, frame, kind, bits, writes=1):
        obj, ir = compile_case(label, body, frame)
        exe = obj.with_suffix('.exe')
        linked = subprocess.run(['cc', '-no-pie', str(obj), str(root / 'lang/c-abi/tests/static_v2_runtime_probe.c'), kernel,
            '-Wl,--wrap=nyash.map.literal_store_v1', '-Wl,--wrap=nyash.box.from_i8_string_const_len_v1',
            '-ldl', '-lpthread', '-lm', '-o', str(exe)], capture_output=True)
        assert linked.returncode == 0, linked.stderr
        result = subprocess.run([str(exe)], capture_output=True, text=True, preexec_fn=no_core,
            env=dict(env, EXPECT_KIND=str(kind), EXPECT_BITS=str(bits), EXPECT_WRITES=str(writes),
                EXPECT_KEY_HEX='6b', EXPECT_HANDLE_TYPE='StringBox', EXPECT_TEXT='cat'))
        assert result.returncode == 0 and 'kernel-readback-ok' in result.stdout, (label, result)
        return ir.read_text()

    def row(fn, value, action, **fields):
        return dict(function=fn['name'], value=value, action=action, **fields)

    for nested in (False, True):
        for operation in ('phi', 'select'):
            for original in (False, True):
                for take_left in (False, True):
                    if original and not take_left: continue
                    body, frame = witness(nested)
                    fn = body['functions'][-1]
                    prefix = fn['blocks'][0]['instructions'][:2]
                    left, right = const(10, 30), const(11, 40 if original else None, 'i64' if original else 'f64')
                    frame['values'] = [row(fn, 10, 1, kind=1, payload=30, flags=int(original)),
                        row(fn, 11, 1, kind=1 if original else 3, payload=40 if original else 0x7ff8000000000042, flags=int(original)),
                        row(fn, 3, 4 if operation == 'phi' else 5, flags=int(original))]
                    tail = [dict(op='map_literal_entry_write', receiver=1, key=2, value=3), const(4, 30),
                        dict(op='ret', value=3 if original else 4)]
                    condition = const(12, int(take_left))
                    if operation == 'phi':
                        fn['blocks'] = [dict(id=0, instructions=prefix+[condition, dict(op='branch', cond=12, then=1, **{'else': 2})]),
                            dict(id=1, instructions=[left, dict(op='jump', target=3)]),
                            dict(id=2, instructions=[right, dict(op='jump', target=3)]),
                            dict(id=3, instructions=[dict(op='phi', dst=3, dst_type='i64', incoming=[[10, 1], [11, 2]])]+tail)]
                        frame['maps'][-1].update(block=3, instruction=1)
                    else:
                        fn['blocks'][0]['instructions'] = prefix+[left, right, condition,
                            dict(op='select', dst=3, cond=12, then_val=10, else_val=11)]+tail
                        frame['maps'][-1]['instruction'] = 6
                    label = f'control-{nested}-{operation}-{original}-{take_left}'
                    text = execute(label, body, frame, 1 if take_left else 3, 30 if take_left else 0x7ff8000000000042)
                    assert f'%map_kind_3 = {operation}' in text and f'%map_payload_3 = {operation}' in text
                    if not original: assert '%r3 =' not in text
                    bad = copy.deepcopy(frame); bad['values'][-1]['encoding'] = 1
                    compile_case(label+'-wire', body, bad, 'value-action')
                    if not original:
                        # A constant branch/condition cannot erase the incompatible incoming domain.
                        bad, bad_frame = copy.deepcopy(body), copy.deepcopy(frame)
                        block = bad['functions'][-1]['blocks'][-1]
                        at = next(i for i, ins in enumerate(block['instructions']) if ins['op'] == 'map_literal_entry_write')
                        block['instructions'][at:at] = [const(13, 1), dict(op='binop', dst=14, lhs=3, rhs=13, operation='+')]
                        block['instructions'][at+2]['value'] = 14
                        bad_frame['maps'][-1]['instruction'] += 2
                        for value in bad_frame['values']: value['flags'] = 1
                        # Float original lane is independently forbidden; same-width Bool still differs in domain.
                        float_ins = next(ins for b in bad['functions'][-1]['blocks'] for ins in b['instructions'] if ins.get('dst') == 11)
                        float_ins['value'] = dict(type='i64', value=1)
                        bad_frame['values'][1].update(kind=2, payload=1)
                        bad_frame['values'] += [row(fn, 13, 1, kind=1, payload=1, flags=1),
                            row(fn, 14, 7, operation=1, kind=1, encoding=1, flags=1)]
                        compile_case(label+'-mixed-operation', bad, bad_frame, 'value-reference-closure')
                        # String remains distinct from other handles even though both use wire kind5.
                        string_body, string_frame = copy.deepcopy(bad), copy.deepcopy(bad_frame)
                        for block in string_body['functions'][-1]['blocks']:
                            for i, ins in enumerate(block['instructions']):
                                if ins.get('dst') == 10: block['instructions'][i] = const(10, 'cat', dict(kind='handle', box_type='StringBox'))
                                if ins.get('dst') == 11: block['instructions'][i] = dict(op='newbox', dst=11, type='MapBox', args=[])
                                if ins.get('dst') == 13: block['instructions'][i] = const(13, 'dog', dict(kind='handle', box_type='StringBox'))
                        for value in string_frame['values']:
                            if value['value'] in (10, 11, 13): value.update(action=2, kind=5, payload=0, encoding=1)
                            if value['value'] == 14: value.update(operation=5, kind=5)
                        compile_case(label+'-string-handle-domain', string_body, string_frame, 'value-reference-closure')
                    if operation == 'select':
                        bad_frame = copy.deepcopy(frame)
                        bad_frame['values'].append(row(fn, 12, 1, kind=2, payload=int(take_left)))
                        compile_case(label+'-condition-original-gap', body, bad_frame, 'body-coverage')
                    if operation == 'phi':
                        bad = copy.deepcopy(body)
                        bad['functions'][-1]['blocks'][-1]['instructions'][0]['incoming'][1][1] = 1
                        compile_case(label+'-duplicate-edge', bad, frame, 'value-action')
                        bad = copy.deepcopy(body)
                        bad['functions'][-1]['blocks'][-1]['instructions'][0]['incoming'] = [[10, i] for i in range(17)]
                        compile_case(label+'-overflow', bad, frame, 'value-action')

        body, frame = witness(nested)
        fn = body['functions'][-1]
        prefix = fn['blocks'][0]['instructions'][:2]
        fn['blocks'] = [dict(id=0, instructions=prefix+[const(10, 30), const(15, 0), const(16, 1), dict(op='jump', target=1)]),
            dict(id=1, instructions=[dict(op='phi', dst=3, incoming=[[10, 0], [11, 2]]),
                dict(op='phi', dst=20, dst_type='i64', incoming=[[15, 0], [21, 2]]),
                dict(op='map_literal_entry_write', receiver=1, key=2, value=3),
                dict(op='compare', dst=22, lhs=20, rhs=16, operation='<'), dict(op='branch', cond=22, then=2, **{'else': 3})]),
            dict(id=2, instructions=[dict(op='copy', dst=11, src=3), dict(op='binop', dst=21, lhs=20, rhs=16, operation='+'),
                dict(op='map_literal_entry_write', receiver=1, key=2, value=11), dict(op='jump', target=1)]),
            dict(id=3, instructions=[const(4, 30), dict(op='ret', value=4)])]
        frame['values'] = [row(fn, 10, 1, kind=1, payload=30), row(fn, 3, 4), row(fn, 11, 3)]
        frame['maps'] = [frame['maps'][0], dict(function=fn['name'], block=1, instruction=2, kind=2),
            dict(function=fn['name'], block=2, instruction=2, kind=2)]
        label = f'control-{nested}-loop'
        text = execute(label, body, frame, 1, 30, writes=3)
        assert '[ %map_payload_11, %exact_status_continue_2_2 ]' in text
        bad = copy.deepcopy(body); bad['functions'][-1]['blocks'][2]['instructions'][0]['src'] = 11
        compile_case(label+'-seedless-arm', bad, frame, 'value-reference-closure')
        bad = copy.deepcopy(body)
        bad['functions'][-1]['blocks'][1]['instructions'][0]['incoming'] = [[11, 0], [11, 2]]
        bad_frame = copy.deepcopy(frame); bad_frame['values'].pop(0)
        compile_case(label+'-seedless-cycle', bad, bad_frame, 'value-reference-closure')

        bad = copy.deepcopy(body); bad_frame = copy.deepcopy(frame)
        bad['functions'][-1]['blocks'][2]['instructions'][0] = dict(op='binop', dst=11, lhs=11, rhs=16, operation='+')
        bad_frame['values'][-1].update(action=7, operation=1, kind=1, encoding=1, flags=1)
        bad_frame['values'].append(row(fn, 16, 1, kind=1, payload=1, flags=1))
        compile_case(label+'-seedless-operation', bad, bad_frame, 'value-reference-closure')

        boxed, boxed_frame = copy.deepcopy(body), copy.deepcopy(frame)
        boxed_fn = boxed['functions'][-1]
        boxed_fn['blocks'][0]['instructions'][-1:-1] = [const(17, 30),
            dict(op='variant_make', dst=7, enum='Option', variant='Some', tag=1, payload=17,
                boxed_sum_abi_plan_id=0, boxed_sum_payload_storage='i64',
                variant_binding=dict(dst=7, tag_const=1, tag_reg=0, payload_reg=17, has_payload=True,
                    enum_name='Option', copy_alias_payload=False, const_zero_result=False, boxed_sum_abi_plan_id=0))]
        boxed_fn['blocks'][2]['instructions'][0] = dict(op='variant_project', dst=11, value=7, enum='Option',
            variant='Some', tag=1, boxed_sum_abi_plan_id=0, boxed_sum_payload_storage='i64', payload_type='Integer')
        boxed['boxed_sum_abi_plans'] = [dict(version='boxed_runtime_v2', plan_id=0, shape_key='Option|0:none,1:i64',
            enum_name='Option', runtime_type_id=700000, runtime_box_name='__NyVariant_Option', tag_storage='i64',
            variants=[dict(name='None', tag=0, payload_storage='none'), dict(name='Some', tag=1, payload_storage='i64')])]
        boxed_frame['values'][-1].update(action=2, kind=1, encoding=1, flags=1)
        text = execute(label+'-future-boxed-alias', boxed, boxed_frame, 1, 30, writes=3)
        assert '[ %map_payload_11, %exact_status_continue_2_2 ]' in text
        for value in boxed_frame['values']: value['flags'] = 1
        boxed_fn['blocks'][1]['instructions'][0]['dst_type'] = 'i64'
        boxed_fn['blocks'][3]['instructions'] = [dict(op='ret', value=3)]
        text = execute(label+'-future-original-alias', boxed, boxed_frame, 1, 30, writes=3)
        assert '[ %map_original_11,' in text
        downstream, downstream_frame = copy.deepcopy(boxed), copy.deepcopy(boxed_frame)
        downstream_fn = downstream['functions'][-1]
        downstream_fn['blocks'][3]['instructions'] = [
            dict(op='phi', dst=24, dst_type='i64', incoming=[[27, 1]]),
            const(25, 0), dict(op='binop', dst=26, lhs=24, rhs=25, operation='+'),
            dict(op='ret', value=26)]
        downstream_fn['blocks'][1]['instructions'].insert(2,
            dict(op='phi', dst=27, dst_type='i64', incoming=[[10, 0], [11, 2]]))
        downstream_frame['maps'][1]['instruction'] += 1
        execute(label+'-nonmapped-original-phi', downstream, downstream_frame, 1, 30, writes=3)
        for mapped in (False, True):
            chained, chained_frame = copy.deepcopy(downstream), copy.deepcopy(downstream_frame)
            cfn = chained['functions'][-1]
            cfn['blocks'][2]['instructions'][1:1] = [
                dict(op='copy', dst=28, src=11), dict(op='copy', dst=29, src=28)]
            cfn['blocks'][1]['instructions'][2]['incoming'][1][0] = 29
            chained_frame['maps'][-1]['instruction'] += 2
            if mapped:
                chained_frame['values'].extend([row(fn, 28, 3, flags=1), row(fn, 29, 3, flags=1)])
                cfn['blocks'][2]['instructions'][4]['value'] = 29
            execute(label+f'-original-copy-chain-{mapped}', chained, chained_frame, 1, 30, writes=3)
            pending = copy.deepcopy(chained)
            pending_frame = copy.deepcopy(chained_frame)
            pending['functions'][-1]['blocks'][0]['instructions'][2] = const(10, 1)
            pending_frame['values'][0]['payload'] = 1
            pending['functions'][-1]['blocks'][1]['instructions'][2]['dst_type'] = 'i1'
            compile_case(label+f'-i1-copy-chain-pending-{mapped}', pending, pending_frame,
                'static_v2_phi_original_alias_pending' if mapped else 'static_v2_phi_original_width')


        for selected in (False, True):
            pending, pending_frame = copy.deepcopy(downstream), copy.deepcopy(downstream_frame)
            pfn = pending['functions'][-1]
            pfn['blocks'][1]['instructions'][0 if selected else 2]['dst_type'] = 'i1'
            pfn['blocks'][0]['instructions'][2] = const(10, 1)
            pending_frame['values'][0]['payload'] = 1
            compile_case(label+f'-i1-alias-pending-{selected}', pending, pending_frame,
                'static_v2_phi_original_alias_pending')


        width, width_frame = copy.deepcopy(body), copy.deepcopy(frame)
        width_fn = width['functions'][-1]
        width_fn['blocks'][2]['instructions'][0] = dict(op='unop', dst=11, src=15, operation='not')
        width_fn['blocks'][1]['instructions'][0]['dst_type'] = 'i64'
        for value in width_frame['values']: value['flags'] = 1
        width_frame['values'][-1].update(action=7, operation=6, kind=2, encoding=2)
        width_frame['values'].append(row(fn, 15, 1, kind=1, payload=0, flags=1))
        width_fn['blocks'][1]['instructions'].pop(2)
        width_frame['maps'].pop(1)
        width_fn['blocks'][3]['instructions'] = [const(4, 29),
            dict(op='binop', dst=23, lhs=3, rhs=4, operation='+'), dict(op='ret', value=23)]
        width_fn['blocks'][3]['instructions'].insert(0,
            dict(op='map_literal_entry_write', receiver=1, key=2, value=3))
        width_frame['maps'].append(dict(function=fn['name'], block=3, instruction=0, kind=2))
        execute(label+'-original-width-normalized', width, width_frame, 2, 1, writes=2)

        arithmetic, arithmetic_frame = copy.deepcopy(body), copy.deepcopy(frame)
        afn = arithmetic['functions'][-1]
        afn['blocks'][1]['instructions'].pop(2)
        afn['blocks'][1]['instructions'][0]['dst_type'] = 'i64'
        afn['blocks'][2]['instructions'][0] = dict(op='binop', dst=11, lhs=3, rhs=16, operation='+')
        arithmetic_frame['maps'].pop(1)
        for value in arithmetic_frame['values']: value['flags'] = 1
        arithmetic_frame['values'][-1].update(action=7, operation=1, kind=1, encoding=1)
        arithmetic_frame['values'].append(row(fn, 16, 1, kind=1, payload=1, flags=1))
        execute(label+'-seeded-operation', arithmetic, arithmetic_frame, 1, 31)

        compatible, compatible_frame = copy.deepcopy(body), copy.deepcopy(frame)
        cfn = compatible['functions'][-1]
        cfn['blocks'][0]['instructions'][2] = const(10, 1)
        cfn['blocks'][1]['instructions'][0]['dst_type'] = 'i1'
        for value in compatible_frame['values']: value['flags'] = 1
        compatible_frame['values'][0]['payload'] = 1
        cfn['blocks'][3]['instructions'] = [const(4, 30), const(5, 99),
            dict(op='select', dst=6, cond=3, then_val=4, else_val=5), dict(op='ret', value=6)]
        execute(label+'-compatible-i1', compatible, compatible_frame, 1, 1, writes=3)
        pending = copy.deepcopy(compatible)
        pending['functions'][-1]['blocks'][3]['instructions'].insert(0,
            dict(op='phi', dst=24, dst_type='i64', incoming=[[3, 1]]))
        compile_case(label+'-i1-to-i64-pending', pending, compatible_frame,
            'static_v2_phi_original_i1_pending')
        incompatible = copy.deepcopy(compatible)
        incompatible['functions'][-1]['blocks'][0]['instructions'][2] = const(10, 30)
        incompatible_frame = copy.deepcopy(compatible_frame)
        incompatible_frame['values'][0]['payload'] = 30
        compile_case(label+'-i1-constant-width', incompatible, incompatible_frame,
            'static_v2_phi_original_width')


        # The boxed Bool projection aliases an i1 producer, despite i64 storage.
        select_body, select_frame = witness(nested)
        select_fn = select_body['functions'][-1]
        make = copy.deepcopy(boxed_fn['blocks'][0]['instructions'][-2])
        project = copy.deepcopy(boxed_fn['blocks'][2]['instructions'][0])
        project['payload_type'] = 'Bool'
        select_body['boxed_sum_abi_plans'] = copy.deepcopy(boxed['boxed_sum_abi_plans'])
        select_fn['blocks'][0]['instructions'] = prefix + [const(18, 0),
            dict(op='unop', dst=17, src=18, operation='not'), make, project, const(10, 30),
            dict(op='select', dst=3, cond=11, then_val=11, else_val=10),
            dict(op='map_literal_entry_write', receiver=1, key=2, value=3), const(4, 30), dict(op='ret', value=4)]
        select_frame['values'] = [row(select_fn, 11, 2, kind=2, encoding=1, flags=1),
            row(select_fn, 10, 1, kind=1, payload=30, flags=1), row(select_fn, 3, 5, flags=1)]
        select_frame['maps'][-1]['instruction'] = 8
        execute(f'control-{nested}-select-boxed-bool-alias', select_body, select_frame, 2, 1)

        future, future_frame = copy.deepcopy(boxed), copy.deepcopy(boxed_frame)
        future_fn = future['functions'][-1]
        future_fn['blocks'][0]['instructions'][-3:-2] = [const(18, 0),
            dict(op='unop', dst=17, src=18, operation='not')]
        project = future_fn['blocks'][2]['instructions'][0]
        project.update(dst=19, payload_type='Bool')
        future_fn['blocks'][2]['instructions'].insert(1,
            dict(op='select', dst=11, cond=16, then_val=19, else_val=19))
        future_frame['maps'][-1]['instruction'] += 1
        future_frame['values'][-1] = row(future_fn, 11, 5, flags=1)
        future_frame['values'].append(row(future_fn, 19, 2, kind=2, encoding=1, flags=1))
        future_fn['blocks'][1]['instructions'].pop(2)
        future_frame['maps'].pop(1)
        future_fn['blocks'][3]['instructions'] = [const(4, 29),
            dict(op='binop', dst=23, lhs=3, rhs=4, operation='+'), dict(op='ret', value=23)]
        future_fn['blocks'][3]['instructions'].insert(0,
            dict(op='map_literal_entry_write', receiver=1, key=2, value=3))
        future_frame['maps'].append(dict(function=fn['name'], block=3, instruction=0, kind=2))
        execute(label+'-future-select-normalized', future, future_frame, 2, 1, writes=2)
