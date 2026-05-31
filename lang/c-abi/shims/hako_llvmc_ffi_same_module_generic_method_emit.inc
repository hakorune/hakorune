      /*
       * Same-module generic method body emit.
       *
       * This file emits direct generic method bodies from MIR-owned route
       * facts only. It must not grow key allowlists or semantic planning.
       */
      auto int same_module_function_emit_method_len_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv) {
        char ab[256];
        int is_string_len = 0;
        int is_array_len = 0;
        const char* helper_symbol = NULL;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (method_view) {
          is_string_len = same_module_method_view_is_direct_string_len(method_view);
          is_array_len = same_module_method_view_is_direct_array_len(method_view);
        }
        if (!(is_string_len || is_array_len)) {
          return 0;
        }
        helper_symbol = is_array_len ? "nyash.array.slot_len_h" : "nyash.string.len_fast_h";
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        same_module_function_emit_i64_call(dst, helper_symbol, ab);
        if (dst) {
          if (is_string_len) remember_string_length_call(dst, recv);
        }
        return 1;
      }

      auto int same_module_function_emit_method_substring_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        char recv_ab[128];
        long long start = 0;
        long long end = 0;
        long long recorded_end = 0;
        size_t arg_count = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        (void)f;
        ab[0] = '\0';
        recv_ab[0] = '\0';
        if (!(method_view &&
              same_module_method_view_is_direct_string_substring(method_view))) {
          return 0;
        }
        if (!(arg_count == 1 || arg_count == 2)) return 0;
        if (!same_module_function_read_arg_reg(args, 0, &start)) return 0;
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), start, 0);
        if (arg_count == 2) {
          if (!same_module_function_read_arg_reg(args, 1, &end)) return 0;
          append_i64_arg_ref(ab, sizeof(ab), end, 0);
          recorded_end = end;
        } else {
          size_t used = strlen(ab);
          append_i64_arg_ref(recv_ab, sizeof(recv_ab), recv, 1);
          EMIT("  %%substr_end_%lld_%zu = call i64 @\"nyash.string.len_fast_h\"(%s)\n", bid, ii, recv_ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(
              ab + used,
              sizeof(ab) - used,
              ", i64 %%substr_end_%lld_%zu",
              bid,
              ii);
        }
        same_module_function_emit_i64_call(dst, "nyash.string.substring_hii", ab);
        if (dst) {
          same_module_function_publish_origin(dst, ORG_STRING);
          remember_string_substring_call(dst, recv, start, recorded_end);
        }
        return 1;
      }

      auto int same_module_function_emit_method_push_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        long long value = 0;
        (void)f;
        (void)bid;
        (void)ii;
        if (!(method_view &&
              same_module_method_view_is_direct_array_push(method_view))) {
          return 0;
        }
        if (!same_module_function_read_array_push_value(args, recv, &value)) return 0;
        same_module_function_emit_array_slot_append_call(dst, recv, value);
        same_module_function_promote_array_string_push(recv, value);
        return 1;
      }

      auto int same_module_function_direct_array_i64_exact_get_enabled(void) {
        const char* value = getenv("HAKO_ARRAY_SLOT_STORE");
        return value && !strcmp(value, "direct_array_i64_exact");
      }

      auto int same_module_function_is_selected_direct_array_i64_get_target(void) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        return name &&
               (!strcmp(name, "HakoAllocPageModel.acquire/1") ||
                !strcmp(name, "HakoAllocPageModel.acquireFreshSmall/1") ||
                !strcmp(name, "HakoAllocPageModel.acquire_usize/1") ||
                !strcmp(name, "HakoAllocPageModel.reactivate/0") ||
                !strcmp(name, "HakoAllocPageModel.releaseLocal/1") ||
                !strcmp(name, "HakoAllocPageModel.releaseLocalKnownLive/1"));
      }

      auto int same_module_function_method_view_is_direct_array_i64_get(
          const struct LoweringPlanGenericMethodView* method_view) {
        return method_view &&
               method_view->core_op &&
               !strcmp(method_view->core_op, "ArrayGet") &&
               method_view->route_kind &&
               !strcmp(method_view->route_kind, "array_slot_load_any") &&
               method_view->symbol &&
               !strcmp(method_view->symbol, "nyash.array.slot_load_hi") &&
               method_view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int same_module_function_emit_direct_array_i64_get_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          long long key) {
        char recv_ref[64];
        char key_ref[64];
        (void)f;
        if (!dst) return 0;
        if (!same_module_function_direct_array_i64_exact_get_enabled()) return 0;
        if (!same_module_function_is_selected_direct_array_i64_get_target()) return 0;
        if (!same_module_function_method_view_is_direct_array_i64_get(method_view)) return 0;

        same_module_function_format_i64_ref(recv_ref, sizeof(recv_ref), recv);
        same_module_function_format_i64_ref(key_ref, sizeof(key_ref), key);

        EMIT("  %%direct_array_i64_get_base_%lld_%zu = and i64 %s, -4\n",
             bid,
             ii,
             recv_ref);
        EMIT("  %%direct_array_i64_get_len_addr_%lld_%zu = add i64 %%direct_array_i64_get_base_%lld_%zu, 16\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_get_len_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_get_len_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_get_len_%lld_%zu = load i64, ptr %%direct_array_i64_get_len_ptr_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_get_nonneg_%lld_%zu = icmp sge i64 %s, 0\n",
             bid,
             ii,
             key_ref);
        EMIT("  %%direct_array_i64_get_in_len_%lld_%zu = icmp ult i64 %s, %%direct_array_i64_get_len_%lld_%zu\n",
             bid,
             ii,
             key_ref,
             bid,
             ii);
        EMIT("  %%direct_array_i64_get_can_load_%lld_%zu = and i1 %%direct_array_i64_get_nonneg_%lld_%zu, %%direct_array_i64_get_in_len_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  br i1 %%direct_array_i64_get_can_load_%lld_%zu, label %%direct_array_i64_get_load_%lld_%zu, label %%direct_array_i64_get_oob_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);

        EMIT("direct_array_i64_get_load_%lld_%zu:\n", bid, ii);
        EMIT("  %%direct_array_i64_get_byte_index_%lld_%zu = mul i64 %s, 8\n",
             bid,
             ii,
             key_ref);
        EMIT("  %%direct_array_i64_get_data_base_%lld_%zu = add i64 %%direct_array_i64_get_base_%lld_%zu, 32\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_get_value_addr_%lld_%zu = add i64 %%direct_array_i64_get_data_base_%lld_%zu, %%direct_array_i64_get_byte_index_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_get_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_get_value_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_get_value_%lld_%zu = load i64, ptr %%direct_array_i64_get_value_ptr_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  br label %%direct_array_i64_get_done_%lld_%zu\n", bid, ii);

        EMIT("direct_array_i64_get_oob_%lld_%zu:\n", bid, ii);
        EMIT("  br label %%direct_array_i64_get_done_%lld_%zu\n", bid, ii);

        EMIT("direct_array_i64_get_done_%lld_%zu:\n", bid, ii);
        EMIT("  %%r%lld = phi i64 [ %%direct_array_i64_get_value_%lld_%zu, %%direct_array_i64_get_load_%lld_%zu ], [ 0, %%direct_array_i64_get_oob_%lld_%zu ]\n",
             dst,
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        set_type(dst, T_I64);
        return 1;
      }

      auto int same_module_function_emit_method_get_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        long long key = 0;
        struct SameModuleFunctionGetRouteView route;
        const char* helper_symbol = NULL;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (!same_module_function_read_get_route_view(method_view, &route)) return 0;
        helper_symbol = method_view->symbol
                            ? method_view->symbol
                            : (route.is_array_load ? "nyash.array.slot_load_hi"
                                                   : "nyash.runtime_data.get_hh");
        if (!same_module_function_read_get_key_value(args, recv, &key)) return 0;
        if (route.is_array_load &&
            same_module_function_emit_direct_array_i64_get_mir_call(
                f, bid, ii, method_view, dst, recv, key)) {
          return 1;
        }
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), key, 0);
        same_module_function_emit_i64_call(dst, helper_symbol, ab);
        if (dst) {
          if (route.is_module_function_array_item_get) {
            same_module_function_publish_origin(dst, ORG_MAP_BIRTH);
          }
          if (route.is_phi_incoming_array_item_get) {
            same_module_function_publish_origin(dst, ORG_ARRAY_BIRTH);
          }
          if (route.publishes_map_get_origin) {
            set_origin(dst, ORG_MAP_GET);
          }
          if (!same_module_function_publish_result_origin_box(
                  dst, route.result_origin_box)) {
            return 0;
          }
        }
        return 1;
      }

      auto int same_module_function_direct_array_i64_exact_enabled(void) {
        const char* value = getenv("HAKO_ARRAY_SLOT_STORE");
        return value && !strcmp(value, "direct_array_i64_exact");
      }

      auto int same_module_function_is_selected_direct_array_i64_store_target(void) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        return name &&
               (!strcmp(name, "HakoAllocPageModel.acquire/1") ||
                !strcmp(name, "HakoAllocPageModel.acquireFreshSmall/1") ||
                !strcmp(name, "HakoAllocPageModel.acquire_usize/1") ||
                !strcmp(name, "HakoAllocPageModel.reactivate/0") ||
                !strcmp(name, "HakoAllocPageModel.releaseLocal/1") ||
                !strcmp(name, "HakoAllocPageModel.releaseLocalKnownLive/1"));
      }

      auto int same_module_function_is_selected_direct_array_i64_unchecked_store_target(void) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        return name &&
               (!strcmp(name, "HakoAllocPageModel.resetToFresh/0") ||
                !strcmp(name, "HakoAllocPageModel.releaseLocalKnownLive/1"));
      }

      auto int same_module_function_emit_direct_array_i64_store_unchecked_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          long long key,
          long long value) {
        char recv_ref[64];
        char key_ref[64];
        char value_ref[64];
        (void)f;
        if (!same_module_function_direct_array_i64_exact_enabled()) return 0;
        if (!same_module_function_is_selected_direct_array_i64_unchecked_store_target()) return 0;
        if (!(value && is_plain_i64_value(value))) return 0;

        same_module_function_format_i64_ref(recv_ref, sizeof(recv_ref), recv);
        same_module_function_format_i64_ref(key_ref, sizeof(key_ref), key);
        same_module_function_format_i64_ref(value_ref, sizeof(value_ref), value);

        EMIT("  %%direct_array_i64_unchecked_base_%lld_%zu = and i64 %s, -4\n",
             bid,
             ii,
             recv_ref);
        EMIT("  %%direct_array_i64_unchecked_len_addr_%lld_%zu = add i64 %%direct_array_i64_unchecked_base_%lld_%zu, 16\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_unchecked_len_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_unchecked_len_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_unchecked_len_%lld_%zu = load i64, ptr %%direct_array_i64_unchecked_len_ptr_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_unchecked_byte_index_%lld_%zu = mul i64 %s, 8\n",
             bid,
             ii,
             key_ref);
        EMIT("  %%direct_array_i64_unchecked_data_base_%lld_%zu = add i64 %%direct_array_i64_unchecked_base_%lld_%zu, 32\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_unchecked_value_addr_%lld_%zu = add i64 %%direct_array_i64_unchecked_data_base_%lld_%zu, %%direct_array_i64_unchecked_byte_index_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_unchecked_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_unchecked_value_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  store i64 %s, ptr %%direct_array_i64_unchecked_value_ptr_%lld_%zu\n",
             value_ref,
             bid,
             ii);
        EMIT("  %%direct_array_i64_unchecked_is_append_%lld_%zu = icmp eq i64 %s, %%direct_array_i64_unchecked_len_%lld_%zu\n",
             bid,
             ii,
             key_ref,
             bid,
             ii);
        EMIT("  %%direct_array_i64_unchecked_len_plus_one_%lld_%zu = add i64 %%direct_array_i64_unchecked_len_%lld_%zu, 1\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_unchecked_next_len_%lld_%zu = select i1 %%direct_array_i64_unchecked_is_append_%lld_%zu, i64 %%direct_array_i64_unchecked_len_plus_one_%lld_%zu, i64 %%direct_array_i64_unchecked_len_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  store i64 %%direct_array_i64_unchecked_next_len_%lld_%zu, ptr %%direct_array_i64_unchecked_len_ptr_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii);
        if (dst) {
          EMIT("  %%r%lld = add i64 1, 0\n", dst);
          set_type(dst, T_I64);
        }
        return 1;
      }

      auto int same_module_function_emit_direct_array_i64_store_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          long long key,
          long long value) {
        char recv_ref[64];
        char key_ref[64];
        char value_ref[64];
        (void)f;
        if (!same_module_function_direct_array_i64_exact_enabled()) return 0;
        if (!same_module_function_is_selected_direct_array_i64_store_target()) return 0;
        if (!(value && is_plain_i64_value(value))) return 0;

        same_module_function_format_i64_ref(recv_ref, sizeof(recv_ref), recv);
        same_module_function_format_i64_ref(key_ref, sizeof(key_ref), key);
        same_module_function_format_i64_ref(value_ref, sizeof(value_ref), value);

        EMIT("  %%direct_array_i64_base_%lld_%zu = and i64 %s, -4\n",
             bid,
             ii,
             recv_ref);
        EMIT("  %%direct_array_i64_len_addr_%lld_%zu = add i64 %%direct_array_i64_base_%lld_%zu, 16\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_len_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_len_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_len_%lld_%zu = load i64, ptr %%direct_array_i64_len_ptr_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_cap_addr_%lld_%zu = add i64 %%direct_array_i64_base_%lld_%zu, 24\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_cap_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_cap_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_cap_%lld_%zu = load i64, ptr %%direct_array_i64_cap_ptr_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_nonneg_%lld_%zu = icmp sge i64 %s, 0\n",
             bid,
             ii,
             key_ref);
        EMIT("  %%direct_array_i64_set_not_past_append_%lld_%zu = icmp ule i64 %s, %%direct_array_i64_len_%lld_%zu\n",
             bid,
             ii,
             key_ref,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_within_cap_%lld_%zu = icmp ult i64 %s, %%direct_array_i64_cap_%lld_%zu\n",
             bid,
             ii,
             key_ref,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_index_ok_%lld_%zu = and i1 %%direct_array_i64_set_nonneg_%lld_%zu, %%direct_array_i64_set_not_past_append_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_can_store_%lld_%zu = and i1 %%direct_array_i64_set_index_ok_%lld_%zu, %%direct_array_i64_set_within_cap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  br i1 %%direct_array_i64_set_can_store_%lld_%zu, label %%direct_array_i64_set_store_%lld_%zu, label %%direct_array_i64_set_fail_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);

        EMIT("direct_array_i64_set_store_%lld_%zu:\n", bid, ii);
        EMIT("  %%direct_array_i64_set_byte_index_%lld_%zu = mul i64 %s, 8\n",
             bid,
             ii,
             key_ref);
        EMIT("  %%direct_array_i64_set_data_base_%lld_%zu = add i64 %%direct_array_i64_base_%lld_%zu, 32\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_value_addr_%lld_%zu = add i64 %%direct_array_i64_set_data_base_%lld_%zu, %%direct_array_i64_set_byte_index_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_set_value_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  store i64 %s, ptr %%direct_array_i64_set_value_ptr_%lld_%zu\n",
             value_ref,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_is_append_%lld_%zu = icmp eq i64 %s, %%direct_array_i64_len_%lld_%zu\n",
             bid,
             ii,
             key_ref,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_len_plus_one_%lld_%zu = add i64 %%direct_array_i64_len_%lld_%zu, 1\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_set_next_len_%lld_%zu = select i1 %%direct_array_i64_set_is_append_%lld_%zu, i64 %%direct_array_i64_set_len_plus_one_%lld_%zu, i64 %%direct_array_i64_len_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  store i64 %%direct_array_i64_set_next_len_%lld_%zu, ptr %%direct_array_i64_len_ptr_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  br label %%direct_array_i64_set_done_%lld_%zu\n", bid, ii);

        EMIT("direct_array_i64_set_fail_%lld_%zu:\n", bid, ii);
        EMIT("  br label %%direct_array_i64_set_done_%lld_%zu\n", bid, ii);

        EMIT("direct_array_i64_set_done_%lld_%zu:\n", bid, ii);
        if (dst) {
          EMIT("  %%r%lld = phi i64 [ 1, %%direct_array_i64_set_store_%lld_%zu ], [ 0, %%direct_array_i64_set_fail_%lld_%zu ]\n",
               dst,
               bid,
               ii,
               bid,
               ii);
          set_type(dst, T_I64);
        }
        return 1;
      }

      auto int same_module_function_emit_method_keys_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        size_t argc = 0;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (!(method_view &&
              (same_module_method_view_is_direct_mir_json_flags_keys(method_view) ||
               same_module_method_view_is_direct_map_keys(method_view)))) {
          return 0;
        }
        argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        if (argc == 1) {
          long long recv_arg = 0;
          if (!same_module_function_read_arg_reg(args, 0, &recv_arg)) return 0;
          if (resolve_copy_source_reg(recv_arg) != resolve_copy_source_reg(recv)) return 0;
        } else if (argc != 0) {
          return 0;
        }
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        same_module_function_emit_i64_call(dst, "nyash.map.keys_h", ab);
        if (dst) {
          same_module_function_publish_origin(dst, ORG_ARRAY_BIRTH);
        }
        return 1;
      }

      auto int same_module_function_emit_method_set_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        long long key = 0;
        long long value = 0;
        size_t argc = 0;
        size_t key_index = 0;
        size_t value_index = 1;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (!(method_view && same_module_method_view_is_direct_set(method_view))) {
          return 0;
        }
        if (!(args && yyjson_is_arr(args))) return 0;
        argc = yyjson_arr_size(args);
        if (argc == 3) {
          key_index = 1;
          value_index = 2;
        } else if (argc != 2) {
          return 0;
        }
        if (!same_module_function_read_arg_reg(args, key_index, &key)) return 0;
        if (!same_module_function_read_arg_reg(args, value_index, &value)) return 0;
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), key, 0);
        append_i64_arg_ref(ab, sizeof(ab), value, 0);
        if (same_module_method_view_is_direct_array_set(method_view)) {
          int value_is_i64 = value && is_plain_i64_value(value);
          int value_is_string_handle = value && get_origin(value) == ORG_STRING;
          enum HakoLlvmcArraySlotEmitDemand demand =
              value_is_i64
                  ? HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_I64
                  : (value_is_string_handle
                         ? HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_TEXT_PUBLIC
                         : HAKO_LLVMC_ARRAY_SLOT_DEMAND_STORE_HANDLE);
          if (!(value_is_i64 &&
                (same_module_function_emit_direct_array_i64_store_unchecked_mir_call(
                     f,
                     bid,
                     ii,
                     dst,
                     recv,
                     key,
                     value) ||
                 same_module_function_emit_direct_array_i64_store_mir_call(
                    f,
                    bid,
                    ii,
                    dst,
                    recv,
                    key,
                    value)))) {
            hako_llvmc_emit_array_slot_store(f, dst, ab, demand);
          }
        } else {
          same_module_function_emit_i64_call(dst, "nyash.map.slot_store_hhh", ab);
        }
        return 1;
      }

      auto int same_module_function_emit_method_has_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        long long key = 0;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (!(method_view && same_module_method_view_is_direct_has(method_view))) {
          return 0;
        }
        if (!same_module_function_read_get_key_value(args, recv, &key)) return 0;
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), key, 0);
        same_module_function_emit_i64_call(dst, method_view->symbol, ab);
        return 1;
      }

      auto int same_module_function_emit_method_delete_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        char ab[256];
        long long key = 0;
        (void)f;
        (void)bid;
        (void)ii;
        ab[0] = '\0';
        if (!(method_view && same_module_method_view_is_direct_map_delete(method_view))) {
          return 0;
        }
        if (!same_module_function_read_get_key_value(args, recv, &key)) return 0;
        append_i64_arg_ref(ab, sizeof(ab), recv, 1);
        append_i64_arg_ref(ab, sizeof(ab), key, 0);
        same_module_function_emit_i64_call(dst, "nyash.map.delete_hh", ab);
        return 1;
      }

      auto int same_module_function_emit_method_indexof_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        long long needle = 0;
        long long start = 0;
        size_t arg_count = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        (void)f;
        if (!(method_view &&
              same_module_method_view_is_direct_string_indexof(method_view))) {
          return 0;
        }
        if (!(arg_count == 1 || arg_count == 2)) return 0;
        if (arg_count == 1) {
          return same_module_function_emit_string_needle_i64_call(
              dst,
              recv,
              args,
              "nyash.string.indexOf_hh");
        }
        if (!same_module_function_read_arg_reg(args, 0, &needle)) return 0;
        if (!same_module_function_read_arg_reg(args, 1, &start)) return 0;
        {
          char recv_ab[128];
          char sub_ab[256];
          char rel_ref[64];
          char start_ref[64];
          recv_ab[0] = '\0';
          sub_ab[0] = '\0';
          append_i64_arg_ref(recv_ab, sizeof(recv_ab), recv, 1);
          append_i64_arg_ref(sub_ab, sizeof(sub_ab), recv, 1);
          append_i64_arg_ref(sub_ab, sizeof(sub_ab), start, 0);
          EMIT("  %%indexof_len_%lld_%zu = call i64 @\"nyash.string.len_fast_h\"(%s)\n",
               bid,
               ii,
               recv_ab);
          {
            size_t used = strlen(sub_ab);
            if (used >= sizeof(sub_ab)) return 0;
            snprintf(
                sub_ab + used,
                sizeof(sub_ab) - used,
                ", i64 %%indexof_len_%lld_%zu",
                bid,
                ii);
          }
          EMIT("  %%indexof_tail_%lld_%zu = call i64 @\"nyash.string.substring_hii\"(%s)\n",
               bid,
               ii,
               sub_ab);
          same_module_function_format_i64_ref(start_ref, sizeof(start_ref), start);
          snprintf(rel_ref, sizeof(rel_ref), "%%indexof_rel_%lld_%zu", bid, ii);
          EMIT("  %s = call i64 @\"nyash.string.indexOf_hh\"(i64 %%indexof_tail_%lld_%zu, i64 %%r%lld)\n",
               rel_ref,
               bid,
               ii,
               resolve_copy_source_reg(needle));
          if (dst) {
            EMIT("  %%indexof_miss_%lld_%zu = icmp slt i64 %s, 0\n", bid, ii, rel_ref);
            EMIT("  %%indexof_abs_%lld_%zu = add i64 %s, %s\n", bid, ii, rel_ref, start_ref);
            EMIT("  %%r%lld = select i1 %%indexof_miss_%lld_%zu, i64 -1, i64 %%indexof_abs_%lld_%zu\n",
                 dst,
                 bid,
                 ii,
                 bid,
                 ii);
            set_type(dst, T_I64);
          }
        }
        return 1;
      }

      auto int same_module_function_emit_method_lastindexof_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        (void)f;
        (void)bid;
        (void)ii;
        if (!(method_view &&
              same_module_method_view_is_direct_string_lastindexof(method_view))) {
          return 0;
        }
        return same_module_function_emit_string_needle_i64_call(
            dst,
            recv,
            args,
            "nyash.string.lastIndexOf_hh");
      }

      auto int same_module_function_emit_method_contains_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        (void)f;
        (void)bid;
        (void)ii;
        if (!(method_view &&
              same_module_method_view_is_direct_string_contains(method_view))) {
          return 0;
        }
        return same_module_function_emit_string_needle_i64_call(
            dst,
            recv,
            args,
            "nyash.string.contains_hh");
      }

      auto int same_module_function_emit_generic_method_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const struct LoweringPlanGenericMethodView* method_view,
          long long dst,
          long long recv,
          yyjson_val* args) {
        int rc = same_module_function_emit_method_len_mir_call(
            f,
            bid,
            ii,
            method_view,
            dst,
            recv);
        if (rc != 1) {
          rc = same_module_function_emit_method_push_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_get_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_keys_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_set_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_has_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_delete_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_substring_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_indexof_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_lastindexof_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        if (rc != 1) {
          rc = same_module_function_emit_method_contains_mir_call(
              f,
              bid,
              ii,
              method_view,
              dst,
              recv,
              args);
        }
        return rc;
      }
