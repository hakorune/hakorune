static int hako_llvmc_emit_indexof_text_state_residence_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long rows,
    long long ops,
    long long flip_period,
    const char* line_seed,
    long long line_seed_len,
    const char* none_seed,
    long long none_seed_len) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long row_mask;
  long long flip_mask;
  int use_flip;

  if (!obj_out || rows != 64 || ops <= 0 || flip_period < 0) {
    return -1;
  }
  if (!line_seed || !none_seed || line_seed_len <= 0 || none_seed_len <= 0) {
    return -1;
  }

  row_mask = rows - 1;
  use_flip = flip_period > 0;
  flip_mask = use_flip ? (flip_period - 1) : 0;
  if ((rows & row_mask) != 0 || row_mask <= 0) {
    return -1;
  }
  if (use_flip && ((flip_period & flip_mask) != 0 || flip_mask <= 0 || flip_period != 16)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_indexof_text_state_residence_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (indexOf text-state residence loop ascii)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(f, "@.indexof_residence_line = private unnamed_addr constant [%lld x i8] c\"", line_seed_len);
  hako_llvmc_emit_llvm_byte_string(f, line_seed, line_seed_len);
  fprintf(f, "\", align 1\n");
  fprintf(f, "@.indexof_residence_none = private unnamed_addr constant [%lld x i8] c\"", none_seed_len);
  hako_llvmc_emit_llvm_byte_string(f, none_seed, none_seed_len);
  fprintf(f, "\", align 1\n");
  fprintf(f, "\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%lines = alloca [%lld x ptr], align 8\n", rows);
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%hits = alloca i64, align 8\n");
  fprintf(f, "  %%lines.ptr = getelementptr inbounds [%lld x ptr], ptr %%lines, i64 0, i64 0\n", rows);
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  br label %%prefill\n\n");

  fprintf(f, "prefill:\n");
  fprintf(f, "  %%i.pre = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%prefill.cond = icmp slt i64 %%i.pre, 64\n");
  fprintf(f, "  br i1 %%prefill.cond, label %%prefill.body, label %%prefill.exit\n\n");

  fprintf(f, "prefill.body:\n");
  fprintf(f, "  %%prefill.slot = getelementptr inbounds ptr, ptr %%lines.ptr, i64 %%i.pre\n");
  fprintf(f, "  %%prefill.evenmask = and i64 %%i.pre, 1\n");
  fprintf(f, "  %%prefill.is_even = icmp eq i64 %%prefill.evenmask, 0\n");
  fprintf(f, "  %%prefill.val = select i1 %%prefill.is_even, ptr @.indexof_residence_line, ptr @.indexof_residence_none\n");
  fprintf(f, "  store ptr %%prefill.val, ptr %%prefill.slot, align 8\n");
  fprintf(f, "  %%i.pre.next = add i64 %%i.pre, 1\n");
  fprintf(f, "  store i64 %%i.pre.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%prefill\n\n");

  fprintf(f, "prefill.exit:\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%hits, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%row = and i64 %%i.cur, %lld\n", row_mask);
  fprintf(f, "  %%slot = getelementptr inbounds ptr, ptr %%lines.ptr, i64 %%row\n");
  fprintf(f, "  %%cur = load ptr, ptr %%slot, align 8\n");
  fprintf(f, "  %%found = icmp eq ptr %%cur, @.indexof_residence_line\n");
  fprintf(f, "  %%hits.cur = load i64, ptr %%hits, align 8\n");
  fprintf(f, "  %%hits.inc = select i1 %%found, i64 1, i64 0\n");
  fprintf(f, "  %%hits.next = add i64 %%hits.cur, %%hits.inc\n");
  fprintf(f, "  store i64 %%hits.next, ptr %%hits, align 8\n");
  if (use_flip) {
    fprintf(f, "  %%flipmask = and i64 %%i.cur, %lld\n", flip_mask);
    fprintf(f, "  %%flip = icmp eq i64 %%flipmask, 0\n");
    fprintf(f, "  br i1 %%flip, label %%flip.then, label %%inc\n\n");

    fprintf(f, "flip.then:\n");
    fprintf(f, "  %%newval = select i1 %%found, ptr @.indexof_residence_none, ptr @.indexof_residence_line\n");
    fprintf(f, "  store ptr %%newval, ptr %%slot, align 8\n");
    fprintf(f, "  br label %%inc\n\n");
  } else {
    fprintf(f, "  br label %%inc\n\n");
  }

  fprintf(f, "inc:\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%hits.fin = load i64, ptr %%hits, align 8\n");
  fprintf(f, "  %%ret = add i64 %%hits.fin, %lld\n", rows);
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  {
    rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  }
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

typedef struct hako_llvmc_indexof_state_residence_payload {
  const char* variant;
  long long rows;
  long long ops;
  long long flip_period;
  const char* line_seed;
  long long line_seed_len;
  const char* none_seed;
  long long none_seed_len;
  const char* needle;
  long long needle_len;
  const char* proof;
  const char* result_use;
  const char* backend_action;
  const char* line_seed_outcome;
  const char* none_seed_outcome;
} hako_llvmc_indexof_state_residence_payload;

static int hako_llvmc_read_indexof_candidate_outcomes(
    yyjson_val* route,
    hako_llvmc_indexof_state_residence_payload* out) {
  yyjson_val* outcomes;
  size_t i;

  if (!route || !out || !out->line_seed || !out->none_seed) return 0;
  outcomes = yyjson_obj_get(route, "candidate_outcomes");
  if (!(outcomes && yyjson_is_arr(outcomes))) return 0;

  for (i = 0; i < yyjson_arr_size(outcomes); i++) {
    yyjson_val* item = yyjson_arr_get(outcomes, i);
    const char* literal = item ? yyjson_get_str(yyjson_obj_get(item, "literal")) : NULL;
    const char* outcome = item ? yyjson_get_str(yyjson_obj_get(item, "outcome")) : NULL;
    if (!(literal && outcome)) return 0;
    if (hako_llvmc_str_eq(literal, out->line_seed)) {
      if (out->line_seed_outcome) return 0;
      out->line_seed_outcome = outcome;
    } else if (hako_llvmc_str_eq(literal, out->none_seed)) {
      if (out->none_seed_outcome) return 0;
      out->none_seed_outcome = outcome;
    } else {
      return 0;
    }
  }

  return out->line_seed_outcome && out->none_seed_outcome;
}

static int hako_llvmc_validate_array_text_state_residence_contract(yyjson_val* route) {
  const char* observer_kind;
  const char* residence;
  const char* result_repr;
  const char* consumer_capability;
  const char* publication_boundary;

  if (!(route && yyjson_is_obj(route))) return 0;
  observer_kind = yyjson_get_str(yyjson_obj_get(route, "observer_kind"));
  residence = yyjson_get_str(yyjson_obj_get(route, "residence"));
  result_repr = yyjson_get_str(yyjson_obj_get(route, "result_repr"));
  consumer_capability = yyjson_get_str(yyjson_obj_get(route, "consumer_capability"));
  publication_boundary = yyjson_get_str(yyjson_obj_get(route, "publication_boundary"));

  return hako_llvmc_str_eq(observer_kind, "indexof") &&
         hako_llvmc_str_eq(residence, "loop_local_pointer_array") &&
         hako_llvmc_str_eq(result_repr, "scalar_i64") &&
         hako_llvmc_str_eq(consumer_capability, "direct_array_text_state_residence") &&
         hako_llvmc_str_eq(publication_boundary, "none");
}

static int hako_llvmc_read_indexof_state_residence_contract(
    yyjson_val* fn,
    const char* expected_variant,
    const char* expected_proof,
    int require_flip,
    hako_llvmc_indexof_state_residence_payload* out) {
  yyjson_val* metadata;
  yyjson_val* route;
  yyjson_val* flip_value;
  long long actual_line_seed_len = 0;
  long long actual_none_seed_len = 0;
  long long actual_needle_len = 0;

  if (!fn || !expected_variant || !expected_proof || !out) return 0;
  memset(out, 0, sizeof(*out));
  metadata = yyjson_obj_get(fn, "metadata");
  route = metadata ? yyjson_obj_get(metadata, "array_text_state_residence_route") : NULL;
  if (!(route && yyjson_is_obj(route))) return 0;
  if (!hako_llvmc_validate_array_text_state_residence_contract(route)) return 0;
  route = yyjson_obj_get(route, "temporary_indexof_seed_payload");
  if (!(route && yyjson_is_obj(route))) return 0;

  out->variant = yyjson_get_str(yyjson_obj_get(route, "variant"));
  out->rows = (long long)yyjson_get_sint(yyjson_obj_get(route, "rows"));
  out->ops = (long long)yyjson_get_sint(yyjson_obj_get(route, "ops"));
  flip_value = yyjson_obj_get(route, "flip_period");
  out->flip_period = flip_value ? (long long)yyjson_get_sint(flip_value) : 0;
  out->line_seed = yyjson_get_str(yyjson_obj_get(route, "line_seed"));
  out->line_seed_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "line_seed_len"));
  out->none_seed = yyjson_get_str(yyjson_obj_get(route, "none_seed"));
  out->none_seed_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "none_seed_len"));
  out->needle = yyjson_get_str(yyjson_obj_get(route, "needle"));
  out->needle_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "needle_len"));
  out->proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  out->result_use = yyjson_get_str(yyjson_obj_get(route, "result_use"));
  out->backend_action = yyjson_get_str(yyjson_obj_get(route, "backend_action"));

  if (!(out->variant &&
        out->line_seed &&
        out->none_seed &&
        out->needle &&
        out->proof &&
        out->result_use &&
        out->backend_action)) {
    return 0;
  }
  if (!hako_llvmc_read_indexof_candidate_outcomes(route, out)) {
    return 0;
  }
  if (!(hako_llvmc_ascii_strlen(out->line_seed, &actual_line_seed_len) &&
        hako_llvmc_ascii_strlen(out->none_seed, &actual_none_seed_len) &&
        hako_llvmc_ascii_strlen(out->needle, &actual_needle_len))) {
    return 0;
  }
  if (!(hako_llvmc_str_eq(out->variant, expected_variant) &&
        hako_llvmc_str_eq(out->proof, expected_proof) &&
        hako_llvmc_str_eq(out->line_seed, "line-seed") &&
        hako_llvmc_str_eq(out->none_seed, "none-seed") &&
        hako_llvmc_str_eq(out->needle, "line") &&
        hako_llvmc_str_eq(out->result_use, "found_predicate") &&
        hako_llvmc_str_eq(out->backend_action, "literal_membership_predicate") &&
        hako_llvmc_str_eq(out->line_seed_outcome, "found") &&
        hako_llvmc_str_eq(out->none_seed_outcome, "not_found"))) {
    return 0;
  }
  if (!(out->rows == 64 &&
        out->ops == 400000 &&
        out->line_seed_len == actual_line_seed_len &&
        out->none_seed_len == actual_none_seed_len &&
        out->needle_len == actual_needle_len &&
        out->line_seed_len == 9 &&
        out->none_seed_len == 9 &&
        out->needle_len == 4)) {
    return 0;
  }
  if (require_flip) {
    if (out->flip_period != 16) return 0;
  } else if (out->flip_period != 0) {
    return 0;
  }
  return 1;
}

