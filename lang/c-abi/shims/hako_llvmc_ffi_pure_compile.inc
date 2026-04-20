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

  if (hako_llvmc_match_substring_concat_loop_ascii_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_substring_concat_len_ascii_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_substring_views_only_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_concat_const_suffix_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_array_string_store_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_array_rmw_add1_leaf_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_array_getset_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_point_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_point_copy_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_flag_local_bool_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_flag_copy_local_bool_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_pointf_local_f64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_pointf_copy_local_f64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_counter_step_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_point_sum_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_counter_step_copy_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_counter_step_chain_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_point_sum_copy_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_counter_step_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_point_sum_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_point_add_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_userbox_flag_toggle_micro_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_tag_local_tag_only_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_tag_local_f64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_tag_local_handle_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_tag_copy_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_project_copy_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_project_copy_local_f64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_project_copy_local_handle_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_project_local_f64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_project_local_handle_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_tag_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_variant_project_local_i64_seed(json_in, obj_out) == 0) {
    return 0;
  }
    // --- Generic CFG/PHI lowering (minimal i64 subset) ---
    // Supported ops: const/compare/branch/jump/ret/phi, mir_call (Array/Map minimal, Global print)
    do {
      yyjson_read_err rerr_g; yyjson_doc* d = yyjson_read_file(json_in, 0, NULL, &rerr_g);
      if (!d) break;
      yyjson_val* root = yyjson_doc_get_root(d);
      yyjson_val* fns = yyjson_obj_get(root, "functions");
      yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
      hako_llvmc_entry_rune_selection selection;
      yyjson_val* fn_metadata =
          fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "metadata") : NULL;
      yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
      if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) >= 1)) { yyjson_doc_free(d); break; }
      hako_llvmc_select_entry_runes(fn0, &selection);
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
      int array_rmw_trace_budget = 8;
      int array_string_len_trace_budget = 8;
      int generic_walk_trace_budget = 24;
      int generic_call_trace_budget = 24;
      int generic_method_trace_budget = 24;
      #include "hako_llvmc_ffi_indexof_observer_state.inc"
      #include "hako_llvmc_ffi_array_string_window_policy.inc"
      #include "hako_llvmc_ffi_array_slot_emit.inc"
      #include "hako_llvmc_ffi_indexof_observer_lowering.inc"
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
      #include "hako_llvmc_ffi_pure_compile_minimal_paths.inc"
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
