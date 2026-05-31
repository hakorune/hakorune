      auto int same_module_function_direct_array_i64_exact_enabled(void) {
        const char* value = getenv("HAKO_ARRAY_SLOT_STORE");
        return value && !strcmp(value, "direct_array_i64_exact");
      }

      auto int same_module_function_is_selected_direct_array_i64_store_target(
          long long bid,
          size_t ii,
          long long recv,
          long long key,
          long long value,
          long long dst) {
        return lowering_plan_direct_array_access_site_matches_checked_i64(
            bid,
            ii,
            "store",
            "direct_array_i64_store",
            recv,
            key,
            value,
            1,
            dst,
            1);
      }

      auto int same_module_function_is_selected_direct_array_i64_unchecked_store_target(void) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        return name &&
               (!strcmp(name, "HakoAllocPageModel.acquireFreshSmall/1") ||
                !strcmp(name, "HakoAllocPageModel.releaseLocalKnownLive/1"));
      }

      auto int same_module_function_is_selected_direct_array_i64_range_index_store_target(
          long long bid,
          size_t ii,
          long long recv,
          long long key,
          long long value,
          long long dst) {
        return lowering_plan_direct_array_access_site_matches_proved_unchecked_range_index_store_i64(
            bid,
            ii,
            "direct_array_i64_store",
            recv,
            key,
            value,
            dst,
            1);
      }

      auto int same_module_function_emit_direct_array_i64_store_range_index_mir_call(
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
        if (!same_module_function_is_selected_direct_array_i64_range_index_store_target(
                bid, ii, recv, key, value, dst)) return 0;
        if (!(value && is_plain_i64_value(value))) return 0;

        same_module_function_format_i64_ref(recv_ref, sizeof(recv_ref), recv);
        same_module_function_format_i64_ref(key_ref, sizeof(key_ref), key);
        same_module_function_format_i64_ref(value_ref, sizeof(value_ref), value);

        EMIT("  %%direct_array_i64_range_index_base_%lld_%zu = and i64 %s, -4\n",
             bid,
             ii,
             recv_ref);
        EMIT("  %%direct_array_i64_range_index_len_addr_%lld_%zu = add i64 %%direct_array_i64_range_index_base_%lld_%zu, 16\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_range_index_len_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_range_index_len_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_range_index_len_%lld_%zu = load i64, ptr %%direct_array_i64_range_index_len_ptr_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_range_index_byte_index_%lld_%zu = mul i64 %s, 8\n",
             bid,
             ii,
             key_ref);
        EMIT("  %%direct_array_i64_range_index_data_base_%lld_%zu = add i64 %%direct_array_i64_range_index_base_%lld_%zu, 32\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_range_index_value_addr_%lld_%zu = add i64 %%direct_array_i64_range_index_data_base_%lld_%zu, %%direct_array_i64_range_index_byte_index_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_range_index_value_ptr_%lld_%zu = inttoptr i64 %%direct_array_i64_range_index_value_addr_%lld_%zu to ptr\n",
             bid,
             ii,
             bid,
             ii);
        EMIT("  store i64 %s, ptr %%direct_array_i64_range_index_value_ptr_%lld_%zu\n",
             value_ref,
             bid,
             ii);
        EMIT("  %%direct_array_i64_range_index_index_plus_one_%lld_%zu = add i64 %s, 1\n",
             bid,
             ii,
             key_ref);
        EMIT("  %%direct_array_i64_range_index_len_grows_%lld_%zu = icmp ult i64 %%direct_array_i64_range_index_len_%lld_%zu, %%direct_array_i64_range_index_index_plus_one_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  %%direct_array_i64_range_index_next_len_%lld_%zu = select i1 %%direct_array_i64_range_index_len_grows_%lld_%zu, i64 %%direct_array_i64_range_index_index_plus_one_%lld_%zu, i64 %%direct_array_i64_range_index_len_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("  store i64 %%direct_array_i64_range_index_next_len_%lld_%zu, ptr %%direct_array_i64_range_index_len_ptr_%lld_%zu\n",
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
        if (!same_module_function_is_selected_direct_array_i64_store_target(bid, ii, recv, key, value, dst)) return 0;
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
                (same_module_function_emit_direct_array_i64_store_range_index_mir_call(
                     f,
                     bid,
                     ii,
                     dst,
                     recv,
                     key,
                     value) ||
                 same_module_function_emit_direct_array_i64_store_unchecked_mir_call(
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
