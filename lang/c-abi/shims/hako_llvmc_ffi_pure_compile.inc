static int hako_llvmc_exact_seed_backend_route_supported(
    const char* tag,
    const char* source_route,
    const char* proof,
    long long selected_value) {
  if (strcmp(tag, "array_string_store_micro") == 0 &&
      strcmp(source_route, "array_string_store_micro_seed_route") == 0 &&
      strcmp(proof, "kilo_micro_array_string_store_8block") == 0 &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "array_getset_micro") == 0 &&
      strcmp(source_route, "array_getset_micro_seed_route") == 0 &&
      strcmp(proof, "kilo_micro_array_getset_7block") == 0 &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "array_rmw_add1_leaf") == 0 &&
      strcmp(source_route, "array_rmw_add1_leaf_seed_route") == 0 &&
      strcmp(proof, "kilo_leaf_array_rmw_add1_7block") == 0 &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "sum_variant_tag_local") == 0 &&
      strcmp(source_route, "sum_variant_tag_seed_route") == 0 &&
      strcmp(proof, "sum_variant_tag_local_aggregate_seed") == 0 &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "sum_variant_project_local") == 0 &&
      strcmp(source_route, "sum_variant_project_seed_route") == 0 &&
      strcmp(proof, "sum_variant_project_local_aggregate_seed") == 0 &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "userbox_point_local_scalar") == 0 &&
      strcmp(source_route, "userbox_local_scalar_seed_route") == 0 &&
      strcmp(proof, "userbox_point_field_local_scalar_seed") == 0 &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "userbox_flag_pointf_local_scalar") == 0 &&
      strcmp(source_route, "userbox_local_scalar_seed_route") == 0 &&
      (strcmp(proof, "userbox_flag_field_local_scalar_seed") == 0 ||
       strcmp(proof, "userbox_pointf_field_local_scalar_seed") == 0) &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "userbox_loop_micro") == 0 &&
      strcmp(source_route, "userbox_loop_micro_seed_route") == 0 &&
      (strcmp(proof, "userbox_point_add_loop_micro_seed") == 0 ||
       strcmp(proof, "userbox_flag_toggle_loop_micro_seed") == 0) &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "userbox_known_receiver_method_seed") == 0 &&
      strcmp(source_route, "userbox_known_receiver_method_seed_route") == 0 &&
      (strcmp(proof, "userbox_counter_step_local_i64_seed") == 0 ||
       strcmp(proof, "userbox_counter_step_chain_local_i64_seed") == 0 ||
       strcmp(proof, "userbox_counter_step_chain_micro_seed") == 0 ||
       strcmp(proof, "userbox_counter_step_micro_seed") == 0 ||
       strcmp(proof, "userbox_point_sum_local_i64_seed") == 0 ||
       strcmp(proof, "userbox_point_sum_micro_seed") == 0) &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "concat_const_suffix_micro") == 0 &&
      strcmp(source_route, "concat_const_suffix_micro_seed_route") == 0 &&
      strcmp(proof, "kilo_micro_concat_const_suffix_5block") == 0 &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "substring_views_only_micro") == 0 &&
      strcmp(source_route, "substring_views_micro_seed_route") == 0 &&
      strcmp(proof, "kilo_micro_substring_views_only_5block") == 0 &&
      selected_value == 0) {
    return 1;
  }
  if (strcmp(tag, "substring_concat_loop_ascii") == 0 &&
      strcmp(source_route, "string_kernel_plans.loop_payload") == 0 &&
      strcmp(proof, "string_kernel_plan_concat_triplet_loop_payload") == 0 &&
      selected_value > 0) {
    return 1;
  }
  return 0;
}

/* inc-codegen-view-owner: generic_pure_view begin */
typedef struct hako_llvmc_generic_pure_function_view {
  yyjson_val* fn;
  size_t index;
  yyjson_val* metadata;
  yyjson_val* blocks;
  size_t block_count;
  hako_llvmc_entry_rune_selection selection;
  int is_entry;
} hako_llvmc_generic_pure_function_view;

