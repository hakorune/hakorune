use super::*;

fn test_const_suffix_add_signature(
    function: &MirFunction,
    bbid: BasicBlockId,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    value: ValueId,
) -> Option<(ValueId, String)> {
    let add = match_add_in_block(function, bbid, def_map, value)?;
    let lhs_root = resolve_value_origin(function, def_map, add.lhs);
    let rhs_root = resolve_value_origin(function, def_map, add.rhs);
    let StringSourceIdentity::ConstString(text) =
        string_source_identity(function, def_map, rhs_root)?
    else {
        return None;
    };
    Some((lhs_root, text))
}

#[test]
fn benchmark_substring_only_compiles_without_substring_len_calls() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_micro_substring_only.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared =
        crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
            .expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");
    let substring_len_calls: Vec<String> = result
        .module
        .functions
        .iter()
        .flat_map(|(name, function)| {
            function.blocks.iter().flat_map(move |(bbid, block)| {
                block
                    .instructions
                    .iter()
                    .filter_map(move |inst| match inst {
                        MirInstruction::Call {
                            callee: Some(Callee::Extern(callee)),
                            ..
                        } if callee == SUBSTRING_LEN_EXTERN => {
                            Some(format!("fn={name} bb={} inst={inst:?}", bbid.0))
                        }
                        _ => None,
                    })
            })
        })
        .collect();
    assert!(
        substring_len_calls.is_empty(),
        "benchmark should fuse substring_len_hii away, found {:?}",
        substring_len_calls
    );
}

#[test]
fn benchmark_len_substring_views_compiles_without_loop_string_consumers() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_micro_len_substring_views.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared =
        crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
            .expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");

    let mut leftover_string_consumers = Vec::new();
    for (name, function) in &result.module.functions {
        for (bbid, block) in &function.blocks {
            for inst in &block.instructions {
                match inst {
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(callee)),
                        ..
                    } if callee == SUBSTRING_LEN_EXTERN => leftover_string_consumers.push(format!(
                        "fn={name} bb={} extern={callee} inst={inst:?}",
                        bbid.0
                    )),
                    MirInstruction::Call {
                        callee:
                            Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(_),
                                ..
                            }),
                        ..
                    } if method == "length" && box_name == "RuntimeDataBox" => {
                        leftover_string_consumers.push(format!(
                            "fn={name} bb={} runtime-data length inst={inst:?}",
                            bbid.0
                        ))
                    }
                    _ => {}
                }
            }
        }
    }

    assert!(
        leftover_string_consumers.is_empty(),
        "len_substring_views should fuse away loop string consumers, found {:?}",
        leftover_string_consumers
    );
}

