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

typedef struct hako_llvmc_generic_pure_program_view {
  yyjson_val* fn;
  yyjson_val* metadata;
  yyjson_val* blocks;
  size_t block_count;
  hako_llvmc_entry_rune_selection selection;
} hako_llvmc_generic_pure_program_view;

static int hako_llvmc_read_generic_pure_program_view(
    yyjson_val* root,
    hako_llvmc_generic_pure_program_view* out) {
  yyjson_val* fns;
  if (!out) return 0;
  memset(out, 0, sizeof(*out));
  fns = yyjson_obj_get(root, "functions");
  out->fn = hako_llvmc_find_entry_function(fns);
  if (!(out->fn && yyjson_is_obj(out->fn))) return 0;
  out->metadata = yyjson_obj_get(out->fn, "metadata");
  out->blocks = yyjson_obj_get(out->fn, "blocks");
  if (!(out->blocks && yyjson_is_arr(out->blocks))) return 0;
  out->block_count = yyjson_arr_size(out->blocks);
  if (out->block_count < 1) return 0;
  hako_llvmc_select_entry_runes(out->fn, &out->selection);
  return 1;
}

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
      yyjson_val* fn0 = program.fn;
      yyjson_val* fn_metadata = program.metadata;
      yyjson_val* blocks = program.blocks;
      size_t generic_pure_block_count = program.block_count;
      hako_llvmc_entry_rune_selection selection = program.selection;
      if (hako_llvmc_match_indexof_line_text_state_residence_fn(fn0, obj_out) == 0 ||
          hako_llvmc_match_indexof_leaf_text_state_residence_fn(fn0, obj_out) == 0) {
        yyjson_doc_free(d);
        return 0;
      }
      enum { T_NONE=0, T_I64=1, T_I1=2 };
      struct { long long reg; long long val; } consts[1024]; size_t consts_n = 0;
      struct { long long reg; const char* val; } str_consts[512]; size_t str_consts_n = 0;
      struct { long long reg; long long len; } known_string_lens[2048]; size_t known_string_lens_n = 0;
      char* owned_str_consts[512]; size_t owned_str_consts_n = 0;
      struct { long long reg; } str_const_box_skip[512]; size_t str_const_box_skip_n = 0;
      struct { long long reg; } str_const_box_emitted[512]; size_t str_const_box_emitted_n = 0;
      struct { long long reg; int ty; } types[2048]; size_t types_n = 0;
      struct { long long reg; long long src; } copy_alias[2048]; size_t copy_alias_n = 0;
      struct { long long reg; int kind; } scan_origin[2048]; size_t scan_origin_n = 0;
      // Track simple origin kinds for selective unbox at ret
      enum { ORG_NONE=0, ORG_MAP_GET=1, ORG_MAP_BIRTH=2, ORG_ARRAY_BIRTH=3, ORG_STRING=4, ORG_ARRAY_STRING_BIRTH=5 };
      struct { long long reg; int kind; } origin[2048]; size_t origin_n = 0;
      struct Incoming { long long pred; long long val_reg; };
      struct PhiRec { long long dst; struct Incoming in[16]; int in_n; };
      struct BlockPhi { long long bid; struct PhiRec recs[16]; int rec_n; } phis[512]; int phi_n = 0;

      // small helpers
      #define ARR_LEN(a) ((int)(sizeof(a)/sizeof((a)[0])))
      auto long long read_int(yyjson_val* obj, const char* key);
      auto const char* read_str(yyjson_val* obj, const char* key);
      #include "hako_llvmc_ffi_compiler_state.inc"
      auto struct BlockPhi* ensure_phi_block(long long bid){ for (int i=0;i<phi_n;i++){ if (phis[i].bid==bid) return &phis[i]; } if (phi_n<512){ phis[phi_n].bid=bid; phis[phi_n].rec_n=0; return &phis[phi_n++]; } return NULL; }
      auto long long read_int(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? (long long)yyjson_get_sint(v) : 0; }
      auto const char* read_str(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? yyjson_get_str(v) : NULL; }
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
      int generic_walk_trace_budget = 24;
      int generic_call_trace_budget = 24;
      int generic_method_trace_budget = 24;
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
  if (compat_harness_replay_enabled()) {
    return compile_json_via_explicit_compat_harness_replay(json_in, obj_out, err_out);
  }
  hako_llvmc_emit_route_replay("none", "unsupported_pure_shape");
  return set_err_owned(err_out, "unsupported pure shape for current backend recipe");
}

__attribute__((visibility("default")))
int hako_llvmc_link_obj(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  return forward_link_obj_to_aot(obj_in, exe_out, extra_ldflags, err_out);
}
