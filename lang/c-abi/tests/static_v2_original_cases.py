"""Original producers through the real retained compiler and kernel storage."""
import copy
import subprocess


def run_original_cases(compile_case, witness, const, root, kernel, env, no_core):
    def execute(label, body, frame, kind, bits=30, handle=None, type_id=None, writes=1, text_value='cat'):
        obj, ir = compile_case(label, body, frame)
        executable = obj.with_suffix('.exe')
        link = subprocess.run(['cc', '-no-pie', str(obj), str(root / 'lang/c-abi/tests/static_v2_runtime_probe.c'), kernel,
            '-Wl,--wrap=nyash.map.literal_store_v1', '-Wl,--wrap=nyash.box.from_i8_string_const_len_v1',
            '-ldl', '-lpthread', '-lm', '-o', str(executable)], capture_output=True)
        assert link.returncode == 0, link.stderr
        runtime = dict(env, EXPECT_KIND=str(kind), EXPECT_BITS=str(bits), EXPECT_WRITES=str(writes), EXPECT_KEY_HEX='6b',
            EXPECT_HANDLE_TYPE=handle or '', EXPECT_TEXT=text_value)
        if type_id: runtime['EXPECT_TYPE_ID'] = str(type_id)
        result = subprocess.run([str(executable)], text=True, capture_output=True, env=runtime, preexec_fn=no_core)
        assert result.returncode == 0 and 'kernel-readback-ok' in result.stdout, (label, result)
        if label == 'original-False-MapBox':
            result = subprocess.run([str(executable)], text=True, capture_output=True,
                env=dict(runtime, CHECK_NON_HOST_CONTRACT='1'), preexec_fn=no_core)
            assert result.returncode == 0 and 'non-host-contract-ok' in result.stdout, result
        return ir.read_text()

    def replace(body, frame, instructions, kind):
        fn = body['functions'][-1]
        fn['blocks'][0]['instructions'][2:3] = instructions
        frame['values'] = [dict(function=fn['name'], value=3, action=2, kind=kind, encoding=1, flags=1)]
        frame['maps'] = [dict(function=fn['name'], block=0, instruction=i, kind=1 if ins['op'] == 'newbox' else 2)
            for i, ins in enumerate(fn['blocks'][0]['instructions'])
            if ins['op'] == 'map_literal_entry_write' or ins.get('target', {}).get('kind') == 'intrinsic_map']
        return fn

    for nested in (False, True):
        for name in ('MapBox', 'ArrayBox', 'DirectArrayI64', 'User', 'intrinsic_array'):
            body, frame = witness(nested)
            instruction = dict(op='newbox', dst=3, args=[])
            if name == 'intrinsic_array': instruction['target'] = dict(kind=name)
            else: instruction['type'] = name
            fn = replace(body, frame, [instruction], 5)
            if name == 'User':
                body['typed_object_plans'] = [dict(box_name='User', type_id=11, field_count=0, fields=[])]
            if name == 'intrinsic_array':
                frame.setdefault('calls', []).append(dict(function=fn['name'], block=0, instruction=2,
                    kind=8, arity=0, dst=3, flags=1))
            if name in ('User', 'DirectArrayI64'):
                compile_case(f'original-{nested}-{name}-nonhost', body, frame, 'static_v2_non_host_handle')
                continue
            execute(f'original-{nested}-{name}', body, frame, 5,
                handle='MapBox' if name == 'MapBox' else None if name == 'User' else 'ArrayBox',
                type_id=11 if name == 'User' else None)
            if name in ('ArrayBox', 'intrinsic_array'):
                compile_case(f'original-{nested}-{name}-configured-raw', body, frame,
                    'static_v2_non_host_handle', settings={'HAKO_ARRAY_SLOT_STORE': 'direct_array_i64_exact'})
        body, frame = witness(nested)
        replace(body, frame, [dict(op='newbox', dst=3, type='StringBox', args=[2])], 5)
        body['typed_object_plans'] = [dict(box_name='StringBox', type_id=11, field_count=0, fields=[])]
        compile_case(f'original-{nested}-alias-not-allocation', body, frame,
            'static_v2_non_host_handle' if nested else 'static_v2_alias_is_not_allocation')
        for call_kind in (1, 3):
            body, frame = witness(nested)
            call = dict(op='mir_call', dst=3, mir_call=dict(callee=dict(type='Global', name='not_the_target'), args=[]))
            fn = replace(body, frame, [call], 1)
            body['functions'][0].setdefault('metadata', {}).setdefault('same_module_function_definitions', []).append(
                dict(target_symbol='answer', definition_kind='same_module_function'))
            body['functions'].append(dict(name='answer', params=[], metadata={}, blocks=[dict(id=0, instructions=[const(1, 30), dict(op='ret', value=1)])]))
            frame.setdefault('calls', []).append(dict(function=fn['name'], block=0, instruction=2, kind=call_kind, arity=0, target='answer'))
            execute(f'original-{nested}-call-{call_kind}', body, frame, 1)
            bad = copy.deepcopy(frame); bad['calls'].pop()
            compile_case(f'original-{nested}-missing-call-{call_kind}', body, bad, 'value-action')

        for runtime in (False, True):
            for mode in ('tag', 'i64', 'bool', 'string', 'unit'):
                storage = 'handle' if mode == 'string' else 'i64'
                payload = 5
                prefix = [const(5, 'cat', dict(kind='handle', box_type='StringBox'))] if mode == 'string' else [const(5, 30)]
                if mode == 'bool':
                    prefix = [const(6, 0), dict(op='unop', dst=5, src=6, operation='not')]
                tag = 0 if mode == 'unit' else 1
                make = dict(op='variant_make', dst=7, enum='Option', variant='None' if tag == 0 else 'Some', tag=tag,
                    boxed_sum_abi_plan_id=0, boxed_sum_payload_storage='none' if tag == 0 else storage,
                    variant_binding=dict(dst=7, tag_const=tag, tag_reg=0, payload_reg=payload if tag else 0,
                        has_payload=bool(tag), enum_name='Option', copy_alias_payload=False, const_zero_result=False,
                        boxed_sum_abi_plan_id=0))
                if tag: make['payload'] = payload
                instructions = prefix + [make]
                source = 7
                if runtime:
                    instructions += [const(8, 1), dict(op='select', dst=9, cond=8, then_val=7, else_val=7)]
                    source = 9
                if mode == 'unit':
                    make['dst'] = 3; make['variant_binding']['dst'] = 3
                    instructions = prefix + [make]
                elif mode == 'tag':
                    instructions.append(dict(op='variant_tag', dst=3, value=source, enum='Option', boxed_sum_abi_plan_id=0))
                else:
                    instructions.append(dict(op='variant_project', dst=3, value=source, enum='Option', variant='Some', tag=1,
                        boxed_sum_abi_plan_id=0, boxed_sum_payload_storage=storage,
                        payload_type='Bool' if mode == 'bool' else 'String' if mode == 'string' else 'Integer'))
                body, frame = witness(nested)
                kind = 5 if mode in ('string', 'unit') else 2 if mode == 'bool' else 1
                fn = replace(body, frame, instructions, kind)
                body['boxed_sum_abi_plans'] = [dict(version='boxed_runtime_v2', plan_id=0,
                    shape_key='Option|0:none,1:' + storage, enum_name='Option', runtime_type_id=700000,
                    runtime_box_name='__NyVariant_Option', tag_storage='i64', variants=[
                        dict(name='None', tag=0, payload_storage='none'), dict(name='Some', tag=1, payload_storage=storage)])]
                label = f'boxed-{nested}-{runtime}-{mode}'
                if mode == 'unit':
                    compile_case(label + '-nonhost', body, frame, 'original-consumer-unsupported')
                    continue
                text = execute(label, body, frame, kind, bits=1 if mode in ('tag', 'bool') else 30,
                    handle='StringBox' if mode == 'string' else None, type_id=700000 if mode == 'unit' else None)
                if runtime and mode != 'unit': assert 'boxed_sum_' + ('type_ok' if mode == 'tag' else 'project_type') in text
                if mode == 'bool': assert 'boxed_sum_i64_payload_0_' in text and 'zext i1' in text
                if mode == 'string':
                    concat_body, concat_frame = copy.deepcopy(body), copy.deepcopy(frame)
                    concat_fn = concat_body['functions'][-1]
                    ci = concat_fn['blocks'][0]['instructions']
                    projection = next(ins for ins in ci if ins.get('dst') == 3)
                    projection['dst'] = 23
                    at = ci.index(projection) + 1
                    ci[at:at] = [const(6, 'dog', dict(kind='handle', box_type='StringBox')),
                        dict(op='binop', dst=3, lhs=23, rhs=6, operation='+')]
                    concat_frame['values'] = [dict(function=concat_fn['name'], value=v, action=2, kind=5, flags=1, encoding=1) for v in (23, 6)]
                    concat_frame['values'].append(dict(function=concat_fn['name'], value=3, action=7, operation=5, kind=5, flags=1, encoding=1))
                    concat_frame['maps'][-1]['instruction'] += 2
                    concat_ir = execute(label + '-concat', concat_body, concat_frame, 5, handle='StringBox', text_value='catdog')
                    if runtime: assert '%r3 = call i64 @nyash.string.concat_hh(i64 %r23, i64 %r6)' in concat_ir, concat_ir
                    if runtime:
                        pair_body, pair_frame = copy.deepcopy(concat_body), copy.deepcopy(concat_frame)
                        pair_ins = pair_body['functions'][-1]['blocks'][0]['instructions']
                        rhs = next(i for i, ins in enumerate(pair_ins) if ins.get('dst') == 6)
                        pair_ins[rhs] = dict(projection, dst=24)
                        next(ins for ins in pair_ins if ins.get('dst') == 3)['rhs'] = 24
                        next(row for row in pair_frame['values'] if row['value'] == 6)['value'] = 24
                        execute(label + '-two-projections', pair_body, pair_frame, 5,
                            handle='StringBox', text_value='catcat')
                if mode == 'i64':
                    loop_body, loop_frame = copy.deepcopy(body), copy.deepcopy(frame)
                    loop_fn = loop_body['functions'][-1]
                    all_ins = loop_fn['blocks'][0]['instructions']
                    pi = next(i for i, ins in enumerate(all_ins) if ins.get('dst') == 3)
                    loop_fn['blocks'] = [
                        dict(id=0, instructions=all_ins[:pi] + [const(4, 30), const(10, 0), const(11, 1), const(12, 2), dict(op='jump', target=1)]),
                        dict(id=1, instructions=[dict(op='phi', dst=20, incoming=[[10, 0], [21, 2]]),
                            dict(op='compare', dst=22, operation='<', lhs=20, rhs=12), dict(op='branch', cond=22, then=2, **{'else': 3})]),
                        dict(id=2, instructions=[all_ins[pi], dict(op='map_literal_entry_write', receiver=1, key=2, value=3),
                            dict(op='variant_tag', dst=24, value=source, enum='Option', boxed_sum_abi_plan_id=0),
                            dict(op='binop', dst=21, lhs=20, rhs=11, operation='+'), dict(op='jump', target=1)]),
                        dict(id=3, instructions=[dict(op='ret', value=4)])]
                    loop_frame['maps'] = [loop_frame['maps'][0], dict(function=loop_fn['name'], block=2, instruction=1, kind=2)]
                    loop_ir = execute(label + '-backedge', loop_body, loop_frame, 1, writes=2)
                    assert '[ %r21, %exact_status_continue_2_2 ]' in loop_ir
                if mode == 'i64' and not runtime and nested:
                    cmp_body, cmp_frame = copy.deepcopy(body), copy.deepcopy(frame)
                    cmp_fn = cmp_body['functions'][-1]
                    ci = cmp_fn['blocks'][0]['instructions']
                    unit = copy.deepcopy(make)
                    unit.update(dst=30, tag=0, variant='None', boxed_sum_payload_storage='none')
                    unit.pop('payload', None)
                    unit['variant_binding'].update(dst=30, tag_const=0, payload_reg=0, has_payload=False)
                    ci[-1:-1] = [unit, dict(op='compare', dst=31, lhs=30, rhs=30, operation='==')]
                    ci[-1]['value'] = 31
                    compile_case(label + '-unit-compare-stop', cmp_body, cmp_frame, 'static_v2_boxed_compare_consumer_pending')
                if mode in ('tag', 'i64'):
                    bad = copy.deepcopy(body)
                    projected = next(ins for ins in bad['functions'][-1]['blocks'][0]['instructions'] if ins.get('dst') == 3)
                    projected['boxed_sum_abi_plan_id'] = 99
                    compile_case(label + '-bad-plan', bad, frame, 'static_v2_boxed_site')
