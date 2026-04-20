      auto void append_i64_arg_ref(char* out, size_t out_cap, long long reg, int first) {
        long long cv = 0;
        long long base = resolve_copy_source_reg(reg);
        char tmp[64];
        size_t used = strlen(out);
        if (used >= out_cap) return;
        if (has_const(base, &cv)) snprintf(tmp, sizeof(tmp), "i64 %lld", cv);
        else snprintf(tmp, sizeof(tmp), "i64 %%r%lld", base);
        snprintf(out + used, out_cap - used, "%s%s", first ? "" : ", ", tmp);
      }
      auto void append_array_string_indexof_method_receiver_args(
          struct DeferredArrayStringIndexofSelect* deferred_array_string_indexof,
          long long recv,
          char* out,
          size_t out_cap) {
        if (deferred_array_string_indexof) {
          append_i64_arg_ref(out, out_cap, deferred_array_string_indexof->recv, 1);
          append_i64_arg_ref(out, out_cap, deferred_array_string_indexof->idx, 0);
          append_i64_arg_ref(out, out_cap, deferred_array_string_indexof->needle_reg, 0);
        } else if (recv) {
          append_i64_arg_ref(out, out_cap, recv, 1);
        }
      }
      struct ArrayTextObserverRoutePlan {
        int matched;
        long long observer_bid;
        size_t observer_ii;
        long long observer_arg0;
        int keep_get_live;
      };
      auto int hako_llvmc_read_bool_key(yyjson_val* obj, const char* key) {
        yyjson_val* value = obj ? yyjson_obj_get(obj, key) : NULL;
        return value && yyjson_is_bool(value) ? yyjson_get_bool(value) : 0;
      }
      auto int match_array_text_observer_route_plan(
          long long get_bid,
          size_t get_ii,
          struct ArrayTextObserverRoutePlan* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_text_observer_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* get_block_val = route ? yyjson_obj_get(route, "get_block") : NULL;
          yyjson_val* get_index_val = route ? yyjson_obj_get(route, "get_instruction_index") : NULL;
          yyjson_val* block_val = route ? yyjson_obj_get(route, "block") : NULL;
          yyjson_val* observer_index_val = route ? yyjson_obj_get(route, "observer_instruction_index") : NULL;
          yyjson_val* observer_arg0_val = route ? yyjson_obj_get(route, "observer_arg0") : NULL;
          const char* observer_kind = route ? read_str(route, "observer_kind") : NULL;
          const char* proof_region = route ? read_str(route, "proof_region") : NULL;
          const char* publication_boundary = route ? read_str(route, "publication_boundary") : NULL;
          const char* result_repr = route ? read_str(route, "result_repr") : NULL;
          if (!(get_block_val && get_index_val &&
                yyjson_is_num(get_block_val) && yyjson_is_num(get_index_val))) {
            continue;
          }
          if ((long long)yyjson_get_sint(get_block_val) != get_bid ||
              (size_t)yyjson_get_sint(get_index_val) != get_ii) {
            continue;
          }
          if (!(block_val && observer_index_val && observer_arg0_val &&
                yyjson_is_num(block_val) &&
                yyjson_is_num(observer_index_val) &&
                yyjson_is_num(observer_arg0_val) &&
                observer_kind && !strcmp(observer_kind, "indexof") &&
                proof_region && !strcmp(proof_region, "array_get_receiver_indexof") &&
                publication_boundary && !strcmp(publication_boundary, "none") &&
                result_repr && !strcmp(result_repr, "scalar_i64"))) {
            return 0;
          }
          if (out) {
            out->matched = 1;
            out->observer_bid = (long long)yyjson_get_sint(block_val);
            out->observer_ii = (size_t)yyjson_get_sint(observer_index_val);
            out->observer_arg0 = (long long)yyjson_get_sint(observer_arg0_val);
            out->keep_get_live = hako_llvmc_read_bool_key(route, "keep_get_live");
          }
          return 1;
        }
        return 0;
      }
      auto int try_defer_array_string_indexof_get_window(
          FILE* f,
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long dst,
          int recv_org,
          int runtime_array_string,
          unsigned char* skip_inst) {
        struct ArrayTextObserverRoutePlan observer_route;
        char keep_ab[128];
        (void)blocks;
        (void)ilen;
        (void)insts;
        (void)bname;
        (void)mname;
        (void)recv_org;
        (void)runtime_array_string;
        (void)skip_inst;
        if (match_array_text_observer_route_plan(bid, ii, &observer_route)) {
          if (dst) {
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
            remember_array_string_get_source(dst, recv, idx);
          }
          if (dst && observer_route.keep_get_live) {
            keep_ab[0] = '\0';
            append_i64_arg_ref(keep_ab, sizeof(keep_ab), recv, 1);
            append_i64_arg_ref(keep_ab, sizeof(keep_ab), idx, 0);
            hako_llvmc_emit_array_slot_load(
                f, dst, keep_ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF);
          }
          remember_deferred_array_string_indexof_select(
              observer_route.observer_bid,
              observer_route.observer_ii,
              recv,
              idx,
              observer_route.observer_arg0);
          return 1;
        }
        return 0;
      }
      auto int emit_deferred_array_string_indexof_call(
          FILE* f,
          struct DeferredArrayStringIndexofSelect* deferred_array_string_indexof,
          long long dst,
          const char* args) {
        if (!deferred_array_string_indexof) return 0;
        hako_llvmc_emit_array_string_indexof(f, dst, args);
        return 1;
      }
