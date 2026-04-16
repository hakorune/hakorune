use super::*;

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
                    } if callee == SUBSTRING_CONCAT3_EXTERN => {
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
                        callee:
                            Some(Callee::Method {
                                method,
                                ..
                            }),
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
