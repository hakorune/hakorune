"""Expanded Formal ABI through real selected calls, object and kernel storage."""
import copy
import subprocess


def run_formal_cases(compile_case, witness, const, root, kernel, env, no_core):
    def execute(label, body, frame, kind, bits, writes=1):
        obj, ir = compile_case(label, body, frame)
        exe = obj.with_suffix('.exe')
        linked = subprocess.run(['cc', '-no-pie', str(obj), str(root / 'lang/c-abi/tests/static_v2_runtime_probe.c'), kernel,
            '-Wl,--wrap=nyash.map.literal_store_v1', '-Wl,--wrap=nyash.box.from_i8_string_const_len_v1',
            '-ldl', '-lpthread', '-lm', '-o', str(exe)], capture_output=True)
        assert linked.returncode == 0, linked.stderr
        ran = subprocess.run([str(exe)], capture_output=True, text=True, preexec_fn=no_core,
            env=dict(env, EXPECT_KIND=str(kind), EXPECT_BITS=str(bits), EXPECT_WRITES=str(writes), EXPECT_KEY_HEX='6b'))
        assert ran.returncode == 0 and 'kernel-readback-ok' in ran.stdout, (label, ran)
        symbols = subprocess.check_output(['nm', '-g', str(obj)], text=True)
        assert 'stash' not in symbols and '__map_stash' not in symbols, symbols
        text = ir.read_text().replace('@__map_stash(', '@"__map_stash"(')
        assert 'define internal i64 @"__map_stash"' in text
        assert 'define i64 @"stash"' not in text
        return text

    def setup(nested, kind, bits, original=False):
        body, frame = witness(nested)
        owner = body['functions'][-1]
        stash = copy.deepcopy(owner)
        stash.update(name='stash', params=[10])
        stash['blocks'][0]['instructions'].pop(2)
        stash['blocks'][0]['instructions'][2]['value'] = 10
        if original: stash['blocks'][0]['instructions'][-1]['value'] = 10
        body['functions'][0].setdefault('metadata', {}).setdefault('same_module_function_definitions', []).append(
            dict(target_symbol='stash', definition_kind='same_module_function'))
        owner['blocks'][0]['instructions'] = [const(5, None if kind == 3 else bits, 'f64' if kind == 3 else 'i64'),
            dict(op='mir_call', dst=3, mir_call=dict(callee=dict(type='Global', name='ignored_source_name'), args=[5])),
            dict(op='ret', value=3)]
        body['functions'].append(stash)
        frame['maps'] = [dict(function='stash', block=0, instruction=0, kind=1),
            dict(function='stash', block=0, instruction=2, kind=2)]
        frame['values'] = [dict(function='stash', value=10, action=6, ordinal=0, flags=int(original)),
            dict(function=owner['name'], value=5, action=1, kind=kind, payload=bits, flags=int(original))]
        frame.setdefault('calls', []).append(dict(function=owner['name'], block=0, instruction=1, kind=3, arity=1, target='stash'))
        frame['expanded'] = [dict(function='stash', target='__map_stash')]
        return body, frame

    for nested in (False, True):
        for kind, bits, original in ((1, 30, False), (2, 1, False), (3, 0x7ff8000000000042, False), (1, 30, True)):
            body, frame = setup(nested, kind, bits, original)
            label = f'formal-{nested}-{kind}-{original}'
            text = execute(label, body, frame, kind, bits)
            signature = 'i32 %map_kind_10, i64 %map_payload_10' + (', i64 %r10' if original else '')
            assert f'@"__map_stash"({signature})' in text
            bad = copy.deepcopy(frame); bad['values'][0]['ordinal'] = 1
            compile_case(label+'-ordinal', body, bad, 'value-action')
            bad = copy.deepcopy(frame); bad['expanded'] = []
            compile_case(label+'-missing-expanded', body, bad, 'value-action')
            bad = copy.deepcopy(frame); bad['values'].pop()
            compile_case(label+'-missing-actual', body, bad, 'value-reference-closure')
            bad = copy.deepcopy(frame); bad['expanded'][0]['target'] = 'main'
            compile_case(label+'-symbol-collision', body, bad, 'function-index')
            if original:
                bad = copy.deepcopy(frame); bad['values'][-1]['flags'] = 0
                compile_case(label+'-original-demand', body, bad, 'body-coverage')

        body, frame = setup(nested, 1, 30)
        frame['calls'][-1]['kind'] = 1
        execute(f'formal-{nested}-static-method', body, frame, 1, 30)
        bad = copy.deepcopy(frame); bad['expanded'].append(dict(bad['expanded'][0]))
        compile_case(f'formal-{nested}-duplicate-expanded', body, bad, 'function-index')
        bad = copy.deepcopy(frame); bad['values'].append(dict(bad['values'][0]))
        compile_case(f'formal-{nested}-duplicate-formal', body, bad, 'value-action')
        bad = copy.deepcopy(frame); bad['values'][0]['kind'] = 1
        compile_case(f'formal-{nested}-unused-wire-kind', body, bad, 'value-action')
        bad_body = copy.deepcopy(body)
        bad_body['functions'][0]['metadata']['same_module_function_definitions'] = [
            d for d in bad_body['functions'][0]['metadata']['same_module_function_definitions'] if d['target_symbol'] != 'stash']
        compile_case(f'formal-{nested}-unplanned-definition', bad_body, frame, 'expanded-row-not-consumed')

        body, frame = setup(nested, 3, 0x7ff8000000000042)
        owner, stash = body['functions'][-2:]
        owner['blocks'][0]['instructions'].insert(1, const(6, 1))
        owner['blocks'][0]['instructions'][2]['mir_call']['args'].append(6)
        frame['calls'][-1].update(instruction=2, arity=2)
        stash['params'].append(11)
        entry = stash['blocks'][0]['instructions'][:3]
        entry += [const(13, 0), dict(op='compare', dst=14, lhs=11, rhs=13, operation='>'),
            dict(op='branch', cond=14, then=1, **{'else': 2})]
        stash['blocks'] = [dict(id=0, instructions=entry), dict(id=1, instructions=[const(15, 1),
            dict(op='binop', dst=12, lhs=11, rhs=15, operation='-'),
            dict(op='mir_call', dst=16, mir_call=dict(callee=dict(type='Global', name='ignored'), args=[10, 12])),
            dict(op='ret', value=16)]), dict(id=2, instructions=[const(4, 30), dict(op='ret', value=4)])]
        frame['calls'].append(dict(function='stash', block=1, instruction=2, kind=3, arity=2, target='stash'))
        execute(f'formal-{nested}-recursive-float', body, frame, 3, 0x7ff8000000000042, writes=2)

        body, frame = setup(nested, 1, 30)
        owner, stash = body['functions'][-2:]
        # Expanded membership may come from the existing leaf-only plan.
        defs = body['functions'][0]['metadata']['same_module_function_definitions']
        next(d for d in defs if d['target_symbol'] == 'stash')['definition_kind'] = 'leaf_i64_function'
        execute(f'formal-{nested}-leaf-only', body, frame, 1, 30)
        for target in ('ny_main', '__chosen_entry'):
            bad_body, bad = copy.deepcopy(body), copy.deepcopy(frame)
            bad['expanded'][0]['target'] = target
            if target != 'ny_main':
                bad_body['functions'][0]['attrs'] = dict(runes=[dict(name='Symbol', args=[target])])
            compile_case(f'formal-{nested}-entry-symbol-{target}', bad_body, bad, 'static_v2_entry_symbol_collision')

        plan = dict(block=0, instruction_index=1, source='global_call_routes', tier='DirectAbi',
            proof='typed_global_call_generic_i64', source_route_id='generic_i64', callee_name='ignored',
            core_op='call', route_kind='same_module', emit_kind='call', target_symbol='stash', arity=1)
        owner['metadata']['lowering_plan'] = [plan]
        execute(f'formal-{nested}-typed-shadow', body, frame, 1, 30)
        ignored = copy.deepcopy(body)
        ignored['functions'][-2]['metadata']['lowering_plan'].append(dict(plan, target_symbol='__map_stash'))
        ignored['functions'][-2]['metadata']['lowering_plan'][0]['source_symbol'] = '__map_stash'
        execute(f'formal-{nested}-first-match-only', ignored, frame, 1, 30)
        external = copy.deepcopy(body)
        external['functions'][-2]['metadata']['lowering_plan'] = [dict(plan,
            source='extern_call_routes', proof='extern_registry', symbol='__map_stash',
            source_symbol='not_the_physical_target', route_proof='extern_registry', return_shape='i64', value_demand='used')]
        compile_case(f'formal-{nested}-extern-physical-ingress', external, frame, 'static_v2_internal_symbol_ingress')
        bad = copy.deepcopy(body)
        bad['functions'][-2]['metadata']['lowering_plan'][0]['target_symbol'] = '__map_stash'
        compile_case(f'formal-{nested}-internal-ingress', bad, frame, 'static_v2_internal_symbol_ingress')
        # Keep the real typed call as a domain source, add an unshadowed old entry.
        old = dict(op='mir_call', dst=9, mir_call=dict(callee=dict(type='Method', name='old', receiver=5), args=[5]))
        owner['blocks'][0]['instructions'].insert(2, old)
        old_plan = dict(plan, instruction_index=2)
        owner['metadata']['lowering_plan'].append(old_plan)
        frame['values'][-1]['flags'] = 1
        compile_case(f'formal-{nested}-old-ingress', body, frame, 'static_v2_old_abi_ingress')
        forged = copy.deepcopy(frame)
        forged['calls'].append(dict(function=owner['name'], block=0, instruction=2, kind=3, arity=1, target='stash'))
        compile_case(f'formal-{nested}-forged-shadow', body, forged, 'function-index')

        for take_float in (False, True):
            body, frame = setup(nested, 1, 30)
            owner = body['functions'][-2]
            owner['blocks'] = [dict(id=0, instructions=[const(7, int(take_float)),
                dict(op='branch', cond=7, then=2, **{'else': 1})]),
                dict(id=1, instructions=[const(5, 30),
                    dict(op='mir_call', dst=3, mir_call=dict(callee=dict(type='Global', name='ignored'), args=[5])), dict(op='ret', value=3)]),
                dict(id=2, instructions=[const(6, None, 'f64'),
                    dict(op='mir_call', dst=8, mir_call=dict(callee=dict(type='Global', name='ignored'), args=[6])), dict(op='ret', value=8)])]
            frame['calls'][-1]['block'] = 1
            frame['calls'].append(dict(frame['calls'][-1], block=2))
            frame['values'].append(dict(function=owner['name'], value=6, action=1, kind=3, payload=0x7ff8000000000042))
            execute(f'formal-{nested}-mixed-callers-{take_float}', body, frame,
                3 if take_float else 1, 0x7ff8000000000042 if take_float else 30)

        body, frame = setup(nested, 2, 1, True)
        owner, stash = body['functions'][-2:]
        owner['blocks'][0]['instructions'][:1] = [const(18, 0), dict(op='unop', dst=5, src=18, operation='not')]
        frame['calls'][-1]['instruction'] += 1
        frame['values'][-1] = dict(function=owner['name'], value=5, action=7, kind=2, encoding=2, flags=1, operation=6)
        frame['values'].append(dict(function=owner['name'], value=18, action=1, kind=1, payload=0, flags=1))
        stash['blocks'][0]['instructions'][-1]['value'] = 4
        stash['blocks'][0]['instructions'].insert(3, dict(op='unop', dst=20, src=10, operation='not'))
        text = execute(f'formal-{nested}-original-bool-i1', body, frame, 2, 1)
        assert 'zext i1 %r5 to i64' in text

        body, frame = setup(nested, 1, 30, True)
        owner, stash = body['functions'][-2:]
        stash['blocks'][0]['instructions'][2:2] = [const(11, 0), dict(op='binop', dst=20, lhs=10, rhs=11, operation='+')]
        stash['blocks'][0]['instructions'][4]['value'] = 20
        frame['maps'][-1]['instruction'] = 4
        frame['values'] += [dict(function='stash', value=11, action=1, kind=1, payload=0, flags=1),
            dict(function='stash', value=20, action=7, kind=1, encoding=1, flags=1, operation=1)]
        execute(f'formal-{nested}-integer-operation', body, frame, 1, 30)
        owner['blocks'][0]['instructions'][-1:-1] = [const(6, 1),
            dict(op='mir_call', dst=8, mir_call=dict(callee=dict(type='Global', name='ignored'), args=[6]))]
        frame['calls'].append(dict(function=owner['name'], block=0, instruction=3, kind=3, arity=1, target='stash'))
        frame['values'].append(dict(function=owner['name'], value=6, action=1, kind=2, payload=1, flags=1))
        compile_case(f'formal-{nested}-mixed-operation-domain', body, frame, 'value-reference-closure')

        body, frame = setup(nested, 1, 30)
        owner = body['functions'][-2]
        instructions = [const(5, 30)]
        frame['calls'] = [r for r in frame['calls'] if r['function'] != owner['name']]
        for i in range(18):
            frame['calls'].append(dict(function=owner['name'], block=0, instruction=len(instructions), kind=3, arity=1, target='stash'))
            instructions.append(dict(op='mir_call', dst=20+i, mir_call=dict(callee=dict(type='Global', name='ignored'), args=[5])))
        owner['blocks'][0]['instructions'] = instructions + [dict(op='ret', value=37)]
        execute(f'formal-{nested}-many-incoming-calls', body, frame, 1, 30, writes=18)

    body, frame = setup(False, 1, 30)
    main = body['functions'][0]
    main['params'] = [5]
    main['blocks'][0]['instructions'].pop(0)
    frame['calls'][0]['instruction'] = 0
    frame['values'][1] = dict(function='main', value=5, action=6, ordinal=0)
    frame['expanded'].append(dict(function='main', target='__map_root'))
    body['functions'].append(dict(name='seed', params=[], metadata={}, blocks=[dict(id=0, instructions=[const(5, 30),
        dict(op='mir_call', dst=6, mir_call=dict(callee=dict(type='Global', name='ignored'), args=[5])), dict(op='ret', value=6)])]))
    frame['values'].append(dict(function='seed', value=5, action=1, kind=1, payload=30))
    frame['calls'].append(dict(function='seed', block=0, instruction=1, kind=3, arity=1, target='main'))
    main['metadata']['same_module_function_definitions'].append(dict(target_symbol='seed', definition_kind='same_module_function'))
    compile_case('formal-root-entry-refusal', body, frame, 'static_v2_expanded_entry')