#[test]
fn benchmark_substring_concat_compiles_without_concat_string_consumers() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_micro_substring_concat.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared =
        crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
            .expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");

    let mut saw_insert_mid = false;
    let mut saw_helper = false;
    let mut substring_call_count = 0usize;
    let mut leftover_concat_consumers = Vec::new();
    let mut leftover_concat_lengths = Vec::new();
    let mut leftover_substring_len = Vec::new();
    for (name, function) in &result.module.functions {
        let def_map = build_value_def_map(function);
        for (bbid, block) in &function.blocks {
            for inst in &block.instructions {
                match inst {
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(callee)),
                        ..
                    } if callee == INSERT_HSI_EXTERN => {
                        saw_insert_mid = true;
                    }
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(callee)),
                        args,
                        ..
                    } if callee == "nyash.string.substring_hii" && args.len() == 3 => {
                        substring_call_count += 1;
                    }
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(callee)),
                        ..
                    } if callee == SUBSTRING_CONCAT3_EXTERN
                        || callee == SUBSTRING_CONCAT3_PUBLISH_EXPLICIT_API_OWNED_EXTERN
                        || callee == SUBSTRING_CONCAT3_PUBLISH_NEED_STABLE_OWNED_EXTERN =>
                    {
                        saw_helper = true;
                    }
                    MirInstruction::Call {
                        callee:
                            Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(receiver),
                                ..
                            }),
                        ..
                    } if box_name == "RuntimeDataBox"
                        && method == "substring"
                        && match_concat_triplet(function, *bbid, &def_map, *receiver).is_some() =>
                    {
                        leftover_concat_consumers.push(format!(
                            "fn={name} bb={} concat substring inst={inst:?}",
                            bbid.0
                        ));
                    }
                    MirInstruction::Call {
                        callee: Some(Callee::Method { method, .. }),
                        args,
                        ..
                    } if matches!(method.as_str(), "substring" | "slice") && args.len() == 2 => {
                        substring_call_count += 1;
                    }
                    MirInstruction::Call {
                        callee:
                            Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(receiver),
                                ..
                            }),
                        ..
                    } if box_name == "RuntimeDataBox"
                        && method == "length"
                        && match_concat_triplet(function, *bbid, &def_map, *receiver).is_some() =>
                    {
                        leftover_concat_lengths.push(format!(
                            "fn={name} bb={} concat length inst={inst:?}",
                            bbid.0
                        ));
                    }
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(callee)),
                        ..
                    } if callee == SUBSTRING_LEN_EXTERN => {
                        leftover_substring_len.push(format!(
                            "fn={name} bb={} substring_len inst={inst:?}",
                            bbid.0
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    assert!(
        saw_insert_mid,
        "benchmark should emit delete-oriented insert_hsi rewrite"
    );
    assert!(
        !saw_helper,
        "benchmark should retire substring_concat3 helper for the exact front"
    );
    assert!(
        leftover_concat_consumers.is_empty(),
        "substring_concat should sink concat substring consumers, found {:?}",
        leftover_concat_consumers
    );
    assert!(
        leftover_concat_lengths.is_empty(),
        "substring_concat should sink concat length consumers, found {:?}",
        leftover_concat_lengths
    );
    assert!(
        leftover_substring_len.is_empty(),
        "substring_concat should fuse loop substring_len_hii away, found {:?}",
        leftover_substring_len
    );
    assert_eq!(
        substring_call_count, 1,
        "delete-oriented rewrite should leave only the final outer substring call"
    );
}

#[test]
fn benchmark_meso_substring_concat_len_compiles_to_arithmetic_len() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_meso_substring_concat_len.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared =
        crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
            .expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");

    let mut leftover_string_consumers = Vec::new();
    for (name, function) in &result.module.functions {
        for (bbid, block) in &function.blocks {
            for inst in &block.instructions {
                match inst {
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(callee)),
                        ..
                    } if callee == SUBSTRING_LEN_EXTERN
                        || callee == "nyash.string.substring_hii" =>
                    {
                        leftover_string_consumers.push(format!(
                            "fn={name} bb={} extern={callee} inst={inst:?}",
                            bbid.0
                        ));
                    }
                    MirInstruction::Call {
                        callee:
                            Some(Callee::Method {
                                method,
                                receiver: Some(_),
                                ..
                            }),
                        ..
                    } if matches!(method.as_str(), "substring" | "slice") => {
                        leftover_string_consumers.push(format!(
                            "fn={name} bb={} method={method} inst={inst:?}",
                            bbid.0
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    assert!(
        leftover_string_consumers.is_empty(),
        "meso substring concat len should fold to arithmetic, found {:?}",
        leftover_string_consumers
    );
}

#[test]
fn benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_meso_substring_concat_array_set_loopcarry.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared =
        crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
            .expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");

    let function = result.module.get_function("main").expect("main function");
    assert_eq!(
        function
            .metadata
            .array_text_loopcarry_len_store_routes
            .len(),
        1,
        "loopcarry benchmark should expose one MIR-owned len-store route"
    );
    let route = &function.metadata.array_text_loopcarry_len_store_routes[0];
    assert_eq!(route.middle_length(), 2);
    assert_eq!(route.proof(), "insert_mid_subrange_trailing_len");
    assert!(
        !route
            .skip_instruction_indices()
            .contains(&route.instruction_index()),
        "route replaces the get instruction and should skip only covered followers"
    );

    assert_eq!(
        function.metadata.array_text_residence_sessions.len(),
        1,
        "loopcarry benchmark should expose one MIR-owned residence session candidate"
    );
    let session = &function.metadata.array_text_residence_sessions[0];
    assert_eq!(session.begin_to_header_block, session.header_block);
    assert_eq!(session.begin_placement.to_string(), "before_preheader_jump");
    assert_eq!(session.body_block, route.block());
    assert_eq!(session.update_block, route.block());
    assert_eq!(session.update_instruction_index, route.instruction_index());
    assert_eq!(session.update_placement.to_string(), "route_instruction");
    assert_eq!(session.end_block, session.exit_block);
    assert_eq!(session.end_placement.to_string(), "exit_block_entry");
    assert_eq!(session.route_instruction_index, route.instruction_index());
    assert_eq!(session.array_value, route.array_value());
    assert_eq!(session.middle_length, 2);
    assert_eq!(
        session.skip_instruction_indices,
        route.skip_instruction_indices().to_vec()
    );
    assert_eq!(session.scope.to_string(), "loop_backedge_single_body");
    assert_eq!(session.proof.to_string(), "loopcarry_len_store_only");
    let executor_contract = session
        .executor_contract
        .as_ref()
        .expect("residence session should expose nested executor contract");
    assert_eq!(
        executor_contract.execution_mode.to_string(),
        "single_region_executor"
    );
    assert_eq!(
        executor_contract.proof_region.to_string(),
        "loop_backedge_single_body"
    );
    assert_eq!(executor_contract.publication_boundary, "none");
    assert_eq!(
        executor_contract.carrier.to_string(),
        "array_lane_text_cell"
    );
    assert_eq!(
        executor_contract
            .effects
            .iter()
            .map(|effect| effect.to_string())
            .collect::<Vec<_>>(),
        vec!["store.cell", "length_only_result_carry"]
    );
    assert_eq!(
        executor_contract
            .consumer_capabilities
            .iter()
            .map(|capability| capability.to_string())
            .collect::<Vec<_>>(),
        vec!["sink_store", "length_only"]
    );
    assert_eq!(
        executor_contract.materialization_policy.to_string(),
        "text_resident_or_stringlike_slot"
    );
    let region_mapping = executor_contract
        .region_mapping
        .as_ref()
        .expect("single-region executor contract should expose loop/PHI/exit mapping");
    assert_eq!(region_mapping.array_root_value.0, 5);
    assert_eq!(region_mapping.loop_index_phi_value.0, 31);
    assert_eq!(region_mapping.loop_index_initial_value.0, 30);
    assert_eq!(region_mapping.loop_index_initial_const, 0);
    assert_eq!(region_mapping.loop_index_next_value.0, 32);
    assert_eq!(region_mapping.loop_bound_value.0, 58);
    assert_eq!(region_mapping.loop_bound_const, 180000);
    assert_eq!(region_mapping.accumulator_phi_value.0, 35);
    assert_eq!(region_mapping.accumulator_initial_value.0, 29);
    assert_eq!(region_mapping.accumulator_initial_const, 0);
    assert_eq!(region_mapping.accumulator_next_value.0, 53);
    assert_eq!(region_mapping.exit_accumulator_value.0, 35);
    assert_eq!(region_mapping.row_index_value.0, 64);
    assert_eq!(region_mapping.row_modulus_value.0, 67);
    assert_eq!(region_mapping.row_modulus_const, 64);

    let begin_block = function
        .blocks
        .get(&session.begin_block)
        .expect("begin block");
    assert!(matches!(
        begin_block.terminator.as_ref(),
        Some(MirInstruction::Jump { target, .. }) if *target == session.header_block
    ));

    let exit_block = function.blocks.get(&session.end_block).expect("end block");
    assert_eq!(exit_block.predecessors.len(), 1);
    assert!(exit_block.predecessors.contains(&session.header_block));
}

#[test]
fn benchmark_kilo_kernel_small_has_combined_edit_observer_region() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_kernel_small.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared =
        crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
            .expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");

    let function = result.module.get_function("main").expect("main function");
    assert_eq!(
        function.metadata.array_text_combined_regions.len(),
        1,
        "kilo kernel should expose one MIR-owned combined edit/observer region"
    );
    let route = &function.metadata.array_text_combined_regions[0];
    assert_eq!(
        route.proof.to_string(),
        "outer_lenhalf_edit_with_periodic_observer_store"
    );
    assert_eq!(
        route.proof_region.to_string(),
        "outer_loop_with_periodic_observer_store"
    );
    assert_eq!(route.execution_mode.to_string(), "single_region_executor");
    assert_eq!(route.loop_bound_const, 60000);
    assert_eq!(route.row_modulus_const, 64);
    assert_eq!(route.observer_period_const, 8);
    assert_eq!(route.observer_bound_const, 64);
    assert_eq!(route.edit_middle_text, "xx");
    assert_eq!(route.observer_needle_text, "line");
    assert_eq!(route.observer_suffix_text, "ln");
    assert_eq!(
        route
            .byte_boundary_proof
            .map(|proof| proof.to_string())
            .as_deref(),
        Some("ascii_preserved_text_cell"),
        "kilo kernel should carry a MIR-owned byte-boundary proof for the covered ASCII text-cell region"
    );
    assert_ne!(
        route.accumulator_phi_value, route.outer_index_phi_value,
        "result accumulator must not alias the loop-index PHI"
    );
}

#[test]
fn benchmark_substring_concat_array_set_compiles_without_helper_len_observers() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_meso_substring_concat_array_set.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared =
        crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
            .expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");

    let mut leftover_helper_lengths = Vec::new();
    for (name, function) in &result.module.functions {
        let def_map = build_value_def_map(function);
        let use_counts = build_use_counts(function);
        for (bbid, block) in &function.blocks {
            for inst in &block.instructions {
                let Some((_dst, receiver, _effects)) = match_len_call(inst) else {
                    continue;
                };
                let Some(receiver_chain) =
                    resolve_copy_chain_in_block(function, *bbid, &def_map, &use_counts, receiver)
                else {
                    continue;
                };
                if publication_helper_shape(function, &def_map, receiver_chain.root).is_some() {
                    leftover_helper_lengths
                        .push(format!("fn={name} bb={} helper-len inst={inst:?}", bbid.0));
                }
            }
        }
    }

    assert!(
        leftover_helper_lengths.is_empty(),
        "substring_concat_array_set should rewrite helper len observers, found {:?}",
        leftover_helper_lengths
    );
}

#[test]
fn benchmark_array_string_store_compiles_with_store_shared_receiver_substring() {
    ensure_ring0_initialized();
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benchmarks/bench_kilo_micro_array_string_store.hako"
    );
    let source = std::fs::read_to_string(path).expect("benchmark source");
    let prepared =
        crate::runner::modes::common_util::source_hint::prepare_source_minimal(&source, path)
            .expect("prepare benchmark source");
    let ast = NyashParser::parse_from_string(&prepared).expect("parse benchmark");
    let mut compiler = MirCompiler::with_options(true);
    let result = compiler
        .compile_with_source(ast, Some(path))
        .expect("compile benchmark");

    let function = result.module.get_function("main").expect("main").clone();
    let def_map = build_value_def_map(&function);
    let interesting_blocks: Vec<String> = function
        .blocks
        .iter()
        .filter_map(|(bbid, block)| {
            let interesting = block.instructions.iter().any(|inst| {
                match_method_set_call(inst).is_some() || match_substring_call(inst).is_some()
            });
            interesting.then(|| format!("bb{} {:?}", bbid.0, block.instructions))
        })
        .collect();
    let mut store_value = None;
    let mut trailing_receiver = None;
    let mut duplicate_const_suffix_adds = 0usize;

    for (bbid, block) in &function.blocks {
        for inst in &block.instructions {
            if let Some(store) = array_store_candidate(&function, &def_map, inst) {
                if let Some((base, suffix)) =
                    test_const_suffix_add_signature(&function, *bbid, &def_map, store.value)
                {
                    if suffix == "xy" {
                        store_value = Some((store.value, base));
                    }
                }
            }

            if let Some((_dst, receiver, _start, _end, _effects)) = match_substring_call(inst) {
                if let Some((base, suffix)) =
                    test_const_suffix_add_signature(&function, *bbid, &def_map, receiver)
                {
                    if suffix == "xy" {
                        duplicate_const_suffix_adds += 1;
                        trailing_receiver = Some((receiver, base));
                    }
                }
            }
        }
    }

    let Some((store_value, store_base)) = store_value else {
        panic!("expected array store const-suffix producer in rewritten benchmark");
    };
    let Some((trailing_receiver, trailing_base)) = trailing_receiver else {
        panic!("expected trailing substring const-suffix producer in rewritten benchmark");
    };

    assert_eq!(
        trailing_receiver, store_value,
        "trailing substring should reuse store-side producer after rewrite"
    );
    assert_eq!(
        trailing_base, store_base,
        "trailing substring and store should share the same base after rewrite"
    );
    assert_eq!(
        duplicate_const_suffix_adds, 1,
        "benchmark should keep only one xy const-suffix producer after compile; blocks={interesting_blocks:?}"
    );
}