static int hako_llvmc_match_indexof_text_state_residence_fn(
    yyjson_val* fn,
    const char* obj_out,
    const char* expected_variant,
    const char* expected_proof,
    int require_flip,
    const char* trace_stage,
    const char* trace_extra) {
  hako_llvmc_entry_rune_selection selection;
  hako_llvmc_indexof_state_residence_payload route;

  if (!fn || !obj_out) return -1;
  hako_llvmc_select_entry_runes(fn, &selection);
  if (!hako_llvmc_read_indexof_state_residence_contract(
          fn,
          expected_variant,
          expected_proof,
          require_flip,
          &route)) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          trace_stage,
          "skip",
          "metadata_contract_mismatch",
          "array_text_state_residence_route");
    }
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    hako_llvmc_emit_route_trace(
        trace_stage,
        "emit",
        "text_state_residence",
        trace_extra);
  }

  return hako_llvmc_emit_indexof_text_state_residence_ir(
      obj_out,
      &selection,
      route.rows,
      route.ops,
      route.flip_period,
      route.line_seed,
      route.line_seed_len,
      route.none_seed,
      route.none_seed_len);
}

static int hako_llvmc_match_indexof_leaf_text_state_residence_fn(
    yyjson_val* fn,
    const char* obj_out) {
  return hako_llvmc_match_indexof_text_state_residence_fn(
      fn,
      obj_out,
      "leaf",
      "kilo_leaf_array_string_indexof_const_10block",
      0,
      "indexof_leaf_text_state_residence",
      "kilo_leaf_array_string_indexof_const mir_route_plan");
}

static int hako_llvmc_match_indexof_line_text_state_residence_fn(
    yyjson_val* fn,
    const char* obj_out) {
  return hako_llvmc_match_indexof_text_state_residence_fn(
      fn,
      obj_out,
      "line",
      "kilo_micro_indexof_line_15block",
      1,
      "indexof_line_text_state_residence",
      "kilo_micro_indexof_line mir_route_plan");
}