typedef struct hako_llvmc_generic_pure_program_view {
  yyjson_val* functions;
  size_t function_count;
  yyjson_val* fn;
  size_t entry_index;
  yyjson_val* metadata;
  yyjson_val* blocks;
  size_t block_count;
  hako_llvmc_entry_rune_selection selection;
  hako_llvmc_generic_pure_function_view entry;
} hako_llvmc_generic_pure_program_view;

static int hako_llvmc_read_generic_pure_function_view(
    yyjson_val* fn,
    size_t index,
    int is_entry,
    hako_llvmc_generic_pure_function_view* out) {
  yyjson_val* blocks;
  if (!out) return 0;
  memset(out, 0, sizeof(*out));
  if (!(fn && yyjson_is_obj(fn))) return 0;
  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks))) return 0;
  out->fn = fn;
  out->index = index;
  out->metadata = yyjson_obj_get(fn, "metadata");
  out->blocks = blocks;
  out->block_count = yyjson_arr_size(blocks);
  if (out->block_count < 1) return 0;
  out->is_entry = is_entry ? 1 : 0;
  if (out->is_entry) {
    hako_llvmc_select_entry_runes(out->fn, &out->selection);
  }
  return 1;
}

static int hako_llvmc_read_generic_pure_program_view(
    yyjson_val* root,
    hako_llvmc_generic_pure_program_view* out) {
  yyjson_val* fns;
  if (!out) return 0;
  memset(out, 0, sizeof(*out));
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) return 0;
  out->functions = fns;
  out->function_count = yyjson_arr_size(fns);
  if (out->function_count < 1) return 0;
  out->fn = hako_llvmc_find_entry_function(fns);
  if (!(out->fn && yyjson_is_obj(out->fn))) return 0;
  out->entry_index = out->function_count;
  for (size_t i = 0; i < out->function_count; i++) {
    if (yyjson_arr_get(fns, i) == out->fn) {
      out->entry_index = i;
      break;
    }
  }
  if (out->entry_index >= out->function_count) return 0;
  if (!hako_llvmc_read_generic_pure_function_view(
          out->fn,
          out->entry_index,
          1,
          &out->entry)) {
    return 0;
  }
  out->metadata = out->entry.metadata;
  out->blocks = out->entry.blocks;
  out->block_count = out->entry.block_count;
  out->selection = out->entry.selection;
  return 1;
}
/* inc-codegen-view-owner: generic_pure_view end */

