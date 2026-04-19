      enum GenericMethodLenRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT,
        HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN,
        HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_STRING_LEN,
      };

      auto enum GenericMethodLenRouteKind classify_generic_method_len_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan) {
        if (!plan) return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE;
        if ((bname && !strcmp(bname, "MapBox")) || plan->runtime_map_size) {
          return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT;
        }
        if ((bname && !strcmp(bname, "ArrayBox")) || plan->runtime_array_len ||
            plan->runtime_array_string) {
          return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN;
        }
        if (plan->runtime_string) {
          return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_STRING_LEN;
        }
        return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE;
      }

      auto int classify_generic_method_len_value_demand_read_ref(
          enum GenericMethodLenRouteKind route) {
        return route != HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE;
      }

      auto void format_i64_ssa_operand_ref(
          char* out,
          size_t out_cap,
          long long reg) {
        long long cv = 0;
        long long base = resolve_copy_source_reg(reg);
        if (!out || out_cap == 0) return;
        if (has_const(base, &cv)) {
          snprintf(out, out_cap, "%lld", cv);
        } else {
          snprintf(out, out_cap, "%%r%lld", base);
        }
      }

      auto int emit_string_len_window_from_plan(
          FILE* f,
          long long dst,
          long long recv,
          long long start,
          long long end) {
        long long start_const = 0;
        long long end_const = 0;
        long long start_base = resolve_copy_source_reg(start);
        long long end_base = resolve_copy_source_reg(end);
        if (!dst) return 1;
        if (has_const(start_base, &start_const) && has_const(end_base, &end_const)) {
          long long window_len = end_const - start_const;
          fprintf(f, "  %%r%lld = add i64 0, %lld\n", dst, window_len);
          set_type(dst, T_I64);
          put_const(dst, window_len);
          remember_string_length_call(dst, recv);
          return 1;
        }
        {
          char start_ref[64];
          char end_ref[64];
          format_i64_ssa_operand_ref(start_ref, sizeof(start_ref), start_base);
          format_i64_ssa_operand_ref(end_ref, sizeof(end_ref), end_base);
          fprintf(f, "  %%r%lld = sub i64 %s, %s\n", dst, end_ref, start_ref);
        }
        set_type(dst, T_I64);
        remember_string_length_call(dst, recv);
        return 1;
      }

      auto int emit_string_len_window_from_placement_effect_routes(
          FILE* f,
          long long dst,
          long long resolved_recv) {
        long long substring_start = 0;
        long long substring_end = 0;
        if (!hako_llvmc_string_corridor_read_route_window_from_placement_effect_routes(
                resolved_recv,
                &substring_start,
                &substring_end)) {
          return 0;
        }
        emit_string_len_window_from_plan(
            f,
            dst,
            resolved_recv,
            substring_start,
            substring_end);
        if (hako_llvmc_route_trace_enabled()) {
          char extra[160];
          snprintf(
              extra,
              sizeof(extra),
              "recv=%lld start=%lld end=%lld",
              resolved_recv,
              substring_start,
              substring_end);
          hako_llvmc_emit_route_trace(
              "string_len_corridor",
              "hit",
              "placement_effect_route_window",
              extra);
        }
        return 1;
      }

      auto int emit_generic_method_len_by_policy(
          FILE* f,
          const char* bname,
          long long dst,
          long long recv,
          const struct GenericMethodEmitPlan* plan,
          const char* ab) {
        long long known_len = 0;
        long long substring_start = 0;
        long long substring_end = 0;
        long long resolved_recv = resolve_copy_source_reg(recv);
        enum GenericMethodLenRouteKind route = classify_generic_method_len_route(bname, plan);
        int value_demand_read_ref =
            classify_generic_method_len_value_demand_read_ref(route);
        (void)value_demand_read_ref;
        switch (route) {
          case HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.entry_count_i64\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.entry_count_i64\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_len_h\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.array.slot_len_h\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_STRING_LEN:
            if (emit_string_len_window_from_placement_effect_routes(
                    f, dst, resolved_recv)) {
              return 1;
            }
            if (known_string_value_len(resolved_recv, &known_len)) {
              if (dst) {
                fprintf(f, "  %%r%lld = add i64 0, %lld\n", dst, known_len);
                set_type(dst, T_I64);
                put_const(dst, known_len);
                remember_string_length_call(dst, recv);
              }
              if (hako_llvmc_route_trace_enabled()) {
                char extra[128];
                snprintf(
                    extra,
                    sizeof(extra),
                    "recv=%lld len=%lld",
                    resolved_recv,
                    known_len);
                hako_llvmc_emit_route_trace(
                    "string_len_corridor",
                    "hit",
                    "known_string_len",
                    extra);
              }
              return 1;
            }
            if (hako_llvmc_string_corridor_read_plan_window(
                    resolved_recv,
                    "direct_kernel_entry",
                    &substring_start,
                    &substring_end)) {
              emit_string_len_window_from_plan(
                  f,
                  dst,
                  resolved_recv,
                  substring_start,
                  substring_end);
              if (hako_llvmc_route_trace_enabled()) {
                char extra[160];
                snprintf(
                    extra,
                    sizeof(extra),
                    "recv=%lld start=%lld end=%lld",
                    resolved_recv,
                    substring_start,
                    substring_end);
                hako_llvmc_emit_route_trace(
                    "string_len_corridor",
                    "hit",
                    "substring_len_direct_kernel_plan_window",
                    extra);
              }
              return 1;
            }
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.string.len_h\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
              remember_string_length_call(dst, recv);
            } else {
              fprintf(f, "  call i64 @\"nyash.string.len_h\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE:
          default:
            return 0;
        }
      }