static int hako_llvmc_read_exact_seed_backend_route_tag(
    const char* json_in,
    char* tag_out,
    size_t tag_out_len,
    long long* selected_value_out,
    char** err_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn;
  yyjson_val* metadata;
  yyjson_val* route;
  yyjson_val* tag_val;
  yyjson_val* source_route_val;
  yyjson_val* proof_val;
  yyjson_val* selected_value_val;
  const char* tag;
  const char* source_route;
  const char* proof;
  long long selected_value = 0;

  if (!tag_out || tag_out_len == 0) {
    return set_err_owned(err_out, "invalid exact seed backend route tag buffer");
  }
  tag_out[0] = '\0';
  if (!doc) {
    return 0;
  }

  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  fn = hako_llvmc_find_entry_function(fns);
  if (!fn) {
    yyjson_doc_free(doc);
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  route = metadata ? yyjson_obj_get(metadata, "exact_seed_backend_route") : NULL;
  if (!route || yyjson_is_null(route)) {
    yyjson_doc_free(doc);
    return 0;
  }
  if (!yyjson_is_obj(route)) {
    yyjson_doc_free(doc);
    return set_err_owned(err_out, "malformed exact_seed_backend_route metadata");
  }

  tag_val = yyjson_obj_get(route, "tag");
  source_route_val = yyjson_obj_get(route, "source_route");
  proof_val = yyjson_obj_get(route, "proof");
  selected_value_val = yyjson_obj_get(route, "selected_value");
  tag = tag_val && yyjson_is_str(tag_val) ? yyjson_get_str(tag_val) : NULL;
  source_route =
      source_route_val && yyjson_is_str(source_route_val) ? yyjson_get_str(source_route_val) : NULL;
  proof = proof_val && yyjson_is_str(proof_val) ? yyjson_get_str(proof_val) : NULL;
  if (selected_value_val && !yyjson_is_null(selected_value_val)) {
    if (!yyjson_is_num(selected_value_val)) {
      yyjson_doc_free(doc);
      return set_err_owned(err_out, "malformed exact_seed_backend_route selected_value");
    }
    selected_value = (long long)yyjson_get_sint(selected_value_val);
  }

  if (!tag || !source_route || !proof) {
    yyjson_doc_free(doc);
    return set_err_owned(err_out, "malformed exact_seed_backend_route fields");
  }
  if (!hako_llvmc_exact_seed_backend_route_supported(tag, source_route, proof, selected_value)) {
    yyjson_doc_free(doc);
    return set_err_owned(err_out, "unsupported exact_seed_backend_route metadata");
  }
  if (strlen(tag) + 1 > tag_out_len) {
    yyjson_doc_free(doc);
    return set_err_owned(err_out, "exact_seed_backend_route tag too long");
  }
  strcpy(tag_out, tag);
  if (selected_value_out) *selected_value_out = selected_value;
  yyjson_doc_free(doc);
  return 1;
}

static int compile_json_compat_pure(const char* json_in, const char* obj_out, char** err_out) {
  // Phase 21.2: validate v1 JSON, try generic pure lowering (CFG/phi),
  // then fall back to a few pattern lowers, and finally to AOT helper.
  char* verr = NULL;
  char unsupported_shape_first_op[64] = "";
  char unsupported_shape_reason[64] = "";
  char unsupported_shape_target_return_type[64] = "";
  char unsupported_shape_target_shape_reason[128] = "";
  char unsupported_shape_blocker_symbol[256] = "";
  char unsupported_shape_blocker_reason[128] = "";
  long long unsupported_shape_first_block = -1;
  long long unsupported_shape_first_inst = -1;

  auto const char* pure_unsupported_owner_hint(const char* op) {
    if (!op || !op[0]) return "unknown";
    if (strcmp(op, "boxcall") == 0 ||
        strcmp(op, "call") == 0 ||
        strcmp(op, "externcall") == 0 ||
        strcmp(op, "newbox") == 0 ||
        strcmp(op, "copy") == 0 ||
        strcmp(op, "phi") == 0) {
      return "mir_normalizer";
    }
    if (strcmp(op, "mir_call") == 0 ||
        strcmp(op, "module_function") == 0) {
      return "backend_lowering";
    }
    if (strcmp(op, "const") == 0 ||
        strcmp(op, "compare") == 0 ||
        strcmp(op, "select") == 0 ||
        strcmp(op, "branch") == 0 ||
        strcmp(op, "jump") == 0 ||
        strcmp(op, "ret") == 0 ||
        strcmp(op, "binop") == 0) {
      return "backend_lowering";
    }
    return "unknown";
  }

  auto void note_pure_unsupported_shape_detail(
      long long block_id,
      size_t inst_idx,
      const char* op,
      const char* reason,
      const char* target_return_type,
      const char* target_shape_reason,
      const char* target_shape_blocker_symbol,
      const char* target_shape_blocker_reason) {
    if (unsupported_shape_first_op[0]) return;
    snprintf(
        unsupported_shape_first_op,
        sizeof(unsupported_shape_first_op),
        "%s",
        (op && op[0]) ? op : "unknown");
    snprintf(
        unsupported_shape_reason,
        sizeof(unsupported_shape_reason),
        "%s",
        (reason && reason[0]) ? reason : "no_lowering_variant");
    snprintf(
        unsupported_shape_target_return_type,
        sizeof(unsupported_shape_target_return_type),
        "%s",
        (target_return_type && target_return_type[0]) ? target_return_type : "-");
    snprintf(
        unsupported_shape_target_shape_reason,
        sizeof(unsupported_shape_target_shape_reason),
        "%s",
        (target_shape_reason && target_shape_reason[0]) ? target_shape_reason : "-");
    snprintf(
        unsupported_shape_blocker_symbol,
        sizeof(unsupported_shape_blocker_symbol),
        "%s",
        (target_shape_blocker_symbol && target_shape_blocker_symbol[0])
            ? target_shape_blocker_symbol
            : "-");
    snprintf(
        unsupported_shape_blocker_reason,
        sizeof(unsupported_shape_blocker_reason),
        "%s",
        (target_shape_blocker_reason && target_shape_blocker_reason[0])
            ? target_shape_blocker_reason
            : "-");
    unsupported_shape_first_block = block_id;
    unsupported_shape_first_inst = (long long)inst_idx;
  }

  auto void note_pure_unsupported_shape(
      long long block_id,
      size_t inst_idx,
      const char* op,
      const char* reason,
      const char* target_shape_reason) {
    note_pure_unsupported_shape_detail(
        block_id,
        inst_idx,
        op,
        reason,
        NULL,
        target_shape_reason,
        NULL,
        NULL);
  }

  auto void emit_pure_unsupported_shape_inventory(void) {
    char block_buf[32];
    char inst_buf[32];
    const char* op = unsupported_shape_first_op[0] ? unsupported_shape_first_op : "unknown";
    const char* reason = unsupported_shape_reason[0] ? unsupported_shape_reason : "no_lowering_variant";
    if (!hako_llvmc_route_trace_enabled()) return;
    if (unsupported_shape_first_block >= 0) {
      snprintf(block_buf, sizeof(block_buf), "%lld", unsupported_shape_first_block);
    } else {
      snprintf(block_buf, sizeof(block_buf), "-");
    }
    if (unsupported_shape_first_inst >= 0) {
      snprintf(inst_buf, sizeof(inst_buf), "%lld", unsupported_shape_first_inst);
    } else {
      snprintf(inst_buf, sizeof(inst_buf), "-");
    }
    fprintf(
        stderr,
        "[llvm-pure/unsupported-shape] recipe=pure-first first_block=%s first_inst=%s first_op=%s owner_hint=%s reason=%s target_return_type=%s target_shape_reason=%s target_shape_blocker_symbol=%s target_shape_blocker_reason=%s\n",
        block_buf,
        inst_buf,
        op,
        pure_unsupported_owner_hint(op),
        reason,
        unsupported_shape_target_return_type[0]
            ? unsupported_shape_target_return_type
            : "-",
        unsupported_shape_target_shape_reason[0]
            ? unsupported_shape_target_shape_reason
            : "-",
        unsupported_shape_blocker_symbol[0]
            ? unsupported_shape_blocker_symbol
            : "-",
        unsupported_shape_blocker_reason[0]
            ? unsupported_shape_blocker_reason
            : "-");
  }

  if (hako_json_v1_validate_file(json_in, &verr) != 0) {
    return set_err_owned(err_out, verr ? verr : "invalid v1 json");
  }

  do {
    yyjson_read_err rerr_t; yyjson_doc* d = yyjson_read_file(json_in, 0, NULL, &rerr_t);
    if (!d) break;
    yyjson_val* root = yyjson_doc_get_root(d);
    yyjson_val* fns = yyjson_obj_get(root, "functions");
    yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
    hako_llvmc_log_selected_entry_runes(fn0);
    yyjson_doc_free(d);
  } while (0);

  {
    char exact_seed_tag[64];
    long long exact_seed_selected_value = 0;
    int exact_seed_route =
        hako_llvmc_read_exact_seed_backend_route_tag(
            json_in,
            exact_seed_tag,
            sizeof(exact_seed_tag),
            &exact_seed_selected_value,
            err_out);
    if (exact_seed_route < 0) {
      return -1;
    }
    if (exact_seed_route > 0) {
      if (hako_llvmc_route_trace_enabled()) {
        hako_llvmc_emit_route_trace(
            "exact_seed_backend_route",
            "hit",
            "mir_route_metadata",
            exact_seed_tag);
      }
      if (strcmp(exact_seed_tag, "array_string_store_micro") == 0) {
        if (hako_llvmc_consume_array_string_store_micro_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route array_string_store_micro failed");
      }
      if (strcmp(exact_seed_tag, "array_getset_micro") == 0) {
        if (hako_llvmc_consume_array_getset_micro_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route array_getset_micro failed");
      }
      if (strcmp(exact_seed_tag, "array_rmw_add1_leaf") == 0) {
        if (hako_llvmc_consume_array_rmw_add1_leaf_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route array_rmw_add1_leaf failed");
      }
      if (strcmp(exact_seed_tag, "sum_variant_tag_local") == 0) {
        if (hako_llvmc_consume_sum_variant_tag_seed_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route sum_variant_tag_local failed");
      }
      if (strcmp(exact_seed_tag, "sum_variant_project_local") == 0) {
        if (hako_llvmc_consume_sum_variant_project_seed_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route sum_variant_project_local failed");
      }
      if (strcmp(exact_seed_tag, "userbox_point_local_scalar") == 0) {
        if (hako_llvmc_consume_userbox_point_local_scalar_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route userbox_point_local_scalar failed");
      }
      if (strcmp(exact_seed_tag, "userbox_flag_pointf_local_scalar") == 0) {
        if (hako_llvmc_consume_userbox_flag_pointf_local_scalar_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route userbox_flag_pointf_local_scalar failed");
      }
      if (strcmp(exact_seed_tag, "userbox_loop_micro") == 0) {
        if (hako_llvmc_consume_userbox_loop_micro_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route userbox_loop_micro failed");
      }
      if (strcmp(exact_seed_tag, "userbox_known_receiver_method_seed") == 0) {
        if (hako_llvmc_consume_userbox_known_receiver_method_seed_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route userbox_known_receiver_method_seed failed");
      }
      if (strcmp(exact_seed_tag, "concat_const_suffix_micro") == 0) {
        if (hako_llvmc_consume_concat_const_suffix_micro_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route concat_const_suffix_micro failed");
      }
      if (strcmp(exact_seed_tag, "substring_views_only_micro") == 0) {
        if (hako_llvmc_consume_substring_views_only_micro_route(json_in, obj_out) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route substring_views_only_micro failed");
      }
      if (strcmp(exact_seed_tag, "substring_concat_loop_ascii") == 0) {
        if (hako_llvmc_consume_substring_concat_loop_ascii_route(
                json_in,
                obj_out,
                exact_seed_selected_value) == 0) {
          return 0;
        }
        return set_err_owned(err_out, "exact_seed_backend_route substring_concat_loop_ascii failed");
      }
      return set_err_owned(err_out, "unsupported exact_seed_backend_route tag");
    }
  }

  if (hako_llvmc_consume_substring_views_only_micro_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_concat_const_suffix_micro_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_array_string_store_micro_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_array_rmw_add1_leaf_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_sum_variant_tag_seed_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_sum_variant_project_seed_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_userbox_point_local_scalar_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_userbox_flag_pointf_local_scalar_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_userbox_loop_micro_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_userbox_known_receiver_method_seed_route(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_consume_array_getset_micro_route(json_in, obj_out) == 0) {
    return 0;
  }
    // --- Generic CFG/PHI lowering (minimal i64 subset) ---
    // Supported ops: const/compare/branch/jump/ret/phi, mir_call (Array/Map minimal, Global print)
    do {
      yyjson_read_err rerr_g; yyjson_doc* d = yyjson_read_file(json_in, 0, NULL, &rerr_g);
      if (!d) break;
      yyjson_val* root = yyjson_doc_get_root(d);
      hako_llvmc_generic_pure_program_view program;
      if (!hako_llvmc_read_generic_pure_program_view(root, &program)) { yyjson_doc_free(d); break; }
      hako_llvmc_generic_pure_function_view* active_function = &program.entry;
      yyjson_val* fn0 = active_function->fn;
      yyjson_val* fn_metadata = active_function->metadata;
      yyjson_val* blocks = active_function->blocks;
      size_t generic_pure_block_count = active_function->block_count;
      hako_llvmc_entry_rune_selection selection = active_function->selection;
      if (hako_llvmc_match_indexof_line_text_state_residence_fn(fn0, obj_out) == 0 ||
          hako_llvmc_match_indexof_leaf_text_state_residence_fn(fn0, obj_out) == 0) {
        yyjson_doc_free(d);
        return 0;
      }
      enum { T_NONE=0, T_I64=1, T_I1=2 };
      // Track simple origin kinds for selective unbox at ret
      enum { ORG_NONE=0, ORG_MAP_GET=1, ORG_MAP_BIRTH=2, ORG_ARRAY_BIRTH=3, ORG_STRING=4, ORG_ARRAY_STRING_BIRTH=5 };
      struct GenericPureI64Const { long long reg; long long val; };
      struct GenericPureStringConst { long long reg; const char* val; };
      struct GenericPureKnownStringLen { long long reg; long long len; };
      struct GenericPureRegMarker { long long reg; };
      struct GenericPureTypeBinding { long long reg; int ty; };
      struct GenericPureCopyAlias { long long reg; long long src; };
      struct GenericPureOriginBinding { long long reg; int kind; };
      struct Incoming { long long pred; long long val_reg; };
      struct PhiRec { long long dst; struct Incoming in[16]; int in_n; };
      struct BlockPhi { long long bid; struct PhiRec recs[16]; int rec_n; };
      struct GenericPureFunctionLoweringState {
        struct GenericPureI64Const consts[1024];
        size_t consts_n;
        struct GenericPureStringConst str_consts[512];
        size_t str_consts_n;
        struct GenericPureKnownStringLen known_string_lens[2048];
        size_t known_string_lens_n;
        char* owned_str_consts[512];
        size_t owned_str_consts_n;
        struct GenericPureRegMarker str_const_box_skip[512];
        size_t str_const_box_skip_n;
        struct GenericPureRegMarker str_const_box_emitted[512];
        size_t str_const_box_emitted_n;
        struct GenericPureTypeBinding types[2048];
        size_t types_n;
        struct GenericPureCopyAlias copy_alias[2048];
        size_t copy_alias_n;
        struct GenericPureOriginBinding scan_origin[2048];
        size_t scan_origin_n;
        struct GenericPureOriginBinding origin[2048];
        size_t origin_n;
        struct BlockPhi phis[512];
        int phi_n;
        int generic_walk_trace_budget;
        int generic_call_trace_budget;
        int generic_method_trace_budget;
      };
      auto void reset_generic_pure_function_lowering_state(
          struct GenericPureFunctionLoweringState* state) {
        if (!state) return;
        memset(state, 0, sizeof(*state));
        state->generic_walk_trace_budget = 24;
        state->generic_call_trace_budget = 24;
        state->generic_method_trace_budget = 24;
      }
      struct GenericPureFunctionLoweringState lowering_state;
      reset_generic_pure_function_lowering_state(&lowering_state);
      #define consts lowering_state.consts
      #define consts_n lowering_state.consts_n
      #define str_consts lowering_state.str_consts
      #define str_consts_n lowering_state.str_consts_n
      #define known_string_lens lowering_state.known_string_lens
      #define known_string_lens_n lowering_state.known_string_lens_n
      #define owned_str_consts lowering_state.owned_str_consts
      #define owned_str_consts_n lowering_state.owned_str_consts_n
      #define str_const_box_skip lowering_state.str_const_box_skip
      #define str_const_box_skip_n lowering_state.str_const_box_skip_n
      #define str_const_box_emitted lowering_state.str_const_box_emitted
      #define str_const_box_emitted_n lowering_state.str_const_box_emitted_n
      #define types lowering_state.types
      #define types_n lowering_state.types_n
      #define copy_alias lowering_state.copy_alias
      #define copy_alias_n lowering_state.copy_alias_n
      #define scan_origin lowering_state.scan_origin
      #define scan_origin_n lowering_state.scan_origin_n
      #define origin lowering_state.origin
      #define origin_n lowering_state.origin_n
      #define phis lowering_state.phis
      #define phi_n lowering_state.phi_n
      #define generic_walk_trace_budget lowering_state.generic_walk_trace_budget
      #define generic_call_trace_budget lowering_state.generic_call_trace_budget
      #define generic_method_trace_budget lowering_state.generic_method_trace_budget

      // small helpers
      #define ARR_LEN(a) ((int)(sizeof(a)/sizeof((a)[0])))
      auto long long read_int(yyjson_val* obj, const char* key);
      auto const char* read_str(yyjson_val* obj, const char* key);
      /* inc-codegen-view-owner: generic_pure_view begin */
      struct GenericPureBlockView {
        yyjson_val* block;
        yyjson_val* instructions;
        size_t instruction_count;
        long long id;
      };
      auto int read_generic_pure_block_view(size_t bi, struct GenericPureBlockView* out) {
        yyjson_val* b = bi < generic_pure_block_count ? yyjson_arr_get(blocks, bi) : NULL;
        yyjson_val* insts = b ? yyjson_obj_get(b, "instructions") : NULL;
        if (!out) return 0;
        memset(out, 0, sizeof(*out));
        if (!insts || !yyjson_is_arr(insts)) return 0;
        out->block = b;
        out->instructions = insts;
        out->instruction_count = yyjson_arr_size(insts);
        out->id = read_int(b, "id");
        return 1;
      }
      auto int read_generic_pure_function_block_view(
          const hako_llvmc_generic_pure_function_view* fn_view,
          size_t bi,
          struct GenericPureBlockView* out) {
        yyjson_val* saved_blocks = blocks;
        size_t saved_generic_pure_block_count = generic_pure_block_count;
        int ok = 0;
        if (!fn_view) return 0;
        blocks = fn_view->blocks;
        generic_pure_block_count = fn_view->block_count;
        ok = read_generic_pure_block_view(bi, out);
        blocks = saved_blocks;
        generic_pure_block_count = saved_generic_pure_block_count;
        return ok;
      }
      /* inc-codegen-view-owner: generic_pure_view end */
      #include "hako_llvmc_ffi_compiler_state.inc"
      auto struct BlockPhi* ensure_phi_block(long long bid){ for (int i=0;i<phi_n;i++){ if (phis[i].bid==bid) return &phis[i]; } if (phi_n<512){ phis[phi_n].bid=bid; phis[phi_n].rec_n=0; return &phis[phi_n++]; } return NULL; }
      auto long long read_int(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? (long long)yyjson_get_sint(v) : 0; }
      auto const char* read_str(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? yyjson_get_str(v) : NULL; }
      auto int llvm_quoted_symbol_name_is_safe(const char* name) {
        const unsigned char* p = (const unsigned char*)name;
        if (!name || !name[0]) return 0;
        while (*p) {
          if (*p < 0x20 || *p == '"' || *p == '\\') return 0;
          p++;
        }
        return 1;
      }
      auto size_t generic_pure_function_param_count(yyjson_val* fn) {
        yyjson_val* params = fn ? yyjson_obj_get(fn, "params") : NULL;
        return (params && yyjson_is_arr(params)) ? yyjson_arr_size(params) : 0;
      }
      auto char* concat_owned_string_consts(const char* lhs, const char* rhs) {
        size_t lhs_len = lhs ? strlen(lhs) : 0;
        size_t rhs_len = rhs ? strlen(rhs) : 0;
        char* out = (char*)malloc(lhs_len + rhs_len + 1);
        if (!out) return NULL;
        if (lhs_len) memcpy(out, lhs, lhs_len);
        if (rhs_len) memcpy(out + lhs_len, rhs, rhs_len);
        out[lhs_len + rhs_len] = '\0';
        return out;
      }
      auto int put_str_const_owned(long long r, char* v) {
        if (!v) return 0;
        if (str_consts_n >= ARR_LEN(str_consts) || owned_str_consts_n >= ARR_LEN(owned_str_consts)) {
          free(v);
          return 0;
        }
        put_str_const(r, v);
        owned_str_consts[owned_str_consts_n++] = v;
        return 1;
      }
      auto void free_owned_str_consts(void) {
        for (size_t i = 0; i < owned_str_consts_n; i++) {
          free(owned_str_consts[i]);
          owned_str_consts[i] = NULL;
        }
        owned_str_consts_n = 0;
      }
      #include "hako_llvmc_ffi_indexof_observer_state.inc"
      #include "hako_llvmc_ffi_array_string_window_policy.inc"
      #include "hako_llvmc_ffi_array_slot_emit.inc"
      #include "hako_llvmc_ffi_indexof_observer_lowering.inc"
      #include "hako_llvmc_ffi_observer_store_region_metadata.inc"
      #include "hako_llvmc_ffi_array_text_edit_metadata.inc"
      #include "hako_llvmc_ffi_array_text_combined_region_metadata.inc"
      #include "hako_llvmc_ffi_string_chain_terms.inc"
      #include "hako_llvmc_ffi_string_candidate_plan_readers.inc"
      #include "hako_llvmc_ffi_mir_call_surface_policy.inc"
      #include "hako_llvmc_ffi_string_concat_lowering.inc"
      #include "hako_llvmc_ffi_string_chain_policy.inc"
      #include "hako_llvmc_ffi_const_string_hoist.inc"
      #include "hako_llvmc_ffi_core_method_metadata.inc"
      #include "hako_llvmc_ffi_lowering_plan_metadata.inc"
      #include "hako_llvmc_ffi_module_leaf_function_emit.inc"
      #include "hako_llvmc_ffi_module_generic_string_plan.inc"
      auto void emit_i64_function_declaration(FILE* f, const char* name, size_t argc) {
        if (!llvm_quoted_symbol_name_is_safe(name)) return;
        fprintf(f, "declare i64 @\"%s\"(", name);
        for (size_t i = 0; i < argc; i++) {
          fprintf(f, "%si64", i ? ", " : "");
        }
        fprintf(f, ")\n");
      }
      auto void emit_module_function_declarations(FILE* f) {
        for (size_t i = 0; i < program.function_count; i++) {
          yyjson_val* fn = yyjson_arr_get(program.functions, i);
          const char* name = fn ? read_str(fn, "name") : NULL;
          if (!name || fn == fn0) continue;
          if (module_function_is_numeric_i64_leaf(fn)) continue;
          if (module_generic_symbol_will_be_defined(name)) continue;
          emit_i64_function_declaration(f, name, generic_pure_function_param_count(fn));
        }
      }
      #include "hako_llvmc_ffi_mir_call_route_policy.inc"
      #include "hako_llvmc_ffi_mir_call_need_policy.inc"
      #include "hako_llvmc_ffi_mir_call_prepass.inc"
      #include "hako_llvmc_ffi_mir_call_shell.inc"
      #define HAKO_LLVM_READONLY_ATTRS " nounwind readonly willreturn"
      auto int should_skip_string_const_box_for_known_len_use(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg);

      #include "hako_llvmc_ffi_pure_compile_generic_lowering.inc"
      #undef consts
      #undef consts_n
      #undef str_consts
      #undef str_consts_n
      #undef known_string_lens
      #undef known_string_lens_n
      #undef owned_str_consts
      #undef owned_str_consts_n
      #undef str_const_box_skip
      #undef str_const_box_skip_n
      #undef str_const_box_emitted
      #undef str_const_box_emitted_n
      #undef types
      #undef types_n
      #undef copy_alias
      #undef copy_alias_n
      #undef scan_origin
      #undef scan_origin_n
      #undef origin
      #undef origin_n
      #undef phis
      #undef phi_n
      #undef generic_walk_trace_budget
      #undef generic_call_trace_budget
      #undef generic_method_trace_budget
  if (compat_harness_replay_enabled()) {
    return compile_json_via_explicit_compat_harness_replay(json_in, obj_out, err_out);
  }
  hako_llvmc_emit_route_replay("none", "unsupported_pure_shape");
  emit_pure_unsupported_shape_inventory();
  return set_err_owned(err_out, "unsupported pure shape for current backend recipe");
}

__attribute__((visibility("default")))
int hako_llvmc_link_obj(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  return forward_link_obj_to_aot(obj_in, exe_out, extra_ldflags, err_out);
}
